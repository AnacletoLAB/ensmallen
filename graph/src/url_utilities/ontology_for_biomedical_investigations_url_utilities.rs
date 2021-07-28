use super::*;

#[automatically_generated_function]
/// Returns whether the given node name respects the Ontology for Biomedical Investigations nodes pattern.
///
/// # Arguments
/// * `node_name`: &str - Node name to check pattern with.
///
/// # Example
/// To validate a node you can use:
/// ```rust
/// # use graph::*;
/// let this_library_node_name = "OBI:0302732";
/// let not_this_library_node_name = "PizzaQuattroStagioni";
/// assert!(is_valid_ontology_for_biomedical_investigations_node_name(this_library_node_name));
/// assert!(!is_valid_ontology_for_biomedical_investigations_node_name(not_this_library_node_name));
/// ```
pub fn is_valid_ontology_for_biomedical_investigations_node_name(node_name: &str) -> bool {
    is_valid_node_name_from_seeds(node_name, Some(&["OBI"]), None, Some(":"), None, None, None)
        .is_ok()
}

#[automatically_generated_function]
/// Returns URL from given Ontology for Biomedical Investigations node name.
///
/// # Arguments
/// * `node_name`: &str - Node name to check pattern with.
///
/// # Safety
/// This method assumes that the provided node name is a Ontology for Biomedical Investigations node name and
/// may cause a panic if the aforementioned assumption is not true.
///
pub(crate) unsafe fn format_ontology_for_biomedical_investigations_url_from_node_name(
    node_name: &str,
) -> String {
    format_url_from_node_name(
        "http://purl.obolibrary.org/obo/OBI_{node_name}",
        node_name,
        Some(":"),
    )
}
