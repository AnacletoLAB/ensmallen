use super::*;
use indicatif::{ParallelProgressIterator, ProgressBar, ProgressStyle};
use linked_hash_set::LinkedHashSet;
use log::info;
use rayon::prelude::*;
use vec_rand::xorshift::xorshift;
use vec_rand::{sample, sample_uniform};

impl Graph {
    /// Return the base weighted transitions.
    ///
    /// # Arguments
    ///
    /// * min_edge_id: EdgeT - The minimum edge id.
    /// * max_edge_id: EdgeT - The maximum edge id.
    ///
    fn get_weighted_transitions(&self, min_edge_id: EdgeT, max_edge_id: EdgeT) -> Vec<WeightT> {
        match &self.weights {
            // If the graph is weighted we return the weights
            Some(ws) => ws[min_edge_id..max_edge_id].to_vec(),
            // Otherwise we return an uniform vector.
            None => vec![1.0; max_edge_id - min_edge_id],
        }
    }

    /// Return the sorted HashSet of the destinations.
    ///
    /// # Arguments
    ///
    /// * min_edge_id: EdgeT - The minimum edge id.
    /// * max_edge_id: EdgeT - The maximum edge id.
    fn get_destinations_hash_set(
        &self,
        min_edge_id: EdgeT,
        max_edge_id: EdgeT,
    ) -> (LinkedHashSet<NodeT>, NodeT, NodeT) {
        let mut destinations = LinkedHashSet::with_capacity(max_edge_id - min_edge_id);
        let mut min_dst: NodeT = 0;
        let mut max_dst: NodeT = 0;
        for edge_id in min_edge_id..max_edge_id {
            let dst = self.get_destination(edge_id);
            destinations.insert(dst);
            // If is first
            if min_edge_id == edge_id {
                min_dst = dst;
            }
            // If is last
            if max_edge_id == edge_id + 1 {
                max_dst = dst;
            }
        }
        (destinations, min_dst, max_dst)
    }

    fn get_node_transition_data(
        &self,
        node: NodeT,
    ) -> (
        LinkedHashSet<NodeT>,
        NodeT,
        NodeT,
        Vec<WeightT>,
        EdgeT,
        EdgeT,
    ) {
        let (min_edge_id, max_edge_id) = self.get_destinations_min_max_edge_ids(node);
        let (destinations, min_dst, max_dst) =
            self.get_destinations_hash_set(min_edge_id, max_edge_id);
        (
            destinations,
            min_dst,
            max_dst,
            self.get_weighted_transitions(min_edge_id, max_edge_id),
            min_edge_id,
            max_edge_id,
        )
    }

    /// TODO: Update docstring!
    fn update_node_transition(
        &self,
        node: NodeT,
        transition: &mut Vec<WeightT>,
        destinations: &LinkedHashSet<NodeT>,
        change_node_type_weight: ParamsT,
    ) {
        //############################################################
        //# Handling of the change node type parameter               #
        //############################################################

        if (change_node_type_weight - 1.0).abs() > f64::EPSILON {
            // If the node types were given:
            if let Some(nt) = &self.node_types {
                // if the destination node type matches the neighbour
                // destination node type (we are not changing the node type)
                // we weigth using the provided change_node_type_weight weight.
                let this_type: NodeTypeT = nt.ids[node];

                transition.iter_mut().zip(destinations.iter()).for_each(
                    |(transition_value, dst)| {
                        if this_type == nt.ids[*dst] {
                            *transition_value /= change_node_type_weight
                        }
                    },
                );
            }
        }
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
    ) -> (
        LinkedHashSet<NodeT>,
        NodeT,
        NodeT,
        Vec<WeightT>,
        EdgeT,
        EdgeT,
    ) {
        // Retrieve the data to compute the update transition
        let (destinations, min_dst, max_dst, mut transition, min_edge_id, max_edge_id) =
            self.get_node_transition_data(node);

        // Compute the transition weights relative to the node weights.
        self.update_node_transition(
            node,
            &mut transition,
            &destinations,
            change_node_type_weight,
        );

        (
            destinations,
            min_dst,
            max_dst,
            transition,
            min_edge_id,
            max_edge_id,
        )
    }

    /// Return the edge transition weights and the related node and edges.
    ///
    /// # Arguments
    ///
    /// * edge: EdgeT - the previous edge from which to compute the transitions.
    /// * weights: WalkWeights - Weights to use for the weighted walk.
    fn get_edge_transition(
        &self,
        edge_id: EdgeT,
        walk_weights: &WalkWeights,
        previous_destinations: &LinkedHashSet<NodeT>,
        previous_min_dst: NodeT,
        previous_max_dst: NodeT,
    ) -> (
        LinkedHashSet<NodeT>,
        NodeT,
        NodeT,
        Vec<WeightT>,
        EdgeT,
        EdgeT,
    ) {
        // Get the source and destination for current edge.
        let (src, dst) = self.get_edge_from_edge_id(edge_id);

        let (destinations, min_dst, max_dst, mut transition, min_edge_id, max_edge_id) =
            self.get_node_transition_data(dst);

        // Compute the transition weights relative to the node weights.
        self.update_node_transition(
            dst,
            &mut transition,
            &destinations,
            walk_weights.change_node_type_weight,
        );

        //############################################################
        //# Handling of the change edge type parameter               #
        //############################################################

        // If the edge types were given:
        if (walk_weights.change_edge_type_weight - 1.0).abs() > f64::EPSILON {
            if let Some(ets) = &self.edge_types {
                //# If the neighbour edge type matches the previous
                //# edge type (we are not changing the edge type)
                //# we weigth using the provided change_edge_type_weight weight.
                let this_type: EdgeTypeT = ets.ids[edge_id];
                transition
                    .iter_mut()
                    .zip(min_edge_id..max_edge_id)
                    .for_each(|(transition_value, edge_id)| {
                        if this_type == ets.ids[edge_id] {
                            *transition_value /= walk_weights.change_edge_type_weight
                        }
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
        if (walk_weights.return_weight - 1.0).abs() > f64::EPSILON
            || (walk_weights.explore_weight - 1.0).abs() > f64::EPSILON
        {
            transition
                .iter_mut()
                .zip(&destinations)
                .for_each(|(transition_value, ndst)| {
                    if src == *ndst || dst == *ndst {
                        *transition_value *= walk_weights.return_weight
                    }
                });
        }
        //############################################################
        //# Handling of the Q parameter: the exploration coefficient #
        //############################################################

        if (walk_weights.explore_weight - 1.0).abs() > f64::EPSILON {
            transition
                .iter_mut()
                .zip(&destinations)
                .for_each(|(transition_value, ndst)| {
                    if !(src == *ndst
                        || dst == *ndst
                        || previous_min_dst > *ndst
                        || previous_max_dst < *ndst
                        || previous_destinations.contains(ndst))
                    {
                        *transition_value *= walk_weights.explore_weight
                    }
                });
        }

        (
            destinations,
            min_dst,
            max_dst,
            transition,
            min_edge_id,
            max_edge_id,
        )
    }

    /// Return new sampled node with the transition edge used.
    ///
    /// # Arguments
    ///
    /// * node: NodeT, the previous node from which to compute the transitions.
    /// * seed: usize, the seed to use for extracting the node.
    ///
    pub fn extract_uniform_node(&self, node: NodeT, seed: usize) -> NodeT {
        let (min_edge, max_edge) = self.get_destinations_min_max_edge_ids(node);
        self.get_destination(min_edge + sample_uniform((max_edge - min_edge) as u64, seed as u64))
    }

    /// Return new sampled node with the transition edge used.
    ///
    /// # Arguments
    ///
    /// * node: NodeT, the previous node from which to compute the transitions.
    /// * seed: usize, the seed to use for extracting the node.
    /// * change_node_type_weight: ParamsT, weight for changing node type.
    pub fn extract_node(
        &self,
        node: NodeT,
        seed: usize,
        change_node_type_weight: ParamsT,
    ) -> (LinkedHashSet<NodeT>, NodeT, NodeT, NodeT, EdgeT) {
        let (destinations, min_dst, max_dst, mut weights, min_edge, _) =
            self.get_node_transition(node, change_node_type_weight);
        let edge_id = min_edge + sample(&mut weights, seed as u64);
        (
            destinations,
            min_dst,
            max_dst,
            self.get_destination(edge_id),
            edge_id,
        )
    }

    /// Return new random edge with given weights.
    ///
    /// # Arguments
    ///
    /// * edge: EdgeT, the previous edge from which to compute the transitions.
    /// * seed: usize, the seed to use for extracting the node.
    /// * walk_weights: WalkWeights, the weights for the weighted random walks.
    pub fn extract_edge(
        &self,
        edge: EdgeT,
        seed: usize,
        walk_weights: &WalkWeights,
        previous_destinations: &LinkedHashSet<NodeT>,
        previous_min_dst: NodeT,
        previous_max_dst: NodeT,
    ) -> (LinkedHashSet<NodeT>, NodeT, NodeT, NodeT, EdgeT) {
        let (destinations, min_dst, max_dst, mut weights, min_edge, _) = self.get_edge_transition(
            edge,
            walk_weights,
            previous_destinations,
            previous_min_dst,
            previous_max_dst,
        );
        let edge_id = min_edge + sample(&mut weights, seed as u64);
        (
            destinations,
            min_dst,
            max_dst,
            self.get_destination(edge_id),
            edge_id,
        )
    }

    /// Return vector of walks run on each non-trap node of the graph.
    ///
    /// # Arguments
    ///
    /// * parameters: WalksParameters - the weighted walks parameters.
    ///
    pub fn random_walks(
        &self,
        quantity: usize,
        parameters: &WalksParameters,
    ) -> Result<Vec<Vec<NodeT>>, String> {
        self.walk(
            quantity,
            |global_index| {
                let local_index = global_index % quantity;
                let random_source_id = xorshift((parameters.seed + local_index) as u64) as usize;
                (random_source_id, self.get_unique_source(random_source_id))
            },
            parameters,
        )
    }

    /// Return vector of walks run on a random subset of the not trap nodes.
    ///
    /// # Arguments
    ///
    /// * parameters: WalksParameters - the weighted walks parameters.
    ///
    pub fn complete_walks(&self, parameters: &WalksParameters) -> Result<Vec<Vec<NodeT>>, String> {
        self.walk(
            self.get_unique_sources_number(),
            |random_source_id| (random_source_id, self.get_unique_source(random_source_id)),
            parameters,
        )
    }

    /// Returns vector of walks.
    ///
    /// # Arguments
    ///
    /// * parameters: WalksParameters - the weighted walks parameters.
    ///
    fn walk(
        &self,
        quantity: usize,
        to_node: impl Fn(usize) -> (usize, NodeT) + Sync + Send,
        parameters: &WalksParameters,
    ) -> Result<Vec<Vec<NodeT>>, String> {
        // Validate if given parameters are compatible with current graph.
        parameters.validate(&self)?;

        let total_iterations = quantity * parameters.iterations;

        info!("Starting random walk.");
        let pb = get_loading_bar(
            parameters.verbose,
            "Compute random walks",
            total_iterations as u64,
        );

        let iterator = (0..total_iterations)
            .into_par_iter()
            .progress_with(pb)
            .map(to_node);

        let mut walks = if self.has_traps {
            if self.weights.is_none() && parameters.is_first_order_walk() {
                info!("Using trap-aware uniform first order random walk algorithm.");
                iterator
                    .map(|(seed, node)| {
                        self.uniform_walk(node, seed, &parameters.single_walk_parameters)
                    })
                    .filter(|walk| walk.len() >= parameters.min_length)
                    .collect::<Vec<Vec<NodeT>>>()
            } else {
                if self.directed {
                    unreachable!("Not supporting directed walks as of now.");
                }
                info!("Using trap-aware second order random walk algorithm.");
                iterator
                    .map(|(seed, node)| {
                        self.single_walk(node, seed, &parameters.single_walk_parameters)
                    })
                    .filter(|walk| walk.len() >= parameters.min_length)
                    .collect::<Vec<Vec<NodeT>>>()
            }
        } else if self.weights.is_none() && parameters.is_first_order_walk() {
            info!("Using uniform first order random walk algorithm.");
            iterator
                .map(|(seed, node)| {
                    self.uniform_walk_no_traps(node, seed, &parameters.single_walk_parameters)
                })
                .collect::<Vec<Vec<NodeT>>>()
        } else {
            if self.directed {
                unreachable!("Not supporting directed walks as of now.");
            }
            info!("Using second order random walk algorithm.");
            iterator
                .map(|(seed, node)| {
                    self.single_walk_no_traps(node, seed, &parameters.single_walk_parameters)
                })
                .collect::<Vec<Vec<NodeT>>>()
        };

        if let Some(dense_node_mapping) = &parameters.dense_node_mapping {
            walks.par_iter_mut().for_each(|walk| {
                walk.iter_mut()
                    .for_each(|node| *node = *dense_node_mapping.get(node).unwrap())
            })
        }

        Ok(walks)
    }

    /// Returns single walk from given node
    ///
    /// # Arguments
    ///
    /// * node: NodeT - Node from where to start the random walks.
    /// * seed: usize, the seed to use for extracting the nodes and edges.
    /// * parameters: SingleWalkParameters - Parameters for the single walk.
    ///
    pub fn single_walk(
        &self,
        node: NodeT,
        seed: usize,
        parameters: &SingleWalkParameters,
    ) -> Vec<NodeT> {
        let (mut previous_destinations, mut previous_min_dst, mut previous_max_dst, dst, mut edge) =
            self.extract_node(node, seed, parameters.weights.change_node_type_weight);

        if self.is_node_trap(dst) {
            return vec![node, dst];
        }

        let mut walk: Vec<NodeT> = Vec::with_capacity(parameters.length);
        walk.push(node);
        walk.push(dst);

        for iteration in 2..parameters.length {
            if self.is_edge_trap(edge) {
                break;
            }
            let (destinations, min_dst, max_dst, dst, inner_edge) = self.extract_edge(
                edge,
                iteration + seed,
                &parameters.weights,
                &previous_destinations,
                previous_min_dst,
                previous_max_dst,
            );
            previous_min_dst = min_dst;
            previous_max_dst = max_dst;
            previous_destinations = destinations;
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
    /// * seed: usize, the seed to use for extracting the nodes and edges.
    /// * parameters: SingleWalkParameters - Parameters for the single walk.
    ///
    pub fn single_walk_no_traps(
        &self,
        node: NodeT,
        seed: usize,
        parameters: &SingleWalkParameters,
    ) -> Vec<NodeT> {
        let mut walk: Vec<NodeT> = Vec::with_capacity(parameters.length);
        walk.push(node);

        let (mut previous_destinations, mut previous_min_dst, mut previous_max_dst, dst, mut edge) =
            self.extract_node(node, seed, parameters.weights.change_node_type_weight);
        walk.push(dst);

        for iteration in 2..parameters.length {
            let (destinations, min_dst, max_dst, dst, inner_edge) = self.extract_edge(
                edge,
                seed + iteration,
                &parameters.weights,
                &previous_destinations,
                previous_min_dst,
                previous_max_dst,
            );
            previous_min_dst = min_dst;
            previous_max_dst = max_dst;
            previous_destinations = destinations;
            edge = inner_edge;
            walk.push(dst);
        }
        walk
    }

    /// Returns single walk from given node executed uniformely.
    ///
    /// This walk executes uniformely a walk of first order. This method
    /// works in context of uniform graphs (all weights are None) and the
    /// weights of the node2vec are all equal to 1.
    ///
    /// # Arguments
    ///
    /// * node: NodeT - Node from where to start the random walks.
    /// * seed: usize, the seed to use for extracting the nodes and edges.
    /// * parameters: SingleWalkParameters - Parameters for the single walk.
    ///
    fn uniform_walk(
        &self,
        node: NodeT,
        seed: usize,
        parameters: &SingleWalkParameters,
    ) -> Vec<NodeT> {
        let mut dst = self.extract_uniform_node(node, seed);

        if self.is_node_trap(dst) {
            return vec![node, dst];
        }

        let mut walk: Vec<NodeT> = Vec::with_capacity(parameters.length);
        walk.push(node);
        walk.push(dst);

        for iteration in 2..parameters.length {
            if self.is_node_trap(dst) {
                break;
            }
            dst = self.extract_uniform_node(dst, seed + iteration);
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
    /// * seed: usize, the seed to use for extracting the nodes and edges.
    /// * parameters: SingleWalkParameters - Parameters for the single walk.
    ///
    fn uniform_walk_no_traps(
        &self,
        node: NodeT,
        seed: usize,
        parameters: &SingleWalkParameters,
    ) -> Vec<NodeT> {
        let mut walk: Vec<NodeT> = Vec::with_capacity(parameters.length);
        let mut dst = self.extract_uniform_node(node, seed);
        walk.push(node);
        walk.push(dst);

        for iteration in 2..parameters.length {
            dst = self.extract_uniform_node(dst, seed + iteration);
            walk.push(dst);
        }
        walk
    }
}
