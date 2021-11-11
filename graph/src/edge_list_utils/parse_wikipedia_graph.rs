use crate::{
    get_rows_number, sort_numeric_edge_list_inplace, utils::get_loading_bar, EdgeFileWriter, EdgeT,
    NodeFileWriter, NodeT, NodeTypeT, Result, TypeFileWriter, Vocabulary,
};
use indicatif::ProgressIterator;
use lazy_static::lazy_static;
use log::info;
#[cfg(target_os = "linux")]
use nix::fcntl::*;
use regex::Regex;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
#[cfg(target_os = "linux")]
use std::os::unix::io::AsRawFd;

const COMMENT_SYMBOLS: &[&str] = &["&lt;", "{", "}", "|", "----", "!", ":", "=", "<", "*"];

lazy_static! {
    static ref LINE_SANITIZER_CURLY_BRACES_REMOVER: Regex = Regex::new(r"\{\{[^\}]+?\}\}").unwrap();
    static ref LINE_SANITIZER_SQUARE_BRACES_REMOVER: Regex =
        Regex::new(r"\[\[(?P<a>[^\]]+?)(?:\|[^\]]+?)?\]\]").unwrap();
    static ref LINE_SANITIZER_ANGULAR_BRACES_REMOVER: Regex = Regex::new(r"&lt;.+?&gt;").unwrap();
    static ref LINE_SANITIZER_SPACES_REMOVER: Regex = Regex::new(r"\s\s+").unwrap();
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

/// Remove metadata and other symbols from the text in a wikipedia page
fn sanitize_line(mut line: String) -> String {
    line.remove_matches("[");
    line.remove_matches("]");
    line.remove_matches("'");
    line.remove_matches("&quot;");
    line = LINE_SANITIZER_SPACES_REMOVER
        .replace_all(&line, " ")
        .to_string();
    line = LINE_SANITIZER_CURLY_BRACES_REMOVER
        .replace_all(&line, "")
        .to_string();
    line = LINE_SANITIZER_ANGULAR_BRACES_REMOVER
        .replace_all(&line, "")
        .to_string();
    line = LINE_SANITIZER_SQUARE_BRACES_REMOVER
        .replace_all(&line, r"\a")
        .to_string();
    line
}

fn sanitize_term(term: &str) -> String {
    let x: &[_] = &[':', '-', '/'];
    term.to_owned().trim().trim_matches(x).to_owned()
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
    verbose: Option<bool>,
) -> Result<(NodeTypeT, NodeT, EdgeT)> {
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
        r"^\[\[[^\]]*?(?:{}):([^\]\|]+?)(?:\|[^\]]*?)?\]\]$",
        categories.join("|")
    ))
    .unwrap();
    // Start to read of the file.
    info!("Starting to build the node list and node type list.");
    // Initialize the current node name.
    let mut current_node_id: Option<NodeT> = None;
    let mut current_node_name: Option<String> = None;
    let mut current_node_types: Vec<NodeTypeT> = Vec::new();
    let mut current_node_description: Vec<String> = Vec::new();
    let mut current_line_number: usize = 0;
    // Start to parse and write the node list and node type list.
    for line in get_lines_iterator(source_path)? {
        // We increase the current line number
        current_line_number += 1;
        // First of all we check that all is fine with the current line reading attempt.
        let line = line?;
        // We check if the current page is finished.
        if end_of_page_regex.is_match(&line) {
            if let (Some(current_node_id), Some(current_node_name)) =
                (current_node_id, current_node_name)
            {
                nodes_stream = nodes_writer.write_line(
                    nodes_stream,
                    current_node_id,
                    current_node_name,
                    if current_node_types.is_empty() {
                        None
                    } else {
                        Some(current_node_types)
                    },
                    None,
                    Some(sanitize_line(current_node_description.join(" "))),
                )?;
            }
            // We write the node to the node list file.
            // Finally we restore the current varibales to defaults.
            current_node_id = None;
            current_node_name = None;
            current_node_types = Vec::new();
            current_node_description = Vec::new();
        }
        // Check if the line contains a title if we don't currently have one.
        if current_node_name.is_none() {
            if let Some(captures) = title_regex.captures(&line) {
                let node_name = sanitize_term(&captures[1]);
                // Check that the node name is not empty
                if node_name.is_empty() {
                    continue;
                }
                let (node_id, was_already_present) = nodes_vocabulary.insert(&node_name)?;
                // Since the node may have been already parsed in the case
                // when multiple pages share the same title we need to check
                // for collisions.
                if !was_already_present {
                    current_node_id = Some(node_id);
                    current_node_name = Some(node_name);
                }
            }
            continue;
        }
        // We check if the line should be skipped
        if should_skip_line(&line) {
            continue;
        }
        // Check if the line is a node type.
        if let Some(captures) = node_types_regex.captures(&line) {
            let node_type_name = sanitize_term(&captures[1]);
            // Check that the note type is not empty
            if node_type_name.is_empty() {
                continue;
            }
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
    let mut source_node_id = None;
    for line in get_lines_iterator(source_path)?.progress_with(pb) {
        // First of all we check that all is fine with the current line reading attempt.
        let line = line?;
        // Each time we finish to read a page, we can safely increase the current node ID.
        if end_of_page_regex.is_match(&line) {
            source_node_id = None;
            continue;
        }
        // Check if the line contains a title if we don't currently have one.
        if source_node_id.is_none() {
            if let Some(captures) = title_regex.captures(&line) {
                source_node_id = nodes_vocabulary.get(&captures[1].to_owned());
            }
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
            .map(|destination_node_name| sanitize_term(&destination_node_name[1]))
            .filter(|destination_node_name| !destination_node_name.is_empty())
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
            if let Some(source_node_id) = source_node_id {
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
