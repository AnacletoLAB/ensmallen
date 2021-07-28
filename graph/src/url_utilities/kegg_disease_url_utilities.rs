use super::*;

#[automatically_generated_function]
/// Returns whether the given node name respects the KEGG Disease nodes pattern.
///
/// # Arguments
/// * `node_name`: &str - Node name to check pattern with.
///
/// # Example
/// To validate a node you can use:
/// ```rust
/// # use graph::*;
/// let this_library_node_name = "KEGG-ds:H00001";
/// let not_this_library_node_name = "PizzaQuattroStagioni";
/// assert!(is_valid_kegg_disease_node_name(this_library_node_name));
/// assert!(!is_valid_kegg_disease_node_name(not_this_library_node_name));
/// ```
pub fn is_valid_kegg_disease_node_name(node_name: &str) -> bool {
    is_valid_node_name_from_seeds(
        node_name,
        Some(&["KEGG-DS"]),
        Some(14),
        Some(":"),
        Some("H"),
        Some(6),
        Some(5),
    )
    .is_ok()
}

#[automatically_generated_function]
/// Returns URL from given KEGG Disease node name.
///
/// # Arguments
/// * `node_name`: &str - Node name to check pattern with.
///
/// # Safety
/// This method assumes that the provided node name is a KEGG Disease node name and
/// may cause a panic if the aforementioned assumption is not true.
///
pub(crate) unsafe fn format_kegg_disease_url_from_node_name(node_name: &str) -> String {
    format_url_from_node_name(
        "http://purl.obolibrary.org/KEGG-ds_{node_name}",
        node_name,
        Some(":"),
    )
}
