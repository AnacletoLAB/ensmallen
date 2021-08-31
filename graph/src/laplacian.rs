use num_traits::Zero;
use rayon::iter::ParallelIterator;

use crate::constructors::build_graph_from_integers;

use super::*;

/// # Generators of laplacian-transformed graphs.
impl Graph {
    /// Returns unweighted laplacian transformation of the graph.
    ///
    /// # Arguments
    /// * `get_edge_weight`: fn(&Graph, NodeT, NodeT) -> WeightT - The closure providing the value for the normal edge weight.
    /// * `get_selfloop_edge_weight`: fn(&Graph, NodeT) -> WeightT - The closure providing the value for the normal selfloop weight.
    /// * `directed`: bool - Whether to create the graph as directed and undirected.
    fn get_transformed_graph(
        &self,
        get_edge_weight: fn(&Graph, NodeT, NodeT) -> WeightT,
        get_selfloop_edge_weight: fn(&Graph, NodeT) -> WeightT,
        directed: bool,
    ) -> Graph {
        let total_edges_number = self.get_directed_edges_number() - self.get_selfloops_number()
            + self.get_nodes_number() as EdgeT;

        // TODO! this method can be made fully sorted parallell by using an offset vector
        // of the selfloops that need to be added.
        build_graph_from_integers(
            Some(
                self.par_iter_edge_node_ids_and_edge_type_id(true)
                    .map(|(_, src, dst, edge_type)| {
                        (
                            // The number of the edge is irrelevant because we cannot load this as sorted.
                            0,
                            (
                                src,
                                dst,
                                edge_type,
                                if src == dst {
                                    get_selfloop_edge_weight(&self, src)
                                } else {
                                    get_edge_weight(&self, src, dst)
                                },
                            ),
                        )
                    })
                    .chain(
                        self.par_iter_node_ids()
                            .filter(|&node_id| !self.has_selfloop_from_node_id(node_id))
                            .filter_map(|node_id| {
                                let weight = get_selfloop_edge_weight(&self, node_id);
                                if weight.is_zero() {
                                    return None;
                                }
                                Some((
                                    // The number of the edge is irrelevant because we cannot load this as sorted.
                                    0,
                                    (node_id, node_id, None, weight),
                                ))
                            }),
                    ),
            ),
            self.nodes.clone(),
            self.node_types.clone(),
            self.edge_types.as_ref().map(|ets| ets.vocabulary.clone()),
            true,
            directed,
            Some(true),
            Some(false),
            Some(false),
            Some(total_edges_number),
            true,
            true,
            self.get_name(),
        )
        .unwrap()
    }

    /// Returns unweighted laplacian transformation of the graph.
    pub fn get_laplacian_transformed_graph(&self) -> Graph {
        self.get_transformed_graph(
            |_, _, _| -1.0,
            |graph, node_id| unsafe {
                graph.get_unchecked_node_degree_from_node_id(node_id) as WeightT
            },
            true,
        )
    }

    /// Returns unweighted random walk normalized laplacian transformation of the graph.
    pub fn get_random_walk_normalized_laplacian_transformed_graph(&self) -> Graph {
        self.get_transformed_graph(
            |graph, src, _| {
                -1.0 / unsafe { graph.get_unchecked_node_degree_from_node_id(src) as WeightT }
            },
            |_, _| 1.0,
            true,
        )
    }

    /// Returns unweighted symmetric normalized laplacian transformation of the graph.
    ///
    /// # Raises
    /// * The graph must be undirected, as we do not currently support this transformation for directed graphs.
    pub fn get_symmetric_normalized_laplacian_transformed_graph(&self) -> Result<Graph> {
        self.must_be_undirected()?;
        Ok(self.get_transformed_graph(
            |graph, src, dst| unsafe {
                -1.0 / {
                    ((graph.get_unchecked_node_degree_from_node_id(src)
                        * graph.get_unchecked_node_degree_from_node_id(dst))
                        as f64)
                        .sqrt() as WeightT
                }
            },
            |_, _| 1.0,
            self.is_directed(),
        ))
    }

    /// Returns unweighted symmetric normalized transformation of the graph.
    ///
    /// # Raises
    /// * The graph must be undirected, as we do not currently support this transformation for directed graphs.
    pub fn get_symmetric_normalized_transformed_graph(&self) -> Result<Graph> {
        self.must_be_undirected()?;
        Ok(self.get_transformed_graph(
            |graph, src, dst| unsafe {
                (1.0 / ((graph.get_unchecked_node_degree_from_node_id(src)
                    * graph.get_unchecked_node_degree_from_node_id(dst))
                    as f64)
                    .sqrt()) as WeightT
            },
            |_, _| 1.0,
            self.is_directed(),
        ))
    }
}
