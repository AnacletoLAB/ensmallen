use super::*;

#[automatically_generated_function]
/// Returns whether the given node name respects the Unified Medical Language System nodes pattern.
///
/// # Arguments
/// * `node_name`: &str - Node name to check pattern with.
///
/// # Example
/// To validate a node you can use:
/// ```ignore
/// # use graph::*;
/// let this_library_node_name = "UMLS:C1843632";
/// let not_this_library_node_name = "PizzaQuattroStagioni";
/// assert!(is_valid_unified_medical_language_system_node_name(this_library_node_name));
/// assert!(!is_valid_unified_medical_language_system_node_name(not_this_library_node_name));
/// ```
pub fn is_valid_unified_medical_language_system_node_name(node_name: &str) -> bool {
    is_valid_node_name_from_seeds(
        node_name,
        Some(&["UMLS"]),
        Some(13),
        Some(":"),
        Some("C"),
        Some(8),
        None,
    )
    .is_ok()
}

#[automatically_generated_function]
/// Returns URL from given Unified Medical Language System node name.
///
/// # Arguments
/// * `node_name`: &str - Node name to check pattern with.
///
/// # Safety
/// This method assumes that the provided node name is a Unified Medical Language System node name and
/// may cause a panic if the aforementioned assumption is not true.
///
pub(crate) unsafe fn format_unified_medical_language_system_url_from_node_name(
    node_name: &str,
) -> String {
    format_url_from_node_name(
        "http://linkedlifedata.com/resource/umls/id/{node_name}",
        node_name,
        Some(":"),
    )
}
