use super::*;

#[automatically_generated_function]
/// Returns whether the given node name respects the Yeast Genome Locus nodes pattern.
///
/// # Arguments
/// * `node_name`: &str - Node name to check pattern with.
///
/// # Example
/// To validate a node you can use:
/// ```ignore
/// # use graph::*;
/// let this_library_node_name = "SGD:S000005675";
/// let not_this_library_node_name = "PizzaQuattroStagioni";
/// assert!(is_valid_yeast_genome_locus_node_name(this_library_node_name));
/// assert!(!is_valid_yeast_genome_locus_node_name(not_this_library_node_name));
/// ```
pub fn is_valid_yeast_genome_locus_node_name(node_name: &str) -> bool {
    is_valid_node_name_from_seeds(
        node_name,
        Some(&["SGD"]),
        Some(14),
        Some(":"),
        Some("S"),
        Some(10),
        Some(9),
    )
    .is_ok()
}

#[automatically_generated_function]
/// Returns URL from given Yeast Genome Locus node name.
///
/// # Arguments
/// * `node_name`: &str - Node name to check pattern with.
///
/// # Safety
/// This method assumes that the provided node name is a Yeast Genome Locus node name and
/// may cause a panic if the aforementioned assumption is not true.
///
pub(crate) unsafe fn format_yeast_genome_locus_url_from_node_name(node_name: &str) -> String {
    format_url_from_node_name(
        "https://www.yeastgenome.org/locus/{node_name}",
        node_name,
        Some(":"),
    )
}
