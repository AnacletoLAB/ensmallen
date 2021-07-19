use crate::{
    utils::ItersWrapper, EdgeFileReader, EdgeFileWriter, EdgeT, NodeT, Result, Vocabulary, WeightT,
};

/// Create a new edge list starting from given one with node IDs densified.
pub fn convert_edge_list_to_numeric_node_ids(
    original_edge_list_path: &str,
    original_edge_list_separator: Option<String>,
    original_edge_list_header: Option<bool>,
    original_edge_list_sources_column_number: Option<usize>,
    original_edge_list_sources_column: Option<String>,
    original_edge_list_destinations_column_number: Option<usize>,
    original_edge_list_destinations_column: Option<String>,
    original_edge_list_edge_type_column: Option<String>,
    original_edge_list_edge_type_column_number: Option<usize>,
    original_edge_list_weights_column: Option<String>,
    original_edge_list_weights_column_number: Option<usize>,
    target_edge_list_path: &str,
    target_edge_list_separator: Option<String>,
    target_edge_list_header: Option<bool>,
    target_edge_list_sources_column_number: Option<usize>,
    target_edge_list_sources_column: Option<String>,
    target_edge_list_destinations_column_number: Option<usize>,
    target_edge_list_destinations_column: Option<String>,
    target_edge_list_edge_type_column: Option<String>,
    target_edge_list_edge_type_column_number: Option<usize>,
    target_edge_list_weights_column: Option<String>,
    target_edge_list_weights_column_number: Option<usize>,
    comment_symbol: Option<String>,
    default_edge_type: Option<String>,
    default_weight: Option<WeightT>,
    max_rows_number: Option<EdgeT>,
    rows_to_skip: Option<usize>,
    skip_edge_types_if_unavailable: Option<bool>,
    skip_weights_if_unavailable: Option<bool>,
    verbose: Option<bool>,
    name: Option<String>,
) -> Result<()> {
    let name = name.unwrap_or("Graph".to_owned());
    let mut nodes: Vocabulary<NodeT> = Vocabulary::new();
    let file_reader = EdgeFileReader::new(original_edge_list_path)?
        .set_comment_symbol(comment_symbol)?
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
        .set_max_rows_number(max_rows_number)
        .set_parallel(Some(false))
        .set_rows_to_skip(rows_to_skip)
        .set_skip_edge_types_if_unavailable(skip_edge_types_if_unavailable)
        .set_skip_weights_if_unavailable(skip_weights_if_unavailable)
        .set_verbose(verbose)
        .set_header(original_edge_list_header)
        .set_graph_name(name);
    let file_writer = EdgeFileWriter::new(target_edge_list_path)
        .set_destinations_column(target_edge_list_destinations_column)
        .set_destinations_column_number(target_edge_list_destinations_column_number)
        .set_sources_column(target_edge_list_sources_column)
        .set_sources_column_number(target_edge_list_sources_column_number)
        .set_edge_types_column(target_edge_list_edge_type_column)
        .set_edge_types_column_number(target_edge_list_edge_type_column_number)
        .set_weights_column(target_edge_list_weights_column)
        .set_weights_column_number(target_edge_list_weights_column_number)
        .set_separator(target_edge_list_separator)
        .set_numeric_node_ids(Some(true))
        .set_header(target_edge_list_header);
    let lines_iterator = file_reader.read_lines()?;
    let lines_iterator = match lines_iterator {
        ItersWrapper::Parallel(_) => unreachable!("This is not meant to run in parallel."),
        ItersWrapper::Sequential(i) => i,
    };
    file_writer.dump_iterator(
        None,
        lines_iterator
            // Removing eventual errors.
            .filter_map(|line| line.ok())
            // Processing line
            .map(
                |(line_number, (src_name, dst_name, edge_type, weight))| unsafe {
                    (
                        line_number as u64,
                        nodes.unchecked_insert(src_name),
                        "".to_owned(),
                        nodes.unchecked_insert(dst_name),
                        "".to_owned(),
                        None,
                        edge_type,
                        if weight.is_nan() { None } else { Some(weight) },
                    )
                },
            ),
    )?;
    Ok(())
}
