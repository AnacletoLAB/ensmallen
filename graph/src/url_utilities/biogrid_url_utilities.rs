use super::*;
/// Returns whether the given node name respects the biogrid nodes pattern.
///
/// # Arguments
/// * `node_name`: &str - Node name to check pattern with.
///
/// # Example
/// To validate a node you can use:
/// ```rust
/// # use graph::*;
/// let biogrid_node_name = "BIOGRID:106534";
/// let not_biogrid_node_name = "PizzaQuattroStagioni";
/// assert!(is_valid_biogrid_node_name(biogrid_node_name));
/// assert!(!is_valid_biogrid_node_name(not_biogrid_node_name));
/// ```
pub fn is_valid_biogrid_node_name(node_name: &str) -> bool {
    is_valid_node_name_from_seeds(
        node_name,
        Some(&["BIOGRID"]),
        None,
        Some(":"),
        None,
        None,
        None,
    )
    .is_ok()
}

/// Returns URL from given biogrid node name.
///
/// # Arguments
/// * `node_name`: &str - Node name to check pattern with.
///
/// # Safety
/// This method assumes that the provided node name is a biogrid node name and
/// may cause a panic if the aforementioned assumption is not true.
pub(crate) unsafe fn format_biogrid_url_from_node_name(node_name: &str) -> String {
    format_url_from_node_name("https://thebiogrid.org/{node_name}", node_name, Some(":"))
}
