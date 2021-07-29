use tags::*;
use crate::general_url_utilities::*;

#[automatically_generated_function]
/// Returns whether the given node name respects the Complex Portal nodes pattern.
///
/// # Arguments
/// * `node_name`: &str - Node name to check pattern with.
///
/// # Example
/// To validate a node you can use:
/// ```rust
/// # use url_utilities::*;
/// let this_library_node_name = "ComplexPortal:CPX-5682";
/// let not_this_library_node_name = "PizzaQuattroStagioni";
/// assert!(is_valid_complex_portal_node_name(this_library_node_name));
/// assert!(!is_valid_complex_portal_node_name(not_this_library_node_name));
/// ```
pub fn is_valid_complex_portal_node_name(node_name: &str) -> bool {
    is_valid_node_name_from_seeds(
        node_name,
        Some(&["COMPLEXPORTAL"]),
        Some(22),
        Some(":"),
        Some("CPX-"),
        Some(8),
        Some(4),
    )
    .is_ok()
}

#[automatically_generated_function]
/// Returns URL from given Complex Portal node name.
///
/// # Arguments
/// * `node_name`: &str - Node name to check pattern with.
///
/// # Safety
/// This method assumes that the provided node name is a Complex Portal node name and
/// may cause a panic if the aforementioned assumption is not true.
///
pub(crate) unsafe fn format_complex_portal_url_from_node_name(node_name: &str) -> String {
    format_url_from_node_name(
        "http://identifiers.org/complexportal/{node_name}",
        node_name,
        Some(":"),
    )
}
