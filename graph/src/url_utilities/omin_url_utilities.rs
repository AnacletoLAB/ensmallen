use super::*;

/// Returns whether the given node name respects the omim nodes pattern.
///
/// # Arguments
/// * `node_name`: &str - Node name to check pattern with.
///
/// # Example
/// To validate a node you can use:
/// ```rust
/// # use graph::*;
/// let omim_node_name = "OMIM:611636";
/// let not_omim_node_name = "PizzaQuattroStagioni";
/// assert!(is_valid_omim_node_name(omim_node_name));
/// assert!(!is_valid_omim_node_name(not_omim_node_name));
/// ```
pub fn is_valid_omim_node_name(node_name: &str) -> bool {
    is_valid_node_name_from_seeds(
        node_name,
        Some(&["OMIM"]),
        Some(11),
        Some(":"),
        None,
        None,
        Some(6),
    )
    .is_ok()
}

/// Returns URL from given omim node name.
///
/// # Arguments
/// * `node_name`: &str - Node name to check pattern with.
///
/// # Safety
/// This method assumes that the provided node name is a omim node name and
/// may cause a panic if the aforementioned assumption is not true.
pub(crate) unsafe fn format_omim_url_from_node_name(node_name: &str) -> String {
    format_url_from_node_name(
        "https://www.omim.org/entry/{node_name}",
        node_name,
        Some(":"),
    )
}
