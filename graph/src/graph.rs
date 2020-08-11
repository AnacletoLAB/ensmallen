//! A graph representation optimized for executing random walks on huge graphs.
use super::*;
use counter::Counter;
use derive_getters::Getters;
use hashbrown::{HashMap as HashBrownMap, HashSet as HashBrownSet};
use indicatif::{ParallelProgressIterator, ProgressBar, ProgressStyle};
use itertools::Itertools;
use log::info;
use rayon::prelude::*;
use std::collections::HashMap;
use vec_rand::{
    sample,
    gen_random_vec
};

// TODO FIGURE OUT HOW TO REMOVE PUB FROM ATTRIBUTES
/// A graph representation optimized for executing random walks on huge graphs.
///
/// This class should be initialized using the two constructors:
/// `graph::Graph::new_directed` or `graph::Graph::new_undirected`
///
/// # Examples
///
#[derive(Debug, Clone, Getters, PartialEq)]
pub struct Graph {
    pub(crate) is_builded: bool,
    pub(crate) is_directed: bool,
    pub(crate) not_trap_nodes: Vec<NodeT>,
    pub(crate) sources: Vec<NodeT>,
    pub(crate) destinations: Vec<NodeT>,
    pub(crate) nodes_mapping: HashMap<String, NodeT>,
    pub(crate) nodes_reverse_mapping: Vec<String>,
    pub(crate) unique_edges: HashBrownMap<(NodeT, NodeT), EdgeT>,
    pub(crate) outbounds: Vec<EdgeT>,
    pub(crate) weights: Option<Vec<WeightT>>,
    pub(crate) node_types: Option<Vec<NodeTypeT>>,
    pub(crate) node_types_mapping: Option<HashMap<String, NodeTypeT>>,
    pub(crate) node_types_reverse_mapping: Option<Vec<String>>,
    pub(crate) edge_types: Option<Vec<EdgeTypeT>>,
    pub(crate) edge_types_mapping: Option<HashMap<String, EdgeTypeT>>,
    pub(crate) edge_types_reverse_mapping: Option<Vec<String>>,
    pub(crate) has_traps: bool,
}

/// # Graph utility methods
impl Graph {
    /// Returns node type of given node.
    ///
    /// # Arguments
    ///
    /// * node_id: NodeT - node whose node type is to be returned.
    ///
    pub fn get_node_type_id(&self, node_id: NodeT) -> Result<NodeTypeT, String> {
        if let Some(nt) = &self.node_types {
            return if node_id <= nt.len() {
                Ok(nt[node_id])
            } else {
                Err(format!(
                    "The node_index {} is too big for the node_types vector which has len {}",
                    node_id,
                    nt.len()
                ))
            };
        }
        Err(String::from(
            "Node types are not defined for current graph instance.",
        ))
    }

    /// Returns edge type of given edge.
    ///
    /// # Arguments
    ///
    /// * edge_id: EdgeT - edge whose edge type is to be returned.
    ///
    pub fn get_edge_type_id(&self, edge_id: EdgeT) -> Result<EdgeTypeT, String> {
        if let Some(et) = &self.edge_types {
            return if edge_id <= et.len() {
                Ok(et[edge_id])
            } else {
                Err(format!(
                    "The edge_index {} is too big for the edge_types vector which has len {}",
                    edge_id,
                    et.len()
                ))
            };
        }
        Err(String::from(
            "Edge types are not defined for current graph instance.",
        ))
    }

    /// Returns edge type counts.
    pub fn get_edge_type_counts(&self) -> Result<HashMap<EdgeTypeT, usize>, String> {
        if let Some(et) = &self.edge_types {
            Ok(Counter::init(et.clone()).into_map())
        } else {
            Err(String::from(
                "Edge types are not defined for current graph instance.",
            ))
        }
    }

    /// Returns node type counts.
    pub fn get_node_type_counts(&self) -> Result<HashMap<NodeTypeT, usize>, String> {
        if let Some(nt) = &self.node_types {
            Ok(Counter::init(nt.clone()).into_map())
        } else {
            Err(String::from(
                "Node types are not defined for current graph instance.",
            ))
        }
    }

    /// Returns top k most common nodes and node types by node type frequency.
    ///
    /// # Arguments
    ///
    /// * k:usize - Number of common node types to return.
    ///
    pub fn get_top_k_nodes_by_node_type(
        &self,
        k: usize,
    ) -> Result<(Vec<NodeT>, Vec<NodeTypeT>), String> {
        if let Some(nt) = &self.node_types {
            let counts = self.get_node_type_counts()?;
            let top_k: HashBrownSet<_> = counts
                .iter()
                .sorted_by(|(_, v1), (_, v2)| Ord::cmp(&v2, &v1))
                .take(k)
                .map(|(k1, _)| k1)
                .collect();
            let filtered: Vec<bool> = nt
                .into_par_iter()
                .map(|node_type| top_k.contains(&node_type))
                .collect();
            Ok((
                (0..self.get_nodes_number())
                    .zip(filtered.iter())
                    .filter_map(|(node, filter)| if *filter { Some(node) } else { None })
                    .collect(),
                nt.iter()
                    .zip(filtered.iter())
                    .filter_map(|(nt, filter)| if *filter { Some(*nt) } else { None })
                    .collect(),
            ))
        } else {
            Err(String::from(
                "Node types are not defined for current graph instance.",
            ))
        }
    }

    /// Returns top k most common edges and edge types by edge type frequency.
    ///
    /// # Arguments
    ///
    /// * k:usize - Number of common node types to return.
    ///
    pub fn get_top_k_edges_by_edge_type(
        &self,
        k: usize,
    ) -> Result<(Vec<EdgeT>, Vec<EdgeTypeT>), String> {
        if let Some(nt) = &self.edge_types {
            let counts = self.get_edge_type_counts()?;
            let top_k: HashBrownSet<_> = counts
                .iter()
                .sorted_by(|(_, v1), (_, v2)| Ord::cmp(&v2, &v1))
                .take(k)
                .map(|(k1, _)| k1)
                .collect();
            let filtered: Vec<bool> = nt
                .into_par_iter()
                .map(|edge_type| top_k.contains(&edge_type))
                .collect();
            Ok((
                (0..self.get_edges_number())
                    .zip(filtered.iter())
                    .filter_map(|(edge, filter)| if *filter { Some(edge) } else { None })
                    .collect(),
                nt.iter()
                    .zip(filtered.iter())
                    .filter_map(|(nt, filter)| if *filter { Some(*nt) } else { None })
                    .collect(),
            ))
        } else {
            Err(String::from(
                "Edge types are not defined for current graph instance.",
            ))
        }
    }

    /// Returns boolean representing if edge passing between given nodes exists.
    ///
    /// # Arguments
    ///
    /// * src: NodeT - The source node of the edge.
    /// * dst: NodeT - The destination node of the edge.
    ///
    pub fn has_edge(&self, src: NodeT, dst: NodeT) -> bool {
        self.unique_edges.contains_key(&(src, dst))
    }

    /// Return true if given graph has any edge overlapping with current graph.
    ///
    /// # Arguments
    ///
    /// * graph: Graph - The graph to check against.
    ///
    pub fn overlaps(&self, graph: &Graph) -> bool {
        graph
            .sources
            .par_iter()
            .zip(graph.destinations.par_iter())
            .any(|(src, dst)| {
                let local_src_id: Option<&NodeT> = self
                    .nodes_mapping
                    .get(&graph.nodes_reverse_mapping[*src].clone());
                let local_dst_id: Option<&NodeT> = self
                    .nodes_mapping
                    .get(&graph.nodes_reverse_mapping[*dst].clone());
                if let Some(lsrc) = local_src_id {
                    if let Some(ldst) = local_dst_id {
                        self.has_edge(*lsrc, *ldst)
                    } else {
                        false
                    }
                } else {
                    false
                }
            })
    }

    /// Return true if given graph edges are all contained within current graph.
    ///
    /// # Arguments
    ///
    /// * graph: Graph - The graph to check against.
    ///
    pub fn contains(&self, graph: &Graph) -> bool {
        graph
            .sources
            .par_iter()
            .zip(graph.destinations.par_iter())
            .all(|(src, dst)| {
                let local_src_id: Option<&NodeT> = self
                    .nodes_mapping
                    .get(&graph.nodes_reverse_mapping[*src].clone());
                let local_dst_id: Option<&NodeT> = self
                    .nodes_mapping
                    .get(&graph.nodes_reverse_mapping[*dst].clone());
                if let Some(lsrc) = local_src_id {
                    if let Some(ldst) = local_dst_id {
                        self.has_edge(*lsrc, *ldst)
                    } else {
                        false
                    }
                } else {
                    false
                }
            })
    }

    /// Returns edge id of the edge passing between given nodes.
    ///
    /// # Arguments
    ///
    /// * src: NodeT - The source node of the edge.
    /// * dst: NodeT - The destination node of the edge.
    ///
    pub fn get_edge_id(&self, src: NodeT, dst: NodeT) -> Result<EdgeT, String> {
        match self.unique_edges.get(&(src, dst)) {
            Some(g) => Ok(*g),
            None => Err(format!(
                concat!(
                    "Required edge passing between {src_name} ({src}) ",
                    "and {dst_name} ({dst}) does not exists in graph."
                ),
                src_name = self.nodes_reverse_mapping[src],
                src = src,
                dst_name = self.nodes_reverse_mapping[dst],
                dst = dst
            )),
        }
    }

    /// Returns number of nodes in the graph.
    pub fn get_nodes_number(&self) -> usize {
        self.nodes_reverse_mapping.len()
    }

    /// Returns number of not node nodes in the graph.
    pub fn get_not_trap_nodes_number(&self) -> usize {
        self.not_trap_nodes.len()
    }

    /// Returns number of edges in the graph.
    pub fn get_edges_number(&self) -> usize {
        self.sources.len()
    }

    /// Returns number of edge types in the graph.
    pub fn get_edge_types_number(&self) -> usize {
        if let Some(etm) = &self.edge_types_mapping {
            etm.keys().len()
        } else {
            0
        }
    }

    /// Returns number of node types in the graph.
    pub fn get_node_types_number(&self) -> usize {
        if let Some(etm) = &self.node_types_mapping {
            etm.keys().len()
        } else {
            0
        }
    }

    /// Return range of outbound edges IDs for given Node.
    ///
    /// # Arguments
    ///
    /// * node: NodeT - Node for which we need to compute the outbounds range.
    ///
    pub(crate) fn get_min_max_edge(&self, node: NodeT) -> (EdgeT, EdgeT) {
        let min_edge: EdgeT = if node == 0 {
            0
        } else {
            self.outbounds[node - 1]
        };
        let max_edge: EdgeT = self.outbounds[node];
        (min_edge, max_edge)
    }

    /// Return mapping from instance not trap nodes to dense nodes.
    pub fn get_dense_nodes_mapping(&self) -> HashMap<NodeT, NodeT> {
        self.sources
            .iter()
            .chain(self.destinations.iter())
            .cloned()
            .unique()
            .enumerate()
            .map(|(i, node)| (node, i))
            .collect()
    }

    /// Returns the number of outbound neighbours of given node.
    ///
    ///
    /// # Arguments
    ///
    /// * `node` - Integer ID of the node.
    ///
    pub fn degree(&self, node: NodeT) -> NodeT {
        let (_min, _max) = self.get_min_max_edge(node);
        _max - _min
    }

    /// Returns the degree of every node in the graph.
    pub fn degrees(&self) -> Vec<NodeT> {
        (0..self.get_nodes_number())
            .into_par_iter()
            .map(|node| self.degree(node))
            .collect()
    }

    /// Returns boolean representing if given node is a trap.
    ///
    /// # Arguments
    ///
    /// * `node` - Integer ID of the node, if this is bigger that the number of nodes it will panic.
    ///
    pub fn is_node_trap(&self, node: NodeT) -> bool {
        self.degree(node) == 0
    }
    /// Returns boolean representing if given edge is a trap.
    ///
    /// # Arguments
    ///
    /// * `edge` - Integer ID of the edge, if this is bigger that the number of edges it will panic.
    ///
    pub fn is_edge_trap(&self, edge: EdgeT) -> bool {
        self.is_node_trap(self.destinations[edge])
    }

    /// Returns list of neigbours of given node.
    ///
    /// # Arguments
    ///
    /// * `node` - Integer ID of the node, if this is bigger that the number of nodes it will panic.
    ///
    pub fn get_node_neighbours(&self, node: NodeT) -> Vec<NodeT> {
        let (min_edge, max_edge) = self.get_min_max_edge(node);
        self.destinations[min_edge..max_edge].to_vec()
    }

    /// Extract random nodes from the graph
    /// 
    /// # Arguments
    ///
    /// * `size` - How many nodes to extract.
    /// * `seed` - Seed to use for the PRNG for reproducibility porpouses
    ///
    pub fn extract_random_nodes(&self, size: usize, seed: u64) -> Vec<NodeT> {
        gen_random_vec(size, seed).iter().map(
            |idx| *idx as NodeT % self.get_nodes_number()
        ).collect()
    }

    /// Extract random nodes from the graph in parallel using multiple threads
    /// 
    /// # Arguments
    ///
    /// * `size` - How many nodes to extract.
    /// * `seed` - Seed to use for the PRNG for reproducibility porpouses
    ///
    pub fn extract_random_nodes_par(&self, size: usize, seed: u64, chunk_size: Option<usize>) -> Vec<NodeT> {
        let _chunk_size = chunk_size.unwrap_or(size / 8);
        if _chunk_size <= 1 {
            return self.extract_random_nodes(size, seed);
        }
        let mut result = (0..(size / _chunk_size) as u64).into_par_iter()
            .map(|i| {
                gen_random_vec(_chunk_size, seed ^ (i * 1337)).par_iter().map(
                    |idx| *idx as NodeT % self.get_nodes_number()
                ).collect::<Vec<NodeT>>()
            })
            .flatten()
            .collect::<Vec<NodeT>>();
        let diff = size - result.len();
        if diff != 0 {
            result.extend(self.extract_random_nodes(diff, seed ^ 1337).iter());
        }
        result
    }

    /// Extract random edges from the graph
    /// 
    /// # Arguments
    ///
    /// * `size` - How many edges to extract.
    /// * `seed` - Seed to use for the PRNG for reproducibility porpouses
    pub fn extract_random_edges(&self, size: usize, seed: u64) -> Vec<Vec<NodeT>> {
        gen_random_vec(size, seed).iter().map(
            |idx| {
                let i: NodeT = *idx as NodeT % self.get_edges_number();
                vec![self.sources[i], self.destinations[i]]
            }
        ).collect()
    }

    /// Extract random edges from the graph in parallel using multiple threads
    /// 
    /// # Arguments
    ///
    /// * `size` - How many edges to extract.
    /// * `seed` - Seed to use for the PRNG for reproducibility porpouses
    pub fn extract_random_edges_par(&self, size: usize, seed: u64, chunk_size: Option<usize>) -> Vec<Vec<NodeT>> {
        let _chunk_size = chunk_size.unwrap_or(size / 8);
        if _chunk_size <= 1 {
            return self.extract_random_edges(size, seed);
        }
        let mut result = (0..(size / _chunk_size) as u64).into_par_iter()
            .map(|i| {
                gen_random_vec(_chunk_size, seed ^ (i * 1337)).par_iter().map(
                    |idx| {
                        let i: NodeT = *idx as NodeT % self.get_edges_number();
                        vec![self.sources[i], self.destinations[i]]
                    }
                ).collect::<Vec<Vec<NodeT>>>()
            })
            .flatten()
            .collect::<Vec<Vec<NodeT>>>();
        let diff = size - result.len();
        if diff != 0 {
            let diffs = self.extract_random_edges(diff, seed ^ 1337);
            for x in diffs {
                result.push(x);
            }
        }
        result
    }

    /// Return the node transition weights and the related node and edges.
    ///
    /// # Arguments
    ///
    /// * node: NodeT, the previous node from which to compute the transitions, if this is bigger that the number of nodes it will panic.
    /// * change_node_type_weight: ParamsT, weight for changing node type.
    ///
    fn get_node_transition(
        &self,
        node: NodeT,
        change_node_type_weight: ParamsT,
    ) -> (Vec<WeightT>, &[NodeT], EdgeT, EdgeT) {
        // Retrieve edge boundaries.
        let (min_edge, max_edge) = self.get_min_max_edge(node);
        // If weights are given
        let mut transition: Vec<WeightT> = if let Some(w) = &self.weights {
            w[min_edge..max_edge].to_vec()
        } else {
            vec![1.0; max_edge - min_edge]
        };

        let destinations: &[NodeT] = &self.destinations[min_edge..max_edge];

        //############################################################
        //# Handling of the change node type parameter               #
        //############################################################

        if (change_node_type_weight - 1.0).abs() > f64::EPSILON {
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

    /// Return the edge transition weights and the related node and edges.
    ///
    /// # Arguments
    ///
    /// * edge: EdgeT - the previous edge from which to compute the transitions.
    /// * weights: WalkWeights - Weights to use for the weighted walk.
    fn get_edge_transition(
        &self,
        edge: EdgeT,
        walk_weights: &WalkWeights,
    ) -> (Vec<WeightT>, &[NodeT], EdgeT, EdgeT) {
        // Get the source and destination for current edge.
        let (src, dst) = (self.sources[edge], self.destinations[edge]);

        // Compute the transition weights relative to the node weights.
        let (mut transition, destinations, min_edge, max_edge) =
            self.get_node_transition(dst, walk_weights.change_node_type_weight);

        //############################################################
        //# Handling of the change edge type parameter               #
        //############################################################

        // If the edge types were given:
        if (walk_weights.change_edge_type_weight - 1.0).abs() > f64::EPSILON {
            if let Some(et) = &self.edge_types {
                //# If the neighbour edge type matches the previous
                //# edge type (we are not changing the edge type)
                //# we weigth using the provided change_edge_type_weight weight.
                let this_type: EdgeTypeT = et[edge];
                transition
                    .iter_mut()
                    .zip(et[min_edge..max_edge].iter())
                    .filter(|(_, &neigh_type)| this_type == neigh_type)
                    .for_each(|(transition_value, _)| {
                        *transition_value /= walk_weights.change_edge_type_weight
                    });
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
        if (walk_weights.return_weight - 1.0).abs() > f64::EPSILON {
            transition
                .iter_mut()
                .zip(destinations.iter())
                .filter(|&(_, ndst)| src == *ndst || dst == *ndst)
                .for_each(|(transition_value, _)| *transition_value *= walk_weights.return_weight);
        }
        //############################################################
        //# Handling of the Q parameter: the exploration coefficient #
        //############################################################

        if (walk_weights.explore_weight - 1.0).abs() > f64::EPSILON {
            transition
                .iter_mut()
                .zip(destinations.iter())
                .filter(|&(_, ndst)| {
                    (src != *ndst || dst == *ndst) && !self.unique_edges.contains_key(&(*ndst, src))
                })
                .for_each(|(transition_value, _)| *transition_value *= walk_weights.explore_weight);
        }

        (transition, destinations, min_edge, max_edge)
    }

    /// Return new sampled node with the transition edge used.
    ///
    /// # Arguments
    ///
    /// * node: NodeT, the previous node from which to compute the transitions.
    /// * change_node_type_weight: ParamsT, weight for changing node type.
    pub fn extract_node(&self, node: NodeT, change_node_type_weight: ParamsT) -> (NodeT, EdgeT) {
        let (mut weights, dsts, min_edge, _) =
            self.get_node_transition(node, change_node_type_weight);
        let index = sample(&mut weights);
        (dsts[index], min_edge + index)
    }

    /// Return new random edge with given weights.
    ///
    /// # Arguments
    ///
    /// * edge: EdgeT, the previous edge from which to compute the transitions.
    /// * walk_weights: WalkWeights, the weights for the weighted random walks.
    pub fn extract_edge(&self, edge: EdgeT, walk_weights: &WalkWeights) -> (NodeT, EdgeT) {
        let (mut weights, dsts, min_edge, _) = self.get_edge_transition(edge, walk_weights);
        let index = sample(&mut weights);
        (dsts[index], min_edge + index)
    }

    /// Returns vector of walks.
    ///
    /// # Arguments
    ///
    /// * parameters: WalksParameters - the weighted walks parameters.
    ///
    pub fn walk(&self, parameters: &WalksParameters) -> Result<Vec<Vec<NodeT>>, String> {
        // Validate if given parameters are compatible with current graph.
        parameters.validate(&self)?;

        info!("Starting random walk.");
        let pb = if parameters.verbose {
            let pb = ProgressBar::new(parameters.total_iterations() as u64);
            pb.set_draw_delta(parameters.total_iterations() as u64 / 100);
            pb.set_style(ProgressStyle::default_bar().template(
                "Computing random walks {spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] ({pos}/{len}, ETA {eta})",
            ));
            pb
        } else {
            ProgressBar::hidden()
        };

        let iterator = (0..parameters.total_iterations())
            .into_par_iter()
            .progress_with(pb)
            .map(|index| self.not_trap_nodes[parameters.mode_index(index)]);

        let mut walks = if self.has_traps {
            iterator
                .map(|node| self.single_walk(node, &parameters.single_walk_parameters))
                .filter(|walk| walk.len() >= parameters.min_length)
                .collect::<Vec<Vec<NodeT>>>()
        } else {
            iterator
                .map(|node| self.single_walk_no_traps(node, &parameters.single_walk_parameters))
                .collect::<Vec<Vec<NodeT>>>()
        };

        if let Some(dense_nodes_mapping) = &parameters.dense_nodes_mapping {
            walks.par_iter_mut().for_each(|walk| {
                walk.iter_mut()
                    .for_each(|node| *node = *dense_nodes_mapping.get(node).unwrap())
            })
        }

        Ok(walks)
    }

    /// Returns single walk from given node
    ///
    /// # Arguments
    ///
    /// * node: NodeT - Node from where to start the random walks.
    /// * parameters: SingleWalkParameters - Parameters for the single walk.
    ///
    pub fn single_walk(&self, node: NodeT, parameters: &SingleWalkParameters) -> Vec<NodeT> {
        let (dst, mut edge) = self.extract_node(node, parameters.weights.change_node_type_weight);

        if self.is_node_trap(dst) {
            return vec![node, dst];
        }

        let mut walk: Vec<NodeT> = Vec::with_capacity(parameters.length);
        walk.push(node);
        walk.push(dst);

        for _ in 2..parameters.length {
            if self.is_edge_trap(edge) {
                break;
            }
            let (dst, inner_edge) = self.extract_edge(edge, &parameters.weights);
            edge = inner_edge;
            walk.push(dst);
        }
        walk
    }

    /// Returns single walk from given node.
    ///
    /// This method assumes that there are no traps in the graph.
    ///
    /// # Arguments
    ///
    /// * node: NodeT - Node from where to start the random walks.
    /// * parameters: SingleWalkParameters - Parameters for the single walk.
    ///
    pub fn single_walk_no_traps(&self, node: NodeT, parameters: &SingleWalkParameters) -> Vec<NodeT> {
        let mut walk: Vec<NodeT> = Vec::with_capacity(parameters.length);
        walk.push(node);

        let (dst, mut edge) = self.extract_node(node, parameters.weights.change_node_type_weight);
        walk.push(dst);

        for _ in 2..parameters.length {
            let (dst, inner_edge) = self.extract_edge(edge, &parameters.weights);
            edge = inner_edge;
            walk.push(dst);
        }
        walk
    }
}
