use rayon::prelude::*;

use super::*;

/// # Subgraph sampling
impl Graph {
    /// Returns iterator over subsampled binary adjacency matrix on the provided nodes.
    ///
    /// # Safety
    /// The provided nodes are assumed to be unique.
    /// Additionally, the nodes are assumed to exist within this graph instance.
    ///
    /// # Arguments
    /// * `nodes`: Vec<NodeT> - The subsampled nodes.
    pub unsafe fn par_iter_subsampled_binary_adjacency_matrix(
        &self,
        nodes: Vec<NodeT>,
    ) -> impl ParallelIterator<Item = (NodeT, usize, NodeT, usize)> + '_ {
        let nodes_number = nodes.len();
        (0..nodes_number)
            .into_par_iter()
            .flat_map_iter(move |src| (0..nodes_number).map(move |dst| (src, dst)))
            .flat_map(move |(src, dst)| {
                let src_node_id = nodes[src];
                let dst_node_id = nodes[dst];
                if (self.is_directed() || src <= dst)
                    && self.has_edge_from_node_ids(src_node_id, dst_node_id)
                {
                    if self.is_directed() || src_node_id == dst_node_id {
                        vec![(src_node_id, src, dst_node_id, dst)]
                    } else {
                        vec![
                            (src_node_id, src, dst_node_id, dst),
                            (dst_node_id, dst, src_node_id, src),
                        ]
                    }
                } else {
                    vec![]
                }
            })
    }

    /// Returns iterator over subsampled weighted adjacency matrix on the provided nodes.
    ///
    /// # Safety
    /// The provided nodes are assumed to be unique.
    /// Additionally, the nodes are assumed to exist within this graph instance.
    ///
    /// # Arguments
    /// * `nodes`: Vec<NodeT> - The subsampled nodes.
    ///
    /// # Raises
    /// * If the graph is a multigraph.
    /// * If the
    pub unsafe fn par_iter_subsampled_weighted_adjacency_matrix(
        &self,
        nodes: Vec<NodeT>,
    ) -> Result<impl ParallelIterator<Item = (NodeT, usize, NodeT, usize, WeightT)> + '_> {
        self.must_not_be_multigraph()?;
        self.must_have_edge_weights()?;
        Ok(self.par_iter_subsampled_binary_adjacency_matrix(nodes).map(
            move |(src_node_id, src, dst_node_id, dst)| {
                (
                    src_node_id,
                    src,
                    dst_node_id,
                    dst,
                    self.get_unchecked_edge_weight_from_node_ids(src_node_id, dst_node_id),
                )
            },
        ))
    }

    /// Returns iterator over subsampled symmetric laplacian adjacency matrix on the provided nodes.
    ///
    /// # Safety
    /// The provided nodes are assumed to be unique.
    /// Additionally, the nodes are assumed to exist within this graph instance.
    ///
    /// # Arguments
    /// * `nodes`: Vec<NodeT> - The subsampled nodes.
    pub unsafe fn par_iter_subsampled_symmetric_laplacian_adjacency_matrix(
        &self,
        nodes: Vec<NodeT>,
    ) -> impl ParallelIterator<Item = (NodeT, usize, NodeT, usize, WeightT)> + '_ {
        let degrees = nodes
            .iter()
            .map(|&node_id| self.get_unchecked_node_degree_from_node_id(node_id))
            .collect::<Vec<_>>();
        let nodes_number = nodes.len();
        (0..nodes_number)
            .into_par_iter()
            .flat_map_iter(move |src| (0..nodes_number).map(move |dst| (src, dst)))
            .flat_map(move |(src, dst)| {
                let src_node_id = nodes[src];
                let src_degree = degrees[src];
                let dst_node_id = nodes[dst];
                let dst_degree = degrees[dst];

                if (self.is_directed() || src <= dst)
                    && self.has_edge_from_node_ids(src_node_id, dst_node_id)
                {
                    if src_node_id == dst_node_id {
                        vec![(src_node_id, src, dst_node_id, dst, 1.0)]
                    } else {
                        let weight = (1.0 / ((src_degree * dst_degree) as f64).sqrt()) as WeightT;
                        if self.is_directed() {
                            vec![(src_node_id, src, dst_node_id, dst, weight)]
                        } else {
                            vec![
                                (src_node_id, src, dst_node_id, dst, weight),
                                (dst_node_id, dst, src_node_id, src, weight),
                            ]
                        }
                    }
                } else {
                    vec![]
                }
            })
    }

    /// Returns iterator over subsampled binary adjacency matrix on the provided nodes.
    ///
    /// # Safety
    /// The provided nodes are assumed to be unique.
    /// Additionally, the nodes are assumed to exist within this graph instance.
    ///
    /// # Arguments
    /// * `nodes`: Vec<NodeT> - The subsampled nodes.
    ///
    /// # Raises
    /// * If the given metric is not supported.
    /// * If The metric requires the graph to be connected but the graph is not.
    /// * If the metric requires the graph to be weighted but the graph is not.
    pub unsafe fn par_iter_subsampled_edge_metric_matrix(
        &self,
        nodes: Vec<NodeT>,
        metric: &str,
    ) -> Result<impl ParallelIterator<Item = (NodeT, usize, NodeT, usize, WeightT)> + '_> {
        let nodes_number = nodes.len();
        let edge_metric: Result<fn(&Graph, NodeT, NodeT) -> f64> = match metric {
            "unweighted_shortest_path" => {
                self.must_be_connected()?;
                // We make sure that the diameter is precomputed.
                self.get_diameter(None, None)?;
                Ok(|graph: &Graph, src: NodeT, dst: NodeT| -> f64 {
                    if src == dst {
                        return 0.0;
                    }
                    graph
                        .get_unchecked_shortest_path_node_ids_from_node_ids(src, dst, None)
                        .unwrap()
                        .len() as f64
                        / (*graph.cache.get())
                            .diameter
                            .as_ref()
                            .unwrap()
                            .as_ref()
                            .unwrap()
                })
            }
            "probabilistic_weighted_shortest_path" => {
                self.must_have_edge_weights_representing_probabilities()?;
                Ok(|graph: &Graph, src: NodeT, dst: NodeT| -> f64 {
                    if src == dst {
                        return 1.0;
                    }
                    graph
                        .get_unchecked_weighted_shortest_path_node_ids_from_node_ids(
                            src,
                            dst,
                            Some(true),
                            None,
                        )
                        .0
                })
            }
            "preferential_attachment" => Ok(|graph: &Graph, src: NodeT, dst: NodeT| -> f64 {
                graph.get_unchecked_preferential_attachment_from_node_ids(src, dst, true)
            }),
            "weighted_preferential_attachment" => {
                self.must_have_edge_weights()?;
                Ok(|graph: &Graph, src: NodeT, dst: NodeT| -> f64 {
                    graph.get_unchecked_weighted_preferential_attachment_from_node_ids(
                        src, dst, true,
                    )
                })
            }
            "jaccard_coefficient" => Ok(|graph: &Graph, src: NodeT, dst: NodeT| -> f64 {
                graph.get_unchecked_jaccard_coefficient_from_node_ids(src, dst)
            }),
            "adamic_adar_index" => Ok(|graph: &Graph, src: NodeT, dst: NodeT| -> f64 {
                graph.get_unchecked_adamic_adar_index_from_node_ids(src, dst)
            }),
            "resource_allocation_index" => Ok(|graph: &Graph, src: NodeT, dst: NodeT| -> f64 {
                graph.get_unchecked_resource_allocation_index_from_node_ids(src, dst)
            }),
            "weighted_resource_allocation_index" => {
                self.must_have_edge_weights()?;
                Ok(|graph: &Graph, src: NodeT, dst: NodeT| -> f64 {
                    graph.get_unchecked_weighted_resource_allocation_index_from_node_ids(src, dst)
                })
            }
            metric => Err(format!(
                concat!(
                    "The provided metric {} is not currenly supported. The supported metrics are:\n",
                    "* unweighted_shortest_path\n",
                    "* probabilistic_weighted_shortest_path\n",
                    "* preferential_attachment\n",
                    "* weighted_preferential_attachment\n",
                    "* jaccard_coefficient\n",
                    "* adamic_adar_index\n",
                    "* resource_allocation_index\n",
                    "* weighted_resource_allocation_index\n",
                ),
                metric
            )),
        };
        let edge_metric = edge_metric?;
        Ok((0..nodes_number)
            .into_par_iter()
            .flat_map_iter(move |src| (0..nodes_number).map(move |dst| (src, dst)))
            .flat_map(move |(src, dst)| {
                if self.is_directed() || src <= dst {
                    let src_node_id = nodes[src];
                    let dst_node_id = nodes[dst];
                    let weight = edge_metric(self, src_node_id, dst_node_id) as WeightT;
                    if self.is_directed() || src_node_id == dst_node_id {
                        vec![(src_node_id, src, dst_node_id, dst, weight)]
                    } else {
                        vec![
                            (src_node_id, src, dst_node_id, dst, weight),
                            (dst_node_id, dst, src_node_id, src, weight),
                        ]
                    }
                } else {
                    vec![]
                }
            }))
    }

    

}
