use super::*;
use graph::{EdgeFileReader, Graph, NodeFileReader, WeightT};
use pyo3::exceptions;
use pyo3::prelude::*;
use pyo3::types::PyDict;

#[pymethods]
impl EnsmallenGraph {
    #[staticmethod]
    #[args(py_kwargs = "**")]
    #[text_signature = "(edge_path, sources_column, destinations_column, directed, *, edge_types_column, default_edge_type, weights_column, default_weight, node_path, nodes_column, node_types_column, default_node_type, edge_sep, node_sep, ignore_duplicated_edges, ignore_duplicated_nodes, force_conversion_to_undirected)"]
    /// Build the graph from a csv (or tsv) in Rust.
    /// 
    /// TODO: update docstring
    fn from_csv(edge_path: String, directed: bool, py_kwargs: Option<&PyDict>) -> PyResult<EnsmallenGraph> {
        let py = pyo3::Python::acquire_gil();
        let kwargs = normalize_kwargs!(py_kwargs, py.python());

        let edges: EdgeFileReader =
            pyex!(pyex!(pyex!(pyex!(pyex!(EdgeFileReader::new(edge_path))?
                .set_sources_column_number(extract_value!(kwargs, "sources_column_number", usize))
                .set_sources_column(extract_value!(kwargs, "sources_column", String)))?
            .set_destinations_column_number(extract_value!(
                kwargs,
                "destinations_column_number",
                usize
            ))
            .set_destinations_column(extract_value!(kwargs, "destinations_column", String)))?
            .set_edge_types_column_number(extract_value!(kwargs, "edge_types_column_number", usize))
            .set_edge_types_column(extract_value!(kwargs, "edge_types_column", String)))?
            .set_default_edge_type(extract_value!(kwargs, "default_edge_type", String))
            .set_weights_column_number(extract_value!(kwargs, "weights_column_number", usize))
            .set_weights_column(extract_value!(kwargs, "weights_column", String)))?
            .set_default_weight(extract_value!(kwargs, "default_weight", WeightT))
            .set_skip_self_loops(extract_value!(kwargs, "skip_self_loops", bool))
            .set_ignore_duplicates(extract_value!(kwargs, "ignore_duplicated_edges", bool))
            .set_header(extract_value!(kwargs, "header", bool))
            .set_rows_to_skip(extract_value!(kwargs, "rows_to_skip", usize))
            .set_separator(extract_value!(kwargs, "separator", String));

        let nodes: Option<NodeFileReader> = match kwargs.get_item("node_path") {
            Some(_) => Some(
                pyex!(pyex!(pyex!(NodeFileReader::new(
                    extract_value!(kwargs, "node_path", String).unwrap()
                ))?
                .set_nodes_column_number(extract_value!(kwargs, "nodes_column_number", usize))
                .set_nodes_column(extract_value!(kwargs, "nodes_column", String)))?
                .set_node_types_column_number(extract_value!(
                    kwargs,
                    "node_types_column_number",
                    usize
                ))
                .set_node_types_column(extract_value!(kwargs, "node_types_column", String)))?
                .set_ignore_duplicates(extract_value!(kwargs, "ignore_duplicated_nodes", bool))
                .set_header(extract_value!(kwargs, "header", bool))
                .set_rows_to_skip(extract_value!(kwargs, "rows_to_skip", usize))
                .set_verbose(extract_value!(kwargs, "verbose", bool)),
            ),
            None => None,
        };

        Ok(EnsmallenGraph {
            graph: pyex!(Graph::from_csv(edges, nodes, directed))?
        })
    }
}
