use crate::{utils::ItersWrapper, EdgeFileReader, Result, WeightT};

/// Return whether the provided edge list contains duplicated edges.
///
/// # Arguments
/// * `original_edge_path`: &str - The path from where to load the original edge list.
/// * `original_edge_list_separator`: Option<char> - Separator to use for the original edge list.
/// * `original_edge_list_header`: Option<bool> - Whether the original edge list has an header.
/// * `original_edge_list_support_balanced_quotes`: Option<bool> - Whether to support balanced quotes.
/// * `original_edge_list_sources_column`: Option<String> - The column name to use to load the sources in the original edges list.
/// * `original_edge_list_sources_column_number`: Option<usize> - The column number to use to load the sources in the original edges list.
/// * `original_edge_list_destinations_column`: Option<String> - The column name to use to load the destinations in the original edges list.
/// * `original_edge_list_destinations_column_number`: Option<usize> - The column number to use to load the destinations in the original edges list.
/// * `original_edge_list_edge_type_column`: Option<String> - The column name to use for the edge types in the original edges list.
/// * `original_edge_list_edge_type_column_number`: Option<usize> - The column number to use for the edge types in the original edges list.
/// * `original_edge_list_weights_column`: Option<String> - The column name to use for the weights in the original edges list.
/// * `original_edge_list_weights_column_number`: Option<usize> - The column number to use for the weights in the original edges list.
/// * `comment_symbol`: Option<String> - The comment symbol to use within the original edge list.
/// * `default_edge_type`: Option<String> - The default edge type to use within the original edge list.
/// * `default_weight`: Option<WeightT> - The default weight to use within the original edge list.
/// * `max_rows_number`: Option<usize> - The amount of rows to load from the original edge list.
/// * `rows_to_skip`: Option<usize> - The amount of rows to skip from the original edge list.
/// * `edges_number`: Option<usize> - The expected number of edges. It will be used for the loading bar.
/// * `skip_edge_types_if_unavailable`: Option<bool> - Whether to automatically skip the edge types if they are not available.
/// * `skip_weights_if_unavailable`: Option<bool> - Whether to automatically skip the weights if they are not available.
/// * `verbose`: Option<bool> - Whether to show the loading bar while processing the file.
/// * `name`: Option<String> - The name of the graph to display in the loading bar.
pub fn has_duplicated_edges_in_edge_list(
    original_edge_path: &str,
    original_edge_list_separator: Option<char>,
    original_edge_list_header: Option<bool>,
    original_edge_list_support_balanced_quotes: Option<bool>,
    original_edge_list_sources_column: Option<String>,
    original_edge_list_sources_column_number: Option<usize>,
    original_edge_list_destinations_column: Option<String>,
    original_edge_list_destinations_column_number: Option<usize>,
    original_edge_list_edge_type_column: Option<String>,
    original_edge_list_edge_type_column_number: Option<usize>,
    original_edge_list_weights_column: Option<String>,
    original_edge_list_weights_column_number: Option<usize>,
    comment_symbol: Option<String>,
    default_edge_type: Option<String>,
    default_weight: Option<WeightT>,
    max_rows_number: Option<usize>,
    rows_to_skip: Option<usize>,
    edges_number: Option<usize>,
    skip_edge_types_if_unavailable: Option<bool>,
    skip_weights_if_unavailable: Option<bool>,
    verbose: Option<bool>,
    name: Option<String>,
) -> Result<bool> {
    let name = name.unwrap_or("Graph".to_owned());
    let file_reader = EdgeFileReader::new(original_edge_path)?
        .set_comment_symbol(comment_symbol)?
        .set_header(original_edge_list_header)?
        .set_support_balanced_quotes(original_edge_list_support_balanced_quotes)
        .set_max_rows_number(max_rows_number)?
        .set_rows_to_skip(rows_to_skip)?
        .set_separator(original_edge_list_separator)?
        .set_default_edge_type(default_edge_type)
        .set_default_weight(default_weight)?
        .set_destinations_column(original_edge_list_destinations_column)?
        .set_destinations_column_number(original_edge_list_destinations_column_number)?
        .set_sources_column(original_edge_list_sources_column)?
        .set_sources_column_number(original_edge_list_sources_column_number)?
        .set_edge_types_column(original_edge_list_edge_type_column)?
        .set_edge_types_column_number(original_edge_list_edge_type_column_number)?
        .set_weights_column(original_edge_list_weights_column)?
        .set_weights_column_number(original_edge_list_weights_column_number)?
        .set_parallel(Some(false))
        .set_skip_edge_types_if_unavailable(skip_edge_types_if_unavailable)
        .set_skip_weights_if_unavailable(skip_weights_if_unavailable)
        // To avoid a duplicated loading bar.
        .set_verbose(verbose.map(|verbose| verbose && edges_number.is_none()))
        .set_graph_name(name);
    let lines_iterator = file_reader.read_lines()?;
    let lines_iterator = match lines_iterator {
        ItersWrapper::Parallel(_) => unreachable!("This is not meant to run in parallel."),
        ItersWrapper::Sequential(i) => i,
    };
    let mut last_line = None;
    let has_duplicated_edges = lines_iterator
    // Removing eventual errors.
    .filter_map(|line| line.ok())
    .any(|(_, line)| {
        last_line
            .replace(line.clone())
            .map_or(false, |last_line| last_line == line)
    });
    Ok(has_duplicated_edges)
}
