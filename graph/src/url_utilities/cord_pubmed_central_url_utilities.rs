use super::*;

/// Returns whether the given node name respects the CORD Pubmed Central nodes pattern.
///
/// # Arguments
/// * `node_name`: &str - Node name to check pattern with.
///
/// # Example
/// To validate a node you can use:
/// ```rust
/// # use graph::*;
/// let cord_pubmed_central_node_name1 = "CORD:PMC7187825";
/// let cord_pubmed_central_node_name2 = "PMC7187825";
/// let not_cord_pubmed_central_node_name = "PizzaQuattroStagioni";
/// assert!(is_valid_cord_pubmed_central_node_name(cord_pubmed_central_node_name1));
/// assert!(is_valid_cord_pubmed_central_node_name(cord_pubmed_central_node_name2));
/// assert!(!is_valid_cord_pubmed_central_node_name(not_cord_pubmed_central_node_name));
/// ```
pub fn is_valid_cord_pubmed_central_node_name(node_name: &str) -> bool {
    is_valid_node_name_from_seeds(
        node_name,
        Some(&["CORD"]),
        Some(15),
        Some(":"),
        Some("PMC"),
        Some(10),
        Some(7),
    )
    .is_ok()
}

/// Returns URL from given CORD Pubmed Central node name.
///
/// # Arguments
/// * `node_name`: &str - Node name to check pattern with.
///
/// # Safety
/// This method assumes that the provided node name is a CORD Pubmed Central node name and
/// may cause a panic if the aforementioned assumption is not true.
pub(crate) unsafe fn format_cord_pubmed_central_url_from_node_name(node_name: &str) -> String {
    format_url_from_node_name(
        "https://www.ncbi.nlm.nih.gov/pmc/articles/{node_name}",
        node_name,
        Some(":"),
    )
}
