use crate::constructors::build_graph_from_integers;

use super::*;
use indicatif::ParallelProgressIterator;
use rayon::iter::ParallelIterator;

/// # Transitivity.
impl Graph {
    #[fuzz_type(iterations: Option<u8>)]
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
        if let Some(i) = iterations {
            if i == 0 {
                return self.clone();
            }
        }
        let verbose = verbose.unwrap_or(true);
        let pb = get_loading_bar(
            verbose,
            "Computing transitive closure",
            self.get_number_of_nodes() as usize,
        );
        build_graph_from_integers(
            Some(
                self.par_iter_node_ids()
                    .progress_with(pb)
                    .filter_map(|src_node_id| unsafe {
                        if self.is_unchecked_singleton_from_node_id(src_node_id) {
                            None
                        } else {
                            Some(
                                self.get_unchecked_breadth_first_search_from_node_id(
                                    src_node_id,
                                    None,
                                    None,
                                    iterations,
                                )
                                .into_distances()
                                .into_iter()
                                .enumerate()
                                .filter(|&(_, distance)| distance != NODE_NOT_PRESENT)
                                .map(move |(dst_node_id, _)| {
                                    (0, (src_node_id, dst_node_id as NodeT, None, WeightT::NAN))
                                })
                                .collect::<Vec<_>>(),
                            )
                        }
                    })
                    .flatten(),
            ),
            self.nodes.clone(),
            self.node_types.clone(),
            self.edge_types
                .as_ref()
                .as_ref()
                .map(|ets| ets.vocabulary.clone()),
            false,
            self.is_directed(),
            Some(true),
            Some(false),
            Some(false),
            None,
            self.has_singleton_nodes(),
            self.has_singleton_nodes_with_selfloops(),
            self.get_name(),
        )
        .unwrap()
    }

    #[fuzz_type(iterations: Option<u8>)]
    /// Returns graph with unweighted shortest paths computed up to the given depth.
    ///
    /// The returned graph will have no selfloops.
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
    pub fn get_all_shortest_paths(
        &self,
        iterations: Option<NodeT>,
        verbose: Option<bool>,
    ) -> Graph {
        if let Some(i) = iterations {
            if i == 0 {
                return self.clone();
            }
        }

        let verbose = verbose.unwrap_or(true);
        let pb = get_loading_bar(
            verbose,
            "Computing all unweighted shortest paths",
            self.get_number_of_nodes() as usize,
        );
        build_graph_from_integers(
            Some(
                self.par_iter_node_ids()
                    .progress_with(pb)
                    .filter_map(|src_node_id| unsafe {
                        if self.is_unchecked_connected_from_node_id(src_node_id) {
                            Some(
                                self.get_unchecked_breadth_first_search_from_node_id(
                                    src_node_id,
                                    None,
                                    None,
                                    iterations,
                                )
                                .into_distances()
                                .into_iter()
                                .enumerate()
                                .filter(move |&(dst_node_id, distance)| {
                                    distance != NODE_NOT_PRESENT
                                        && src_node_id != dst_node_id as NodeT
                                })
                                .map(move |(dst_node_id, distance)| {
                                    (
                                        0,
                                        (
                                            src_node_id,
                                            dst_node_id as NodeT,
                                            None,
                                            distance as WeightT,
                                        ),
                                    )
                                })
                                .collect::<Vec<_>>(),
                            )
                        } else {
                            None
                        }
                    })
                    .flatten(),
            ),
            self.nodes.clone(),
            self.node_types.clone(),
            self.edge_types
                .as_ref()
                .as_ref()
                .map(|ets| ets.vocabulary.clone()),
            true,
            self.is_directed(),
            Some(true),
            Some(false),
            Some(false),
            None,
            self.has_singleton_nodes() || self.has_singleton_nodes_with_selfloops(),
            false,
            self.get_name(),
        )
        .unwrap()
    }

    #[fuzz_type(iterations: Option<u8>)]
    /// Returns graph with weighted shortest paths computed up to the given depth.
    ///
    /// The returned graph will have no selfloops.
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
    /// # Raises
    /// * If the graph does not have weights.
    /// * If the graph contains negative weights.
    /// * If the user has asked for the weights to be treated as probabilities but the weights are not between 0 and 1.
    ///
    pub fn get_weighted_all_shortest_paths(
        &self,
        iterations: Option<NodeT>,
        use_edge_weights_as_probabilities: Option<bool>,
        verbose: Option<bool>,
    ) -> Result<Graph> {
        if let Some(i) = iterations {
            if i == 0 {
                return Ok(self.clone());
            }
        }
        if let Some(uewap) = use_edge_weights_as_probabilities {
            if uewap {
                self.must_have_edge_weights_representing_probabilities()?;
            }
        }
        self.must_have_positive_edge_weights()?;
        let verbose = verbose.unwrap_or(true);
        let pb = get_loading_bar(
            verbose,
            "Computing all unweighted shortest paths",
            self.get_number_of_nodes() as usize,
        );
        build_graph_from_integers(
            Some(
                self.par_iter_node_ids()
                    .progress_with(pb)
                    .filter_map(|src_node_id| unsafe {
                        if self.is_unchecked_connected_from_node_id(src_node_id) {
                            Some(
                                self.get_unchecked_dijkstra_from_node_id(
                                    src_node_id,
                                    None,
                                    None,
                                    Some(false),
                                    iterations,
                                    use_edge_weights_as_probabilities,
                                )
                                .distances
                                .into_iter()
                                .enumerate()
                                // We need to convert the distance to WeightT before
                                // the checks because the distance is an f64 while currently
                                // the WeightT is an f32, and values outside the resolution of
                                // f32 and within f64 will convert to zeros and infinities.
                                .map(|(dst_node_id, distance)| (dst_node_id, distance as WeightT))
                                .filter(move |(dst_node_id, distance)| {
                                    distance.is_finite()
                                        && src_node_id != *dst_node_id as NodeT
                                        && *distance > 0.0
                                })
                                .map(move |(dst_node_id, distance)| {
                                    (0, (src_node_id, dst_node_id as NodeT, None, distance))
                                })
                                .collect::<Vec<_>>(),
                            )
                        } else {
                            None
                        }
                    })
                    .flatten(),
            ),
            self.nodes.clone(),
            self.node_types.clone(),
            self.edge_types
                .as_ref()
                .as_ref()
                .map(|ets| ets.vocabulary.clone()),
            true,
            self.is_directed() || iterations.is_some(),
            Some(true),
            Some(false),
            Some(false),
            None,
            self.has_singleton_nodes() || self.has_singleton_nodes_with_selfloops(),
            false,
            self.get_name(),
        )
    }
}
