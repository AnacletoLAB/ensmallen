use super::*;
use counter::Counter;
use indicatif::ProgressIterator;
use roaring::RoaringBitmap;
use std::collections::HashSet;

/// # remove.
impl Graph {
    /// Returns a **NEW** Graph that does not have the required attributes.
    ///
    /// ## Implementation details
    ///
    /// ### How the collapse of multigraphs is handled
    /// We keep only the first edge when a multigraph is collapsed while removing
    /// the edge types, in the order provided when first reading from the CSV file.
    ///
    /// ### Generation of new singleton nodes when removeping edges
    /// Some of the remove operations allowed in this method might lead to the
    /// generation of new singleton nodes that will not be handled within this
    /// function call even if you provide the flag singletons to true, but you
    /// will need to call the method again if you want to get reed of also those
    /// newly created singleton nodes.
    ///
    /// # Arguments
    /// * `allow_nodes_set`: Option<HashSet<String>> - Optional set of nodes names to keep.
    /// * `deny_nodes_set`: Option<HashSet<String>> - Optional set of nodes names to remove.
    /// * `allow_node_types_set`: Option<HashSet<String>> - Optional set of node type names to keep.
    /// * `deny_node_types_set`: Option<HashSet<String>> - Optional set of node type names to remove.
    /// * `allow_edge_set`: Option<HashSet<EdgeT>>- Optional set of numeric edge IDs to keep.
    /// * `deny_edge_set`: Option<HashSet<EdgeT>>- Optional set of numeric edge IDs to remove.
    /// * `allow_edge_types_set`: Option<HashSet<String>> - Optional set of edge type names to keep.
    /// * `deny_edge_types_set`: Option<HashSet<String>> - Optional set of edge type names to remove.
    /// * `weights`: bool - Wether to remove the weights.
    /// * `node_types`: bool - Wether to remove the node types.
    /// * `edge_types`: bool - Wether to remove the edge types.
    /// * `singletons`: bool - Wether to remove the singleton nodes.
    /// * `selfloops`: bool - Wether to remove edges with self-loops.
    /// * `verbose`: bool - Wether to show a loading bar while building the graph.
    ///
    pub fn remove(
        &self,
        allow_nodes_set: Option<HashSet<String>>,
        deny_nodes_set: Option<HashSet<String>>,
        allow_node_types_set: Option<HashSet<String>>,
        deny_node_types_set: Option<HashSet<String>>,
        allow_edge_set: Option<HashSet<EdgeT>>,
        deny_edge_set: Option<HashSet<EdgeT>>,
        allow_edge_types_set: Option<HashSet<String>>,
        deny_edge_types_set: Option<HashSet<String>>,
        weights: bool,
        node_types: bool,
        edge_types: bool,
        singletons: bool,
        selfloops: bool,
        verbose: bool,
    ) -> Result<Graph, String> {
        let pb_edges = get_loading_bar(
            verbose,
            format!(
                "Building edges of graph {} without required attributes",
                self.name
            )
            .as_ref(),
            self.get_directed_edges_number() as usize,
        );
        let pb_nodes = get_loading_bar(
            verbose,
            format!(
                "Building nodes of graph {} without required attributes",
                self.name
            )
            .as_ref(),
            self.get_nodes_number() as usize,
        );

        Graph::from_string_sorted(
            self.get_edges_string_quadruples(true)
                .progress_with(pb_edges)
                .filter_map(|(edge_id, src_name, dst_name, edge_type, weight)| {
                    // If an allow edge set was provided
                    if let Some(aes) = &allow_edge_set {
                        // We check that the current edge ID is within the edge set.
                        if !aes.contains(&edge_id) {
                            return None;
                        }
                    }
                    // If selfloops need to be filtered out.
                    if selfloops && src_name == dst_name {
                        return None;
                    }
                    // If a deny edge set was provided
                    if let Some(des) = &deny_edge_set {
                        // We check that the current edge ID is NOT within the edge set.
                        if des.contains(&edge_id) {
                            return None;
                        }
                    }
                    // If an allow nodes set was provided
                    if let Some(ans) = &allow_nodes_set {
                        // We check that the current source or destination node name is within the edge set.
                        if !ans.contains(&src_name) || !ans.contains(&dst_name) {
                            return None;
                        }
                    }
                    // If a deny nodes set was provided
                    if let Some(dns) = &deny_nodes_set {
                        // We check that the current source or destination node name is NOT within the edge set.
                        if dns.contains(&src_name) || dns.contains(&dst_name) {
                            return None;
                        }
                    }
                    // If the allow edge types set was provided
                    if let (Some(aets), Some(et)) = (&allow_edge_types_set, &edge_type) {
                        // We check that the current edge type name is within the edge type set.
                        if !aets.contains(et) {
                            return None;
                        }
                    }
                    // If the deny edge types set was provided
                    if let (Some(dets), Some(et)) = (&deny_edge_types_set, &edge_type) {
                        // We check that the current edge type name is NOT within the edge type set.
                        if dets.contains(et) {
                            return None;
                        }
                    }

                    if allow_node_types_set.is_some() || deny_node_types_set.is_some() {
                        let src_node_type =
                            self.get_unchecked_node_type_id_by_node_id(self.get_unchecked_node_id(&src_name));
                        let dst_node_type =
                            self.get_unchecked_node_type_id_by_node_id(self.get_unchecked_node_id(&dst_name));
                        // If the graph has node types
                        if let (Some(src_nt), Some(dst_nt)) = (src_node_type, dst_node_type) {
                            let node_type_names = self
                                .translate_node_type_id_vector(
                                    src_nt.into_iter().chain(dst_nt.into_iter()).collect(),
                                )
                                .unwrap();
                            // If the allow node types set was provided
                            if let Some(ants) = &allow_node_types_set {
                                // We check that the current node type name is NOT within the node type set.
                                if node_type_names
                                    .iter()
                                    .any(|node_type_name| !ants.contains(node_type_name))
                                {
                                    return None;
                                }
                            }
                            // If the deny node types set was provided
                            if let Some(dnts) = &deny_node_types_set {
                                // We check that the current node type name is NOT within the node type set.
                                if node_type_names
                                    .iter()
                                    .any(|node_type_name| dnts.contains(node_type_name))
                                {
                                    return None;
                                }
                            }
                        }
                    }

                    Some(Ok((
                        src_name,
                        dst_name,
                        match edge_types {
                            false => edge_type,
                            true => None,
                        },
                        match weights {
                            false => weight,
                            true => None,
                        },
                    )))
                }),
            Some(
                self.get_nodes_names_iter()
                    .progress_with(pb_nodes)
                    .filter_map(|(node_id, node_name, node_type_names)| {
                        if singletons && self.is_singleton_by_nide_name(&node_name).unwrap() {
                            return None;
                        }
                        // If singletons and selfloops need to be removed.
                        // We need to check all the destinations of the node if they are equal
                        // with the source node, as in multigraphs there may be multiple selfloops of different
                        // node types.
                        if singletons && selfloops && self.is_singleton_with_self_loops(node_id) {
                            return None;
                        }
                        if let Some(ans) = &allow_nodes_set {
                            if !ans.contains(&node_name) {
                                return None;
                            }
                        }
                        if let Some(dns) = &deny_nodes_set {
                            if dns.contains(&node_name) {
                                return None;
                            }
                        }
                        if let (Some(ants), Some(nts)) = (&allow_node_types_set, &node_type_names) {
                            // We check that the current node type name is NOT within the node type set.
                            if nts
                                .iter()
                                .any(|node_type_name| !ants.contains(node_type_name))
                            {
                                return None;
                            }
                        }
                        if let (Some(dnts), Some(nts)) = (&deny_node_types_set, &node_type_names) {
                            // We check that the current node type name is NOT within the node type set.
                            if nts
                                .iter()
                                .any(|node_type_name| dnts.contains(node_type_name))
                            {
                                return None;
                            }
                        }
                        Some(Ok((
                            node_name,
                            match node_types {
                                false => node_type_names,
                                true => None,
                            },
                        )))
                    }),
            ),
            self.directed,
            false,
            false,
            true,
            true,
            self.get_directed_edges_number() as usize, // Approximation of expected edges number.
            self.get_nodes_number(), // Approximation of expected nodes number.
            false,
            false,
            false,
            false,
            self.has_node_types() && !node_types,
            self.has_edge_types() && !edge_types,
            self.has_weights() && !weights,
            self.get_name(),
        )
    }

    /// remove all the components that are not connected to interesting
    /// nodes and edges.
    ///
    /// # Arguments
    /// * `node_names` : Option<Vec<String>> - The name of the nodes of which components to keep.
    /// * `node_types` : Option<Vec<String>> - The types of the nodes of which components to keep.
    /// * `edge_types` : Option<Vec<String>> - The types of the edges of which components to keep.
    /// * `minimum_component_size`: Option<NodeT> - Optional, Minimum size of the components to keep.
    /// * `top_k_components`: Option<NodeT> - Optional, number of components to keep sorted by number of nodes.
    /// * `verbose`: bool - Wether to show the loading bar.
    pub fn remove_components(
        &self,
        node_names: Option<Vec<String>>,
        node_types: Option<Vec<Option<String>>>,
        edge_types: Option<Vec<Option<String>>>,
        minimum_component_size: Option<NodeT>,
        top_k_components: Option<NodeT>,
        verbose: bool,
    ) -> Result<Graph, String> {
        let mut keep_components = RoaringBitmap::new();
        let components_vector = self.get_node_components_vector(verbose);

        // Extend the components so the include the given node Ids and node types.
        if let Some(node_ids) = self.get_filter_bitmap(node_names, node_types)? {
            keep_components.extend(
                node_ids
                    .iter()
                    .map(|node_id| components_vector[node_id as usize]),
            );
        }

        // Extend the components to keep those that include the given edge types.
        if let Some(ets) = edge_types {
            let edge_types_ids: HashSet<Option<EdgeTypeT>> = self.translate_edge_types(ets)?.into_iter().collect();

            let pb = get_loading_bar(
                verbose,
                &format!(
                    "Computing which components are to keep for the graph {}",
                    &self.name
                ),
                self.get_directed_edges_number() as usize,
            );

            self.get_edges_triples(self.directed)
                .progress_with(pb)
                .for_each(|(_, src, dst, edge_type)| {
                    if edge_types_ids.contains(&edge_type) {
                        keep_components.insert(components_vector[src as usize]);
                        keep_components.insert(components_vector[dst as usize]);
                    }
                });
        }

        // Retrieve minimal size of the smallest top k components
        let components_counts = Counter::init(components_vector.clone()).most_common();
        let updated_min_component_size = match top_k_components {
            Some(tkc) => Some(match components_counts.len() < tkc as usize {
                true => components_counts.last().unwrap().1,
                false => components_counts.get(tkc as usize).unwrap().1,
            }),
            None => minimum_component_size,
        };

        // Remove components that are smaller than given amount
        if let Some(mcs) = updated_min_component_size {
            components_counts
                .iter()
                .for_each(|(component, component_size)| {
                    if *component_size < mcs {
                        keep_components.remove(*component);
                    }
                });
        }

        let pb = get_loading_bar(
            verbose,
            &format!("removing components for the graph {}", &self.name),
            self.get_directed_edges_number() as usize,
        );

        Graph::build_graph(
            self.get_edges_quadruples(true)
                .progress_with(pb)
                .filter_map(|(_, src, dst, edge_type, weight)| {
                    match keep_components.contains(components_vector[src as usize]) {
                        true => Some(Ok((src, dst, edge_type, weight))),
                        false => None,
                    }
                }),
            self.get_directed_edges_number() as usize,
            self.nodes.clone(),
            self.node_types.clone(),
            self.edge_types.as_ref().map(|ets| ets.vocabulary.clone()),
            self.directed,
            true,
            self.name.clone(),
            true,
            self.has_edge_types(),
            self.has_weights(),
        )
    }
}
