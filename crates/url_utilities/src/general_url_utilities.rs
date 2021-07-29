use itertools::Itertools;
use shared::types::Result;
use crate::biolink_url_utilities::*;

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
                    if node_name.matches(separator).count() >= 1 {
                        node_name = node_name.split(separator).skip(1).join("");
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
        if !node_name
            .to_uppercase()
            .starts_with(&id_acronym.to_uppercase())
        {
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
        separator.map_or(node_name.to_string(), |sep| {
            if node_name.contains(sep) {
                node_name.split(sep).skip(1).join("")
            } else {
                node_name.to_string()
            }
        }).as_str(),
    )
}

/// Returns built url for given element
pub(crate) fn get_url_formatted(url: &str, content: &str, repository: &str) -> String {
    format!(
        "<a href='{url}' target='_blank' title='Go to {repository} to get more informations about {content}'>{content}</a>",
        url = url,
        content = content,
        repository=repository
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
