use super::*;

#[automatically_generated_function]
/// Returns whether the given node name respects the RDF nodes pattern.
///
/// # Arguments
/// * `node_name`: &str - Node name to check pattern with.
///
/// # Example
/// To validate a node you can use:
/// ```ignore
/// # use graph::*;
/// let this_library_node_name = "rdf:predicate";
/// let not_this_library_node_name = "PizzaQuattroStagioni";
/// assert!(is_valid_rdf_node_name(this_library_node_name));
/// assert!(!is_valid_rdf_node_name(not_this_library_node_name));
/// ```
pub fn is_valid_rdf_node_name(node_name: &str) -> bool {
    is_valid_node_name_from_seeds(node_name, Some(&["RDF"]), None, Some(":"), None, None, None)
        .is_ok()
}

#[automatically_generated_function]
/// Returns URL from given RDF node name.
///
/// # Arguments
/// * `node_name`: &str - Node name to check pattern with.
///
/// # Safety
/// This method assumes that the provided node name is a RDF node name and
/// may cause a panic if the aforementioned assumption is not true.
///
pub(crate) unsafe fn format_rdf_url_from_node_name(node_name: &str) -> String {
    format_url_from_node_name(
        "http://www.w3.org/1999/02/22-rdf-syntax-ns#{node_name}",
        node_name,
        Some(":"),
    )
}
