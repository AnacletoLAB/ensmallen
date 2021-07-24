
use super::*;

#[automatically_generated_function]
/// Returns whether the given node name respects the Wormbase Vocabulary nodes pattern.
///
/// # Arguments
/// * `node_name`: &str - Node name to check pattern with.
///
/// # Example
/// To validate a node you can use:
/// ```rust
/// # use graph::*;
/// let this_library_node_name = "WBVocab:Gene-GO-Association";
/// let not_this_library_node_name = "PizzaQuattroStagioni";
/// assert!(is_valid_wormbase_vocabulary_node_name(this_library_node_name));
/// assert!(!is_valid_wormbase_vocabulary_node_name(not_this_library_node_name));
/// ```
pub fn is_valid_wormbase_vocabulary_node_name(node_name: &str) -> bool {
    is_valid_node_name_from_seeds(
        node_name,
        Some(&["WBVOCAB"]),
        None,
        Some(":"),
        Some("GENE-G"),
        None,
        None
    )
    .is_ok()
}


#[automatically_generated_function]
/// Returns URL from given Wormbase Vocabulary node name.
///
/// # Arguments
/// * `node_name`: &str - Node name to check pattern with.
///
/// # Safety
/// This method assumes that the provided node name is a Wormbase Vocabulary node name and
/// may cause a panic if the aforementioned assumption is not true.
///
pub(crate) unsafe fn format_wormbase_vocabulary_url_from_node_name(node_name: &str) -> String {
    format_url_from_node_name(
        "http://bio2rdf.org/wormbase_vocabulary{node_name}",
        node_name,
        Some(":"),
    )
}

