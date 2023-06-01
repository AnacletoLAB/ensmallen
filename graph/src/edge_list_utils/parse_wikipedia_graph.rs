use crate::{
    compute_hash, get_rows_number, sort_numeric_edge_list_inplace, utils::get_loading_bar,
    EdgeFileWriter, EdgeT, EdgeTypeT, NodeFileWriter, NodeT, NodeTypeT, Result, TypeFileWriter,
    Vocabulary,
};
use indicatif::ProgressIterator;
use lazy_static::lazy_static;
use log::info;
#[cfg(target_os = "linux")]
use nix::fcntl::*;
use rayon::iter::IntoParallelRefIterator;
use rayon::iter::ParallelIterator;
use regex::Captures;
use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
#[cfg(target_os = "linux")]
use std::os::unix::io::AsRawFd;

const COMMENT_SYMBOLS: &[&str] = &["&lt;", "{", "}", "|", "----", "!", "=", "<"];
// Then we define the regex to extract the node types.
const CATEGORIES: &[&str] = &[
    "category",
    "categoria",
    "категория",
    "kategória",
    "kategorie",
    "κατηγορία",
    "категорія",
    "categoría",
    "luokka",
    "kategori",
    "კატეგორია",
    "분류",
    "kategorija",
];
const SPECIAL_NODE_STARTERS: &[&str] = &[
    "image:",
    "immagine:",
    "user:",
    "en:user:",
    "en:user:",
    "file:",
    "file",
    "media:",
    "template:",
    "commons:",
    "mediawiki:",
    "progetto:",
    "wiktionary:",
    "wikisource:",
    "wikispecies:",
    "portal talk:",
    "list of",
    "timedtext:",
    "talk:",
    "imdbtitle:",
    "imdbname:",
    "iarchive:",
    "portal:",
    "portale:",
    "module:",
    "draft:",
    "wikt:",
    "list of acronyms:",
    "wikipedia:",
    "en:wikipedia:",
    "wp:",
    "en:wp:",
    "help:",
    "user talk:",
    "user talk",
    "user_talk:",
    "special:",
    "{",
    "=",
];

const SPECIAL_NODES: &[&str] = &["/", "../", "...", "v", "e", "t"];

/// Returns boolean represing whether the given candidate node is a special node.
///
/// # Arguments
/// `candidate_node`: &str - Candidate node to check.
fn is_special_node(candidate_node: &str) -> bool {
    candidate_node.len() < 2
        || candidate_node.parse::<f64>().is_ok()
        || SPECIAL_NODES
            .iter()
            .any(|special_node| *special_node == candidate_node)
        || SPECIAL_NODE_STARTERS
            .iter()
            .chain(CATEGORIES.iter())
            .any(|&starter| candidate_node.starts_with(starter))
}

lazy_static! {
    static ref LINE_SANITIZER_CURLY_BRACES_REMOVER: Regex = Regex::new(r"\{\{[^\}]+?\}\}").unwrap();
    static ref LINE_SANITIZER_SQUARE_BRACES_REMOVER: Regex =
        Regex::new(r"\[\[(?P<a>[^\]]+?)(?:\|[^\]]+?)?\]\]").unwrap();
    static ref LINE_SANITIZER_ANGULAR_BRACES_REMOVER: Regex = Regex::new(r"&lt;.+?&gt;").unwrap();
    static ref LINE_SANITIZER_SPACES_REMOVER: Regex = Regex::new(r"\s+").unwrap();
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
            Ok(l)=>Ok(l),
            Err(_)=>Err("There might have been an I/O error or the line could contains bytes that are not valid UTF-8".to_string()),
        }
    }))
}

/// Remove metadata and other symbols from the text in a wikipedia page
fn sanitize_paragraph(mut line: String) -> String {
    line = LINE_SANITIZER_CURLY_BRACES_REMOVER
        .replace_all(&line, "")
        .to_string();
    line = LINE_SANITIZER_ANGULAR_BRACES_REMOVER
        .replace_all(&line, "")
        .to_string();
    line = LINE_SANITIZER_SQUARE_BRACES_REMOVER
        .replace_all(&line, "$a")
        .to_string();
    let x: &[_] = &['[', ']', '*', '"'];
    line.remove_matches(x);
    line.remove_matches("&quot;");
    line.remove_matches("</text>");
    line = LINE_SANITIZER_SPACES_REMOVER
        .replace_all(&line, " ")
        .to_string();
    line
}

// Return provided term with partial sanitization which still allows for check for special nodes.
fn sanitize_term(mut term: String) -> String {
    let x: &[_] = &['[', ']', '*', ':', ' ', '"'];
    term = term.replace("&#039;", "'");
    term.remove_matches("&quot;");
    term.remove_matches("\t");
    term.trim_matches(x).to_lowercase()
}

/// TODO: write the docstring
pub fn parse_wikipedia_graph(
    source_path: &str,
    edge_path: &str,
    node_path: &str,
    node_type_path: &str,
    edge_type_path: &str,
    node_list_separator: char,
    node_type_list_separator: char,
    edge_type_list_separator: char,
    node_types_separator: &str,
    nodes_column: &str,
    node_types_column: &str,
    node_list_node_types_column: &str,
    edge_types_column: &str,
    node_descriptions_column: &str,
    edge_list_separator: char,
    directed: bool,
    sort_temporary_directory: Option<String>,
    compute_node_description: Option<bool>,
    keep_nodes_without_descriptions: Option<bool>,
    keep_nodes_without_categories: Option<bool>,
    keep_interwikipedia_nodes: Option<bool>,
    keep_external_nodes: Option<bool>,
    verbose: Option<bool>,
) -> Result<(NodeTypeT, NodeT, EdgeT)> {
    let compute_node_description = compute_node_description.unwrap_or(true);
    let keep_external_nodes = keep_external_nodes.unwrap_or(true);
    let keep_interwikipedia_nodes = keep_interwikipedia_nodes.unwrap_or(true);
    let keep_nodes_without_descriptions = keep_nodes_without_descriptions.unwrap_or(true);
    let keep_nodes_without_categories = keep_nodes_without_categories.unwrap_or(true);
    let mut redirect_hashmap: HashMap<u64, String> = HashMap::new();
    let verbose = verbose.unwrap_or(true);
    let mut node_types_vocabulary: Vocabulary<NodeTypeT> = Vocabulary::new(false, "Node types".to_string());
    let mut nodes_vocabulary: Vocabulary<NodeT> = Vocabulary::new(false, "Nodes".to_string());
    let edge_types = ["internal_wiki_link", "external_wiki_link", "websites_link"];
    let node_types = ["internal_wiki", "external_wiki", "websites"];
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

    for node_type in node_types {
        node_types_writer.write_line(
            &mut node_types_stream,
            unsafe { node_types_vocabulary.unchecked_insert(node_type.to_string()) },
            node_type.to_string(),
        )?;
    }

    let edge_types_writer = TypeFileWriter::new(edge_type_path)
        .set_separator(Some(edge_type_list_separator))?
        .set_types_column(Some(edge_types_column));

    edge_types_writer.dump_iterator(
        Some(edge_types.len()),
        edge_types
            .iter()
            .enumerate()
            .map(|(edge_type_id, &edge_type_name)| {
                (edge_type_id as EdgeTypeT, edge_type_name.to_string())
            }),
    )?;

    let edges_writer = EdgeFileWriter::new(edge_path)
        .set_sources_column_number(Some(0))
        .set_destinations_column_number(Some(1))
        .set_edge_types_column_number(Some(2))
        .set_separator(Some(edge_list_separator))?
        .set_numeric_node_ids(Some(true))
        .set_numeric_edge_type_ids(Some(true))
        .set_header(Some(false));

    let mut edges_stream = edges_writer.start_writer()?;

    // Create the required regex.
    // First we create the regex to recognize titles.
    let title_regex = Regex::new(r"^<title>([^<]+)</title>$").unwrap();
    let redirect_title_regex = Regex::new(r#"^<redirect title="([^"]+)"\s*/>$"#).unwrap();
    // Then we create the regex to recognize the end of a page.
    let end_of_page_regex = Regex::new(r"^</page>$").unwrap();
    // Then we define the regex to extract the destination nodes.
    let internal_destination_nodes_regex = Regex::new(r"\[\[([^\]]+?)(?:\|[^\]]+?)?\]\]").unwrap();
    let external_destination_nodes_regex =
        Regex::new(r"[^\[]\[([^\]]+?)(?:\|[^\]]+?)?\][^\]]").unwrap();

    let node_types_regex = Regex::new(&format!(
        r"(?i)^\[\[[^\]]*?(?:{}):([^\]\|]+?)(?:\|[^\]]*?)?\]\]",
        CATEGORIES.join("|")
    ))
    .unwrap();

    info!("Starting to build the nodes and node types list.");
    let pb = get_loading_bar(
        verbose,
        "Building node list",
        std::fs::metadata(source_path)
            .map_err(|x| x.to_string())?
            .len() as _,
    );
    // Initialize the current node name.
    let mut current_node_name: Option<String> = None;
    let mut current_node_types: Vec<NodeTypeT> = vec![0];
    let mut current_node_description: Vec<String> = Vec::new();
    let mut current_line_number: usize = 0;
    // Start to parse and write the node list and node type list.
    for line in get_lines_iterator(source_path)? {
        // We increase the current line number
        current_line_number += 1;
        // First of all we check that all is fine with the current line reading attempt.
        let line = line?;
        pb.inc(line.len() as _);
        // sanitize the string
        let line = line.trim().to_string();
        // We check if the current page is finished.
        if end_of_page_regex.is_match(&line) {
            if let Some(current_node_name) = current_node_name {
                let description = sanitize_paragraph(current_node_description.join(" "));
                if (!description.is_empty() || keep_nodes_without_descriptions)
                    && (current_node_types.len() > 1 || keep_nodes_without_categories)
                {
                    let (current_node_id, was_already_present) =
                        nodes_vocabulary.insert(&current_node_name)?;
                    if !was_already_present {
                        nodes_writer.write_line(
                            &mut nodes_stream,
                            current_node_id,
                            current_node_name,
                            Some(current_node_types),
                            None,
                            if compute_node_description {
                                Some(description)
                            } else {
                                None
                            },
                        )?;
                    }
                }
            }
            // We write the node to the node list file.
            // Finally we restore the current varibales to defaults.
            current_node_name = None;
            current_node_types = vec![0];
            current_node_description = Vec::new();
        }
        // Check if the line contains a title if we don't currently have one.
        if current_node_name.is_none() {
            if let Some(captures) = title_regex.captures(&line) {
                let node_name = sanitize_term(captures[1].trim().to_string());
                // Check if the node is a semantic node for website content
                // If so, we skip it.
                if is_special_node(&node_name) {
                    continue;
                }
                current_node_name = Some(node_name);
            }
            continue;
        }
        if let Some(node_name) = &mut current_node_name {
            if let Some(captures) = redirect_title_regex.captures(&line) {
                let mut redirect_node_name = sanitize_term(captures[1].to_string());
                if !is_special_node(&redirect_node_name) {
                    redirect_node_name = redirect_hashmap
                        .get(&compute_hash(&redirect_node_name))
                        .unwrap_or(&redirect_node_name)
                        .to_owned();
                    if redirect_node_name != *node_name {
                        redirect_hashmap.insert(compute_hash(&node_name), redirect_node_name);
                    }
                }
                current_node_name = None;
                continue;
            }
        }
        // We check if the line should be skipped
        if should_skip_line(&line) {
            continue;
        }
        // Check if the line is a node type.
        if let Some(captures) = node_types_regex.captures(&line) {
            let node_type_name = sanitize_term(captures[1].to_string());
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
                node_types_writer.write_line(
                    &mut node_types_stream,
                    node_type_id,
                    node_type_name,
                )?;
            }
            current_node_types.push(node_type_id);
            continue;
        }
        if compute_node_description {
            current_node_description.push(line);
        }
    }
    pb.finish();

    info!(
        "Renormalize redictions, specifically {} redirections were detected.",
        redirect_hashmap.len().to_string()
    );
    let mut adjusted_redirect: HashMap<u64, String> = redirect_hashmap
        .par_iter()
        .filter(|(_, node_name)| !redirect_hashmap.contains_key(&compute_hash(node_name)))
        .map(|(key, value)| (*key, value.clone()))
        .collect();
    redirect_hashmap = redirect_hashmap
        .par_iter()
        .filter(|(_, node_name)| redirect_hashmap.contains_key(&compute_hash(node_name)))
        .map(|(key, value)| (*key, value.clone()))
        .collect();
    let mut number_of_adjusted_redirections: usize = 0;
    let pb = get_loading_bar(verbose, "Adjusting redirections", redirect_hashmap.len());

    while let Some(&source_hash) = redirect_hashmap
        .keys()
        .take(1)
        .cloned()
        .collect::<Vec<_>>()
        .first()
    {
        number_of_adjusted_redirections += 1;
        let mut explored_nodes: Vec<u64> = vec![source_hash];
        pb.inc(1);

        let mut node_name = redirect_hashmap.remove(&source_hash).unwrap();
        let mut node_name_hash = compute_hash(&node_name);

        'inner: loop {
            // if the next node is also a reference just go to the next node
            // this also implicitely break cycles
            if let Some(next_node_name) = redirect_hashmap.remove(&node_name_hash) {
                node_name = next_node_name;
                node_name_hash = compute_hash(&node_name);
                explored_nodes.push(node_name_hash);
                pb.inc(1);
            } else {
                break 'inner;
            }
        }

        // if the next is a node already solved, we can just propagate the result
        if let Some(result_name) = adjusted_redirect.get(&node_name_hash) {
            node_name = result_name.clone();
        }
        // update the loading bar
        // the node is a destination, so we can just propagate it backward
        for node_hash in explored_nodes {
            adjusted_redirect.insert(node_hash, node_name.clone());
        }
    }
    pb.finish();

    if number_of_adjusted_redirections > 0 {
        info!(
            "Adjusted {} redirections.",
            number_of_adjusted_redirections.to_string()
        );
    }
    // Reset the buffer
    info!("Starting to build the edge list.");
    let pb = get_loading_bar(verbose, "Building edge list", current_line_number);
    let mut source_node_id = None;
    for line in get_lines_iterator(source_path)?.progress_with(pb) {
        // First of all we check that all is fine with the current line reading attempt.
        let line = line?.trim().to_string();
        // Each time we finish to read a page, we can safely increase the current node ID.
        if end_of_page_regex.is_match(&line) {
            source_node_id = None;
            continue;
        }
        // Check if the line contains a title if we don't currently have one.git log
        if source_node_id.is_none() {
            if let Some(captures) = title_regex.captures(&line) {
                let node_name = sanitize_term(captures[1].to_string());
                if redirect_hashmap.contains_key(&compute_hash(&node_name)) {
                    continue;
                }
                source_node_id = nodes_vocabulary.get(&node_name);
            }
            continue;
        }
        // We check if the line should be skipped
        if should_skip_line(&line) || node_types_regex.is_match(&line) {
            continue;
        }
        let external_iterator: Box<dyn Iterator<Item = (Captures, EdgeTypeT)>> =
            if keep_external_nodes {
                Box::new(
                    external_destination_nodes_regex
                        .captures_iter(&line)
                        .into_iter()
                        .map(|capture| (capture, 2)),
                )
            } else {
                Box::new(::std::iter::empty())
            };
        // Finally, we parse the line and extract the destination nodes.
        for (mut destination_node_name, mut edge_type_id) in internal_destination_nodes_regex
            .captures_iter(&line)
            .into_iter()
            .map(|capture| (capture, 0))
            .chain(external_iterator)
            .map(|(destination_node_name, edge_type_id)| {
                (
                    sanitize_term(destination_node_name[1].to_owned()),
                    edge_type_id,
                )
            })
            .filter(|(destination_node_name, _)| !is_special_node(destination_node_name))
        {
            if let Some(new_destination_name) =
                adjusted_redirect.get(&compute_hash(&destination_node_name))
            {
                destination_node_name = new_destination_name.to_string();
            }

            let destination_node_id = if keep_interwikipedia_nodes || keep_external_nodes {
                let (destination_node_id, was_already_present) =
                    nodes_vocabulary.insert(&destination_node_name)?;
                if !was_already_present {
                    if edge_type_id == 0 {
                        edge_type_id = 1;
                    }
                    nodes_writer.write_line(
                        &mut nodes_stream,
                        destination_node_id,
                        destination_node_name,
                        Some(vec![edge_type_id as NodeTypeT]),
                        None,
                        None,
                    )?;
                }
                destination_node_id
            } else {
                if let Some(destination_node_id) = nodes_vocabulary.get(&destination_node_name) {
                    destination_node_id
                } else {
                    continue;
                }
            };

            if let Some(source_node_id) = source_node_id {
                edges_writer.write_line(
                    &mut edges_stream,
                    0,
                    source_node_id,
                    "".to_string(),
                    destination_node_id,
                    "".to_string(),
                    Some(edge_type_id),
                    None,
                    None,
                )?;
                if !directed && source_node_id != destination_node_id {
                    edges_writer.write_line(
                        &mut edges_stream,
                        0,
                        destination_node_id,
                        "".to_string(),
                        source_node_id,
                        "".to_string(),
                        Some(edge_type_id),
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
        Some(2),
        None,
        None,
        sort_temporary_directory,
    )?;

    info!("Count the lines in the path, that match exactly with the number of edges.");
    let number_of_edges = get_rows_number(edge_path.as_ref())? as EdgeT;

    Ok((
        node_types_vocabulary.len() as NodeTypeT,
        nodes_vocabulary.len() as NodeT,
        number_of_edges,
    ))
}
