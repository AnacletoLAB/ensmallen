use super::*;

#[automatically_generated_function]
/// Returns whether the given node name respects the Panther Database nodes pattern.
///
/// # Arguments
/// * `node_name`: &str - Node name to check pattern with.
///
/// # Example
/// To validate a node you can use:
/// ```rust
/// # use graph::*;
/// let this_library_node_name = "PANTHER:PTHR10003";
/// let not_this_library_node_name = "PizzaQuattroStagioni";
/// assert!(is_valid_panther_database_node_name(this_library_node_name));
/// assert!(!is_valid_panther_database_node_name(not_this_library_node_name));
/// ```
pub fn is_valid_panther_database_node_name(node_name: &str) -> bool {
    is_valid_node_name_from_seeds(
        node_name,
        Some(&["PANTHER"]),
        Some(17),
        Some(":"),
        Some("PTHR"),
        Some(9),
        Some(5),
    )
    .is_ok()
}

#[automatically_generated_function]
/// Returns URL from given Panther Database node name.
///
/// # Arguments
/// * `node_name`: &str - Node name to check pattern with.
///
/// # Safety
/// This method assumes that the provided node name is a Panther Database node name and
/// may cause a panic if the aforementioned assumption is not true.
///
pub(crate) unsafe fn format_panther_database_url_from_node_name(node_name: &str) -> String {
    format_url_from_node_name(
        "http://www.pantherdb.org/panther/family.do?clsAccession={node_name}",
        node_name,
        Some(":"),
    )
}
