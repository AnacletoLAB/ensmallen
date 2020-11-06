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
        verbose: bool,
    ) -> Result<Graph, String> {
        let pb_edges = get_loading_bar(
            verbose,
            format!(
                "Building edges of graph {} without required attributes",
                self.name
            )
            .as_ref(),
            self.get_edges_number() as usize,
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
                    let src_node_type = self.get_unchecked_node_type(self.get_unchecked_node_id(&src_name));
                    let dst_node_type = self.get_unchecked_node_type(self.get_unchecked_node_id(&dst_name));
                    // If the graph has node types
                    if let (Some(src_nt), Some(dst_nt)) = (src_node_type, dst_node_type){
                        let src_node_type_name = self.get_node_type_name(src_nt).unwrap();
                        let dst_node_type_name = self.get_node_type_name(dst_nt).unwrap();
                        // If the allow node types set was provided
                        if let Some(ants) = &allow_node_types_set {
                            // We check that the current node type name is NOT within the node type set.
                            if !ants.contains(&src_node_type_name) || !ants.contains(&dst_node_type_name){
                                return None;
                            }
                        }
                        // If the deny node types set was provided
                        if let Some(dnts) = &deny_node_types_set {
                            // We check that the current node type name is NOT within the node type set.
                            if dnts.contains(&src_node_type_name) && dnts.contains(&dst_node_type_name){
                                return None;
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
                    .filter_map(|(_, node_name, node_type)| {
                        if singletons && self.is_singleton_string(&node_name).unwrap() {
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
                        if let (Some(ants), Some(nt)) = (&allow_node_types_set, &node_type) {
                            if !ants.contains(nt) {
                                return None;
                            }
                        }
                        if let (Some(dnts), Some(nt)) = (&deny_node_types_set, &node_type) {
                            if dnts.contains(nt) {
                                return None;
                            }
                        }
                        Some(Ok((
                            node_name,
                            match node_types {
                                false => node_type,
                                true => None,
                            },
                        )))
                    }),
            ),
            self.directed,
            true,
            false,
            self.is_multigraph() && edge_types,
            self.get_edges_number(), // Approximation of expected edges number.
            self.get_nodes_number(), // Approximation of expected nodes number.
            match &self.edge_types {
                Some(ets) => ets.has_numeric_ids(),
                None => false,
            },
            self.nodes.has_numeric_ids() && (!singletons || !self.has_singletons()),
            match &self.node_types {
                Some(nts) => nts.has_numeric_ids(),
                None => false,
            },
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
        node_types: Option<Vec<String>>,
        edge_types: Option<Vec<String>>,
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
            let mut edge_types_ids = RoaringBitmap::new();
            edge_types_ids.extend(self.translate_edge_types(ets)?.iter().map(|x| *x as u32));

            let pb = get_loading_bar(
                verbose,
                &format!(
                    "Computing which components are to keep for the graph {}",
                    &self.name
                ),
                self.get_edges_number() as usize,
            );

            self.get_edges_triples(self.directed)
                .progress_with(pb)
                .for_each(|(_, src, dst, edge_type)| {
                    if let Some(et) = edge_type {
                        if edge_types_ids.contains(et as u32) {
                            keep_components.insert(components_vector[src as usize]);
                            keep_components.insert(components_vector[dst as usize]);
                        }
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
            self.get_edges_number() as usize,
        );

        Graph::build_graph(
            self.get_edges_quadruples(true).progress_with(pb).filter_map(
                |(_, src, dst, edge_type, weight)| match keep_components
                    .contains(components_vector[src as usize])
                {
                    true => Some(Ok((src, dst, edge_type, weight))),
                    false => None,
                },
            ),
            self.get_edges_number(),
            self.nodes.clone(),
            self.node_types.clone(),
            match &self.edge_types {
                Some(ets) => Some(ets.vocabulary.clone()),
                None => None,
            },
            self.directed,
            self.name.clone(),
            true,
        )
    }
}
