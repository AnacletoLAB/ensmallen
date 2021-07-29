use tags::*;
use crate::general_url_utilities::*;

#[automatically_generated_function]
/// Returns whether the given node name respects the Semantic MEDLINE Database nodes pattern.
///
/// # Arguments
/// * `node_name`: &str - Node name to check pattern with.
///
/// # Example
/// To validate a node you can use:
/// ```rust
/// # use url_utilities::*;
/// let this_library_node_name = "semmeddb:PREVENTS";
/// let not_this_library_node_name = "PizzaQuattroStagioni";
/// assert!(is_valid_semantic_medline_database_node_name(this_library_node_name));
/// assert!(!is_valid_semantic_medline_database_node_name(not_this_library_node_name));
/// ```
pub fn is_valid_semantic_medline_database_node_name(node_name: &str) -> bool {
    is_valid_node_name_from_seeds(
        node_name,
        Some(&["SEMMEDDB"]),
        None,
        Some(":"),
        None,
        None,
        None,
    )
    .is_ok()
}

#[automatically_generated_function]
/// Returns URL from given Semantic MEDLINE Database node name.
///
/// # Arguments
/// * `node_name`: &str - Node name to check pattern with.
///
/// # Safety
/// This method assumes that the provided node name is a Semantic MEDLINE Database node name and
/// may cause a panic if the aforementioned assumption is not true.
///
pub(crate) unsafe fn format_semantic_medline_database_url_from_node_name(
    node_name: &str,
) -> String {
    format_url_from_node_name(
        "https://skr3.nlm.nih.gov/SemMedDB{node_name}",
        node_name,
        Some(":"),
    )
}
