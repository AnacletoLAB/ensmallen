use crate::constructors::build_graph_from_strings_without_type_iterators;

use super::*;
use indicatif::ParallelProgressIterator;
use itertools::Itertools;
use log::warn;
use rayon::iter::ParallelIterator;
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
    /// * `verbose`: Option<bool> - Whether to show a loading bar.
    ///
    /// # Raises
    /// * If the given node names mapping would lead to nodes duplication.
    pub fn replace(
        &self,
        node_name_mapping: Option<HashMap<String, String>>,
        node_type_name_mapping: Option<HashMap<String, String>>,
        node_type_names_mapping: Option<HashMap<Option<Vec<String>>, Option<Vec<String>>>>,
        edge_type_name_mapping: Option<HashMap<Option<String>, Option<String>>>,
        verbose: Option<bool>,
    ) -> Result<Graph> {
        let verbose = verbose.unwrap_or(false);
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
                if self.has_node_name(new_node_name) {
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
            "Replacing attributes in edge list",
            self.get_directed_edges_number() as usize,
        );

        let pb_nodes = get_loading_bar(
            verbose,
            "Replacing attributes in node list",
            self.get_nodes_number() as usize,
        );

        // TODO! this method may be rewritten more efficiently
        // by also using the node and edge type iterators.
        build_graph_from_strings_without_type_iterators(
            self.has_node_types(),
            Some(
                self.par_iter_node_names_and_node_type_names()
                    .progress_with(pb_nodes)
                    .map(|(node_id, node_name, _, node_types)| {
                        Ok((
                            node_id as usize,
                            (
                                node_name_mapping
                                    .as_ref()
                                    .map_or(&node_name, |nns| {
                                        nns.get(&node_name).unwrap_or(&node_name)
                                    })
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
                            ),
                        ))
                    }),
            ),
            Some(self.get_nodes_number()),
            true,
            false,
            false,
            None,
            self.has_edge_types(),
            Some(
                self.par_iter_directed_edge_node_names_and_edge_type_name_and_edge_weight()
                    .progress_with(pb_edges)
                    .map(
                        |(edge_id, _, src_name, _, dst_name, _, edge_type_name, weight)| {
                            Ok((
                                edge_id as usize,
                                (
                                    node_name_mapping
                                        .as_ref()
                                        .map_or(&src_name, |nns| {
                                            nns.get(&src_name).unwrap_or(&src_name)
                                        })
                                        .clone(),
                                    node_name_mapping
                                        .as_ref()
                                        .map_or(&dst_name, |nns| {
                                            nns.get(&dst_name).unwrap_or(&dst_name)
                                        })
                                        .clone(),
                                    edge_type_name_mapping
                                        .as_ref()
                                        .map_or(&edge_type_name, |etns| {
                                            etns.get(&edge_type_name).unwrap_or(&edge_type_name)
                                        })
                                        .clone(),
                                    weight.unwrap_or(WeightT::NAN),
                                ),
                            ))
                        },
                    ),
            ),
            self.has_edge_weights(),
            self.is_directed(),
            Some(true),
            Some(true),
            Some(false),
            Some(false),
            Some(self.get_directed_edges_number()),
            None,
            None,
            self.get_name(),
        )
    }

    /// Replace unknown node types with given node type.
    ///
    /// # Arguments
    /// * `node_type_names`: Vec<String> - The node types to replace the unknown with.
    /// * `verbose`: Option<bool> - Whether to show a loading bar.
    pub fn replace_unknown_node_types_with_node_type_name(
        &self,
        node_type_names: Vec<String>,
        verbose: Option<bool>,
    ) -> Result<Graph> {
        if node_type_names
            .iter()
            .any(|node_type_name| node_type_name.is_empty())
        {
            return Err("One or more of the given node types are empty!".to_string());
        }
        self.replace(
            None,
            None,
            Some([(None, Some(node_type_names))].iter().cloned().collect()),
            None,
            verbose,
        )
    }

    /// Replace unknown edge types with given edge type name.
    ///
    /// # Arguments
    /// * `edge_type_name`: String - The edge type name to replace the unknown with.
    /// * `verbose`: Option<bool> - Whether to show a loading bar.
    pub fn replace_unknown_edge_types_with_edge_type_name(
        &self,
        edge_type_name: String,
        verbose: Option<bool>,
    ) -> Result<Graph> {
        if edge_type_name.is_empty() {
            return Err("The given edge type is empty!".to_string());
        }
        self.replace(
            None,
            None,
            None,
            Some([(None, Some(edge_type_name))].iter().cloned().collect()),
            verbose,
        )
    }
}
