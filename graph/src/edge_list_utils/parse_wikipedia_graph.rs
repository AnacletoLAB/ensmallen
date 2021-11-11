use crate::{
    get_rows_number, sort_numeric_edge_list_inplace, utils::get_loading_bar, EdgeFileWriter, EdgeT,
    NodeFileWriter, NodeT, NodeTypeT, Result, TypeFileWriter, Vocabulary,
};
use indicatif::ProgressIterator;
use log::info;
use regex::Regex;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

const COMMENT_SYMBOLS: &[&str] = &["&lt;", "{", "}", "|", "----", "!", ":", "=", "<"];
const SPECIAL_NODE_STARTERS: &[&str] = &[
    "/",
    "Image:",
    "image:",
    "User:",
    "File:",
    ":File",
    "media:",
    "Cookbook:",
    "Template:",
];

/// Returns boolean represing whether the given candidate node is a special node.
///
/// # Arguments
/// `candidate_node`: &str - Candidate node to check.
fn is_special_node(candidate_node: &str) -> bool {
    SPECIAL_NODE_STARTERS
        .iter()
        .any(|&starter| candidate_node.starts_with(starter))
}

/// Returns boolean representing whether the given line should be skipped.
///
/// # Arguments
/// `line`: &str - The line to check for.
///
/// # Implementative details
/// We skip the line if:
/// * The line is empty.
/// * The line starts with a comment symbol.
fn should_skip_line(line: &str) -> bool {
    line.is_empty()
        || COMMENT_SYMBOLS
            .iter()
            .any(|comment_symbol| line.starts_with(comment_symbol))
}

/// Returns the iterator of lines on the file provided at given path.
///
/// # Arguments
/// `path`: &str - The path to be read.
fn get_lines_iterator(path: &str) -> Result<impl Iterator<Item = Result<String>>> {
    let file = File::open(path);
    if file.is_err() {
        return Err(format!("Cannot open the file at {}", path));
    }
    let file = file.unwrap();
    #[cfg(target_os = "linux")]
    let _ = posix_fadvise(
        file.as_raw_fd(),
        0,
        0,
        PosixFadviseAdvice::POSIX_FADV_SEQUENTIAL,
    );
    let buffer = BufReader::with_capacity(8 * 1024 * 1024, file);
    Ok(buffer.lines().map(|line|{
        match line {
            Ok(l)=>Ok(l.trim().to_string()),
            Err(_)=>Err("There might have been an I/O error or the line could contains bytes that are not valid UTF-8".to_string()),
        }
    }))
}

/// TODO: write the docstring
pub fn parse_wikipedia_graph(
    source_path: &str,
    edge_path: &str,
    node_path: &str,
    node_type_path: &str,
    node_list_separator: char,
    node_type_list_separator: char,
    node_types_separator: &str,
    nodes_column: &str,
    node_types_column: &str,
    node_list_node_types_column: &str,
    node_descriptions_column: &str,
    edge_list_separator: char,
    sort_temporary_directory: Option<String>,
    directed: bool,
    keep_interwikipedia_nodes: Option<bool>,
    keep_external_nodes: Option<bool>,
    verbose: Option<bool>,
) -> Result<(NodeTypeT, NodeT, EdgeT)> {
    // TODO! When normalizing the nodes, keep only the right-most value after : if the node name does not start with http.
    let keep_external_nodes = keep_external_nodes.unwrap_or(true);
    let keep_interwikipedia_nodes = keep_interwikipedia_nodes.unwrap_or(true);
    let verbose = verbose.unwrap_or(true);
    let mut node_types_vocabulary: Vocabulary<NodeTypeT> = Vocabulary::new(false);
    let mut nodes_vocabulary: Vocabulary<NodeT> = Vocabulary::new(false);

    let nodes_writer: NodeFileWriter = NodeFileWriter::new(node_path)
        .set_separator(Some(node_list_separator))?
        .set_node_types_separator(Some(node_types_separator))?
        .set_nodes_column(Some(nodes_column))
        .set_nodes_column_number(Some(0))
        .set_node_types_column(Some(node_list_node_types_column))
        .set_node_types_column_number(Some(1))
        .set_node_descriptions_column(Some(node_descriptions_column))
        .set_node_descriptions_column_number(Some(2))
        .set_numeric_node_type_ids(Some(true));

    let mut nodes_stream = nodes_writer.start_writer()?;

    let node_types_writer = TypeFileWriter::new(node_type_path)
        .set_separator(Some(node_type_list_separator))?
        .set_types_column(Some(node_types_column));

    let mut node_types_stream = node_types_writer.start_writer()?;

    let edges_writer = EdgeFileWriter::new(edge_path)
        .set_sources_column_number(Some(0))
        .set_destinations_column_number(Some(1))
        .set_separator(Some(edge_list_separator))?
        .set_numeric_node_ids(Some(true))
        .set_numeric_edge_type_ids(Some(true))
        .set_header(Some(false));

    let mut edges_stream = edges_writer.start_writer()?;

    // Create the required regex.
    // First we create the regex to recognize titles.
    let title_regex = Regex::new(r"^<title>([^<]+)</title>$").unwrap();
    // Then we create the regex to recognize the end of a page.
    let end_of_page_regex = Regex::new(r"^</page>$").unwrap();
    // Then we define the regex to extract the destination nodes.
    let destination_nodes_regex = Regex::new(r"\[\[([^\]]+?)(?:\|[^\]]+?)?\]\]").unwrap();
    // Then we define the regex to extract the node types.
    let categories = [
        "Category",
        "Categoria",
        "Категория",
        "Kategória",
        "Kategorie",
        "Κατηγορία",
        "Categoría",
        "Luokka",
        "Kategori",
        "კატეგორია",
        "분류",
        "Kategorija",
    ];

    let node_types_regex = Regex::new(&format!(
        r"^\[\[[^\]]*?(?:{}):([^\]]+?)\]\]$",
        categories.join("|")
    ))
    .unwrap();

    info!("Count the lines in the source path, so to be able to show a loading bar.");
    let lines_number = if verbose {
        get_rows_number(source_path.as_ref())?
    } else {
        0
    };

    // Start to read of the file.
    info!("Starting to build the node list and node type list.");
    let pb = get_loading_bar(
        verbose,
        "Executing first parse to build the node and node type list.",
        lines_number,
    );
    // Initialize the current node name.
    let mut current_node_name: Option<String> = None;
    let mut current_node_types: Vec<NodeTypeT> = Vec::new();
    let mut current_node_description: Vec<String> = Vec::new();
    let mut current_line_number: usize = 0;
    // Start to parse and write the node list and node type list.
    for line in get_lines_iterator(source_path)?.progress_with(pb) {
        // We increase the current line number
        current_line_number += 1;
        // First of all we check that all is fine with the current line reading attempt.
        let line = line?;
        // We check if the current page is finished.
        if end_of_page_regex.is_match(&line) {
            if let Some(current_node_name) = current_node_name {
                let (node_id, was_already_present) = nodes_vocabulary.insert(&current_node_name)?;
                if was_already_present {
                    return Err(format!(
                        "The title `{}` was encounter in more than a single page!",
                        current_node_name
                    ));
                } else {
                    nodes_stream = nodes_writer.write_line(
                        nodes_stream,
                        node_id,
                        current_node_name,
                        if current_node_types.is_empty() {
                            None
                        } else {
                            Some(current_node_types)
                        },
                        None,
                        Some(current_node_description.join("")),
                    )?;
                }
            } else {
                return Err("End of page was reached, but no title was found!".to_string());
            }
            // We write the node to the node list file.
            // Finally we restore the current varibales to defaults.
            current_node_name = None;
            current_node_types = Vec::new();
            current_node_description = Vec::new();
        }
        // Check if the line contains a title if we don't currently have one.
        if current_node_name.is_none() {
            if let Some(captures) = title_regex.captures(&line) {
                current_node_name = Some(captures[1].to_owned());
            }
            continue;
        }
        // We check if the line should be skipped
        if should_skip_line(&line) {
            continue;
        }
        // Check if the line is a node type.
        if let Some(captures) = node_types_regex.captures(&line) {
            let node_type_name = captures[1].to_owned();
            // Get the node type ID and insert the original string into the dictionary.
            let (node_type_id, was_already_present) =
                node_types_vocabulary.insert(&node_type_name)?;
            // If the node type was not already present
            // we write it to disk.
            if !was_already_present {
                node_types_stream = node_types_writer.write_line(
                    node_types_stream,
                    node_type_id,
                    node_type_name,
                )?;
            }
            current_node_types.push(node_type_id);
            continue;
        }
        current_node_description.push(line);
    }
    // Reset the buffer
    info!("Starting to build the edge list.");
    let pb = get_loading_bar(
        verbose,
        "Executing second parse to build the edge list.",
        current_line_number,
    );
    let mut source_node_id = 0;
    for line in get_lines_iterator(source_path)?.progress_with(pb) {
        // First of all we check that all is fine with the current line reading attempt.
        let line = line?;
        // Each time we finish to read a page, we can safely increase the current node ID.
        if end_of_page_regex.is_match(&line) {
            source_node_id += 1;
            continue;
        }
        // We check if the line should be skipped
        if should_skip_line(&line) || node_types_regex.is_match(&line) {
            continue;
        }
        // Finally, we parse the line and extract the destination nodes.
        for destination_node_name in destination_nodes_regex
            .captures_iter(&line)
            .into_iter()
            .map(|destination_node_name| destination_node_name[1].to_owned())
        {
            let (destination_node_id, was_already_present) =
                nodes_vocabulary.insert(&destination_node_name)?;
            if !was_already_present {
                nodes_stream = nodes_writer.write_line(
                    nodes_stream,
                    destination_node_id,
                    destination_node_name,
                    None,
                    None,
                    None,
                )?;
            }
            edges_stream = edges_writer.write_line(
                edges_stream,
                0,
                source_node_id,
                "".to_owned(),
                destination_node_id,
                "".to_owned(),
                None,
                None,
                None,
            )?;
            if !directed && source_node_id != destination_node_id {
                edges_stream = edges_writer.write_line(
                    edges_stream,
                    0,
                    destination_node_id,
                    "".to_owned(),
                    source_node_id,
                    "".to_owned(),
                    None,
                    None,
                    None,
                )?;
            }
        }
    }
    nodes_writer.close_writer(nodes_stream)?;
    node_types_writer.close_writer(node_types_stream)?;
    edges_writer.close_writer(edges_stream)?;

    // Finally, we sort the edge list we have created in place.
    info!("Sorting the edge list in place.");
    sort_numeric_edge_list_inplace(
        edge_path,
        Some(edge_list_separator),
        Some(false),
        None,
        Some(0),
        None,
        Some(1),
        None,
        None,
        None,
        None,
        sort_temporary_directory,
    )?;

    info!("Count the lines in the path, that match exactly with the number of edges.");
    let edges_number = get_rows_number(edge_path.as_ref())? as EdgeT;

    Ok((
        node_types_vocabulary.len() as NodeTypeT,
        nodes_vocabulary.len() as NodeT,
        edges_number,
    ))
}
