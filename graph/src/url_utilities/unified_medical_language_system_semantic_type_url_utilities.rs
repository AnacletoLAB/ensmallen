use super::*;

#[automatically_generated_function]
/// Returns whether the given node name respects the Unified Medical Language System Semantic Type nodes pattern.
///
/// # Arguments
/// * `node_name`: &str - Node name to check pattern with.
///
/// # Example
/// To validate a node you can use:
/// ```ignore
/// # use graph::*;
/// let this_library_node_name = "umlsst:ffas";
/// let not_this_library_node_name = "PizzaQuattroStagioni";
/// assert!(is_valid_unified_medical_language_system_semantic_type_node_name(this_library_node_name));
/// assert!(!is_valid_unified_medical_language_system_semantic_type_node_name(not_this_library_node_name));
/// ```
pub fn is_valid_unified_medical_language_system_semantic_type_node_name(node_name: &str) -> bool {
    is_valid_node_name_from_seeds(
        node_name,
        Some(&["UMLSST"]),
        Some(11),
        Some(":"),
        None,
        Some(4),
        None,
    )
    .is_ok()
}

#[automatically_generated_function]
/// Returns URL from given Unified Medical Language System Semantic Type node name.
///
/// # Arguments
/// * `node_name`: &str - Node name to check pattern with.
///
/// # Safety
/// This method assumes that the provided node name is a Unified Medical Language System Semantic Type node name and
/// may cause a panic if the aforementioned assumption is not true.
///
pub(crate) unsafe fn format_unified_medical_language_system_semantic_type_url_from_node_name(
    node_name: &str,
) -> String {
    format_url_from_node_name(
        "https://metamap.nlm.nih.gov/Docs/SemanticTypes_2018AB.txt/type#{node_name}",
        node_name,
        Some(":"),
    )
}
