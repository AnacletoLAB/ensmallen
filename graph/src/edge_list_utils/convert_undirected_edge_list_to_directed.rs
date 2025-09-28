use crate::{utils::ItersWrapper, EdgeFileReader, EdgeFileWriter, EdgeT, Result, WeightT};

/// Create a new directed edge list from a given undirected one by duplicating the undirected edges.
///
/// # Arguments
/// * `original_edge_path`: &str - The path from where to load the original edge list.
/// * `original_edge_list_separator`: Option<char> - Separator to use for the original edge list.
/// * `original_edge_list_header`: Option<bool> - Whether the original edge list has an header.
/// * `original_edge_list_support_balanced_quotes`: Option<bool> - Whether to support balanced quotes while reading the edge list.
/// * `original_sources_column`: Option<String> - The column name to use to load the sources in the original edges list.
/// * `original_sources_column_number`: Option<usize> - The column number to use to load the sources in the original edges list.
/// * `original_destinations_column`: Option<String> - The column name to use to load the destinations in the original edges list.
/// * `original_destinations_column_number`: Option<usize> - The column number to use to load the destinations in the original edges list.
/// * `original_edge_list_edge_type_column`: Option<String> - The column name to use for the edge types in the original edges list.
/// * `original_edge_list_edge_type_column_number`: Option<usize> - The column number to use for the edge types in the original edges list.
/// * `original_weights_column`: Option<String> - The column name to use for the weights in the original edges list.
/// * `original_weights_column_number`: Option<usize> - The column number to use for the weights in the original edges list.
/// * `target_edge_path`: &str - The path from where to load the target edge list. This must be different from the original edge list path.
/// * `target_edge_list_separator`: Option<char> - Separator to use for the target edge list. If None, the one provided from the original edge list will be used.
/// * `target_edge_list_header`: Option<bool> - Whether the target edge list has an header. If None, the one provided from the original edge list will be used.
/// * `target_sources_column`: Option<String> - The column name to use to load the sources in the target edges list. If None, the one provided from the original edge list will be used.
/// * `target_sources_column_number`: Option<usize> - The column number to use to load the sources in the target edges list. If None, the one provided from the original edge list will be used.
/// * `target_destinations_column`: Option<String> - The column name to use to load the destinations in the target edges list. If None, the one provided from the original edge list will be used.
/// * `target_destinations_column_number`: Option<usize> - The column number to use to load the destinations in the target edges list. If None, the one provided from the original edge list will be used.
/// * `target_edge_list_edge_type_column`: Option<String> - The column name to use for the edge types in the target edges list. If None, the one provided from the original edge list will be used.
/// * `target_edge_list_edge_type_column_number`: Option<usize> - The column number to use for the edge types in the target edges list. If None, the one provided from the original edge list will be used.
/// * `target_weights_column`: Option<String> - The column name to use for the weights in the target edges list. If None, the one provided from the original edge list will be used.
/// * `target_weights_column_number`: Option<usize> - The column number to use for the weights in the target edges list. If None, the one provided from the original edge list will be used.
/// * `comment_symbol`: Option<String> - The comment symbol to use within the original edge list.
/// * `default_edge_type`: Option<String> - The default edge type to use within the original edge list.
/// * `default_weight`: Option<WeightT> - The default weight to use within the original edge list.
/// * `max_rows_number`: Option<usize> - The amount of rows to load from the original edge list.
/// * `rows_to_skip`: Option<usize> - The amount of rows to skip from the original edge list.
/// * `number_of_edges`: Option<usize> - The expected number of edges. It will be used for the loading bar.
/// * `skip_edge_types_if_unavailable`: Option<bool> - Whether to automatically skip the edge types if they are not available.
/// * `skip_weights_if_unavailable`: Option<bool> - Whether to automatically skip the weights if they are not available.
/// * `verbose`: Option<bool> - Whether to show the loading bar while processing the file.
/// * `name`: Option<String> - The name of the graph to display in the loading bar.
pub fn convert_undirected_edge_list_to_directed(
    original_edge_path: &str,
    target_edge_path: &str,

    original_edge_list_separator: Option<char>,
    original_edge_list_header: Option<bool>,
    original_edge_list_support_balanced_quotes: Option<bool>,
    original_sources_column: Option<String>,
    original_sources_column_number: Option<usize>,
    original_destinations_column: Option<String>,
    original_destinations_column_number: Option<usize>,
    original_edge_list_edge_type_column: Option<String>,
    original_edge_list_edge_type_column_number: Option<usize>,
    original_weights_column: Option<String>,
    original_weights_column_number: Option<usize>,
    target_edge_list_separator: Option<char>,
    target_edge_list_header: Option<bool>,
    target_sources_column: Option<String>,
    target_sources_column_number: Option<usize>,
    target_destinations_column: Option<String>,
    target_destinations_column_number: Option<usize>,
    target_edge_list_edge_type_column: Option<String>,
    target_edge_list_edge_type_column_number: Option<usize>,
    target_weights_column: Option<String>,
    target_weights_column_number: Option<usize>,
    comment_symbol: Option<String>,
    default_edge_type: Option<String>,
    default_weight: Option<WeightT>,
    max_rows_number: Option<usize>,
    rows_to_skip: Option<usize>,
    number_of_edges: Option<usize>,
    skip_edge_types_if_unavailable: Option<bool>,
    skip_weights_if_unavailable: Option<bool>,
    verbose: Option<bool>,
    name: Option<String>,
) -> Result<EdgeT> {
    if original_edge_path == target_edge_path {
        return Err(concat!(
            "Both the original and the target edge list path ",
            "are set to the same path.\n",
            "It is not possible to write this file inplace, ",
            "as each line would be slightly longer ",
            "than the pre-existing one and would overwrite ",
            "a part of the successive line."
        )
        .to_string());
    }
    let name = name.unwrap_or("Graph".to_owned());
    let file_reader = EdgeFileReader::new(original_edge_path)?
        .set_comment_symbol(comment_symbol)?
        .set_max_rows_number(max_rows_number)?
        .set_rows_to_skip(rows_to_skip)?
        .set_header(original_edge_list_header)?
        .set_support_balanced_quotes(original_edge_list_support_balanced_quotes)
        .set_separator(original_edge_list_separator)?
        .set_default_edge_type(default_edge_type)
        .set_default_weight(default_weight)?
        .set_destinations_column(original_destinations_column.clone())?
        .set_destinations_column_number(original_destinations_column_number)?
        .set_sources_column(original_sources_column.clone())?
        .set_sources_column_number(original_sources_column_number)?
        .set_edge_types_column(original_edge_list_edge_type_column.clone())?
        .set_edge_types_column_number(original_edge_list_edge_type_column_number)?
        .set_weights_column(original_weights_column.clone())?
        .set_weights_column_number(original_weights_column_number)?
        .set_parallel(Some(false))
        .set_skip_edge_types_if_unavailable(skip_edge_types_if_unavailable)
        .set_skip_weights_if_unavailable(skip_weights_if_unavailable)
        // To avoid a duplicated loading bar.
        .set_verbose(verbose.map(|verbose| verbose && number_of_edges.is_none()))
        .set_graph_name(name);
    let file_writer = EdgeFileWriter::new(target_edge_path)
        .set_destinations_column(target_destinations_column.or(original_destinations_column))
        .set_destinations_column_number(
            target_destinations_column_number
                .or(Some(file_reader.get_destinations_column_number())),
        )
        .set_sources_column(target_sources_column.or(original_sources_column))
        .set_sources_column_number(
            target_sources_column_number.or(Some(file_reader.get_sources_column_number())),
        )
        .set_edge_types_column(
            target_edge_list_edge_type_column.or(original_edge_list_edge_type_column),
        )
        .set_edge_types_column_number(
            target_edge_list_edge_type_column_number.or(file_reader.get_edge_types_column_number()),
        )
        .set_weights_column(target_weights_column.or(original_weights_column))
        .set_weights_column_number(
            target_weights_column_number.or(file_reader.get_weights_column_number()),
        )
        .set_separator(target_edge_list_separator.or(Some(file_reader.get_separator())))?
        .set_numeric_node_ids(Some(false))
        .set_numeric_edge_type_ids(Some(false))
        .set_verbose(verbose)
        .set_header(target_edge_list_header.or(Some(file_reader.has_header())));
    let lines_iterator = file_reader.read_lines()?;
    let lines_iterator = match lines_iterator {
        ItersWrapper::Parallel(_) => unreachable!("This is not meant to run in parallel."),
        ItersWrapper::Sequential(i) => i,
    };
    let mut new_number_of_edges = 0;
    file_writer.dump_iterator(
        // We do not care to be exact here: if the graph does not contain
        // selfloops the value will be correct.
        number_of_edges.map(|number_of_edges| number_of_edges * 2),
        lines_iterator
            // Removing eventual errors.
            .filter_map(|line| line.ok())
            // Processing line
            .flat_map(|(_, (src_name, dst_name, edge_type, weight))| {
                let weight = if weight.is_nan() { None } else { Some(weight) };
                // In most well formed graphs, there should be a small
                // percentage of selfloops
                if src_name == dst_name {
                    new_number_of_edges += 1;
                    vec![(0, 0, src_name, 0, dst_name, None, edge_type, weight)]
                } else {
                    new_number_of_edges += 2;
                    vec![
                        (
                            0,
                            0,
                            src_name.clone(),
                            0,
                            dst_name.clone(),
                            None,
                            edge_type.clone(),
                            weight,
                        ),
                        (0, 0, dst_name, 0, src_name, None, edge_type, weight),
                    ]
                }
            }),
    )?;
    Ok(new_number_of_edges)
}
