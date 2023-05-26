use itertools::Itertools;
use rayon::prelude::*;
use std::collections::HashMap;
use std::sync::Arc;

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
    /// * `node_ids`: Vec<NodeT> - The subsampled nodes.
    /// * `add_selfloops_where_missing`: Option<bool> - Whether to add selfloops where they are missing. By default, true.
    /// * `complete`: Option<bool> - Whether to return the complete edge list in undirected graphs.
    pub unsafe fn par_iter_subsampled_binary_adjacency_matrix<'a>(
        &'a self,
        node_ids: &'a [NodeT],
        add_selfloops_where_missing: Option<bool>,
        complete: Option<bool>,
    ) -> impl ParallelIterator<Item = (NodeT, usize, NodeT, usize)> + 'a {
        let number_of_nodes = node_ids.len();
        let complete = complete.unwrap_or(false);
        let add_selfloops_where_missing = add_selfloops_where_missing.unwrap_or(true);
        (0..number_of_nodes)
            .into_par_iter()
            .flat_map(move |src| (0..number_of_nodes).into_par_iter().map(move |dst| (src, dst)))
            .map(move |(src, dst)| (node_ids[src], src, node_ids[dst], dst))
            .filter(move |&(src_node_id, src, dst_node_id, dst)| {
                (self.is_directed() || complete || src <= dst)
                    && (add_selfloops_where_missing && src == dst
                        || self.has_edge_from_node_ids(src_node_id, dst_node_id))
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
    /// * `node_ids`: Vec<NodeT> - The subsampled nodes.
    /// * `add_selfloops_where_missing`: Option<bool> - Whether to add selfloops where they are missing. By default, true.
    /// * `complete`: Option<bool> - Whether to return the complete edge list in undirected graphs.
    pub unsafe fn par_iter_subsampled_symmetric_laplacian_adjacency_matrix<'a>(
        &'a self,
        node_ids: &'a [NodeT],
        add_selfloops_where_missing: Option<bool>,
        complete: Option<bool>,
    ) -> impl ParallelIterator<Item = (usize, usize, WeightT)> + 'a {
        let complete = complete.unwrap_or(false) || self.is_directed();
        let degrees = node_ids
            .par_iter()
            .map(|&node_id| self.get_unchecked_node_degree_from_node_id(node_id) as f64)
            .collect::<Vec<_>>();
        let number_of_nodes_usize = node_ids.len();
        let number_of_nodes_float = number_of_nodes_usize as f64;
        let nodes_map: HashMap<NodeT, usize> = node_ids
            .iter()
            .cloned()
            .enumerate()
            .map(|(i, node_id)| (node_id, i))
            .collect();
        let add_selfloops_where_missing = add_selfloops_where_missing.unwrap_or(true);
        (0..number_of_nodes_usize)
            .into_par_iter()
            .flat_map(move |src| {
                let src_degree = degrees[src];
                let src_node_id = node_ids[src];
                let mut result: Vec<(usize, usize, WeightT)> = if src_degree > number_of_nodes_float {
                    let starting_index = if self.is_directed() { 0 } else { src };
                    (starting_index..number_of_nodes_usize)
                        .into_par_iter()
                        .filter(|&dst| {
                            !(add_selfloops_where_missing && src == dst)
                                && self.has_edge_from_node_ids(src_node_id, node_ids[dst])
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
                            match nodes_map.get(&dst_node_id) {
                                Some(&dst) => {
                                    if src < dst {
                                        Some((
                                            src,
                                            dst,
                                            (1.0 / (src_degree * degrees[dst]).sqrt()) as WeightT,
                                        ))
                                    } else {
                                        None
                                    }
                                }
                                None => None,
                            }
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
                    if complete {
                        vec![(src, dst, weight), (dst, src, weight)]
                    } else {
                        vec![(src, dst, weight)]
                    }
                }
            })
    }

    /// Returns iterator over subsampled symmetric laplacian adjacency matrix on the provided nodes.
    ///
    /// # Implementative details
    /// This implementation works exclusively on undirected graphs where each node
    /// has a selfloop. Additionally, the graph cannot be a multigraph.
    ///
    /// # Safety
    /// The provided nodes are assumed to be unique.
    /// Additionally, the nodes are assumed to exist within this graph instance.
    ///
    /// # Arguments
    /// * `sorted_node_ids`: &[NodeT] - The sorted subsampled nodes.
    ///
    /// TODO: consider caching reciprocal_sqrt_degrees
    pub unsafe fn par_iter_undirected_with_selfloops_subsampled_symmetric_laplacian_adjacency_matrix<
        'a,
    >(
        &'a self,
        sorted_node_ids: &'a [NodeT],
    ) -> Result<impl ParallelIterator<Item = (usize, usize, WeightT)> + 'a> {
        self.must_be_undirected()?;
        self.must_contain_identity_matrix()?;
        self.must_not_be_multigraph()?;
        // Sort the nodes
        // Computing the reciprocal_sqrt_degrees
        let reciprocal_sqrt_degrees =
            Arc::new(self.get_unchecked_reciprocal_sqrt_degrees_from_node_ids(sorted_node_ids));
        //
        let number_of_nodes_usize = sorted_node_ids.len();
        Ok((0..number_of_nodes_usize)
            .into_par_iter()
            .flat_map_iter(move |src| {
                let src_node_id = sorted_node_ids[src];
                let local_reciprocal_sqrt_degrees = reciprocal_sqrt_degrees.clone();
                let mut dst = 0;
                self.iter_unchecked_neighbour_node_ids_from_source_node_id(src_node_id)
                    .take_while(move |_| dst < number_of_nodes_usize)
                    .filter_map(move |dst_node_id| {
                        if src_node_id == dst_node_id {
                            return Some((src, src, 1.0));
                        }
                        while dst < number_of_nodes_usize {
                            match dst_node_id.cmp(&sorted_node_ids[dst]) {
                                std::cmp::Ordering::Equal => {
                                    return Some((
                                        src,
                                        dst,
                                        local_reciprocal_sqrt_degrees[src]
                                            * local_reciprocal_sqrt_degrees[dst],
                                    ));
                                }
                                std::cmp::Ordering::Less => {
                                    return None;
                                }
                                std::cmp::Ordering::Greater => {
                                    dst += 1;
                                }
                            }
                        }
                        None
                    })
            }))
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
    /// * `node_ids`: Vec<NodeT> - The subsampled nodes.
    /// * `edge_weighting_method`: &str - The edge_weighting_method to use to compute the adjacency matrix.
    ///
    /// # Raises
    /// * If the given edge_weighting_method is not supported.
    /// * If The edge_weighting_method requires the graph to be connected but the graph is not.
    /// * If the edge_weighting_method requires the graph to be weighted but the graph is not.
    pub unsafe fn par_iter_subsampled_edge_metric_matrix<'a>(
        &'a self,
        node_ids: &'a [NodeT],
        edge_weighting_method: &str,
    ) -> Result<impl ParallelIterator<Item = (NodeT, usize, NodeT, usize, WeightT)> + 'a> {
        let number_of_nodes = node_ids.len();
        let edge_weighting_method: Result<fn(&Graph, NodeT, NodeT) -> f32> = match edge_weighting_method {
            "unweighted_shortest_path" => {
                self.must_be_connected()?;
                // We make sure that the diameter is precomputed.
                self.get_diameter(None, None)?;
                Ok(|graph: &Graph, src: NodeT, dst: NodeT| -> f32 {
                    if src == dst {
                        return 0.0;
                    }
                    graph
                        .get_unchecked_shortest_path_node_ids_from_node_ids(src, dst, None)
                        .unwrap()
                        .len() as f32
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
                Ok(|graph: &Graph, src: NodeT, dst: NodeT| -> f32 {
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
            "preferential_attachment" => Ok(|graph: &Graph, src: NodeT, dst: NodeT| -> f32 {
                graph.get_unchecked_preferential_attachment_from_node_ids(src, dst, true)
            }),
            "weighted_preferential_attachment" => {
                self.must_have_edge_weights()?;
                Ok(|graph: &Graph, src: NodeT, dst: NodeT| -> f32 {
                    graph.get_unchecked_weighted_preferential_attachment_from_node_ids(
                        src, dst, true,
                    )
                })
            }
            "jaccard_coefficient" => Ok(|graph: &Graph, src: NodeT, dst: NodeT| -> f32 {
                graph.get_unchecked_jaccard_coefficient_from_node_ids(src, dst)
            }),
            "adamic_adar_index" => Ok(|graph: &Graph, src: NodeT, dst: NodeT| -> f32 {
                graph.get_unchecked_adamic_adar_index_from_node_ids(src, dst)
            }),
            "resource_allocation_index" => Ok(|graph: &Graph, src: NodeT, dst: NodeT| -> f32 {
                graph.get_unchecked_resource_allocation_index_from_node_ids(src, dst)
            }),
            "weighted_resource_allocation_index" => {
                self.must_have_edge_weights()?;
                Ok(|graph: &Graph, src: NodeT, dst: NodeT| -> f32 {
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
        Ok((0..number_of_nodes)
            .into_par_iter()
            .flat_map(move |src| (0..number_of_nodes).into_par_iter().map(move |dst| (src, dst)))
            .filter(move |(src, dst)| self.is_directed() || src <= dst)
            .map(move |(src, dst)| {
                let src_node_id = node_ids[src];
                let dst_node_id = node_ids[dst];
                let weight = edge_weighting_method(self, src_node_id, dst_node_id) as WeightT;
                (src_node_id, src, dst_node_id, dst, weight)
            }))
    }
}
