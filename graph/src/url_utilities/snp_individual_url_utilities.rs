use super::*;

#[automatically_generated_function]
/// Returns whether the given node name respects the SNP Individual nodes pattern.
///
/// # Arguments
/// * `node_name`: &str - Node name to check pattern with.
///
/// # Example
/// To validate a node you can use:
/// ```rust
/// # use graph::*;
/// let this_library_node_name = "dbSNPIndividual:16370";
/// let not_this_library_node_name = "PizzaQuattroStagioni";
/// assert!(is_valid_snp_individual_node_name(this_library_node_name));
/// assert!(!is_valid_snp_individual_node_name(not_this_library_node_name));
/// ```
pub fn is_valid_snp_individual_node_name(node_name: &str) -> bool {
    is_valid_node_name_from_seeds(
        node_name,
        Some(&["DBSNPINDIVIDUAL"]),
        None,
        Some(":"),
        None,
        None,
        None,
    )
    .is_ok()
}

#[automatically_generated_function]
/// Returns URL from given SNP Individual node name.
///
/// # Arguments
/// * `node_name`: &str - Node name to check pattern with.
///
/// # Safety
/// This method assumes that the provided node name is a SNP Individual node name and
/// may cause a panic if the aforementioned assumption is not true.
///
pub(crate) unsafe fn format_snp_individual_url_from_node_name(node_name: &str) -> String {
    format_url_from_node_name(
        "http://www.ncbi.nlm.nih.gov/SNP/snp_ind.cgi?ind_id={node_name}",
        node_name,
        Some(":"),
    )
}
