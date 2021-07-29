use tags::*;
use crate::general_url_utilities::*;

#[automatically_generated_function]
/// Returns whether the given node name respects the Uberon multi-species anatomy ontology nodes pattern.
///
/// # Arguments
/// * `node_name`: &str - Node name to check pattern with.
///
/// # Example
/// To validate a node you can use:
/// ```rust
/// # use url_utilities::*;
/// let this_library_node_name = "UBERON:0016525PHENOTYPE";
/// let not_this_library_node_name = "PizzaQuattroStagioni";
/// assert!(is_valid_uberon_multispecies_anatomy_ontology_node_name(this_library_node_name));
/// assert!(!is_valid_uberon_multispecies_anatomy_ontology_node_name(not_this_library_node_name));
/// ```
pub fn is_valid_uberon_multispecies_anatomy_ontology_node_name(node_name: &str) -> bool {
    is_valid_node_name_from_seeds(
        node_name,
        Some(&["UBERON"]),
        None,
        Some(":"),
        None,
        None,
        None,
    )
    .is_ok()
}

#[automatically_generated_function]
/// Returns URL from given Uberon multi-species anatomy ontology node name.
///
/// # Arguments
/// * `node_name`: &str - Node name to check pattern with.
///
/// # Safety
/// This method assumes that the provided node name is a Uberon multi-species anatomy ontology node name and
/// may cause a panic if the aforementioned assumption is not true.
///
pub(crate) unsafe fn format_uberon_multispecies_anatomy_ontology_url_from_node_name(
    node_name: &str,
) -> String {
    format_url_from_node_name(
        "http://purl.obolibrary.org/obo/UBERON_{node_name}",
        node_name,
        Some(":"),
    )
}
