use super::*;
use graph::{EdgeFileReader, NodeFileReader, NodeT, WalksParameters, WeightT};
use std::collections::HashMap;

pub(crate) fn build_csv_file_reader(
    edge_path: String,
    py_kwargs: Option<&PyDict>,
) -> Result<(EdgeFileReader, Option<NodeFileReader>, String, bool), String> {
    let py = pyo3::Python::acquire_gil();
    let kwargs = normalize_kwargs!(py_kwargs, py.python());

    validate_kwargs(
        kwargs,
        &[
            "directed_edge_list",
            "sources_column_number",
            "sources_column",
            "destinations_column_number",
            "destinations_column",
            "edge_types_column_number",
            "edge_types_column",
            "default_edge_type",
            "weights_column_number",
            "weights_column",
            "default_weight",
            "skip_selfloops",
            "ignore_duplicated_edges",
            "edge_header",
            "edge_rows_to_skip",
            "edge_separator",
            "edge_max_rows_number",
            "node_path",
            "nodes_column_number",
            "nodes_column",
            "node_types_column_number",
            "node_types_column",
            "default_node_type",
            "ignore_duplicated_nodes",
            "node_header",
            "node_rows_to_skip",
            "node_separator",
            "node_types_separator",
            "node_max_rows_number",
            "verbose",
            "numeric_node_ids",
            "numeric_edge_node_ids",
            "numeric_node_type_ids",
            "numeric_edge_type_ids",
            "edge_file_comment_symbol",
            "node_file_comment_symbol",
            "skip_weights_if_unavailable",
            "skip_edge_types_if_unavailable",
            "skip_node_types_if_unavailable",
            "name",
            "node_list_is_correct",
            "edge_list_is_correct",
        ],
    )?;

    let graph_name =
        extract_value_rust_result!(kwargs, "name", String).unwrap_or_else(|| "Graph".to_owned());

    let edges: EdgeFileReader = EdgeFileReader::new(edge_path)?
        .set_separator(extract_value_rust_result!(kwargs, "edge_separator", String))?
        .set_csv_is_correct(extract_value_rust_result!(
            kwargs,
            "edge_list_is_correct",
            bool
        ))
        .set_skip_edge_types_if_unavailable(extract_value_rust_result!(
            kwargs,
            "skip_edge_types_if_unavailable",
            bool
        ))
        .set_skip_weights_if_unavailable(extract_value_rust_result!(
            kwargs,
            "skip_weights_if_unavailable",
            bool
        ))
        .set_comment_symbol(extract_value_rust_result!(
            kwargs,
            "edge_file_comment_symbol",
            String
        ))?
        .set_header(extract_value_rust_result!(kwargs, "edge_header", bool))
        .set_rows_to_skip(extract_value_rust_result!(
            kwargs,
            "edge_rows_to_skip",
            usize
        ))
        .set_sources_column_number(extract_value_rust_result!(
            kwargs,
            "sources_column_number",
            usize
        ))?
        .set_sources_column(extract_value_rust_result!(kwargs, "sources_column", String))?
        .set_destinations_column_number(extract_value_rust_result!(
            kwargs,
            "destinations_column_number",
            usize
        ))?
        .set_destinations_column(extract_value_rust_result!(
            kwargs,
            "destinations_column",
            String
        ))?
        .set_edge_types_column_number(extract_value_rust_result!(
            kwargs,
            "edge_types_column_number",
            usize
        ))?
        .set_edge_types_column(extract_value_rust_result!(
            kwargs,
            "edge_types_column",
            String
        ))?
        .set_default_edge_type(extract_value_rust_result!(
            kwargs,
            "default_edge_type",
            String
        ))
        .set_weights_column_number(extract_value_rust_result!(
            kwargs,
            "weights_column_number",
            usize
        ))?
        .set_weights_column(extract_value_rust_result!(kwargs, "weights_column", String))?
        .set_default_weight(extract_value_rust_result!(
            kwargs,
            "default_weight",
            WeightT
        ))
        .set_skip_selfloops(extract_value_rust_result!(kwargs, "skip_selfloops", bool))
        .set_ignore_duplicates(extract_value_rust_result!(
            kwargs,
            "ignore_duplicated_edges",
            bool
        ))
        .set_verbose(extract_value_rust_result!(kwargs, "verbose", bool))
        .set_numeric_node_ids(extract_value_rust_result!(
            kwargs,
            "numeric_edge_node_ids",
            bool
        ))
        .set_numeric_edge_type_ids(extract_value_rust_result!(
            kwargs,
            "numeric_edge_type_ids",
            bool
        ))
        .set_max_rows_number(extract_value_rust_result!(
            kwargs,
            "edge_max_rows_number",
            u64
        ));

    let nodes: Option<NodeFileReader> = match kwargs.get_item("node_path") {
        Some(_) => {
            let maybe_node_path = extract_value_rust_result!(kwargs, "node_path", String);
            match maybe_node_path {
                None => None,
                Some(node_path) => Some(
                    NodeFileReader::new(node_path)?
                        .set_separator(extract_value_rust_result!(
                            kwargs,
                            "node_separator",
                            String
                        ))?
                        .set_skip_node_types_if_unavailable(extract_value_rust_result!(
                            kwargs,
                            "skip_node_types_if_unavailable",
                            bool
                        ))?
                        .set_comment_symbol(extract_value_rust_result!(
                            kwargs,
                            "node_file_comment_symbol",
                            String
                        ))?
                        .set_csv_is_correct(extract_value_rust_result!(
                            kwargs,
                            "node_list_is_correct",
                            bool
                        ))
                        .set_header(extract_value_rust_result!(kwargs, "node_header", bool))
                        .set_rows_to_skip(extract_value_rust_result!(
                            kwargs,
                            "node_rows_to_skip",
                            usize
                        ))
                        .set_nodes_column_number(extract_value_rust_result!(
                            kwargs,
                            "nodes_column_number",
                            usize
                        ))
                        .set_nodes_column(extract_value_rust_result!(
                            kwargs,
                            "nodes_column",
                            String
                        ))?
                        .set_node_types_column_number(extract_value_rust_result!(
                            kwargs,
                            "node_types_column_number",
                            usize
                        ))
                        .set_node_types_column(extract_value_rust_result!(
                            kwargs,
                            "node_types_column",
                            String
                        ))?
                        .set_node_types_separator(extract_value_rust_result!(
                            kwargs,
                            "node_types_separator",
                            String
                        ))?
                        .set_default_node_type(extract_value_rust_result!(
                            kwargs,
                            "default_node_type",
                            String
                        ))
                        .set_ignore_duplicates(extract_value_rust_result!(
                            kwargs,
                            "ignore_duplicated_nodes",
                            bool
                        ))
                        .set_verbose(extract_value_rust_result!(kwargs, "verbose", bool))
                        .set_numeric_node_ids(extract_value_rust_result!(
                            kwargs,
                            "numeric_node_ids",
                            bool
                        ))
                        .set_numeric_node_type_ids(extract_value_rust_result!(
                            kwargs,
                            "numeric_node_type_ids",
                            bool
                        ))
                        .set_max_rows_number(extract_value_rust_result!(
                            kwargs,
                            "node_max_rows_number",
                            u64
                        )),
                ),
            }
        }
        None => None,
    };

    Ok((
        edges,
        nodes,
        graph_name,
        extract_value_rust_result!(kwargs, "directed_edge_list", bool).unwrap_or(false),
    ))
}

impl EnsmallenGraph {
    pub(crate) fn build_walk_parameters(
        &self,
        walk_length: u64,
        kwargs: &PyDict,
    ) -> Result<WalksParameters, String> {
        Ok(WalksParameters::new(walk_length)?
            .set_change_edge_type_weight(extract_value_rust_result!(
                kwargs,
                "change_edge_type_weight",
                WeightT
            ))?
            .set_change_node_type_weight(extract_value_rust_result!(
                kwargs,
                "change_node_type_weight",
                WeightT
            ))?
            .set_explore_weight(extract_value_rust_result!(
                kwargs,
                "explore_weight",
                WeightT
            ))?
            .set_return_weight(extract_value_rust_result!(kwargs, "return_weight", WeightT))?
            .set_random_state(extract_value_rust_result!(kwargs, "random_state", usize))
            .set_max_neighbours(extract_value_rust_result!(kwargs, "max_neighbours", NodeT))?
            .set_iterations(extract_value_rust_result!(kwargs, "iterations", NodeT))?
            .set_dense_node_mapping(
                extract_value_rust_result!(kwargs, "dense_node_mapping", HashMap<NodeT, NodeT>),
            ))
    }
}
