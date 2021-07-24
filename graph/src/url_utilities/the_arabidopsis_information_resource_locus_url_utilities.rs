
use super::*;

#[automatically_generated_function]
/// Returns whether the given node name respects the The Arabidopsis Information Resource Locus nodes pattern.
///
/// # Arguments
/// * `node_name`: &str - Node name to check pattern with.
///
/// # Example
/// To validate a node you can use:
/// ```rust
/// # use graph::*;
/// let this_library_node_name = "TAIR.LOCUS:2174557";
/// let not_this_library_node_name = "PizzaQuattroStagioni";
/// assert!(is_valid_the_arabidopsis_information_resource_locus_node_name(this_library_node_name));
/// assert!(!is_valid_the_arabidopsis_information_resource_locus_node_name(not_this_library_node_name));
/// ```
pub fn is_valid_the_arabidopsis_information_resource_locus_node_name(node_name: &str) -> bool {
    is_valid_node_name_from_seeds(
        node_name,
        Some(&["TAIR.LOCUS"]),
        Some(18),
        Some(":"),
        None,
        Some(7),
        Some(7)
    )
    .is_ok()
}


#[automatically_generated_function]
/// Returns URL from given The Arabidopsis Information Resource Locus node name.
///
/// # Arguments
/// * `node_name`: &str - Node name to check pattern with.
///
/// # Safety
/// This method assumes that the provided node name is a The Arabidopsis Information Resource Locus node name and
/// may cause a panic if the aforementioned assumption is not true.
///
pub(crate) unsafe fn format_the_arabidopsis_information_resource_locus_url_from_node_name(node_name: &str) -> String {
    format_url_from_node_name(
        "http://identifiers.org/tair.locus/{node_name}",
        node_name,
        Some(":"),
    )
}

