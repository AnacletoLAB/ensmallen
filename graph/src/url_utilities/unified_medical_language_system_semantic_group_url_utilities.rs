use super::*;

#[automatically_generated_function]
/// Returns whether the given node name respects the Unified Medical Language System Semantic Group nodes pattern.
///
/// # Arguments
/// * `node_name`: &str - Node name to check pattern with.
///
/// # Example
/// To validate a node you can use:
/// ```rust
/// # use graph::*;
/// let this_library_node_name = "umlssg:ANAT";
/// let not_this_library_node_name = "PizzaQuattroStagioni";
/// assert!(is_valid_unified_medical_language_system_semantic_group_node_name(this_library_node_name));
/// assert!(!is_valid_unified_medical_language_system_semantic_group_node_name(not_this_library_node_name));
/// ```
pub fn is_valid_unified_medical_language_system_semantic_group_node_name(node_name: &str) -> bool {
    is_valid_node_name_from_seeds(
        node_name,
        Some(&["UMLSSG"]),
        Some(11),
        Some(":"),
        None,
        Some(4),
        None,
    )
    .is_ok()
}

#[automatically_generated_function]
/// Returns URL from given Unified Medical Language System Semantic Group node name.
///
/// # Arguments
/// * `node_name`: &str - Node name to check pattern with.
///
/// # Safety
/// This method assumes that the provided node name is a Unified Medical Language System Semantic Group node name and
/// may cause a panic if the aforementioned assumption is not true.
///
pub(crate) unsafe fn format_unified_medical_language_system_semantic_group_url_from_node_name(
    node_name: &str,
) -> String {
    format_url_from_node_name(
        "https://metamap.nlm.nih.gov/Docs/SemGroups_2018.txt/group#{node_name}",
        node_name,
        Some(":"),
    )
}
