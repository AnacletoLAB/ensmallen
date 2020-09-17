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
    /// Parameters
    /// ---------------------
    /// edge_path:str,
    ///     Path to CSV file from where to load the edge data.
    /// sources_column:str,
    ///     Column name of the edge file where the source nodes are listed.
    /// destinations_column:str,
    ///     Column name of the edge file where the destination nodes are listed.
    /// directed:bool,
    ///     Boolean representing if given graph is directed or undirected.
    /// edge_types_column:str,
    ///     Column name of the edge file where the edge types are listed.
    /// default_edge_type:str,
    ///     The default edge type to use when an empty edge type is found in the
    ///     provided edge file. It is REQUIRED when passing an edge types column.
    /// weights_column:str,
    ///     Column name of the edge file where the edge weights are listed.
    /// default_weight:float,
    ///     The default weight to use when an empty weight is found in the
    ///     provided edge file. It is REQUIRED when passing a weights column.
    /// node_path:str,
    ///     Path to CSV file from where to load the node data.
    /// nodes_column:str,
    ///     Column name of the node file where the nodes names are listed.
    /// default_node_type:str,
    ///     The default node type to use when an empty node type is found in the
    ///     provided node file. It is REQUIRED when passing an node types column.
    /// node_types_column:str,
    ///     Column name of the node file where the node types are listed.
    /// edge_sep:str="\t",
    ///     Separator to use for the edge files.
    /// node_sep:str="\t",
    ///     Separator to use for the node files.
    /// ignore_duplicated_edges:bool=False,
    ///     Wethever to ignore duplicated edges or to raise an exception.
    ///     The duplication includes the edge type, if provided, so for example
    ///     an edge from A to B of type 1 is different from an edge A to B
    ///     of type 2.
    ///     The default behaviour is to raise an exception.
    /// ignore_duplicated_nodes:bool=False,
    ///     Wethever to ignore duplicated nodes or to raise an exception.
    ///     The default behaviour is to raise an exception.
    ///
    fn from_csv(edge_path: String, directed: bool, py_kwargs: Option<&PyDict>) -> PyResult<EnsmallenGraph> {
        let kwargs = normalize_kwargs!(py_kwargs);

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
