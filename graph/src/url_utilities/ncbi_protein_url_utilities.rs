use super::*;

#[automatically_generated_function]
/// Returns whether the given node name respects the NCBI Protein nodes pattern.
///
/// # Arguments
/// * `node_name`: &str - Node name to check pattern with.
///
/// # Example
/// To validate a node you can use:
/// ```ignore
/// # use graph::*;
/// let this_library_node_name = "NCBIProtein:AY394850.2";
/// let not_this_library_node_name = "PizzaQuattroStagioni";
/// assert!(is_valid_ncbi_protein_node_name(this_library_node_name));
/// assert!(!is_valid_ncbi_protein_node_name(not_this_library_node_name));
/// ```
pub fn is_valid_ncbi_protein_node_name(node_name: &str) -> bool {
    is_valid_node_name_from_seeds(
        node_name,
        Some(&["NCBIPROTEIN"]),
        Some(22),
        Some(":"),
        None,
        Some(10),
        None,
    )
    .is_ok()
}

#[automatically_generated_function]
/// Returns URL from given NCBI Protein node name.
///
/// # Arguments
/// * `node_name`: &str - Node name to check pattern with.
///
/// # Safety
/// This method assumes that the provided node name is a NCBI Protein node name and
/// may cause a panic if the aforementioned assumption is not true.
///
pub(crate) unsafe fn format_ncbi_protein_url_from_node_name(node_name: &str) -> String {
    format_url_from_node_name(
        "http://identifiers.org/ncbiprotein/{node_name}",
        node_name,
        Some(":"),
    )
}
