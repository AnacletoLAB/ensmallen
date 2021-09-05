use super::*;

#[automatically_generated_function]
/// Returns whether the given node name respects the Ontology for General Medical Science nodes pattern.
///
/// # Arguments
/// * `node_name`: &str - Node name to check pattern with.
///
/// # Example
/// To validate a node you can use:
/// ```rust
/// # use graph::*;
/// let this_library_node_name = "OGMS:0000031";
/// let not_this_library_node_name = "PizzaQuattroStagioni";
/// assert!(is_valid_ontology_for_general_medical_science_node_name(this_library_node_name));
/// assert!(!is_valid_ontology_for_general_medical_science_node_name(not_this_library_node_name));
/// ```
pub fn is_valid_ontology_for_general_medical_science_node_name(node_name: &str) -> bool {
    is_valid_node_name_from_seeds(
        node_name,
        Some(&["OGMS"]),
        Some(12),
        Some(":"),
        None,
        Some(7),
        Some(7),
    )
    .is_ok()
}

#[automatically_generated_function]
/// Returns URL from given Ontology for General Medical Science node name.
///
/// # Arguments
/// * `node_name`: &str - Node name to check pattern with.
///
/// # Safety
/// This method assumes that the provided node name is a Ontology for General Medical Science node name and
/// may cause a panic if the aforementioned assumption is not true.
///
pub(crate) unsafe fn format_ontology_for_general_medical_science_url_from_node_name(
    node_name: &str,
) -> String {
    format_url_from_node_name(
        "http://purl.obolibrary.org/obo/OGMS_{node_name}",
        node_name,
        Some(":"),
    )
}
