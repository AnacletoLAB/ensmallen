/// Returns whether the given node name respects the Website nodes pattern.
///
/// # Arguments
/// * `node_name`: &str - Node name to check pattern with.
///
/// # Example
/// To validate a node you can use:
/// ```ignore
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

/// Returns URL from given website node name.
///
/// # Arguments
/// * `node_name`: &str - Node name to check pattern with.
///
pub(crate) fn format_website_url_from_node_name(node_name: &str) -> String {
    node_name.to_string()
}
