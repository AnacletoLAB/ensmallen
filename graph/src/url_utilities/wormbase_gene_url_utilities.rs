use super::*;

/// Returns whether the given node name respects the WormBase nodes pattern.
///
/// # Arguments
/// * `node_name`: &str - Node name to check pattern with.
///
/// # Example
/// To validate a node you can use:
/// ```rust
/// # use graph::*;
/// let wormbase_node_name1 = "WormBase:WBGene00195045";
/// let wormbase_node_name2 = "WBGene00195045";
/// let not_wormbase_node_name = "PizzaQuattroStagioni";
/// assert!(is_valid_wormbase_gene_node_name(wormbase_node_name1));
/// assert!(is_valid_wormbase_gene_node_name(wormbase_node_name2));
/// assert!(!is_valid_wormbase_gene_node_name(not_wormbase_node_name));
/// ```
pub fn is_valid_wormbase_gene_node_name(node_name: &str) -> bool {
    is_valid_node_name_from_seeds(
        node_name,
        Some(&["WormBase"]),
        Some(23),
        Some(":"),
        Some("WBGene"),
        Some(14),
        Some(8),
    )
    .is_ok()
}

/// Returns URL from given WormBase Gene node name.
///
/// # Arguments
/// * `node_name`: &str - Node name to check pattern with.
///
/// # Safety
/// This method assumes that the provided node name is a WormBase Gene node name and
/// may cause a panic if the aforementioned assumption is not true.
pub(crate) unsafe fn format_wormbase_gene_url_from_node_name(node_name: &str) -> String {
    format_url_from_node_name(
        "https://wormbase.org/search/all/{node_name}",
        node_name,
        Some(":"),
    )
}
