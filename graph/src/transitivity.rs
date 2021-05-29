use super::*;

/// # Transitivity.
impl Graph {
    /// Returns graph to the i-th transitivity closure iteration.
    ///
    /// # Implementative details
    /// If the given iterations is None, it will return the complete
    /// number of transitivity.
    ///
    /// If the number of iterations given is 0, the method will return
    /// the same graph.
    ///
    /// # Arguments
    /// * `iterations`: Option<NodeT> - The number of iterations of the transitive closure to execute. If None, the complete transitive closure is computed.
    /// * `verbose`: Option<bool> - Whether to show a loading bar while building the graph.
    ///
    pub fn get_transitive_closure(
        &self,
        iterations: Option<NodeT>,
        verbose: Option<bool>,
    ) -> Graph {
        let verbose = verbose.unwrap_or(true);
        Graph::from_integer_unsorted(
            self.iter_node_ids()
                .filter(|src_node_id| unsafe {
                    !self.is_unchecked_singleton_from_node_id(*src_node_id)
                })
                .map(|src_node_id| unsafe {
                    self.get_unchecked_breath_first_search_from_node_ids(
                        src_node_id,
                        None,
                        None,
                        Some(false),
                        Some(false),
                        Some(true),
                        iterations,
                    )
                    .2
                    .unwrap()
                    .into_iter()
                    .enumerate()
                    .filter(|(_, flag)| *flag)
                    .map(move |(dst_node_id, _)| {
                        Ok((src_node_id, dst_node_id as NodeT, None, None))
                    })
                })
                .flatten(),
            self.nodes.clone(),
            self.node_types.clone(),
            self.edge_types.as_ref().map(|ets| ets.vocabulary.clone()),
            self.is_directed(),
            self.get_name(),
            true,
            false,
            false,
            self.has_singleton_nodes(),
            self.has_singleton_nodes_with_selfloops(),
            self.has_trap_nodes(),
            verbose,
        )
        .unwrap()
    }

    /// Returns graph with unweighted shortest paths computed up to the given depth.
    ///
    /// # Implementative details
    /// If the given iterations is None, it will return the complete
    /// sparse matrix of shortest paths.
    ///
    /// If the number of iterations given is 0, the method will return
    /// the same graph.
    ///
    /// # Arguments
    /// * `iterations`: Option<NodeT> - The number of iterations of the transitive closure to execute. If None, the complete transitive closure is computed.
    /// * `verbose`: Option<bool> - Whether to show a loading bar while building the graph.
    ///
    pub fn get_unweighted_all_shortest_paths(
        &self,
        iterations: Option<NodeT>,
        verbose: Option<bool>,
    ) -> Graph {
        let verbose = verbose.unwrap_or(true);
        Graph::from_integer_unsorted(
            self.iter_node_ids()
                .filter(|src_node_id| unsafe {
                    !self.is_unchecked_singleton_from_node_id(*src_node_id)
                })
                .map(|src_node_id| unsafe {
                    self.get_unchecked_breath_first_search_from_node_ids(
                        src_node_id,
                        None,
                        None,
                        Some(true),
                        Some(false),
                        Some(false),
                        iterations,
                    )
                    .0
                    .unwrap()
                    .into_iter()
                    .enumerate()
                    .filter(|(_, distance)| *distance == NodeT::MAX)
                    .map(move |(dst_node_id, distance)| {
                        Ok((
                            src_node_id,
                            dst_node_id as NodeT,
                            None,
                            Some(distance as WeightT),
                        ))
                    })
                })
                .flatten(),
            self.nodes.clone(),
            self.node_types.clone(),
            self.edge_types.as_ref().map(|ets| ets.vocabulary.clone()),
            self.is_directed(),
            self.get_name(),
            true,
            false,
            self.has_edge_weights(),
            self.has_singleton_nodes(),
            self.has_singleton_nodes_with_selfloops(),
            self.has_trap_nodes(),
            verbose,
        )
        .unwrap()
    }

    /// Returns graph with weighted shortest paths computed up to the given depth.
    ///
    /// # Implementative details
    /// If the given iterations is None, it will return the complete
    /// sparse matrix of shortest paths.
    ///
    /// If the number of iterations given is 0, the method will return
    /// the same graph.
    ///
    /// # Arguments
    /// * `iterations`: Option<NodeT> - The number of iterations of the transitive closure to execute. If None, the complete transitive closure is computed.
    /// * `use_edge_weights_as_probabilities`: Option<bool> - Whether to treat the edge weights as probabilities.
    /// * `verbose`: Option<bool> - Whether to show a loading bar while building the graph.
    ///
    pub fn get_weighted_all_shortest_paths(
        &self,
        iterations: Option<NodeT>,
        use_edge_weights_as_probabilities: Option<bool>,
        verbose: Option<bool>,
    ) -> Graph {
        let verbose = verbose.unwrap_or(true);
        Graph::from_integer_unsorted(
            self.iter_node_ids()
                .filter(|src_node_id| unsafe {
                    !self.is_unchecked_singleton_from_node_id(*src_node_id)
                })
                .map(|src_node_id| unsafe {
                    self.get_unchecked_dijkstra_from_node_ids(
                        src_node_id,
                        None,
                        None,
                        Some(true),
                        iterations,
                        use_edge_weights_as_probabilities,
                    )
                    .0
                    .into_iter()
                    .enumerate()
                    .filter(|(_, distance)| distance.is_finite())
                    .map(move |(dst_node_id, distance)| {
                        Ok((
                            src_node_id,
                            dst_node_id as NodeT,
                            None,
                            Some(distance as WeightT),
                        ))
                    })
                })
                .flatten(),
            self.nodes.clone(),
            self.node_types.clone(),
            self.edge_types.as_ref().map(|ets| ets.vocabulary.clone()),
            self.is_directed(),
            self.get_name(),
            true,
            false,
            self.has_edge_weights(),
            self.has_singleton_nodes(),
            self.has_singleton_nodes_with_selfloops(),
            self.has_trap_nodes(),
            verbose,
        )
        .unwrap()
    }
}
