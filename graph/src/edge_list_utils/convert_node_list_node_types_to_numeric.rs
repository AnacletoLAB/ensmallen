use crate::{
    parse_types, utils::ItersWrapper, NodeFileReader, NodeFileWriter, NodeT, NodeTypeT, Result,
    TypeFileReader, TypeFileWriter, Vocabulary,
};

pub fn convert_node_list_node_types_to_numeric(
    original_node_type_path: Option<String>,
    original_node_type_list_separator: Option<String>,
    original_node_types_column_number: Option<usize>,
    original_node_types_column: Option<String>,
    original_node_types_ids_column_number: Option<usize>,
    original_node_types_ids_column: Option<String>,
    node_types_number: Option<NodeTypeT>,
    original_numeric_node_type_ids: Option<bool>,
    original_minimum_node_type_id: Option<NodeTypeT>,
    original_node_type_list_header: Option<bool>,
    original_node_type_list_rows_to_skip: Option<usize>,
    original_node_type_list_is_correct: Option<bool>,
    original_node_type_list_max_rows_number: Option<usize>,
    original_node_type_list_comment_symbol: Option<String>,
    original_load_node_type_list_in_parallel: Option<bool>,

    target_node_type_list_path: Option<String>,
    target_node_type_list_separator: Option<String>,
    target_node_type_list_header: Option<bool>,
    target_node_type_list_node_types_column: Option<String>,
    target_node_type_list_node_types_column_number: Option<usize>,
    target_node_types_ids_column: Option<String>,
    target_node_types_ids_column_number: Option<usize>,

    original_node_path: String,
    original_node_list_separator: Option<String>,
    original_node_list_header: Option<bool>,
    node_list_rows_to_skip: Option<usize>,
    node_list_is_correct: Option<bool>,
    node_list_max_rows_number: Option<usize>,
    node_list_comment_symbol: Option<String>,
    default_node_type: Option<String>,
    original_nodes_column_number: Option<usize>,
    original_nodes_column: Option<String>,
    original_node_types_separator: Option<String>,
    original_node_list_node_types_column_number: Option<usize>,
    original_node_list_node_types_column: Option<String>,
    original_node_ids_column: Option<String>,
    original_node_ids_column_number: Option<usize>,
    original_minimum_node_id: Option<NodeT>,
    original_numeric_node_ids: Option<bool>,
    original_node_list_numeric_node_type_ids: Option<bool>,
    original_skip_node_types_if_unavailable: Option<bool>,
    original_load_node_list_in_parallel: Option<bool>,

    target_node_path: String,
    target_node_list_separator: Option<String>,
    target_node_list_header: Option<bool>,
    target_nodes_column_number: Option<usize>,
    target_nodes_column: Option<String>,
    target_node_types_separator: Option<String>,
    target_node_list_node_types_column_number: Option<usize>,
    target_node_list_node_types_column: Option<String>,
    target_node_ids_column: Option<String>,
    target_node_ids_column_number: Option<usize>,
    nodes_number: Option<NodeT>,
) -> Result<NodeTypeT> {
    let mut node_types: Vocabulary<NodeTypeT> =
        if let Some(original_node_type_path) = original_node_type_path {
            let node_type_file_reader = TypeFileReader::new(Some(original_node_type_path))?
                .set_comment_symbol(original_node_type_list_comment_symbol)?
                .set_header(original_node_type_list_header)?
                .set_max_rows_number(original_node_type_list_max_rows_number)?
                .set_rows_to_skip(original_node_type_list_rows_to_skip)?
                .set_separator(original_node_type_list_separator)?
                .set_type_column_number(original_node_types_column_number)?
                .set_type_column(original_node_types_column)?
                .set_type_ids_column(original_node_types_ids_column)?
                .set_type_ids_column_number(original_node_types_ids_column_number)?
                .set_minimum_type_id(original_minimum_node_type_id)
                .set_numeric_type_ids(original_numeric_node_type_ids)
                .set_csv_is_correct(original_node_type_list_is_correct)?
                .set_types_number(node_types_number)
                .set_parallel(original_load_node_type_list_in_parallel)?;
            let node_types_vocabulary = parse_types(
                node_type_file_reader.read_lines().transpose()?,
                node_types_number,
                Some(node_type_file_reader.has_numeric_type_ids()),
                node_type_file_reader.get_minimum_type_id(),
                true,
            )?
            .unwrap();
            node_types_vocabulary
        } else {
            Vocabulary::new()
        };

    let nodes_reader: NodeFileReader = NodeFileReader::new(Some(original_node_path))?
        .set_comment_symbol(node_list_comment_symbol)?
        .set_header(original_node_list_header)?
        .set_max_rows_number(node_list_max_rows_number)?
        .set_rows_to_skip(node_list_rows_to_skip)?
        .set_separator(original_node_list_separator)?
        .set_nodes_column_number(original_nodes_column_number)?
        .set_node_ids_column(original_node_ids_column.clone())?
        .set_node_ids_column_number(original_node_ids_column_number)?
        .set_nodes_column(original_nodes_column.clone())?
        .set_minimum_node_id(original_minimum_node_id)
        .set_node_types_column_number(original_node_list_node_types_column_number)?
        .set_node_types_column(original_node_list_node_types_column.clone())?
        .set_node_types_separator(original_node_types_separator)?
        .set_skip_node_types_if_unavailable(original_skip_node_types_if_unavailable)?
        .set_default_node_type(default_node_type)
        .set_numeric_node_ids(original_numeric_node_ids)
        .set_numeric_node_type_ids(original_node_list_numeric_node_type_ids)?
        .set_csv_is_correct(node_list_is_correct)?
        .set_nodes_number(nodes_number)
        .set_parallel(original_load_node_list_in_parallel)?;

    let nodes_writer: NodeFileWriter = NodeFileWriter::new(target_node_path)
        .set_separator(target_node_list_separator.or(Some(nodes_reader.get_separator()?)))?
        .set_node_types_separator(
            target_node_types_separator.or(nodes_reader.get_node_types_separator()),
        )?
        .set_header(target_node_list_header.or(Some(nodes_reader.has_header()?)))
        .set_node_ids_column(target_node_ids_column.or(original_node_ids_column))
        .set_node_ids_column_number(
            target_node_ids_column_number.or(nodes_reader.get_node_ids_column_number()),
        )
        .set_node_types_column(
            target_node_list_node_types_column.or(original_node_list_node_types_column),
        )
        .set_node_types_column_number(
            target_node_list_node_types_column_number
                .or(nodes_reader.get_node_types_column_number()),
        )
        .set_nodes_column(target_nodes_column.or(original_nodes_column))
        .set_nodes_column_number(
            target_nodes_column_number.or(nodes_reader.get_nodes_column_number()),
        )
        .set_numeric_node_type_ids(Some(true));

    let lines_iterator = nodes_reader.read_lines().unwrap()?;
    let lines_iterator = match lines_iterator {
        ItersWrapper::Parallel(_) => unreachable!("This is not meant to run in parallel."),
        ItersWrapper::Sequential(i) => i,
    };

    nodes_writer.dump_iterator(
        nodes_number.map(|nodes_number| nodes_number as usize),
        lines_iterator.filter_map(|line| line.ok()).enumerate().map(
            |(line_number, (_, (node_name, maybe_node_type_names)))| {
                (
                    line_number as NodeT,
                    node_name,
                    maybe_node_type_names.as_ref().map(|node_type_names| {
                        node_type_names
                            .iter()
                            .cloned()
                            .map(|node_type_name| unsafe {
                                node_types.unchecked_insert(node_type_name)
                            })
                            .collect()
                    }),
                    maybe_node_type_names,
                )
            },
        ),
    )?;

    if let Some(target_node_type_list_path) = target_node_type_list_path {
        let node_type_writer = TypeFileWriter::new(target_node_type_list_path)
            .set_separator(target_node_type_list_separator)?
            .set_header(target_node_type_list_header)
            .set_type_ids_column(target_node_type_list_node_types_column)
            .set_type_ids_column_number(target_node_type_list_node_types_column_number)
            .set_types_column(target_node_types_ids_column)
            .set_types_column_number(target_node_types_ids_column_number);

        node_type_writer.dump_iterator(
            Some(node_types.len()),
            node_types
                .iter_keys()
                .enumerate()
                .map(|(node_type_id, node_type_name)| (node_type_id as NodeTypeT, node_type_name)),
        )?;
    }

    Ok(node_types.len() as NodeTypeT)
}
