use super::*;
/// Returns whether the given node name respects the Coriell nodes pattern.
///
/// # Arguments
/// * `node_name`: &str - Node name to check pattern with.
///
/// # Example
/// To validate a node you can use:
/// ```rust
/// # use graph::*;
/// let coriell_node_name = "Coriell:AG01439";
/// let not_coriell_node_name = "PizzaQuattroStagioni";
/// assert!(is_valid_coriell_node_name(coriell_node_name));
/// assert!(!is_valid_coriell_node_name(not_coriell_node_name));
/// ```
pub fn is_valid_coriell_node_name(node_name: &str) -> bool {
    is_valid_node_name_from_seeds(
        node_name,
        Some(&["Coriell"]),
        Some(15),
        Some(":"),
        None,
        None,
        None,
    )
    .is_ok()
}

/// Returns URL from given Coriell node name.
///
/// # Arguments
/// * `node_name`: &str - Node name to check pattern with.
///
/// # Safety
/// This method assumes that the provided node name is a Coriell node name and
/// may cause a panic if the aforementioned assumption is not true.
pub(crate) unsafe fn format_coriell_url_from_node_name(node_name: &str) -> String {
    format_url_from_node_name(
        "https://coriell.org/0/Sections/Search/Sample_Detail.aspx?Ref={node_name}",
        node_name,
        Some(":"),
    )
}