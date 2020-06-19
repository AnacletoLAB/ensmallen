use derive_getters::Getters;
use log::info;
use rayon::prelude::*;
use std::collections::{HashMap};
use super::types::*;
use super::random::sample;


// TODO FIGURE OUT HOW TO REMOVE PUB FROM ATTRIBUTES
#[derive(Debug, Clone, Getters)]
pub struct Graph {
    pub sources: Vec<NodeT>,
    pub destinations: Vec<NodeT>,
    pub nodes_mapping: HashMap<String, NodeT>,
    pub nodes_reverse_mapping: Vec<String>,
    pub unique_edges: HashMap<(NodeT, NodeT), EdgeT>,
    pub outbounds: Vec<EdgeT>,
    pub weights: Option<Vec<WeightT>>,
    pub node_types: Option<Vec<NodeTypeT>>,
    pub node_types_mapping: Option<HashMap<String, NodeTypeT>>,
    pub node_types_reverse_mapping: Option<Vec<String>>,
    pub edge_types: Option<Vec<EdgeTypeT>>,
    pub edge_types_mapping: Option<HashMap<String, EdgeTypeT>>,
    pub edge_types_reverse_mapping: Option<Vec<String>>
}

impl Graph {

    pub fn compute_outbounds(nodes_number: NodeT, sources: &[NodeT]) -> Vec<EdgeT> {
        info!("Computing outbound edges ranges from each node.");
        let mut last_src: NodeT = 0;
        // Instead of fixing the last values after the loop, we set directly
        // all values to the length of the sources, which is the sum of all
        // possible neighbors.
        let mut outbounds: Vec<EdgeT> = vec![sources.len(); nodes_number];

        for (i, src) in sources.iter().enumerate() {
            if last_src != *src {
                // Assigning to range instead of single value, so that traps
                // have as delta between previous and next node zero.
                for o in &mut outbounds[last_src..*src] {
                    *o = i;
                }
                last_src = *src;
            }
        }

        outbounds
    }

    pub fn get_node_type_id(&self, node_id:NodeT) -> NodeTypeT {
        if let Some(nt) = &self.node_types{
            return nt[node_id]
        }
        panic!("Node types are not defined for current class.");
    }

    pub fn get_edge_type_id(&self, edge_id:EdgeT) -> EdgeTypeT {
        if let Some(et) = &self.edge_types{
            return et[edge_id]
        }
        panic!("Edge types are not defined for current class.");
    }

    pub fn get_edge_id(&self, src:NodeT, dst:NodeT) -> EdgeT {
        *self.unique_edges.get(&(src, dst)).unwrap()
    }

    pub fn get_nodes_number(&self) -> usize {
        self.nodes_reverse_mapping.len()
    }

    pub fn get_edges_number(&self) -> usize {
        self.sources.len()
    }

    pub fn get_edge_types_number(&self) -> usize {
        if let Some(etm) = &self.edge_types_mapping {
            etm.keys().len()
        } else {
            0
        }
    }

    pub fn get_node_types_number(&self) -> usize {
        if let Some(etm) = &self.node_types_mapping {
            etm.keys().len()
        } else {
            0
        }
    }

    fn get_min_max_edge(&self, node: NodeT) -> (EdgeT, EdgeT) {
        let min_edge: EdgeT = if node == 0 {
            0
        } else {
            self.outbounds[node - 1]
        };
        let max_edge: EdgeT = self.outbounds[node];
        (min_edge, max_edge)
    }

    fn is_node_trap(&self, node: NodeT) -> bool {
        let (_min, _max) = self.get_min_max_edge(node);
        _min == _max
    }

    fn is_edge_trap(&self, edge: EdgeT) -> bool {
        self.is_node_trap(self.destinations[edge])
    }

    fn get_node_transition(
        &self,
        node: NodeT,
        change_node_type_weight: ParamsT,
    ) -> (Vec<WeightT>, Vec<NodeT>, EdgeT, EdgeT) {
        // Retrieve edge boundaries.
        let (min_edge, max_edge) = self.get_min_max_edge(node);
        // If weights are given
        let mut transition: Vec<WeightT> = if let Some(w) = &self.weights {
            w[min_edge..max_edge].to_vec()
        } else {
            vec![1.0; max_edge - min_edge]
        };

        let destinations: Vec<NodeT> = self.destinations[min_edge..max_edge].to_vec();

        //############################################################
        //# Handling of the change node type parameter               #
        //############################################################

        if (change_node_type_weight  - 1.0).abs() > f64::EPSILON {
            // If the node types were given:
            if let Some(nt) = &self.node_types {
                // if the destination node type matches the neighbour
                // destination node type (we are not changing the node type)
                // we weigth using the provided change_node_type_weight weight.
                let this_type: NodeTypeT = nt[node];

                transition
                    .iter_mut()
                    .zip(destinations.iter().map(|dst| nt[*dst]))
                    .filter(|(_, neigh_type)| this_type == *neigh_type)
                    .for_each(|(transition_value, _)| *transition_value /= change_node_type_weight);
                // credo non serva collect perche' modifichiamo i valori direttamente
            }
        }
        (transition, destinations, min_edge, max_edge)
    }

    fn get_edge_transition(
        &self,
        edge: EdgeT,
        return_weight: ParamsT,
        explore_weight: ParamsT,
        change_node_type_weight: ParamsT,
        change_edge_type_weight: ParamsT,
    ) -> (Vec<WeightT>, Vec<NodeT>, EdgeT, EdgeT) {
        // Get the source and destination for current edge.
        let (src, dst) = (self.sources[edge], self.destinations[edge]);

        // Compute the transition weights relative to the node weights.
        let (mut transition, destinations, min_edge, max_edge) =
            self.get_node_transition(dst, change_node_type_weight);

        //############################################################
        //# Handling of the change edge type parameter               #
        //############################################################

        // If the edge types were given:
        if (change_edge_type_weight - 1.0).abs() > f64::EPSILON {
            if let Some(et) = &self.edge_types {
                //# If the neighbour edge type matches the previous
                //# edge type (we are not changing the edge type)
                //# we weigth using the provided change_edge_type_weight weight.
                let this_type: EdgeTypeT = et[edge];
                transition
                    .iter_mut()
                    .zip(et[min_edge..max_edge].iter())
                    .filter(|(_, &neigh_type)| this_type == neigh_type)
                    .for_each(|(transition_value, _)| *transition_value /= change_edge_type_weight);
            }
        }

        //############################################################
        //# Handling of the P parameter: the return coefficient      #
        //############################################################

        //# If the neigbour matches with the source, hence this is
        //# a backward loop like the following:
        //# SRC -> DST
        //#  ▲     /
        //#   \___/
        //#
        //# We weight the edge weight with the given return weight.

        // If the return weight, which is the inverse of p, is not 1, hence
        // it has some impact, we procced and increase by the given weight
        // the probability of transitions that go back a previously visited
        // node.
        if (return_weight  - 1.0).abs() > f64::EPSILON {
            transition
                .iter_mut()
                .zip(destinations.iter())
                .filter(|&(_, ndst)| src == *ndst || dst == *ndst)
                .for_each(|(transition_value, _)| *transition_value *= return_weight);
        }
        //############################################################
        //# Handling of the Q parameter: the exploration coefficient #
        //############################################################

        if (explore_weight  - 1.0).abs() > f64::EPSILON {
            transition
                .iter_mut()
                .zip(destinations.iter())
                .filter(|&(_, ndst)| (src != *ndst || dst == *ndst) && !self.unique_edges.contains_key(&(*ndst, src)))
                .for_each(|(transition_value, _)| *transition_value *= explore_weight);
        }

        (transition, destinations, min_edge, max_edge)
    }

    fn extract_node(&self, node: NodeT, change_node_type_weight: ParamsT) -> (NodeT, EdgeT) {
        let (weights, dsts, min_edge, _) = self.get_node_transition(node, change_node_type_weight);
        let index = sample(&weights);
        (dsts[index], min_edge + index)
    }

    fn extract_edge(
        &self,
        edge: EdgeT,
        return_weight: ParamsT,
        explore_weight: ParamsT,
        change_node_type_weight: ParamsT,
        change_edge_type_weight: ParamsT,
    ) -> (NodeT, EdgeT) {
        let (weights, dsts, min_edge, _) = self.get_edge_transition(
            edge,
            return_weight,
            explore_weight,
            change_node_type_weight,
            change_edge_type_weight,
        );
        let index = sample(&weights);
        (dsts[index], min_edge + index)
    }

    pub fn walk(
        &self,
        iterations: usize,
        length: usize,
        min_length: Option<usize>,
        return_weight: Option<ParamsT>,
        explore_weight: Option<ParamsT>,
        change_node_type_weight: Option<ParamsT>,
        change_edge_type_weight: Option<ParamsT>,
    ) -> Vec<Vec<NodeT>> {
        let _min_length = min_length.unwrap_or(0);
        let _return_weight = return_weight.unwrap_or(1.0);
        let _explore_weight = explore_weight.unwrap_or(1.0);
        let _change_node_type_weight = change_node_type_weight.unwrap_or(1.0);
        let _change_edge_type_weight = change_edge_type_weight.unwrap_or(1.0);

        if _return_weight <= 0.0 {
            panic!("Given 'return_weight' is not a strictly positive real number.")
        }
        if _explore_weight <= 0.0 {
            panic!("Given 'explore_weight' is not a strictly positive real number.")
        }
        if _change_node_type_weight <= 0.0 {
            panic!("Given 'change_node_type_weight' is not a strictly positive real number.")
        }
        if _change_edge_type_weight <= 0.0 {
            panic!("Given 'change_edge_type_weight' is not a strictly positive real number.")
        }

        info!("Starting random walk.");
        let number_of_results = iterations * self.get_nodes_number();

        (0..number_of_results)
            .into_par_iter()
            .map(|node| {
                self.single_walk(
                    length,
                    node / iterations,
                    _return_weight,
                    _explore_weight,
                    _change_node_type_weight,
                    _change_edge_type_weight
                )
            })
            .filter(|walk| walk.len() >= _min_length)
            .collect::<Vec<Vec<NodeT>>>()
    }

    fn single_walk(
        &self,
        length: usize,
        node: NodeT,
        return_weight: ParamsT,
        explore_weight: ParamsT,
        change_node_type_weight: ParamsT,
        change_edge_type_weight: ParamsT,
    ) -> Vec<NodeT> {
        let mut walk: Vec<NodeT> = Vec::with_capacity(length);
        walk.push(node);

        if self.is_node_trap(node) {
            return walk;
        }

        let (dst, mut edge) = self.extract_node(node, change_node_type_weight);
        walk.push(dst);

        for _ in 2..length {
            if self.is_edge_trap(edge) {
                break;
            }
            let (dst, inner_edge) = self.extract_edge(
                edge,
                return_weight,
                explore_weight,
                change_node_type_weight,
                change_edge_type_weight,
            );
            edge = inner_edge;
            walk.push(dst);
        }
        walk
    }
}
