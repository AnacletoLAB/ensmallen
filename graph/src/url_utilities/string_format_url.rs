use super::*;

#[automatically_generated_function]
/// Returns URL from given STRING node name.
///
/// # Arguments
/// * `node_name`: &str - Node name to check pattern with.
///
/// # Safety
/// This method assumes that the provided node name is a {library_name} node name and
/// may cause a panic if the aforementioned assumption is not true.
///
pub(crate) unsafe fn format_string_url_from_node_name(node_name: &str) -> String {
    {
        format!("https://string-db.org/network/{}", node_name)
    }
}
