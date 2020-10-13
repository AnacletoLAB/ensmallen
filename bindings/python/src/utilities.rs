use super::*;
use graph::{EdgeFileReader, NodeFileReader, NodeT, WalksParameters, WeightT};
use std::collections::HashMap;

pub(crate) fn build_csv_file_reader(
    edge_path: String,
    py_kwargs: Option<&PyDict>,
) -> Result<(EdgeFileReader, Option<NodeFileReader>), String> {
    let py = pyo3::Python::acquire_gil();
    let kwargs = normalize_kwargs!(py_kwargs, py.python());

    validate_kwargs(
        kwargs,
        [
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
            "skip_self_loops",
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
            "node_max_rows_number",
            "verbose",
            "numeric_node_ids",
            "numeric_node_type_ids",
            "numeric_edge_type_ids",
        ]
        .iter()
        .map(|x| x.to_string())
        .collect(),
    )?;

    let edges: EdgeFileReader = EdgeFileReader::new(edge_path)?
        .set_separator(extract_value!(kwargs, "edge_separator", String)?)?
        .set_header(extract_value!(kwargs, "edge_header", bool)?)
        .set_rows_to_skip(extract_value!(kwargs, "edge_rows_to_skip", usize)?)
        .set_sources_column_number(extract_value!(kwargs, "sources_column_number", usize)?)?
        .set_sources_column(extract_value!(kwargs, "sources_column", String)?)?
        .set_destinations_column_number(extract_value!(
            kwargs,
            "destinations_column_number",
            usize
        )?)?
        .set_destinations_column(extract_value!(kwargs, "destinations_column", String)?)?
        .set_edge_types_column_number(extract_value!(kwargs, "edge_types_column_number", usize)?)?
        .set_edge_types_column(extract_value!(kwargs, "edge_types_column", String)?)?
        .set_default_edge_type(extract_value!(kwargs, "default_edge_type", String)?)
        .set_weights_column_number(extract_value!(kwargs, "weights_column_number", usize)?)?
        .set_weights_column(extract_value!(kwargs, "weights_column", String)?)?
        .set_default_weight(extract_value!(kwargs, "default_weight", WeightT)?)
        .set_skip_self_loops(extract_value!(kwargs, "skip_self_loops", bool)?)
        .set_ignore_duplicates(extract_value!(kwargs, "ignore_duplicated_edges", bool)?)
        .set_verbose(extract_value!(kwargs, "verbose", bool)?)
        .set_numeric_node_ids(extract_value!(kwargs, "numeric_node_ids", bool)?)
        .set_numeric_edge_type_ids(extract_value!(kwargs, "numeric_edge_type_ids", bool)?)
        .set_max_rows_number(extract_value!(kwargs, "edge_max_rows_number", u64)?);

    let nodes: Option<NodeFileReader> = match kwargs.get_item("node_path") {
        Some(_) => Some(
            NodeFileReader::new(extract_value!(kwargs, "node_path", String)?.unwrap())?
                .set_separator(extract_value!(kwargs, "node_separator", String)?)?
                .set_header(extract_value!(kwargs, "edge_header", bool)?)
                .set_rows_to_skip(extract_value!(kwargs, "edge_rows_to_skip", usize)?)
                .set_nodes_column_number(extract_value!(kwargs, "nodes_column_number", usize)?)?
                .set_nodes_column(extract_value!(kwargs, "nodes_column", String)?)?
                .set_node_types_column_number(extract_value!(
                    kwargs,
                    "node_types_column_number",
                    usize
                )?)?
                .set_node_types_column(extract_value!(kwargs, "node_types_column", String)?)?
                .set_default_node_type(extract_value!(kwargs, "default_node_type", String)?)
                .set_ignore_duplicates(extract_value!(kwargs, "ignore_duplicated_nodes", bool)?)
                .set_verbose(extract_value!(kwargs, "verbose", bool)?)
                .set_numeric_node_ids(extract_value!(kwargs, "numeric_node_ids", bool)?)
                .set_numeric_node_type_ids(extract_value!(kwargs, "numeric_node_type_ids", bool)?)
                .set_max_rows_number(extract_value!(kwargs, "node_max_rows_number", u64)?),
        ),
        None => None,
    };

    Ok((edges, nodes))
}

impl EnsmallenGraph {
    pub(crate) fn build_walk_parameters(
        &self,
        length: NodeT,
        kwargs: &PyDict,
    ) -> Result<WalksParameters, String> {
        Ok(WalksParameters::new(length)?
            .set_change_edge_type_weight(extract_value!(
                kwargs,
                "change_edge_type_weight",
                WeightT
            )?)?
            .set_change_node_type_weight(extract_value!(
                kwargs,
                "change_node_type_weight",
                WeightT
            )?)?
            .set_explore_weight(extract_value!(kwargs, "explore_weight", WeightT)?)?
            .set_return_weight(extract_value!(kwargs, "return_weight", WeightT)?)?
            .set_random_state(extract_value!(kwargs, "random_state", usize)?)
            .set_verbose(extract_value!(kwargs, "verbose", bool)?)
            .set_iterations(extract_value!(kwargs, "iterations", NodeT)?)?
            .set_min_length(extract_value!(kwargs, "min_length", NodeT)?)?
            .set_dense_node_mapping(
                extract_value!(kwargs, "dense_node_mapping", HashMap<NodeT, NodeT>)?,
            ))
    }
}
