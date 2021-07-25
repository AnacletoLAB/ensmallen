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
    pub(crate) node_ids_column: Option<String>,
    pub(crate) node_ids_column_number: Option<usize>,
    pub(crate) nodes_column: String,
    pub(crate) node_types_column: Option<String>,
    pub(crate) nodes_column_number: usize,
    pub(crate) node_types_column_number: Option<usize>,
    pub(crate) node_types_separator: Option<String>,
    number_of_columns: usize,
    columns_are_dense: bool,
}

impl NodeFileWriter {
    /// Return new NodeFileWriter object.
    ///
    /// # Arguments
    ///
    /// * path: String - Path where to store/load the file.
    ///
    pub fn new<S: Into<String>>(path: S) -> NodeFileWriter {
        NodeFileWriter {
            writer: CSVFileWriter::new(path),
            node_ids_column: None,
            node_ids_column_number: None,
            nodes_column: "node_names".to_string(),
            nodes_column_number: 0,
            node_types_column: None,
            node_types_column_number: None,
            node_types_separator: None,
            number_of_columns: 1,
            columns_are_dense: true,
        }
    }

    // Return whether the columns are currently dense.
    fn are_columns_dense(&self) -> bool {
        let mut offset = 0;
        if self
            .node_ids_column_number
            .map_or(false, |node_ids_column_number| node_ids_column_number != 0)
        {
            return false;
        }
        if self.node_ids_column_number.is_some() {
            offset += 1;
        }
        self.nodes_column_number == offset
            && self
                .node_types_column_number
                .as_ref()
                .map_or(true, |&ntcn| ntcn == offset + 1)
    }

    /// Set the column of the nodes.
    ///
    /// # Arguments
    ///
    /// * nodes_column: Option<String> - The nodes column to use for the file.
    ///
    pub fn set_nodes_column<S: Into<String>>(mut self, nodes_column: Option<S>) -> NodeFileWriter {
        if let Some(column) = nodes_column {
            self.nodes_column = column.into();
        }
        self
    }

    /// Set the column of the nodes.
    ///
    /// # Arguments
    ///
    /// * node_types_column: Option<String> - The node types column to use for the file.
    ///
    pub fn set_node_types_column<S: Into<String>>(
        mut self,
        nodes_type_column: Option<S>,
    ) -> NodeFileWriter {
        if let Some(column) = nodes_type_column {
            self.node_types_column = Some(column.into());
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
        if let Some(column_number) = nodes_column_number {
            self.nodes_column_number = column_number;
            self.number_of_columns = self.number_of_columns.max(column_number + 1);
            self.columns_are_dense = self.are_columns_dense();
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
        if let Some(column_number) = node_types_column_number {
            self.node_types_column_number = Some(column_number);
            self.number_of_columns = self.number_of_columns.max(column_number + 1);
            self.columns_are_dense = self.are_columns_dense();
        }
        self
    }

    /// Set the column of the node IDs.
    ///
    /// # Arguments
    /// * node_ids_column: Option<String> - The node IDs column to use for the file.
    ///
    pub fn set_node_ids_column(mut self, node_ids_column: Option<String>) -> NodeFileWriter {
        self.node_ids_column = node_ids_column;
        self
    }

    /// Set the column number of the node IDs.
    ///
    /// # Arguments
    /// * node_ids_column_number: Option<usize> - The node types column to use for the file.
    ///
    pub fn set_node_ids_column_number(
        mut self,
        node_ids_column_number: Option<usize>,
    ) -> NodeFileWriter {
        if let Some(column_number) = node_ids_column_number {
            self.node_ids_column_number = Some(column_number);
            self.number_of_columns = self.number_of_columns.max(column_number + 1);
            self.columns_are_dense = self.are_columns_dense();
        }
        self
    }

    /// Set the verbose.
    ///
    /// # Arguments
    ///
    /// * `verbose`: Option<bool> - Whether to show the loading bar or not.
    ///
    pub fn set_verbose(mut self, verbose: Option<bool>) -> NodeFileWriter {
        self.writer = self.writer.set_verbose(verbose);
        self
    }

    /// Set the separator.
    ///
    /// # Arguments
    /// * separator: Option<String> - The separator to use for the file.
    ///
    pub fn set_separator(mut self, separator: Option<String>) -> Result<NodeFileWriter> {
        self.writer = self.writer.set_separator(separator)?;
        Ok(self)
    }

    /// Set the header.
    ///
    /// # Arguments
    ///
    /// * header: Option<bool> - Whether to write out an header or not.
    ///
    pub fn set_header(mut self, header: Option<bool>) -> NodeFileWriter {
        self.writer = self.writer.set_header(header);
        self
    }

    fn build_header(&self) -> (Vec<String>, Vec<usize>) {
        // build the header
        let mut header_values = vec![];
        let mut header_positions = vec![];

        if let (Some(node_ids_column), Some(node_ids_column_number)) =
            (&self.node_ids_column, self.node_ids_column_number)
        {
            header_values.push(node_ids_column.clone());
            header_positions.push(node_ids_column_number);
        }

        header_positions.push(self.nodes_column_number.clone());
        header_values.push(self.nodes_column.clone());

        if let (Some(node_types_column), Some(node_types_column_number)) =
            (&self.node_types_column, self.node_types_column_number)
        {
            header_values.push(node_types_column.clone());
            header_positions.push(node_types_column_number);
        }

        (header_values, header_positions)
    }

    /// Write nodes to file.
    ///
    /// # Arguments
    ///
    /// * `graph`: &Graph, reference to graph to use.
    pub fn dump(&self, graph: &Graph) -> Result<()> {
        // build the header
        let (header_values, header_positions) = self.build_header();
        // If the graph has multiple node labels we need a separator to join them.
        if self.node_types_separator.is_none()
            && graph.has_node_types()
            && graph.has_multilabel_node_types().unwrap()
        {
            return Err(concat!(
                "The current graph instance has multilabel node types ",
                "but no node type separator was provided!"
            )
            .to_string());
        }
        self.writer.write_lines(
            Some(graph.get_nodes_number() as usize),
            compose_lines(self.number_of_columns, header_values, header_positions),
            graph.iter_node_names_and_node_type_names().map(
                |(node_id, node_name, _, node_type_names)| {
                    let mut line = vec![node_name];
                    let mut positions = vec![];

                    if let Some(node_ids_column_number) = &self.node_ids_column_number {
                        line.push(node_id.to_string());
                        if !self.columns_are_dense {
                            positions.push(*node_ids_column_number);
                        }
                    }

                    if !self.columns_are_dense {
                        positions.push(self.nodes_column_number);
                    }

                    if let Some(column_number) = &self.node_types_column_number {
                        line.push(match (node_type_names, &self.node_types_separator) {
                            (None, _) => "".to_string(),
                            (Some(ntns), Some(sep)) => ntns.join(sep),
                            (Some(mut ntns), None) => ntns.pop().unwrap(),
                        });
                        if !self.columns_are_dense {
                            positions.push(*column_number);
                        }
                    }
                    if self.columns_are_dense {
                        line
                    } else {
                        compose_lines(self.number_of_columns, line, positions)
                    }
                },
            ),
        )
    }
}
