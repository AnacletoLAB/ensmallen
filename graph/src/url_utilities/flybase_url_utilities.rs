use super::*;

/// Returns whether the given node name respects the flybase nodes pattern.
///
/// # Arguments
/// * `node_name`: &str - Node name to check pattern with.
///
/// # Example
/// To validate a node you can use:
/// ```rust
/// # use graph::*;
/// let flybase_node_name = "FlyBase:FBgn0000003";
/// let not_flybase_node_name = "PizzaQuattroStagioni";
/// assert!(is_valid_flybase_node_name(flybase_node_name));
/// assert!(!is_valid_flybase_node_name(not_flybase_node_name));
/// ```
pub fn is_valid_flybase_node_name(node_name: &str) -> bool {
    is_valid_node_name_from_seeds(
        node_name,
        Some("FlyBase"),
        Some(19),
        Some(":"),
        Some("FB"),
        Some(11),
        Some(7),
    )
    .is_ok()
}

/// Returns URL from given flybase node name.
///
/// # Arguments
/// * `node_name`: &str - Node name to check pattern with.
///
/// # Safety
/// This method assumes that the provided node name is a flybase node name and
/// may cause a panic if the aforementioned assumption is not true.
pub(crate) unsafe fn format_flybase_url_from_node_name(node_name: &str) -> String {
    format_url_from_node_name(
        "https://flybase.org/reports/{node_name}",
        node_name,
        Some(":"),
    )
}
