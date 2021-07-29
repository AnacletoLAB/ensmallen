use crate::general_url_utilities::*;

/// Returns whether the given node name respects the BioLink nodes pattern.
///
/// # Arguments
/// * `node_name`: &str - Node name to check pattern with.
///
/// # Example
/// To validate a node you can use:
/// ```rust
/// # use url_utilities::*;
/// let biolink_node_name1 = "biolink:BehavioralOutcome";
/// let biolink_node_name2 = "biolink:Book";
/// let not_biolink_node_name = "PizzaQuattroStagioni";
/// assert!(is_valid_biolink_from_object(biolink_node_name1));
/// assert!(is_valid_biolink_from_object(biolink_node_name2));
/// assert!(!is_valid_biolink_from_object(not_biolink_node_name));
/// ```
pub fn is_valid_biolink_from_object(node_name: &str) -> bool {
    is_valid_node_name_from_seeds(
        node_name,
        Some(&["biolink"]),
        None,
        Some(":"),
        None,
        None,
        None,
    )
    .is_ok()
}

/// Returns URL from given biolink node name.
///
/// # Arguments
/// * `object_name`: &str - Object name to build pattern for.
///
/// # Safety
/// This method assumes that the provided node name is a JAX node name and
/// may cause a panic if the aforementioned assumption is not true.
pub(crate) unsafe fn format_biolink_from_object(object_name: &str) -> String {
    format_url_from_node_name(
        "https://biolink.github.io/biolink-model/docs/{node_name}.html",
        object_name,
        Some(":"),
    )
}
