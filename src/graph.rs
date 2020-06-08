use rand::distributions::WeightedIndex;
use rand::rngs::ThreadRng;
use std::collections::{HashMap, HashSet};
use std::iter::Enumerate;
use std::iter::FromIterator;

type node_t = usize;
type edge_t = usize;
type weight_t = f64;
type node_type_t = u16;
type edge_type_t = u16;

struct Graph {
    sources: Vec<node_t>,
    destinations: Vec<node_t>,
    nodes_mapping: HashMap<String, node_t>,
    reverse_nodes_mapping: Vec<String>,
    unique_edges: HashSet<(node_t, node_t)>,
    outbounds: Vec<edge_t>,
    weights: Option<Vec<weight_t>>,
    node_types: Option<Vec<node_type_t>>,
    edge_types: Option<Vec<edge_type_t>>,
}

impl Graph {
    pub fn new_directed(
        nodes: Vec<String>,
        sources_names: Vec<String>,
        destinations_names: Vec<String>,
        node_types: Option<Vec<node_type_t>>,
        edge_types: Option<Vec<edge_type_t>>,
        weights: Option<Vec<edge_type_t>>,
    ) -> Graph {
        debug!("Computing nodes to node IDs mapping.");
        let nodes_mapping: HashMap<String, node_t> =
            nodes.iter().zip((0..nodes.len()).iter()).collect();

        debug!("Computing sources node IDs.");
        let sources: Vec<node_t> = sources_names
            .iter()
            .map(|dst| nodes_mapping.get(&dst))
            .collect();

        debug!("Computing destinations node IDs.");
        let destinations: Vec<node_t> = destinations_names
            .iter()
            .map(|dst| nodes_mapping.get(&dst))
            .collect();

        debug!("Computing unique edges.");
        let unique_edges: HashSet<(node_t, node_t)> =
            HashSet::from_iter(sources.iter().zip(destinations.iter()));

        debug!("Computing sorting of given edges based on sources.");
        let permutation = permutation::sort(&sources[..]);

        debug!("Sorting given sources.");
        sources = permutation.apply_slice(&sources[..]);
        debug!("Sorting given destinations.");
        destinations = permutation.apply_slice(&destinations[..]);
        if let Some(w) = &mut weights {
            debug!("Sorting given weights.");
            w = permutation.apply_slice(&w[..]);
        }
        if let Some(e) = &mut edge_types {
            debug!("Sorting given edge types.");
            e = permutation.apply_slice(&e[..]);
        }

        Graph {
            sources: sources,
            destinations: destinations,
            nodes_mapping: nodes_mapping,
            reverse_nodes_mapping: nodes,
            unique_edges: unique_edges,
            outbounds: Graph::compute_outbounds(nodes.len(), sources),
            weights: weights,
            node_types: node_types,
            edge_types: edge_types,
        }
    }

    pub fn new_undirected(
        nodes: Vec<String>,
        sources_names: Vec<String>,
        destinations_names: Vec<String>,
        node_types: Option<Vec<node_type_t>>,
        edge_types: Option<Vec<edge_type_t>>,
        weights: Option<Vec<edge_type_t>>,
    ) -> Graph {
        debug!("Identifying self-loops present in given graph.");
        let loops_mask: Vec<bool> = sources_names
            .iter()
            .zip(destinations_names.iter())
            .map(|a, b| a == b)
            .collect();

        let total_loops: u64 = loops_mask.iter().sum();
        let total_edges: u64 = (sources_names.len() - total_loops) * 2 + total_loops;

        debug!("Building undirected graph sources.");
        let full_sources: Vec<String> = sources_names.clone();
        full_sources.extend(
            sources_names
                .iter()
                .zip(loops_mask.iter())
                .filter(|&(_, &mask)| mask)
                .map(|(value, _)| value)
                .collect(),
        );

        debug!("Building undirected graph destinations.");
        let full_destinations: Vec<String> = destinations_names.clone();
        full_destinations.extend(
            destinations_names
                .iter()
                .zip(loops_mask.iter())
                .filter(|&(_, &mask)| mask)
                .map(|(value, _)| value)
                .collect(),
        );

        if let Some(e) = &mut edge_types {
            debug!("Building undirected graph edge types.");
            e.extend(
                e.iter()
                    .zip(loops_mask.iter())
                    .filter(|&(_, &mask)| mask)
                    .map(|(value, _)| value)
                    .collect(),
            );
        }

        if let Some(w) = &mut weights {
            debug!("Building undirected graph weights.");
            w.extend(
                w.iter()
                    .zip(loops_mask.iter())
                    .filter(|&(_, &mask)| mask)
                    .map(|(value, _)| value)
                    .collect(),
            );
        }

        Graph::new_directed(
            nodes,
            full_sources,
            full_destinations,
            node_types,
            edge_types,
            weights,
        )
    }

    fn compute_outbounds(nodes_number: node_t, sources: Vec<node_t>) -> Vec<edge_t> {
        debug!("Computing outbound edges ranges from each node.");
        let last_src: node_t = 0;
        // Instead of fixing the last values after the loop, we set directly
        // all values to the length of the sources, which is the sum of all
        // possible neighbors.
        let mut outbounds: Vec<edge_t> = vec![sources.len(); nodes_number];

        for (i, src) in self.sources.iter().enumerate() {
            if last_src != src {
                // Assigning to range instead of single value, so that traps
                // have as delta between previous and next node zero.
                for j in last_src..src {
                    outbounds[j] = i;
                }
                last_src = src;
            }
        }

        return outbounds;
    }

    fn get_min_max_edge(&self, node: node_t) -> (edge_t, edge_t) {
        let min_edge: edge_t = if node == 0 {
            0
        } else {
            self.outbounds[node - 1]
        };
        let max_edge: edge_t = self.outbounds[node];
        return (min_edge, max_edge);
    }

    fn is_node_trap(&self, node: node_t) -> bool {
        let (_min, _max) = self.get_min_max_edge(node);
        return _min == _max;
    }

    fn is_edge_trap(&self, edge: edge_t) -> bool {
        return self.is_node_trap(self.destinations[edge]);
    }

    fn get_node_transition(self, node: node_t) -> (Vec<float>, Vec<int>, edge_t, edge_t) {
        // Retrieve edge boundaries.
        let (min_edge, max_edge) = self.get_min_max_edge(node);
        // If weights are given
        let mut transition: Vec<weight_t> = if let Some(w) = &self.weights {
            self.weights[min_edge..max_edge]
        } else {
            vec![1; max_edge - min_edge]
        };

        destinations = self.destinations[min_edge..max_edge];

        //############################################################
        //# Handling of the change node type parameter               #
        //############################################################

        // If the node types were given:
        if let Some(nt) = &self.node_types {
            // if the destination node type matches the neighbour
            // destination node type (we are not changing the node type)
            // we weigth using the provided change_node_type_weight weight.
            let this_type: node_type_t = nt[node];

            transition
                .iter_mut()
                .zip(nt[destinations].iter())
                .filter(|(_, neigh_type)| this_type == neigh_type)
                .map(|(transition_value, _)| *transition_value /= self._change_node_type_weight)
                .collect();
        }

        return (transition, destinations, min_edge, max_edge);
    }

    fn get_edge_transition(self, edge: edge_t) -> (Vec<float>, Vec<int>, edge_t, edge_t) {
        // Get the source and destination for current edge.
        let (src, dst) = (self.sources[edge], self.destinations[edge]);

        // Compute the transition weights relative to the node weights.
        let (transition, destinations, min_edge, max_edge) = self.get_node_transition(dst);

        //############################################################
        //# Handling of the change edge type parameter               #
        //############################################################

        // If the edge types were given:
        if let Some(et) = &self.edge_types {
            //# If the neighbour edge type matches the previous
            //# edge type (we are not changing the edge type)
            //# we weigth using the provided change_edge_type_weight weight.
            let this_type: edge_type_t = et[edge];
            transition
                .iter_mut()
                .zip(et[min_edge..max_edge].iter())
                .filter(|(_, neigh_type)| this_type == neigh_type)
                .map(|(transition_value, _)| *transition_value /= self._change_edge_type_weight)
                .collect();
        }

        //############################################################
        //# Handling of the Q parameter: the return coefficient      #
        //############################################################

        //# If the neigbour matches with the source, hence this is
        //# a backward loop like the following:
        //# SRC -> DST
        //#  ▲     /
        //#   \___/
        //#
        //# We weight the edge weight with the given return weight.

        transition
            .iter_mut()
            .zip(destinations.iter())
            .filter(|(_, dst)| src == dst)
            .map(|(transition_value, _)| *transition_value *= self._return_weight)
            .collect();

        //############################################################
        //# Handling of the P parameter: the exploration coefficient #
        //############################################################

        transition
            .iter_mut()
            .zip(destinations.iter())
            .filter(|(_, dst)| !self.unique_edges.contains((ndst, src)))
            .map(|(transition_value, _)| *transition_value *= self._explore_weight)
            .collect();

        return (transition, destinations, min_edge, max_edge);
    }

    fn extract(self, weights: Vec<weight_t>) -> node_t {
        // TODO! This creates an object every time. Are we sure?
        return WeightedIndex::new(&weights)
            .unwrap("Cannot sample weights")
            .sample(&thread_rng());
    }

    fn extract_node(self, node: int) -> (node_t, edge_t) {
        let (weights, dsts, min_edge, _) = self.get_node_transition(node);
        index = self._lazy_extraction(weights);
        return (dsts[index], min_edge + index);
    }

    fn extract_edge(self, edge: int) -> (node_t, edge_t) {
        let (weights, dsts, min_edge, _) = self.get_edge_transition(edge);
        index = self._lazy_extraction(weights);
        return (dsts[index], min_edge + index);
    }

    pub fn walk(&self, number: u64, length: usize ) -> Vec<Vec<node_t>>{
        (0..number).into_par_iter().map(|_| 
            (0..self.nodes_neighbours.len()).into_par_iter().map(
                |x| self.single_walk(length, x)
            ).collect()
        ).collect() 
    }

    fn single_walk(&self, length: usize, node: node_t) -> Vec<node_t>{
        let mut walk: Vec<node_t> = Vec::with_capacity(length);
        walk.append(node);

        if self.is_node_trap(node) {
            return walk;
        }

        let (dst, edge) = self.extract_random_node_neighbour(node);
        walk.append(dst);

        for _ in 2..length {

            if self.is_edge_trap(edge){
                break;
            }
            let (dst, edge) = self.extract_random_edge_neighbour(edge);
            walk.append(dst);
        }
        walk
    }
}
