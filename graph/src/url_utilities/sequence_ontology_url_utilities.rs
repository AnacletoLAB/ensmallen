use super::*;

/// Returns whether the given node name respects the sequence ontology nodes pattern.
///
/// # Arguments
/// * `node_name`: &str - Node name to check pattern with.
///
/// # Example
/// To validate a node you can use:
/// ```ignore
/// # use graph::*;
/// let sequence_ontology_node_name = "SO:0001217";
/// let not_sequence_ontology_node_name = "PizzaQuattroStagioni";
/// assert!(is_valid_sequence_ontology_node_name(sequence_ontology_node_name));
/// assert!(!is_valid_sequence_ontology_node_name(not_sequence_ontology_node_name));
/// ```
pub fn is_valid_sequence_ontology_node_name(node_name: &str) -> bool {
    is_valid_node_name_from_seeds(
        node_name,
        Some(&["SO"]),
        Some(10),
        Some(":"),
        None,
        None,
        Some(7),
    )
    .is_ok()
}

/// Returns URL from given sequence ontology node name.
///
/// # Arguments
/// * `node_name`: &str - Node name to check pattern with.
///
/// # Safety
/// This method assumes that the provided node name is a sequence ontology node name and
/// may cause a panic if the aforementioned assumption is not true.
pub(crate) unsafe fn format_sequence_ontology_url_from_node_name(node_name: &str) -> String {
    format_url_from_node_name(
        "http://www.sequenceontology.org/browser/current_svn/term/{node_name}",
        node_name,
        None,
    )
}
