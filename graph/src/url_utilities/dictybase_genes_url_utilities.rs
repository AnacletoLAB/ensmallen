use super::*;

#[automatically_generated_function]
/// Returns whether the given node name respects the Dictybase Genes nodes pattern.
///
/// # Arguments
/// * `node_name`: &str - Node name to check pattern with.
///
/// # Example
/// To validate a node you can use:
/// ```ignore
/// # use graph::*;
/// let this_library_node_name = "dictyBase:DDB_G0267374";
/// let not_this_library_node_name = "PizzaQuattroStagioni";
/// assert!(is_valid_dictybase_genes_node_name(this_library_node_name));
/// assert!(!is_valid_dictybase_genes_node_name(not_this_library_node_name));
/// ```
pub fn is_valid_dictybase_genes_node_name(node_name: &str) -> bool {
    is_valid_node_name_from_seeds(
        node_name,
        Some(&["DICTYBASE"]),
        Some(22),
        Some(":"),
        Some("DDB_G"),
        Some(12),
        Some(7),
    )
    .is_ok()
}

#[automatically_generated_function]
/// Returns URL from given Dictybase Genes node name.
///
/// # Arguments
/// * `node_name`: &str - Node name to check pattern with.
///
/// # Safety
/// This method assumes that the provided node name is a Dictybase Genes node name and
/// may cause a panic if the aforementioned assumption is not true.
///
pub(crate) unsafe fn format_dictybase_genes_url_from_node_name(node_name: &str) -> String {
    format_url_from_node_name(
        "http://dictybase.org/gene/{node_name}",
        node_name,
        Some(":"),
    )
}
