use super::*;
/// Returns whether the given node name respects the doi nodes pattern.
///
/// # Arguments
/// * `node_name`: &str - Node name to check pattern with.
///
/// # Example
/// To validate a node you can use:
/// ```ignore
/// # use graph::*;
/// let doi_node_name1 = "DOI:10.1002/1873-3468.12198";
/// let doi_node_name2 = "DOI:000337984";
/// let not_doi_node_name = "PizzaQuattroStagioni";
/// assert!(is_valid_doi_node_name(doi_node_name1));
/// assert!(is_valid_doi_node_name(doi_node_name2));
/// assert!(!is_valid_doi_node_name(not_doi_node_name));
/// ```
pub fn is_valid_doi_node_name(node_name: &str) -> bool {
    is_valid_node_name_from_seeds(node_name, Some(&["DOI"]), None, Some(":"), None, None, None)
        .is_ok()
}

/// Returns URL from given doi node name.
///
/// # Arguments
/// * `node_name`: &str - Node name to check pattern with.
///
/// # Safety
/// This method assumes that the provided node name is a doi node name and
/// may cause a panic if the aforementioned assumption is not true.
pub(crate) unsafe fn format_doi_url_from_node_name(node_name: &str) -> String {
    format_url_from_node_name("http://doi.org/{node_name}", node_name, Some(":"))
}
