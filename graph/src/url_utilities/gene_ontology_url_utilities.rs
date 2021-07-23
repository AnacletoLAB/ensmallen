use super::*;

/// Returns whether the given node name respects the Gene Ontology nodes pattern.
///
/// # Arguments
/// * `node_name`: &str - Node name to check pattern with.
///
/// # Example
/// To validate a node you can use:
/// ```rust
/// # use graph::*;
/// let go_node_name1 = "GO:0043281PHENOTYPE";
/// let go_node_name2 = "GO:0043281";
/// let not_go_node_name = "PizzaQuattroStagioni";
/// assert!(is_valid_gene_ontology_node_name(go_node_name1));
/// assert!(is_valid_gene_ontology_node_name(go_node_name2));
/// assert!(!is_valid_gene_ontology_node_name(not_go_node_name));
/// ```
pub fn is_valid_gene_ontology_node_name(node_name: &str) -> bool {
    let mut node_name = node_name.to_string();
    if node_name.ends_with("PHENOTYPE") {
        node_name = node_name[0..(node_name.len() - 9)].to_string();
    }
    is_valid_node_name_from_seeds(
        node_name.as_ref(),
        Some("GO"),
        Some(10),
        Some(":"),
        None,
        None,
        Some(7),
    )
    .is_ok()
}

/// Returns URL from given Gene Ontology node name.
///
/// # Arguments
/// * `node_name`: &str - Node name to check pattern with.
///
/// # Safety
/// This method assumes that the provided node name is a Gene Ontology node name and
/// may cause a panic if the aforementioned assumption is not true.
pub(crate) unsafe fn format_gene_ontology_url_from_node_name(node_name: &str) -> String {
    format_url_from_node_name(
        "http://amigo.geneontology.org/amigo/term/{node_name}",
        node_name,
        None,
    )
}