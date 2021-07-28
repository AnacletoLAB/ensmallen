use super::*;

/// Returns whether the given node name respects the ensembl nodes pattern.
///
/// # Arguments
/// * `node_name`: &str - Node name to check pattern with.
///
/// # Example
/// To validate a node you can use:
/// ```rust
/// # use graph::*;
/// let ensembl_node_name1 = "ENSEMBL:ENSACAG00000000017";
/// let ensembl_node_name2 = "ENSACAG00000000017";
/// let ensembl_node_name3 = "ENSEMBL:ENSG00000004059";
/// let not_ensembl_node_name = "PizzaQuattroStagioni";
/// assert!(is_valid_ensembl_node_name(ensembl_node_name1));
/// assert!(is_valid_ensembl_node_name(ensembl_node_name2));
/// assert!(is_valid_ensembl_node_name(ensembl_node_name2));
/// assert!(!is_valid_ensembl_node_name(not_ensembl_node_name));
/// ```
pub fn is_valid_ensembl_node_name(node_name: &str) -> bool {
    is_valid_node_name_from_seeds(
        node_name,
        Some(&["ENSEMBL"]),
        None,
        Some(":"),
        Some("ENS"),
        None,
        None,
    )
    .is_ok()
}

/// Returns URL from given ensembl node name.
///
/// # Arguments
/// * `node_name`: &str - Node name to check pattern with.
///
/// # Safety
/// This method assumes that the provided node name is a ensembl node name and
/// may cause a panic if the aforementioned assumption is not true.
pub(crate) unsafe fn format_ensembl_url_from_node_name(node_name: &str) -> String {
    format_url_from_node_name(
        "https://www.ensembl.org/Gene/Summary?g={node_name}",
        node_name,
        Some(":"),
    )
}