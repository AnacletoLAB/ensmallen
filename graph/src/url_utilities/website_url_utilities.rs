use super::*;

#[automatically_generated_function]
/// Returns whether the given node name respects the Gene Ontology Relations nodes pattern.
///
/// # Arguments
/// * `node_name`: &str - Node name to check pattern with.
///
/// # Example
/// To validate a node you can use:
/// ```rust
/// # use graph::*;
/// let this_library_node_name = "https://docs.rs/url/2.2.2/url/";
/// let not_this_library_node_name1 = "PizzaQuattroStagioni";
/// let not_this_library_node_name2 = "CORD:b65faf5b8f0846d278b50285898e849e45d30839";
/// assert!(is_valid_website_node_name(this_library_node_name));
/// assert!(!is_valid_website_node_name(not_this_library_node_name1));
/// assert!(!is_valid_website_node_name(not_this_library_node_name2));
/// ```
pub fn is_valid_website_node_name(node_name: &str) -> bool {
    (node_name.starts_with("http") || node_name.starts_with("ftp"))
        && validator::validate_url(node_name)
}

#[automatically_generated_function]
/// Returns URL from given Gene Ontology Relations node name.
///
/// # Arguments
/// * `node_name`: &str - Node name to check pattern with.
///
/// # Safety
/// This method assumes that the provided node name is a Gene Ontology Relations node name and
/// may cause a panic if the aforementioned assumption is not true.
///
pub(crate) unsafe fn format_website_url_from_node_name(node_name: &str) -> String {
    node_name.to_string()
}
