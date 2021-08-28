use itertools::Itertools;
use rayon::prelude::*;
use std::collections::HashMap;

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
    /// * `add_selfloops_where_missing`: Option<bool> - Whether to add selfloops where they are missing. By default, true.
    /// * `complete`: Option<bool> - Whether to return the complete edge list in undirected graphs.
    pub unsafe fn par_iter_subsampled_binary_adjacency_matrix<'a>(
        &'a self,
        nodes: &'a [NodeT],
        add_selfloops_where_missing: Option<bool>,
        complete: Option<bool>,
    ) -> impl ParallelIterator<Item = (NodeT, usize, NodeT, usize)> + 'a {
        let nodes_number = nodes.len();
        let complete = complete.unwrap_or(false);
        let add_selfloops_where_missing = add_selfloops_where_missing.unwrap_or(true);
        (0..nodes_number)
            .into_par_iter()
            .flat_map(move |src| (0..nodes_number).into_par_iter().map(move |dst| (src, dst)))
            .map(move |(src, dst)| (nodes[src], src, nodes[dst], dst))
            .filter(move |&(src_node_id, src, dst_node_id, dst)| {
                (self.is_directed() || complete || src <= dst)
                    && (add_selfloops_where_missing && src == dst
                        || self.has_edge_from_node_ids(src_node_id, dst_node_id))
            })
            .map(move |(src_node_id, src, dst_node_id, dst)| (src_node_id, src, dst_node_id, dst))
    }

    /// Returns iterator over subsampled weighted adjacency matrix on the provided nodes.
    ///
    /// # Safety
    /// The provided nodes are assumed to be unique.
    /// Additionally, the nodes are assumed to exist within this graph instance.
    ///
    /// # Arguments
    /// * `nodes`: Vec<NodeT> - The subsampled nodes.
    /// * `complete`: Option<bool> - Whether to return the complete edge list in undirected graphs.
    ///
    /// # Raises
    /// * If the graph is a multigraph.
    /// * If the graph ddoes not contain weights.
    pub unsafe fn par_iter_subsampled_weighted_adjacency_matrix<'a>(
        &'a self,
        nodes: &'a [NodeT],
        complete: Option<bool>,
    ) -> Result<impl ParallelIterator<Item = (NodeT, usize, NodeT, usize, WeightT)> + 'a> {
        self.must_not_be_multigraph()?;
        self.must_have_edge_weights()?;
        Ok(self
            .par_iter_subsampled_binary_adjacency_matrix(nodes, Some(false), complete)
            .map(move |(src_node_id, src, dst_node_id, dst)| {
                (
                    src_node_id,
                    src,
                    dst_node_id,
                    dst,
                    self.get_unchecked_edge_weight_from_node_ids(src_node_id, dst_node_id),
                )
            }))
    }

    /// Returns iterator over subsampled symmetric laplacian adjacency matrix on the provided nodes.
    ///
    /// # Safety
    /// The provided nodes are assumed to be unique.
    /// Additionally, the nodes are assumed to exist within this graph instance.
    ///
    /// # Arguments
    /// * `nodes`: Vec<NodeT> - The subsampled nodes.
    /// * `add_selfloops_where_missing`: Option<bool> - Whether to add selfloops where they are missing. By default, true.
    /// * `complete`: Option<bool> - Whether to return the complete edge list in undirected graphs.
    pub unsafe fn par_iter_subsampled_symmetric_laplacian_adjacency_matrix<'a>(
        &'a self,
        nodes: &'a [NodeT],
        add_selfloops_where_missing: Option<bool>,
        complete: Option<bool>,
    ) -> impl ParallelIterator<Item = (usize, usize, WeightT)> + 'a {
        let complete = complete.unwrap_or(false);
        let degrees = nodes
            .par_iter()
            .map(|&node_id| self.get_unchecked_node_degree_from_node_id(node_id) as f64)
            .collect::<Vec<_>>();
        let nodes_number_usize = nodes.len();
        let nodes_number_float = nodes_number_usize as f64;
        let nodes_map: HashMap<NodeT, usize> = nodes
            .iter()
            .cloned()
            .enumerate()
            .map(|(i, node_id)| (node_id, i))
            .collect();
        let add_selfloops_where_missing = add_selfloops_where_missing.unwrap_or(true);
        (0..nodes_number_usize)
            .into_par_iter()
            .flat_map(move |src| {
                let src_degree = degrees[src];
                let src_node_id = nodes[src];
                let mut result: Vec<(usize, usize, WeightT)> = if src_degree > nodes_number_float {
                    let starting_index = if self.is_directed() { 0 } else { src };
                    (starting_index..nodes_number_usize)
                        .into_par_iter()
                        .filter(|&dst| {
                            !(add_selfloops_where_missing && src == dst)
                                && self.has_edge_from_node_ids(src_node_id, nodes[dst])
                        })
                        .map(|dst| {
                            (
                                src,
                                dst,
                                (1.0 / (src_degree * degrees[dst]).sqrt()) as WeightT,
                            )
                        })
                        .collect()
                } else {
                    self.par_iter_unchecked_neighbour_node_ids_from_source_node_id(src_node_id)
                        .filter_map(|dst_node_id| {
                            if add_selfloops_where_missing && src_node_id == dst_node_id {
                                return None;
                            }
                            nodes_map.get(&dst_node_id).map(|&dst| {
                                (
                                    src,
                                    dst,
                                    (1.0 / (src_degree * degrees[dst]).sqrt()) as WeightT,
                                )
                            })
                        })
                        .collect()
                };
                if add_selfloops_where_missing {
                    result.push((src, src, 1.0));
                }
                result
            })
            .flat_map(move |(src, dst, weight)| {
                if src == dst {
                    vec![(src, dst, weight)]
                } else {
                    if complete && !self.is_directed() {
                        vec![(src, dst, weight), (dst, src, weight)]
                    } else {
                        vec![(src, dst, weight)]
                    }
                }
            })
    }

    /// Return list of the supported sparse edge weighting methods.
    pub fn get_sparse_edge_weighting_methods(&self) -> Vec<&str> {
        vec!["weights", "laplacian"]
    }

    /// Return list of the supported edge weighting methods.
    pub fn get_edge_weighting_methods(&self) -> Vec<&str> {
        vec![
            "unweighted_shortest_path",
            "probabilistic_weighted_shortest_path",
            "preferential_attachment",
            "weighted_preferential_attachment",
            "jaccard_coefficient",
            "adamic_adar_index",
            "resource_allocation_index",
            "weighted_resource_allocation_index",
            "weights",
            "laplacian",
        ]
    }

    /// Returns iterator over subsampled binary adjacency matrix on the provided nodes.
    ///
    /// # Safety
    /// The provided nodes are assumed to be unique.
    /// Additionally, the nodes are assumed to exist within this graph instance.
    ///
    /// # Arguments
    /// * `nodes`: Vec<NodeT> - The subsampled nodes.
    /// * `edge_weighting_method`: &str - The edge_weighting_method to use to compute the adjacency matrix.
    ///
    /// # Raises
    /// * If the given edge_weighting_method is not supported.
    /// * If The edge_weighting_method requires the graph to be connected but the graph is not.
    /// * If the edge_weighting_method requires the graph to be weighted but the graph is not.
    pub unsafe fn par_iter_subsampled_edge_metric_matrix<'a>(
        &'a self,
        nodes: &'a [NodeT],
        edge_weighting_method: &str,
    ) -> Result<impl ParallelIterator<Item = (NodeT, usize, NodeT, usize, WeightT)> + 'a> {
        let nodes_number = nodes.len();
        let edge_weighting_method: Result<fn(&Graph, NodeT, NodeT) -> f64> = match edge_weighting_method {
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
            edge_weighting_method => Err(format!(
                concat!(
                    "The provided edge weighting method {} is not currenly supported. The supported edge weighting methods are:\n",
                    "{}"
                ),
                edge_weighting_method,
                self.get_edge_weighting_methods().into_iter().map(|edge_sampling_schema| format!("* {}", edge_sampling_schema)).join("\n")
            )),
        };
        let edge_weighting_method = edge_weighting_method?;
        Ok((0..nodes_number)
            .into_par_iter()
            .flat_map(move |src| (0..nodes_number).into_par_iter().map(move |dst| (src, dst)))
            .filter(move |(src, dst)| self.is_directed() || src <= dst)
            .map(move |(src, dst)| {
                let src_node_id = nodes[src];
                let dst_node_id = nodes[dst];
                let weight = edge_weighting_method(self, src_node_id, dst_node_id) as WeightT;
                (src_node_id, src, dst_node_id, dst, weight)
            }))
    }
}
