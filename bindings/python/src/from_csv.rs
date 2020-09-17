use super::*;
use graph::{CSVFileReader, EdgeFileReader, NodeFileReader, Graph};
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
    fn from_csv(
        edge_path: &str,
        directed: bool,
        py_kwargs: Option<&PyDict>,
    ) -> PyResult<Self> {
        let kwargs = normalize_kwargs!(py_kwargs);
        validate_kwargs(
            kwargs,
            &[
                "edge_sep",
                "weights_column",
                "default_weight",
                "node_path",
                "nodes_column",
                "node_types_column",
                "default_node_type",
                "node_sep",
                "ignore_duplicated_nodes",
                "edge_types_column",
                "default_edge_type",
                "ignore_duplicated_edges",
                "force_conversion_to_undirected",
                "validate_input_data",
            ],
        )?;

        let edges = EdgeFileReader::new(edge_path)
            .set_sources_column(extract_value!(kwargs, "sources_column", &str));

        let weights_column = extract_value!(kwargs, "weights_column", &str);
        if let Some(wc) = weights_column {
            result = result.set_weights(wc, extract_value!(kwargs, "default_weight", WeightT));
        }
        let node_path = extract_value!(kwargs, "node_path", &str);
        let nodes_column = extract_value!(kwargs, "nodes_column", &str);
        let node_types_column = extract_value!(kwargs, "node_types_column", &str);
        let default_node_type = extract_value!(kwargs, "default_node_type", &str);
        let node_sep = extract_value!(kwargs, "node_sep", &str);
        let ignore_duplicated_nodes = extract_value!(kwargs, "ignore_duplicated_nodes", bool);

        if node_path.is_some() {
            result = match result.load_nodes_csv(
                node_path.unwrap(),
                nodes_column.unwrap(),
                node_types_column.unwrap(),
                default_node_type,
                node_sep,
                ignore_duplicated_nodes,
            ) {
                Ok(g) => Ok(g),
                Err(e) => Err(PyErr::new::<exceptions::ValueError, _>(e)),
            }?;
        }

        let edge_types_column = extract_value!(kwargs, "edge_types_column", &str);
        if let Some(etc) = edge_types_column {
            result = result.set_edge_types(etc, extract_value!(kwargs, "default_edge_type", &str));
        }

        let ignore_duplicated_edges = extract_value!(kwargs, "ignore_duplicated_edges", bool);
        if let Some(ide) = ignore_duplicated_edges {
            if ide {
                result = result.set_ignore_duplicated_edges();
            }
        }

        match result.build() {
            Ok(g) => Ok(EnsmallenGraph { graph: g }),
            Err(e) => Err(PyErr::new::<exceptions::ValueError, _>(e)),
        }
    }
}
