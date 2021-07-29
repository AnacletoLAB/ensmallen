use tags::*;
use crate::general_url_utilities::*;

#[automatically_generated_function]
/// Returns whether the given node name respects the NCBI Books nodes pattern.
///
/// # Arguments
/// * `node_name`: &str - Node name to check pattern with.
///
/// # Example
/// To validate a node you can use:
/// ```rust
/// # use url_utilities::*;
/// let this_library_node_name = "GeneReviews:NBK100238";
/// let not_this_library_node_name = "PizzaQuattroStagioni";
/// assert!(is_valid_ncbi_books_node_name(this_library_node_name));
/// assert!(!is_valid_ncbi_books_node_name(not_this_library_node_name));
/// ```
pub fn is_valid_ncbi_books_node_name(node_name: &str) -> bool {
    is_valid_node_name_from_seeds(
        node_name,
        Some(&["GENEREVIEWS"]),
        None,
        Some(":"),
        Some("NBK"),
        None,
        None,
    )
    .is_ok()
}

#[automatically_generated_function]
/// Returns URL from given NCBI Books node name.
///
/// # Arguments
/// * `node_name`: &str - Node name to check pattern with.
///
/// # Safety
/// This method assumes that the provided node name is a NCBI Books node name and
/// may cause a panic if the aforementioned assumption is not true.
///
pub(crate) unsafe fn format_ncbi_books_url_from_node_name(node_name: &str) -> String {
    format_url_from_node_name(
        "http://www.ncbi.nlm.nih.gov/books/{node_name}",
        node_name,
        Some(":"),
    )
}
