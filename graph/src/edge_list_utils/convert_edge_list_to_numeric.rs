use std::intrinsics::unlikely;

use crate::{
    parse_nodes, parse_types, utils::ItersWrapper, EdgeFileReader, EdgeFileWriter, EdgeT,
    EdgeTypeT, NodeFileReader, NodeFileWriter, NodeT, Result, TypeFileReader, TypeFileWriter,
    Vocabulary, WeightT, NODE_NOT_PRESENT,
};

/// Create a new edge list starting from given one with node IDs densified.
///
/// # Raises
/// * If there are problems with opening the original or target file.
/// * If the original and target paths are identical.
///
/// TODO! add check for space on disk where possible.
/// TODO! Update docstring!
pub fn convert_edge_list_to_numeric(
    original_edge_path: &str,
    target_edge_path: &str,
    directed: bool,

    original_node_path: Option<String>,
    original_node_list_separator: Option<char>,
    original_node_list_header: Option<bool>,
    original_node_list_support_balanced_quotes: Option<bool>,
    node_list_rows_to_skip: Option<usize>,
    node_list_is_correct: Option<bool>,
    node_list_max_rows_number: Option<usize>,
    node_list_comment_symbol: Option<String>,
    original_nodes_column_number: Option<usize>,
    original_nodes_column: Option<String>,
    nodes_number: Option<NodeT>,
    original_minimum_node_id: Option<NodeT>,
    original_numeric_node_ids: Option<bool>,
    original_load_node_list_in_parallel: Option<bool>,

    original_edge_type_path: Option<String>,
    original_edge_types_column_number: Option<usize>,
    original_edge_types_column: Option<String>,
    edge_types_number: Option<EdgeTypeT>,
    original_numeric_edge_type_ids: Option<bool>,
    original_minimum_edge_type_id: Option<EdgeTypeT>,
    original_edge_type_list_separator: Option<char>,
    original_edge_type_list_header: Option<bool>,
    original_edge_type_list_support_balanced_quotes: Option<bool>,
    edge_type_list_rows_to_skip: Option<usize>,
    edge_type_list_is_correct: Option<bool>,
    edge_type_list_max_rows_number: Option<usize>,
    edge_type_list_comment_symbol: Option<String>,
    load_edge_type_list_in_parallel: Option<bool>,

    original_edge_list_separator: Option<char>,
    original_edge_list_header: Option<bool>,
    original_edge_list_support_balanced_quotes: Option<bool>,
    original_sources_column_number: Option<usize>,
    original_sources_column: Option<String>,
    original_destinations_column_number: Option<usize>,
    original_destinations_column: Option<String>,
    original_edge_list_edge_types_column: Option<String>,
    original_edge_list_edge_types_column_number: Option<usize>,
    original_weights_column: Option<String>,
    original_weights_column_number: Option<usize>,

    target_edge_list_separator: Option<char>,
    target_edge_list_header: Option<bool>,
    target_sources_column: Option<String>,
    target_sources_column_number: Option<usize>,
    target_destinations_column: Option<String>,
    target_destinations_column_number: Option<usize>,
    target_edge_list_edge_types_column: Option<String>,
    target_edge_list_edge_types_column_number: Option<usize>,
    target_weights_column: Option<String>,
    target_weights_column_number: Option<usize>,

    target_node_path: Option<&str>,
    target_node_list_separator: Option<char>,
    target_node_list_header: Option<bool>,
    target_nodes_column: Option<String>,
    target_nodes_column_number: Option<usize>,

    target_edge_type_list_path: Option<String>,
    target_edge_type_list_separator: Option<char>,
    target_edge_type_list_header: Option<bool>,
    target_edge_type_list_edge_types_column: Option<String>,
    target_edge_type_list_edge_types_column_number: Option<usize>,

    remove_chevrons: Option<bool>,
    remove_spaces: Option<bool>,

    comment_symbol: Option<String>,
    default_edge_type: Option<String>,
    default_weight: Option<WeightT>,
    max_rows_number: Option<usize>,
    rows_to_skip: Option<usize>,
    edges_number: Option<usize>,
    skip_edge_types_if_unavailable: Option<bool>,
    skip_weights_if_unavailable: Option<bool>,
    numeric_rows_are_surely_smaller_than_original: Option<bool>,
    verbose: Option<bool>,
    name: Option<String>,
) -> Result<(NodeT, Option<EdgeTypeT>)> {
    let numeric_rows_are_surely_smaller_than_original =
        numeric_rows_are_surely_smaller_than_original.unwrap_or(false);
    if !numeric_rows_are_surely_smaller_than_original && original_edge_path == target_edge_path {
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

    if original_edge_list_edge_types_column.is_none()
        && original_edge_list_edge_types_column_number.is_none()
        && target_edge_type_list_path.is_some()
    {
        return Err(concat!(
            "The path where to store the edge types has been provided, but ",
            "neither the edge types column nor the edge types column number ",
            "have been provided."
        )
        .to_string());
    }

    if original_node_path.is_some()
        && (!node_list_is_correct.clone().unwrap_or(false) || nodes_number.is_none())
        && original_load_node_list_in_parallel.clone().unwrap_or(false)
    {
        return Err(concat!(
            "Since the nodes number was not provided but the node list is requested to be loaded ",
            "in parallel and it is not provided the information whether it is correct.\n",
            "This may cause the edge list node IDs to be mis-aligned to the desired ",
            "node IDs.\n",
            "This is likely a mis-configuration in the edge list preprocessing pipeline ",
            "and should be reported to the Ensmallen repository. Thanks!"
        )
        .to_string());
    }

    let name = name.unwrap_or("Graph".to_owned());
    let (mut nodes, writable): (Vocabulary<NodeT>, bool) = if let Some(original_node_path) = &original_node_path {
        let node_file_reader = NodeFileReader::new(Some(original_node_path.to_string()))?
            .set_comment_symbol(node_list_comment_symbol)?
            .set_header(original_node_list_header)?
            .set_support_balanced_quotes(original_node_list_support_balanced_quotes)?
            .set_max_rows_number(node_list_max_rows_number)?
            .set_rows_to_skip(node_list_rows_to_skip)?
            .set_separator(original_node_list_separator)?
            .set_nodes_column_number(original_nodes_column_number)?
            .set_nodes_column(original_nodes_column)?
            .set_minimum_node_id(original_minimum_node_id)
            .set_numeric_node_ids(original_numeric_node_ids)
            .set_csv_is_correct(node_list_is_correct)?
            .set_number_of_nodes(nodes_number)
            .set_parallel(original_load_node_list_in_parallel)?
            .set_remove_chevrons(remove_chevrons)
            .set_remove_spaces(remove_spaces);
        let (nodes, _) = parse_nodes(
            node_file_reader.read_lines().transpose()?,
            node_file_reader.nodes_number.clone(),
            None,
            node_file_reader.is_csv_correct()?,
            node_file_reader.has_numeric_node_ids(),
            false,
            node_file_reader.get_minimum_node_id(),
            None,
        )?;
        (nodes, false)
    } else {
        (Vocabulary::new(false), true)
    };

    let mut edge_types: Vocabulary<EdgeTypeT> =
        if let Some(original_edge_type_path) = original_edge_type_path {
            let edge_type_file_reader = TypeFileReader::new(Some(original_edge_type_path))?
                .set_comment_symbol(edge_type_list_comment_symbol)?
                .set_header(original_edge_type_list_header)?
                .set_support_balanced_quotes(original_edge_type_list_support_balanced_quotes)?
                .set_max_rows_number(edge_type_list_max_rows_number)?
                .set_rows_to_skip(edge_type_list_rows_to_skip)?
                .set_separator(original_edge_type_list_separator)?
                .set_type_column_number(original_edge_types_column_number)?
                .set_type_column(original_edge_types_column)?
                .set_minimum_type_id(original_minimum_edge_type_id)
                .set_numeric_type_ids(original_numeric_edge_type_ids)
                .set_csv_is_correct(edge_type_list_is_correct)?
                .set_types_number(edge_types_number)
                .set_parallel(load_edge_type_list_in_parallel)?
                .set_remove_chevrons(remove_chevrons)
                .set_remove_spaces(remove_spaces);
            let edge_types_vocabulary = parse_types(
                edge_type_file_reader.read_lines().transpose()?,
                edge_types_number,
                Some(edge_type_file_reader.has_numeric_type_ids()),
                edge_type_file_reader.get_minimum_type_id(),
                true,
                edge_type_list_is_correct,
            )?
            .unwrap();
            edge_types_vocabulary
        } else {
            Vocabulary::new(true)
        };

    let file_reader = EdgeFileReader::new(original_edge_path)?
        .set_comment_symbol(comment_symbol)?
        .set_max_rows_number(max_rows_number)?
        .set_rows_to_skip(rows_to_skip)?
        .set_header(original_edge_list_header)?
        .set_support_balanced_quotes(original_edge_list_support_balanced_quotes)
        .set_separator(original_edge_list_separator)?
        .set_skip_edge_types_if_unavailable(skip_edge_types_if_unavailable)
        .set_skip_weights_if_unavailable(skip_weights_if_unavailable)
        .set_default_edge_type(default_edge_type)
        .set_default_weight(default_weight)?
        .set_destinations_column(original_destinations_column.clone())?
        .set_destinations_column_number(original_destinations_column_number)?
        .set_sources_column(original_sources_column.clone())?
        .set_sources_column_number(original_sources_column_number)?
        .set_edge_types_column(original_edge_list_edge_types_column.clone())?
        .set_edge_types_column_number(original_edge_list_edge_types_column_number)?
        .set_weights_column(original_weights_column.clone())?
        .set_weights_column_number(original_weights_column_number)?
        .set_parallel(Some(false))
        .set_remove_chevrons(remove_chevrons)
        .set_remove_spaces(remove_spaces)
        // To avoid a duplicated loading bar.
        .set_verbose(verbose.map(|verbose| verbose && edges_number.is_none()))
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
            target_edge_list_edge_types_column.or(original_edge_list_edge_types_column),
        )
        .set_edge_types_column_number(
            target_edge_list_edge_types_column_number
                .or(file_reader.get_edge_types_column_number()),
        )
        .set_weights_column(target_weights_column.or(original_weights_column))
        .set_weights_column_number(
            target_weights_column_number.or(file_reader.get_weights_column_number()),
        )
        .set_separator(target_edge_list_separator.or(Some(file_reader.get_separator())))?
        .set_numeric_node_ids(Some(true))
        .set_numeric_edge_type_ids(Some(true))
        .set_verbose(verbose)
        .set_header(target_edge_list_header.or(Some(file_reader.has_header())));
    let lines_iterator = file_reader.read_lines()?;
    let lines_iterator = match lines_iterator {
        ItersWrapper::Parallel(_) => unreachable!("This is not meant to run in parallel."),
        ItersWrapper::Sequential(i) => i,
    };
    let (node_file_writer, mut node_file_stream) = if original_node_path.is_none() {
        if let Some(target_node_path) = target_node_path {
            nodes.build()?;
            let node_file_writer = NodeFileWriter::new(target_node_path)
                .set_separator(target_node_list_separator)?
                .set_header(target_node_list_header)
                .set_nodes_column(target_nodes_column)
                .set_nodes_column_number(target_nodes_column_number);

            node_file_writer.dump_iterator(
                Some(nodes.len()),
                nodes
                    .iter_keys()
                    .enumerate()
                    .map(|(node_id, node_name)| (node_id as NodeT, node_name, None, None)),
            )?;
            let stream = node_file_writer.start_writer()?;
            (Some(node_file_writer), Some(stream))
        } else {
            (None, None)
        }
    } else {
        (None, None)
    };

    let mut edge_file_stream = file_writer.start_writer()?;

    for (_, (src_name, dst_name, edge_type, weight)) in lines_iterator.filter_map(|line| line.ok())
    {
        let (src_id, src_was_already_present) = nodes.insert(src_name.clone())?;
        let (dst_id, dst_was_already_present) = nodes.insert(dst_name.clone())?;

        // If the node list was already provided as a file, it means that no further additions
        // to the vocabulary derived from nodes present solely in the edge list are expected.
        // We denote the case where the node list is writable with omonymous variable
        // When we encounter a case where the node list is not writable, yet we have
        // encountered some value that is not present in the node list, we must provide
        // a meaningful and extensive error message to help them debug this situation.
        if !writable && (src_was_already_present || dst_was_already_present) {
            return Err(format!(
                concat!(
                    "The node list was provided as a file at the path {}, ",
                    "and as such, the node list is not writable.\n",
                    "This means that the node list is not expected to be modified ",
                    "by this function.\n",
                    "However, the node list is missing the following nodes that appear",
                    "in the edge list provided at the path {}:\n",
                    "Source: {}\n",
                    "Destination: {}\n",
                    "Please either provide a node list that contains all the nodes present in the edge list.\n",
                    "If you are sure that the node list is correct, and we are mistaken, ",
                    "please open an issue on the Ensmallen repository.\n",
                    "Thanks!"
                ),
                original_node_path, original_edge_path,
                src_name, dst_name
            ));
        }
        node_file_stream = node_file_stream.and_then(|mut nfs| {
            if let Some(node_file_writer) = &node_file_writer {
                if !src_was_already_present {
                    node_file_writer
                        .write_line(&mut nfs, src_id, src_name, None, None, None)
                        .unwrap();
                }
                if !dst_was_already_present {
                    node_file_writer
                        .write_line(&mut nfs, dst_id, dst_name, None, None, None)
                        .unwrap();
                }
                Some(nfs)
            } else {
                None
            }
        });

        let edge_type =
            edge_type.map(|edge_type| unsafe { edge_types.unchecked_insert(edge_type) });
        let weight = if weight.is_nan() { None } else { Some(weight) };
        if directed || src_id == dst_id {
            file_writer.write_line(
                &mut edge_file_stream,
                0,
                src_id,
                "".to_string(),
                dst_id,
                "".to_string(),
                edge_type,
                None,
                weight,
            )?;
        } else {
            file_writer.write_line(
                &mut edge_file_stream,
                0,
                src_id,
                "".to_string(),
                dst_id,
                "".to_string(),
                edge_type,
                None,
                weight,
            )?;
            file_writer.write_line(
                &mut edge_file_stream,
                0,
                dst_id,
                "".to_string(),
                src_id,
                "".to_string(),
                edge_type,
                None,
                weight,
            )?;
        }
    }
    file_writer.close_writer(edge_file_stream)?;

    if let (Some(node_file_writer), Some(stream)) = (node_file_writer, node_file_stream) {
        node_file_writer.close_writer(stream)?;
    }

    if let Some(target_edge_type_list_path) = target_edge_type_list_path {
        edge_types.build()?;
        let edge_type_writer = TypeFileWriter::new(target_edge_type_list_path)
            .set_separator(target_edge_type_list_separator)?
            .set_header(target_edge_type_list_header)
            .set_types_column(target_edge_type_list_edge_types_column)
            .set_types_column_number(target_edge_type_list_edge_types_column_number);

        edge_type_writer.dump_iterator(
            Some(edge_types.len()),
            edge_types
                .iter()
                .map(|(edge_type_id, edge_type_name)| (edge_type_id as EdgeTypeT, edge_type_name)),
        )?;
    }

    Ok((
        nodes.len() as NodeT,
        if edge_types.is_empty() {
            None
        } else {
            Some(edge_types.len() as EdgeTypeT)
        },
    ))
}

/// Create a new edge list starting from given numeric one with node IDs densified and returns the number of unique nodes.
///
/// This method is meant as a solution to parse very large sparse numeric graphs,
/// like for instance ClueWeb.
///
/// # Safety
/// This method will panic if the node IDs are not numeric.
/// TODO: In the future we may handle this case as a normal error.
///
/// # Arguments
/// * `maximum_node_id`: Option<EdgeT> - The maximum node ID present in this graph. If available, optimal memory allocation will be used.
/// * `original_edge_path`: &str - The path from where to load the original edge list.
/// * `original_edge_list_separator`: Option<char> - Separator to use for the original edge list.
/// * `original_edge_list_header`: Option<bool> - Whether the original edge list has an header.
/// * `original_edge_list_support_balanced_quotes`: Option<bool> -
/// * `original_sources_column`: Option<String> - The column name to use to load the sources in the original edges list.
/// * `original_sources_column_number`: Option<usize> - The column number to use to load the sources in the original edges list.
/// * `original_destinations_column`: Option<String> - The column name to use to load the destinations in the original edges list.
/// * `original_destinations_column_number`: Option<usize> - The column number to use to load the destinations in the original edges list.
/// * `original_edge_list_edge_types_column`: Option<String> - The column name to use for the edge types in the original edges list.
/// * `original_edge_list_edge_types_column_number`: Option<usize> - The column number to use for the edge types in the original edges list.
/// * `original_weights_column`: Option<String> - The column name to use for the weights in the original edges list.
/// * `original_weights_column_number`: Option<usize> - The column number to use for the weights in the original edges list.
/// * `target_edge_path`: &str - The path from where to load the target edge list.
/// * `target_edge_list_separator`: Option<char> - Separator to use for the target edge list.
/// * `target_edge_list_header`: Option<bool> - Whether the target edge list has an header.
/// * `target_sources_column`: Option<String> - The column name to use to load the sources in the target edges list.
/// * `target_sources_column_number`: Option<usize> - The column number to use to load the sources in the target edges list.
/// * `target_destinations_column`: Option<String> - The column name to use to load the destinations in the target edges list.
/// * `target_destinations_column_number`: Option<usize> - The column number to use to load the destinations in the target edges list.
/// * `target_edge_list_edge_types_column`: Option<String> - The column name to use for the edge types in the target edges list.
/// * `target_edge_list_edge_types_column_number`: Option<usize> - The column number to use for the edge types in the target edges list.
/// * `target_weights_column`: Option<String> - The column name to use for the weights in the target edges list.
/// * `target_weights_column_number`: Option<usize> - The column number to use for the weights in the target edges list.
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
///
/// TODO! Update docstring!
pub fn densify_sparse_numeric_edge_list(
    original_edge_path: &str,
    target_edge_path: &str,
    directed: bool,

    maximum_node_id: Option<EdgeT>,
    original_edge_list_separator: Option<char>,
    original_edge_list_header: Option<bool>,
    original_sources_column: Option<String>,
    original_sources_column_number: Option<usize>,
    original_destinations_column: Option<String>,
    original_destinations_column_number: Option<usize>,
    original_edge_list_edge_types_column: Option<String>,
    original_edge_list_edge_types_column_number: Option<usize>,
    original_weights_column: Option<String>,
    original_weights_column_number: Option<usize>,

    original_edge_type_path: Option<String>,
    original_edge_types_column_number: Option<usize>,
    original_edge_types_column: Option<String>,
    edge_types_number: Option<EdgeTypeT>,
    original_numeric_edge_type_ids: Option<bool>,
    original_minimum_edge_type_id: Option<EdgeTypeT>,
    original_edge_type_list_separator: Option<char>,
    original_edge_type_list_header: Option<bool>,
    edge_type_list_rows_to_skip: Option<usize>,
    edge_type_list_is_correct: Option<bool>,
    edge_type_list_max_rows_number: Option<usize>,
    edge_type_list_comment_symbol: Option<String>,
    load_edge_type_list_in_parallel: Option<bool>,

    target_edge_list_separator: Option<char>,
    target_edge_list_header: Option<bool>,
    target_sources_column: Option<String>,
    target_sources_column_number: Option<usize>,
    target_destinations_column: Option<String>,
    target_destinations_column_number: Option<usize>,
    target_edge_list_edge_types_column: Option<String>,
    target_edge_list_edge_types_column_number: Option<usize>,
    target_weights_column: Option<String>,
    target_weights_column_number: Option<usize>,

    target_node_path: Option<&str>,
    target_node_list_separator: Option<char>,
    target_node_list_header: Option<bool>,
    target_nodes_column: Option<String>,
    target_nodes_column_number: Option<usize>,

    target_edge_type_list_path: Option<String>,
    target_edge_type_list_separator: Option<char>,
    target_edge_type_list_header: Option<bool>,
    target_edge_type_list_edge_types_column: Option<String>,
    target_edge_type_list_edge_types_column_number: Option<usize>,

    comment_symbol: Option<String>,
    default_edge_type: Option<String>,
    default_weight: Option<WeightT>,
    max_rows_number: Option<usize>,
    rows_to_skip: Option<usize>,
    edges_number: Option<usize>,
    skip_edge_types_if_unavailable: Option<bool>,
    skip_weights_if_unavailable: Option<bool>,
    numeric_rows_are_surely_smaller_than_original: Option<bool>,
    verbose: Option<bool>,
    name: Option<String>,
) -> Result<(NodeT, Option<EdgeTypeT>)> {
    let numeric_rows_are_surely_smaller_than_original =
        numeric_rows_are_surely_smaller_than_original.unwrap_or(false);
    if !numeric_rows_are_surely_smaller_than_original && original_edge_path == target_edge_path {
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

    if original_edge_list_edge_types_column.is_none()
        && original_edge_list_edge_types_column_number.is_none()
        && target_edge_type_list_path.is_some()
    {
        return Err(concat!(
            "The path where to store the edge types has been provided, but ",
            "neither the edge types column nor the edge types column number ",
            "have been provided."
        )
        .to_string());
    }

    let name = name.unwrap_or("Graph".to_owned());
    let mut nodes: Vec<NodeT> = if let Some(maximum_node_id) = maximum_node_id {
        vec![NODE_NOT_PRESENT; maximum_node_id as usize]
    } else {
        Vec::new()
    };
    let mut inserted_nodes: NodeT = 0;
    let mut edge_types: Vocabulary<EdgeTypeT> =
        if let Some(original_edge_type_path) = original_edge_type_path {
            let edge_type_file_reader = TypeFileReader::new(Some(original_edge_type_path))?
                .set_comment_symbol(edge_type_list_comment_symbol)?
                .set_header(original_edge_type_list_header)?
                .set_max_rows_number(edge_type_list_max_rows_number)?
                .set_rows_to_skip(edge_type_list_rows_to_skip)?
                .set_separator(original_edge_type_list_separator)?
                .set_type_column_number(original_edge_types_column_number)?
                .set_type_column(original_edge_types_column)?
                .set_csv_is_correct(edge_type_list_is_correct)?
                .set_minimum_type_id(original_minimum_edge_type_id)
                .set_numeric_type_ids(original_numeric_edge_type_ids)
                .set_types_number(edge_types_number)
                .set_parallel(load_edge_type_list_in_parallel)?;
            let edge_types_vocabulary = parse_types(
                edge_type_file_reader.read_lines().transpose()?,
                edge_types_number,
                Some(edge_type_file_reader.has_numeric_type_ids()),
                edge_type_file_reader.get_minimum_type_id(),
                true,
                edge_type_list_is_correct,
            )?
            .unwrap();
            edge_types_vocabulary
        } else {
            Vocabulary::new(true)
        };
    let file_reader = EdgeFileReader::new(original_edge_path)?
        .set_comment_symbol(comment_symbol)?
        .set_max_rows_number(max_rows_number)?
        .set_rows_to_skip(rows_to_skip)?
        .set_header(original_edge_list_header)?
        .set_separator(original_edge_list_separator)?
        .set_default_edge_type(default_edge_type)
        .set_default_weight(default_weight)?
        .set_destinations_column(original_destinations_column.clone())?
        .set_destinations_column_number(original_destinations_column_number)?
        .set_sources_column(original_sources_column.clone())?
        .set_sources_column_number(original_sources_column_number)?
        .set_edge_types_column(original_edge_list_edge_types_column.clone())?
        .set_edge_types_column_number(original_edge_list_edge_types_column_number)?
        .set_weights_column(original_weights_column.clone())?
        .set_weights_column_number(original_weights_column_number)?
        .set_parallel(Some(false))
        .set_skip_edge_types_if_unavailable(skip_edge_types_if_unavailable)
        .set_skip_weights_if_unavailable(skip_weights_if_unavailable)
        // To avoid a duplicated loading bar.
        .set_verbose(verbose.map(|verbose| verbose && edges_number.is_none()))
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
            target_edge_list_edge_types_column.or(original_edge_list_edge_types_column),
        )
        .set_edge_types_column_number(
            target_edge_list_edge_types_column_number
                .or(file_reader.get_edge_types_column_number()),
        )
        .set_weights_column(target_weights_column.or(original_weights_column))
        .set_weights_column_number(
            target_weights_column_number.or(file_reader.get_weights_column_number()),
        )
        .set_separator(target_edge_list_separator.or(Some(file_reader.get_separator())))?
        .set_numeric_node_ids(Some(true))
        .set_numeric_edge_type_ids(Some(true))
        .set_verbose(verbose)
        .set_header(target_edge_list_header.or(Some(file_reader.has_header())));
    let lines_iterator = file_reader.read_lines()?;
    let lines_iterator = match lines_iterator {
        ItersWrapper::Parallel(_) => unreachable!("This is not meant to run in parallel."),
        ItersWrapper::Sequential(i) => i,
    };

    let mut to_numeric_node_name = |node_name: &str| {
        let numeric_node_name = node_name.parse::<EdgeT>().unwrap() as usize;
        // If the vector of the nodes is not big enough, we need to
        // expand it up to the required amount.
        // We use the unlikely directive to specify to the compiler
        // that this branch should not be visited often during the
        // execution of this script, except for pathological cases.
        if unlikely(nodes.len() <= numeric_node_name) {
            nodes.extend((nodes.len()..=numeric_node_name).map(|_| NODE_NOT_PRESENT));
        }
        // If the ID for the current source node was not already provided
        // we assign to it the current number of inserted nodes
        if nodes[numeric_node_name] == NODE_NOT_PRESENT {
            nodes[numeric_node_name] = inserted_nodes;
            inserted_nodes += 1;
        };
        // And we return the value
        nodes[numeric_node_name]
    };
    file_writer.dump_iterator(
        // We do not care to be exact here: if the graph does not contain
        // selfloops the value will be correct.
        edges_number.map(|edges_number| {
            if directed {
                edges_number
            } else {
                edges_number * 2
            }
        }),
        lines_iterator
            // Removing eventual errors.
            .filter_map(|line| line.ok())
            // Processing line
            .flat_map(|(_, (src_name, dst_name, edge_type, weight))| unsafe {
                let src_id = to_numeric_node_name(&src_name);
                let dst_id = to_numeric_node_name(&dst_name);
                let edge_type = edge_type.map(|edge_type| edge_types.unchecked_insert(edge_type));
                let weight = if weight.is_nan() { None } else { Some(weight) };
                if directed || src_id == dst_id {
                    vec![(
                        0, src_id, src_name, dst_id, dst_name, edge_type, None, weight,
                    )]
                } else {
                    vec![
                        (
                            0,
                            src_id,
                            src_name.clone(),
                            dst_id,
                            dst_name.clone(),
                            edge_type,
                            None,
                            weight,
                        ),
                        (
                            0, dst_id, dst_name, src_id, src_name, edge_type, None, weight,
                        ),
                    ]
                }
            }),
    )?;

    if let Some(target_node_path) = target_node_path {
        let node_file_writer = NodeFileWriter::new(target_node_path)
            .set_separator(target_node_list_separator)?
            .set_header(target_node_list_header)
            .set_nodes_column(target_nodes_column)
            .set_nodes_column_number(target_nodes_column_number);

        node_file_writer.dump_iterator(
            Some(inserted_nodes as usize),
            nodes
                .into_iter()
                .enumerate()
                .filter(|&(_, numeric_node_name)| numeric_node_name != NODE_NOT_PRESENT)
                .map(|(numeric_node_name, node_id)| {
                    (node_id, numeric_node_name.to_string(), None, None)
                }),
        )?;
    }

    if let Some(target_edge_type_list_path) = target_edge_type_list_path {
        if edge_types.is_empty() {
            edge_types.build()?;
        }
        let edge_type_writer = TypeFileWriter::new(target_edge_type_list_path)
            .set_separator(target_edge_type_list_separator)?
            .set_header(target_edge_type_list_header)
            .set_types_column(target_edge_type_list_edge_types_column)
            .set_types_column_number(target_edge_type_list_edge_types_column_number);

        edge_type_writer.dump_iterator(
            Some(edge_types.len()),
            edge_types
                .iter()
                .map(|(edge_type_id, edge_type_name)| (edge_type_id as EdgeTypeT, edge_type_name)),
        )?;
    }

    Ok((
        inserted_nodes,
        if edge_types.is_empty() {
            None
        } else {
            Some(edge_types.len() as EdgeTypeT)
        },
    ))
}
