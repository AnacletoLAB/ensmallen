/// Returns whether the given node name is valid according to given seeds.
///
/// # Arguments
/// * `node_name`: &str - The name of the nodes.
/// * `base_name`: Option<&str> - The expected base of the node name, how the expected KG nodes may start.
/// * `base_length`: Option<usize> - The expected length of the node name when it includes the base name.
/// * `separator`: Option<&str> - The expected separator that must be included when the node name starts with the base name.
/// * `id_acronym`: Option<&str> - The expected ID the node name must start with AFTER the separator.
/// * `id_length`: Option<usize> - The expected length of the node name after the separator.
/// * `numeric_part_length`: Option<usize> - The expected length of the numeric part at the end of the node name.
fn is_valid_node_name_from_seeds(
    node_name: &str,
    base_name: Option<&str>,
    base_length: Option<usize>,
    separator: Option<&str>,
    id_acronym: Option<&str>,
    id_length: Option<usize>,
    numeric_part_length: Option<usize>,
) -> Result<(), String> {
    if base_name.is_none() && id_acronym.is_none() {
        panic!(concat!(
            "There is some error: both base name and ID acronym are None.",
            "At least one of the two must be not None."
        ));
    }
    let mut validated = false;
    let mut node_name = node_name.to_string();
    // First of all, we check if the node name starts with the base name
    // and the base name was provided.
    if let Some(base_name) = base_name {
        if node_name.starts_with(base_name) {
            // If so, it must have the provided base length.
            if base_length
                .as_ref()
                .map_or(false, |len| node_name.len() != *len)
            {
                return Err(format!(
                    "The given node name {} does not respect the given base length {:?}.",
                    node_name, base_length
                ));
            }
            // We remove the base name from the given node name
            node_name = node_name[base_name.len()..node_name.len()].to_string();
            // The provided separator must exist in the given node name
            if let Some(separator) = separator {
                if node_name.contains(separator) {
                    node_name = node_name
                        .split(separator)
                        .collect::<Vec<_>>()
                        .last()
                        .unwrap()
                        .to_string();
                } else {
                    return Err(format!(
                        "The given node name {} does not contain the given separator {}.",
                        node_name, separator
                    ));
                }
            }

            // Finally, we check that the final `numeric_part_length` values must
            // be numeric digits, if this value was provided.
            if let Some(numeric_part_length) = numeric_part_length {
                if node_name[(node_name.len() - numeric_part_length)..node_name.len()]
                    .chars()
                    .any(|character| !character.is_ascii_digit())
                {
                    return Err(format!(
                        "The last {} characters of the node name {} are expected to be digits.",
                        numeric_part_length, node_name,
                    ));
                }
            }

            validated = true;
        }
    }

    // If an ID acronym was provided, we expect the node to respect it.
    if let Some(id_acronym) = id_acronym {
        if !node_name.starts_with(id_acronym) {
            return Err(format!(
                "The given node name {} does not start with the expected ID acronym {}.",
                node_name, id_acronym
            ));
        }
        // If it starts with the provided acronym, it must have the provided
        // acronym length.
        if id_length
            .as_ref()
            .map_or(false, |len| node_name.len() != *len)
        {
            return Err(format!(
                "The given node name {} starts with the given acronym {} but does not have the expected length {:?}.",
                node_name,
                id_acronym,
                id_length
            ));
        }
        // Finally, we check that the final `numeric_part_length` values must
        // be numeric digits, if this value was provided.
        if let Some(numeric_part_length) = numeric_part_length {
            if node_name[(node_name.len() - numeric_part_length)..node_name.len()]
                .chars()
                .any(|character| !character.is_ascii_digit())
            {
                return Err(format!(
                    "The last {} characters of the node name {} are expected to be digits.",
                    numeric_part_length, node_name,
                ));
            }
        }
        validated = true;
    }

    if validated {
        Ok(())
    } else {
        Err(format!(
            "The given node name {node_name} was not validated successfully.",
            node_name = node_name
        ))
    }
}

/// Returns formatted url.
///
/// # Arguments
/// * `url_pattern`: &str - The URL pattern.
/// * `node_name`: &str - The node name.
/// * `separator`: Option<&str> - The expected separator.
fn format_url_from_node_name(
    url_pattern: &str,
    node_name: &str,
    separator: Option<&str>,
) -> String {
    url_pattern.replace(
        "{node_name}",
        separator.map_or(node_name, |sep| {
            if node_name.contains(sep) {
                node_name.split(sep).collect::<Vec<_>>().last().unwrap()
            } else {
                node_name
            }
        }),
    )
}

/// Returns whether the given node name respects the flybase nodes pattern.
///
/// # Arguments
/// * `node_name`: &str - Node name to check pattern with.
///
/// # Example
/// To validate a node you can use:
/// ```rust
/// # use graph::*;
/// let flybase_node_name = "FlyBase:FBgn0000003";
/// let not_flybase_node_name = "PizzaQuattroStagioni";
/// assert!(is_valid_flybase_node_name(flybase_node_name));
/// assert!(!is_valid_flybase_node_name(not_flybase_node_name));
/// ```
pub fn is_valid_flybase_node_name(node_name: &str) -> bool {
    is_valid_node_name_from_seeds(
        node_name,
        Some("FlyBase"),
        Some(19),
        Some(":"),
        Some("FB"),
        Some(11),
        Some(7),
    )
    .is_ok()
}

/// Returns URL from given flybase node name.
///
/// # Arguments
/// * `node_name`: &str - Node name to check pattern with.
///
/// # Safety
/// This method assumes that the provided node name is a flybase node name and
/// may cause a panic if the aforementioned assumption is not true.
unsafe fn format_flybase_url_from_node_name(node_name: &str) -> String {
    format_url_from_node_name(
        "https://flybase.org/reports/{node_name}",
        node_name,
        Some(":"),
    )
}

/// Returns whether the given node name respects the sequence ontology nodes pattern.
///
/// # Arguments
/// * `node_name`: &str - Node name to check pattern with.
///
/// # Example
/// To validate a node you can use:
/// ```rust
/// # use graph::*;
/// let sequence_ontology_node_name = "SO:0001217";
/// let not_sequence_ontology_node_name = "PizzaQuattroStagioni";
/// assert!(is_valid_sequence_ontology_node_name(sequence_ontology_node_name));
/// assert!(!is_valid_sequence_ontology_node_name(not_sequence_ontology_node_name));
/// ```
pub fn is_valid_sequence_ontology_node_name(node_name: &str) -> bool {
    is_valid_node_name_from_seeds(
        node_name,
        Some("SO"),
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
unsafe fn format_sequence_ontology_url_from_node_name(node_name: &str) -> String {
    format_url_from_node_name(
        "http://www.sequenceontology.org/browser/current_svn/term/{node_name}",
        node_name,
        None,
    )
}

/// Returns whether the given node name respects the mouse genome informatics nodes pattern.
///
/// # Arguments
/// * `node_name`: &str - Node name to check pattern with.
///
/// # Example
/// To validate a node you can use:
/// ```rust
/// # use graph::*;
/// let mouse_genome_informatics_node_name = "MGI:2159965";
/// let not_mouse_genome_informatics_node_name = "PizzaQuattroStagioni";
/// assert!(is_valid_mouse_genome_informatics_node_name(mouse_genome_informatics_node_name));
/// assert!(!is_valid_mouse_genome_informatics_node_name(not_mouse_genome_informatics_node_name));
/// ```
pub fn is_valid_mouse_genome_informatics_node_name(node_name: &str) -> bool {
    is_valid_node_name_from_seeds(node_name, Some("MGI"), None, Some(":"), None, None, None).is_ok()
}

/// Returns URL from given mouse genome informatics node name.
///
/// # Arguments
/// * `node_name`: &str - Node name to check pattern with.
///
/// # Safety
/// This method assumes that the provided node name is a mouse genome informatics node name and
/// may cause a panic if the aforementioned assumption is not true.
unsafe fn format_mouse_genome_informatics_url_from_node_name(node_name: &str) -> String {
    format_url_from_node_name(
        "http://www.informatics.jax.org/reference/strain/{node_name}",
        node_name,
        None,
    )
}

/// Returns whether the given node name respects the Pubmed NCBI nodes pattern.
///
/// # Arguments
/// * `node_name`: &str - Node name to check pattern with.
///
/// # Example
/// To validate a node you can use:
/// ```rust
/// # use graph::*;
/// let pubmed_ncbi_node_name = "PMID:1001879";
/// let not_pubmed_ncbi_node_name = "PizzaQuattroStagioni";
/// assert!(is_valid_pubmed_ncbi_node_name(pubmed_ncbi_node_name));
/// assert!(!is_valid_pubmed_ncbi_node_name(not_pubmed_ncbi_node_name));
/// ```
pub fn is_valid_pubmed_ncbi_node_name(node_name: &str) -> bool {
    is_valid_node_name_from_seeds(node_name, Some("PMID"), None, Some(":"), None, None, None)
        .is_ok()
}

/// Returns URL from given Pubmed NCBI node name.
///
/// # Arguments
/// * `node_name`: &str - Node name to check pattern with.
///
/// # Safety
/// This method assumes that the provided node name is a Pubmed NCBI node name and
/// may cause a panic if the aforementioned assumption is not true.
unsafe fn format_pubmed_ncbi_url_from_node_name(node_name: &str) -> String {
    format_url_from_node_name(
        "https://pubmed.ncbi.nlm.nih.gov/{node_name}/",
        node_name,
        Some(":"),
    )
}

/// Returns whether the given node name respects the NCBI Genes nodes pattern.
///
/// # Arguments
/// * `node_name`: &str - Node name to check pattern with.
///
/// # Example
/// To validate a node you can use:
/// ```rust
/// # use graph::*;
/// let ncbi_gene_node_name1 = "NCBIGene:100000024";
/// let ncbi_gene_node_name2 = "NCBIGene:562690";
/// let not_ncbi_gene_node_name = "PizzaQuattroStagioni";
/// assert!(is_valid_ncbi_gene_node_name(ncbi_gene_node_name1));
/// assert!(is_valid_ncbi_gene_node_name(ncbi_gene_node_name2));
/// assert!(!is_valid_ncbi_gene_node_name(not_ncbi_gene_node_name));
/// ```
pub fn is_valid_ncbi_gene_node_name(node_name: &str) -> bool {
    is_valid_node_name_from_seeds(
        node_name,
        Some("NCBIGene"),
        None,
        Some(":"),
        None,
        None,
        None,
    )
    .is_ok()
}

/// Returns URL from given NCBI Genes node name.
///
/// # Arguments
/// * `node_name`: &str - Node name to check pattern with.
///
/// # Safety
/// This method assumes that the provided node name is a NCBI Genes node name and
/// may cause a panic if the aforementioned assumption is not true.
unsafe fn format_ncbi_gene_url_from_node_name(node_name: &str) -> String {
    format_url_from_node_name(
        "https://www.ncbi.nlm.nih.gov/gene/{node_name}",
        node_name,
        Some(":"),
    )
}

/// Returns whether the given node name respects the NCBI clinvars nodes pattern.
///
/// # Arguments
/// * `node_name`: &str - Node name to check pattern with.
///
/// # Example
/// To validate a node you can use:
/// ```rust
/// # use graph::*;
/// let ncbi_clinvar_node_name1 = "ClinVarVariant:10003";
/// let ncbi_clinvar_node_name2 = "ClinVarVariant:100059";
/// let not_ncbi_clinvar_node_name = "PizzaQuattroStagioni";
/// assert!(is_valid_ncbi_clinvar_node_name(ncbi_clinvar_node_name1));
/// assert!(is_valid_ncbi_clinvar_node_name(ncbi_clinvar_node_name2));
/// assert!(!is_valid_ncbi_clinvar_node_name(not_ncbi_clinvar_node_name));
/// ```
pub fn is_valid_ncbi_clinvar_node_name(node_name: &str) -> bool {
    is_valid_node_name_from_seeds(
        node_name,
        Some("ClinVarVariant"),
        None,
        Some(":"),
        None,
        None,
        None,
    )
    .is_ok()
}

/// Returns URL from given NCBI clinvars node name.
///
/// # Arguments
/// * `node_name`: &str - Node name to check pattern with.
///
/// # Safety
/// This method assumes that the provided node name is a NCBI clinvars node name and
/// may cause a panic if the aforementioned assumption is not true.
unsafe fn format_ncbi_clinvar_url_from_node_name(node_name: &str) -> String {
    format_url_from_node_name(
        "https://www.ncbi.nlm.nih.gov/clinvar/variation/{node_name}/",
        node_name,
        Some(":"),
    )
}

/// Returns whether the given node name respects the WormBase nodes pattern.
///
/// # Arguments
/// * `node_name`: &str - Node name to check pattern with.
///
/// # Example
/// To validate a node you can use:
/// ```rust
/// # use graph::*;
/// let wormbase_node_name1 = "WormBase:WBGene00195045";
/// let wormbase_node_name2 = "WBGene00195045";
/// let not_wormbase_node_name = "PizzaQuattroStagioni";
/// assert!(is_valid_wormbase_node_name(wormbase_node_name1));
/// assert!(is_valid_wormbase_node_name(wormbase_node_name2));
/// assert!(!is_valid_wormbase_node_name(not_wormbase_node_name));
/// ```
pub fn is_valid_wormbase_node_name(node_name: &str) -> bool {
    is_valid_node_name_from_seeds(
        node_name,
        Some("WormBase"),
        Some(23),
        Some(":"),
        Some("WBGene"),
        Some(14),
        Some(8),
    )
    .is_ok()
}

/// Returns URL from given WormBase node name.
///
/// # Arguments
/// * `node_name`: &str - Node name to check pattern with.
///
/// # Safety
/// This method assumes that the provided node name is a WormBase node name and
/// may cause a panic if the aforementioned assumption is not true.
unsafe fn format_wormbase_url_from_node_name(node_name: &str) -> String {
    format_url_from_node_name(
        "https://wormbase.org/search/all/{node_name}",
        node_name,
        Some(":"),
    )
}

/// Returns whether the given node name respects the ensembl nodes pattern.
///
/// # Arguments
/// * `node_name`: &str - Node name to check pattern with.
///
/// # Example
/// To validate a node you can use:
/// ```rust
/// # use graph::*;
/// let ensembl_node_name1 = "ENSEMBL:ENSACAG00000000017";
/// let ensembl_node_name2 = "ENSACAG00000000017";
/// let not_ensembl_node_name = "PizzaQuattroStagioni";
/// assert!(is_valid_ensembl_node_name(ensembl_node_name1));
/// assert!(is_valid_ensembl_node_name(ensembl_node_name2));
/// assert!(!is_valid_ensembl_node_name(not_ensembl_node_name));
/// ```
pub fn is_valid_ensembl_node_name(node_name: &str) -> bool {
    is_valid_node_name_from_seeds(
        node_name,
        Some("ENSEMBL"),
        Some(26),
        Some(":"),
        Some("ENS"),
        Some(18),
        Some(11),
    )
    .is_ok()
}

/// Returns URL from given ensembl node name.
///
/// # Arguments
/// * `node_name`: &str - Node name to check pattern with.
///
/// # Safety
/// This method assumes that the provided node name is a ensembl node name and
/// may cause a panic if the aforementioned assumption is not true.
unsafe fn format_ensembl_url_from_node_name(node_name: &str) -> String {
    format_url_from_node_name(
        "https://www.ensembl.org/Gene/Summary?g={node_name}",
        node_name,
        Some(":"),
    )
}

/// Returns whether the given node name respects the zfin nodes pattern.
///
/// # Arguments
/// * `node_name`: &str - Node name to check pattern with.
///
/// # Example
/// To validate a node you can use:
/// ```rust
/// # use graph::*;
/// let zfin_node_name1 = "ZFIN:ZDB-GENE-130530-778";
/// let zfin_node_name2 = "ZDB-GENE-101108-4";
/// let not_zfin_node_name = "PizzaQuattroStagioni";
/// assert!(is_valid_zfin_node_name(zfin_node_name1));
/// assert!(is_valid_zfin_node_name(zfin_node_name2));
/// assert!(!is_valid_zfin_node_name(not_zfin_node_name));
/// ```
pub fn is_valid_zfin_node_name(node_name: &str) -> bool {
    is_valid_node_name_from_seeds(
        node_name,
        Some("ZFIN"),
        None,
        Some(":"),
        Some("ZDB"),
        None,
        None,
    )
    .is_ok()
}

/// Returns URL from given zfin node name.
///
/// # Arguments
/// * `node_name`: &str - Node name to check pattern with.
///
/// # Safety
/// This method assumes that the provided node name is a zfin node name and
/// may cause a panic if the aforementioned assumption is not true.
unsafe fn format_zfin_url_from_node_name(node_name: &str) -> String {
    format_url_from_node_name("http://zfin.org/{node_name}", node_name, Some(":"))
}

/// Returns whether the given node name respects the doi nodes pattern.
///
/// # Arguments
/// * `node_name`: &str - Node name to check pattern with.
///
/// # Example
/// To validate a node you can use:
/// ```rust
/// # use graph::*;
/// let doi_node_name1 = "DOI:10.1002/1873-3468.12198";
/// let doi_node_name2 = "DOI:000337984";
/// let not_doi_node_name = "PizzaQuattroStagioni";
/// assert!(is_valid_doi_node_name(doi_node_name1));
/// assert!(is_valid_doi_node_name(doi_node_name2));
/// assert!(!is_valid_doi_node_name(not_doi_node_name));
/// ```
pub fn is_valid_doi_node_name(node_name: &str) -> bool {
    is_valid_node_name_from_seeds(node_name, Some("DOI"), None, Some(":"), None, None, None).is_ok()
}

/// Returns URL from given doi node name.
///
/// # Arguments
/// * `node_name`: &str - Node name to check pattern with.
///
/// # Safety
/// This method assumes that the provided node name is a doi node name and
/// may cause a panic if the aforementioned assumption is not true.
unsafe fn format_doi_url_from_node_name(node_name: &str) -> String {
    format_url_from_node_name("http://doi.org/{node_name}", node_name, Some(":"))
}

/// Returns whether the given node name respects the chebi nodes pattern.
///
/// # Arguments
/// * `node_name`: &str - Node name to check pattern with.
///
/// # Example
/// To validate a node you can use:
/// ```rust
/// # use graph::*;
/// let chebi_node_name1 = "CHEBI:145556";
/// let chebi_node_name2 = "CHEBI:85302";
/// let not_chebi_node_name = "PizzaQuattroStagioni";
/// assert!(is_valid_chebi_node_name(chebi_node_name1));
/// assert!(is_valid_chebi_node_name(chebi_node_name2));
/// assert!(!is_valid_chebi_node_name(not_chebi_node_name));
/// ```
pub fn is_valid_chebi_node_name(node_name: &str) -> bool {
    is_valid_node_name_from_seeds(node_name, Some("CHEBI"), None, Some(":"), None, None, None)
        .is_ok()
}

/// Returns URL from given chebi node name.
///
/// # Arguments
/// * `node_name`: &str - Node name to check pattern with.
///
/// # Safety
/// This method assumes that the provided node name is a chebi node name and
/// may cause a panic if the aforementioned assumption is not true.
unsafe fn format_chebi_url_from_node_name(node_name: &str) -> String {
    format_url_from_node_name(
        "https://www.ebi.ac.uk/chebi/searchId.do?chebiId={node_name}",
        node_name,
        None,
    )
}

/// Returns whether the given node name respects the biogrid nodes pattern.
///
/// # Arguments
/// * `node_name`: &str - Node name to check pattern with.
///
/// # Example
/// To validate a node you can use:
/// ```rust
/// # use graph::*;
/// let biogrid_node_name = "BIOGRID:106534";
/// let not_biogrid_node_name = "PizzaQuattroStagioni";
/// assert!(is_valid_biogrid_node_name(biogrid_node_name));
/// assert!(!is_valid_biogrid_node_name(not_biogrid_node_name));
/// ```
pub fn is_valid_biogrid_node_name(node_name: &str) -> bool {
    is_valid_node_name_from_seeds(
        node_name,
        Some("BIOGRID"),
        None,
        Some(":"),
        None,
        None,
        None,
    )
    .is_ok()
}

/// Returns URL from given biogrid node name.
///
/// # Arguments
/// * `node_name`: &str - Node name to check pattern with.
///
/// # Safety
/// This method assumes that the provided node name is a biogrid node name and
/// may cause a panic if the aforementioned assumption is not true.
unsafe fn format_biogrid_url_from_node_name(node_name: &str) -> String {
    format_url_from_node_name("https://thebiogrid.org/{node_name}", node_name, Some(":"))
}

/// Returns whether the given node name respects the omim nodes pattern.
///
/// # Arguments
/// * `node_name`: &str - Node name to check pattern with.
///
/// # Example
/// To validate a node you can use:
/// ```rust
/// # use graph::*;
/// let omim_node_name = "OMIM:611636";
/// let not_omim_node_name = "PizzaQuattroStagioni";
/// assert!(is_valid_omim_node_name(omim_node_name));
/// assert!(!is_valid_omim_node_name(not_omim_node_name));
/// ```
pub fn is_valid_omim_node_name(node_name: &str) -> bool {
    is_valid_node_name_from_seeds(
        node_name,
        Some("OMIM"),
        Some(11),
        Some(":"),
        None,
        None,
        Some(6),
    )
    .is_ok()
}

/// Returns URL from given omim node name.
///
/// # Arguments
/// * `node_name`: &str - Node name to check pattern with.
///
/// # Safety
/// This method assumes that the provided node name is a omim node name and
/// may cause a panic if the aforementioned assumption is not true.
unsafe fn format_omim_url_from_node_name(node_name: &str) -> String {
    format_url_from_node_name(
        "https://www.omim.org/entry/{node_name}",
        node_name,
        Some(":"),
    )
}

/// Returns whether the given node name respects the rat_genome_database nodes pattern.
///
/// # Arguments
/// * `node_name`: &str - Node name to check pattern with.
///
/// # Example
/// To validate a node you can use:
/// ```rust
/// # use graph::*;
/// let rat_genome_database_node_name = "RGD:10059728";
/// let not_rat_genome_database_node_name = "PizzaQuattroStagioni";
/// assert!(is_valid_rat_genome_database_node_name(rat_genome_database_node_name));
/// assert!(!is_valid_rat_genome_database_node_name(not_rat_genome_database_node_name));
/// ```
pub fn is_valid_rat_genome_database_node_name(node_name: &str) -> bool {
    is_valid_node_name_from_seeds(
        node_name,
        Some("RGD"),
        Some(12),
        Some(":"),
        None,
        None,
        Some(8),
    )
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
unsafe fn format_rat_genome_database_url_from_node_name(node_name: &str) -> String {
    format_url_from_node_name(
        "https://rgd.mcw.edu/rgdweb/report/gene/main.html?id={node_name}",
        node_name,
        Some(":"),
    )
}

/// Returns whether the given node name respects the Mutant Mouse Resource & Research Center nodes pattern.
///
/// # Arguments
/// * `node_name`: &str - Node name to check pattern with.
///
/// # Example
/// To validate a node you can use:
/// ```rust
/// # use graph::*;
/// let mmrrc_node_name = "MMRRC:000123";
/// let not_mmrrc_node_name = "PizzaQuattroStagioni";
/// assert!(is_valid_mmrrc_node_name(mmrrc_node_name));
/// assert!(!is_valid_mmrrc_node_name(not_mmrrc_node_name));
/// ```
pub fn is_valid_mmrrc_node_name(node_name: &str) -> bool {
    is_valid_node_name_from_seeds(
        node_name,
        Some("MMRRC"),
        Some(12),
        Some(":"),
        None,
        None,
        Some(6),
    )
    .is_ok()
}

/// Returns URL from given Mutant Mouse Resource & Research Center node name.
///
/// # Arguments
/// * `node_name`: &str - Node name to check pattern with.
///
/// # Safety
/// This method assumes that the provided node name is a Mutant Mouse Resource & Research Center node name and
/// may cause a panic if the aforementioned assumption is not true.
unsafe fn format_mmrrc_url_from_node_name(node_name: &str) -> String {
    format_url_from_node_name(
        "https://www.mmrrc.org/catalog/sds.php?mmrrc_id={node_name}",
        node_name,
        Some(":"),
    )
}

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
unsafe fn format_gene_ontology_url_from_node_name(node_name: &str) -> String {
    format_url_from_node_name(
        "http://amigo.geneontology.org/amigo/term/{node_name}",
        node_name,
        None,
    )
}

/// Returns whether the given node name respects the UNIPROT nodes pattern.
///
/// # Arguments
/// * `node_name`: &str - Node name to check pattern with.
///
/// # Example
/// To validate a node you can use:
/// ```rust
/// # use graph::*;
/// let uniprotkb_node_name1 = "UniProtKB:Q63ZW2";
/// let uniprotkb_node_name2 = "UniProtKB:A0A2R9YJI3";
/// let not_uniprotkb_node_name = "PizzaQuattroStagioni";
/// assert!(is_valid_uniprotkb_node_name(uniprotkb_node_name1));
/// assert!(is_valid_uniprotkb_node_name(uniprotkb_node_name2));
/// assert!(!is_valid_uniprotkb_node_name(not_uniprotkb_node_name));
/// ```
pub fn is_valid_uniprotkb_node_name(node_name: &str) -> bool {
    is_valid_node_name_from_seeds(
        node_name,
        Some("UniProtKB"),
        None,
        Some(":"),
        None,
        None,
        None,
    )
    .is_ok()
}

/// Returns URL from given UniProtKB node name.
///
/// # Arguments
/// * `node_name`: &str - Node name to check pattern with.
///
/// # Safety
/// This method assumes that the provided node name is a UniProtKB node name and
/// may cause a panic if the aforementioned assumption is not true.
unsafe fn format_uniprotkb_url_from_node_name(node_name: &str) -> String {
    format_url_from_node_name(
        "https://www.uniprot.org/uniprot/{node_name}",
        node_name,
        Some(":"),
    )
}

/// Returns whether the given node name respects the Coriell nodes pattern.
///
/// # Arguments
/// * `node_name`: &str - Node name to check pattern with.
///
/// # Example
/// To validate a node you can use:
/// ```rust
/// # use graph::*;
/// let coriell_node_name = "Coriell:AG01439";
/// let not_coriell_node_name = "PizzaQuattroStagioni";
/// assert!(is_valid_coriell_node_name(coriell_node_name));
/// assert!(!is_valid_coriell_node_name(not_coriell_node_name));
/// ```
pub fn is_valid_coriell_node_name(node_name: &str) -> bool {
    is_valid_node_name_from_seeds(
        node_name,
        Some("Coriell"),
        Some(15),
        Some(":"),
        None,
        None,
        None,
    )
    .is_ok()
}

/// Returns URL from given Coriell node name.
///
/// # Arguments
/// * `node_name`: &str - Node name to check pattern with.
///
/// # Safety
/// This method assumes that the provided node name is a Coriell node name and
/// may cause a panic if the aforementioned assumption is not true.
unsafe fn format_coriell_url_from_node_name(node_name: &str) -> String {
    format_url_from_node_name(
        "https://coriell.org/0/Sections/Search/Sample_Detail.aspx?Ref={node_name}",
        node_name,
        Some(":"),
    )
}

/// Returns whether the given node name respects the drugcentral nodes pattern.
///
/// # Arguments
/// * `node_name`: &str - Node name to check pattern with.
///
/// # Example
/// To validate a node you can use:
/// ```rust
/// # use graph::*;
/// let drugcentral_node_name = "DrugCentral:217";
/// let not_drugcentral_node_name = "PizzaQuattroStagioni";
/// assert!(is_valid_drugcentral_node_name(drugcentral_node_name));
/// assert!(!is_valid_drugcentral_node_name(not_drugcentral_node_name));
/// ```
pub fn is_valid_drugcentral_node_name(node_name: &str) -> bool {
    is_valid_node_name_from_seeds(
        node_name,
        Some("DrugCentral"),
        None,
        Some(":"),
        None,
        None,
        None,
    )
    .is_ok()
}

/// Returns URL from given DrugCentral node name.
///
/// # Arguments
/// * `node_name`: &str - Node name to check pattern with.
///
/// # Safety
/// This method assumes that the provided node name is a DrugCentral node name and
/// may cause a panic if the aforementioned assumption is not true.
unsafe fn format_drugcentral_url_from_node_name(node_name: &str) -> String {
    format_url_from_node_name(
        "https://drugcentral.org/drugcard/{node_name}",
        node_name,
        Some(":"),
    )
}

/// Returns whether the given node name respects the NCBI MESH nodes pattern.
///
/// # Arguments
/// * `node_name`: &str - Node name to check pattern with.
///
/// # Example
/// To validate a node you can use:
/// ```rust
/// # use graph::*;
/// let ncbi_mesh_node_name = "MESH:217";
/// let not_ncbi_mesh_node_name = "PizzaQuattroStagioni";
/// assert!(is_valid_ncbi_mesh_node_name(ncbi_mesh_node_name));
/// assert!(!is_valid_ncbi_mesh_node_name(not_ncbi_mesh_node_name));
/// ```
pub fn is_valid_ncbi_mesh_node_name(node_name: &str) -> bool {
    is_valid_node_name_from_seeds(node_name, Some("MESH"), None, Some(":"), None, None, None)
        .is_ok()
}

/// Returns URL from given NCBI MESH node name.
///
/// # Arguments
/// * `node_name`: &str - Node name to check pattern with.
///
/// # Safety
/// This method assumes that the provided node name is a NCBI MESH node name and
/// may cause a panic if the aforementioned assumption is not true.
unsafe fn format_ncbi_mesh_url_from_node_name(node_name: &str) -> String {
    format_url_from_node_name(
        "ncbi.nlm.nih.gov/mesh/?term={node_name}",
        node_name,
        Some(":"),
    )
}

/// Returns whether the given node name respects the NCBI taxonomy nodes pattern.
///
/// # Arguments
/// * `node_name`: &str - Node name to check pattern with.
///
/// # Example
/// To validate a node you can use:
/// ```rust
/// # use graph::*;
/// let ncbi_taxonomy_node_name = "NCBITaxon:264379";
/// let not_ncbi_taxonomy_node_name = "PizzaQuattroStagioni";
/// assert!(is_valid_ncbi_taxonomy_node_name(ncbi_taxonomy_node_name));
/// assert!(!is_valid_ncbi_taxonomy_node_name(not_ncbi_taxonomy_node_name));
/// ```
pub fn is_valid_ncbi_taxonomy_node_name(node_name: &str) -> bool {
    is_valid_node_name_from_seeds(
        node_name,
        Some("NCBITaxon"),
        Some(16),
        Some(":"),
        None,
        None,
        Some(6),
    )
    .is_ok()
}

/// Returns URL from given NCBI taxonomy node name.
///
/// # Arguments
/// * `node_name`: &str - Node name to check pattern with.
///
/// # Safety
/// This method assumes that the provided node name is a NCBI taxonomy node name and
/// may cause a panic if the aforementioned assumption is not true.
unsafe fn format_ncbi_taxonomy_url_from_node_name(node_name: &str) -> String {
    format_url_from_node_name(
        "https://www.ncbi.nlm.nih.gov/taxonomy/?term={node_name}",
        node_name,
        Some(":"),
    )
}

/// Returns whether the given node name respects the JAX nodes pattern.
///
/// # Arguments
/// * `node_name`: &str - Node name to check pattern with.
///
/// # Example
/// To validate a node you can use:
/// ```rust
/// # use graph::*;
/// let jax_node_name = "JAX:000046";
/// let not_jax_node_name = "PizzaQuattroStagioni";
/// assert!(is_valid_jax_node_name(jax_node_name));
/// assert!(!is_valid_jax_node_name(not_jax_node_name));
/// ```
pub fn is_valid_jax_node_name(node_name: &str) -> bool {
    is_valid_node_name_from_seeds(
        node_name,
        Some("JAX"),
        Some(10),
        Some(":"),
        None,
        None,
        Some(6),
    )
    .is_ok()
}

/// Returns URL from given JAX node name.
///
/// # Arguments
/// * `node_name`: &str - Node name to check pattern with.
///
/// # Safety
/// This method assumes that the provided node name is a JAX node name and
/// may cause a panic if the aforementioned assumption is not true.
unsafe fn format_jax_url_from_node_name(node_name: &str) -> String {
    format_url_from_node_name(
        "https://www.jax.org/strain/{node_name}",
        node_name,
        Some(":"),
    )
}

/// Returns url describing the given node name if a pattern is known.
///
/// # Implementative details
/// Currently we have support for building the URLs for:
/// * [Sequence Ontology](http://www.sequenceontology.org/)
/// * [FlyBase](http://flybase.org/)
/// * [Mouse Genome Informations](http://www.informatics.jax.org/)
/// * [Pubmed NCBI](https://www.ncbi.nlm.nih.gov/)
/// * [NCBI Gene](https://www.ncbi.nlm.nih.gov/gene/)
/// * [NCBI ClinVar](https://www.ncbi.nlm.nih.gov/clinvar/)
/// * [WormBase](https://wormbase.org)
/// * [Ensembl](https://www.ensembl.org/index.html)
/// * [ZFIN](http://zfin.org/)
/// * [DOI](https://www.doi.org/)
/// * [CHEBI](https://www.ebi.ac.uk/chebi/init.do)
/// * [BioGrid](https://thebiogrid.org/)
/// * [OMIM](https://www.omim.org/)
/// * [Rat Genome DataBase](https://rgd.mcw.edu/rgdweb/homepage/)
/// * [MMRRC](https://www.mmrrc.org/)
/// * [GO](http://amigo.geneontology.org/amigo/landing)
/// * [UniProtKB](https://www.uniprot.org/)
/// * [Coriell](https://coriell.org/)
/// * [DrugCentral](https://drugcentral.org/)
/// * [NCBI MESH](https://www.ncbi.nlm.nih.gov/mesh/)
/// * [NCBI Taxonomy](https://www.ncbi.nlm.nih.gov/taxonomy)
/// * [JAX](https://www.jax.org/strain)
///
/// # Arguments
/// * `node_name`: &str - Node name to query for.
///
/// # Raises
/// * If there is no known url source for the given node name.
pub fn get_node_source_url_from_node_name(node_name: &str) -> Result<String, String> {
    if is_valid_flybase_node_name(node_name) {
        return Ok(unsafe { format_flybase_url_from_node_name(node_name) });
    }
    if is_valid_sequence_ontology_node_name(node_name) {
        return Ok(unsafe { format_sequence_ontology_url_from_node_name(node_name) });
    }
    if is_valid_mmrrc_node_name(node_name) {
        return Ok(unsafe { format_mmrrc_url_from_node_name(node_name) });
    }
    if is_valid_wormbase_node_name(node_name) {
        return Ok(unsafe { format_wormbase_url_from_node_name(node_name) });
    }
    if is_valid_mouse_genome_informatics_node_name(node_name) {
        return Ok(unsafe { format_mouse_genome_informatics_url_from_node_name(node_name) });
    }
    if is_valid_pubmed_ncbi_node_name(node_name) {
        return Ok(unsafe { format_pubmed_ncbi_url_from_node_name(node_name) });
    }
    if is_valid_ncbi_gene_node_name(node_name) {
        return Ok(unsafe { format_ncbi_gene_url_from_node_name(node_name) });
    }
    if is_valid_ncbi_clinvar_node_name(node_name) {
        return Ok(unsafe { format_ncbi_clinvar_url_from_node_name(node_name) });
    }
    if is_valid_ensembl_node_name(node_name) {
        return Ok(unsafe { format_ensembl_url_from_node_name(node_name) });
    }
    if is_valid_zfin_node_name(node_name) {
        return Ok(unsafe { format_zfin_url_from_node_name(node_name) });
    }
    if is_valid_doi_node_name(node_name) {
        return Ok(unsafe { format_doi_url_from_node_name(node_name) });
    }
    if is_valid_chebi_node_name(node_name) {
        return Ok(unsafe { format_chebi_url_from_node_name(node_name) });
    }
    if is_valid_biogrid_node_name(node_name) {
        return Ok(unsafe { format_biogrid_url_from_node_name(node_name) });
    }
    if is_valid_omim_node_name(node_name) {
        return Ok(unsafe { format_omim_url_from_node_name(node_name) });
    }
    if is_valid_rat_genome_database_node_name(node_name) {
        return Ok(unsafe { format_rat_genome_database_url_from_node_name(node_name) });
    }
    if is_valid_gene_ontology_node_name(node_name) {
        return Ok(unsafe { format_gene_ontology_url_from_node_name(node_name) });
    }
    if is_valid_uniprotkb_node_name(node_name) {
        return Ok(unsafe { format_uniprotkb_url_from_node_name(node_name) });
    }
    if is_valid_coriell_node_name(node_name) {
        return Ok(unsafe { format_coriell_url_from_node_name(node_name) });
    }
    if is_valid_drugcentral_node_name(node_name) {
        return Ok(unsafe { format_drugcentral_url_from_node_name(node_name) });
    }
    if is_valid_ncbi_mesh_node_name(node_name) {
        return Ok(unsafe { format_ncbi_mesh_url_from_node_name(node_name) });
    }
    if is_valid_ncbi_taxonomy_node_name(node_name) {
        return Ok(unsafe { format_ncbi_taxonomy_url_from_node_name(node_name) });
    }
    if is_valid_jax_node_name(node_name) {
        return Ok(unsafe { format_jax_url_from_node_name(node_name) });
    }
    Err(format!(
        concat!(
            "There is no known url with a pattern for the provided node name {:?}.\n",
            "If you believe there should be one, please do open a pull request to ",
            "add it to the library!"
        ),
        node_name
    ))
}

/// Returns name of the graph repository from the given node name.
///
/// # Implementative details
/// Currently we have support for building the URLs for:
/// * [Sequence Ontology](http://www.sequenceontology.org/)
/// * [FlyBase](http://flybase.org/)
/// * [Mouse Genome Informations](http://www.informatics.jax.org/)
/// * [Pubmed NCBI](https://www.ncbi.nlm.nih.gov/)
/// * [NCBI Gene](https://www.ncbi.nlm.nih.gov/gene/)
/// * [NCBI ClinVar](https://www.ncbi.nlm.nih.gov/clinvar/)
/// * [WormBase](https://wormbase.org)
/// * [Ensembl](https://www.ensembl.org/index.html)
/// * [ZFIN](http://zfin.org/)
/// * [DOI](https://www.doi.org/)
/// * [CHEBI](https://www.ebi.ac.uk/chebi/init.do)
/// * [BioGrid](https://thebiogrid.org/)
/// * [OMIM](https://www.omim.org/)
/// * [Rat Genome DataBase](https://rgd.mcw.edu/rgdweb/homepage/)
/// * [MMRRC](https://www.mmrrc.org/)
/// * [GO](http://amigo.geneontology.org/amigo/landing)
/// * [UniProtKB](https://www.uniprot.org/)
/// * [Coriell](https://coriell.org/)
/// * [DrugCentral](https://drugcentral.org/)
/// * [NCBI MESH](https://www.ncbi.nlm.nih.gov/mesh/)
/// * [NCBI Taxonomy](https://www.ncbi.nlm.nih.gov/taxonomy)
/// * [JAX](https://www.jax.org/strain)
///
/// # Arguments
/// * `node_name`: &str - Node name to query for.
///
/// # Raises
/// * If there is no known url source for the given node name.
pub fn get_node_repository_from_node_name(node_name: &str) -> Result<&str, String> {
    if is_valid_flybase_node_name(node_name) {
        return Ok("FlyBase");
    }
    if is_valid_sequence_ontology_node_name(node_name) {
        return Ok("Sequence Ontology");
    }
    if is_valid_mmrrc_node_name(node_name) {
        return Ok("MMRRC");
    }
    if is_valid_wormbase_node_name(node_name) {
        return Ok("WormBase");
    }
    if is_valid_mouse_genome_informatics_node_name(node_name) {
        return Ok("Mouse Genome Informatics");
    }
    if is_valid_pubmed_ncbi_node_name(node_name) {
        return Ok("Pubmed NCBI");
    }
    if is_valid_ncbi_gene_node_name(node_name) {
        return Ok("NCBI Gene");
    }
    if is_valid_ncbi_clinvar_node_name(node_name) {
        return Ok("NCBI ClinVar");
    }
    if is_valid_ensembl_node_name(node_name) {
        return Ok("ENSEMBL");
    }
    if is_valid_zfin_node_name(node_name) {
        return Ok("ZFIN");
    }
    if is_valid_doi_node_name(node_name) {
        return Ok("DOI");
    }
    if is_valid_chebi_node_name(node_name) {
        return Ok("CHEBI");
    }
    if is_valid_biogrid_node_name(node_name) {
        return Ok("BIOGRID");
    }
    if is_valid_omim_node_name(node_name) {
        return Ok("OMIN");
    }
    if is_valid_rat_genome_database_node_name(node_name) {
        return Ok("Rat Genome Database");
    }
    if is_valid_gene_ontology_node_name(node_name) {
        return Ok("Gene Ontology");
    }
    if is_valid_uniprotkb_node_name(node_name) {
        return Ok("UniProtKB");
    }
    if is_valid_coriell_node_name(node_name) {
        return Ok("Coriell");
    }
    if is_valid_drugcentral_node_name(node_name) {
        return Ok("DrugCentral");
    }
    if is_valid_ncbi_mesh_node_name(node_name) {
        return Ok("NCBI Mesh");
    }
    if is_valid_ncbi_taxonomy_node_name(node_name) {
        return Ok("NCBI Taxonomy");
    }
    if is_valid_jax_node_name(node_name) {
        return Ok("JAX");
    }
    Err(format!(
        concat!(
            "There is no known url with a pattern for the provided node name {:?}.\n",
            "If you believe there should be one, please do open a pull request to ",
            "add it to the library!"
        ),
        node_name
    ))
}

/// Returns whether the given node name respects the BioLink nodes pattern.
///
/// # Arguments
/// * `node_name`: &str - Node name to check pattern with.
///
/// # Example
/// To validate a node you can use:
/// ```rust
/// # use graph::*;
/// let biolink_node_name1 = "biolink:BehavioralOutcome";
/// let biolink_node_name2 = "biolink:Book";
/// let not_biolink_node_name = "PizzaQuattroStagioni";
/// assert!(is_valid_biolink_from_object(biolink_node_name1));
/// assert!(is_valid_biolink_from_object(biolink_node_name2));
/// assert!(!is_valid_biolink_from_object(not_biolink_node_name));
/// ```
pub fn is_valid_biolink_from_object(node_name: &str) -> bool {
    is_valid_node_name_from_seeds(
        node_name,
        Some("biolink"),
        None,
        Some(":"),
        None,
        None,
        None,
    )
    .is_ok()
}

/// Returns URL from given JAX node name.
///
/// # Arguments
/// * `object_name`: &str - Object name to build pattern for.
///
/// # Safety
/// This method assumes that the provided node name is a JAX node name and
/// may cause a panic if the aforementioned assumption is not true.
unsafe fn format_biolink_from_object(object_name: &str) -> String {
    format_url_from_node_name(
        "https://biolink.github.io/biolink-model/docs/{node_name}.html",
        object_name,
        Some(":"),
    )
}

/// Returns url describing the given node type if a pattern is known.
///
/// # Implementative details
/// Currently we have support for building the URLs for:
/// * [BioLink](https://biolink.github.io/biolink-model/)
///
/// # Arguments
/// * `node_type_name`: &str - Node name to query for.
///
/// # Raises
/// * If there is no known url source for the given node type.
pub fn get_node_type_source_url_from_node_type_name(
    node_type_name: &str,
) -> Result<String, String> {
    if is_valid_biolink_from_object(node_type_name) {
        return Ok(unsafe { format_biolink_from_object(node_type_name) });
    }
    Err(format!(
        concat!(
            "There is no known url with a pattern for the provided node type {:?}.\n",
            "If you believe there should be one, please do open a pull request to ",
            "add it to the library!"
        ),
        node_type_name
    ))
}

/// Returns url describing the given edge type if a pattern is known.
///
/// # Implementative details
/// Currently we have support for building the URLs for:
/// * [BioLink](https://biolink.github.io/biolink-model/)
///
/// # Arguments
/// * `edge_type_name`: &str - edge name to query for.
///
/// # Raises
/// * If there is no known url source for the given edge type.
pub fn get_edge_type_source_url_from_edge_type_name(
    edge_type_name: &str,
) -> Result<String, String> {
    if is_valid_biolink_from_object(edge_type_name) {
        return Ok(unsafe { format_biolink_from_object(edge_type_name) });
    }
    Err(format!(
        concat!(
            "There is no known url with a pattern for the provided edge type {:?}.\n",
            "If you believe there should be one, please do open a pull request to ",
            "add it to the library!"
        ),
        edge_type_name
    ))
}

/// Returns built url for given element
fn get_url_formatted(url: &str, content: &str, repository: &str) -> String {
    format!(
        "<a href='{url}' target='_blank' title='Go to {repository} to get more informations about {content}'>{content}</a>",
        url = url,
        content = content,
        repository=repository
    )
}

/// Returns html-formatted source of given node name if known.
///
/// # Implementative details
/// If an URL is detected from the provided node name then a standard
/// html URL formatting is returned, otherwise the node name is returned.
/// Refer to the `get_node_source_url_from_node_name` method documentation
/// to see which node databases are supported currently.
///
/// # Arguments
/// * `node_name`: &str - Node name to query for.
pub fn get_node_source_html_url_from_node_name(node_name: &str) -> String {
    match get_node_source_url_from_node_name(node_name) {
        Ok(url) => get_url_formatted(
            url.as_str(),
            node_name,
            get_node_repository_from_node_name(node_name).unwrap(),
        ),
        Err(_) => node_name.to_string(),
    }
}

/// Returns html-formatted source of given node type name if known.
///
/// # Implementative details
/// If an URL is detected from the provided node type name then a standard
/// html URL formatting is returned, otherwise the node type name is returned.
/// Refer to the `get_node_source_url_from_node_type_name` method documentation
/// to see which node databases are supported currently.
///
/// # Arguments
/// * `node_type_name`: &str - Node name to query for.
pub fn get_node_type_source_html_url_from_node_type_name(node_type_name: &str) -> String {
    match get_node_type_source_url_from_node_type_name(node_type_name) {
        Ok(url) => get_url_formatted(url.as_str(), node_type_name, "BioLink"),
        Err(_) => node_type_name.to_string(),
    }
}

/// Returns html-formatted source of given edge type name if known.
///
/// # Implementative details
/// If an URL is detected from the provided edge type name then a standard
/// html URL formatting is returned, otherwise the edge type name is returned.
/// Refer to the `get_edge_source_url_from_edge_type_name` method documentation
/// to see which edge databases are supported currently.
///
/// # Arguments
/// * `edge_type_name`: &str - edge name to query for.
pub fn get_edge_type_source_html_url_from_edge_type_name(edge_type_name: &str) -> String {
    match get_edge_type_source_url_from_edge_type_name(edge_type_name) {
        Ok(url) => get_url_formatted(url.as_str(), edge_type_name, "BioLink"),
        Err(_) => edge_type_name.to_string(),
    }
}
