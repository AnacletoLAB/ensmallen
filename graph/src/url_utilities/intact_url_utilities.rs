use super::*;

#[automatically_generated_function]
/// Returns whether the given node name respects the INTACT nodes pattern.
///
/// # Arguments
/// * `node_name`: &str - Node name to check pattern with.
///
/// # Example
/// To validate a node you can use:
/// ```rust
/// # use graph::*;
/// let this_library_node_name1 = "INTACT:EBI-25567068";
/// let this_library_node_name2 = "INTACT:EBI-986894";
/// let not_this_library_node_name = "PizzaQuattroStagioni";
/// assert!(is_valid_intact_node_name(this_library_node_name1));
/// assert!(is_valid_intact_node_name(this_library_node_name2));
/// assert!(!is_valid_intact_node_name(not_this_library_node_name));
/// ```
pub fn is_valid_intact_node_name(node_name: &str) -> bool {
    is_valid_node_name_from_seeds(
        node_name,
        Some(&["INTACT"]),
        None,
        Some(":"),
        None,
        None,
        None,
    )
    .is_ok()
}

#[automatically_generated_function]
/// Returns URL from given INTACT node name.
///
/// # Arguments
/// * `node_name`: &str - Node name to check pattern with.
///
/// # Safety
/// This method assumes that the provided node name is a INTACT node name and
/// may cause a panic if the aforementioned assumption is not true.
///
pub(crate) unsafe fn format_intact_url_from_node_name(node_name: &str) -> String {
    format_url_from_node_name(
        "http://www.ebi.ac.uk/intact/search/do/search?searchString={node_name}",
        node_name,
        Some(":"),
    )
}
