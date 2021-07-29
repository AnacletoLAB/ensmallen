use crate::general_url_utilities::*;

/// Returns whether the given node name respects the rat_genome_database nodes pattern.
///
/// # Arguments
/// * `node_name`: &str - Node name to check pattern with.
///
/// # Example
/// To validate a node you can use:
/// ```rust
/// # use url_utilities::*;
/// let rat_genome_database_node_name1 = "RGD:10059728";
/// let rat_genome_database_node_name2 = "RGD:1564808";
/// let rat_genome_database_node_name3 = "RGD:3157";
/// let not_rat_genome_database_node_name = "PizzaQuattroStagioni";
/// assert!(is_valid_rat_genome_database_node_name(rat_genome_database_node_name1));
/// assert!(is_valid_rat_genome_database_node_name(rat_genome_database_node_name2));
/// assert!(is_valid_rat_genome_database_node_name(rat_genome_database_node_name3));
/// assert!(!is_valid_rat_genome_database_node_name(not_rat_genome_database_node_name));
/// ```
pub fn is_valid_rat_genome_database_node_name(node_name: &str) -> bool {
    is_valid_node_name_from_seeds(node_name, Some(&["RGD"]), None, Some(":"), None, None, None)
        .is_ok()
}

/// Returns URL from given Rat Genome DataBase node name.
///
/// # Arguments
/// * `node_name`: &str - Node name to check pattern with.
///
/// # Safety
/// This method assumes that the provided node name is a Rat Genome DataBase node name and
/// may cause a panic if the aforementioned assumption is not true.
pub(crate) unsafe fn format_rat_genome_database_url_from_node_name(node_name: &str) -> String {
    format_url_from_node_name(
        "https://rgd.mcw.edu/rgdweb/report/gene/main.html?id={node_name}",
        node_name,
        Some(":"),
    )
}
