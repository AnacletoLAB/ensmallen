use crate::{EdgeFileReader, EdgeT, Result};

/// Return number of selfloops in the given edge list.
pub fn get_selfloops_number_from_edge_list(
    path: &str,
    separator: Option<String>,
    header: Option<bool>,
    sources_column_number: Option<usize>,
    sources_column: Option<String>,
    destinations_column_number: Option<usize>,
    destinations_column: Option<String>,
    comment_symbol: Option<String>,
    max_rows_number: Option<EdgeT>,
    rows_to_skip: Option<usize>,
    edges_number: Option<EdgeT>,
    load_edge_list_in_parallel: Option<bool>,
    skip_edge_types_if_unavailable: Option<bool>,
    skip_weights_if_unavailable: Option<bool>,
    verbose: Option<bool>,
    name: Option<String>,
) -> Result<EdgeT> {
    let name = name.unwrap_or("Graph".to_owned());
    let file_reader = EdgeFileReader::new(path)?
        .set_comment_symbol(comment_symbol)?
        .set_separator(separator)?
        .set_destinations_column(destinations_column)?
        .set_destinations_column_number(destinations_column_number)?
        .set_sources_column(sources_column)?
        .set_sources_column_number(sources_column_number)?
        .set_max_rows_number(max_rows_number)
        .set_parallel(load_edge_list_in_parallel)
        .set_rows_to_skip(rows_to_skip)
        .set_edges_number(edges_number)
        .set_skip_edge_types_if_unavailable(skip_edge_types_if_unavailable)
        .set_skip_weights_if_unavailable(skip_weights_if_unavailable)
        .set_verbose(verbose)
        .set_header(header)
        .set_graph_name(name);

    let total_selfloops = file_reader
        .read_lines()?
        // Removing eventual errors.
        .filter_map(|line| line.ok())
        .map(|(_, (src_name, dst_name, _, _))| (src_name == dst_name) as EdgeT)
        .sum::<EdgeT>();
    Ok(total_selfloops)
}
