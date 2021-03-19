use super::*;
use graph::{EdgeT, Graph, NodeT};

#[pymethods]
impl EnsmallenGraph {
    #[staticmethod]
    #[args(py_kwargs = "**")]
    #[text_signature = "(edge_path, directed, *, directed_edge_list, sources_column_number, sources_column, destinations_column_number, destinations_column, edge_types_column_number, edge_types_column, default_edge_type, weights_column_number, weights_column, default_weight, skip_self_loops, ignore_duplicated_edges, edge_header, edge_rows_to_skip, edge_max_rows_number, edge_separator, node_path, nodes_column_number, nodes_column, node_types_column_number, node_types_column, default_node_type, ignore_duplicated_nodes, node_header, node_rows_to_skip, node_max_rows_number, node_separator, numeric_node_ids, numeric_edge_node_ids, numeric_node_type_ids, numeric_edge_type_ids, edge_file_comment_symbol, node_file_comment_symbol, skip_weights_if_unavailable, skip_edge_types_if_unavailable, skip_node_types_if_unavailable, name, verbose)"]
    /// Return graph loaded from given edge file and optionally node file.
    ///
    /// Parameters
    /// -------------------------------
    /// edge_path: String,
    ///     The path from where load the edge file.
    /// directed: bool,
    ///     whether to load the graph as directed or undirected.
    /// directed_edge_list: bool = False,
    ///     Wether to load the edge list as directed or undirected.
    ///     The default behaviour is to treat he list as undirected and handle the
    ///     undirected edges automatically if the parameter `directed=False`.
    /// sources_column_number: int = 0,
    ///     The column number of the sources of the edges.
    ///     This value is overwritten by the source column value if one is provided.
    ///     If the edge file you are loading does not have a header, remember
    ///     to set the edge_header parameter to false.
    /// sources_column: str = None,
    ///     Name of the column to be loaded as source of the edges.
    /// destinations_column_number: int = 1,
    ///     The column number of the destinations of the edges.
    ///     This value is overwritten by the destination column value if one is provided.
    ///     If the edge file you are loading does not have a header, remember
    ///     to set the edge_header parameter to false.
    /// destinations_column: str = None,
    ///     Name of the column to be loaded as destination of the edges.
    /// edge_types_column_number: int = None,
    ///     The column number of the edge type of the edges.
    ///     This value is overwritten by the edge types column value if one is provided.
    ///     If the edge file you are loading does not have a header, remember
    ///     to set the edge_header parameter to false.
    /// edge_types_column: str = None,
    ///     Name of the column to be loaded as edge type of the edges.
    /// default_edge_type: str = None,
    ///     String representing the default edge type to use when the edge type
    ///     in the provided column is empty.
    /// weights_column_number: int = None,
    ///     The column number of the weight of the edges.
    ///     This value is overwritten by the weights column value if one is provided.
    ///     If the edge file you are loading does not have a header, remember
    ///     to set the edge_header parameter to false.
    /// weights_column: str = None,
    ///     Name of the column to be loaded as weight of the edges.
    /// default_weight: float = None,
    ///     String representing the default edge type to use when the edge type
    ///     in the provided column is empty.
    /// skip_self_loops: bool = False,
    ///     whether to skip self loops while loading the edge file.
    /// ignore_duplicated_edges: bool = True,
    ///     whether to skip duplicated edges while loading the edge file.
    ///     When NOT ignoring the duplicated edges, an exception with information
    ///     on the duplicated edge will be raised.
    ///     When ignoring the edge type while reading the file duplicated edges
    ///     in a multi-graph will be marked as duplicates.
    /// edge_header: bool = True,
    ///     whether to expect the first line of the edge file to be a header.
    /// edge_rows_to_skip: int = 0,
    ///     If the edge file has some descriptive text in the first few lines,
    ///     this is the parameter that allows you to skip it.
    /// edge_max_rows_number: int = None,
    ///     Number of rows to read.
    /// edge_separator: str = "\t",
    ///     The expected separator for the edge file.
    /// node_path: str = None,
    ///     The path from where to load the node file.
    ///     If one is not provided, no node types will be loaded and the graph
    ///     might end-up with node IDs that are not aligned with other subgraphs
    ///     from the same edge file.
    /// nodes_column_number: int = None,
    ///     The column number of the node Ids.
    ///     This value is overwritten by the nodes column value if one is provided.
    ///     If the node file you are loading does not have a header, remember
    ///     to set the node_header parameter to false.
    /// nodes_column: str = None,
    ///     Name of the column to be loaded as node Ids.
    /// node_types_column_number: int = None,
    ///     The column number of the node type of the nodes.
    ///     This value is overwritten by the node types column value if one is provided.
    ///     If the node file you are loading does not have a header, remember
    ///     to set the node_header parameter to false.
    /// node_types_column: str = None,
    ///     Name of the column to be loaded as node types.
    /// default_node_type: str = None,
    ///     String representing the default node type to use when the node type
    ///     in the provided column is empty.
    /// ignore_duplicated_nodes: bool = True,
    ///     whether to skip duplicated nodes while loading the node file.
    ///     When NOT ignoring the duplicated nodes, an exception with information
    ///     on the duplicated node will be raised.
    /// node_header: bool = True,
    ///     whether to expect the first line of the node file to be a header.
    /// node_rows_to_skip: int = 0,
    ///     If the node file has some descriptive text in the first few lines,
    ///     this is the parameter that allows you to skip it.
    /// node_max_rows_number: int = None,
    ///     Number of rows to read.
    /// node_separator: str = "\t",
    ///      The expected separator for the node file.
    /// numeric_node_ids: bool = False,
    ///     Wether to load the Node Ids as numeric.
    /// numeric_edge_node_ids: bool = False,
    ///     Wether to load the edge file Node Ids as numeric.
    /// numeric_node_type_ids: bool = False,
    ///     Wether to load the Node Type Ids as numeric.
    /// numeric_edge_type_ids: bool = False,
    ///     Wether to load the Edge Type Ids as numeric.
    /// edge_file_comment_symbol: str = None,
    ///     The symbol to use for the lines to be ignored in the edge file.
    /// node_file_comment_symbol: str = None,
    ///     The symbol to use for the lines to be ignored in the node file.
    /// skip_weights_if_unavailable: bool = False,
    ///     Wether to skip the loading of the weights even if requested but
    ///     in the file the column is actually unavailable.
    /// skip_edge_types_if_unavailable: bool = False,
    ///     Wether to skip the loading of the edge types even if requested but
    ///     in the file the column is actually unavailable.
    /// skip_node_types_if_unavailable: bool = False,
    ///     Wether to skip the loading of the node types even if requested but
    ///     in the file the column is actually unavailable.
    /// name: str = "Graph",
    ///     The name of the graph to use.
    /// verbose: bool = True,
    ///     whether to load the files verbosely, showing a loading bar.
    ///
    /// Raises
    /// ------------------------
    /// ValueError,
    ///     TODO: Update the list of raised exceptions.
    ///
    /// Returns
    /// ------------------------
    /// The loaded graph.
    fn from_unsorted_csv(
        edge_path: String,
        directed: bool,
        py_kwargs: Option<&PyDict>,
    ) -> PyResult<EnsmallenGraph> {
        let _ = ctrlc::set_handler(|| std::process::exit(2));
        let (edges, nodes, name, directed_edge_list) =
            pyex!(build_csv_file_reader(edge_path, py_kwargs))?;

        Ok(EnsmallenGraph {
            graph: pyex!(Graph::from_unsorted_csv(
                edges,
                nodes,
                directed,
                directed_edge_list,
                name,
            ))?,
        })
    }

    #[staticmethod]
    #[args(py_kwargs = "**")]
    #[text_signature = "(edge_path, directed, *, directed_edge_list, sources_column_number, sources_column, destinations_column_number, destinations_column, edge_types_column_number, edge_types_column, default_edge_type, weights_column_number, weights_column, default_weight, skip_self_loops, ignore_duplicated_edges, edge_header, edge_rows_to_skip, edge_max_rows_number, edge_separator, node_path, nodes_column_number, nodes_column, node_types_column_number, node_types_column, default_node_type, ignore_duplicated_nodes, node_header, node_rows_to_skip, node_max_rows_number, node_separator, numeric_node_ids, numeric_edge_node_ids, numeric_node_type_ids, numeric_edge_type_ids, edge_file_comment_symbol, node_file_comment_symbol, skip_weights_if_unavailable, skip_edge_types_if_unavailable, skip_node_types_if_unavailable, name, verbose, )"]
    /// Return graph loaded from given edge file and optionally node file.
    ///
    /// Parameters
    /// -------------------------------
    /// edge_path: String,
    ///     The path from where load the edge file.
    /// directed: bool,
    ///     whether to load the graph as directed or undirected.
    /// directed_edge_list: bool = False,
    ///     Wether to load the edge list as directed or undirected.
    ///     The default behaviour is to the list as undirected and handle the
    ///     undirected edges automatically if the parameter `directed=False`.
    /// sources_column_number: int = 0,
    ///     The column number of the sources of the edges.
    ///     This value is overwritten by the source column value if one is provided.
    ///     If the edge file you are loading does not have a header, remember
    ///     to set the edge_header parameter to false.
    /// sources_column: str = None,
    ///     Name of the column to be loaded as source of the edges.
    /// destinations_column_number: int = 1,
    ///     The column number of the destinations of the edges.
    ///     This value is overwritten by the destination column value if one is provided.
    ///     If the edge file you are loading does not have a header, remember
    ///     to set the edge_header parameter to false.
    /// destinations_column: str = None,
    ///     Name of the column to be loaded as destination of the edges.
    /// edge_types_column_number: int = None,
    ///     The column number of the edge type of the edges.
    ///     This value is overwritten by the edge types column value if one is provided.
    ///     If the edge file you are loading does not have a header, remember
    ///     to set the edge_header parameter to false.
    /// edge_types_column: str = None,
    ///     Name of the column to be loaded as edge type of the edges.
    /// default_edge_type: str = None,
    ///     String representing the default edge type to use when the edge type
    ///     in the provided column is empty.
    /// weights_column_number: int = None,
    ///     The column number of the weight of the edges.
    ///     This value is overwritten by the weights column value if one is provided.
    ///     If the edge file you are loading does not have a header, remember
    ///     to set the edge_header parameter to false.
    /// weights_column: str = None,
    ///     Name of the column to be loaded as weight of the edges.
    /// default_weight: float = None,
    ///     String representing the default edge type to use when the edge type
    ///     in the provided column is empty.
    /// skip_self_loops: bool = False,
    ///     whether to skip self loops while loading the edge file.
    /// ignore_duplicated_edges: bool = True,
    ///     whether to skip duplicated edges while loading the edge file.
    ///     When NOT ignoring the duplicated edges, an exception with information
    ///     on the duplicated edge will be raised.
    ///     When ignoring the edge type while reading the file duplicated edges
    ///     in a multi-graph will be marked as duplicates.
    /// edge_header: bool = True,
    ///     whether to expect the first line of the edge file to be a header.
    /// edge_rows_to_skip: int = 0,
    ///     If the edge file has some descriptive text in the first few lines,
    ///     this is the parameter that allows you to skip it.
    /// edge_max_rows_number: int = None,
    ///     Number of rows to read.
    /// edge_separator: str = "\t",
    ///     The expected separator for the edge file.
    /// node_path: str = None,
    ///     The path from where to load the node file.
    ///     If one is not provided, no node types will be loaded and the graph
    ///     might end-up with node IDs that are not aligned with other subgraphs
    ///     from the same edge file.
    /// nodes_column_number: int = None,
    ///     The column number of the node Ids.
    ///     This value is overwritten by the nodes column value if one is provided.
    ///     If the node file you are loading does not have a header, remember
    ///     to set the node_header parameter to false.
    /// nodes_column: str = None,
    ///     Name of the column to be loaded as node Ids.
    /// node_types_column_number: int = None,
    ///     The column number of the node type of the nodes.
    ///     This value is overwritten by the node types column value if one is provided.
    ///     If the node file you are loading does not have a header, remember
    ///     to set the node_header parameter to false.
    /// node_types_column: str = None,
    ///     Name of the column to be loaded as node types.
    /// default_node_type: str = None,
    ///     String representing the default node type to use when the node type
    ///     in the provided column is empty.
    /// ignore_duplicated_nodes: bool = True,
    ///     whether to skip duplicated nodes while loading the node file.
    ///     When NOT ignoring the duplicated nodes, an exception with information
    ///     on the duplicated node will be raised.
    /// node_header: bool = True,
    ///     whether to expect the first line of the node file to be a header.
    /// node_rows_to_skip: int = 0,
    ///     If the node file has some descriptive text in the first few lines,
    ///     this is the parameter that allows you to skip it.
    /// node_max_rows_number: int = None,
    ///     Number of rows to read.
    /// node_separator: str = "\t",
    ///      The expected separator for the node file.
    /// numeric_node_ids: bool = False,
    ///     Wether to load the Node Ids as numeric.
    /// numeric_edge_node_ids: bool = False,
    ///     Wether to load the edge file Node Ids as numeric.
    /// numeric_node_type_ids: bool = False,
    ///     Wether to load the Node Type Ids as numeric.
    /// numeric_edge_type_ids: bool = False,
    ///     Wether to load the Edge Type Ids as numeric.
    /// edge_file_comment_symbol: str = None,
    ///     The symbol to use for the lines to be ignored in the edge file.
    /// node_file_comment_symbol: str = None,
    ///     The symbol to use for the lines to be ignored in the node file.
    /// skip_weights_if_unavailable: bool = False,
    ///     Wether to skip the loading of the weights even if requested but
    ///     in the file the column is actually unavailable.
    /// skip_edge_types_if_unavailable: bool = False,
    ///     Wether to skip the loading of the edge types even if requested but
    ///     in the file the column is actually unavailable.
    /// skip_node_types_if_unavailable: bool = False,
    ///     Wether to skip the loading of the node types even if requested but
    ///     in the file the column is actually unavailable.
    /// name: str = "Graph",
    ///     The name of the graph to use.
    /// verbose: bool = True,
    ///     whether to load the files verbosely, showing a loading bar.
    ///
    /// Raises
    /// ------------------------
    /// ValueError,
    ///     TODO: Update the list of raised exceptions.
    ///
    /// Returns
    /// ------------------------
    /// The loaded graph.
    fn from_sorted_csv(
        edge_path: String,
        directed: bool,
        nodes_number: NodeT,
        edges_number: EdgeT,
        py_kwargs: Option<&PyDict>,
    ) -> PyResult<EnsmallenGraph> {
        let _ = ctrlc::set_handler(|| std::process::exit(2));
        let (edges, nodes, name, directed_edge_list) =
            pyex!(build_csv_file_reader(edge_path, py_kwargs))?;

        Ok(EnsmallenGraph {
            graph: pyex!(Graph::from_sorted_csv(
                edges,
                nodes,
                directed,
                directed_edge_list,
                edges_number,
                nodes_number,
                name
            ))?,
        })
    }
}
