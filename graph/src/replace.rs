use super::*;
use log::warn;
use std::collections::HashMap;

/// # Replace.
impl Graph {
    /// Replace given node, node type and edge type names.
    ///
    /// # Arguments
    /// * `node_name_mapping`: Option<HashMap<String, String>> - The node names to replace.
    /// * `node_type_name_mapping`: Option<HashMap<String, String>> - The node type names to replace.
    /// * `edge_type_name_mapping`: Option<HashMap<String, String>> - The edge type names to replace.
    ///
    /// # Raises
    /// * If the given node names mapping would lead to nodes duplication.
    ///
    pub fn replace(
        &self,
        node_name_mapping: Option<HashMap<String, String>>,
        node_type_name_mapping: Option<HashMap<String, String>>,
        edge_type_name_mapping: Option<HashMap<String, String>>,
    ) -> Result<Graph> {
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

        let mut new_graph = self.clone();
        if let Some(node_name_mapping) = node_name_mapping {
            for (key, value) in node_name_mapping.into_iter() {
                Arc::make_mut(&mut new_graph.nodes).replace_inplace(key, value)?;
            }
        }
        if let (Some(node_types), Some(node_type_name_mapping)) =
            (Arc::make_mut(&mut new_graph.node_types), node_type_name_mapping)
        {
            for (key, value) in node_type_name_mapping.into_iter() {
                node_types.vocabulary.replace_inplace(key, value)?;
            }
        }
        if let (Some(edge_types), Some(edge_type_name_mapping)) =
            (Arc::make_mut(&mut new_graph.edge_types), edge_type_name_mapping)
        {
            for (key, value) in edge_type_name_mapping.into_iter() {
                edge_types.vocabulary.replace_inplace(key, value)?;
            }
        }

        Ok(new_graph)
    }
}
