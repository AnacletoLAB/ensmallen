use super::*;
use graph::EdgeFileWriter;
use pyo3::types::PyDict;

#[pymethods]
impl EnsmallenGraph {
    #[args(py_kwargs = "**")]
    #[text_signature = "($self, path, *, verbose, separator, header, sources_column_number, sources_column, destinations_column_number, destinations_column, weights_column_number, weights_column, edge_types_column_number, edges_type_column, numeric_node_ids, directed)"]
    /// Write to disk the edges (and optionally the metadata) of the graph.
    ///
    /// Parameters
    /// ------------------------
    /// path: str,
    ///     Path where to save the edges and their metadata.
    /// verbose: bool = True,
    ///     Wether to show a loading bar while writing to file.
    /// separator: str = "\t",
    ///     What separator to use while writing out to file.
    /// header: bool = True,
    ///     Wether to write out the header of the file.
    /// sources_column_number: int = 0,
    ///     The column number where to write out the .
    /// sources_column: str = "subject",
    ///     The name of the column where to write out the .
    /// destinations_column_number: int = 1,
    ///     The column number where to write out the .
    /// destinations_column: str = "object",
    ///     The name of the column where to write out the .
    /// edge_types_column_number: int = 2,
    ///     The column number where to write out the .
    /// edges_type_column: str = "label",
    ///     The name of the column where to write out the .
    /// weights_column_number: int = 3,
    ///     The column number where to write out the .
    /// weights_column: str = "weight",
    ///     The name of the column where to write out the .
    /// numeric_node_ids: bool = False,
    ///     whether to save the internal numeric Ids instead of the string names.
    /// directed: bool = False,
    ///     whether to save graph as directed or undirected.
    ///
    /// Raises
    /// ------------------------
    /// TODO: update the set of exceptions
    ///
    fn dump_edges(&self, path: String, py_kwargs: Option<&PyDict>) -> PyResult<()> {
        let py = pyo3::Python::acquire_gil();
        let kwargs = normalize_kwargs!(py_kwargs, py.python());

        pe!(validate_kwargs(
            kwargs,
            &[
                "verbose",
                "separator",
                "header",
                "sources_column_number",
                "sources_column",
                "destinations_column_number",
                "destinations_column",
                "weights_column_number",
                "weights_column",
                "edge_types_column_number",
                "edges_type_column",
                "numeric_node_ids",
                "directed"
            ]
        ))?;

        let writer = EdgeFileWriter::new(path)
            .set_verbose(extract_value!(kwargs, "verbose", bool))
            .set_separator(extract_value!(kwargs, "separator", String))
            .set_header(extract_value!(kwargs, "header", bool))
            .set_directed(extract_value!(kwargs, "directed", bool))
            .set_sources_column_number(extract_value!(kwargs, "sources_column_number", usize))
            .set_sources_column(extract_value!(kwargs, "sources_column", String))
            .set_destinations_column_number(extract_value!(
                kwargs,
                "destinations_column_number",
                usize
            ))
            .set_destinations_column(extract_value!(kwargs, "destinations_column", String))
            .set_weights_column_number(extract_value!(kwargs, "weights_column_number", usize))
            .set_weights_column(extract_value!(kwargs, "weights_column", String))
            .set_edge_types_column_number(extract_value!(kwargs, "edge_types_column_number", usize))
            .set_numeric_node_ids(extract_value!(kwargs, "numeric_node_ids", bool))
            .set_edge_types_column(extract_value!(kwargs, "edges_type_column", String));
        pe!(writer.dump_graph(&self.graph))
    }
}
