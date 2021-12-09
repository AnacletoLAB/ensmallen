use super::*;

/// Returns whether the given node name respects the drugcentral nodes pattern.
///
/// # Arguments
/// * `node_name`: &str - Node name to check pattern with.
///
/// # Example
/// To validate a node you can use:
/// ```ignore
/// # use graph::*;
/// let drugcentral_node_name = "DrugCentral:217";
/// let not_drugcentral_node_name = "PizzaQuattroStagioni";
/// assert!(is_valid_drugcentral_node_name(drugcentral_node_name));
/// assert!(!is_valid_drugcentral_node_name(not_drugcentral_node_name));
/// ```
pub fn is_valid_drugcentral_node_name(node_name: &str) -> bool {
    is_valid_node_name_from_seeds(
        node_name,
        Some(&["DrugCentral"]),
        None,
        Some(":"),
        None,
        None,
        None,
    )
    .is_ok()
}

/// Returns URL from given DrugCentral node name.
///
/// # Arguments
/// * `node_name`: &str - Node name to check pattern with.
///
/// # Safety
/// This method assumes that the provided node name is a DrugCentral node name and
/// may cause a panic if the aforementioned assumption is not true.
pub(crate) unsafe fn format_drugcentral_url_from_node_name(node_name: &str) -> String {
    format_url_from_node_name(
        "https://drugcentral.org/drugcard/{node_name}",
        node_name,
        Some(":"),
    )
}
