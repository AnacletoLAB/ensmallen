use super::*;
use indicatif::ProgressIterator;
use itertools::Itertools;
use log::warn;
use std::collections::HashMap;

/// # Replace.
impl Graph {
    /// Replace given node, node type and edge type names.
    ///
    /// # Arguments
    /// * `node_name_mapping`: Option<HashMap<String, String>> - The node names to replace.
    /// * `node_type_name_mapping`: Option<HashMap<String, String>> - The node type names to replace.
    /// * `node_type_names_mapping`: Option<HashMap<Option<Vec<String>>, Option<Vec<String>>>> - The node type names (as vectors) to replace.
    /// * `edge_type_name_mapping`: Option<HashMap<Option<String>, Option<String>>> - The edge type names to replace.
    /// * `verbose`: bool - Whether to show a loading bar.
    ///
    /// # Raises
    /// * If the given node names mapping would lead to nodes duplication.
    pub fn replace(
        &self,
        node_name_mapping: Option<HashMap<String, String>>,
        node_type_name_mapping: Option<HashMap<String, String>>,
        node_type_names_mapping: Option<HashMap<Option<Vec<String>>, Option<Vec<String>>>>,
        edge_type_name_mapping: Option<HashMap<Option<String>, Option<String>>>,
        verbose: bool,
    ) -> Result<Graph, String> {
        if node_type_names_mapping.is_some() && node_type_name_mapping.is_some() {
            return Err(
                "Using at once node_type_name_mapping and node_type_names_mapping is not supported.".to_string()
            );
        }
        if let Some(nns) = &node_name_mapping {
            for (original_node_name, new_node_name) in nns.iter() {
                if *original_node_name == *new_node_name {
                    warn!(
                        concat!(
                            "The required remapping operation includes remapping between ",
                            "the same node names: {} => {}"
                        ),
                        original_node_name, new_node_name
                    );
                    continue;
                }
                if self.has_node_from_node_name(new_node_name) {
                    return Err(format!(
                        concat!(
                            "One of the new node names ({new_node_name}) already exists in the graph ",
                            "and the required remapping operation ({original_node_name} => {new_node_name}) would lead to ",
                            "a duplicated node name."
                        ),
                        new_node_name=new_node_name,
                        original_node_name=original_node_name
                    ));
                }
            }
        }

        let pb_edges = get_loading_bar(
            verbose,
            format!(
                "Building edges of graph {} replacing required attributes",
                self.name
            )
            .as_ref(),
            self.get_directed_edges_number() as usize,
        );

        let pb_nodes = get_loading_bar(
            verbose,
            format!(
                "Building nodes of graph {} replacing required attributes",
                self.name
            )
            .as_ref(),
            self.get_nodes_number() as usize,
        );

        Graph::from_string_sorted(
            self.iter_edge_node_names_and_edge_type_name_and_edge_weight(true)
                .progress_with(pb_edges)
                .map(|(_, _, src_name, _, dst_name, _, edge_type_name, weight)| {
                    Ok((
                        node_name_mapping
                            .as_ref()
                            .map_or(&src_name, |nns| nns.get(&src_name).unwrap_or(&src_name))
                            .clone(),
                        node_name_mapping
                            .as_ref()
                            .map_or(&dst_name, |nns| nns.get(&dst_name).unwrap_or(&dst_name))
                            .clone(),
                        edge_type_name_mapping
                            .as_ref()
                            .map_or(&edge_type_name, |etns| {
                                etns.get(&edge_type_name).unwrap_or(&edge_type_name)
                            })
                            .clone(),
                        weight,
                    ))
                }),
            Some(
                self.iter_node_names_and_node_type_names()
                    .progress_with(pb_nodes)
                    .map(|(_, node_name, _, node_types)| {
                        Ok((
                            node_name_mapping
                                .as_ref()
                                .map_or(&node_name, |nns| nns.get(&node_name).unwrap_or(&node_name))
                                .clone(),
                            match (
                                &node_type_name_mapping,
                                &node_type_names_mapping,
                                node_types,
                            ) {
                                (Some(ntn_mapping), None, Some(nts)) => Some(
                                    nts.into_iter()
                                        .map(|node_type_name| {
                                            ntn_mapping
                                                .get(&node_type_name)
                                                .map_or(node_type_name, |new_value| {
                                                    new_value.clone()
                                                })
                                        })
                                        .unique()
                                        .collect(),
                                ),
                                (None, Some(ntns_mapping), node_types) => {
                                    ntns_mapping.get(&node_types).unwrap_or(&node_types).clone()
                                }
                                (_, _, node_types) => node_types,
                            },
                        ))
                    }),
            ),
            self.is_directed(),
            true,
            false,
            true,
            false,
            true,
            self.get_directed_edges_number() as usize,
            self.get_nodes_number(),
            // TODO: UPDATE THE FOLLOWING FOUR BOOLEANS
            false,
            false,
            false,
            false,
            self.has_node_types(),
            self.has_edge_types(),
            self.has_edge_weights(),
            self.has_singletons(),
            self.has_singletons_with_selfloops(),
            self.has_trap_nodes(),
            self.get_name(),
        )
    }

    /// Replace unknown node types with given node type.
    ///
    /// # Arguments
    /// * `node_types`: Vec<NodeType> - The node types to replace the unknown with.
    /// * `verbose`: bool - Whether to show a loading bar.
    ///
    /// # Raises
    /// * If the given node names mapping would lead to nodes duplication.
    pub fn replace_unknown_node_types_with_node_type_name(
        &self,
        node_type_names: Vec<String>,
        verbose: bool,
    ) -> Graph {
        self.replace(
            None,
            None,
            Some([(None, Some(node_type_names))].iter().cloned().collect()),
            None,
            verbose,
        )
        .unwrap()
    }
}
