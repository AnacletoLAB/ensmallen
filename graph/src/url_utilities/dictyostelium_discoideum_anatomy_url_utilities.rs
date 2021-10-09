use super::*;

#[automatically_generated_function]
/// Returns whether the given node name respects the Dictyostelium discoideum anatomy nodes pattern.
///
/// # Arguments
/// * `node_name`: &str - Node name to check pattern with.
///
/// # Example
/// To validate a node you can use:
/// ```ignore
/// # use graph::*;
/// let this_library_node_name = "DDANAT:0010085";
/// let not_this_library_node_name = "PizzaQuattroStagioni";
/// assert!(is_valid_dictyostelium_discoideum_anatomy_node_name(this_library_node_name));
/// assert!(!is_valid_dictyostelium_discoideum_anatomy_node_name(not_this_library_node_name));
/// ```
pub fn is_valid_dictyostelium_discoideum_anatomy_node_name(node_name: &str) -> bool {
    is_valid_node_name_from_seeds(
        node_name,
        Some(&["DDANAT"]),
        Some(14),
        Some(":"),
        None,
        Some(7),
        Some(7),
    )
    .is_ok()
}

#[automatically_generated_function]
/// Returns URL from given Dictyostelium discoideum anatomy node name.
///
/// # Arguments
/// * `node_name`: &str - Node name to check pattern with.
///
/// # Safety
/// This method assumes that the provided node name is a Dictyostelium discoideum anatomy node name and
/// may cause a panic if the aforementioned assumption is not true.
///
pub(crate) unsafe fn format_dictyostelium_discoideum_anatomy_url_from_node_name(
    node_name: &str,
) -> String {
    format_url_from_node_name(
        "http://purl.obolibrary.org/obo/DDANAT_{node_name}",
        node_name,
        Some(":"),
    )
}
