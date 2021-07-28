use super::*;

#[automatically_generated_function]
/// Returns whether the given node name respects the Provenance, Authoring and Versioning nodes pattern.
///
/// # Arguments
/// * `node_name`: &str - Node name to check pattern with.
///
/// # Example
/// To validate a node you can use:
/// ```rust
/// # use graph::*;
/// let this_library_node_name = "pav:curatedBy";
/// let not_this_library_node_name = "PizzaQuattroStagioni";
/// assert!(is_valid_provenance_authoring_and_versioning_node_name(this_library_node_name));
/// assert!(!is_valid_provenance_authoring_and_versioning_node_name(not_this_library_node_name));
/// ```
pub fn is_valid_provenance_authoring_and_versioning_node_name(node_name: &str) -> bool {
    is_valid_node_name_from_seeds(node_name, Some(&["PAV"]), None, Some(":"), None, None, None)
        .is_ok()
}

#[automatically_generated_function]
/// Returns URL from given Provenance, Authoring and Versioning node name.
///
/// # Arguments
/// * `node_name`: &str - Node name to check pattern with.
///
/// # Safety
/// This method assumes that the provided node name is a Provenance, Authoring and Versioning node name and
/// may cause a panic if the aforementioned assumption is not true.
///
pub(crate) unsafe fn format_provenance_authoring_and_versioning_url_from_node_name(
    node_name: &str,
) -> String {
    format_url_from_node_name("http://purl.org/pav/{node_name}", node_name, Some(":"))
}
