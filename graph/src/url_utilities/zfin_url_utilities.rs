use super::*;

/// Returns whether the given node name respects the zfin nodes pattern.
///
/// # Arguments
/// * `node_name`: &str - Node name to check pattern with.
///
/// # Example
/// To validate a node you can use:
/// ```ignore
/// # use graph::*;
/// let zfin_node_name1 = "ZFIN:ZDB-GENE-130530-778";
/// let zfin_node_name2 = "ZDB-GENE-101108-4";
/// let not_zfin_node_name = "PizzaQuattroStagioni";
/// assert!(is_valid_zfin_node_name(zfin_node_name1));
/// assert!(is_valid_zfin_node_name(zfin_node_name2));
/// assert!(!is_valid_zfin_node_name(not_zfin_node_name));
/// ```
pub fn is_valid_zfin_node_name(node_name: &str) -> bool {
    is_valid_node_name_from_seeds(
        node_name,
        Some(&["ZFIN"]),
        None,
        Some(":"),
        Some("ZDB"),
        None,
        None,
    )
    .is_ok()
}

/// Returns URL from given zfin node name.
///
/// # Arguments
/// * `node_name`: &str - Node name to check pattern with.
///
/// # Safety
/// This method assumes that the provided node name is a zfin node name and
/// may cause a panic if the aforementioned assumption is not true.
pub(crate) unsafe fn format_zfin_url_from_node_name(node_name: &str) -> String {
    format_url_from_node_name("http://zfin.org/{node_name}", node_name, Some(":"))
}
