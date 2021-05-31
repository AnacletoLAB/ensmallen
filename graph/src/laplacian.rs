use indicatif::ProgressIterator;
use num_traits::Zero;
use rayon::iter::ParallelIterator;

use super::*;

/// # Generators of laplacian-transformed graphs.
impl Graph {
    /// Returns unweighted laplacian transformation of the graph.
    ///
    /// # Arguments
    /// * `verbose`: Option<bool> - Whether to show a loading bar while building the graph.
    pub fn get_unweighted_laplacian_transformed_graph(&self, verbose: Option<bool>) -> Graph {
        Graph::from_integer_unsorted(
            self.par_iter_edge_node_ids_and_edge_type_id(true)
                .map(|(_, src, dst, edge_type)| {
                    Ok((
                        src,
                        dst,
                        edge_type,
                        Some(if src == dst {
                            unsafe {
                                self.get_unchecked_unweighted_node_degree_from_node_id(src)
                                    as WeightT
                            }
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
            true,
            false,
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
            self.par_iter_edge_node_ids_and_edge_type_id(true)
                .map(|(_, src, dst, edge_type)| {
                    Ok((
                        src,
                        dst,
                        edge_type,
                        Some(if src == dst {
                            1.0
                        } else {
                            -1.0 / unsafe {
                                self.get_unchecked_unweighted_node_degree_from_node_id(src)
                            } as WeightT
                        }),
                    ))
                }),
            self.nodes.clone(),
            self.node_types.clone(),
            self.edge_types.as_ref().map(|ets| ets.vocabulary.clone()),
            true,
            self.get_name(),
            true,
            self.has_edge_types(),
            true,
            false,
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
            self.par_iter_edge_node_ids_and_edge_type_id(true).map(
                |(_, src, dst, edge_type)| unsafe {
                    Ok((
                        src,
                        dst,
                        edge_type,
                        Some(if src == dst {
                            1.0
                        } else {
                            -1.0 / (self.get_unchecked_unweighted_node_degree_from_node_id(src)
                                as f64
                                * self.get_unchecked_unweighted_node_degree_from_node_id(dst)
                                    as f64)
                                .sqrt() as WeightT
                        }),
                    ))
                },
            ),
            self.nodes.clone(),
            self.node_types.clone(),
            self.edge_types.as_ref().map(|ets| ets.vocabulary.clone()),
            self.is_directed(),
            self.get_name(),
            true,
            self.has_edge_types(),
            true,
            false,
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
            self.par_iter_edge_node_ids_and_edge_type_id(true)
                .filter(|(_, src, dst, _)| src != dst)
                .map(|(_, src, dst, edge_type)| unsafe {
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
            true,
            false,
            self.has_singleton_nodes() || self.has_singleton_nodes_with_selfloops(),
            false,
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
        self.must_not_contain_weighted_singleton_nodes()?;
        Graph::from_integer_unsorted(
            self.par_iter_edge_node_ids_and_edge_type_id_and_edge_weight(true)
                .filter_map(|(_, src, dst, edge_type, edge_weight)| unsafe {
                    let weight = if src == dst {
                        self.get_unchecked_weighted_node_degree_from_node_id(src) as WeightT
                    } else {
                        -edge_weight.unwrap()
                    };
                    if weight.is_zero() || weight.is_infinite() {
                        return None;
                    }
                    Some(Ok((src, dst, edge_type, Some(weight))))
                }),
            self.nodes.clone(),
            self.node_types.clone(),
            self.edge_types.as_ref().map(|ets| ets.vocabulary.clone()),
            self.is_directed(),
            self.get_name(),
            true,
            self.has_edge_types(),
            true,
            false,
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
        self.must_not_contain_weighted_singleton_nodes()?;
        Graph::from_integer_unsorted(
            self.par_iter_edge_node_ids_and_edge_type_id(true)
                .filter_map(|(_, src, dst, edge_type)| unsafe {
                    let weight = if src == dst {
                        1.0
                    } else {
                        (-1.0
                            / (self.get_unchecked_weighted_node_degree_from_node_id(src)
                                * self.get_unchecked_weighted_node_degree_from_node_id(dst))
                            .sqrt()) as WeightT
                    };
                    if weight.is_zero() || weight.is_infinite() {
                        return None;
                    }
                    Some(Ok((src, dst, edge_type, Some(weight))))
                }),
            self.nodes.clone(),
            self.node_types.clone(),
            self.edge_types.as_ref().map(|ets| ets.vocabulary.clone()),
            self.is_directed(),
            self.get_name(),
            true,
            self.has_edge_types(),
            true,
            false,
            // This method may produce singletons though to
            // problems of numerical instability, even though if either
            // weights are in 'normal' ranges it will not happen or
            // if the type used for the weights used will have, in the future
            // use a higher resolution, like an f64.
            true,
            // As per above, also this might introduce singletons
            // in the presence of numerical instability.
            true,
            // As per above, also this might introduce singletons
            // in the presence of numerical instability.
            true,
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
        self.must_be_undirected()?;
        self.must_not_contain_weighted_singleton_nodes()?;
        let weighted_node_degrees = self.get_weighted_node_degrees()?;
        let loading_bar = get_loading_bar(
            verbose.unwrap_or(true),
            "Building weighted symmetric normalized transformed graph",
            self.get_directed_edges_number() as usize,
        );
        Graph::from_integer_sorted(
            self.iter_edge_node_ids_and_edge_type_id(true)
                .progress_with(loading_bar)
                .filter_map(|(_, src, dst, edge_type)| {
                    if src == dst {
                        return None;
                    }
                    let distance = (1.0
                        / (weighted_node_degrees[src as usize]
                            * weighted_node_degrees[dst as usize])
                            .sqrt()) as WeightT;
                    if distance.is_finite() && !distance.is_zero() {
                        Some(Ok((src, dst, edge_type, Some(distance))))
                    } else {
                        None
                    }
                }),
            (self.get_directed_edges_number() - self.get_selfloop_nodes_number()) as usize,
            self.nodes.clone(),
            self.node_types.clone(),
            self.edge_types.as_ref().map(|ets| ets.vocabulary.clone()),
            self.is_directed(),
            true,
            self.get_name(),
            false,
            self.has_edge_types(),
            true,
            false,
            true,
            false,
            true,
        )
    }

    /// Returns weighted random walk normalized laplacian transformation of the graph.
    ///
    /// # Arguments
    /// * `verbose`: Option<bool> - Whether to show a loading bar while building the graph.
    ///
    /// # Raises
    /// * If the graph is not weighted it is not possible to compute the weighted laplacian transformation.
    /// * If the graph contains nodes with zero weighted degree.
    pub fn get_weighted_random_walk_normalized_laplacian_transformed_graph(
        &self,
        verbose: Option<bool>,
    ) -> Result<Graph, String> {
        self.must_have_edge_weights()?;
        self.must_not_contain_weighted_singleton_nodes()?;
        Graph::from_integer_unsorted(
            self.par_iter_edge_node_ids_and_edge_type_id(true)
                .filter_map(|(_, src, dst, edge_type)| unsafe {
                    let weight = if src == dst {
                        1.0
                    } else {
                        -1.0 / self.get_unchecked_weighted_node_degree_from_node_id(src) as WeightT
                    };
                    // Even if we do the weighted singleton nodes check,
                    // we may still endup with infinities though to numerical
                    // instability in the conversion from f64 to f32.
                    if weight.is_zero() || weight.is_infinite() {
                        return None;
                    }
                    Some(Ok((src, dst, edge_type, Some(weight))))
                }),
            self.nodes.clone(),
            self.node_types.clone(),
            self.edge_types.as_ref().map(|ets| ets.vocabulary.clone()),
            true,
            self.get_name(),
            true,
            self.has_edge_types(),
            true,
            false,
            // This method may produce singletons though to
            // problems of numerical instability, even though if either
            // weights are in 'normal' ranges it will not happen or
            // if the type used for the weights used will have, in the future
            // use a higher resolution, like an f64.
            true,
            // As per above, also this might introduce singletons
            // in the presence of numerical instability.
            true,
            // The graph produce is directed, and may introduce new trap nodes.
            true,
            verbose.unwrap_or(true),
        )
    }
}
