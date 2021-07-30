use super::*;

/// Returns whether the given node name respects the wikidata nodes pattern.
///
/// # Arguments
/// * `node_name`: &str - Node name to check pattern with.
///
/// # Example
/// To validate a node you can use:
/// ```rust
/// # use graph::*;
/// let wikidata_node_name = "WD:Q30";
/// let wikidata_node_name = "WIKIDATA:Q30";
/// let not_wikidata_node_name = "PizzaQuattroStagioni";
/// assert!(is_valid_wikidata_node_name(wikidata_node_name));
/// assert!(!is_valid_wikidata_node_name(not_wikidata_node_name));
/// ```
pub fn is_valid_wikidata_node_name(node_name: &str) -> bool {
    is_valid_node_name_from_seeds(
        node_name,
        Some(&["WD", "WIKIDATA"]),
        None,
        Some(":"),
        None,
        None,
        None,
    )
    .is_ok()
}

/// Returns URL from given WikiData node name.
///
/// # Arguments
/// * `node_name`: &str - Node name to check pattern with.
///
/// # Safety
/// This method assumes that the provided node name is a WikiData node name and
/// may cause a panic if the aforementioned assumption is not true.
pub(crate) unsafe fn format_wikidata_url_from_node_name(node_name: &str) -> String {
    format_url_from_node_name(
        "https://www.wikidata.org/wiki/{node_name}",
        node_name,
        Some(":"),
    )
}
