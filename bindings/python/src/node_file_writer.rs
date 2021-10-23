use super::*;
use graph::NodeFileWriter;

#[pymethods]
impl Graph {
    #[args(py_kwargs = "**")]
    #[text_signature = "($self, path, *, verbose, separator, header, nodes_column_number, nodes_column, node_types_column_number, nodes_type_column)"]
    /// Write to disk the nodes (and optionally the metadata) of the graph.
    ///
    /// Parameters
    /// ----------
    /// path: str
    ///     Path where to save the nodes and their metadata.
    /// verbose: bool = True
    ///     Wether to show a loading bar while writing to file.
    /// separator: str = '\t'
    ///     What separator to use while writing out to file.
    /// header: bool = True
    ///     Wether to write out the header of the file.
    /// nodes_column_number: int = 0
    ///     The column number where to write the nodes.
    /// nodes_column: str = "id"
    ///     The name of the column of the nodes.
    /// node_types_column_number: int = 1
    ///     The column number where to write the node types.
    /// nodes_type_column: str = "category"
    ///     The name of the column of the node types.
    ///
    /// Raises
    /// ------
    /// TODO: update the set of exceptions
    ///
    fn dump_nodes(&self, path: String, py_kwargs: Option<&PyDict>) -> PyResult<()> {
        let py = pyo3::Python::acquire_gil();
        let kwargs = normalize_kwargs!(py_kwargs, py.python());

        pe!(validate_kwargs(
            kwargs,
            &[
                "verbose",
                "separator",
                "header",
                "nodes_column_number",
                "nodes_column",
                "node_types_column_number",
                "nodes_type_column",
            ],
        ))?;

        let writer = pe!(NodeFileWriter::new(path)
            .set_verbose(extract_value!(kwargs, "verbose", bool))
            .set_separator(extract_value!(kwargs, "separator", String)))?
        .set_header(extract_value!(kwargs, "header", bool))
        .set_nodes_column_number(extract_value!(kwargs, "nodes_column_number", usize))
        .set_nodes_column(extract_value!(kwargs, "nodes_column", String))
        .set_node_types_column_number(extract_value!(kwargs, "node_types_column_number", usize))
        .set_node_types_column(extract_value!(kwargs, "nodes_type_column", String));
        pe!(writer.dump_graph(&self.inner))
    }
}
