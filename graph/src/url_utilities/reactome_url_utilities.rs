use super::*;

/// Returns whether the given node name respects the Reactome nodes pattern.
///
/// # Arguments
/// * `node_name`: &str - Node name to check pattern with.
///
/// # Example
/// To validate a node you can use:
/// ```rust
/// # use graph::*;
/// let reactome_node_name1 = "REACT:R-HSA-8851222";
/// let reactome_node_name2 = "REACT:R-HSA-77267";
/// let not_reactome_node_name = "PizzaQuattroStagioni";
/// assert!(is_valid_reactome_node_name(reactome_node_name1));
/// assert!(is_valid_reactome_node_name(reactome_node_name2));
/// assert!(!is_valid_reactome_node_name(not_reactome_node_name));
/// ```
pub fn is_valid_reactome_node_name(node_name: &str) -> bool {
    is_valid_node_name_from_seeds(
        node_name,
        Some(&["REACT"]),
        None,
        Some(":"),
        None,
        None,
        None,
    )
    .is_ok()
}

/// Returns URL from given Reactome node name.
///
/// # Arguments
/// * `node_name`: &str - Node name to check pattern with.
///
/// # Safety
/// This method assumes that the provided node name is a Reactome node name and
/// may cause a panic if the aforementioned assumption is not true.
pub(crate) unsafe fn format_reactome_url_from_node_name(node_name: &str) -> String {
    format_url_from_node_name(
        "https://reactome.org/content/detail/{node_name}",
        node_name,
        Some(":"),
    )
}
