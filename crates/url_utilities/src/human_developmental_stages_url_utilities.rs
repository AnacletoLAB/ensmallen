use tags::*;
use crate::general_url_utilities::*;

#[automatically_generated_function]
/// Returns whether the given node name respects the Human Developmental Stages nodes pattern.
///
/// # Arguments
/// * `node_name`: &str - Node name to check pattern with.
///
/// # Example
/// To validate a node you can use:
/// ```rust
/// # use url_utilities::*;
/// let this_library_node_name = "HsapDv:0000204";
/// let not_this_library_node_name = "PizzaQuattroStagioni";
/// assert!(is_valid_human_developmental_stages_node_name(this_library_node_name));
/// assert!(!is_valid_human_developmental_stages_node_name(not_this_library_node_name));
/// ```
pub fn is_valid_human_developmental_stages_node_name(node_name: &str) -> bool {
    is_valid_node_name_from_seeds(
        node_name,
        Some(&["HSAPDV"]),
        Some(14),
        Some(":"),
        None,
        Some(7),
        Some(7),
    )
    .is_ok()
}

#[automatically_generated_function]
/// Returns URL from given Human Developmental Stages node name.
///
/// # Arguments
/// * `node_name`: &str - Node name to check pattern with.
///
/// # Safety
/// This method assumes that the provided node name is a Human Developmental Stages node name and
/// may cause a panic if the aforementioned assumption is not true.
///
pub(crate) unsafe fn format_human_developmental_stages_url_from_node_name(
    node_name: &str,
) -> String {
    format_url_from_node_name(
        "http://purl.obolibrary.org/obo/HSAPDV_{node_name}",
        node_name,
        Some(":"),
    )
}
