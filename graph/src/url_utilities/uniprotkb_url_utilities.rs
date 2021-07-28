use super::*;

/// Returns whether the given node name respects the UNIPROT nodes pattern.
///
/// # Arguments
/// * `node_name`: &str - Node name to check pattern with.
///
/// # Example
/// To validate a node you can use:
/// ```rust
/// # use graph::*;
/// let uniprotkb_node_name1 = "UniProtKB:Q63ZW2";
/// let uniprotkb_node_name2 = "UniProtKB:A0A2R9YJI3";
/// let not_uniprotkb_node_name = "PizzaQuattroStagioni";
/// assert!(is_valid_uniprotkb_node_name(uniprotkb_node_name1));
/// assert!(is_valid_uniprotkb_node_name(uniprotkb_node_name2));
/// assert!(!is_valid_uniprotkb_node_name(not_uniprotkb_node_name));
/// ```
pub fn is_valid_uniprotkb_node_name(node_name: &str) -> bool {
    is_valid_node_name_from_seeds(
        node_name,
        Some(&["UniProtKB"]),
        None,
        Some(":"),
        None,
        None,
        None,
    )
    .is_ok()
}

/// Returns URL from given UniProtKB node name.
///
/// # Arguments
/// * `node_name`: &str - Node name to check pattern with.
///
/// # Safety
/// This method assumes that the provided node name is a UniProtKB node name and
/// may cause a panic if the aforementioned assumption is not true.
pub(crate) unsafe fn format_uniprotkb_url_from_node_name(node_name: &str) -> String {
    format_url_from_node_name(
        "https://www.uniprot.org/uniprot/{node_name}",
        node_name,
        Some(":"),
    )
}
