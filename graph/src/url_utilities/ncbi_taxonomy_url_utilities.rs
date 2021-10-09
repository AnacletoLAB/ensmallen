use super::*;

/// Returns whether the given node name respects the NCBI taxonomy nodes pattern.
///
/// # Arguments
/// * `node_name`: &str - Node name to check pattern with.
///
/// # Example
/// To validate a node you can use:
/// ```ignore
/// # use graph::*;
/// let ncbi_taxonomy_node_name1 = "NCBITaxon:264379";
/// let ncbi_taxonomy_node_name2 = "NCBITaxon:2697049";
/// let not_ncbi_taxonomy_node_name = "PizzaQuattroStagioni";
/// assert!(is_valid_ncbi_taxonomy_node_name(ncbi_taxonomy_node_name1));
/// assert!(is_valid_ncbi_taxonomy_node_name(ncbi_taxonomy_node_name2));
/// assert!(!is_valid_ncbi_taxonomy_node_name(not_ncbi_taxonomy_node_name));
/// ```
pub fn is_valid_ncbi_taxonomy_node_name(node_name: &str) -> bool {
    is_valid_node_name_from_seeds(
        node_name,
        Some(&["NCBITaxon"]),
        None,
        Some(":"),
        None,
        None,
        None,
    )
    .is_ok()
}

/// Returns URL from given NCBI taxonomy node name.
///
/// # Arguments
/// * `node_name`: &str - Node name to check pattern with.
///
/// # Safety
/// This method assumes that the provided node name is a NCBI taxonomy node name and
/// may cause a panic if the aforementioned assumption is not true.
pub(crate) unsafe fn format_ncbi_taxonomy_url_from_node_name(node_name: &str) -> String {
    format_url_from_node_name(
        "https://www.ncbi.nlm.nih.gov/taxonomy/?term={node_name}",
        node_name,
        Some(":"),
    )
}
