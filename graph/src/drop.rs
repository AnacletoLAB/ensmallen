use super::*;
use counter::Counter;
use indicatif::ProgressIterator;
use roaring::RoaringBitmap;
use std::collections::HashSet;

/// # Drop.
impl Graph {
    /// Returns a **NEW** Graph that does not have the required attributes.
    ///
    /// ## Implementation details
    ///
    /// ### How the collapse of multigraphs is handled
    /// We keep only the first edge when a multigraph is collapsed while removing
    /// the edge types, in the order provided when first reading from the CSV file.
    ///
    /// ### Generation of new singleton nodes when dropping edges
    /// Some of the drop operations allowed in this method might lead to the
    /// generation of new singleton nodes that will not be handled within this
    /// function call even if you provide the flag singletons to true, but you
    /// will need to call the method again if you want to get reed of also those
    /// newly created singleton nodes.
    ///
    /// # Arguments
    /// * `allow_nodes_set`: Option<HashSet<String>> - Optional set of nodes names to keep.
    /// * `deny_nodes_set`: Option<HashSet<String>> - Optional set of nodes names to drop.
    /// * `allow_edge_set`: Option<HashSet<EdgeT>>- Optional set of numeric edge IDs to keep.
    /// * `deny_edge_set`: Option<HashSet<EdgeT>>- Optional set of numeric edge IDs to drop.
    /// * `weights`: bool - Wether to drop the weights.
    /// * `node_types`: bool - Wether to drop the node types.
    /// * `edge_types`: bool - Wether to drop the edge types.
    /// * `singletons`: bool - Wether to drop the singleton nodes.
    /// * `verbose`: bool - Wether to show a loading bar while building the graph.
    ///
    pub fn drop(
        &self,
        allow_nodes_set: Option<HashSet<String>>,
        deny_nodes_set: Option<HashSet<String>>,
        allow_edge_set: Option<HashSet<EdgeT>>,
        deny_edge_set: Option<HashSet<EdgeT>>,
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
                "Building names of graph {} without required attributes",
                self.name
            )
            .as_ref(),
            self.get_nodes_number() as usize,
        );

        Graph::from_string_unsorted(
            self.get_edges_string_quadruples()
                .progress_with(pb_edges)
                .filter_map(|(edge_id, src_name, dst_name, edge_type, weight)| {
                    if let Some(aes) = &allow_edge_set {
                        if !aes.contains(&edge_id) {
                            return None;
                        }
                    }
                    if let Some(des) = &deny_edge_set {
                        if des.contains(&edge_id) {
                            return None;
                        }
                    }
                    if let Some(ans) = &allow_nodes_set {
                        if !ans.contains(&src_name) || !ans.contains(&dst_name) {
                            return None;
                        }
                    }
                    if let Some(dns) = &deny_nodes_set {
                        if dns.contains(&src_name) || dns.contains(&dst_name) {
                            return None;
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
                    .filter_map(|(node_name, node_type)| {
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
            self.get_name(),
            false,
            true,
            verbose,
            false,
            false,
            false,
        )
    }

    /// Drop all the components that are not connected to interesting
    /// nodes and edges.
    ///
    /// # Arguments
    /// * `node_names` : Option<Vec<String>> - The name of the nodes of which components to keep.
    /// * `node_types` : Option<Vec<String>> - The types of the nodes of which components to keep.
    /// * `edge_types` : Option<Vec<String>> - The types of the edges of which components to keep.
    /// * `minimum_component_size`: Option<usize> - Optional, Minimum size of the components to keep.
    /// * `top_k_components`: Option<usize> - Optional, number of components to keep by number of nodes.
    /// * `verbose`: bool - Wether to show the loading bar.
    pub fn drop_components(
        &self,
        node_names: Option<Vec<String>>,
        node_types: Option<Vec<String>>,
        edge_types: Option<Vec<String>>,
        minimum_component_size: Option<usize>,
        top_k_components: Option<usize>,
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

            self.get_edges_triples()
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
            Some(tkc) => Some(match components_counts.len() < tkc {
                true => components_counts.last().unwrap().1,
                false => components_counts.get(tkc).unwrap().1,
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
            &format!("Dropping components for the graph {}", &self.name),
            self.get_edges_number() as usize,
        );

        Graph::build_graph(
            self.get_edges_quadruples().progress_with(pb).filter_map(
                |(_, src, dst, edge_type, weight)| match keep_components
                    .contains(components_vector[src as usize])
                    && keep_components.contains(components_vector[dst as usize])
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
