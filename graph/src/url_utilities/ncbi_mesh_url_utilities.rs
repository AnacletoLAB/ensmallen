use super::*;

/// Returns whether the given node name respects the NCBI MESH nodes pattern.
///
/// # Arguments
/// * `node_name`: &str - Node name to check pattern with.
///
/// # Example
/// To validate a node you can use:
/// ```rust
/// # use graph::*;
/// let ncbi_mesh_node_name = "MESH:217";
/// let not_ncbi_mesh_node_name = "PizzaQuattroStagioni";
/// assert!(is_valid_ncbi_mesh_node_name(ncbi_mesh_node_name));
/// assert!(!is_valid_ncbi_mesh_node_name(not_ncbi_mesh_node_name));
/// ```
pub fn is_valid_ncbi_mesh_node_name(node_name: &str) -> bool {
    is_valid_node_name_from_seeds(node_name, Some("MESH"), None, Some(":"), None, None, None)
        .is_ok()
}

/// Returns URL from given NCBI MESH node name.
///
/// # Arguments
/// * `node_name`: &str - Node name to check pattern with.
///
/// # Safety
/// This method assumes that the provided node name is a NCBI MESH node name and
/// may cause a panic if the aforementioned assumption is not true.
pub(crate) unsafe fn format_ncbi_mesh_url_from_node_name(node_name: &str) -> String {
    format_url_from_node_name(
        "https://ncbi.nlm.nih.gov/mesh/?term={node_name}",
        node_name,
        Some(":"),
    )
}