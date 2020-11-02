use super::*;
use log::info;
use rayon::prelude::*;
use vec_rand::xorshift::xorshift;
use vec_rand::sample_uniform;
use vec_rand::sample_f32 as sample;

#[inline(always)]
fn update_return_weight_transition(
    transition: &mut Vec<WeightT>,
    destinations: &[NodeT],
    src: NodeT,
    dst: NodeT,
    return_weight: ParamsT,
) {
    if let Ok(mut i) = destinations.binary_search(&src) {
        let mut j = i;
        while j > 0 && destinations[j] == src {
            transition[j] *= return_weight;
            j -= 1;
        }
        while i < destinations.len() && destinations[i] == src {
            transition[i] *= return_weight;
            i += 1;
        }
    }
    if src != dst {
        if let Ok(mut i) = destinations.binary_search(&dst) {
            let mut j = i;
            while j > 0 && destinations[j] == src {
                transition[j] *= return_weight;
                j -= 1;
            }
            while i < destinations.len() && destinations[i] == dst {
                transition[i] *= return_weight;
                i += 1;
            }
        }
    }
}

#[inline(always)]
fn update_explore_weight_transition(
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
fn update_return_explore_weight_transition(
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

#[cfg(test)]
mod tests {
    use super::WeightT;
    use super::update_explore_weight_transition;
    use super::update_return_weight_transition;
    #[test]
    fn test_update_explore_weight_transition() {
        let destinations = vec![1, 2, 3, 4, 4, 4, 5, 6, 100];
        let previous_destinations = vec![2, 4, 4, 4];
        let mut transitions = (0..destinations.len()).map(|_| 1.0).collect::<Vec<WeightT>>();
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
            vec![2.0, 1.0, 2.0, 1.0, 1.0, 1.0, 2.0, 1.0, 1.0]
        )
    }

    #[test]
    fn test_update_return_weight_transition() {
        let destinations = vec![1, 2, 3, 4, 4, 4, 5, 6, 100];
        let mut transitions = (0..destinations.len()).map(|_| 1.0).collect::<Vec<WeightT>>();
        update_return_weight_transition(&mut transitions, &destinations, 6, 2, 2.0);
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
    ///
    /// * min_edge_id: EdgeT - The minimum edge id.
    /// * max_edge_id: EdgeT - The maximum edge id.
    ///
    fn get_weighted_transitions(&self, min_edge_id: EdgeT, max_edge_id: EdgeT) -> Vec<WeightT> {
        match &self.weights {
            // If the graph is weighted we return the weights
            Some(ws) => ws[(min_edge_id as usize)..(max_edge_id as usize)].to_vec(),
            // Otherwise we return an uniform vector.
            None => vec![1.0; (max_edge_id - min_edge_id) as usize],
        }
    }

    /// TODO: Update docstring!
    fn update_node_transition(
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
                let this_type: NodeTypeT = nt.ids[node as usize];

                transition
                    .iter_mut()
                    .zip(destinations)
                    .for_each(|(transition_value, dst)| {
                        if this_type == nt.ids[dst as usize] {
                            *transition_value /= change_node_type_weight
                        }
                    });
            }
        }
    }

    /// Return the node transition weights and the related node and edges.
    ///
    /// # Arguments
    ///
    /// * node: NodeT, the previous node from which to compute the transitions, if this is bigger that the number of nodes it will panic.
    /// * walk_weights: WalkWeights, the weights for the weighted random walks.
    ///
    fn get_node_transition(
        &self,
        node: NodeT,
        walk_weights: &WalkWeights,
        min_edge_id: EdgeT,
        max_edge_id: EdgeT,
        destinations: &[NodeT],
    ) -> Vec<WeightT> {
        // Retrieve the data to compute the update transition
        let mut transition = self.get_weighted_transitions(min_edge_id, max_edge_id);

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
    /// * edge: EdgeT - the previous edge from which to compute the transitions.
    /// * weights: WalkWeights - Weights to use for the weighted walk.
    fn get_edge_transition(
        &self,
        src: NodeT,
        dst: NodeT,
        edge_id: EdgeT,
        walk_weights: &WalkWeights,
        min_edge_id: EdgeT,
        max_edge_id: EdgeT,
        destinations: &[NodeT],
        previous_destinations: &[NodeT],
    ) -> (Vec<WeightT>, EdgeT) {
        let mut transition = self.get_weighted_transitions(min_edge_id, max_edge_id);

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
                let this_type: EdgeTypeT = ets.ids[edge_id as usize];
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
        let has_q = not_one(walk_weights.explore_weight);
        let has_p = not_one(walk_weights.return_weight);
        if has_p && has_q {
            update_return_explore_weight_transition(
                &mut transition,
                destinations,
                previous_destinations,
                walk_weights.return_weight,
                walk_weights.explore_weight,
                src,
                dst,
            );
        } else {
            if has_q {
                update_explore_weight_transition(
                    &mut transition,
                    destinations,
                    previous_destinations,
                    walk_weights.explore_weight,
                    src,
                    dst,
                );
            }

            if has_p {
                update_return_weight_transition(
                    &mut transition,
                    destinations,
                    src,
                    dst,
                    walk_weights.return_weight,
                );
            }
        }

        (transition, min_edge_id)
    }

    /// Return new sampled node with the transition edge used.
    ///
    /// # Arguments
    ///
    /// * node: NodeT, the previous node from which to compute the transitions.
    /// * random_state: usize, the random_state to use for extracting the node.
    ///
    pub fn extract_uniform_node(&self, node: NodeT, random_state: NodeT) -> NodeT {
        let (min_edge, max_edge) = self.get_destinations_min_max_edge_ids(node);
        self.get_destination(
            min_edge + sample_uniform((max_edge - min_edge) as u64, random_state as u64) as EdgeT,
        )
    }

    /// Return new sampled node with the transition edge used.
    ///
    /// # Arguments
    ///
    /// * node: NodeT, the previous node from which to compute the transitions.
    /// * random_state: usize, the random_state to use for extracting the node.
    /// * walk_weights: WalkWeights, the weights for the weighted random walks.
    pub fn extract_node(
        &self,
        node: NodeT,
        random_state: NodeT,
        walk_weights: &WalkWeights,
        min_edge_id: EdgeT,
        max_edge_id: EdgeT,
        destinations: &[NodeT],
    ) -> (NodeT, EdgeT) {
        let mut weights =
            self.get_node_transition(node, walk_weights, min_edge_id, max_edge_id, destinations);
        let edge_id = min_edge_id + sample(&mut weights, random_state as u64) as EdgeT;
        (self.get_destination(edge_id), edge_id)
    }

    /// Return new random edge with given weights.
    ///
    /// # Arguments
    ///
    /// * edge: EdgeT, the previous edge from which to compute the transitions.
    /// * random_state: usize, the random_state to use for extracting the node.
    /// * walk_weights: WalkWeights, the weights for the weighted random walks.
    pub fn extract_edge(
        &self,
        src: NodeT,
        dst: NodeT,
        edge: EdgeT,
        random_state: NodeT,
        walk_weights: &WalkWeights,
        min_edge_id: EdgeT,
        max_edge_id: EdgeT,
        destinations: &[NodeT],
        previous_destinations: &[NodeT],
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
        );
        let edge_id = min_edge_id + sample(&mut weights, random_state as u64) as EdgeT;
        (self.get_destination(edge_id), edge_id)
    }

    /// Return vector of walks run on each non-trap node of the graph.
    ///
    /// # Arguments
    ///
    /// * parameters: WalksParameters - the weighted walks parameters.
    ///
    pub fn random_walks_iter<'a>(
        &'a self,
        quantity: NodeT,
        parameters: &'a WalksParameters,
    ) -> Result<impl IndexedParallelIterator<Item = Vec<NodeT>> + 'a, String> {
        self.walk_iter(
            quantity,
            move |global_index| {
                let local_index = global_index % quantity;
                let random_source_id =
                    xorshift((parameters.random_state + local_index as NodeT) as u64) as NodeT;
                (
                    random_source_id as NodeT,
                    self.get_unique_source(random_source_id),
                )
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
    pub fn complete_walks_iter<'a>(
        &'a self,
        parameters: &'a WalksParameters,
    ) -> Result<impl IndexedParallelIterator<Item = Vec<NodeT>> + 'a, String> {
        self.walk_iter(
            self.get_unique_sources_number(),
            move |source_id| (source_id, self.get_unique_source(source_id as NodeT)),
            parameters,
        )
    }

    /// Returns vector of walks.
    ///
    /// # Arguments
    ///
    /// * parameters: WalksParameters - the weighted walks parameters.
    ///
    pub fn walk_iter<'a>(
        &'a self,
        quantity: NodeT,
        to_node: impl Fn(NodeT) -> (NodeT, NodeT) + Sync + Send + 'a,
        parameters: &'a WalksParameters,
    ) -> Result<impl IndexedParallelIterator<Item = Vec<NodeT>> + 'a, String> {
        if self.directed {
            return Err("Not supporting directed walks as of now.".to_owned());
        }

        // Validate if given parameters are compatible with current graph.
        parameters.validate(&self)?;

        let total_iterations = quantity * parameters.iterations;
        info!("Starting random walk.");

        let use_uniform = !self.has_weights() && parameters.is_first_order_walk();

        let walks = (0..total_iterations).into_par_iter().map(move |index| {
            let (random_state, node) = to_node(index);
            let mut walk = match use_uniform {
                true => self.uniform_walk(node, random_state, &parameters.single_walk_parameters),
                false => self.single_walk(node, random_state, &parameters.single_walk_parameters),
            };

            if let Some(dense_node_mapping) = &parameters.dense_node_mapping {
                walk.iter_mut()
                    .for_each(|node| *node = *dense_node_mapping.get(node).unwrap());
            }
            walk
        });

        Ok(walks)
    }

    fn get_node_edges_and_destinations(&self, node: NodeT) -> (EdgeT, EdgeT, Option<Vec<NodeT>>) {
        let (min_edge_id, max_edge_id) = self.get_destinations_min_max_edge_ids(node);
        let destinations: Option<Vec<NodeT>> =
            match (&self.cached_destinations, &self.destinations) {
                (Some(cached_destinations), None) => match cached_destinations.contains_key(&node) {
                    true => None,
                    false => Some(
                        self.get_destinations_range(min_edge_id, max_edge_id)
                            .collect(),
                    ),
                },
                (None, Some(_)) => None,
                (None, None) => Some(
                    self.get_destinations_range(min_edge_id, max_edge_id)
                        .collect(),
                ),
                _ => unreachable!("Cached destinations cannot exists alongside destinations!")
            };
        (min_edge_id, max_edge_id, destinations)
    }

    fn get_destinations_slice<'a>(
        &'a self,
        min_edge_id: EdgeT,
        max_edge_id: EdgeT,
        node: NodeT,
        destinations: &'a Option<Vec<NodeT>>,
    ) -> &'a [NodeT] {
        match (&self.destinations, &self.cached_destinations, destinations) {
            (_, _, Some(dsts)) => &dsts.as_slice(),
            (Some(dsts), None, None) => &dsts[min_edge_id as usize..max_edge_id as usize],
            (None, Some(dsts), None) => dsts.get(&node).unwrap(),
            _ => unreachable!("It is not possible to have both destinations and cached destinations at once."),
        }
    }

    /// Returns single walk from given node.
    ///
    /// This method assumes that there are no traps in the graph.
    ///
    /// # Arguments
    ///
    /// * node: NodeT - Node from where to start the random walks.
    /// * random_state: usize, the random_state to use for extracting the nodes and edges.
    /// * parameters: SingleWalkParameters - Parameters for the single walk.
    ///
    pub fn single_walk(
        &self,
        node: NodeT,
        random_state: NodeT,
        parameters: &SingleWalkParameters,
    ) -> Vec<NodeT> {
        let mut walk: Vec<NodeT> = Vec::with_capacity(parameters.length as usize);
        walk.push(node);
        let mut src = node;
        let (mut min_edge_id, mut max_edge_id, mut destinations) = self.get_node_edges_and_destinations(node);
        let mut previous_destinations: Option<Vec<NodeT>>;
        let mut previous_min_edge_id: EdgeT;
        let mut previous_max_edge_id: EdgeT;
        let (mut dst, mut edge) = self.extract_node(
            node,
            random_state,
            &parameters.weights,
            min_edge_id,
            max_edge_id,
            self.get_destinations_slice(min_edge_id, max_edge_id, node, &destinations),
        );
        walk.push(dst);

        for iteration in 2..parameters.length {
            previous_destinations = destinations;
            previous_min_edge_id = min_edge_id;
            previous_max_edge_id = max_edge_id;
            let (new_min_edge_id, new_max_edge_id, new_destinations) = self.get_node_edges_and_destinations(dst);
            min_edge_id = new_min_edge_id;
            max_edge_id = new_max_edge_id;
            destinations = new_destinations;
            let (new_dst, inner_edge) = self.extract_edge(
                src,
                dst,
                edge,
                random_state + iteration,
                &parameters.weights,
                min_edge_id,
                max_edge_id,
                self.get_destinations_slice(min_edge_id, max_edge_id, dst, &destinations),
                self.get_destinations_slice(previous_min_edge_id, previous_max_edge_id, src, &previous_destinations),
            );
            src = dst;
            dst = new_dst;
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
    /// * random_state: usize, the random_state to use for extracting the nodes and edges.
    /// * parameters: SingleWalkParameters - Parameters for the single walk.
    ///
    fn uniform_walk(
        &self,
        node: NodeT,
        random_state: NodeT,
        parameters: &SingleWalkParameters,
    ) -> Vec<NodeT> {
        let mut walk: Vec<NodeT> = Vec::with_capacity(parameters.length as usize);
        let mut dst = self.extract_uniform_node(node, random_state);
        walk.push(node);
        walk.push(dst);

        for iteration in 2..parameters.length {
            dst = self.extract_uniform_node(dst, random_state + iteration);
            walk.push(dst);
        }
        walk
    }
}
