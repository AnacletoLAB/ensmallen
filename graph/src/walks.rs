use super::*;
use log::info;
use rayon::prelude::*;
use vec_rand::sample_f32 as sample;
use vec_rand::sample_uniform;
use vec_rand::splitmix64;

#[inline(always)]
fn update_return_weight_transition(
    transition: &mut Vec<WeightT>,
    destinations: &[NodeT],
    src: NodeT,
    dst: NodeT,
    return_weight: ParamsT,
    has_selfloop: bool,
) {
    if let Ok(mut i) = destinations.binary_search(&src) {
        let mut j = i;
        while j > 0 && destinations[j] == src {
            transition[j] *= return_weight;
            j -= 1;
        }
        i += 1;
        while i < destinations.len() && destinations[i] == src {
            transition[i] *= return_weight;
            i += 1;
        }
    }

    if src != dst && has_selfloop {
        if let Ok(mut i) = destinations.binary_search(&dst) {
            let mut j = i;
            while j > 0 && destinations[j] == dst {
                transition[j] *= return_weight;
                j -= 1;
            }
            i += 1;
            while i < destinations.len() && destinations[i] == dst {
                transition[i] *= return_weight;
                i += 1;
            }
        }
    }
}

#[inline(always)]
fn rust_update_explore_weight_transition(
    transition: &mut Vec<WeightT>,
    destinations: &[NodeT],
    previous_destinations: &[NodeT],
    explore_weight: ParamsT,
    src: NodeT,
    dst: NodeT,
) {
    let mut i = 0;
    let mut j = 0;
    let mut v1: NodeT;
    let mut v2: NodeT;
    //############################################################
    //# Handling of the Q parameter: the explore coefficient     #
    //############################################################
    // This coefficient increases the probability of switching
    // to nodes not locally seen.
    while i < destinations.len() && j < previous_destinations.len() {
        v1 = destinations[i];
        v2 = previous_destinations[j];
        if v1 <= v2 {
            let is_less = v1 < v2;
            if is_less && v1 != src && v1 != dst {
                transition[i] *= explore_weight;
            }
            j += !is_less as usize;
            i += 1;
        } else {
            j += 1;
        }
    }
    for k in i..destinations.len() {
        v1 = destinations[k];
        transition[k] *= 1.0 + (v1 != src && v1 != dst) as u64 as WeightT * (explore_weight - 1.0);
    }
}

#[inline(always)]
fn rust_update_return_explore_weight_transition(
    transition: &mut Vec<WeightT>,
    destinations: &[NodeT],
    previous_destinations: &[NodeT],
    return_weight: ParamsT,
    explore_weight: ParamsT,
    src: NodeT,
    dst: NodeT,
) {
    let mut i = 0;
    let mut j = 0;
    let mut v1: NodeT;
    let mut v2: NodeT;
    //############################################################
    //# Handling of the Q parameter: the explore coefficient     #
    //############################################################
    // This coefficient increases the probability of switching
    // to nodes not locally seen.
    while i < destinations.len() && j < previous_destinations.len() {
        v1 = destinations[i];
        v2 = previous_destinations[j];
        if v1 == src || v1 == dst {
            transition[i] *= return_weight;
            i += 1;
            continue;
        }
        if v1 <= v2 {
            let is_less = v1 < v2;
            if is_less {
                transition[i] *= explore_weight;
            }
            j += !is_less as usize;
            i += 1;
        } else {
            j += 1;
        }
    }
    for k in i..destinations.len() {
        v1 = destinations[k];
        if v1 == src || v1 == dst {
            transition[k] *= return_weight;
        } else {
            transition[k] *= explore_weight;
        }
    }
}

extern "C" {
    fn c_update_explore_weight_transition(
        transition: *const f32,
        destinations: *const u32,
        destinations_len: u32,
        previous_destinations: *const u32,
        previous_destinations_len: u32,
        explore_weight: f32,
        src: u32,
        dst: u32,
    );
    fn c_update_return_explore_weight_transition(
        transition: *const f32,
        destinations: *const u32,
        destinations_len: u32,
        previous_destinations: *const u32,
        previous_destinations_len: u32,
        explore_weight: f32,
        return_weight: f32,
        src: u32,
        dst: u32,
    );
}

fn update_explore_weight_transition(
    transition: &mut Vec<WeightT>,
    destinations: &[NodeT],
    previous_destinations: &[NodeT],
    explore_weight: ParamsT,
    src: NodeT,
    dst: NodeT,
) {
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    {
        if is_x86_feature_detected!("avx2") {
            unsafe {
                c_update_explore_weight_transition(
                    transition.as_ptr(),
                    destinations.as_ptr(),
                    destinations.len() as u32,
                    previous_destinations.as_ptr(),
                    previous_destinations.len() as u32,
                    explore_weight,
                    src,
                    dst,
                );
            }
            return;
        }
    }
    rust_update_explore_weight_transition(
        transition,
        destinations,
        previous_destinations,
        explore_weight,
        src,
        dst,
    );
}

fn update_return_explore_weight_transition(
    transition: &mut Vec<WeightT>,
    destinations: &[NodeT],
    previous_destinations: &[NodeT],
    return_weight: ParamsT,
    explore_weight: ParamsT,
    src: NodeT,
    dst: NodeT,
) {
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    {
        if is_x86_feature_detected!("avx2") {
            unsafe {
                c_update_return_explore_weight_transition(
                    transition.as_ptr(),
                    destinations.as_ptr(),
                    destinations.len() as u32,
                    previous_destinations.as_ptr(),
                    previous_destinations.len() as u32,
                    explore_weight,
                    return_weight,
                    src,
                    dst,
                );
            }
            return;
        }
    }
    rust_update_return_explore_weight_transition(
        transition,
        destinations,
        previous_destinations,
        return_weight,
        explore_weight,
        src,
        dst,
    );
}

#[cfg(test)]
mod tests {
    use super::update_explore_weight_transition;
    use super::update_return_explore_weight_transition;
    use super::update_return_weight_transition;
    use super::WeightT;

    #[test]
    fn test_update_explore_weight_transition() {
        let destinations = vec![
            1, 2, 3, 4, 4, 4, 5, 6, 100, 101, 101, 101, 101, 101, 101, 101, 101, 101, 101, 101,
            101, 101, 101, 101,
        ];
        let previous_destinations = vec![2, 4, 4, 4];
        let mut transitions = (0..destinations.len())
            .map(|_| 1.0)
            .collect::<Vec<WeightT>>();
        update_explore_weight_transition(
            &mut transitions,
            &destinations,
            &previous_destinations,
            2.0,
            6,
            100,
        );
        assert_eq!(
            transitions,
            vec![
                2.0, 1.0, 2.0, 1.0, 1.0, 1.0, 2.0, 1.0, 1.0, 2.0, 2.0, 2.0, 2.0, 2.0, 2.0, 2.0,
                2.0, 2.0, 2.0, 2.0, 2.0, 2.0, 2.0, 2.0
            ]
        )
    }

    #[test]
    fn test_update_return_explore_weight_transition() {
        let destinations = vec![1, 2, 3, 4, 4, 4, 5, 6, 100];
        let previous_destinations = vec![2, 4, 4, 4];
        let mut transitions = (0..destinations.len())
            .map(|_| 1.0)
            .collect::<Vec<WeightT>>();
        update_return_explore_weight_transition(
            &mut transitions,
            &destinations,
            &previous_destinations,
            3.0,
            2.0,
            6,
            100,
        );
        assert_eq!(
            transitions,
            vec![2.0, 1.0, 2.0, 1.0, 1.0, 1.0, 2.0, 3.0, 3.0]
        )
    }

    #[test]
    fn test_update_return_weight_transition() {
        let destinations = vec![1, 2, 3, 4, 4, 4, 5, 6, 100];
        let mut transitions = (0..destinations.len())
            .map(|_| 1.0)
            .collect::<Vec<WeightT>>();
        update_return_weight_transition(&mut transitions, &destinations, 6, 2, 2.0, true);
        assert_eq!(
            transitions,
            vec![1.0, 2.0, 1.0, 1.0, 1.0, 1.0, 1.0, 2.0, 1.0]
        )
    }
}

impl Graph {
    /// Return the base weighted transitions.
    ///
    /// # Arguments
    /// * `min_edge_id`: EdgeT - The minimum edge id.
    /// * `max_edge_id`: EdgeT - The maximum edge id.
    /// * `probabilistic_indices`: &Option<Vec<u64>> - Optional list of the indices used to subsample.
    ///
    /// # Safety
    /// Calling this method with either edge ID ranges that do not exist in this
    /// graph or calling this method on a graph without edge weights will cause
    /// this method to panic.
    pub(crate) unsafe fn get_edge_weighted_transitions(
        &self,
        min_edge_id: EdgeT,
        max_edge_id: EdgeT,
        probabilistic_indices: &Option<Vec<u64>>,
    ) -> Vec<WeightT> {
        match &probabilistic_indices {
            Some(indices) => match &self.weights {
                Some(ws) => indices
                    .iter()
                    .map(|edge_id| ws[*edge_id as usize])
                    .collect(),
                // Otherwise we return an uniform vector.
                None => vec![1.0; indices.len()],
            },
            None => match &self.weights {
                Some(ws) => ws[(min_edge_id as usize)..(max_edge_id as usize)].to_vec(),
                // Otherwise we return an uniform vector.
                None => vec![1.0; (max_edge_id - min_edge_id) as usize],
            },
        }
    }

    /// Updates the the transitions probability score for the change of the node type.
    ///
    /// Specifically, we multiply the transition score by the given `change_node_type_weight`
    /// when the node type changes.
    ///
    /// # Arguments
    ///
    /// * `node`: NodeT - Source node.
    /// * `transition`: &mut Vec<WeightT> - Vector of transitions to update.
    /// * `destinations`: impl Iterator<Item = NodeT> - Iterator of the destinations.
    /// * `change_node_type_weight`: ParamsT - The weight to multiply the transition by if there is a change of node type.
    ///
    /// # Safety
    /// If a non-existing node ID is provided, this method may cause an out of bound.
    unsafe fn update_node_transition(
        &self,
        node: NodeT,
        transition: &mut Vec<WeightT>,
        destinations: impl Iterator<Item = NodeT>,
        change_node_type_weight: ParamsT,
    ) {
        //############################################################
        //# Handling of the change node type parameter               #
        //############################################################

        if not_one(change_node_type_weight) {
            // If the node types were given:
            if let Some(nt) = &self.node_types {
                // if the destination node type matches the neighbour
                // destination node type (we are not changing the node type)
                // we weigth using the provided change_node_type_weight weight.

                transition
                    .iter_mut()
                    .zip(destinations)
                    .for_each(|(transition_value, dst)| {
                        if nt.ids[node as usize] != nt.ids[dst as usize] {
                            *transition_value *= change_node_type_weight
                        }
                    });
            }
        }
    }

    /// Return the node transition weights and the related node and edges.
    ///
    /// # Arguments
    ///
    /// * `node`: NodeT, the previous node from which to compute the transitions, if this is bigger that the number of nodes it will panic.
    /// * `walk_weights`: WalkWeights, the weights for the weighted random walks.
    ///
    /// # Safety
    /// If a non-existing node ID is provided, this method may cause an out of bound.
    unsafe fn get_node_transition(
        &self,
        node: NodeT,
        walk_weights: &WalkWeights,
        min_edge_id: EdgeT,
        max_edge_id: EdgeT,
        destinations: &[NodeT],
        probabilistic_indices: &Option<Vec<u64>>,
    ) -> Vec<WeightT> {
        // Retrieve the data to compute the update transition
        let mut transition =
            self.get_edge_weighted_transitions(min_edge_id, max_edge_id, probabilistic_indices);

        // Compute the transition weights relative to the node weights.
        self.update_node_transition(
            node,
            &mut transition,
            destinations.iter().cloned(),
            walk_weights.change_node_type_weight,
        );

        transition
    }

    /// Return the edge transition weights and the related node and edges.
    ///
    /// # Arguments
    ///
    /// * `edge`: EdgeT - the previous edge from which to compute the transitions.
    /// * `weights`: WalkWeights - Weights to use for the weighted walk.
    ///
    /// # Safety
    /// If a non-existing node ID is provided, this method may cause an out of bound.
    unsafe fn get_edge_transition(
        &self,
        src: NodeT,
        dst: NodeT,
        edge_id: EdgeT,
        walk_weights: &WalkWeights,
        min_edge_id: EdgeT,
        max_edge_id: EdgeT,
        destinations: &[NodeT],
        previous_destinations: &[NodeT],
        probabilistic_indices: &Option<Vec<u64>>,
        has_selfloop: bool,
    ) -> (Vec<WeightT>, EdgeT) {
        let mut transition =
            self.get_edge_weighted_transitions(min_edge_id, max_edge_id, probabilistic_indices);

        // Compute the transition weights relative to the node weights.
        self.update_node_transition(
            dst,
            &mut transition,
            destinations.iter().cloned(),
            walk_weights.change_node_type_weight,
        );

        //############################################################
        //# Handling of the change edge type parameter               #
        //############################################################

        // If the edge types were given:
        if not_one(walk_weights.change_edge_type_weight) {
            if let Some(ets) = &self.edge_types {
                //# If the neighbour edge type matches the previous
                //# edge type (we are not changing the edge type)
                //# we weigth using the provided change_edge_type_weight weight.
                let this_type: Option<EdgeTypeT> = ets.ids[edge_id as usize];
                transition
                    .iter_mut()
                    .zip(min_edge_id..max_edge_id)
                    .for_each(|(transition_value, edge_id)| {
                        if this_type == ets.ids[edge_id as usize] {
                            *transition_value /= walk_weights.change_edge_type_weight
                        }
                    });
            }
        }

        //###############################################################
        //# Handling of the P & Q parameters: the node2vec coefficients #
        //###############################################################
        match (
            not_one(walk_weights.return_weight),
            not_one(walk_weights.explore_weight),
        ) {
            (false, false) => {}
            (false, true) => {
                update_explore_weight_transition(
                    &mut transition,
                    destinations,
                    previous_destinations,
                    walk_weights.explore_weight,
                    src,
                    dst,
                );
            }
            (true, false) => {
                update_return_weight_transition(
                    &mut transition,
                    destinations,
                    src,
                    dst,
                    walk_weights.return_weight,
                    has_selfloop,
                );
            }
            (true, true) => {
                update_return_explore_weight_transition(
                    &mut transition,
                    destinations,
                    previous_destinations,
                    walk_weights.return_weight,
                    walk_weights.explore_weight,
                    src,
                    dst,
                );
            }
        }

        (transition, min_edge_id)
    }

    /// Return new sampled node with the transition edge used.
    ///
    /// # Arguments
    ///
    /// * `node`: NodeT, the previous node from which to compute the transitions.
    /// * `random_state`: u64, the random_state to use for extracting the node.
    ///
    /// # Safety
    /// If a non-existing node ID is provided, this method may cause an out of bound.
    unsafe fn extract_uniform_node(&self, node: NodeT, random_state: u64) -> NodeT {
        let (min_edge, max_edge) = self.get_unchecked_minmax_edge_ids_from_source_node_id(node);
        let sampled_offset = sample_uniform((max_edge - min_edge) as u64, random_state);

        self.get_unchecked_destination_node_id_from_edge_id(min_edge + sampled_offset as EdgeT)
    }

    /// Return new sampled node with the transition edge used.
    ///
    /// # Arguments
    ///
    /// * `node`: NodeT, the previous node from which to compute the transitions.
    /// * `random_state`: usize, the random_state to use for extracting the node.
    /// * `walk_weights`: WalkWeights, the weights for the weighted random walks.
    ///
    /// # Safety
    /// If a non-existing node ID is provided, this method may cause an out of bound.
    unsafe fn extract_node(
        &self,
        node: NodeT,
        random_state: u64,
        walk_weights: &WalkWeights,
        min_edge_id: EdgeT,
        max_edge_id: EdgeT,
        destinations: &[NodeT],
        probabilistic_indices: &Option<Vec<u64>>,
    ) -> (NodeT, EdgeT) {
        let mut weights = self.get_node_transition(
            node,
            walk_weights,
            min_edge_id,
            max_edge_id,
            destinations,
            probabilistic_indices,
        );
        let sampled_offset = sample(&mut weights, random_state);
        let edge_id = match probabilistic_indices {
            Some(inds) => inds[sampled_offset],
            None => min_edge_id + sampled_offset as EdgeT,
        };

        (
            self.get_unchecked_destination_node_id_from_edge_id(edge_id),
            edge_id,
        )
    }

    /// Return new random edge with given weights.
    ///
    /// # Arguments
    ///
    /// * `src`: NodeT - Current source node id.
    /// * `dst`: NodeT - Current destination node id.
    /// * `edge`: EdgeT - Current edge id.
    /// * `random_state`: NodeT - The random state to use to sample the next edge id.
    /// * `walk_weights`: &WalkWeights - Struct with the weights to use to update the transitions.
    /// * `min_edge_id`: EdgeT - Minimum edge id to sample for given destination node id.
    /// * `max_edge_id`: EdgeT - Maximum edge id to sample for given destination node id.
    /// * `destinations`: &[NodeT] - Current destinations slice.
    /// * `previous_destinations`: &[NodeT] - Previous destination slice.
    /// * `probabilistic_indices`: &Option<Vec<u64>> - Probabilistic indices, used when max neighbours is provided.
    ///
    /// # Safety
    /// If a non-existing node ID is provided, this method may cause an out of bound.
    unsafe fn extract_edge(
        &self,
        src: NodeT,
        dst: NodeT,
        edge: EdgeT,
        random_state: u64,
        walk_weights: &WalkWeights,
        min_edge_id: EdgeT,
        max_edge_id: EdgeT,
        destinations: &[NodeT],
        previous_destinations: &[NodeT],
        probabilistic_indices: &Option<Vec<u64>>,
    ) -> (NodeT, EdgeT) {
        let (mut weights, min_edge_id) = self.get_edge_transition(
            src,
            dst,
            edge,
            walk_weights,
            min_edge_id,
            max_edge_id,
            destinations,
            previous_destinations,
            probabilistic_indices,
            self.has_selfloops(),
        );
        let sampled_offset = sample(&mut weights, random_state as u64);
        let edge_id = match probabilistic_indices {
            Some(inds) => inds[sampled_offset],
            None => min_edge_id + sampled_offset as EdgeT,
        };
        (
            self.get_unchecked_destination_node_id_from_edge_id(edge_id),
            edge_id,
        )
    }

    /// Return vector of walks run on each non-trap node of the graph.
    ///
    /// # Arguments
    ///
    /// * `quantity`: NodeT - Number of random walk to compute.
    /// * `parameters`: &'a WalksParameters - the weighted walks parameters.
    ///
    /// # Raises
    /// * If the graph does not contain edges.
    /// * If the graph is directed.
    /// * If the given walks parameters are not compatible with the current graph instance.
    pub fn iter_random_walks<'a>(
        &'a self,
        quantity: NodeT,
        parameters: &'a WalksParameters,
    ) -> Result<impl IndexedParallelIterator<Item = Vec<NodeT>> + 'a> {
        self.must_have_edges()?;
        let factor = 0xDEAD;
        let random_state = splitmix64(parameters.random_state.wrapping_mul(factor) as u64);
        self.iter_walk(
            quantity,
            move |index| {
                let local_index = index % quantity;
                let random_source_id =
                    splitmix64(random_state + local_index.wrapping_mul(factor) as u64) as NodeT;
                (
                    splitmix64(random_state + index.wrapping_mul(factor) as u64),
                    unsafe {
                        self.get_unchecked_unique_source_node_id(
                            random_source_id % self.get_unique_source_nodes_number(),
                        )
                    },
                )
            },
            parameters,
        )
    }

    /// Return vector of walks run on a random subset of the not trap nodes.
    ///
    /// # Arguments
    ///
    /// * `parameters`: &'a WalksParameters - the weighted walks parameters.
    ///
    /// # Raises
    /// * If the graph does not contain edges.
    /// * If the graph is directed.
    /// * If the given walks parameters are not compatible with the current graph instance.
    pub fn iter_complete_walks<'a>(
        &'a self,
        parameters: &'a WalksParameters,
    ) -> Result<impl IndexedParallelIterator<Item = Vec<NodeT>> + 'a> {
        self.must_have_edges()?;
        let factor = 0xDEAD;
        let random_state = splitmix64(parameters.random_state.wrapping_mul(factor) as u64);
        self.iter_walk(
            self.get_unique_source_nodes_number(),
            move |index| {
                (
                    splitmix64(random_state + index.wrapping_mul(factor) as u64),
                    unsafe {
                        self.get_unchecked_unique_source_node_id(
                            index as NodeT % self.get_unique_source_nodes_number(),
                        )
                    },
                )
            },
            parameters,
        )
    }

    /// Returns vector of walks.
    ///
    /// # Arguments
    ///
    /// * `parameters`: WalksParameters - the weighted walks parameters.
    ///
    /// # Raises
    /// * If the graph is directed.
    /// * If the given walks parameters are not compatible with the current graph instance.
    /// * If the graph contains negative edge weights.
    fn iter_walk<'a>(
        &'a self,
        quantity: NodeT,
        to_node: impl Fn(NodeT) -> (u64, NodeT) + Sync + Send + 'a,
        parameters: &'a WalksParameters,
    ) -> Result<impl IndexedParallelIterator<Item = Vec<NodeT>> + 'a> {
        self.must_be_undirected()?;
        if self.has_edge_weights() {
            self.must_have_positive_edge_weights()?;
        }

        // Validate if given parameters are compatible with current graph.
        parameters.validate(&self)?;

        let total_iterations = quantity * parameters.iterations;
        info!("Starting random walk.");

        // If the graph does not have any weights and the parameters
        // for the walks are all equal to 1, we can use the first-order
        // random walk algorithm.
        let use_uniform = !self.has_edge_weights() && parameters.is_first_order_walk();

        let walks = (0..total_iterations)
            .into_par_iter()
            .map(move |index| unsafe {
                let (random_state, node) = to_node(index);
                let mut walk = match use_uniform {
                    true => self.uniform_walk(
                        node,
                        random_state,
                        parameters.single_walk_parameters.walk_length,
                    ),
                    false => self.get_unchecked_single_walk(
                        node,
                        random_state,
                        &parameters.single_walk_parameters,
                    ),
                };

                if let Some(dense_node_mapping) = &parameters.dense_node_mapping {
                    walk.iter_mut()
                        .for_each(|node| *node = *dense_node_mapping.get(node).unwrap());
                }
                walk
            });

        Ok(walks)
    }

    /// Returns single walk from given node.
    ///
    /// This method assumes that there are no traps in the graph.
    ///
    /// # Arguments
    /// * `node`: NodeT - Node from where to start the random walks.
    /// * `random_state`: usize, the random_state to use for extracting the nodes and edges.
    /// * `parameters`: SingleWalkParameters - Parameters for the single walk.
    ///
    /// # Safety
    /// If the given node ID does not exists, the method will cause an out of bound.
    unsafe fn get_unchecked_single_walk(
        &self,
        node: NodeT,
        random_state: u64,
        parameters: &SingleWalkParameters,
    ) -> Vec<NodeT> {
        let (min_edge_id, max_edge_id, destinations, indices) = self
            .get_unchecked_edges_and_destinations_from_source_node_id(
                parameters.max_neighbours,
                random_state,
                node,
            );
        let (dst, edge) = self.extract_node(
            node,
            random_state,
            &parameters.weights,
            min_edge_id,
            max_edge_id,
            self.get_destinations_slice(min_edge_id, max_edge_id, &destinations),
            &indices,
        );

        let mut result = Vec::with_capacity(parameters.walk_length as usize);
        result.push(node);
        result.push(dst);
        // We iterate two times before because we need to parse the two initial nodes

        let mut previous_min_edge_id = min_edge_id;
        let mut previous_max_edge_id = max_edge_id;
        let mut previous_destinations = destinations;
        let mut previous_src = node;
        let mut previous_dst = dst;
        let mut previous_edge = edge;

        for i in 2..parameters.walk_length {
            let (min_edge_id, max_edge_id, destinations, indices) = self
                .get_unchecked_edges_and_destinations_from_source_node_id(
                    parameters.max_neighbours,
                    random_state + i,
                    previous_dst,
                );
            let (dst, edge) = self.extract_edge(
                previous_src,
                previous_dst,
                previous_edge,
                random_state + i,
                &parameters.weights,
                min_edge_id,
                max_edge_id,
                self.get_destinations_slice(min_edge_id, max_edge_id, &destinations),
                self.get_destinations_slice(
                    previous_min_edge_id,
                    previous_max_edge_id,
                    &previous_destinations,
                ),
                &indices,
            );

            previous_min_edge_id = min_edge_id;
            previous_max_edge_id = max_edge_id;
            previous_destinations = destinations;
            previous_src = previous_dst;
            previous_dst = dst;
            previous_edge = edge;
            result.push(dst);
        }

        result
    }

    /// Returns single walk iterator from given node.
    ///
    /// This method assumes that there are no traps in the graph.
    ///
    /// # Arguments
    /// * `node`: NodeT - Node from where to start the random walks.
    /// * `random_state`: usize - the random_state to use for extracting the nodes and edges.
    /// * `walk_length`: u64 - Length of the random walk.
    ///
    /// # Safety
    /// If a non-existing node ID is provided, this method may cause an out of bound.
    pub(crate) unsafe fn iter_uniform_walk(
        &self,
        node: NodeT,
        random_state: u64,
        walk_length: u64,
    ) -> impl Iterator<Item = NodeT> + '_ {
        // We iterate one time before because we need to parse the initial node.
        (0..1)
            .map(move |_| node)
            .chain((1..walk_length).scan(node, move |node, iteration| {
                *node = self.extract_uniform_node(*node, random_state + iteration);
                Some(*node)
            }))
    }

    /// Returns single walk vector from given node.
    ///
    /// This method assumes that there are no traps in the graph.
    ///
    /// # Arguments
    /// * `node`: NodeT - Node from where to start the random walks.
    /// * `random_state`: usize - the random_state to use for extracting the nodes and edges.
    /// * `walk_length`: u64 - Length of the random walk.
    ///
    /// # Safety
    /// If a non-existing node ID is provided, this method may cause an out of bound.
    unsafe fn uniform_walk(&self, node: NodeT, random_state: u64, walk_length: u64) -> Vec<NodeT> {
        self.iter_uniform_walk(node, random_state, walk_length)
            .collect()
    }
}
