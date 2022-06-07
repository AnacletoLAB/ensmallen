use super::*;

/// Returns whether the given node name respects the Therapeutic Target Database nodes pattern.
///
/// # Arguments
/// * `node_name`: &str - Node name to check pattern with.
///
/// # Example
/// To validate a node you can use:
/// ```ignore
/// # use graph::*;
/// let therapeutic_target_database_node_name1 = "ttd.drug:D0O1KD";
/// let therapeutic_target_database_node_name2 = "ttd.drug:D06FHW";
/// let therapeutic_target_database_node_name3 = "ttd.drug:D09RZX";
/// let therapeutic_target_database_node_name4 = "ttd.drug:D0G2MM";
/// let therapeutic_target_database_node_name5 = "ttd.drug:D08-Sep";
/// let not_therapeutic_target_database_node_name = "PizzaQuattroStagioni";
/// assert!(is_valid_therapeutic_target_database_node_name(therapeutic_target_database_node_name1));
/// assert!(is_valid_therapeutic_target_database_node_name(therapeutic_target_database_node_name2));
/// assert!(is_valid_therapeutic_target_database_node_name(therapeutic_target_database_node_name3));
/// assert!(is_valid_therapeutic_target_database_node_name(therapeutic_target_database_node_name4));
/// assert!(is_valid_therapeutic_target_database_node_name(therapeutic_target_database_node_name5));
/// assert!(!is_valid_therapeutic_target_database_node_name(not_therapeutic_target_database_node_name));
/// ```
pub fn is_valid_therapeutic_target_database_node_name(node_name: &str) -> bool {
    is_valid_node_name_from_seeds(
        node_name,
        Some(&["ttd.drug"]),
        None,
        Some(":"),
        None,
        None,
        None,
    )
    .is_ok()
}

/// Returns URL from given Therapeutic Target Database node name.
///
/// # Arguments
/// * `node_name`: &str - Node name to check pattern with.
///
/// # Safety
/// This method assumes that the provided node name is a therapeutic_target_database node name and
/// may cause a panic if the aforementioned assumption is not true.
pub(crate) unsafe fn format_therapeutic_target_database_url_from_node_name(
    node_name: &str,
) -> String {
    format_url_from_node_name(
        "http://db.idrblab.net/ttd/search/ttd/target?search_api_fulltext={node_name}",
        node_name,
        Some(":"),
    )
}
