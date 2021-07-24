
use super::*;

#[automatically_generated_function]
/// Returns whether the given node name respects the Mouse Phenome Database Strain nodes pattern.
///
/// # Arguments
/// * `node_name`: &str - Node name to check pattern with.
///
/// # Example
/// To validate a node you can use:
/// ```rust
/// # use graph::*;
/// let this_library_node_name = "MPD-strain:1000";
/// let not_this_library_node_name = "PizzaQuattroStagioni";
/// assert!(is_valid_mouse_phenome_database_strain_node_name(this_library_node_name));
/// assert!(!is_valid_mouse_phenome_database_strain_node_name(not_this_library_node_name));
/// ```
pub fn is_valid_mouse_phenome_database_strain_node_name(node_name: &str) -> bool {
    is_valid_node_name_from_seeds(
        node_name,
        Some(&["MPD-STRAIN"]),
        None,
        Some(":"),
        None,
        None,
        None
    )
    .is_ok()
}


#[automatically_generated_function]
/// Returns URL from given Mouse Phenome Database Strain node name.
///
/// # Arguments
/// * `node_name`: &str - Node name to check pattern with.
///
/// # Safety
/// This method assumes that the provided node name is a Mouse Phenome Database Strain node name and
/// may cause a panic if the aforementioned assumption is not true.
///
pub(crate) unsafe fn format_mouse_phenome_database_strain_url_from_node_name(node_name: &str) -> String {
    format_url_from_node_name(
        "http://phenome.jax.org/db/q?rtn=strains/details&strainid={node_name}",
        node_name,
        Some(":"),
    )
}

