use crate::{EdgeFileReader, EdgeT, Result};

/// Return minimum and maximum node number from given numeric edge list.
pub fn get_minmax_node_from_numeric_edge_list(
    original_edge_list_path: &str,
    original_edge_list_separator: Option<String>,
    original_edge_list_header: Option<bool>,
    original_edge_list_sources_column_number: Option<usize>,
    original_edge_list_sources_column: Option<String>,
    original_edge_list_destinations_column_number: Option<usize>,
    original_edge_list_destinations_column: Option<String>,
    comment_symbol: Option<String>,
    max_rows_number: Option<EdgeT>,
    rows_to_skip: Option<usize>,
    edges_number: Option<EdgeT>,
    load_edge_list_in_parallel: Option<bool>,
    skip_edge_types_if_unavailable: Option<bool>,
    skip_weights_if_unavailable: Option<bool>,
    verbose: Option<bool>,
    name: Option<String>,
) -> Result<(EdgeT, EdgeT)> {
    let name = name.unwrap_or("Graph".to_owned());
    let file_reader = EdgeFileReader::new(original_edge_list_path)?
        .set_comment_symbol(comment_symbol)?
        .set_separator(original_edge_list_separator)?
        .set_destinations_column(original_edge_list_destinations_column)?
        .set_destinations_column_number(original_edge_list_destinations_column_number)?
        .set_sources_column(original_edge_list_sources_column)?
        .set_sources_column_number(original_edge_list_sources_column_number)?
        .set_max_rows_number(max_rows_number)
        .set_parallel(load_edge_list_in_parallel)
        .set_rows_to_skip(rows_to_skip)
        .set_edges_number(edges_number)
        .set_skip_edge_types_if_unavailable(skip_edge_types_if_unavailable)
        .set_skip_weights_if_unavailable(skip_weights_if_unavailable)
        .set_verbose(verbose)
        .set_header(original_edge_list_header)
        .set_graph_name(name);

    let (min, max) = file_reader
        .read_lines()?
        // Removing eventual errors.
        .filter_map(|line| line.ok())
        .map(
            |(_, (src_name, dst_name, _, _))| match src_name.parse::<EdgeT>() {
                Ok(src_id) => match dst_name.parse::<EdgeT>() {
                    Ok(dst_id) => Ok((dst_id.min(src_id), dst_id.max(src_id))),
                    Err(_) => Err(format!(
                        "Unable to convert given destination node ID {} to numeric.",
                        dst_name
                    )),
                },
                Err(_) => Err(format!(
                    "Unable to convert given source node ID {} to numeric.",
                    src_name
                )),
            },
        )
        .reduce(
            || Ok((EdgeT::MAX, 0 as EdgeT)),
            |line1: Result<(EdgeT, EdgeT)>, line2: Result<(EdgeT, EdgeT)>| match (line1, line2) {
                (Ok((min1, max1)), Ok((min2, max2))) => Ok((min1.min(min2), max1.max(max2))),
                (Err(e), _) => Err(e),
                (_, Err(e)) => Err(e),
            },
        )?;
    if min > max {
        return Err("The provided edge list was empty.".to_string());
    }
    Ok((min, max))
}
