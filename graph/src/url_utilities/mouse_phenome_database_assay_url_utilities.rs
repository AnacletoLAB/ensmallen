use super::*;

#[automatically_generated_function]
/// Returns whether the given node name respects the Mouse Phenome Database Assay nodes pattern.
///
/// # Arguments
/// * `node_name`: &str - Node name to check pattern with.
///
/// # Example
/// To validate a node you can use:
/// ```rust
/// # use graph::*;
/// let this_library_node_name = "MPD-assay:10305";
/// let not_this_library_node_name = "PizzaQuattroStagioni";
/// assert!(is_valid_mouse_phenome_database_assay_node_name(this_library_node_name));
/// assert!(!is_valid_mouse_phenome_database_assay_node_name(not_this_library_node_name));
/// ```
pub fn is_valid_mouse_phenome_database_assay_node_name(node_name: &str) -> bool {
    is_valid_node_name_from_seeds(
        node_name,
        Some(&["MPD-ASSAY"]),
        None,
        Some(":"),
        None,
        None,
        None,
    )
    .is_ok()
}

#[automatically_generated_function]
/// Returns URL from given Mouse Phenome Database Assay node name.
///
/// # Arguments
/// * `node_name`: &str - Node name to check pattern with.
///
/// # Safety
/// This method assumes that the provided node name is a Mouse Phenome Database Assay node name and
/// may cause a panic if the aforementioned assumption is not true.
///
pub(crate) unsafe fn format_mouse_phenome_database_assay_url_from_node_name(
    node_name: &str,
) -> String {
    format_url_from_node_name(
        "https://phenome.jax.org/db/qp?rtn=views/catlines&keymeas={node_name}",
        node_name,
        Some(":"),
    )
}
