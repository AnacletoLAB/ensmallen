use crate::general_url_utilities::*;

/// Returns whether the given node name respects the Pubmed NCBI nodes pattern.
///
/// # Arguments
/// * `node_name`: &str - Node name to check pattern with.
///
/// # Example
/// To validate a node you can use:
/// ```rust
/// # use url_utilities::*;
/// let pubmed_ncbi_node_name = "PMID:1001879";
/// let not_pubmed_ncbi_node_name = "PizzaQuattroStagioni";
/// assert!(is_valid_pubmed_ncbi_node_name(pubmed_ncbi_node_name));
/// assert!(!is_valid_pubmed_ncbi_node_name(not_pubmed_ncbi_node_name));
/// ```
pub fn is_valid_pubmed_ncbi_node_name(node_name: &str) -> bool {
    is_valid_node_name_from_seeds(
        node_name,
        Some(&["PMID"]),
        None,
        Some(":"),
        None,
        None,
        None,
    )
    .is_ok()
}

/// Returns URL from given Pubmed NCBI node name.
///
/// # Arguments
/// * `node_name`: &str - Node name to check pattern with.
///
/// # Safety
/// This method assumes that the provided node name is a Pubmed NCBI node name and
/// may cause a panic if the aforementioned assumption is not true.
pub(crate) unsafe fn format_pubmed_ncbi_url_from_node_name(node_name: &str) -> String {
    format_url_from_node_name(
        "https://pubmed.ncbi.nlm.nih.gov/{node_name}/",
        node_name,
        Some(":"),
    )
}
