use crate::constructors::build_graph_from_integers;
use crate::constructors::build_graph_from_strings_without_type_iterators;
use rayon::iter::IntoParallelIterator;

use super::*;
use rayon::iter::ParallelIterator;

impl Graph {
    /// Returns a **NEW** Graph that does not have the required attributes.
    ///
    /// # Arguments
    /// * `node_ids_to_keep`: Option<Vec<NodeT>> - List of node IDs to keep during filtering.
    /// * `node_ids_to_filter`: Option<Vec<NodeT>> - List of node IDs to remove during filtering.
    /// * `node_type_ids_to_keep`: Option<Vec<Option<Vec<NodeTypeT>>>> - List of node type IDs to keep during filtering. The node types must match entirely the given node types vector provided.
    /// * `node_type_ids_to_filter`: Option<Vec<Option<Vec<NodeTypeT>>>> - List of node type IDs to remove during filtering. The node types must match entirely the given node types vector provided.
    /// * `node_type_id_to_keep`: Option<Vec<Option<NodeTypeT>>> - List of node type IDs to keep during filtering. Any of node types must match with one of the node types given.
    /// * `node_type_id_to_filter`: Option<Vec<Option<NodeTypeT>>> - List of node type IDs to remove during filtering. Any of node types must match with one of the node types given.
    /// * `edge_ids_to_keep`: Option<Vec<EdgeT>> - List of edge IDs to keep during filtering.
    /// * `edge_ids_to_filter`: Option<Vec<EdgeT>> - List of edge IDs to remove during filtering.
    /// * `edge_node_ids_to_keep`: Option<Vec<(NodeT, NodeT)>> - List of tuple of node IDs to keep during filtering.
    /// * `edge_node_ids_to_filter`: Option<Vec<(NodeT, NodeT)>> - List of tuple of node IDs to remove during filtering.
    /// * `edge_type_ids_to_keep`: Option<Vec<Option<EdgeTypeT>>> - List of edge type IDs to keep during filtering.
    /// * `edge_type_ids_to_filter`: Option<Vec<Option<EdgeTypeT>>> - List of edge type IDs to remove during filtering.
    /// * `min_edge_weight`: Option<WeightT> - Minimum edge weight. Values lower than this are removed.
    /// * `max_edge_weight`: Option<WeightT> - Maximum edge weight. Values higher than this are removed.
    /// * `filter_singleton_nodes`: Option<bool> - Whether to filter out singleton nodes.
    /// * `filter_singleton_nodes_with_selfloop`: Option<bool> - Whether to filter out singleton nodes with selfloops.
    /// * `filter_selfloops`: Option<bool> - Whether to filter out selfloops.
    /// * `filter_parallel_edges`: Option<bool> - Whether to filter out parallel edges.
    /// * `verbose`: Option<bool> - Whether to show loading bar while building the graphs.
    ///
    /// ## Implementation details
    ///
    /// ### How the collapse of multigraphs is handled
    /// We keep only the first edge when a multigraph is collapsed while removing
    /// the edge types, in the order provided when first reading from the CSV file.
    ///
    /// ### Generation of new singleton nodes when removing edges
    /// Some of the remove operations allowed in this method might lead to the
    /// generation of new singleton nodes that will not be handled within this
    /// function call even if you provide the flag singletons to true, but you
    /// will need to call the method again if you want to get reed of also those
    /// newly created singleton nodes.
    ///
    pub fn filter_from_ids(
        &self,
        node_ids_to_keep: Option<Vec<NodeT>>,
        node_ids_to_filter: Option<Vec<NodeT>>,
        node_type_ids_to_keep: Option<Vec<Option<Vec<NodeTypeT>>>>,
        node_type_ids_to_filter: Option<Vec<Option<Vec<NodeTypeT>>>>,
        node_type_id_to_keep: Option<Vec<Option<NodeTypeT>>>,
        node_type_id_to_filter: Option<Vec<Option<NodeTypeT>>>,
        edge_ids_to_keep: Option<Vec<EdgeT>>,
        edge_ids_to_filter: Option<Vec<EdgeT>>,
        edge_node_ids_to_keep: Option<Vec<(NodeT, NodeT)>>,
        edge_node_ids_to_filter: Option<Vec<(NodeT, NodeT)>>,
        edge_type_ids_to_keep: Option<Vec<Option<EdgeTypeT>>>,
        edge_type_ids_to_filter: Option<Vec<Option<EdgeTypeT>>>,
        min_edge_weight: Option<WeightT>,
        max_edge_weight: Option<WeightT>,
        filter_singleton_nodes: Option<bool>,
        filter_singleton_nodes_with_selfloop: Option<bool>,
        filter_selfloops: Option<bool>,
        filter_parallel_edges: Option<bool>,
    ) -> Result<Graph> {
        if !self.is_directed() && (edge_ids_to_keep.is_some() || edge_ids_to_filter.is_some()) {
            return Err(concat!(
                "It is not possible to filter by edge ids on an undirected ",
                "graph as the resulting graph may become a directed graph.\n",
                "If you still want to remove this edges, convert the graph ",
                "to directed by using `to_directed` or `to_directed_inplace`."
            )
            .to_string());
        }

        let filter_singleton_nodes = filter_singleton_nodes.unwrap_or(false);
        let filter_singleton_nodes_with_selfloop =
            filter_singleton_nodes_with_selfloop.unwrap_or(false);
        let filter_selfloops = filter_selfloops.unwrap_or(false);
        let filter_parallel_edges = filter_parallel_edges.unwrap_or(false);

        let has_node_filters = self.has_nodes()
            && [
                node_ids_to_keep.is_some(),
                node_ids_to_filter.is_some(),
                node_type_ids_to_keep.is_some(),
                node_type_ids_to_filter.is_some(),
                node_type_id_to_keep.is_some(),
                node_type_id_to_filter.is_some(),
                filter_singleton_nodes && self.has_singleton_nodes(),
                filter_singleton_nodes_with_selfloop && self.has_singleton_nodes_with_selfloops(),
            ]
            .iter()
            .any(|value| *value);

        let has_edge_filters = self.has_edges()
            && [
                edge_ids_to_keep.is_some(),
                edge_ids_to_filter.is_some(),
                edge_node_ids_to_keep.is_some(),
                edge_node_ids_to_filter.is_some(),
                edge_type_ids_to_keep.is_some(),
                edge_type_ids_to_filter.is_some(),
                (min_edge_weight.is_some() || max_edge_weight.is_some()) && self.has_edge_weights(),
                filter_selfloops && self.has_selfloops(),
                filter_parallel_edges && self.is_multigraph(),
                filter_singleton_nodes_with_selfloop && self.has_singleton_nodes_with_selfloops(),
            ]
            .iter()
            .any(|value| *value);

        let min_edge_weight = min_edge_weight.unwrap_or(WeightT::NEG_INFINITY);
        let max_edge_weight = max_edge_weight.unwrap_or(WeightT::INFINITY);

        let edge_filter = |(edge_id, src, dst, edge_type_id, weight): &(
            EdgeT,
            NodeT,
            NodeT,
            Option<EdgeTypeT>,
            Option<WeightT>,
        )| {
            edge_ids_to_keep.as_ref().map_or(true, |edge_ids| edge_ids.contains(edge_id)) &&
            edge_ids_to_filter.as_ref().map_or(true, |edge_ids| !edge_ids.contains(edge_id)) &&
            // If parallel edges need to be filtered out.
            (!filter_parallel_edges || {
                if *edge_id == 0 {
                    true
                } else {
                    let (last_src, last_dst) = unsafe {self.get_unchecked_node_ids_from_edge_id(edge_id-1)};
                    last_src != *src || last_dst != *dst
                }
            }) &&
            // If selfloops need to be filtered out.
            (!filter_selfloops || src != dst) &&
            // If singleton nodes with selfloops need to be filtered out
            (!filter_singleton_nodes_with_selfloop || src != dst || unsafe{!self.is_unchecked_singleton_with_selfloops_from_node_id(*src)}) &&
            // If the allow edge types set was provided
            edge_node_ids_to_keep.as_ref().map_or(true, |edge_node_ids| edge_node_ids.contains(&(*src, *dst)) || !self.is_directed() && edge_node_ids.contains(&(*dst, *src))) &&
            // If the deny edge types set was provided
            !edge_node_ids_to_filter.as_ref().map_or(false, |edge_node_ids| edge_node_ids.contains(&(*src, *dst)) || !self.is_directed() && edge_node_ids.contains(&(*dst, *src))) &&
            edge_type_ids_to_keep.as_ref().map_or(true, |ntitk| ntitk.contains(edge_type_id)) &&
            edge_type_ids_to_filter.as_ref().map_or(true, |ntitf| !ntitf.contains(edge_type_id)) &&
            weight.map_or(true, |weight| weight >= min_edge_weight && weight <= max_edge_weight)
        };

        let node_filter = |(node_id, _, node_type_ids, _): &(
            NodeT,
            String,
            Option<Vec<NodeTypeT>>,
            Option<Vec<String>>,
        )| {
            node_ids_to_keep
                .as_ref()
                .map_or(true, |nitk| nitk.contains(node_id))
                && node_ids_to_filter
                    .as_ref()
                    .map_or(true, |nitf| !nitf.contains(node_id))
                && node_type_ids_to_keep
                    .as_ref()
                    .map_or(true, |ntitk| ntitk.contains(node_type_ids))
                && node_type_ids_to_filter
                    .as_ref()
                    .map_or(true, |ntitf| !ntitf.contains(node_type_ids))
                && node_type_id_to_keep
                    .as_ref()
                    .map_or(true, |ntitk| match node_type_ids {
                        Some(node_type_ids) => node_type_ids
                            .iter()
                            .any(|node_type_id| ntitk.contains(&Some(*node_type_id))),
                        None => ntitk.contains(&None),
                    })
                && !node_type_id_to_filter
                    .as_ref()
                    .map_or(false, |ntitf| match node_type_ids {
                        Some(node_type_ids) => node_type_ids
                            .iter()
                            .any(|node_type_id| ntitf.contains(&Some(*node_type_id))),
                        None => ntitf.contains(&None),
                    })
                && !(filter_singleton_nodes && unsafe{self.is_unchecked_singleton_from_node_id(*node_id)})
                && !(filter_singleton_nodes
                    && filter_selfloops
                    && unsafe{self.is_unchecked_singleton_with_selfloops_from_node_id(*node_id)}) &&
                // If singleton nodes with selfloops need to be filtered out
                (!filter_singleton_nodes_with_selfloop || unsafe{!self.is_unchecked_singleton_with_selfloops_from_node_id(*node_id)})
        };

        let mut edges_number = self.get_directed_edges_number();

        if filter_parallel_edges {
            edges_number -= self.get_parallel_edges_number();
            if filter_selfloops {
                edges_number -= self.get_unique_selfloops_number() as EdgeT;
            }
        } else if filter_selfloops {
            edges_number -= self.get_selfloops_number();
        }

        match (has_node_filters, has_edge_filters) {
            (false, false) => Ok(self.clone()),
            (false, true) => build_graph_from_integers(
                Some(
                    self.par_iter_directed_edge_node_ids_and_edge_type_id_and_edge_weight()
                        .filter(edge_filter)
                        .map(|(_, src, dst, edge_type, weight)| {
                            // We use 0 as index because this edge list
                            // is filtered and therefore there will be gaps
                            // in between the various edges and we cannot build
                            // an Elias-Fano object in parallell with gaps.
                            (0, (src, dst, edge_type, weight.unwrap_or(WeightT::NAN)))
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
                Some(edges_number),
                true,
                self.has_selfloops(),
                self.get_name(),
            ),
            (true, _) => {
                let nodes_iterator: ItersWrapper<_, std::iter::Empty<_>, _> =
                    ItersWrapper::Parallel(
                        self.par_iter_node_names_and_node_type_names()
                            .filter(node_filter)
                            .map(|(_, node_name, _, node_types)| {
                                Ok((0 as usize, (node_name, node_types)))
                            }),
                    );
                let edges_iterator: ItersWrapper<_, std::iter::Empty<_>, _> = ItersWrapper::Parallel(
                    self.par_iter_edge_node_names_and_edge_type_name_and_edge_weight(true)
                        .filter(
                            |(
                                edge_id,
                                src,
                                src_name,
                                dst,
                                dst_name,
                                edge_type,
                                _,
                                weight,
                            )| unsafe {
                                edge_filter(&(*edge_id, *src, *dst, *edge_type, *weight))
                                    && node_filter(&(
                                        *src,
                                        src_name.clone(),
                                        self.get_unchecked_node_type_ids_from_node_id(*src),
                                        None,
                                    ))
                                    && node_filter(&(
                                        *dst,
                                        dst_name.clone(),
                                        self.get_unchecked_node_type_ids_from_node_id(*dst),
                                        None,
                                    ))
                            },
                        )
                        .map(|(_, _, src_name, _, dst_name, _, edge_type_name, weight)| {
                            Ok((
                                0 as usize,
                                (
                                    src_name,
                                    dst_name,
                                    edge_type_name,
                                    weight.unwrap_or(WeightT::NAN),
                                ),
                            ))
                        }),
                );
                build_graph_from_strings_without_type_iterators(
                    self.has_node_types(),
                    Some(nodes_iterator),
                    // The number of nodes is unknown because of the filter
                    // it may be possible, in some cases, to get this value by
                    // further expanding this filtering method.
                    None,
                    true,
                    false,
                    false,
                    None,
                    self.has_edge_types(),
                    Some(edges_iterator),
                    self.has_edge_weights(),
                    self.is_directed(),
                    Some(true),
                    Some(true),
                    Some(false),
                    Some(false),
                    // The number of edges is unknown because of the filter
                    // it may be possible, in some cases, to get this value by
                    // further expanding this filtering method.
                    None,
                    None,
                    None,
                    None,
                    None,
                    true,
                    self.has_selfloops(),
                    self.get_name(),
                )
            }
        }
    }

    /// Returns a **NEW** Graph that does not have the required attributes.
    ///
    /// # Arguments
    /// * `node_names_to_keep`: Option<Vec<&str>> - List of node names to keep during filtering.
    /// * `node_names_to_filter`: Option<Vec<&str>> - List of node names to remove during filtering.
    /// * `node_type_names_to_keep`: Option<Vec<Option<Vec<&str>>>> - List of node type names to keep during filtering. The node types must match entirely the given node types vector provided.
    /// * `node_type_names_to_filter`: Option<Vec<Option<Vec<&str>>>> - List of node type names to remove during filtering. The node types must match entirely the given node types vector provided.
    /// * `node_type_name_to_keep`: Option<Vec<Option<String>>> - List of node type name to keep during filtering. Any of node types must match with one of the node types given.
    /// * `node_type_name_to_filter`: Option<Vec<Option<String>>> - List of node type name to remove during filtering. Any of node types must match with one of the node types given.
    /// * `edge_node_names_to_keep`: Option<Vec<(&str, &str)>> - List of tuple of node names to keep during filtering.
    /// * `edge_node_names_to_filter`: Option<Vec<(&str, &str)>> - List of tuple of node names to remove during filtering.
    /// * `edge_type_names_to_keep`: Option<Vec<Option<String>>> - List of edge type names to keep during filtering.
    /// * `edge_type_names_to_filter`: Option<Vec<Option<String>>> - List of edge type names to remove during filtering.
    /// * `min_edge_weight`: Option<WeightT> - Minimum edge weight. Values lower than this are removed.
    /// * `max_edge_weight`: Option<WeightT> - Maximum edge weight. Values higher than this are removed.
    /// * `filter_singleton_nodes`: Option<bool> - Whether to filter out singletons.
    /// * `filter_singleton_nodes_with_selfloop`: Option<bool> - Whether to filter out singleton nodes with selfloops.
    /// * `filter_selfloops`: Option<bool> - Whether to filter out selfloops.
    /// * `filter_parallel_edges`: Option<bool> - Whether to filter out parallel edges.
    /// * `verbose`: Option<bool> - Whether to show loading bar while building the graphs.
    ///
    /// ## Implementation details
    ///
    /// ### How the collapse of multigraphs is handled
    /// We keep only the first edge when a multigraph is collapsed while removing
    /// the edge types, in the order provided when first reading from the CSV file.
    ///
    /// ### Generation of new singleton nodes when removing edges
    /// Some of the remove operations allowed in this method might lead to the
    /// generation of new singleton nodes that will not be handled within this
    /// function call even if you provide the flag singletons to true, but you
    /// will need to call the method again if you want to get reed of also those
    /// newly created singleton nodes.
    ///
    pub fn filter_from_names(
        &self,
        node_names_to_keep: Option<Vec<&str>>,
        node_names_to_filter: Option<Vec<&str>>,
        node_type_names_to_keep: Option<Vec<Option<Vec<&str>>>>,
        node_type_names_to_filter: Option<Vec<Option<Vec<&str>>>>,
        node_type_name_to_keep: Option<Vec<Option<String>>>,
        node_type_name_to_filter: Option<Vec<Option<String>>>,
        edge_node_names_to_keep: Option<Vec<(&str, &str)>>,
        edge_node_names_to_filter: Option<Vec<(&str, &str)>>,
        edge_type_names_to_keep: Option<Vec<Option<String>>>,
        edge_type_names_to_filter: Option<Vec<Option<String>>>,
        min_edge_weight: Option<WeightT>,
        max_edge_weight: Option<WeightT>,
        filter_singleton_nodes: Option<bool>,
        filter_singleton_nodes_with_selfloop: Option<bool>,
        filter_selfloops: Option<bool>,
        filter_parallel_edges: Option<bool>,
    ) -> Result<Graph> {
        self.filter_from_ids(
            node_names_to_keep.map_or(Ok::<_, String>(None), |nntk| {
                Ok(Some(self.get_node_ids_from_node_names(nntk)?))
            })?,
            node_names_to_filter.map_or(Ok::<_, String>(None), |nntf| {
                Ok(Some(self.get_node_ids_from_node_names(nntf)?))
            })?,
            node_type_names_to_keep.map_or(Ok::<_, String>(None), |ntntk| {
                Ok(Some(
                    self.get_multiple_node_type_ids_from_node_type_names(ntntk)?,
                ))
            })?,
            node_type_names_to_filter.map_or(Ok::<_, String>(None), |ntntf| {
                Ok(Some(
                    self.get_multiple_node_type_ids_from_node_type_names(ntntf)?,
                ))
            })?,
            node_type_name_to_keep.map_or(Ok::<_, String>(None), |ntntf| {
                Ok(Some(self.get_node_type_ids_from_node_type_names(ntntf)?))
            })?,
            node_type_name_to_filter.map_or(Ok::<_, String>(None), |ntntf| {
                Ok(Some(self.get_node_type_ids_from_node_type_names(ntntf)?))
            })?,
            None,
            None,
            edge_node_names_to_keep.map_or(Ok::<_, String>(None), |enntk| {
                Ok(Some(self.get_edge_node_ids_from_edge_node_names(enntk)?))
            })?,
            edge_node_names_to_filter.map_or(Ok::<_, String>(None), |enntf| {
                Ok(Some(self.get_edge_node_ids_from_edge_node_names(enntf)?))
            })?,
            edge_type_names_to_keep.map_or(Ok::<_, String>(None), |etnk| {
                Ok(Some(self.get_edge_type_ids_from_edge_type_names(etnk)?))
            })?,
            edge_type_names_to_filter.map_or(Ok::<_, String>(None), |etnf| {
                Ok(Some(self.get_edge_type_ids_from_edge_type_names(etnf)?))
            })?,
            min_edge_weight,
            max_edge_weight,
            filter_singleton_nodes,
            filter_singleton_nodes_with_selfloop,
            filter_selfloops,
            filter_parallel_edges,
        )
    }

    /// Returns new graph without unknown node types and relative nodes.
    ///
    /// Note that this method will remove ALL nodes labeled with unknown node
    /// type!
    ///
    pub fn drop_unknown_node_types(&self) -> Graph {
        self.filter_from_ids(
            None,
            None,
            None,
            None,
            None,
            Some(vec![None]),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
        )
        .unwrap()
    }

    /// Returns new graph without unknown edge types and relative edges.
    ///
    /// Note that this method will remove ALL edges labeled with unknown edge
    /// type!
    ///
    pub fn drop_unknown_edge_types(&self) -> Graph {
        self.filter_from_ids(
            None,
            None,
            None,
            None,
            None,
            Some(vec![None]),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
        )
        .unwrap()
    }

    /// Returns new graph without singleton nodes.
    ///
    /// A node is singleton when does not have neither incoming or outgoing edges.
    ///
    pub fn drop_singleton_nodes(&self) -> Graph {
        self.filter_from_ids(
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            Some(true),
            None,
            None,
            None,
        )
        .unwrap()
    }

    /// Returns new graph without tendrils.
    pub fn drop_tendrils(&self) -> Result<Graph> {
        self.filter_from_ids(
            None,
            Some(
                self.par_iter_tendrils(Some(1), Some(true))?
                    .flat_map(|tendril| tendril.get_tendril_node_ids().into_par_iter())
                    .collect(),
            ),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
        )
    }

    /// Returns new graph without tendrils.
    pub fn drop_dendritic_trees(&self) -> Result<Graph> {
        self.filter_from_ids(
            None,
            Some(
                self.get_dendritic_trees()?
                    .into_par_iter()
                    .flat_map(|dendric_tree| {
                        dendric_tree.get_dentritic_trees_node_ids()
                    })
                    .collect(),
            ),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
        )
    }

    /// Returns new graph without isomorphic nodes, only keeping the smallest node ID of each group.
    ///
    /// # Arguments
    /// * `minimum_node_degree`: Option<NodeT> - Minimum node degree for the topological synonims. By default equal to 2.
    pub fn drop_isomorphic_nodes(&self, minimum_node_degree: Option<NodeT>) -> Graph {
        let minimum_node_degree = minimum_node_degree.unwrap_or(2);
        self.filter_from_ids(
            None,
            Some(
                self.par_iter_isomorphic_node_ids_groups(Some(minimum_node_degree), None)
                    .flat_map(|mut group| {
                        group.pop();
                        group.into_par_iter()
                    })
                    .collect(),
            ),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
        )
        .unwrap()
    }

    /// Returns new graph without singleton nodes with selfloops.
    ///
    /// A node is singleton with selfloop when does not have neither incoming or outgoing edges.
    ///
    pub fn drop_singleton_nodes_with_selfloops(&self) -> Graph {
        self.filter_from_ids(
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            Some(true),
            None,
            None,
        )
        .unwrap()
    }

    /// Returns new graph without disconnected nodes.
    ///
    /// A disconnected node is a node with no connection to any other node.
    ///
    pub fn drop_disconnected_nodes(&self) -> Graph {
        self.filter_from_ids(
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            Some(true),
            Some(true),
            None,
            None,
        )
        .unwrap()
    }

    /// Returns new graph without selfloops.
    ///
    pub fn drop_selfloops(&self) -> Graph {
        self.filter_from_ids(
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            Some(true),
            None,
        )
        .unwrap()
    }

    /// Returns new graph without parallel edges.
    pub fn drop_parallel_edges(&self) -> Graph {
        self.filter_from_ids(
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            Some(true),
        )
        .unwrap()
    }
}
