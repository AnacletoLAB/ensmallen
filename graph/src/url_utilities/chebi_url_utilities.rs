use super::*;

/// Returns whether the given node name respects the chebi nodes pattern.
///
/// # Arguments
/// * `node_name`: &str - Node name to check pattern with.
///
/// # Example
/// To validate a node you can use:
/// ```rust
/// # use graph::*;
/// let chebi_node_name1 = "CHEBI:145556";
/// let chebi_node_name2 = "CHEBI:85302";
/// let not_chebi_node_name = "PizzaQuattroStagioni";
/// assert!(is_valid_chebi_node_name(chebi_node_name1));
/// assert!(is_valid_chebi_node_name(chebi_node_name2));
/// assert!(!is_valid_chebi_node_name(not_chebi_node_name));
/// ```
pub fn is_valid_chebi_node_name(node_name: &str) -> bool {
    is_valid_node_name_from_seeds(
        node_name,
        Some(&["CHEBI"]),
        None,
        Some(":"),
        None,
        None,
        None,
    )
    .is_ok()
}

/// Returns URL from given chebi node name.
///
/// # Arguments
/// * `node_name`: &str - Node name to check pattern with.
///
/// # Safety
/// This method assumes that the provided node name is a chebi node name and
/// may cause a panic if the aforementioned assumption is not true.
pub(crate) unsafe fn format_chebi_url_from_node_name(node_name: &str) -> String {
    format_url_from_node_name(
        "https://www.ebi.ac.uk/chebi/searchId.do?chebiId={node_name}",
        node_name,
        None,
    )
}
