use rayon::iter::ParallelIterator;

use crate::constructors::build_graph_from_integers;

use super::*;

/// # Conversion of the graph.
impl Graph {
    /// Convert inplace the graph to directed.
    ///
    /// # Implementative details
    /// The conversion to a directed graph is trivial as only requires to
    /// switch the flag for directed to true.
    pub fn to_directed_inplace(&mut self) -> &mut Graph {
        self.directed = true;
        self
    }

    /// Return a new instance of the current graph as directed.
    pub fn to_directed(&self) -> Graph {
        let mut new_graph = self.clone();
        new_graph.to_directed_inplace();
        new_graph
    }

    /// Return the directed graph from the upper triangular adjacency matrix.
    ///
    /// # Implementative details
    /// Filtering a graph to the upper triangular matrix means that the
    /// resulting graph will exclusively have edges so that `dst > src`.
    ///
    pub fn to_upper_triangular(&self) -> Graph {
        build_graph_from_integers(
            Some(
                self.par_iter_directed_edge_node_ids_and_edge_type_id_and_edge_weight()
                    .filter_map(|(_, src, dst, edge_type, weight)| {
                        if dst > src {
                            Some((0, (src, dst, edge_type, weight.unwrap_or(WeightT::NAN))))
                        } else {
                            None
                        }
                    }),
            ),
            self.nodes.clone(),
            self.node_types.clone(),
            self.edge_types
                .as_ref()
                .as_ref()
                .map(|ets| ets.vocabulary.clone()),
            self.has_edge_weights(),
            true,
            Some(true),
            Some(false),
            Some(false),
            // TODO: possibly the edges number can be precomputed.
            None,
            true,
            true,
            self.get_name(),
        )
        .unwrap()
    }

    /// Return the directed graph from the lower triangular adjacency matrix.
    ///
    /// # Implementative details
    /// Filtering a graph to the lower triangular matrix means that the
    /// resulting graph will exclusively have edges so that `src > dst`.
    ///
    pub fn to_lower_triangular(&self) -> Graph {
        build_graph_from_integers(
            Some(
                self.par_iter_directed_edge_node_ids_and_edge_type_id_and_edge_weight()
                    .filter_map(|(_, src, dst, edge_type, weight)| {
                        if src > dst {
                            Some((0, (src, dst, edge_type, weight.unwrap_or(WeightT::NAN))))
                        } else {
                            None
                        }
                    }),
            ),
            self.nodes.clone(),
            self.node_types.clone(),
            self.edge_types
                .as_ref()
                .as_ref()
                .map(|ets| ets.vocabulary.clone()),
            self.has_edge_weights(),
            true,
            Some(true),
            Some(false),
            Some(false),
            // TODO: possibly the edges number can be precomputed.
            None,
            true,
            true,
            self.get_name(),
        )
        .unwrap()
    }

    /// Return the graph from the main diagonal adjacency matrix.
    ///
    /// # Implementative details
    /// The resulting graph will only contain the selfloops present in the
    /// original graph.
    ///
    pub fn to_main_diagonal(&self) -> Graph {
        build_graph_from_integers(
            Some(
                self.par_iter_directed_edge_node_ids_and_edge_type_id_and_edge_weight()
                    .filter_map(|(_, src, dst, edge_type, weight)| {
                        if src == dst {
                            Some((0, (src, dst, edge_type, weight.unwrap_or(WeightT::NAN))))
                        } else {
                            None
                        }
                    }),
            ),
            self.nodes.clone(),
            self.node_types.clone(),
            self.edge_types
                .as_ref()
                .as_ref()
                .map(|ets| ets.vocabulary.clone()),
            self.has_edge_weights(),
            self.is_directed(),
            Some(true),
            Some(false),
            Some(false),
            // TODO: possibly the edges number can be precomputed.
            None,
            true,
            true,
            self.get_name(),
        )
        .unwrap()
    }

    /// Return the graph from the anti-diagonal adjacency matrix.
    ///
    /// # Implementative details
    /// The resulting graph will include only the edges present on the
    /// anti-diagonal of the graph.
    ///
    pub fn to_anti_diagonal(&self) -> Graph {
        let nodes_number = self.get_nodes_number();
        build_graph_from_integers(
            Some(
                self.par_iter_directed_edge_node_ids_and_edge_type_id_and_edge_weight()
                    .filter_map(|(_, src, dst, edge_type, weight)| {
                        if src == nodes_number - dst {
                            Some((0, (src, dst, edge_type, weight.unwrap_or(WeightT::NAN))))
                        } else {
                            None
                        }
                    }),
            ),
            self.nodes.clone(),
            self.node_types.clone(),
            self.edge_types
                .as_ref()
                .as_ref()
                .map(|ets| ets.vocabulary.clone()),
            self.has_edge_weights(),
            self.is_directed(),
            Some(true),
            Some(false),
            Some(false),
            // TODO: possibly the edges number can be precomputed.
            None,
            true,
            true,
            self.get_name(),
        )
        .unwrap()
    }

    /// Return the graph from the bidiagonal adjacency matrix.
    ///
    /// # Implementative details
    /// The resulting graph will include only the edges present on either
    /// the diagonal or anti-diagonal matrix.
    ///
    pub fn to_bidiagonal(&self) -> Graph {
        let nodes_number = self.get_nodes_number();
        build_graph_from_integers(
            Some(
                self.par_iter_directed_edge_node_ids_and_edge_type_id_and_edge_weight()
                    .filter_map(|(_, src, dst, edge_type, weight)| {
                        if src == dst || src == nodes_number - dst {
                            Some((0, (src, dst, edge_type, weight.unwrap_or(WeightT::NAN))))
                        } else {
                            None
                        }
                    }),
            ),
            self.nodes.clone(),
            self.node_types.clone(),
            self.edge_types
                .as_ref()
                .as_ref()
                .map(|ets| ets.vocabulary.clone()),
            self.has_edge_weights(),
            self.is_directed(),
            Some(true),
            Some(false),
            Some(false),
            // TODO: possibly the edges number can be precomputed.
            None,
            true,
            true,
            self.get_name(),
        )
        .unwrap()
    }

    /// Return the graph from the arrowhead adjacency matrix.
    ///
    pub fn to_arrowhead(&self) -> Graph {
        build_graph_from_integers(
            Some(
                self.par_iter_directed_edge_node_ids_and_edge_type_id_and_edge_weight()
                    .filter_map(|(_, src, dst, edge_type, weight)| {
                        if src == 0 || dst == 0 || src == dst {
                            Some((0, (src, dst, edge_type, weight.unwrap_or(WeightT::NAN))))
                        } else {
                            None
                        }
                    }),
            ),
            self.nodes.clone(),
            self.node_types.clone(),
            self.edge_types
                .as_ref()
                .as_ref()
                .map(|ets| ets.vocabulary.clone()),
            self.has_edge_weights(),
            self.is_directed(),
            Some(true),
            Some(false),
            Some(false),
            // TODO: possibly the edges number can be precomputed.
            None,
            true,
            true,
            self.get_name(),
        )
        .unwrap()
    }

    /// Return the graph from the transposed adjacency matrix.
    ///
    pub fn to_transposed(&self) -> Graph {
        if !self.is_directed() {
            return self.clone();
        }
        build_graph_from_integers(
            Some(
                self.par_iter_directed_edge_node_ids_and_edge_type_id_and_edge_weight()
                    .map(|(_, src, dst, edge_type, weight)| {
                        (0, (dst, src, edge_type, weight.unwrap_or(WeightT::NAN)))
                    }),
            ),
            self.nodes.clone(),
            self.node_types.clone(),
            self.edge_types
                .as_ref()
                .as_ref()
                .map(|ets| ets.vocabulary.clone()),
            self.has_edge_weights(),
            self.is_directed(),
            Some(true),
            Some(false),
            Some(false),
            Some(self.get_directed_edges_number()),
            self.has_singleton_nodes(),
            self.has_singleton_nodes_with_selfloops(),
            self.get_name(),
        )
        .unwrap()
    }

    /// Return the complementary graph.
    ///
    /// # Implementative details
    /// Note that the resulting graph may require a significant amount
    /// of memory.
    ///
    pub fn to_complementary(&self) -> Graph {
        build_graph_from_integers(
            Some(self.par_iter_node_ids().flat_map(|src| {
                self.iter_node_ids()
                    .filter_map(|dst| {
                        if self.has_edge_from_node_ids(src, dst) {
                            None
                        } else {
                            Some((0, (src, dst, None, WeightT::NAN)))
                        }
                    })
                    .collect::<Vec<_>>()
            })),
            self.nodes.clone(),
            self.node_types.clone(),
            self.edge_types
                .as_ref()
                .as_ref()
                .map(|ets| ets.vocabulary.clone()),
            self.has_edge_weights(),
            self.is_directed(),
            Some(true),
            Some(false),
            Some(false),
            Some(
                (self.get_nodes_number() as EdgeT).pow(2) - self.get_unique_directed_edges_number(),
            ),
            true,
            true,
            self.get_name(),
        )
        .unwrap()
    }
}
