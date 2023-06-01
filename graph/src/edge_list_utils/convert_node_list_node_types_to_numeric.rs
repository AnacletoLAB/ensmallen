use crate::{
    parse_types, utils::ItersWrapper, NodeFileReader, NodeFileWriter, NodeT, NodeTypeT, Result,
    TypeFileReader, TypeFileWriter, Vocabulary,
};

/// Converts the node list at given path to numeric saving in stream to file. Furthermore, returns the number of nodes that were written and their node types if any.
///
/// # Arguments
/// * `original_node_type_path`: Option<String> - Path to the original list of node types.
/// * `original_node_type_list_separator`: Option<char> - Separator to be used for the original node type list.
/// * `original_node_types_column_number`: Option<usize> - Number of the node types column to be used for the original node types list.
/// * `original_node_types_column`: Option<String> - Name of the node types column to be used for the original node types list.
/// * `number_of_node_types`: Option<NodeTypeT> - Number of node types present in the provided original list of node types. If provided, it will allow to make assumptions and to load the node types faster.
/// * `original_numeric_node_type_ids`: Option<bool> - Whether to load the node types as numeric.
/// * `original_minimum_node_type_id`: Option<NodeTypeT> - The minimum numeric node type ID. If provided, it will allow for additional assumptions in the creation of the node types vocabulary.
/// * `original_node_type_list_header`: Option<bool> - Whether the provided node type list has a header.
/// * `original_node_type_list_support_balanced_quotes`: Option<bool> - Whether to support balanced quotes while reading the node type list.
/// * `original_node_type_list_rows_to_skip`: Option<usize> - Number of rows to skip before starting to parse the file for the provided node type list file.
/// * `original_node_type_list_is_correct`: Option<bool> - Whether is it safe to assume that the provided node type list is correct.
/// * `original_node_type_list_max_rows_number`: Option<usize> - Maximum number of rows to be read in the provided original node type list.
/// * `original_node_type_list_comment_symbol`: Option<String> - Symbol to be used to skip rows starting with it.
/// * `original_load_node_type_list_in_parallel`: Option<bool> - Whether to load the node type list in parallel.
/// * `target_node_type_list_path`: Option<String> - Path where to store the parsed node types (if any).
/// * `target_node_type_list_separator`: Option<char> - Separator to be used for the target node type list.
/// * `target_node_type_list_header`: Option<bool> - Whether to add header when writing the target node type list.
/// * `target_node_type_list_node_types_column`: Option<String> - Name of the column of the node types in the target node type list.
/// * `target_node_type_list_node_types_column_number`: Option<usize> - Number of the column of the node types in the target node type list.
/// * `original_node_path`: String - Path to the original list of nodes.
/// * `original_node_list_separator`: Option<char> - Separator to be used for rows of the original node list.
/// * `original_node_list_header`: Option<bool> - Whether to expect a header in the original node list file.
/// * `original_node_list_support_balanced_quotes`: Option<bool> - Whether to support balanced quotes while reading the node list.
/// * `node_list_rows_to_skip`: Option<usize> - Number of rows to skip before starting to parse the original node list.
/// * `node_list_max_rows_number`: Option<usize> - Maximum number of rows to read from the origina node list.
/// * `node_list_comment_symbol`: Option<String> - Symbol to use to skip rows starting with it in the original node list.
/// * `default_node_type`: Option<String> - Default node type to be used when none are provided or are missing for some nodes.
/// * `original_nodes_column_number`: Option<usize> - Number of the column for the node name in the original node list.
/// * `original_nodes_column`: Option<String> - Name of the column for the node name in the original node list.
/// * `original_node_types_separator`: Option<char> - Separator to be used for the node types within the original node list.
/// * `original_node_list_node_types_column_number`: Option<usize> - Number of the column for the node types in the original node list.
/// * `original_node_list_node_types_column`: Option<String> - Name of the column for the node types in the original node list.
/// * `original_minimum_node_id`: Option<NodeT> - The minimum numeric node ID. If provided, it will allow for additional assumptions in the creation of the nodes vocabulary.
/// * `original_numeric_node_ids`: Option<bool> - Whether to load the node names as numeric.
/// * `original_node_list_numeric_node_type_ids`: Option<bool> - Whether to load the node type names from the original node list as numeric.
/// * `original_skip_node_types_if_unavailable`: Option<bool> - Whether to skip the node types if the provided node types column is not provided.
/// * `remove_chevrons`: Option<bool> - Whether remove chevrons while reading elements.
/// * `remove_spaces`: Option<bool> - Whether remove spaces while reading elements.
/// * `target_node_path`: String - Path where to store the target node paths.
/// * `target_node_list_separator`: Option<char> - Separator to be used for the target node list.
/// * `target_node_list_header`: Option<bool> - Whether to add an header to the target node list.
/// * `target_nodes_column_number`: Option<usize> - Number of the column where to store the node names.
/// * `target_nodes_column`: Option<String> - Name of the column where to store the node names.
/// * `target_node_types_separator`: Option<char> - Separator to be used for the node types within the target node list.
/// * `target_node_list_node_types_column_number`: Option<usize> - Number for the column with the node type names within the target node list.
/// * `target_node_list_node_types_column`: Option<String> - Name for the column with the node type names within the target node list.
/// * `number_of_nodes`: Option<NodeT> - Number of the nodes in the original node list.
///
pub fn convert_node_list_node_types_to_numeric(
    original_node_path: String,
    target_node_path: String,

    original_node_type_path: Option<String>,
    original_node_type_list_separator: Option<char>,
    original_node_types_column_number: Option<usize>,
    original_node_types_column: Option<String>,
    number_of_node_types: Option<NodeTypeT>,
    original_numeric_node_type_ids: Option<bool>,
    original_minimum_node_type_id: Option<NodeTypeT>,
    original_node_type_list_header: Option<bool>,
    original_node_type_list_support_balanced_quotes: Option<bool>,
    original_node_type_list_rows_to_skip: Option<usize>,
    original_node_type_list_is_correct: Option<bool>,
    original_node_type_list_max_rows_number: Option<usize>,
    original_node_type_list_comment_symbol: Option<String>,
    original_load_node_type_list_in_parallel: Option<bool>,

    target_node_type_list_path: Option<String>,
    target_node_type_list_separator: Option<char>,
    target_node_type_list_header: Option<bool>,
    target_node_type_list_node_types_column: Option<String>,
    target_node_type_list_node_types_column_number: Option<usize>,

    original_node_list_separator: Option<char>,
    original_node_list_header: Option<bool>,
    original_node_list_support_balanced_quotes: Option<bool>,
    node_list_rows_to_skip: Option<usize>,
    node_list_max_rows_number: Option<usize>,
    node_list_comment_symbol: Option<String>,
    default_node_type: Option<String>,
    original_nodes_column_number: Option<usize>,
    original_nodes_column: Option<String>,
    original_node_types_separator: Option<char>,
    original_node_list_node_types_column_number: Option<usize>,
    original_node_list_node_types_column: Option<String>,
    original_minimum_node_id: Option<NodeT>,
    original_numeric_node_ids: Option<bool>,
    original_node_list_numeric_node_type_ids: Option<bool>,
    original_skip_node_types_if_unavailable: Option<bool>,

    remove_chevrons: Option<bool>,
    remove_spaces: Option<bool>,

    target_node_list_separator: Option<char>,
    target_node_list_header: Option<bool>,
    target_nodes_column_number: Option<usize>,
    target_nodes_column: Option<String>,
    target_node_types_separator: Option<char>,
    target_node_list_node_types_column_number: Option<usize>,
    target_node_list_node_types_column: Option<String>,
    number_of_nodes: Option<NodeT>,
) -> Result<(NodeT, Option<NodeTypeT>)> {
    let mut node_types: Vocabulary<NodeTypeT> =
        if let Some(original_node_type_path) = original_node_type_path {
            let node_type_file_reader = TypeFileReader::new(Some(original_node_type_path))?
                .set_comment_symbol(original_node_type_list_comment_symbol)?
                .set_header(original_node_type_list_header)?
                .set_support_balanced_quotes(original_node_type_list_support_balanced_quotes)?
                .set_max_rows_number(original_node_type_list_max_rows_number)?
                .set_rows_to_skip(original_node_type_list_rows_to_skip)?
                .set_separator(original_node_type_list_separator)?
                .set_type_column_number(original_node_types_column_number)?
                .set_type_column(original_node_types_column)?
                .set_minimum_type_id(original_minimum_node_type_id)
                .set_numeric_type_ids(original_numeric_node_type_ids)
                .set_csv_is_correct(original_node_type_list_is_correct)?
                .set_types_number(number_of_node_types)
                .set_parallel(original_load_node_type_list_in_parallel)?
                .set_remove_chevrons(remove_chevrons)
                .set_remove_spaces(remove_spaces);
            let node_types_vocabulary = parse_types(
                node_type_file_reader.read_lines().transpose()?,
                number_of_node_types,
                Some(node_type_file_reader.has_numeric_type_ids()),
                node_type_file_reader.get_minimum_type_id(),
                true,
                original_node_type_list_is_correct,
                "Node types".to_string()
            )?
            .unwrap();
            node_types_vocabulary
        } else {
            Vocabulary::new(true, "Node types".to_string())
        };

    let nodes_reader: NodeFileReader = NodeFileReader::new(Some(original_node_path))?
        .set_comment_symbol(node_list_comment_symbol)?
        .set_header(original_node_list_header)?
        .set_support_balanced_quotes(original_node_list_support_balanced_quotes)?
        .set_max_rows_number(node_list_max_rows_number)?
        .set_rows_to_skip(node_list_rows_to_skip)?
        .set_separator(original_node_list_separator)?
        .set_nodes_column_number(original_nodes_column_number)?
        .set_nodes_column(original_nodes_column.clone())?
        .set_minimum_node_id(original_minimum_node_id)
        .set_skip_node_types_if_unavailable(original_skip_node_types_if_unavailable)?
        .set_node_types_column_number(original_node_list_node_types_column_number.clone())?
        .set_node_types_column(original_node_list_node_types_column.clone())?
        .set_node_types_separator(original_node_types_separator)?
        .set_default_node_type(default_node_type.clone())
        .set_numeric_node_ids(original_numeric_node_ids)
        .set_numeric_node_type_ids(original_node_list_numeric_node_type_ids)?
        .set_number_of_nodes(number_of_nodes)
        .set_parallel(Some(false))?
        .set_remove_chevrons(remove_chevrons)
        .set_remove_spaces(remove_spaces);

    let has_node_types = original_node_list_node_types_column_number.is_some()
        || original_node_list_node_types_column.is_some()
        || default_node_type.is_some();

    let nodes_writer: NodeFileWriter = NodeFileWriter::new(target_node_path)
        .set_separator(target_node_list_separator.or(Some(nodes_reader.get_separator()?)))?
        .set_node_types_separator(
            target_node_types_separator.or(nodes_reader.get_node_types_separator()),
        )?
        .set_header(target_node_list_header.or(Some(nodes_reader.has_header()?)))
        .set_node_types_column(if has_node_types {
            target_node_list_node_types_column.or(original_node_list_node_types_column.clone())
        } else {
            None
        })
        .set_node_types_column_number(if has_node_types {
            target_node_list_node_types_column_number
                .or(nodes_reader.get_node_types_column_number())
        } else {
            None
        })
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

    let mut new_number_of_nodes = 0;

    nodes_writer.dump_iterator(
        number_of_nodes.map(|number_of_nodes| number_of_nodes as usize),
        lines_iterator.filter_map(|line| line.ok()).enumerate().map(
            |(line_number, (_, (node_name, maybe_node_type_names)))| {
                new_number_of_nodes += 1;
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
                            .collect::<Vec<NodeTypeT>>()
                    }),
                    maybe_node_type_names,
                )
            },
        ),
    )?;

    if let Some(target_node_type_list_path) = target_node_type_list_path {
        node_types.build()?;
        let node_type_writer = TypeFileWriter::new(target_node_type_list_path)
            .set_separator(target_node_type_list_separator)?
            .set_header(target_node_type_list_header)
            .set_types_column(target_node_type_list_node_types_column)
            .set_types_column_number(target_node_type_list_node_types_column_number);

        node_type_writer.dump_iterator(
            Some(node_types.len()),
            node_types
                .iter()
                .map(|(node_type_id, node_type_name)| (node_type_id as NodeTypeT, node_type_name)),
        )?;
    }

    // We verify that self-consistency is not violated by checking that if the node list
    // seems to have node types, then there are node types in the node types vocabulary
    // and viceversa.
    if has_node_types && node_types.is_empty() {
        return Err(concat!(
            "The node list seems to have node types, but no node types were found in the ",
            "node types vocabulary. This is likely due to a bug in the code. Please open ",
            "an issue on the GRAPE GitHub repository with all the details."
        ).to_string());
    }

    // We also validate the opposite case, where somehow the node list seems not
    // to have any node types, yet there are node types in the node types vocabulary.
    if !has_node_types && !node_types.is_empty() {
        return Err(format!(
            concat!(
                "The node list seems not to have node types, but {} node types were found ",
                "in the node types vocabulary. This is likely due to a bug in the code. ",
                "Please open an issue on the GRAPE GitHub repository with all the details."
            ),
            node_types.len()
        ));
    }

    Ok((
        new_number_of_nodes,
        if original_node_list_node_types_column.is_some()
            || original_node_list_node_types_column_number.is_some()
        {
            Some(node_types.len() as NodeTypeT)
        } else {
            None
        },
    ))
}
