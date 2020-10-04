use super::*;
use graph::{EdgeFileReader, EdgeT, Graph, NodeFileReader, NodeT, WeightT};

#[pymethods]
impl EnsmallenGraph {
    #[staticmethod]
    #[args(py_kwargs = "**")]
    #[text_signature = "(edge_path, directed, *, sources_column_number, sources_column, destinations_column_number, destinations_column, edge_types_column_number, edge_types_column, default_edge_type, weights_column_number, weights_column, default_weight, skip_self_loops, ignore_duplicated_edges, edge_header, edge_rows_to_skip, edge_separator, node_path, nodes_column_number, nodes_column, node_types_column_number, node_types_column, default_node_type, ignore_duplicated_nodes, node_header, node_rows_to_skip, node_separator, verbose, numeric_node_ids, numeric_node_type_ids, numeric_edge_type_ids)"]
    /// Return graph loaded from given edge file and optionally node file.
    ///
    /// Parameters
    /// -------------------------------
    /// edge_path: String,
    ///     The path from where load the edge file.
    /// directed: bool,
    ///     Wethever to load the graph as directed or undirected.
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
    ///     Wethever to skip self loops while loading the edge file.
    /// ignore_duplicated_edges: bool = True,
    ///     Wethever to skip duplicated edges while loading the edge file.
    ///     When NOT ignoring the duplicated edges, an exception with information
    ///     on the duplicated edge will be raised.
    ///     When ignoring the edge type while reading the file duplicated edges
    ///     in a multi-graph will be marked as duplicates.
    /// edge_header: bool = True,
    ///     Wethever to expect the first line of the edge file to be a header.
    /// edge_rows_to_skip: int = 0,
    ///     If the edge file has some descriptive text in the first few lines,
    ///     this is the parameter that allows you to skip it.
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
    ///     Wethever to skip duplicated nodes while loading the node file.
    ///     When NOT ignoring the duplicated nodes, an exception with information
    ///     on the duplicated node will be raised.
    /// node_header: bool = True,
    ///     Wethever to expect the first line of the node file to be a header.
    /// node_rows_to_skip: int = 0,
    ///     If the node file has some descriptive text in the first few lines,
    ///     this is the parameter that allows you to skip it.
    /// node_separator: str = "\t",
    ///      The expected separator for the node file.
    /// verbose: bool = True,
    ///     Wethever to load the files verbosely, showing a loading bar.
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
                "cached_edges_number"
            ]
            .iter()
            .map(|x| x.to_string())
            .collect(),
        )?;

        let edges: EdgeFileReader = pyex!(pyex!(pyex!(pyex!(pyex!(pyex!(pyex!(pyex!(
            pyex!(EdgeFileReader::new(edge_path))?
                .set_separator(extract_value!(kwargs, "edge_separator", String))
                .set_header(extract_value!(kwargs, "edge_header", bool))
                .set_rows_to_skip(extract_value!(kwargs, "edge_rows_to_skip", usize))
                .set_sources_column_number(extract_value!(kwargs, "sources_column_number", usize))
        )?
        .set_sources_column(extract_value!(kwargs, "sources_column", String)))?
        .set_destinations_column_number(extract_value!(
            kwargs,
            "destinations_column_number",
            usize
        )))?
        .set_destinations_column(extract_value!(kwargs, "destinations_column", String)))?
        .set_edge_types_column_number(extract_value!(
            kwargs,
            "edge_types_column_number",
            usize
        )))?
        .set_edge_types_column(extract_value!(kwargs, "edge_types_column", String)))?
        .set_default_edge_type(extract_value!(kwargs, "default_edge_type", String))
        .set_weights_column_number(extract_value!(kwargs, "weights_column_number", usize)))?
        .set_weights_column(extract_value!(kwargs, "weights_column", String)))?
        .set_default_weight(extract_value!(kwargs, "default_weight", WeightT))
        .set_skip_self_loops(extract_value!(kwargs, "skip_self_loops", bool))
        .set_ignore_duplicates(extract_value!(kwargs, "ignore_duplicated_edges", bool))
        .set_verbose(extract_value!(kwargs, "verbose", bool))
        .set_numeric_node_ids(extract_value!(kwargs, "numeric_node_ids", bool))
        .set_numeric_edge_type_ids(extract_value!(kwargs, "numeric_edge_type_ids", bool))
        .set_cached_edges_number(extract_value!(kwargs, "cached_edges_number", EdgeT))
        .set_max_rows_number(extract_value!(kwargs, "edge_max_rows_number", u64));

        let nodes: Option<NodeFileReader> =
            match kwargs.get_item("node_path") {
                Some(_) => Some(
                    pyex!(pyex!(pyex!(pyex!(pyex!(NodeFileReader::new(
                        extract_value!(kwargs, "node_path", String).unwrap()
                    ))?
                    .set_separator(extract_value!(kwargs, "node_separator", String))
                    .set_header(extract_value!(kwargs, "edge_header", bool))
                    .set_rows_to_skip(extract_value!(kwargs, "edge_rows_to_skip", usize))
                    .set_nodes_column_number(extract_value!(
                        kwargs,
                        "nodes_column_number",
                        usize
                    )))?
                    .set_nodes_column(extract_value!(kwargs, "nodes_column", String)))?
                    .set_node_types_column_number(extract_value!(
                        kwargs,
                        "node_types_column_number",
                        usize
                    )))?
                    .set_node_types_column(extract_value!(
                        kwargs,
                        "node_types_column",
                        String
                    )))?
                    .set_default_node_type(extract_value!(kwargs, "default_node_type", String))
                    .set_ignore_duplicates(extract_value!(kwargs, "ignore_duplicated_nodes", bool))
                    .set_verbose(extract_value!(kwargs, "verbose", bool))
                    .set_numeric_node_ids(extract_value!(kwargs, "numeric_node_ids", bool))
                    .set_numeric_node_type_ids(extract_value!(kwargs, "numeric_node_type_ids", bool))
                    .set_max_rows_number(extract_value!(
                        kwargs,
                        "node_max_rows_number",
                        u64
                    )),
                ),
                None => None,
            };

        Ok(EnsmallenGraph {
            graph: pyex!(Graph::from_unsorted_csv(edges, nodes, directed))?,
        })
    }

    #[staticmethod]
    #[args(py_kwargs = "**")]
    #[text_signature = "(edge_path, directed, *, sources_column_number, sources_column, destinations_column_number, destinations_column, edge_types_column_number, edge_types_column, default_edge_type, weights_column_number, weights_column, default_weight, skip_self_loops, ignore_duplicated_edges, edge_header, edge_rows_to_skip, edge_separator, node_path, nodes_column_number, nodes_column, node_types_column_number, node_types_column, default_node_type, ignore_duplicated_nodes, node_header, node_rows_to_skip, node_separator, verbose, numeric_node_ids, numeric_node_type_ids, numeric_edge_type_ids)"]
    /// Return graph loaded from given edge file and optionally node file.
    ///
    /// TODO: update docstrinG!!!
    ///
    /// Parameters
    /// -------------------------------
    /// edge_path: String,
    ///     The path from where load the edge file.
    /// directed: bool,
    ///     Wethever to load the graph as directed or undirected.
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
    ///     Wethever to skip self loops while loading the edge file.
    /// ignore_duplicated_edges: bool = True,
    ///     Wethever to skip duplicated edges while loading the edge file.
    ///     When NOT ignoring the duplicated edges, an exception with information
    ///     on the duplicated edge will be raised.
    ///     When ignoring the edge type while reading the file duplicated edges
    ///     in a multi-graph will be marked as duplicates.
    /// edge_header: bool = True,
    ///     Wethever to expect the first line of the edge file to be a header.
    /// edge_rows_to_skip: int = 0,
    ///     If the edge file has some descriptive text in the first few lines,
    ///     this is the parameter that allows you to skip it.
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
    ///     Wethever to skip duplicated nodes while loading the node file.
    ///     When NOT ignoring the duplicated nodes, an exception with information
    ///     on the duplicated node will be raised.
    /// node_header: bool = True,
    ///     Wethever to expect the first line of the node file to be a header.
    /// node_rows_to_skip: int = 0,
    ///     If the node file has some descriptive text in the first few lines,
    ///     this is the parameter that allows you to skip it.
    /// node_separator: str = "\t",
    ///      The expected separator for the node file.
    /// verbose: bool = True,
    ///     Wethever to load the files verbosely, showing a loading bar.
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

        let edges: EdgeFileReader = pyex!(pyex!(pyex!(pyex!(pyex!(pyex!(pyex!(pyex!(
            pyex!(EdgeFileReader::new(edge_path))?
                .set_separator(extract_value!(kwargs, "edge_separator", String))
                .set_header(extract_value!(kwargs, "edge_header", bool))
                .set_rows_to_skip(extract_value!(kwargs, "edge_rows_to_skip", usize))
                .set_sources_column_number(extract_value!(kwargs, "sources_column_number", usize))
        )?
        .set_sources_column(extract_value!(kwargs, "sources_column", String)))?
        .set_destinations_column_number(extract_value!(
            kwargs,
            "destinations_column_number",
            usize
        )))?
        .set_destinations_column(extract_value!(kwargs, "destinations_column", String)))?
        .set_edge_types_column_number(extract_value!(
            kwargs,
            "edge_types_column_number",
            usize
        )))?
        .set_edge_types_column(extract_value!(kwargs, "edge_types_column", String)))?
        .set_default_edge_type(extract_value!(kwargs, "default_edge_type", String))
        .set_weights_column_number(extract_value!(kwargs, "weights_column_number", usize)))?
        .set_weights_column(extract_value!(kwargs, "weights_column", String)))?
        .set_default_weight(extract_value!(kwargs, "default_weight", WeightT))
        .set_skip_self_loops(extract_value!(kwargs, "skip_self_loops", bool))
        .set_ignore_duplicates(extract_value!(kwargs, "ignore_duplicated_edges", bool))
        .set_verbose(extract_value!(kwargs, "verbose", bool))
        .set_numeric_node_ids(extract_value!(kwargs, "numeric_node_ids", bool))
        .set_numeric_edge_type_ids(extract_value!(kwargs, "numeric_edge_type_ids", bool))
        .set_max_rows_number(extract_value!(kwargs, "edge_max_rows_number", u64));

        let nodes: Option<NodeFileReader> =
            match kwargs.get_item("node_path") {
                Some(_) => Some(
                    pyex!(pyex!(pyex!(pyex!(pyex!(NodeFileReader::new(
                        extract_value!(kwargs, "node_path", String).unwrap()
                    ))?
                    .set_separator(extract_value!(kwargs, "node_separator", String))
                    .set_header(extract_value!(kwargs, "edge_header", bool))
                    .set_rows_to_skip(extract_value!(kwargs, "edge_rows_to_skip", usize))
                    .set_nodes_column_number(extract_value!(
                        kwargs,
                        "nodes_column_number",
                        usize
                    )))?
                    .set_nodes_column(extract_value!(kwargs, "nodes_column", String)))?
                    .set_node_types_column_number(extract_value!(
                        kwargs,
                        "node_types_column_number",
                        usize
                    )))?
                    .set_node_types_column(extract_value!(
                        kwargs,
                        "node_types_column",
                        String
                    )))?
                    .set_default_node_type(extract_value!(kwargs, "default_node_type", String))
                    .set_ignore_duplicates(extract_value!(kwargs, "ignore_duplicated_nodes", bool))
                    .set_verbose(extract_value!(kwargs, "verbose", bool))
                    .set_numeric_node_ids(extract_value!(kwargs, "numeric_node_ids", bool))
                    .set_numeric_node_type_ids(extract_value!(kwargs, "numeric_node_type_ids", bool))
                    .set_max_rows_number(extract_value!(
                        kwargs,
                        "node_max_rows_number",
                        u64
                    )),
                ),
                None => None,
            };

        Ok(EnsmallenGraph {
            graph: pyex!(Graph::from_sorted_csv(
                edges,
                nodes,
                directed,
                edges_number,
                nodes_number
            ))?,
        })
    }
}
