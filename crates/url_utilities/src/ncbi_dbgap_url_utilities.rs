use crate::general_url_utilities::*;

/// Returns whether the given node name respects the NCBI dbGaP nodes pattern.
///
/// # Arguments
/// * `node_name`: &str - Node name to check pattern with.
///
/// # Example
/// To validate a node you can use:
/// ```rust
/// # use url_utilities::*;
/// let node_name1 = "DBGAP:phs001982.v1.p1";
/// let node_name2 = "phs001110.v2.p1";
/// let not_node_name = "PizzaQuattroStagioni";
/// assert!(is_valid_ncbi_dbgap_node_name(node_name1));
/// assert!(is_valid_ncbi_dbgap_node_name(node_name2));
/// assert!(!is_valid_ncbi_dbgap_node_name(not_node_name));
/// ```
pub fn is_valid_ncbi_dbgap_node_name(node_name: &str) -> bool {
    is_valid_node_name_from_seeds(
        node_name,
        Some(&["DBGAP"]),
        None,
        Some(":"),
        Some("phs"),
        None,
        None,
    )
    .is_ok()
}

/// Returns URL from given NCBI dbGaP node name.
///
/// # Arguments
/// * `node_name`: &str - Node name to check pattern with.
///
/// # Safety
/// This method assumes that the provided node name is a NCBI dbGaP node name and
/// may cause a panic if the aforementioned assumption is not true.
pub(crate) unsafe fn format_ncbi_dbgap_url_from_node_name(node_name: &str) -> String {
    format_url_from_node_name(
        "https://www.ncbi.nlm.nih.gov/projects/gap/cgi-bin/study.cgi?study_id={node_name}",
        node_name,
        Some(":"),
    )
}
