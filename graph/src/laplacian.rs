use super::*;

/// # Generators of laplacian-transformed graphs.
impl Graph {
    /// Returns unweighted laplacian transformation of the graph.
    ///
    /// # Arguments
    /// * `verbose`: Option<bool> - Whether to show a loading bar while building the graph.
    pub fn get_unweighted_laplacian_transformed_graph(&self, verbose: Option<bool>) -> Graph {
        Graph::from_integer_unsorted(
            self.iter_edge_node_ids_and_edge_type_id(true)
                .map(|(_, src, dst, edge_type)| {
                    Ok((
                        src,
                        dst,
                        edge_type,
                        Some(if src == dst {
                            self.get_unchecked_unweighted_node_degree_from_node_id(src) as WeightT
                        } else {
                            -1.0
                        }),
                    ))
                }),
            self.nodes.clone(),
            self.node_types.clone(),
            self.edge_types.as_ref().map(|ets| ets.vocabulary.clone()),
            self.is_directed(),
            self.get_name(),
            true,
            self.has_edge_types(),
            self.has_edge_weights(),
            self.has_singleton_nodes(),
            self.has_singleton_nodes_with_selfloops(),
            self.has_trap_nodes(),
            verbose.unwrap_or(true),
        )
        .unwrap()
    }

    /// Returns unweighted random walk normalized laplacian transformation of the graph.
    ///
    /// # Arguments
    /// * `verbose`: Option<bool> - Whether to show a loading bar while building the graph.
    pub fn get_unweighted_random_walk_normalized_laplacian_transformed_graph(
        &self,
        verbose: Option<bool>,
    ) -> Graph {
        Graph::from_integer_unsorted(
            self.iter_edge_node_ids_and_edge_type_id(true)
                .map(|(_, src, dst, edge_type)| {
                    Ok((
                        src,
                        dst,
                        edge_type,
                        Some(if src == dst {
                            1.0
                        } else {
                            -1.0 / self.get_unchecked_unweighted_node_degree_from_node_id(src)
                                as WeightT
                        }),
                    ))
                }),
            self.nodes.clone(),
            self.node_types.clone(),
            self.edge_types.as_ref().map(|ets| ets.vocabulary.clone()),
            self.is_directed(),
            self.get_name(),
            true,
            self.has_edge_types(),
            self.has_edge_weights(),
            self.has_singleton_nodes(),
            self.has_singleton_nodes_with_selfloops(),
            self.has_trap_nodes(),
            verbose.unwrap_or(true),
        )
        .unwrap()
    }

    /// Returns unweighted symmetric normalized laplacian transformation of the graph.
    ///
    /// # Arguments
    /// * `verbose`: Option<bool> - Whether to show a loading bar while building the graph.
    ///
    /// # Raises
    /// * The graph must be undirected, as we do not currently support this transformation for directed graphs.
    pub fn get_unweighted_symmetric_normalized_laplacian_transformed_graph(
        &self,
        verbose: Option<bool>,
    ) -> Result<Graph, String> {
        self.must_be_undirected()?;
        Graph::from_integer_unsorted(
            self.iter_edge_node_ids_and_edge_type_id(true)
                .map(|(_, src, dst, edge_type)| {
                    Ok((
                        src,
                        dst,
                        edge_type,
                        Some(if src == dst {
                            1.0
                        } else {
                            -1.0 / ((self.get_unchecked_unweighted_node_degree_from_node_id(src)
                                * self.get_unchecked_unweighted_node_degree_from_node_id(dst))
                                as WeightT)
                                .sqrt()
                        }),
                    ))
                }),
            self.nodes.clone(),
            self.node_types.clone(),
            self.edge_types.as_ref().map(|ets| ets.vocabulary.clone()),
            self.is_directed(),
            self.get_name(),
            true,
            self.has_edge_types(),
            self.has_edge_weights(),
            self.has_singleton_nodes(),
            self.has_singleton_nodes_with_selfloops(),
            self.has_trap_nodes(),
            verbose.unwrap_or(true),
        )
    }

    /// Returns unweighted symmetric normalized transformation of the graph.
    ///
    /// # Arguments
    /// * `verbose`: Option<bool> - Whether to show a loading bar while building the graph.
    ///
    /// # Raises
    /// * The graph must be undirected, as we do not currently support this transformation for directed graphs.
    pub fn get_unweighted_symmetric_normalized_transformed_graph(
        &self,
        verbose: Option<bool>,
    ) -> Result<Graph, String> {
        self.must_be_undirected()?;
        Graph::from_integer_unsorted(
            self.iter_edge_node_ids_and_edge_type_id(true)
                .filter(|(_, src, dst, _)| src != dst)
                .map(|(_, src, dst, edge_type)| {
                    Ok((
                        src,
                        dst,
                        edge_type,
                        Some(
                            1.0 / ((self.get_unchecked_unweighted_node_degree_from_node_id(src)
                                * self.get_unchecked_unweighted_node_degree_from_node_id(dst))
                                as WeightT)
                                .sqrt(),
                        ),
                    ))
                }),
            self.nodes.clone(),
            self.node_types.clone(),
            self.edge_types.as_ref().map(|ets| ets.vocabulary.clone()),
            self.is_directed(),
            self.get_name(),
            true,
            self.has_edge_types(),
            self.has_edge_weights(),
            self.has_singleton_nodes(),
            self.has_singleton_nodes_with_selfloops(),
            self.has_trap_nodes(),
            verbose.unwrap_or(true),
        )
    }

    /// Returns weighted laplacian transformation of the graph.
    ///
    /// # Arguments
    /// * `verbose`: Option<bool> - Whether to show a loading bar while building the graph.
    ///
    /// # Raises
    /// * If the graph is not weighted it is not possible to compute the weighted laplacian transformation.
    pub fn get_weighted_laplacian_transformed_graph(
        &self,
        verbose: Option<bool>,
    ) -> Result<Graph, String> {
        self.must_have_edge_weights()?;
        Graph::from_integer_unsorted(
            self.iter_edge_node_ids_and_edge_type_id_and_edge_weight(true)
                .map(|(_, src, dst, edge_type, edge_weight)| {
                    Ok((
                        src,
                        dst,
                        edge_type,
                        Some(if src == dst {
                            self.get_unchecked_weighted_node_degree_from_node_id(src)
                        } else {
                            -edge_weight.unwrap()
                        }),
                    ))
                }),
            self.nodes.clone(),
            self.node_types.clone(),
            self.edge_types.as_ref().map(|ets| ets.vocabulary.clone()),
            self.is_directed(),
            self.get_name(),
            true,
            self.has_edge_types(),
            self.has_edge_weights(),
            self.has_singleton_nodes(),
            self.has_singleton_nodes_with_selfloops(),
            self.has_trap_nodes(),
            verbose.unwrap_or(true),
        )
    }

    /// Returns unweighted symmetric normalized laplacian transformation of the graph.
    ///
    /// # Arguments
    /// * `verbose`: Option<bool> - Whether to show a loading bar while building the graph.
    ///
    /// # Raises
    /// * The graph must be undirected, as we do not currently support this transformation for directed graphs.
    /// * If the graph is not weighted it is not possible to compute the weighted laplacian transformation.
    pub fn get_weighted_symmetric_normalized_laplacian_transformed_graph(
        &self,
        verbose: Option<bool>,
    ) -> Result<Graph, String> {
        self.must_have_edge_weights()?;
        self.must_be_undirected()?;
        Graph::from_integer_unsorted(
            self.iter_edge_node_ids_and_edge_type_id(true)
                .map(|(_, src, dst, edge_type)| {
                    Ok((
                        src,
                        dst,
                        edge_type,
                        Some(if src == dst {
                            1.0
                        } else {
                            -1.0 / ((self.get_unchecked_weighted_node_degree_from_node_id(src)
                                * self.get_unchecked_weighted_node_degree_from_node_id(dst))
                                as WeightT)
                                .sqrt()
                        }),
                    ))
                }),
            self.nodes.clone(),
            self.node_types.clone(),
            self.edge_types.as_ref().map(|ets| ets.vocabulary.clone()),
            self.is_directed(),
            self.get_name(),
            true,
            self.has_edge_types(),
            self.has_edge_weights(),
            self.has_singleton_nodes(),
            self.has_singleton_nodes_with_selfloops(),
            self.has_trap_nodes(),
            verbose.unwrap_or(true),
        )
    }

    /// Returns weighted symmetric normalized transformation of the graph.
    ///
    /// # Arguments
    /// * `verbose`: Option<bool> - Whether to show a loading bar while building the graph.
    ///
    /// # Raises
    /// * The graph must be undirected, as we do not currently support this transformation for directed graphs.
    /// * If the graph is not weighted it is not possible to compute the weighted laplacian transformation.
    pub fn get_weighted_symmetric_normalized_transformed_graph(
        &self,
        verbose: Option<bool>,
    ) -> Result<Graph, String> {
        self.must_have_edge_weights()?;
        self.must_be_undirected()?;
        Graph::from_integer_unsorted(
            self.iter_edge_node_ids_and_edge_type_id(true)
                .filter(|(_, src, dst, _)| src != dst)
                .map(|(_, src, dst, edge_type)| {
                    Ok((
                        src,
                        dst,
                        edge_type,
                        Some(
                            1.0 / (self.get_unchecked_weighted_node_degree_from_node_id(src)
                                * self.get_unchecked_weighted_node_degree_from_node_id(dst))
                            .sqrt(),
                        ),
                    ))
                }),
            self.nodes.clone(),
            self.node_types.clone(),
            self.edge_types.as_ref().map(|ets| ets.vocabulary.clone()),
            self.is_directed(),
            self.get_name(),
            true,
            self.has_edge_types(),
            self.has_edge_weights(),
            self.has_singleton_nodes(),
            self.has_singleton_nodes_with_selfloops(),
            self.has_trap_nodes(),
            verbose.unwrap_or(true),
        )
    }

    /// Returns weighted random walk normalized laplacian transformation of the graph.
    ///
    /// # Arguments
    /// * `verbose`: Option<bool> - Whether to show a loading bar while building the graph.
    ///
    /// # Raises
    /// * The graph must be undirected, as we do not currently support this transformation for directed graphs.
    /// * If the graph is not weighted it is not possible to compute the weighted laplacian transformation.
    pub fn get_weighted_random_walk_normalized_laplacian_transformed_graph(
        &self,
        verbose: Option<bool>,
    ) -> Result<Graph, String> {
        self.must_have_edge_weights()?;
        self.must_be_undirected()?;
        Graph::from_integer_unsorted(
            self.iter_edge_node_ids_and_edge_type_id(true)
                .map(|(_, src, dst, edge_type)| {
                    Ok((
                        src,
                        dst,
                        edge_type,
                        Some(if src == dst {
                            1.0
                        } else {
                            -1.0 / self.get_unchecked_weighted_node_degree_from_node_id(src)
                                as WeightT
                        }),
                    ))
                }),
            self.nodes.clone(),
            self.node_types.clone(),
            self.edge_types.as_ref().map(|ets| ets.vocabulary.clone()),
            self.is_directed(),
            self.get_name(),
            true,
            self.has_edge_types(),
            self.has_edge_weights(),
            self.has_singleton_nodes(),
            self.has_singleton_nodes_with_selfloops(),
            self.has_trap_nodes(),
            verbose.unwrap_or(true),
        )
    }
}
