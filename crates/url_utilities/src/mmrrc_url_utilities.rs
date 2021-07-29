use crate::general_url_utilities::*;

/// Returns whether the given node name respects the Mutant Mouse Resource & Research Center nodes pattern.
///
/// # Arguments
/// * `node_name`: &str - Node name to check pattern with.
///
/// # Example
/// To validate a node you can use:
/// ```rust
/// # use url_utilities::*;
/// let mmrrc_node_name = "MMRRC:000123";
/// let not_mmrrc_node_name = "PizzaQuattroStagioni";
/// assert!(is_valid_mmrrc_node_name(mmrrc_node_name));
/// assert!(!is_valid_mmrrc_node_name(not_mmrrc_node_name));
/// ```
pub fn is_valid_mmrrc_node_name(node_name: &str) -> bool {
    is_valid_node_name_from_seeds(
        node_name,
        Some(&["MMRRC"]),
        Some(12),
        Some(":"),
        None,
        None,
        Some(6),
    )
    .is_ok()
}

/// Returns URL from given Mutant Mouse Resource & Research Center node name.
///
/// # Arguments
/// * `node_name`: &str - Node name to check pattern with.
///
/// # Safety
/// This method assumes that the provided node name is a Mutant Mouse Resource & Research Center node name and
/// may cause a panic if the aforementioned assumption is not true.
pub(crate) unsafe fn format_mmrrc_url_from_node_name(node_name: &str) -> String {
    format_url_from_node_name(
        "https://www.mmrrc.org/catalog/sds.php?mmrrc_id={node_name}",
        node_name,
        Some(":"),
    )
}
