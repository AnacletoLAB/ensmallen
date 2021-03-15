use super::*;

/// Structure that saves the writer specific to writing and reading a nodes csv file.
///
/// # Attributes
/// * writer: CSVFileWriter - The common writer for readin and writing a csv.
/// * nodes_column: String - The name of the nodes names column. This parameter is mutually exclusive with nodes_column_number.
/// * nodes_column_number: usize - The rank of the column with the nodes names. This parameter is mutually exclusive with nodes_column.
/// * node_types_column: String - The name of the nodes type column. This parameter is mutually exclusive with node_types_column_number.
/// * node_types_column_number: usize - The rank of the column with the nodes types. This parameter is mutually exclusive with node_types_column.
/// * node_types_separator: String - Separator to split the node types.
pub struct NodeFileWriter {
    pub(crate) writer: CSVFileWriter,
    pub(crate) nodes_column: String,
    pub(crate) node_types_column: String,
    pub(crate) nodes_column_number: usize,
    pub(crate) node_types_column_number: usize,
    pub(crate) node_types_separator: String,
}

impl NodeFileWriter {
    /// Return new NodeFileWriter object.
    ///
    /// # Arguments
    ///
    /// * path: String - Path where to store/load the file.
    ///
    pub fn new(path: String) -> NodeFileWriter {
        NodeFileWriter {
            writer: CSVFileWriter::new(path),
            nodes_column: "id".to_string(),
            nodes_column_number: 0,
            node_types_column: "category".to_string(),
            node_types_column_number: 1,
            node_types_separator: "".to_string(),
        }
    }

    /// Set the column of the nodes.
    ///
    /// # Arguments
    ///
    /// * nodes_column: Option<String> - The nodes column to use for the file.
    ///
    pub fn set_nodes_column(mut self, nodes_column: Option<String>) -> NodeFileWriter {
        if let Some(column) = nodes_column {
            self.nodes_column = column;
        }
        self
    }

    /// Set the column of the nodes.
    ///
    /// # Arguments
    ///
    /// * node_types_column: Option<String> - The node types column to use for the file.
    ///
    pub fn set_node_types_column(mut self, nodes_type_column: Option<String>) -> NodeFileWriter {
        if let Some(column) = nodes_type_column {
            self.node_types_column = column;
        }
        self
    }

    /// Set the column_number of the nodes.
    ///
    /// # Arguments
    ///
    /// * nodes_column_number: Option<usize> - The nodes column_number to use for the file.
    ///
    pub fn set_nodes_column_number(mut self, nodes_column_number: Option<usize>) -> NodeFileWriter {
        if let Some(column) = nodes_column_number {
            self.nodes_column_number = column;
        }
        self
    }

    /// Set the column_number of the nodes.
    ///
    /// # Arguments
    ///
    /// * node_types_column_number: Option<usize> - The node types column_number to use for the file.
    ///
    pub fn set_node_types_column_number(
        mut self,
        node_types_column_number: Option<usize>,
    ) -> NodeFileWriter {
        if let Some(v) = node_types_column_number {
            self.node_types_column_number = v;
        }
        self
    }

    /// Set the verbose.
    ///
    /// # Arguments
    ///
    /// * verbose: Option<bool> - Wethever to show the loading bar or not.
    ///
    pub fn set_verbose(mut self, verbose: Option<bool>) -> NodeFileWriter {
        if let Some(v) = verbose {
            self.writer.verbose = v;
        }
        self
    }

    /// Set the separator.
    ///
    /// # Arguments
    ///
    /// * separator: Option<String> - The separator to use for the file.
    ///
    pub fn set_separator(mut self, separator: Option<String>) -> NodeFileWriter {
        if let Some(v) = separator {
            self.writer.separator = v;
        }
        self
    }

    /// Set the header.
    ///
    /// # Arguments
    ///
    /// * header: Option<bool> - Wethever to write out an header or not.
    ///
    pub fn set_header(mut self, header: Option<bool>) -> NodeFileWriter {
        if let Some(v) = header {
            self.writer.header = v;
        }
        self
    }

    /// Write nodes to file.
    ///
    /// # Arguments
    ///
    /// * `graph`: &Graph, reference to graph to use.
    pub fn dump(&self, graph: &Graph) -> Result<(), String> {
        // build the header
        let mut header = vec![(self.nodes_column.clone(), self.nodes_column_number)];

        if graph.has_node_types() {
            header.push((
                self.node_types_column.clone(),
                self.node_types_column_number,
            ));
        }

        let number_of_columns = 1 + header.iter().map(|(_, i)| i).max().unwrap();

        self.writer.write_lines(
            graph.get_nodes_number() as usize,
            compose_lines(number_of_columns, header),
            (0..graph.get_nodes_number()).map(|node_id| {
                let mut line = vec![(
                    graph.nodes.translate(node_id).to_string(),
                    self.nodes_column_number,
                )];

                if graph.has_node_types() {
                    line.push((
                        match graph.get_node_type_string(node_id) {
                            Some(values) => values.join(&self.node_types_separator),
                            None => "".to_string(),
                        },
                        self.node_types_column_number,
                    ));
                }
                compose_lines(number_of_columns, line)
            }),
        )
    }
}
