use super::*;
/// Returns whether the given node name respects the mouse genome informatics nodes pattern.
///
/// # Arguments
/// * `node_name`: &str - Node name to check pattern with.
///
/// # Example
/// To validate a node you can use:
/// ```rust
/// # use graph::*;
/// let mouse_genome_informatics_node_name1 = "MGI:2159965";
/// let mouse_genome_informatics_node_name2 = "MGI:MGI:2159965";
/// let not_mouse_genome_informatics_node_name = "PizzaQuattroStagioni";
/// assert!(is_valid_mouse_genome_informatics_node_name(mouse_genome_informatics_node_name1));
/// assert!(!is_valid_mouse_genome_informatics_node_name(mouse_genome_informatics_node_name2));
/// assert!(!is_valid_mouse_genome_informatics_node_name(not_mouse_genome_informatics_node_name));
/// ```
pub fn is_valid_mouse_genome_informatics_node_name(node_name: &str) -> bool {
    is_valid_node_name_from_seeds(node_name, Some(&["MGI"]), None, Some(":"), None, None, None)
        .is_ok()
}

/// Returns URL from given mouse genome informatics node name.
///
/// # Arguments
/// * `node_name`: &str - Node name to check pattern with.
///
/// # Safety
/// This method assumes that the provided node name is a mouse genome informatics node name and
/// may cause a panic if the aforementioned assumption is not true.
pub(crate) unsafe fn format_mouse_genome_informatics_url_from_node_name(node_name: &str) -> String {
    format_url_from_node_name(
        "http://www.informatics.jax.org/reference/strain/{node_name}",
        node_name,
        None,
    )
}
