
use super::*;

#[automatically_generated_function]
/// Returns whether the given node name respects the Open Biomedical Association nodes pattern.
///
/// # Arguments
/// * `node_name`: &str - Node name to check pattern with.
///
/// # Example
/// To validate a node you can use:
/// ```rust
/// # use graph::*;
/// let this_library_node_name = "OBAN:date_association_created";
/// let not_this_library_node_name = "PizzaQuattroStagioni";
/// assert!(is_valid_open_biomedical_association_node_name(this_library_node_name));
/// assert!(!is_valid_open_biomedical_association_node_name(not_this_library_node_name));
/// ```
pub fn is_valid_open_biomedical_association_node_name(node_name: &str) -> bool {
    is_valid_node_name_from_seeds(
        node_name,
        Some(&["OBAN"]),
        None,
        Some(":"),
        None,
        None,
        None
    )
    .is_ok()
}


#[automatically_generated_function]
/// Returns URL from given Open Biomedical Association node name.
///
/// # Arguments
/// * `node_name`: &str - Node name to check pattern with.
///
/// # Safety
/// This method assumes that the provided node name is a Open Biomedical Association node name and
/// may cause a panic if the aforementioned assumption is not true.
///
pub(crate) unsafe fn format_open_biomedical_association_url_from_node_name(node_name: &str) -> String {
    format_url_from_node_name(
        "http://purl.org/oban/{node_name}",
        node_name,
        Some(":"),
    )
}

