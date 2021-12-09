use super::*;

/// Returns whether the given node name respects the NCBI Gene Expression Omnibus nodes pattern.
///
/// # Arguments
/// * `node_name`: &str - Node name to check pattern with.
///
/// # Example
/// To validate a node you can use:
/// ```ignore
/// # use graph::*;
/// let node_name = "GEO.SERIES:GSE66597";
/// let not_node_name = "PizzaQuattroStagioni";
/// assert!(is_valid_ncbi_gene_expression_omnibus_node_name(node_name));
/// assert!(!is_valid_ncbi_gene_expression_omnibus_node_name(not_node_name));
/// ```
pub fn is_valid_ncbi_gene_expression_omnibus_node_name(node_name: &str) -> bool {
    is_valid_node_name_from_seeds(
        node_name,
        Some(&["GEO.SERIES"]),
        None,
        Some(":"),
        Some("GSE"),
        None,
        None,
    )
    .is_ok()
}

/// Returns URL from given NCBI Gene Expression Omnibus node name.
///
/// # Arguments
/// * `node_name`: &str - Node name to check pattern with.
///
/// # Safety
/// This method assumes that the provided node name is a NCBI Gene Expression Omnibus node name and
/// may cause a panic if the aforementioned assumption is not true.
pub(crate) unsafe fn format_ncbi_gene_expression_omnibus_url_from_node_name(
    node_name: &str,
) -> String {
    format_url_from_node_name(
        "https://www.ncbi.nlm.nih.gov/geo/query/acc.cgi?acc={node_name}",
        node_name,
        Some(":"),
    )
}
