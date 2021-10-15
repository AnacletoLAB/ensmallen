use super::*;

/// Returns whether the given node name respects the Gene Ontology Relations nodes pattern.
///
/// # Arguments
/// * `node_name`: &str - Node name to check pattern with.
///
/// # Example
/// To validate a node you can use:
/// ```ignore
/// # use graph::*;
/// let this_library_node_name = "<https://docs.rs/url/2.2.2/url/>";
/// let not_this_library_node_name1 = "PizzaQuattroStagioni";
/// let not_this_library_node_name2 = "CORD:b65faf5b8f0846d278b50285898e849e45d30839";
/// assert!(is_valid_angular_link(this_library_node_name));
/// assert!(!is_valid_angular_link(not_this_library_node_name1));
/// assert!(!is_valid_angular_link(not_this_library_node_name2));
/// ```
pub fn is_valid_angular_link(node_name: &str) -> bool {
    node_name.starts_with("<")
        && node_name.ends_with(">")
        && is_valid_website_node_name(format_angular_link_url_from_object(node_name).as_str())
}

/// Returns URL from given Gene Ontology Relations node name.
///
/// # Arguments
/// * `node_name`: &str - Node name to check pattern with.
///
/// # Safety
/// This method assumes that the provided node name is a Gene Ontology Relations node name and
/// may cause a panic if the aforementioned assumption is not true.
///
pub(crate) fn format_angular_link_url_from_object(node_name: &str) -> String {
    let mut chars = node_name.chars();
    chars.next();
    chars.next_back();
    chars.as_str().to_string()
}
