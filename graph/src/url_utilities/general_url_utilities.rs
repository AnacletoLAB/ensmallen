use super::*;

/// Returns whether the given node name is valid according to given seeds.
///
/// # Arguments
/// * `node_name`: &str - The name of the nodes.
/// * `base_names`: Option<&[&str]> - The expected base of the node name, how the expected KG nodes may start.
/// * `base_length`: Option<usize> - The expected length of the node name when it includes the base name.
/// * `separator`: Option<&str> - The expected separator that must be included when the node name starts with the base name.
/// * `id_acronym`: Option<&str> - The expected ID the node name must start with AFTER the separator.
/// * `id_length`: Option<usize> - The expected length of the node name after the separator.
/// * `numeric_part_length`: Option<usize> - The expected length of the numeric part at the end of the node name.
pub(crate) fn is_valid_node_name_from_seeds(
    node_name: &str,
    base_names: Option<&[&str]>,
    base_length: Option<usize>,
    separator: Option<&str>,
    id_acronym: Option<&str>,
    id_length: Option<usize>,
    numeric_part_length: Option<usize>,
) -> Result<()> {
    if (base_names.is_none()
        || base_names
            .as_ref()
            .map_or(false, |base_names| base_names.is_empty()))
        && id_acronym.is_none()
    {
        panic!(concat!(
            "There is some error: both base name and ID acronym are None.",
            "At least one of the two must be not None."
        ));
    }
    let mut validated = false;
    let mut node_name = node_name.to_string();

    // First of all, we check if the node name starts with the base name
    // and the base name was provided.
    if let Some(base_names) = base_names {
        for base_name in base_names.iter() {
            if node_name
                .to_uppercase()
                .starts_with(&base_name.to_string().to_uppercase())
            {
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
                break;
            }
        }
    }

    // If an ID acronym was provided, we expect the node to respect it.
    if let Some(id_acronym) = id_acronym {
        if !node_name.to_uppercase().starts_with(&id_acronym.to_uppercase()) {
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
/// TODO! Update documentation.
/// TODO! Add support for URLs.
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
    if is_valid_wormbase_gene_node_name(node_name) {
        return Ok(unsafe { format_wormbase_gene_url_from_node_name(node_name) });
    }
    if is_valid_mouse_genome_informatics_node_name(node_name) {
        return Ok(unsafe { format_mouse_genome_informatics_url_from_node_name(node_name) });
    }
    if is_valid_ncbi_dbgap_node_name(node_name) {
        return Ok(unsafe { format_ncbi_dbgap_url_from_node_name(node_name) });
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

    if is_valid_jackson_lab_reference_node_name(node_name) {
        return Ok(unsafe { format_jackson_lab_reference_url_from_node_name(node_name) });
    }

    if is_valid_uberon_core_node_name(node_name) {
        return Ok(unsafe { format_uberon_core_url_from_node_name(node_name) });
    }

    if is_valid_gene_ontology_relations_node_name(node_name) {
        return Ok(unsafe { format_gene_ontology_relations_url_from_node_name(node_name) });
    }

    if is_valid_quantity_unit_dimension_and_type_node_name(node_name) {
        return Ok(unsafe {
            format_quantity_unit_dimension_and_type_url_from_node_name(node_name)
        });
    }

    if is_valid_open_biomedical_association_node_name(node_name) {
        return Ok(unsafe { format_open_biomedical_association_url_from_node_name(node_name) });
    }

    if is_valid_kegg_homo_sapiens_node_name(node_name) {
        return Ok(unsafe { format_kegg_homo_sapiens_url_from_node_name(node_name) });
    }

    if is_valid_dublin_core_node_name(node_name) {
        return Ok(unsafe { format_dublin_core_url_from_node_name(node_name) });
    }

    if is_valid_xsd_node_name(node_name) {
        return Ok(unsafe { format_xsd_url_from_node_name(node_name) });
    }

    if is_valid_kegg_ko_node_name(node_name) {
        return Ok(unsafe { format_kegg_ko_url_from_node_name(node_name) });
    }

    if is_valid_swiss_protein_node_name(node_name) {
        return Ok(unsafe { format_swiss_protein_url_from_node_name(node_name) });
    }

    if is_valid_ncbi_books_node_name(node_name) {
        return Ok(unsafe { format_ncbi_books_url_from_node_name(node_name) });
    }

    if is_valid_chembl_compound_node_name(node_name) {
        return Ok(unsafe { format_chembl_compound_url_from_node_name(node_name) });
    }

    if is_valid_hugo_gene_name_consortium_node_name(node_name) {
        return Ok(unsafe { format_hugo_gene_name_consortium_url_from_node_name(node_name) });
    }

    if is_valid_provenance_authoring_and_versioning_node_name(node_name) {
        return Ok(unsafe {
            format_provenance_authoring_and_versioning_url_from_node_name(node_name)
        });
    }

    if is_valid_feature_annotation_location_description_ontology_node_name(node_name) {
        return Ok(unsafe {
            format_feature_annotation_location_description_ontology_url_from_node_name(node_name)
        });
    }

    if is_valid_rbrc_node_name(node_name) {
        return Ok(unsafe { format_rbrc_url_from_node_name(node_name) });
    }

    if is_valid_dictybase_gene_node_name(node_name) {
        return Ok(unsafe { format_dictybase_gene_url_from_node_name(node_name) });
    }

    if is_valid_monarch_initiative_node_name(node_name) {
        return Ok(unsafe { format_monarch_initiative_url_from_node_name(node_name) });
    }

    if is_valid_edam_data_node_name(node_name) {
        return Ok(unsafe { format_edam_data_url_from_node_name(node_name) });
    }

    if is_valid_ncimr_node_name(node_name) {
        return Ok(unsafe { format_ncimr_url_from_node_name(node_name) });
    }

    if is_valid_ecogene_node_name(node_name) {
        return Ok(unsafe { format_ecogene_url_from_node_name(node_name) });
    }

    if is_valid_aspergillus_genomic_data_reference_node_name(node_name) {
        return Ok(unsafe {
            format_aspergillus_genomic_data_reference_url_from_node_name(node_name)
        });
    }

    if is_valid_european_mouse_mutant_archive_node_name(node_name) {
        return Ok(unsafe { format_european_mouse_mutant_archive_url_from_node_name(node_name) });
    }

    if is_valid_translation_of_embl_nucleotide_sequence_database_node_name(node_name) {
        return Ok(unsafe {
            format_translation_of_embl_nucleotide_sequence_database_url_from_node_name(node_name)
        });
    }

    if is_valid_mouse_phenome_database_strain_node_name(node_name) {
        return Ok(unsafe { format_mouse_phenome_database_strain_url_from_node_name(node_name) });
    }

    if is_valid_basic_formal_ontology_node_name(node_name) {
        return Ok(unsafe { format_basic_formal_ontology_url_from_node_name(node_name) });
    }

    if is_valid_dictybase_genes_node_name(node_name) {
        return Ok(unsafe { format_dictybase_genes_url_from_node_name(node_name) });
    }

    if is_valid_cell_line_ontology_node_name(node_name) {
        return Ok(unsafe { format_cell_line_ontology_url_from_node_name(node_name) });
    }

    if is_valid_animal_genome_horse_qtl_node_name(node_name) {
        return Ok(unsafe { format_animal_genome_horse_qtl_url_from_node_name(node_name) });
    }

    if is_valid_wormbase_node_name(node_name) {
        return Ok(unsafe { format_wormbase_url_from_node_name(node_name) });
    }

    if is_valid_animal_qtl_traits_node_name(node_name) {
        return Ok(unsafe { format_animal_qtl_traits_url_from_node_name(node_name) });
    }

    if is_valid_friend_of_a_friend_node_name(node_name) {
        return Ok(unsafe { format_friend_of_a_friend_url_from_node_name(node_name) });
    }

    if is_valid_omim_phenotypic_series_node_name(node_name) {
        return Ok(unsafe { format_omim_phenotypic_series_url_from_node_name(node_name) });
    }

    if is_valid_owl_node_name(node_name) {
        return Ok(unsafe { format_owl_url_from_node_name(node_name) });
    }

    if is_valid_arabidopsis_tair_node_name(node_name) {
        return Ok(unsafe { format_arabidopsis_tair_url_from_node_name(node_name) });
    }

    if is_valid_unified_medical_language_system_node_name(node_name) {
        return Ok(unsafe { format_unified_medical_language_system_url_from_node_name(node_name) });
    }

    if is_valid_gene_ontology_reference_node_name(node_name) {
        return Ok(unsafe { format_gene_ontology_reference_url_from_node_name(node_name) });
    }

    if is_valid_ncbi_clinical_variants_submitters_node_name(node_name) {
        return Ok(unsafe {
            format_ncbi_clinical_variants_submitters_url_from_node_name(node_name)
        });
    }

    if is_valid_gdc_project_node_name(node_name) {
        return Ok(unsafe { format_gdc_project_url_from_node_name(node_name) });
    }

    if is_valid_complex_portal_node_name(node_name) {
        return Ok(unsafe { format_complex_portal_url_from_node_name(node_name) });
    }

    if is_valid_ucsc_golden_path_node_name(node_name) {
        return Ok(unsafe { format_ucsc_golden_path_url_from_node_name(node_name) });
    }

    if is_valid_yeast_genome_reference_node_name(node_name) {
        return Ok(unsafe { format_yeast_genome_reference_url_from_node_name(node_name) });
    }

    if is_valid_xenbase_node_name(node_name) {
        return Ok(unsafe { format_xenbase_url_from_node_name(node_name) });
    }

    if is_valid_chembl_target_node_name(node_name) {
        return Ok(unsafe { format_chembl_target_url_from_node_name(node_name) });
    }

    if is_valid_coriell_collection_node_name(node_name) {
        return Ok(unsafe { format_coriell_collection_url_from_node_name(node_name) });
    }

    if is_valid_gdc_annotation_node_name(node_name) {
        return Ok(unsafe { format_gdc_annotation_url_from_node_name(node_name) });
    }

    if is_valid_animal_genome_chicken_qtl_node_name(node_name) {
        return Ok(unsafe { format_animal_genome_chicken_qtl_url_from_node_name(node_name) });
    }

    if is_valid_semantic_medline_database_node_name(node_name) {
        return Ok(unsafe { format_semantic_medline_database_url_from_node_name(node_name) });
    }

    if is_valid_animal_genome_rainbow_trout_qtl_node_name(node_name) {
        return Ok(unsafe { format_animal_genome_rainbow_trout_qtl_url_from_node_name(node_name) });
    }

    if is_valid_animal_genome_pig_qtl_node_name(node_name) {
        return Ok(unsafe { format_animal_genome_pig_qtl_url_from_node_name(node_name) });
    }

    if is_valid_livestock_product_trait_node_name(node_name) {
        return Ok(unsafe { format_livestock_product_trait_url_from_node_name(node_name) });
    }

    if is_valid_variation_representation_specification_node_name(node_name) {
        return Ok(unsafe {
            format_variation_representation_specification_url_from_node_name(node_name)
        });
    }

    if is_valid_semantic_science_node_name(node_name) {
        return Ok(unsafe { format_semantic_science_url_from_node_name(node_name) });
    }

    if is_valid_experimental_factor_ontology_node_name(node_name) {
        return Ok(unsafe { format_experimental_factor_ontology_url_from_node_name(node_name) });
    }

    if is_valid_animal_genome_sheep_qtl_node_name(node_name) {
        return Ok(unsafe { format_animal_genome_sheep_qtl_url_from_node_name(node_name) });
    }

    if is_valid_open_biomedical_ontology_node_name(node_name) {
        return Ok(unsafe { format_open_biomedical_ontology_url_from_node_name(node_name) });
    }

    if is_valid_intact_node_name(node_name) {
        return Ok(unsafe { format_intact_url_from_node_name(node_name) });
    }

    if is_valid_game_ontology_project_node_name(node_name) {
        return Ok(unsafe { format_game_ontology_project_url_from_node_name(node_name) });
    }

    if is_valid_apergillus_genome_data_node_name(node_name) {
        return Ok(unsafe { format_apergillus_genome_data_url_from_node_name(node_name) });
    }

    if is_valid_animal_genome_pubblications_node_name(node_name) {
        return Ok(unsafe { format_animal_genome_pubblications_url_from_node_name(node_name) });
    }

    if is_valid_snp_individual_node_name(node_name) {
        return Ok(unsafe { format_snp_individual_url_from_node_name(node_name) });
    }

    if is_valid_ncbi_protein_node_name(node_name) {
        return Ok(unsafe { format_ncbi_protein_url_from_node_name(node_name) });
    }

    if is_valid_orpha_node_name(node_name) {
        return Ok(unsafe { format_orpha_url_from_node_name(node_name) });
    }

    if is_valid_wormbase_vocabulary_node_name(node_name) {
        return Ok(unsafe { format_wormbase_vocabulary_url_from_node_name(node_name) });
    }

    if is_valid_kegg_path_node_name(node_name) {
        return Ok(unsafe { format_kegg_path_url_from_node_name(node_name) });
    }

    if is_valid_vertebrate_gene_names_consortium_node_name(node_name) {
        return Ok(unsafe {
            format_vertebrate_gene_names_consortium_url_from_node_name(node_name)
        });
    }

    if is_valid_yeast_genome_locus_node_name(node_name) {
        return Ok(unsafe { format_yeast_genome_locus_url_from_node_name(node_name) });
    }

    if is_valid_schema_node_name(node_name) {
        return Ok(unsafe { format_schema_url_from_node_name(node_name) });
    }

    if is_valid_provenance_ontology_node_name(node_name) {
        return Ok(unsafe { format_provenance_ontology_url_from_node_name(node_name) });
    }

    if is_valid_kegg_disease_node_name(node_name) {
        return Ok(unsafe { format_kegg_disease_url_from_node_name(node_name) });
    }

    if is_valid_database_snp_node_name(node_name) {
        return Ok(unsafe { format_database_snp_url_from_node_name(node_name) });
    }

    if is_valid_cell_ontology_node_name(node_name) {
        return Ok(unsafe { format_cell_ontology_url_from_node_name(node_name) });
    }

    if is_valid_mugen_node_name(node_name) {
        return Ok(unsafe { format_mugen_url_from_node_name(node_name) });
    }

    if is_valid_world_geodetic_system_node_name(node_name) {
        return Ok(unsafe { format_world_geodetic_system_url_from_node_name(node_name) });
    }

    if is_valid_chromosome_ontology_node_name(node_name) {
        return Ok(unsafe { format_chromosome_ontology_url_from_node_name(node_name) });
    }

    if is_valid_pharmgkb_node_name(node_name) {
        return Ok(unsafe { format_pharmgkb_url_from_node_name(node_name) });
    }

    if is_valid_the_arabidopsis_information_resource_locus_node_name(node_name) {
        return Ok(unsafe {
            format_the_arabidopsis_information_resource_locus_url_from_node_name(node_name)
        });
    }

    if is_valid_void_node_name(node_name) {
        return Ok(unsafe { format_void_url_from_node_name(node_name) });
    }

    if is_valid_rdf_node_name(node_name) {
        return Ok(unsafe { format_rdf_url_from_node_name(node_name) });
    }

    if is_valid_rgd_reference_node_name(node_name) {
        return Ok(unsafe { format_rgd_reference_url_from_node_name(node_name) });
    }

    if is_valid_mouse_phenome_database_assay_node_name(node_name) {
        return Ok(unsafe { format_mouse_phenome_database_assay_url_from_node_name(node_name) });
    }

    if is_valid_drugbank_node_name(node_name) {
        return Ok(unsafe { format_drugbank_url_from_node_name(node_name) });
    }

    if is_valid_coriell_family_node_name(node_name) {
        return Ok(unsafe { format_coriell_family_url_from_node_name(node_name) });
    }

    if is_valid_pombase_node_name(node_name) {
        return Ok(unsafe { format_pombase_url_from_node_name(node_name) });
    }

    if is_valid_online_mendelian_inheritance_in_animals_node_name(node_name) {
        return Ok(unsafe {
            format_online_mendelian_inheritance_in_animals_url_from_node_name(node_name)
        });
    }

    if is_valid_interpro_node_name(node_name) {
        return Ok(unsafe { format_interpro_url_from_node_name(node_name) });
    }

    if is_valid_panther_database_node_name(node_name) {
        return Ok(unsafe { format_panther_database_url_from_node_name(node_name) });
    }

    if is_valid_human_developmental_stages_node_name(node_name) {
        return Ok(unsafe { format_human_developmental_stages_url_from_node_name(node_name) });
    }

    if is_valid_mouse_pathology_ontology_node_name(node_name) {
        return Ok(unsafe { format_mouse_pathology_ontology_url_from_node_name(node_name) });
    }

    if is_valid_unified_phenotype_ontology_upheno_node_name(node_name) {
        return Ok(unsafe {
            format_unified_phenotype_ontology_upheno_url_from_node_name(node_name)
        });
    }

    if is_valid_human_phenotype_ontology_node_name(node_name) {
        return Ok(unsafe { format_human_phenotype_ontology_url_from_node_name(node_name) });
    }

    if is_valid_protein_ontology_pro_node_name(node_name) {
        return Ok(unsafe { format_protein_ontology_pro_url_from_node_name(node_name) });
    }

    if is_valid_human_disease_ontology_node_name(node_name) {
        return Ok(unsafe { format_human_disease_ontology_url_from_node_name(node_name) });
    }

    if is_valid_zebrafish_anatomy_and_development_ontology_node_name(node_name) {
        return Ok(unsafe {
            format_zebrafish_anatomy_and_development_ontology_url_from_node_name(node_name)
        });
    }

    if is_valid_an_ontology_of_core_ecological_entities_node_name(node_name) {
        return Ok(unsafe {
            format_an_ontology_of_core_ecological_entities_url_from_node_name(node_name)
        });
    }

    if is_valid_pathway_ontology_node_name(node_name) {
        return Ok(unsafe { format_pathway_ontology_url_from_node_name(node_name) });
    }

    if is_valid_ascomycete_phenotype_ontology_node_name(node_name) {
        return Ok(unsafe { format_ascomycete_phenotype_ontology_url_from_node_name(node_name) });
    }

    if is_valid_environment_ontology_node_name(node_name) {
        return Ok(unsafe { format_environment_ontology_url_from_node_name(node_name) });
    }

    if is_valid_nci_thesaurus_obo_edition_node_name(node_name) {
        return Ok(unsafe { format_nci_thesaurus_obo_edition_url_from_node_name(node_name) });
    }

    if is_valid_mouse_adult_gross_anatomy_node_name(node_name) {
        return Ok(unsafe { format_mouse_adult_gross_anatomy_url_from_node_name(node_name) });
    }

    if is_valid_neuro_behavior_ontology_node_name(node_name) {
        return Ok(unsafe { format_neuro_behavior_ontology_url_from_node_name(node_name) });
    }

    if is_valid_chemical_methods_ontology_node_name(node_name) {
        return Ok(unsafe { format_chemical_methods_ontology_url_from_node_name(node_name) });
    }

    if is_valid_zebrafish_developmental_stages_ontology_node_name(node_name) {
        return Ok(unsafe {
            format_zebrafish_developmental_stages_ontology_url_from_node_name(node_name)
        });
    }

    if is_valid_fungal_gross_anatomy_node_name(node_name) {
        return Ok(unsafe { format_fungal_gross_anatomy_url_from_node_name(node_name) });
    }

    if is_valid_zebrafish_phenotype_ontology_node_name(node_name) {
        return Ok(unsafe { format_zebrafish_phenotype_ontology_url_from_node_name(node_name) });
    }

    if is_valid_ontology_of_biological_attributes_node_name(node_name) {
        return Ok(unsafe {
            format_ontology_of_biological_attributes_url_from_node_name(node_name)
        });
    }

    if is_valid_confidence_information_ontology_node_name(node_name) {
        return Ok(unsafe { format_confidence_information_ontology_url_from_node_name(node_name) });
    }

    if is_valid_ncbi_gene_expression_omnibus_node_name(node_name) {
        return Ok(unsafe { format_ncbi_gene_expression_omnibus_url_from_node_name(node_name) });
    }

    if is_valid_genotype_ontology_node_name(node_name) {
        return Ok(unsafe { format_genotype_ontology_url_from_node_name(node_name) });
    }

    if is_valid_ontology_of_adverse_events_node_name(node_name) {
        return Ok(unsafe { format_ontology_of_adverse_events_url_from_node_name(node_name) });
    }

    if is_valid_experimental_condition_ontology_node_name(node_name) {
        return Ok(unsafe { format_experimental_condition_ontology_url_from_node_name(node_name) });
    }

    if is_valid_plant_ontology_node_name(node_name) {
        return Ok(unsafe { format_plant_ontology_url_from_node_name(node_name) });
    }

    if is_valid_environmental_conditions_treatments_and_exposures_ontology_node_name(node_name) {
        return Ok(unsafe {
            format_environmental_conditions_treatments_and_exposures_ontology_url_from_node_name(
                node_name,
            )
        });
    }

    if is_valid_units_of_measurement_ontology_node_name(node_name) {
        return Ok(unsafe { format_units_of_measurement_ontology_url_from_node_name(node_name) });
    }

    if is_valid_ontology_for_biomedical_investigations_node_name(node_name) {
        return Ok(unsafe {
            format_ontology_for_biomedical_investigations_url_from_node_name(node_name)
        });
    }

    if is_valid_c_elegans_phenotype_node_name(node_name) {
        return Ok(unsafe { format_c_elegans_phenotype_url_from_node_name(node_name) });
    }

    if is_valid_biological_spatial_ontology_node_name(node_name) {
        return Ok(unsafe { format_biological_spatial_ontology_url_from_node_name(node_name) });
    }

    if is_valid_scientific_evidence_and_provenance_information_ontology_node_name(node_name) {
        return Ok(unsafe {
            format_scientific_evidence_and_provenance_information_ontology_url_from_node_name(
                node_name,
            )
        });
    }

    if is_valid_drosophila_gross_anatomy_node_name(node_name) {
        return Ok(unsafe { format_drosophila_gross_anatomy_url_from_node_name(node_name) });
    }

    if is_valid_the_statistical_methods_ontology_node_name(node_name) {
        return Ok(unsafe {
            format_the_statistical_methods_ontology_url_from_node_name(node_name)
        });
    }

    if is_valid_infectious_disease_ontology_node_name(node_name) {
        return Ok(unsafe { format_infectious_disease_ontology_url_from_node_name(node_name) });
    }

    if is_valid_drosophila_development_node_name(node_name) {
        return Ok(unsafe { format_drosophila_development_url_from_node_name(node_name) });
    }

    if is_valid_mental_disease_ontology_node_name(node_name) {
        return Ok(unsafe { format_mental_disease_ontology_url_from_node_name(node_name) });
    }

    if is_valid_human_developmental_anatomy_abstract_node_name(node_name) {
        return Ok(unsafe {
            format_human_developmental_anatomy_abstract_url_from_node_name(node_name)
        });
    }

    if is_valid_c_elegans_gross_anatomy_ontology_node_name(node_name) {
        return Ok(unsafe {
            format_c_elegans_gross_anatomy_ontology_url_from_node_name(node_name)
        });
    }

    if is_valid_phenotype_and_trait_ontology_node_name(node_name) {
        return Ok(unsafe { format_phenotype_and_trait_ontology_url_from_node_name(node_name) });
    }

    if is_valid_evidence_ontology_node_name(node_name) {
        return Ok(unsafe { format_evidence_ontology_url_from_node_name(node_name) });
    }

    if is_valid_information_artifact_ontology_node_name(node_name) {
        return Ok(unsafe { format_information_artifact_ontology_url_from_node_name(node_name) });
    }

    if is_valid_brenda_tissue_enzyme_source_node_name(node_name) {
        return Ok(unsafe { format_brenda_tissue_enzyme_source_url_from_node_name(node_name) });
    }

    if is_valid_mouse_developmental_anatomy_ontology_node_name(node_name) {
        return Ok(unsafe {
            format_mouse_developmental_anatomy_ontology_url_from_node_name(node_name)
        });
    }

    if is_valid_mammalian_phenotype_ontology_node_name(node_name) {
        return Ok(unsafe { format_mammalian_phenotype_ontology_url_from_node_name(node_name) });
    }

    if is_valid_foodon_node_name(node_name) {
        return Ok(unsafe { format_foodon_url_from_node_name(node_name) });
    }

    if is_valid_vertebrate_trait_ontology_node_name(node_name) {
        return Ok(unsafe { format_vertebrate_trait_ontology_url_from_node_name(node_name) });
    }

    if is_valid_exposure_ontology_node_name(node_name) {
        return Ok(unsafe { format_exposure_ontology_url_from_node_name(node_name) });
    }

    if is_valid_population_and_community_ontology_node_name(node_name) {
        return Ok(unsafe {
            format_population_and_community_ontology_url_from_node_name(node_name)
        });
    }

    if is_valid_clinical_measurement_ontology_node_name(node_name) {
        return Ok(unsafe { format_clinical_measurement_ontology_url_from_node_name(node_name) });
    }

    if is_valid_mondo_disease_ontology_node_name(node_name) {
        return Ok(unsafe { format_mondo_disease_ontology_url_from_node_name(node_name) });
    }

    if is_valid_ontology_for_general_medical_science_node_name(node_name) {
        return Ok(unsafe {
            format_ontology_for_general_medical_science_url_from_node_name(node_name)
        });
    }

    if is_valid_common_anatomy_reference_ontology_node_name(node_name) {
        return Ok(unsafe {
            format_common_anatomy_reference_ontology_url_from_node_name(node_name)
        });
    }

    if is_valid_gazetteer_node_name(node_name) {
        return Ok(unsafe { format_gazetteer_url_from_node_name(node_name) });
    }

    if is_valid_uberon_multispecies_anatomy_ontology_node_name(node_name) {
        return Ok(unsafe {
            format_uberon_multispecies_anatomy_ontology_url_from_node_name(node_name)
        });
    }

    if is_valid_mental_functioning_ontology_node_name(node_name) {
        return Ok(unsafe { format_mental_functioning_ontology_url_from_node_name(node_name) });
    }

    if is_valid_relation_ontology_node_name(node_name) {
        return Ok(unsafe { format_relation_ontology_url_from_node_name(node_name) });
    }

    if is_valid_ontology_for_parasite_lifecycle_node_name(node_name) {
        return Ok(unsafe { format_ontology_for_parasite_lifecycle_url_from_node_name(node_name) });
    }

    if is_valid_dictyostelium_discoideum_anatomy_node_name(node_name) {
        return Ok(unsafe {
            format_dictyostelium_discoideum_anatomy_url_from_node_name(node_name)
        });
    }

    if is_valid_emotion_ontology_node_name(node_name) {
        return Ok(unsafe { format_emotion_ontology_url_from_node_name(node_name) });
    }

    if is_valid_eaglei_resource_ontology_node_name(node_name) {
        return Ok(unsafe { format_eaglei_resource_ontology_url_from_node_name(node_name) });
    }

    if is_valid_dublin_core_terms_node_name(node_name) {
        return Ok(unsafe { format_dublin_core_terms_url_from_node_name(node_name) });
    }

    if is_valid_dublin_core_types_node_name(node_name) {
        return Ok(unsafe { format_dublin_core_types_url_from_node_name(node_name) });
    }

    if is_valid_flybase_controlled_vocabulary_node_name(node_name) {
        return Ok(unsafe { format_flybase_controlled_vocabulary_url_from_node_name(node_name) });
    }

    if is_valid_foundational_model_of_anatomy_ontology_subset_node_name(node_name) {
        return Ok(unsafe {
            format_foundational_model_of_anatomy_ontology_subset_url_from_node_name(node_name)
        });
    }

    if is_valid_gene_ontology_obo_in_owl_node_name(node_name) {
        return Ok(unsafe { format_gene_ontology_obo_in_owl_url_from_node_name(node_name) });
    }

    if is_valid_monarch_initiative_archive_node_name(node_name) {
        return Ok(unsafe { format_monarch_initiative_archive_url_from_node_name(node_name) });
    }

    if is_valid_monarch_initiative_data_node_name(node_name) {
        return Ok(unsafe { format_monarch_initiative_data_url_from_node_name(node_name) });
    }

    if is_valid_protein_modification_node_name(node_name) {
        return Ok(unsafe { format_protein_modification_url_from_node_name(node_name) });
    }

    if is_valid_rdfs_node_name(node_name) {
        return Ok(unsafe { format_rdfs_url_from_node_name(node_name) });
    }

    if is_valid_unified_medical_language_system_semantic_code_node_name(node_name) {
        return Ok(unsafe {
            format_unified_medical_language_system_semantic_code_url_from_node_name(node_name)
        });
    }

    if is_valid_unified_medical_language_system_semantic_type_node_name(node_name) {
        return Ok(unsafe {
            format_unified_medical_language_system_semantic_type_url_from_node_name(node_name)
        });
    }

    if is_valid_unified_medical_language_system_semantic_group_node_name(node_name) {
        return Ok(unsafe {
            format_unified_medical_language_system_semantic_group_url_from_node_name(node_name)
        });
    }

    if is_valid_cord_pubmed_central_node_name(node_name) {
        return Ok(unsafe {
            format_cord_pubmed_central_url_from_node_name(node_name)
        });
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
    if is_valid_wormbase_gene_node_name(node_name) {
        return Ok("WormBase Gene");
    }

    if is_valid_mouse_genome_informatics_node_name(node_name) {
        return Ok("Mouse Genome Informatics");
    }
    if is_valid_ncbi_dbgap_node_name(node_name) {
        return Ok("NCBI dbGaP");
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

    if is_valid_jackson_lab_reference_node_name(node_name) {
        return Ok("Jackson Lab Reference");
    }

    if is_valid_uberon_core_node_name(node_name) {
        return Ok("Uberon Core");
    }

    if is_valid_gene_ontology_relations_node_name(node_name) {
        return Ok("Gene Ontology Relations");
    }

    if is_valid_quantity_unit_dimension_and_type_node_name(node_name) {
        return Ok("Quantity, Unit, Dimension and Type");
    }

    if is_valid_open_biomedical_association_node_name(node_name) {
        return Ok("Open Biomedical Association");
    }

    if is_valid_kegg_homo_sapiens_node_name(node_name) {
        return Ok("KEGG Homo sapiens");
    }

    if is_valid_dublin_core_node_name(node_name) {
        return Ok("Dublin Core");
    }

    if is_valid_xsd_node_name(node_name) {
        return Ok("XSD");
    }

    if is_valid_kegg_ko_node_name(node_name) {
        return Ok("KEGG-KO");
    }

    if is_valid_swiss_protein_node_name(node_name) {
        return Ok("Swiss Protein");
    }

    if is_valid_ncbi_books_node_name(node_name) {
        return Ok("NCBI Books");
    }

    if is_valid_chembl_compound_node_name(node_name) {
        return Ok("Chembl Compound");
    }

    if is_valid_hugo_gene_name_consortium_node_name(node_name) {
        return Ok("Hugo Gene Name Consortium");
    }

    if is_valid_provenance_authoring_and_versioning_node_name(node_name) {
        return Ok("Provenance, Authoring and Versioning");
    }

    if is_valid_feature_annotation_location_description_ontology_node_name(node_name) {
        return Ok("Feature Annotation Location Description Ontology");
    }

    if is_valid_rbrc_node_name(node_name) {
        return Ok("RBRC");
    }

    if is_valid_dictybase_gene_node_name(node_name) {
        return Ok("dictyBase Gene");
    }

    if is_valid_edam_data_node_name(node_name) {
        return Ok("EDAM Data");
    }

    if is_valid_ncimr_node_name(node_name) {
        return Ok("NCIMR");
    }

    if is_valid_ecogene_node_name(node_name) {
        return Ok("EcoGene");
    }

    if is_valid_aspergillus_genomic_data_reference_node_name(node_name) {
        return Ok("Aspergillus Genomic Data Reference");
    }

    if is_valid_european_mouse_mutant_archive_node_name(node_name) {
        return Ok("European Mouse Mutant Archive");
    }

    if is_valid_translation_of_embl_nucleotide_sequence_database_node_name(node_name) {
        return Ok("Translation of EMBL nucleotide sequence database");
    }

    if is_valid_mouse_phenome_database_strain_node_name(node_name) {
        return Ok("Mouse Phenome Database Strain");
    }

    if is_valid_basic_formal_ontology_node_name(node_name) {
        return Ok("Basic Formal Ontology");
    }

    if is_valid_dictybase_genes_node_name(node_name) {
        return Ok("Dictybase Genes");
    }

    if is_valid_cell_line_ontology_node_name(node_name) {
        return Ok("Cell Line Ontology");
    }

    if is_valid_animal_genome_horse_qtl_node_name(node_name) {
        return Ok("Animal Genome Horse QTL");
    }

    if is_valid_wormbase_node_name(node_name) {
        return Ok("WormBase");
    }

    if is_valid_animal_qtl_traits_node_name(node_name) {
        return Ok("Animal QTL Traits");
    }

    if is_valid_friend_of_a_friend_node_name(node_name) {
        return Ok("Friend of a Friend");
    }

    if is_valid_omim_phenotypic_series_node_name(node_name) {
        return Ok("OMIM Phenotypic Series");
    }

    if is_valid_arabidopsis_tair_node_name(node_name) {
        return Ok("Arabidopsis Tair");
    }

    if is_valid_unified_medical_language_system_node_name(node_name) {
        return Ok("Unified Medical Language System");
    }

    if is_valid_gene_ontology_reference_node_name(node_name) {
        return Ok("Gene Ontology Reference");
    }

    if is_valid_ncbi_clinical_variants_submitters_node_name(node_name) {
        return Ok("NCBI Clinical Variants Submitters");
    }

    if is_valid_gdc_project_node_name(node_name) {
        return Ok("GDC Project");
    }

    if is_valid_complex_portal_node_name(node_name) {
        return Ok("Complex Portal");
    }

    if is_valid_ucsc_golden_path_node_name(node_name) {
        return Ok("UCSC Golden Path");
    }

    if is_valid_yeast_genome_reference_node_name(node_name) {
        return Ok("Yeast Genome Reference");
    }

    if is_valid_xenbase_node_name(node_name) {
        return Ok("XENBASE");
    }

    if is_valid_chembl_target_node_name(node_name) {
        return Ok("CHEMBL Target");
    }

    if is_valid_coriell_collection_node_name(node_name) {
        return Ok("Coriell Collection");
    }

    if is_valid_gdc_annotation_node_name(node_name) {
        return Ok("GDC Annotation");
    }

    if is_valid_animal_genome_chicken_qtl_node_name(node_name) {
        return Ok("Animal Genome Chicken QTL");
    }

    if is_valid_semantic_medline_database_node_name(node_name) {
        return Ok("Semantic MEDLINE Database");
    }

    if is_valid_animal_genome_rainbow_trout_qtl_node_name(node_name) {
        return Ok("Animal Genome Rainbow-Trout QTL");
    }

    if is_valid_animal_genome_pig_qtl_node_name(node_name) {
        return Ok("Animal Genome Pig QTL");
    }

    if is_valid_livestock_product_trait_node_name(node_name) {
        return Ok("Livestock Product Trait");
    }

    if is_valid_variation_representation_specification_node_name(node_name) {
        return Ok("Variation Representation Specification");
    }

    if is_valid_semantic_science_node_name(node_name) {
        return Ok("Semantic Science");
    }

    if is_valid_experimental_factor_ontology_node_name(node_name) {
        return Ok("Experimental Factor Ontology");
    }

    if is_valid_animal_genome_sheep_qtl_node_name(node_name) {
        return Ok("Animal Genome Sheep QTL");
    }

    if is_valid_open_biomedical_ontology_node_name(node_name) {
        return Ok("Open Biomedical Ontology");
    }

    if is_valid_intact_node_name(node_name) {
        return Ok("INTACT");
    }

    if is_valid_game_ontology_project_node_name(node_name) {
        return Ok("Game Ontology Project");
    }

    if is_valid_apergillus_genome_data_node_name(node_name) {
        return Ok("Apergillus Genome Data");
    }

    if is_valid_animal_genome_pubblications_node_name(node_name) {
        return Ok("Animal Genome Pubblications");
    }

    if is_valid_monarch_initiative_node_name(node_name) {
        return Ok("Monarch Initiative");
    }

    if is_valid_snp_individual_node_name(node_name) {
        return Ok("SNP Individual");
    }

    if is_valid_ncbi_protein_node_name(node_name) {
        return Ok("NCBI Protein");
    }

    if is_valid_orpha_node_name(node_name) {
        return Ok("ORPHA");
    }

    if is_valid_wormbase_vocabulary_node_name(node_name) {
        return Ok("Wormbase Vocabulary");
    }

    if is_valid_kegg_path_node_name(node_name) {
        return Ok("KEGG Path");
    }

    if is_valid_vertebrate_gene_names_consortium_node_name(node_name) {
        return Ok("Vertebrate Gene Names Consortium");
    }

    if is_valid_yeast_genome_locus_node_name(node_name) {
        return Ok("Yeast Genome Locus");
    }

    if is_valid_schema_node_name(node_name) {
        return Ok("SCHEMA");
    }

    if is_valid_provenance_ontology_node_name(node_name) {
        return Ok("Provenance Ontology");
    }

    if is_valid_kegg_disease_node_name(node_name) {
        return Ok("KEGG Disease");
    }

    if is_valid_database_snp_node_name(node_name) {
        return Ok("Database SNP");
    }

    if is_valid_cell_ontology_node_name(node_name) {
        return Ok("Cell Ontology");
    }

    if is_valid_mugen_node_name(node_name) {
        return Ok("Mugen");
    }

    if is_valid_world_geodetic_system_node_name(node_name) {
        return Ok("World Geodetic System");
    }

    if is_valid_chromosome_ontology_node_name(node_name) {
        return Ok("Chromosome Ontology");
    }

    if is_valid_pharmgkb_node_name(node_name) {
        return Ok("PHARMGKB");
    }

    if is_valid_the_arabidopsis_information_resource_locus_node_name(node_name) {
        return Ok("The Arabidopsis Information Resource Locus");
    }

    if is_valid_owl_node_name(node_name) {
        return Ok("OWL");
    }

    if is_valid_void_node_name(node_name) {
        return Ok("VOID");
    }

    if is_valid_rdf_node_name(node_name) {
        return Ok("RDF");
    }

    if is_valid_rgd_reference_node_name(node_name) {
        return Ok("RGD Reference");
    }

    if is_valid_mouse_phenome_database_assay_node_name(node_name) {
        return Ok("Mouse Phenome Database Assay");
    }

    if is_valid_drugbank_node_name(node_name) {
        return Ok("DrugBank");
    }

    if is_valid_coriell_family_node_name(node_name) {
        return Ok("Coriell Family");
    }

    if is_valid_pombase_node_name(node_name) {
        return Ok("POMBASE");
    }

    if is_valid_online_mendelian_inheritance_in_animals_node_name(node_name) {
        return Ok("Online Mendelian Inheritance in Animals");
    }

    if is_valid_interpro_node_name(node_name) {
        return Ok("InterPro");
    }

    if is_valid_panther_database_node_name(node_name) {
        return Ok("Panther Database");
    }

    if is_valid_human_developmental_stages_node_name(node_name) {
        return Ok("Human Developmental Stages");
    }

    if is_valid_mouse_pathology_ontology_node_name(node_name) {
        return Ok("Mouse pathology ontology");
    }

    if is_valid_unified_phenotype_ontology_upheno_node_name(node_name) {
        return Ok("Unified phenotype ontology (uPheno)");
    }

    if is_valid_human_phenotype_ontology_node_name(node_name) {
        return Ok("Human Phenotype Ontology");
    }

    if is_valid_protein_ontology_pro_node_name(node_name) {
        return Ok("PRotein Ontology (PRO)");
    }

    if is_valid_human_disease_ontology_node_name(node_name) {
        return Ok("Human Disease Ontology");
    }

    if is_valid_zebrafish_anatomy_and_development_ontology_node_name(node_name) {
        return Ok("Zebrafish anatomy and development ontology");
    }

    if is_valid_an_ontology_of_core_ecological_entities_node_name(node_name) {
        return Ok("An ontology of core ecological entities");
    }

    if is_valid_pathway_ontology_node_name(node_name) {
        return Ok("Pathway ontology");
    }

    if is_valid_ascomycete_phenotype_ontology_node_name(node_name) {
        return Ok("Ascomycete phenotype ontology");
    }

    if is_valid_environment_ontology_node_name(node_name) {
        return Ok("Environment Ontology");
    }

    if is_valid_nci_thesaurus_obo_edition_node_name(node_name) {
        return Ok("NCI Thesaurus OBO Edition");
    }

    if is_valid_mouse_adult_gross_anatomy_node_name(node_name) {
        return Ok("Mouse adult gross anatomy");
    }

    if is_valid_neuro_behavior_ontology_node_name(node_name) {
        return Ok("Neuro Behavior Ontology");
    }

    if is_valid_chemical_methods_ontology_node_name(node_name) {
        return Ok("Chemical Methods Ontology");
    }

    if is_valid_zebrafish_developmental_stages_ontology_node_name(node_name) {
        return Ok("Zebrafish developmental stages ontology");
    }

    if is_valid_fungal_gross_anatomy_node_name(node_name) {
        return Ok("Fungal gross anatomy");
    }

    if is_valid_zebrafish_phenotype_ontology_node_name(node_name) {
        return Ok("Zebrafish Phenotype Ontology");
    }

    if is_valid_ontology_of_biological_attributes_node_name(node_name) {
        return Ok("Ontology of Biological Attributes");
    }

    if is_valid_confidence_information_ontology_node_name(node_name) {
        return Ok("Confidence Information Ontology");
    }

    if is_valid_ncbi_gene_expression_omnibus_node_name(node_name) {
        return Ok("NCBI Gene Expression Omnibus");
    }

    if is_valid_genotype_ontology_node_name(node_name) {
        return Ok("Genotype Ontology");
    }

    if is_valid_ontology_of_adverse_events_node_name(node_name) {
        return Ok("Ontology of Adverse Events");
    }

    if is_valid_experimental_condition_ontology_node_name(node_name) {
        return Ok("Experimental condition ontology");
    }

    if is_valid_plant_ontology_node_name(node_name) {
        return Ok("Plant Ontology");
    }

    if is_valid_environmental_conditions_treatments_and_exposures_ontology_node_name(node_name) {
        return Ok("Environmental conditions, treatments and exposures ontology");
    }

    if is_valid_units_of_measurement_ontology_node_name(node_name) {
        return Ok("Units of measurement ontology");
    }

    if is_valid_ontology_for_biomedical_investigations_node_name(node_name) {
        return Ok("Ontology for Biomedical Investigations");
    }

    if is_valid_c_elegans_phenotype_node_name(node_name) {
        return Ok("C. elegans phenotype");
    }

    if is_valid_biological_spatial_ontology_node_name(node_name) {
        return Ok("Biological Spatial Ontology");
    }

    if is_valid_scientific_evidence_and_provenance_information_ontology_node_name(node_name) {
        return Ok("Scientific Evidence and Provenance Information Ontology");
    }

    if is_valid_drosophila_gross_anatomy_node_name(node_name) {
        return Ok("Drosophila gross anatomy");
    }

    if is_valid_the_statistical_methods_ontology_node_name(node_name) {
        return Ok("The Statistical Methods Ontology");
    }

    if is_valid_infectious_disease_ontology_node_name(node_name) {
        return Ok("Infectious Disease Ontology");
    }

    if is_valid_drosophila_development_node_name(node_name) {
        return Ok("Drosophila development");
    }

    if is_valid_mental_disease_ontology_node_name(node_name) {
        return Ok("Mental Disease Ontology");
    }

    if is_valid_human_developmental_anatomy_abstract_node_name(node_name) {
        return Ok("Human developmental anatomy, abstract");
    }

    if is_valid_c_elegans_gross_anatomy_ontology_node_name(node_name) {
        return Ok("C. elegans Gross Anatomy Ontology");
    }

    if is_valid_phenotype_and_trait_ontology_node_name(node_name) {
        return Ok("Phenotype And Trait Ontology");
    }

    if is_valid_evidence_ontology_node_name(node_name) {
        return Ok("Evidence ontology");
    }

    if is_valid_information_artifact_ontology_node_name(node_name) {
        return Ok("Information Artifact Ontology");
    }

    if is_valid_brenda_tissue_enzyme_source_node_name(node_name) {
        return Ok("BRENDA tissue / enzyme source");
    }

    if is_valid_mouse_developmental_anatomy_ontology_node_name(node_name) {
        return Ok("Mouse Developmental Anatomy Ontology");
    }

    if is_valid_mammalian_phenotype_ontology_node_name(node_name) {
        return Ok("Mammalian Phenotype Ontology");
    }

    if is_valid_foodon_node_name(node_name) {
        return Ok("FOODON");
    }

    if is_valid_vertebrate_trait_ontology_node_name(node_name) {
        return Ok("Vertebrate trait ontology");
    }

    if is_valid_exposure_ontology_node_name(node_name) {
        return Ok("Exposure ontology");
    }

    if is_valid_population_and_community_ontology_node_name(node_name) {
        return Ok("Population and Community Ontology");
    }

    if is_valid_clinical_measurement_ontology_node_name(node_name) {
        return Ok("Clinical measurement ontology");
    }

    if is_valid_mondo_disease_ontology_node_name(node_name) {
        return Ok("Mondo Disease Ontology");
    }

    if is_valid_ontology_for_general_medical_science_node_name(node_name) {
        return Ok("Ontology for General Medical Science");
    }

    if is_valid_common_anatomy_reference_ontology_node_name(node_name) {
        return Ok("Common Anatomy Reference Ontology");
    }

    if is_valid_gazetteer_node_name(node_name) {
        return Ok("Gazetteer");
    }

    if is_valid_uberon_multispecies_anatomy_ontology_node_name(node_name) {
        return Ok("Uberon multi-species anatomy ontology");
    }

    if is_valid_mental_functioning_ontology_node_name(node_name) {
        return Ok("Mental Functioning Ontology");
    }

    if is_valid_relation_ontology_node_name(node_name) {
        return Ok("Relation Ontology");
    }

    if is_valid_ontology_for_parasite_lifecycle_node_name(node_name) {
        return Ok("Ontology for Parasite LifeCycle");
    }

    if is_valid_dictyostelium_discoideum_anatomy_node_name(node_name) {
        return Ok("Dictyostelium discoideum anatomy");
    }

    if is_valid_emotion_ontology_node_name(node_name) {
        return Ok("Emotion Ontology");
    }

    if is_valid_eaglei_resource_ontology_node_name(node_name) {
        return Ok("eagle-i resource ontology");
    }

    if is_valid_dublin_core_terms_node_name(node_name) {
        return Ok("Dublin Core Terms");
    }

    if is_valid_dublin_core_types_node_name(node_name) {
        return Ok("Dublin Core Types");
    }

    if is_valid_flybase_controlled_vocabulary_node_name(node_name) {
        return Ok("FlyBase Controlled Vocabulary");
    }

    if is_valid_foundational_model_of_anatomy_ontology_subset_node_name(node_name) {
        return Ok("Foundational Model of Anatomy Ontology (subset)");
    }

    if is_valid_gene_ontology_obo_in_owl_node_name(node_name) {
        return Ok("Gene Ontology OBO in OWL");
    }

    if is_valid_monarch_initiative_archive_node_name(node_name) {
        return Ok("Monarch Initiative Archive");
    }

    if is_valid_monarch_initiative_data_node_name(node_name) {
        return Ok("Monarch Initiative Data");
    }

    if is_valid_protein_modification_node_name(node_name) {
        return Ok("Protein modification");
    }

    if is_valid_rdfs_node_name(node_name) {
        return Ok("RDFS");
    }

    if is_valid_unified_medical_language_system_semantic_code_node_name(node_name) {
        return Ok("Unified Medical Language System Semantic Code");
    }

    if is_valid_unified_medical_language_system_semantic_type_node_name(node_name) {
        return Ok("Unified Medical Language System Semantic Type");
    }

    if is_valid_unified_medical_language_system_semantic_group_node_name(node_name) {
        return Ok("Unified Medical Language System Semantic Group");
    }

    if is_valid_cord_pubmed_central_node_name(node_name) {
        return Ok("CORD Pubmed Central");
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
