use crate::{EdgeFileReader, EdgeT, Result};

/// Return minimum and maximum node number from given numeric edge list.
///
/// # Arguments
/// * `path`: &str - The path from where to load the edge list.
/// * `separator`: Option<char> - The separator for the rows in the edge list.
/// * `header`: Option<bool> - Whether the edge list has an header.
/// * `support_balanced_quotes`: Option<bool> - Whether to support balanced quotes.
/// * `sources_column`: Option<String> - The column name to use for the source nodes.
/// * `sources_column_number`: Option<usize> - The column number to use for the source nodes.
/// * `destinations_column`: Option<String> - The column name to use for the destination nodes.
/// * `destinations_column_number`: Option<usize> - The column number to use for the destination nodes.
/// * `comment_symbol`: Option<String> - The comment symbol to use for the lines to skip.
/// * `max_rows_number`: Option<usize> - The number of rows to read at most. Note that this parameter is ignored when reading in parallel.
/// * `rows_to_skip`: Option<usize> - Number of rows to skip in the edge list.
/// * `edges_number`: Option<EdgeT> - Number of edges in the edge list.
/// * `load_edge_list_in_parallel`: Option<bool> - Whether to execute the task in parallel or sequential. Generally, parallel is preferable.
/// * `remove_chevrons`: Option<bool> - Whether remove chevrons while reading elements.
/// * `remove_spaces`: Option<bool> - Whether remove spaces while reading elements.
/// * `verbose`: Option<bool> - Whether to show the loading bar while processing the file.
/// * `name`: Option<String> - The name of the graph to display in the loading bar.
///
/// # Raises
/// * If there are problems with the edge list file.
/// * If the elements in the edge list are not numeric.
/// * If the edge list is empty.
pub fn get_minmax_node_from_numeric_edge_list(
    path: &str,
    separator: Option<char>,
    header: Option<bool>,
    support_balanced_quotes: Option<bool>,
    sources_column: Option<String>,
    sources_column_number: Option<usize>,
    destinations_column: Option<String>,
    destinations_column_number: Option<usize>,
    comment_symbol: Option<String>,
    max_rows_number: Option<usize>,
    rows_to_skip: Option<usize>,
    edges_number: Option<EdgeT>,
    load_edge_list_in_parallel: Option<bool>,
    remove_chevrons: Option<bool>,
    remove_spaces: Option<bool>,
    verbose: Option<bool>,
    name: Option<String>,
) -> Result<(EdgeT, EdgeT, EdgeT)> {
    let name = name.unwrap_or("Graph".to_owned());
    let file_reader = EdgeFileReader::new(path)?
        .set_comment_symbol(comment_symbol)?
        .set_header(header)?
        .set_support_balanced_quotes(support_balanced_quotes)
        .set_max_rows_number(max_rows_number)?
        .set_rows_to_skip(rows_to_skip)?
        .set_separator(separator)?
        .set_destinations_column(destinations_column)?
        .set_destinations_column_number(destinations_column_number)?
        .set_sources_column(sources_column)?
        .set_sources_column_number(sources_column_number)?
        .set_parallel(load_edge_list_in_parallel)
        .set_number_of_edges(edges_number)
        .set_verbose(verbose)
        .set_graph_name(name)
        .set_remove_chevrons(remove_chevrons)
        .set_remove_spaces(remove_spaces);

    let (min, max, edges_number) = file_reader
        .read_lines()?
        // Removing eventual errors.
        .filter_map(|line| line.ok())
        .map(
            |(_, (src_name, dst_name, _, _))| match src_name.parse::<EdgeT>() {
                Ok(src_id) => match dst_name.parse::<EdgeT>() {
                    Ok(dst_id) => Ok((dst_id.min(src_id), dst_id.max(src_id), 1)),
                    Err(_) => Err(format!(
                        concat!(
                            "While searching for the minimum and maximum node ID, ",
                            "we found a non-numeric destination node ID: {}."
                        ),
                        dst_name
                    )),
                },
                Err(_) => Err(format!(
                    concat!(
                        "While searching for the minimum and maximum node ID, ",
                        "we found a non-numeric source node ID: {}."
                    ),
                    src_name
                )),
            },
        )
        .reduce(
            || Ok((EdgeT::MAX, 0 as EdgeT, 0)),
            |line1: Result<(EdgeT, EdgeT, EdgeT)>, line2: Result<(EdgeT, EdgeT, EdgeT)>| match (
                line1, line2,
            ) {
                (Ok((min1, max1, total_edges1)), Ok((min2, max2, total_edges2))) => {
                    Ok((min1.min(min2), max1.max(max2), total_edges1 + total_edges2))
                }
                (Err(e), _) => Err(e),
                (_, Err(e)) => Err(e),
            },
        )?;
    if min > max {
        return Err("The provided edge list was empty.".to_string());
    }
    Ok((min, max, edges_number))
}
