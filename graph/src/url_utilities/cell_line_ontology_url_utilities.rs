use super::*;

#[automatically_generated_function]
/// Returns whether the given node name respects the Cell Line Ontology nodes pattern.
///
/// # Arguments
/// * `node_name`: &str - Node name to check pattern with.
///
/// # Example
/// To validate a node you can use:
/// ```rust
/// # use graph::*;
/// let this_library_node_name = "CLO:0010710";
/// let not_this_library_node_name = "PizzaQuattroStagioni";
/// assert!(is_valid_cell_line_ontology_node_name(this_library_node_name));
/// assert!(!is_valid_cell_line_ontology_node_name(not_this_library_node_name));
/// ```
pub fn is_valid_cell_line_ontology_node_name(node_name: &str) -> bool {
    is_valid_node_name_from_seeds(
        node_name,
        Some(&["CLO"]),
        Some(11),
        Some(":"),
        None,
        Some(7),
        Some(7),
    )
    .is_ok()
}

#[automatically_generated_function]
/// Returns URL from given Cell Line Ontology node name.
///
/// # Arguments
/// * `node_name`: &str - Node name to check pattern with.
///
/// # Safety
/// This method assumes that the provided node name is a Cell Line Ontology node name and
/// may cause a panic if the aforementioned assumption is not true.
///
pub(crate) unsafe fn format_cell_line_ontology_url_from_node_name(node_name: &str) -> String {
    format_url_from_node_name(
        "http://purl.obolibrary.org/obo/CLO_{node_name}",
        node_name,
        Some(":"),
    )
}
