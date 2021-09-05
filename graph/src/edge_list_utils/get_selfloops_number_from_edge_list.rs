use crate::{EdgeFileReader, EdgeT, Result};

/// Return number of selfloops in the given edge list.
///
/// # Arguments
/// * `path`: &str - The path from where to load the edge list.
/// * `separator`: Option<String> - The separator for the rows in the edge list.
/// * `header`: Option<bool> - Whether the edge list has an header.
/// * `sources_column`: Option<String> - The column name to use for the source nodes.
/// * `sources_column_number`: Option<usize> - The column number to use for the source nodes.
/// * `destinations_column`: Option<String> - The column name to use for the destination nodes.
/// * `destinations_column_number`: Option<usize> - The column number to use for the destination nodes.
/// * `comment_symbol`: Option<String> - The comment symbol to use for the lines to skip.
/// * `max_rows_number`: Option<usize> - The number of rows to read at most. Note that this parameter is ignored when reading in parallel.
/// * `rows_to_skip`: Option<usize> - Number of rows to skip in the edge list.
/// * `edges_number`: Option<EdgeT> - Number of edges in the edge list.
/// * `load_edge_list_in_parallel`: Option<bool> - Whether to execute the task in parallel or sequential. Generally, parallel is preferable.
/// * `verbose`: Option<bool> - Whether to show the loading bar while processing the file.
/// * `name`: Option<String> - The name of the graph to display in the loading bar.
///
pub fn get_selfloops_number_from_edge_list(
    path: &str,
    separator: Option<String>,
    header: Option<bool>,
    sources_column: Option<String>,
    sources_column_number: Option<usize>,
    destinations_column: Option<String>,
    destinations_column_number: Option<usize>,
    comment_symbol: Option<String>,
    max_rows_number: Option<usize>,
    rows_to_skip: Option<usize>,
    edges_number: Option<EdgeT>,
    load_edge_list_in_parallel: Option<bool>,
    verbose: Option<bool>,
    name: Option<String>,
) -> Result<EdgeT> {
    let name = name.unwrap_or("Graph".to_owned());
    let file_reader = EdgeFileReader::new(path)?
        .set_comment_symbol(comment_symbol)?
        .set_rows_to_skip(rows_to_skip)?
        .set_header(header)?
        .set_max_rows_number(max_rows_number)?
        .set_separator(separator)?
        .set_destinations_column(destinations_column)?
        .set_destinations_column_number(destinations_column_number)?
        .set_sources_column(sources_column)?
        .set_sources_column_number(sources_column_number)?
        .set_parallel(load_edge_list_in_parallel)
        .set_edges_number(edges_number)
        .set_verbose(verbose)
        .set_graph_name(name);

    let total_selfloops = file_reader
        .read_lines()?
        // Removing eventual errors.
        .filter_map(|line| line.ok())
        .map(|(_, (src_name, dst_name, _, _))| (src_name == dst_name) as EdgeT)
        .sum::<EdgeT>();
    Ok(total_selfloops)
}
