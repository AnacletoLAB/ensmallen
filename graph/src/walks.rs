use super::*;
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

/*
#[inline(always)]
fn update_explore_weight_transition_directed(
    graph: &Graph,
    transition: &mut Vec<WeightT>,
    destinations: &[NodeT],
    previous_destinations: &[NodeT],
    explore_weight: ParamsT,
    src: NodeT,
    _dst: NodeT,
) {
    for (trans_node, trans_value) in destinations.iter().zip(transition.iter_mut()) {
        // first check if the prev node has the edge to this transition node
        // check src -> trans_node
        if previous_destinations.binary_search(&trans_node).is_ok() {
            *trans_value *= explore_weight;
            continue;
        }
        // otherwise we have to check the neighours of this node
        // check trans_node -> src
        if graph.has_edge_from_node_ids(*trans_node, src) {
            *trans_value *= explore_weight;
        }
    }
} */

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
            Some(indices) => match &*self.weights {
                Some(ws) => indices
                    .iter()
                    .map(|edge_id| ws[*edge_id as usize])
                    .collect(),
                // Otherwise we return an uniform vector.
                None => vec![1.0; indices.len()],
            },
            None => match &*self.weights {
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
    /// * `normalize_by_degree`: bool - Whether to normalize the random walk by the degree of the destination nodes.
    ///
    /// # Safety
    /// If a non-existing node ID is provided, this method may cause an out of bound.
    unsafe fn update_node_transition(
        &self,
        node: NodeT,
        transition: &mut Vec<WeightT>,
        destinations: &[NodeT],
        change_node_type_weight: ParamsT,
        normalize_by_degree: bool,
    ) {
        //############################################################
        //# Handling of the change node type parameter               #
        //############################################################

        if normalize_by_degree {
            transition
                .iter_mut()
                .zip(destinations.iter().cloned())
                .for_each(|(transition_value, dst)| {
                    *transition_value /= self.get_unchecked_node_degree_from_node_id(dst) as f32;
                });
        }

        if not_one(change_node_type_weight) {
            // If the node types were given:
            if let Some(nt) = &*self.node_types {
                // if the destination node type matches the neighbour
                // destination node type (we are not changing the node type)
                // we weigth using the provided change_node_type_weight weight.

                transition
                    .iter_mut()
                    .zip(destinations.iter().cloned())
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
    /// * `normalize_by_degree`: bool - Whether to normalize the random walk by the degree of the destination nodes.
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
        normalize_by_degree: bool,
    ) -> Vec<WeightT> {
        // Retrieve the data to compute the update transition
        let mut transition =
            self.get_edge_weighted_transitions(min_edge_id, max_edge_id, probabilistic_indices);

        // Compute the transition weights relative to the node weights.
        self.update_node_transition(
            node,
            &mut transition,
            destinations,
            walk_weights.change_node_type_weight,
            normalize_by_degree,
        );

        transition
    }

    /// Return the edge transition weights and the related node and edges.
    ///
    /// # Arguments
    ///
    /// * `edge`: EdgeT - the previous edge from which to compute the transitions.
    /// * `weights`: WalkWeights - Weights to use for the weighted walk.
    /// * `normalize_by_degree`: bool - Whether to normalize the random walk by the degree of the destination nodes.
    ///
    /// TODO! Update docstring!
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
        normalize_by_degree: bool,
    ) -> (Vec<WeightT>, EdgeT) {
        let mut transition =
            self.get_edge_weighted_transitions(min_edge_id, max_edge_id, probabilistic_indices);

        // Compute the transition weights relative to the node weights.
        self.update_node_transition(
            dst,
            &mut transition,
            destinations,
            walk_weights.change_node_type_weight,
            normalize_by_degree,
        );

        //############################################################
        //# Handling of the change edge type parameter               #
        //############################################################

        // If the edge types were given:
        if not_one(walk_weights.change_edge_type_weight) {
            if let Some(ets) = &*self.edge_types {
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
        //if !self.is_directed() {
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
        /*
        } else {
            if not_one(walk_weights.return_weight) {
                update_return_weight_transition(
                    &mut transition,
                    destinations,
                    src,
                    dst,
                    walk_weights.return_weight,
                    has_selfloop,
                );
            }
            if not_one(walk_weights.return_weight) {
                update_explore_weight_transition_directed(
                    self,
                    &mut transition,
                    destinations,
                    previous_destinations,
                    walk_weights.explore_weight,
                    src,
                    dst,
                );
            }
        }*/

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
    /// * `normalize_by_degree`: bool - Whether to normalize the random walk by the degree of the destination nodes.
    ///
    /// !TODO: Update docstring!
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
        normalize_by_degree: bool,
    ) -> (NodeT, EdgeT) {
        let mut weights = self.get_node_transition(
            node,
            walk_weights,
            min_edge_id,
            max_edge_id,
            destinations,
            probabilistic_indices,
            normalize_by_degree,
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
    /// * `normalize_by_degree`: bool - Whether to normalize the random walk by the degree of the destination nodes.
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
        normalize_by_degree: bool,
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
            normalize_by_degree,
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
    /// * `quantity`: NodeT - Number of random walk to compute.
    /// * `parameters`: &'a WalksParameters - the weighted walks parameters.
    /// * `random_walks_buffer`: &mut [NodeT] - Buffer where to write the random walks.
    ///
    /// # Raises
    /// * If the graph does not contain edges.
    /// * If the graph is directed.
    /// * If the given walks parameters are not compatible with the current graph instance.
    pub fn populate_random_walks_slice<'a>(
        &'a self,
        quantity: NodeT,
        parameters: &'a WalksParameters,
        random_walks_buffer: &mut [NodeT],
    ) -> Result<()> {
        self.must_have_edges()?;
        let random_state = splitmix64(parameters.random_state as u64);
        self.populate_walks_slice(
            quantity,
            move |index| {
                let local_index = index % quantity;
                let random_source_id = splitmix64(
                    (random_state + local_index as u64).wrapping_add(0x4cc4854c0155130a),
                ) as NodeT;
                (splitmix64(random_state + index as u64), unsafe {
                    self.get_unchecked_unique_source_node_id(
                        random_source_id % self.get_number_of_unique_source_nodes(),
                    )
                })
            },
            parameters,
            random_walks_buffer,
        )
    }

    #[inline(always)]
    /// Return vector of walks run on a random subset of the not trap nodes.
    ///
    /// # Arguments
    /// * `quantity`: NodeT - Number of random walk to compute.
    /// * `parameters`: &'a WalksParameters - the weighted walks parameters.
    ///
    /// # Raises
    /// * If the graph does not contain edges.
    /// * If the graph is directed.
    /// * If the given walks parameters are not compatible with the current graph instance.
    pub fn par_iter_random_walks<'a>(
        &'a self,
        quantity: NodeT,
        parameters: &'a WalksParameters,
    ) -> Result<impl IndexedParallelIterator<Item = Vec<NodeT>> + 'a> {
        self.must_have_edges()?;
        let random_state = splitmix64(parameters.random_state as u64);
        self.par_iter_walks(
            quantity,
            move |index| {
                let local_index = index % quantity;
                let random_source_id = splitmix64(
                    (random_state + local_index as u64).wrapping_add(0x4cc4854c0155130a),
                ) as NodeT;
                (splitmix64(random_state + index as u64), unsafe {
                    self.get_unchecked_unique_source_node_id(
                        random_source_id % self.get_number_of_unique_source_nodes(),
                    )
                })
            },
            parameters,
        )
    }

    #[inline(always)]
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
    pub fn par_iter_complete_walks<'a>(
        &'a self,
        parameters: &'a WalksParameters,
    ) -> Result<impl IndexedParallelIterator<Item = Vec<NodeT>> + 'a> {
        self.must_have_edges()?;
        let random_state = splitmix64(parameters.random_state as u64);
        self.par_iter_walks(
            self.get_number_of_unique_source_nodes(),
            move |index| {
                (splitmix64(random_state + index as u64), unsafe {
                    self.get_unchecked_unique_source_node_id(
                        index as NodeT % self.get_number_of_unique_source_nodes(),
                    )
                })
            },
            parameters,
        )
    }

    /// Return vector of walks run on a random subset of the not trap nodes.
    ///
    /// # Arguments
    /// * `parameters`: &'a WalksParameters - the weighted walks parameters.
    ///
    /// # Raises
    /// * If the graph does not contain edges.
    /// * If the given walks parameters are not compatible with the current graph instance.
    pub fn iter_complete_walks<'a>(
        &'a self,
        parameters: &'a WalksParameters,
    ) -> Result<impl Iterator<Item = Vec<NodeT>> + 'a> {
        self.must_have_edges()?;
        let random_state = splitmix64(parameters.random_state as u64);
        self.iter_walks(
            self.get_number_of_unique_source_nodes(),
            move |index| {
                (splitmix64(random_state + index as u64), unsafe {
                    self.get_unchecked_unique_source_node_id(
                        index as NodeT % self.get_number_of_unique_source_nodes(),
                    )
                })
            },
            parameters,
        )
    }

    /// Returns vector of walks.
    ///
    /// # Arguments
    /// * `quantity`: NodeT - Number of random walks to generate.
    /// * `to_node`: impl Fn(NodeT) -> (u64, NodeT) + Sync + Send + 'a - Closure to use to sampled nodes.
    /// * `parameters`: WalksParameters - the weighted walks parameters.
    ///
    /// # Raises
    /// * If the graph is directed.
    /// * If the given walks parameters are not compatible with the current graph instance.
    /// * If the graph contains negative edge weights.
    fn par_iter_walks<'a>(
        &'a self,
        quantity: NodeT,
        to_node: impl Fn(NodeT) -> (u64, NodeT) + Sync + Send + 'a,
        parameters: &'a WalksParameters,
    ) -> Result<impl IndexedParallelIterator<Item = Vec<NodeT>> + 'a> {
        if self.has_edge_weights() {
            self.must_have_positive_edge_weights()?;
        }

        // Validate if given parameters are compatible with current graph.
        parameters.validate(&self)?;

        let total_iterations = quantity * parameters.iterations;

        // If the graph does not have any weights and the parameters
        // for the walks are all equal to 1, we can use the first-order
        // random walk algorithm.
        let use_uniform = (!self.has_edge_weights() || self.has_constant_edge_weights()?)
            && parameters.is_first_order_walk();

        let walks = (0..total_iterations)
            .into_par_iter()
            .map(move |index| unsafe {
                let (random_state, node) = to_node(index);
                let mut walk_buffer =
                    vec![0; parameters.single_walk_parameters.walk_length as usize];
                match use_uniform {
                    true => self.uniform_walk_from_slice(
                        node,
                        random_state,
                        parameters.single_walk_parameters.walk_length,
                        &mut walk_buffer,
                    ),
                    false => self.get_unchecked_single_walk_from_slice(
                        node,
                        random_state,
                        &parameters.single_walk_parameters,
                        &mut walk_buffer,
                    ),
                };

                walk_buffer
            });

        Ok(walks)
    }

    /// Returns vector of walks.
    ///
    /// # Arguments
    /// * `quantity`: NodeT - Number of random walks to generate.
    /// * `to_node`: impl Fn(NodeT) -> (u64, NodeT) + Sync + Send + 'a - Closure to use to sampled nodes.
    /// * `parameters`: WalksParameters - the weighted walks parameters.
    ///
    /// # Raises
    /// * If the given walks parameters are not compatible with the current graph instance.
    /// * If the graph contains negative edge weights.
    fn iter_walks<'a>(
        &'a self,
        quantity: NodeT,
        to_node: impl Fn(NodeT) -> (u64, NodeT) + Sync + Send + 'a,
        parameters: &'a WalksParameters,
    ) -> Result<impl Iterator<Item = Vec<NodeT>> + 'a> {
        if self.has_edge_weights() {
            self.must_have_positive_edge_weights()?;
        }

        // Validate if given parameters are compatible with current graph.
        parameters.validate(&self)?;

        let total_iterations = quantity * parameters.iterations;

        // If the graph does not have any weights and the parameters
        // for the walks are all equal to 1, we can use the first-order
        // random walk algorithm.
        let use_uniform = !self.has_edge_weights() && parameters.is_first_order_walk();

        let walks = (0..total_iterations).map(move |index| unsafe {
            let (random_state, node) = to_node(index);
            let mut walk_buffer = vec![0; parameters.single_walk_parameters.walk_length as usize];
            match use_uniform {
                true => self.uniform_walk_from_slice(
                    node,
                    random_state,
                    parameters.single_walk_parameters.walk_length,
                    &mut walk_buffer,
                ),
                false => self.get_unchecked_single_walk_from_slice(
                    node,
                    random_state,
                    &parameters.single_walk_parameters,
                    &mut walk_buffer,
                ),
            };

            walk_buffer
        });

        Ok(walks)
    }

    /// Returns vector of walks.
    ///
    /// # Arguments
    /// * `quantity`: NodeT - Number of random walks to generate.
    /// * `to_node`: impl Fn(NodeT) -> (u64, NodeT) + Sync + Send + 'a - Closure to use to sampled nodes.
    /// * `parameters`: WalksParameters - the weighted walks parameters.
    /// * `random_walks_buffer`: &mut [NodeT] - Buffer where to write the random walks.
    ///
    /// # Raises
    /// * If the given walks parameters are not compatible with the current graph instance.
    /// * If the graph contains negative edge weights.
    fn populate_walks_slice<'a>(
        &'a self,
        quantity: NodeT,
        to_node: impl Fn(NodeT) -> (u64, NodeT) + Sync + Send + 'a,
        parameters: &'a WalksParameters,
        random_walks_buffer: &mut [NodeT],
    ) -> Result<()> {
        if self.has_edge_weights() {
            self.must_have_positive_edge_weights()?;
        }

        // Validate if given parameters are compatible with current graph.
        parameters.validate(&self)?;

        let total_iterations = quantity * parameters.iterations;

        // If the graph does not have any weights and the parameters
        // for the walks are all equal to 1, we can use the first-order
        // random walk algorithm.
        let use_uniform = !self.has_edge_weights() && parameters.is_first_order_walk();

        (0..total_iterations)
            .into_par_iter()
            .zip(random_walks_buffer.par_chunks_mut(parameters.get_random_walk_length() as usize))
            .for_each(move |(index, walk_buffer)| unsafe {
                let (random_state, node) = to_node(index);
                match use_uniform {
                    true => self.uniform_walk_from_slice(
                        node,
                        random_state,
                        parameters.single_walk_parameters.walk_length,
                        walk_buffer,
                    ),
                    false => self.get_unchecked_single_walk_from_slice(
                        node,
                        random_state,
                        &parameters.single_walk_parameters,
                        walk_buffer,
                    ),
                };
            });
        Ok(())
    }

    /// Returns slice of destinations corresponding to given minmax edge ID and node.
    ///
    /// # Arguments
    /// * `min_edge_id`: EdgeT - Minimum edge ID for the slice.
    /// * `max_edge_id`: EdgeT - Maximum edge ID for the slice.
    /// * `source_node_id`: NodeT - The source node ID.
    /// * `destinations`: &'a Option<Vec<NodeT>> - The optional destinations slice that may have been provided when working with subsampling.
    fn get_destinations_slice<'a>(
        &'a self,
        min_edge_id: EdgeT,
        max_edge_id: EdgeT,
        destinations: &'a Option<Vec<NodeT>>,
    ) -> &'a [NodeT] {
        // TODO! This is stupid and can be done much faster by removing the call to
        // get_unchecked_edges_and_destinations_from_source_node_id
        match destinations {
            Some(dsts) => dsts.as_slice(),
            None => &self.edges.destinations[min_edge_id as usize..max_edge_id as usize],
        }
    }

    /// Returns single walk from given node.
    ///
    /// This method assumes that there are no traps in the graph.
    ///
    /// # Arguments
    /// * `node`: NodeT - Node from where to start the random walks.
    /// * `random_state`: usize, the random_state to use for extracting the nodes and edges.
    /// * `parameters`: SingleWalkParameters - Parameters for the single walk.
    /// * `walk_buffer`: &mut [NodeT] - Buffer where to write the random walk.
    ///
    /// # Safety
    /// If the given node ID does not exists, the method will cause an out of bound.
    unsafe fn get_unchecked_single_walk_from_slice(
        &self,
        node: NodeT,
        mut random_state: u64,
        parameters: &SingleWalkParameters,
        walk_buffer: &mut [NodeT],
    ) {
        let (min_edge_id, max_edge_id, destinations, indices) = self
            .get_unchecked_edges_and_destinations_from_source_node_id(
                parameters.max_neighbours,
                random_state,
                node,
            );
        random_state = splitmix64(random_state);
        let (dst, edge) = self.extract_node(
            node,
            random_state,
            &parameters.weights,
            min_edge_id,
            max_edge_id,
            self.get_destinations_slice(min_edge_id, max_edge_id, &destinations),
            &indices,
            parameters.normalize_by_degree,
        );

        // Here we use the get unchecked mut and the get unchecked
        // because we need to avoid multiple bound checks, which
        // would be useless as we are allocating the correct
        // size of this vector.
        *walk_buffer.get_unchecked_mut(0) = node;
        *walk_buffer.get_unchecked_mut(1) = dst;

        // We iterate two times before because we need to parse the two initial nodes
        let mut previous_min_edge_id = min_edge_id;
        let mut previous_max_edge_id = max_edge_id;
        let mut previous_destinations = destinations;
        let mut previous_src = node;
        let mut previous_dst = dst;
        let mut previous_edge = edge;

        for iteration in 2..parameters.walk_length {
            random_state = splitmix64(random_state);
            let (min_edge_id, max_edge_id, destinations, indices) = self
                .get_unchecked_edges_and_destinations_from_source_node_id(
                    parameters.max_neighbours,
                    random_state,
                    previous_dst,
                );
            random_state = splitmix64(random_state);
            let (dst, edge) = self.extract_edge(
                previous_src,
                previous_dst,
                previous_edge,
                random_state,
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
                parameters.normalize_by_degree,
            );

            previous_min_edge_id = min_edge_id;
            previous_max_edge_id = max_edge_id;
            previous_destinations = destinations;
            previous_src = previous_dst;
            previous_dst = dst;
            previous_edge = edge;
            *walk_buffer.get_unchecked_mut(iteration as usize) = dst;
        }
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
    pub unsafe fn iter_uniform_walk(
        &self,
        node: NodeT,
        random_state: u64,
        walk_length: u64,
    ) -> impl Iterator<Item = NodeT> + '_ {
        // We iterate one time before because we need to parse the initial node.
        (0..1)
            .map(move |_| node)
            .chain((1..walk_length).scan(node, move |node, iteration| {
                *node = self.extract_uniform_node(*node, splitmix64(random_state + iteration));
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
    /// * `walk_buffer`: &mut [NodeT] - Slice where to store the random walk.
    ///
    /// # Safety
    /// If a non-existing node ID is provided, this method may cause an out of bound.
    unsafe fn uniform_walk_from_slice(
        &self,
        node: NodeT,
        random_state: u64,
        walk_length: u64,
        walk_buffer: &mut [NodeT],
    ) {
        walk_buffer[0] = node;
        (1..walk_length).for_each(move |iteration| {
            // Here we use the get unchecked mut and the get unchecked
            // because we need to avoid multiple bound checks, which
            // would be useless as we are allocating the correct
            // size of this vector.
            *walk_buffer.get_unchecked_mut(iteration as usize) = self.extract_uniform_node(
                *walk_buffer.get_unchecked(iteration as usize - 1),
                splitmix64(random_state + iteration),
            );
        });
    }
}
