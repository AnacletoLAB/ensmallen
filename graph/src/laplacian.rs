use indicatif::ParallelProgressIterator;
use rayon::iter::ParallelIterator;

use crate::constructors::build_graph_from_integers;

use super::*;

/// # Generators of laplacian-transformed graphs.
impl Graph {
    /// Returns unweighted laplacian transformation of the graph.
    ///
    /// # Arguments
    /// * `verbose`: Option<bool> - Whether to show a loading bar while building the graph. By default, true.
    fn get_transformed_graph(
        &self,
        edge_value: fn(&Graph, NodeT, NodeT) -> WeightT,
        selfloop_value: fn(&Graph, NodeT) -> WeightT,
        verbose: Option<bool>,
    ) -> Graph {
        let verbose = verbose.unwrap_or(true);
        let edges_progress_bar = get_loading_bar(
            verbose,
            "Building Laplacian edges",
            self.get_directed_edges_number() as usize,
        );
        let selfloop_progress_bar = get_loading_bar(
            verbose,
            "Building Laplacian selfloops",
            self.get_nodes_number() as usize,
        );
        let total_edges_number = self.get_directed_edges_number() - self.get_selfloops_number()
            + self.get_nodes_number() as EdgeT;

        // TODO! this method can be made fully sorted parallell by using an offset vector
        // of the selfloops that need to be added.
        build_graph_from_integers(
            Some(
                self.par_iter_edge_node_ids_and_edge_type_id(true)
                    .progress_with(edges_progress_bar)
                    .map(|(_, src, dst, edge_type)| {
                        (
                            // The number of the edge is irrelevant because we cannot load this as sorted.
                            0,
                            (
                                src,
                                dst,
                                edge_type,
                                if src == dst {
                                    selfloop_value(&self, src)
                                } else {
                                    edge_value(&self, src, dst)
                                },
                            ),
                        )
                    })
                    .chain(
                        self.par_iter_node_ids()
                            .progress_with(selfloop_progress_bar)
                            .filter(|&node_id| !self.has_selfloop_from_node_id(node_id))
                            .map(|node_id| {
                                (
                                    // The number of the edge is irrelevant because we cannot load this as sorted.
                                    0,
                                    (node_id, node_id, None, selfloop_value(&self, node_id)),
                                )
                            }),
                    ),
            ),
            self.nodes.clone(),
            self.node_types.clone(),
            self.edge_types.as_ref().map(|ets| ets.vocabulary.clone()),
            true,
            self.is_directed(),
            Some(true),
            Some(false),
            Some(false),
            Some(total_edges_number),
            self.get_name(),
        )
        .unwrap()
    }

    /// Returns unweighted laplacian transformation of the graph.
    ///
    /// # Arguments
    /// * `verbose`: Option<bool> - Whether to show a loading bar while building the graph. By default, true.
    pub fn get_laplacian_transformed_graph(&self, verbose: Option<bool>) -> Graph {
        self.get_transformed_graph(
            |_, _, _| -1.0,
            |graph, node_id| unsafe {
                graph.get_unchecked_node_degree_from_node_id(node_id) as WeightT
            },
            verbose,
        )
    }

    /// Returns unweighted random walk normalized laplacian transformation of the graph.
    ///
    /// # Arguments
    /// * `verbose`: Option<bool> - Whether to show a loading bar while building the graph.
    pub fn get_random_walk_normalized_laplacian_transformed_graph(
        &self,
        verbose: Option<bool>,
    ) -> Graph {
        self.get_transformed_graph(
            |graph, src, _| {
                -1.0 / unsafe { graph.get_unchecked_node_degree_from_node_id(src) as WeightT }
            },
            |_, _| 1.0,
            verbose,
        )
    }

    /// Returns unweighted symmetric normalized laplacian transformation of the graph.
    ///
    /// # Arguments
    /// * `verbose`: Option<bool> - Whether to show a loading bar while building the graph.
    ///
    /// # Raises
    /// * The graph must be undirected, as we do not currently support this transformation for directed graphs.
    pub fn get_symmetric_normalized_laplacian_transformed_graph(
        &self,
        verbose: Option<bool>,
    ) -> Result<Graph> {
        self.must_be_undirected()?;
        Ok(self.get_transformed_graph(
            |graph, src, dst| {
                -1.0 / unsafe {
                    ((graph.get_unchecked_node_degree_from_node_id(src)
                        * graph.get_unchecked_node_degree_from_node_id(dst))
                        as f64)
                        .sqrt() as WeightT
                }
            },
            |_, _| 1.0,
            verbose,
        ))
    }

    /// Returns unweighted symmetric normalized transformation of the graph.
    ///
    /// # Arguments
    /// * `verbose`: Option<bool> - Whether to show a loading bar while building the graph.
    ///
    /// # Raises
    /// * The graph must be undirected, as we do not currently support this transformation for directed graphs.
    pub fn get_symmetric_normalized_transformed_graph(
        &self,
        verbose: Option<bool>,
    ) -> Result<Graph> {
        self.must_be_undirected()?;
        Ok(self.get_transformed_graph(
            |graph, src, dst| {
                1.0 / unsafe {
                    ((graph.get_unchecked_node_degree_from_node_id(src)
                        * graph.get_unchecked_node_degree_from_node_id(dst))
                        as f64)
                        .sqrt() as WeightT
                }
            },
            |_, _| 1.0,
            verbose,
        ))
    }
}
