use crate::{
    convert_edge_list_to_numeric, convert_node_list_node_types_to_numeric,
    densify_sparse_numeric_edge_list, get_minmax_node_from_numeric_edge_list, get_rows_number,
    is_numeric_edge_list, sort_numeric_edge_list_inplace, EdgeT, EdgeTypeT, NodeT, NodeTypeT,
    Result, WeightT,
};
use log::info;

/// TODO: write the docstring
pub fn build_optimal_lists_files(
    original_edge_path: String,
    target_edge_path: String,
    directed: bool,

    original_node_type_path: Option<String>,
    original_node_type_list_separator: Option<char>,
    original_node_types_column_number: Option<usize>,
    original_node_types_column: Option<String>,
    original_numeric_node_type_ids: Option<bool>,
    original_minimum_node_type_id: Option<NodeTypeT>,

    original_node_type_list_header: Option<bool>,
    original_node_type_list_support_balanced_quotes: Option<bool>,
    original_node_type_list_rows_to_skip: Option<usize>,
    original_node_type_list_max_rows_number: Option<usize>,
    original_node_type_list_comment_symbol: Option<String>,
    original_load_node_type_list_in_parallel: Option<bool>,
    original_node_type_list_is_correct: Option<bool>,
    mut node_types_number: Option<NodeTypeT>,

    target_node_type_list_path: Option<String>,
    target_node_type_list_separator: Option<char>,
    mut target_node_type_list_node_types_column_number: Option<usize>,
    mut target_node_type_list_node_types_column: Option<String>,
    target_node_type_list_header: Option<bool>,

    mut original_node_path: Option<String>,
    mut original_node_list_separator: Option<char>,
    mut original_node_list_header: Option<bool>,
    original_node_list_support_balanced_quotes: Option<bool>,
    node_list_rows_to_skip: Option<usize>,
    mut node_list_is_correct: Option<bool>,
    node_list_max_rows_number: Option<usize>,
    node_list_comment_symbol: Option<String>,
    default_node_type: Option<String>,
    mut original_nodes_column_number: Option<usize>,
    mut original_nodes_column: Option<String>,
    original_node_types_separator: Option<char>,
    original_node_list_node_types_column_number: Option<usize>,
    original_node_list_node_types_column: Option<String>,
    mut nodes_number: Option<NodeT>,
    original_minimum_node_id: Option<NodeT>,
    original_numeric_node_ids: Option<bool>,
    original_node_list_numeric_node_type_ids: Option<bool>,
    original_skip_node_types_if_unavailable: Option<bool>,
    mut original_load_node_list_in_parallel: Option<bool>,
    mut maximum_node_id: Option<EdgeT>,

    target_node_path: Option<String>,
    mut target_node_list_separator: Option<char>,
    mut target_node_list_header: Option<bool>,
    mut target_nodes_column: Option<String>,
    mut target_nodes_column_number: Option<usize>,
    target_node_types_separator: Option<char>,
    mut target_node_list_node_types_column: Option<String>,
    mut target_node_list_node_types_column_number: Option<usize>,

    original_edge_type_path: Option<String>,
    original_edge_type_list_separator: Option<char>,
    original_edge_types_column_number: Option<usize>,
    original_edge_types_column: Option<String>,
    original_numeric_edge_type_ids: Option<bool>,
    original_minimum_edge_type_id: Option<EdgeTypeT>,
    original_edge_type_list_header: Option<bool>,
    original_edge_type_list_support_balanced_quotes: Option<bool>,
    edge_type_list_rows_to_skip: Option<usize>,
    edge_type_list_max_rows_number: Option<usize>,
    edge_type_list_comment_symbol: Option<String>,
    load_edge_type_list_in_parallel: Option<bool>,
    edge_type_list_is_correct: Option<bool>,
    edge_types_number: Option<EdgeTypeT>,

    target_edge_type_list_path: Option<String>,
    target_edge_type_list_separator: Option<char>,
    mut target_edge_type_list_edge_types_column_number: Option<usize>,
    mut target_edge_type_list_edge_types_column: Option<String>,
    target_edge_type_list_header: Option<bool>,

    original_edge_list_separator: Option<char>,
    original_edge_list_header: Option<bool>,
    original_edge_list_support_balanced_quotes: Option<bool>,
    original_sources_column_number: Option<usize>,
    original_sources_column: Option<String>,
    original_destinations_column_number: Option<usize>,
    original_destinations_column: Option<String>,
    original_edge_list_edge_types_column_number: Option<usize>,
    original_edge_list_edge_types_column: Option<String>,
    default_edge_type: Option<String>,
    original_weights_column_number: Option<usize>,
    original_weights_column: Option<String>,
    default_weight: Option<WeightT>,
    original_edge_list_numeric_node_ids: Option<bool>,
    skip_weights_if_unavailable: Option<bool>,
    skip_edge_types_if_unavailable: Option<bool>,
    edge_list_comment_symbol: Option<String>,
    edge_list_max_rows_number: Option<usize>,
    edge_list_rows_to_skip: Option<usize>,
    load_edge_list_in_parallel: Option<bool>,
    mut edges_number: Option<EdgeT>,

    target_edge_list_separator: Option<char>,
    remove_chevrons: Option<bool>,
    remove_spaces: Option<bool>,

    numeric_rows_are_surely_smaller_than_original: Option<bool>,
    sort_temporary_directory: Option<String>,
    verbose: Option<bool>,
    name: Option<String>,
) -> Result<(Option<NodeTypeT>, NodeT, Option<EdgeTypeT>, EdgeT)> {
    // It does not make sense to provide a node types file
    // to be parsed but not providing any node type column
    // to be loaded within the node list file.
    if original_node_type_path.is_some()
        && original_node_list_node_types_column_number.is_none()
        && original_node_list_node_types_column.is_none()
    {
        return Err(concat!(
            "A node type path was provided, but no node type column name or number was specified ",
            "for the node list file."
        )
        .to_string());
    }

    let _ = target_nodes_column.insert("node_name".to_string());
    let _ = target_nodes_column_number.insert(0);
    let _ = target_node_list_header.insert(true);

    if original_node_list_node_types_column_number.is_some()
        || original_node_list_node_types_column.is_some()
    {
        let _ = target_node_type_list_node_types_column.insert("node_type".to_string());
        let _ = target_node_type_list_node_types_column_number.insert(0);
        let _ = target_node_list_node_types_column.insert("node_type".to_string());
        let _ = target_node_list_node_types_column_number.insert(1);
    }

    if original_edge_list_edge_types_column_number.is_some()
        || original_edge_list_edge_types_column.is_some()
    {
        let _ = target_edge_type_list_edge_types_column.insert("edge_type".to_string());
        let _ = target_edge_type_list_edge_types_column_number.insert(0);
    }

    // It does not make sense to provide a edge types file
    // to be parsed but not providing any edge type column
    // to be loaded within the edge list file.
    if original_edge_type_path.is_some()
        && original_edge_list_edge_types_column_number.is_none()
        && original_edge_list_edge_types_column.is_none()
    {
        return Err(concat!(
            "A edge type path was provided, but no edge type column name or number was specified ",
            "for the edge list file."
        )
        .to_string());
    }

    // We need to handle the optimization of the
    // nodes list, which only includes making sure that if there are
    // node types, there is a node types list and the node types
    // provided in the nodes file are numerical and dense.
    // Also, we need to make sure that the node list does not
    // include additional ignored fields in it, like for instance
    // textual node descriptions, that make loading the file
    // into Ensmallen slower.
    // Finally, the produced node list will also include the
    // node ID as a field, Ensuring that the parallel loading
    // procedure produces a deterministic internal node ID to
    // node name mapping.
    if let Some(original_node_path) = &mut original_node_path {
        if target_node_path.is_none() {
            return Err(concat!(
                "When providing the original node path that must be ",
                "parsed to produce the optimized node list, also the ",
                "target node path must be provided."
            )
            .to_string());
        }

        info!("Converting the node list node type names to numeric node type IDs.");
        let (new_nodes_number, new_node_types_number) = convert_node_list_node_types_to_numeric(
            original_node_path.clone(),
            target_node_path.clone().unwrap(),
            original_node_type_path,
            original_node_type_list_separator,
            original_node_types_column_number,
            original_node_types_column,
            node_types_number,
            original_numeric_node_type_ids,
            original_minimum_node_type_id,
            original_node_type_list_header,
            original_node_type_list_support_balanced_quotes,
            original_node_type_list_rows_to_skip,
            original_node_type_list_is_correct,
            original_node_type_list_max_rows_number,
            original_node_type_list_comment_symbol,
            original_load_node_type_list_in_parallel,
            target_node_type_list_path,
            target_node_type_list_separator,
            target_node_type_list_header,
            target_node_type_list_node_types_column,
            target_node_type_list_node_types_column_number,
            original_node_list_separator,
            original_node_list_header,
            original_node_list_support_balanced_quotes,
            node_list_rows_to_skip,
            node_list_max_rows_number,
            node_list_comment_symbol.clone(),
            default_node_type,
            original_nodes_column_number,
            original_nodes_column,
            original_node_types_separator,
            original_node_list_node_types_column_number,
            original_node_list_node_types_column,
            original_minimum_node_id,
            original_numeric_node_ids,
            original_node_list_numeric_node_type_ids,
            original_skip_node_types_if_unavailable,
            remove_chevrons,
            remove_spaces,
            target_node_list_separator.clone(),
            target_node_list_header,
            target_nodes_column_number,
            target_nodes_column.clone(),
            target_node_types_separator.clone(),
            target_node_list_node_types_column_number,
            target_node_list_node_types_column,
            nodes_number,
        )?;
        // Now we need to update the node list parameters
        // that should be used in the next step.
        // We do not update again the node types as it
        // is not needed after this step.
        node_types_number = new_node_types_number;
        *original_node_path = target_node_path.clone().unwrap();
        original_node_list_separator = target_node_list_separator;
        target_node_list_separator = None;
        original_node_list_header = target_node_list_header;
        original_nodes_column_number = target_nodes_column_number;
        original_nodes_column = None;
        // Now we know the number of nodes
        nodes_number = Some(new_nodes_number);
        // And that the node list is correct
        node_list_is_correct = Some(true);
        // therefore we can now load this in parallel.
        original_load_node_list_in_parallel = Some(true);
    }

    // We check if the edge list has numeric node IDs
    // unless the information was already provided.
    // We always treat as non-numeric the nodes if the
    // node list vocabulary has been provided.
    info!("Computing whether the edge list has numeric node IDs.");
    let numeric_edge_list_node_ids = original_edge_list_numeric_node_ids.map_or_else(
        || {
            is_numeric_edge_list(
                original_edge_path.as_ref(),
                original_edge_list_separator.clone(),
                original_edge_list_header,
                original_edge_list_support_balanced_quotes,
                original_sources_column.clone(),
                original_sources_column_number,
                original_destinations_column.clone(),
                original_destinations_column_number,
                edge_list_comment_symbol.clone(),
                edge_list_max_rows_number,
                edge_list_rows_to_skip,
                None,
                load_edge_list_in_parallel,
                remove_chevrons,
                remove_spaces,
                verbose,
                name.clone(),
            )
        },
        |value| Ok(value),
    )?;

    // We identify if the edge list is meant to have edge types
    let has_edge_types = original_edge_list_edge_types_column.is_some()
        || original_edge_list_edge_types_column_number.is_some();
    // We identify if the edge list is meant to have edge weights
    let has_edge_weights =
        original_weights_column.is_some() || original_weights_column_number.is_some();

    // We convert the edge list to dense numeric
    let (nodes_number, edge_types_number) = if numeric_edge_list_node_ids {
        info!("Computing maximum node ID from sparse numeric edge list.");
        if maximum_node_id.is_none() {
            let (_, new_maximum_node_id, new_edges_number) =
                get_minmax_node_from_numeric_edge_list(
                    original_edge_path.as_ref(),
                    original_edge_list_separator.clone(),
                    original_edge_list_header,
                    original_edge_list_support_balanced_quotes,
                    original_sources_column.clone(),
                    original_sources_column_number,
                    original_destinations_column.clone(),
                    original_destinations_column_number,
                    edge_list_comment_symbol.clone(),
                    edge_list_max_rows_number,
                    edge_list_rows_to_skip,
                    None,
                    load_edge_list_in_parallel,
                    remove_chevrons,
                    remove_spaces,
                    verbose,
                    name.clone(),
                )?;
            maximum_node_id = Some(new_maximum_node_id);
            edges_number = Some(new_edges_number);
        }

        info!("Converting sparse numeric edge list to dense numeric edge list.");
        densify_sparse_numeric_edge_list(
            original_edge_path.as_ref(),
            target_edge_path.as_ref(),
            directed,
            maximum_node_id,
            original_edge_list_separator.clone(),
            original_edge_list_header,
            original_sources_column.clone(),
            original_sources_column_number,
            original_destinations_column.clone(),
            original_destinations_column_number,
            original_edge_list_edge_types_column.clone(),
            original_edge_list_edge_types_column_number,
            original_weights_column.clone(),
            original_weights_column_number,
            original_edge_type_path,
            original_edge_types_column_number,
            original_edge_types_column,
            edge_types_number,
            original_numeric_edge_type_ids,
            original_minimum_edge_type_id,
            original_edge_type_list_separator,
            original_edge_type_list_header,
            edge_type_list_rows_to_skip,
            edge_type_list_is_correct,
            edge_type_list_max_rows_number,
            edge_type_list_comment_symbol,
            load_edge_type_list_in_parallel,
            target_edge_list_separator.clone(),
            Some(false),
            None,
            Some(0),
            None,
            Some(1),
            None,
            if has_edge_types { Some(2) } else { None },
            None,
            if has_edge_weights {
                Some(2 + has_edge_types as usize)
            } else {
                None
            },
            target_node_path.as_deref(),
            target_node_list_separator.clone(),
            target_node_list_header,
            target_nodes_column,
            target_nodes_column_number,
            target_edge_type_list_path,
            target_edge_type_list_separator.clone(),
            target_edge_type_list_header,
            target_edge_type_list_edge_types_column,
            target_edge_type_list_edge_types_column_number,
            edge_list_comment_symbol.clone(),
            default_edge_type.clone(),
            default_weight,
            edge_list_max_rows_number,
            edge_list_rows_to_skip,
            edges_number.map(|edges_number| edges_number as usize),
            skip_edge_types_if_unavailable,
            skip_weights_if_unavailable,
            numeric_rows_are_surely_smaller_than_original,
            verbose,
            name.clone(),
        )
    } else {
        info!("Converting non-numeric edge list to numeric.");
        convert_edge_list_to_numeric(
            original_edge_path.as_ref(),
            target_edge_path.as_ref(),
            directed,
            original_node_path,
            original_node_list_separator,
            original_node_list_header,
            original_node_list_support_balanced_quotes,
            node_list_rows_to_skip,
            node_list_is_correct,
            node_list_max_rows_number,
            node_list_comment_symbol,
            original_nodes_column_number,
            original_nodes_column,
            nodes_number,
            original_minimum_node_id,
            original_numeric_node_ids,
            original_load_node_list_in_parallel,
            original_edge_type_path,
            original_edge_types_column_number,
            original_edge_types_column,
            edge_types_number,
            original_numeric_edge_type_ids,
            original_minimum_edge_type_id,
            original_edge_type_list_separator,
            original_edge_type_list_header,
            original_edge_type_list_support_balanced_quotes,
            edge_type_list_rows_to_skip,
            edge_type_list_is_correct,
            edge_type_list_max_rows_number,
            edge_type_list_comment_symbol,
            load_edge_type_list_in_parallel,
            original_edge_list_separator.clone(),
            original_edge_list_header,
            original_edge_list_support_balanced_quotes,
            original_sources_column_number,
            original_sources_column.clone(),
            original_destinations_column_number,
            original_destinations_column.clone(),
            original_edge_list_edge_types_column.clone(),
            original_edge_list_edge_types_column_number,
            original_weights_column.clone(),
            original_weights_column_number,
            target_edge_list_separator.clone(),
            Some(false),
            None,
            Some(0),
            None,
            Some(1),
            None,
            if has_edge_types { Some(2) } else { None },
            None,
            if has_edge_weights {
                Some(2 + has_edge_types as usize)
            } else {
                None
            },
            target_node_path.as_deref(),
            target_node_list_separator,
            target_node_list_header,
            target_nodes_column,
            target_nodes_column_number,
            target_edge_type_list_path,
            target_edge_type_list_separator.clone(),
            target_edge_type_list_header,
            target_edge_type_list_edge_types_column,
            target_edge_type_list_edge_types_column_number,
            remove_chevrons,
            remove_spaces,
            edge_list_comment_symbol.clone(),
            default_edge_type.clone(),
            default_weight,
            edge_list_max_rows_number,
            edge_list_rows_to_skip,
            edges_number.map(|edges_number| edges_number as usize),
            skip_edge_types_if_unavailable,
            skip_weights_if_unavailable,
            numeric_rows_are_surely_smaller_than_original,
            verbose,
            name.clone(),
        )
    }?;

    // Sort the edge list
    info!("Sorting the edge list.");
    sort_numeric_edge_list_inplace(
        target_edge_path.as_ref(),
        target_edge_type_list_separator.clone(),
        Some(false),
        None,
        Some(0),
        None,
        Some(1),
        None,
        if has_edge_types { Some(2) } else { None },
        None,
        None,
        sort_temporary_directory,
    )?;

    info!("Count the lines in the path, that match exactly with the number of edges.");
    let edges_number = get_rows_number(target_edge_path.as_ref())? as EdgeT;

    Ok((
        node_types_number,
        nodes_number,
        edge_types_number,
        edges_number,
    ))
}
