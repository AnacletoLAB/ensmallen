use super::*;

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
pub(crate) fn is_valid_node_name_from_seeds(
    node_name: &str,
    base_name: Option<&str>,
    base_length: Option<usize>,
    separator: Option<&str>,
    id_acronym: Option<&str>,
    id_length: Option<usize>,
    numeric_part_length: Option<usize>,
) -> Result<()> {
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
                // If the separator was provided, it must appear exactly once
                if node_name.matches(separator).count() == 1 {
                    node_name = node_name
                        .split(separator)
                        .collect::<Vec<_>>()
                        .last()
                        .unwrap()
                        .to_string();
                } else {
                    return Err(format!(
                        "We expect for the given separator {} to appear exactly once in the given object '{}'.",
                        separator,
                        node_name
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
pub(crate) fn format_url_from_node_name(
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

/// Returns built url for given element
fn get_url_formatted(url: &str, content: &str, repository: &str) -> String {
    format!(
        "<a href='{url}' target='_blank' title='Go to {repository} to get more informations about {content}'>{content}</a>",
        url = url,
        content = content,
        repository=repository
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
/// * [WikiData](https://www.wikidata.org/wiki/Wikidata:Main_Page)
/// * [Therapeutic Target Database](http://db.idrblab.net/ttd/)
/// * [Reactome](https://reactome.org/)
///
/// # Arguments
/// * `node_name`: &str - Node name to query for.
///
/// # Raises
/// * If there is no known url source for the given node name.
pub fn get_node_source_url_from_node_name(node_name: &str) -> Result<String> {
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
    if is_valid_therapeutic_target_database_node_name(node_name) {
        return Ok(unsafe { format_therapeutic_target_database_url_from_node_name(node_name) });
    }
    if is_valid_wikidata_node_name(node_name) {
        return Ok(unsafe { format_wikidata_url_from_node_name(node_name) });
    }
    if is_valid_reactome_node_name(node_name) {
        return Ok(unsafe { format_reactome_url_from_node_name(node_name) });
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
/// * [WikiData](https://www.wikidata.org/wiki/Wikidata:Main_Page)
/// * [Therapeutic Target Database](http://db.idrblab.net/ttd/)
/// * [Reactome](https://reactome.org/)
///
/// # Arguments
/// * `node_name`: &str - Node name to query for.
///
/// # Raises
/// * If there is no known url source for the given node name.
pub fn get_node_repository_from_node_name(node_name: &str) -> Result<&str> {
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
    if is_valid_wikidata_node_name(node_name) {
        return Ok("WikiData");
    }
    if is_valid_therapeutic_target_database_node_name(node_name) {
        return Ok("Therapeutic Target Database");
    }
    if is_valid_reactome_node_name(node_name) {
        return Ok("Reactome");
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
pub fn get_node_type_source_url_from_node_type_name(node_type_name: &str) -> Result<String> {
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
pub fn get_edge_type_source_url_from_edge_type_name(edge_type_name: &str) -> Result<String> {
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
