use super::*;

/// Structure that saves the reader specific to writing and reading a nodes csv file.
///
/// # Attributes
pub struct EdgeFileWriter {
    pub(crate) writer: CSVFileWriter,
    pub(crate) sources_column: String,
    pub(crate) sources_column_number: usize,
    pub(crate) destinations_column: String,
    pub(crate) destinations_column_number: usize,
    pub(crate) edge_types_column: Option<String>,
    pub(crate) edge_types_column_number: Option<usize>,
    pub(crate) weights_column: Option<String>,
    pub(crate) weights_column_number: Option<usize>,
    pub(crate) numeric_node_ids: bool,
    pub(crate) numeric_edge_type_ids: bool,
    pub(crate) directed: Option<bool>,
    number_of_columns: usize,
}

impl EdgeFileWriter {
    /// Return new EdgeFileWriter object.
    ///
    /// # Arguments
    /// * path: String - Path where to store/load the file.
    ///
    pub fn new<S: Into<String>>(path: S) -> EdgeFileWriter {
        EdgeFileWriter {
            writer: CSVFileWriter::new(path),
            sources_column: "subject".to_string(),
            sources_column_number: 0,
            destinations_column: "object".to_string(),
            destinations_column_number: 1,
            edge_types_column: None,
            edge_types_column_number: None,
            weights_column: None,
            weights_column_number: None,
            numeric_node_ids: false,
            numeric_edge_type_ids: false,
            directed: None,
            number_of_columns: 2,
        }
    }

    /// Set the column of the source nodes.
    ///
    /// # Arguments
    /// * sources_column: Option<String> - The source nodes column to use for the file.
    ///
    pub fn set_sources_column<S: Into<String>>(
        mut self,
        sources_column: Option<S>,
    ) -> EdgeFileWriter {
        if let Some(column) = sources_column {
            self.sources_column = column.into();
        }
        self
    }

    /// Set the column of the source nodes.
    ///
    /// # Arguments
    /// * sources_column_number: Option<String> - The source nodes column to use for the file.
    ///
    pub fn set_sources_column_number(
        mut self,
        sources_column_number: Option<usize>,
    ) -> EdgeFileWriter {
        if let Some(column_number) = sources_column_number {
            self.sources_column_number = column_number;
            self.number_of_columns = self.number_of_columns.max(column_number + 1);
        }
        self
    }

    /// Set the column of the nodes.
    ///
    /// # Arguments
    /// * destinations_column: Option<String> - The node types column to use for the file.
    ///
    pub fn set_destinations_column<S: Into<String>>(
        mut self,
        destinations_column: Option<S>,
    ) -> EdgeFileWriter {
        if let Some(column) = destinations_column {
            self.destinations_column = column.into();
        }
        self
    }

    /// Set the column of the nodes.
    ///
    /// # Arguments
    /// * destinations_column_number: Option<String> - The node types column to use for the file.
    ///
    pub fn set_destinations_column_number(
        mut self,
        destinations_column_number: Option<usize>,
    ) -> EdgeFileWriter {
        if let Some(column_number) = destinations_column_number {
            self.destinations_column_number = column_number;
            self.number_of_columns = self.number_of_columns.max(column_number + 1);
        }
        self
    }

    /// Set the column of the edge types.
    ///
    /// # Arguments
    /// * edge_types_column: Option<String> - The edge types column to use for the file.
    ///
    pub fn set_edge_types_column(mut self, edge_type_column: Option<String>) -> EdgeFileWriter {
        self.edge_types_column = edge_type_column;
        self
    }

    /// Set the column number of the edge types.
    ///
    /// # Arguments
    /// * edge_types_column_number: Option<usize> - The node types column to use for the file.
    ///
    pub fn set_edge_types_column_number(
        mut self,
        edge_types_column_number: Option<usize>,
    ) -> EdgeFileWriter {
        if let Some(column_number) = edge_types_column_number {
            self.edge_types_column_number = Some(column_number);
            self.number_of_columns = self.number_of_columns.max(column_number + 1);
        }
        self
    }

    /// Set the column of the weights.
    ///
    /// # Arguments
    /// * weights_column: Option<String> - The weight column to use for the file.
    ///
    pub fn set_weights_column(mut self, weights_column: Option<String>) -> EdgeFileWriter {
        self.weights_column = weights_column;
        self
    }

    /// Set the column number of the weights.
    ///
    /// # Arguments
    /// * weights_column_number: Option<usize> - The weight column to use for the file.
    ///
    pub fn set_weights_column_number(
        mut self,
        weights_column_number: Option<usize>,
    ) -> EdgeFileWriter {
        if let Some(column_number) = weights_column_number {
            self.weights_column_number = Some(column_number);
            self.number_of_columns = self.number_of_columns.max(column_number + 1);
        }
        self
    }

    /// Set the verbose.
    ///
    /// # Arguments
    /// * `verbose`: Option<bool> - Whether to show the loading bar or not.
    ///
    pub fn set_verbose(mut self, verbose: Option<bool>) -> EdgeFileWriter {
        if let Some(v) = verbose {
            self.writer.verbose = v;
        }
        self
    }

    /// Set whether the node IDs are to be treated as numeric.
    ///
    /// # Arguments
    /// * numeric_node_ids: Option<bool> - Whether the node IDs are to be treated as numeric.
    ///
    pub fn set_numeric_node_ids(mut self, numeric_node_ids: Option<bool>) -> EdgeFileWriter {
        if let Some(nni) = numeric_node_ids {
            self.numeric_node_ids = nni;
        }
        self
    }

    /// Set whether the edge type IDs are to be treated as numeric.
    ///
    /// # Arguments
    /// * numernumeric_edge_type_idsic_id: Option<bool> - Whether the edge type IDs are to be treated as numeric.
    ///
    pub fn set_numeric_edge_type_ids(mut self, numeric_edge_type_ids: Option<bool>) -> EdgeFileWriter {
        if let Some(nni) = numeric_edge_type_ids {
            self.numeric_edge_type_ids = nni;
        }
        self
    }

    /// Set the separator.
    ///
    /// # Arguments
    /// * separator: Option<String> - The separator to use for the file.
    ///
    pub fn set_separator<S: Into<String>>(mut self, separator: Option<S>) -> EdgeFileWriter {
        if let Some(v) = separator {
            self.writer.separator = v.into();
        }
        self
    }

    /// Set the header.
    ///
    /// # Arguments
    /// * header: Option<bool> - Whether to write out an header or not.
    ///
    pub fn set_header(mut self, header: Option<bool>) -> EdgeFileWriter {
        if let Some(v) = header {
            self.writer.header = v;
        }
        self
    }

    /// Set the directed.
    ///
    /// # Arguments
    /// * `directed`: Option<bool> - Whether to write out the graph as directed or not.
    ///
    pub fn set_directed(mut self, directed: Option<bool>) -> EdgeFileWriter {
        self.directed = directed;
        self
    }

    /// Parses provided line into a vector of strings writable by the CSVFileWriter.
    fn parse_line(
        &self,
        src: NodeT,
        src_name: String,
        dst: NodeT,
        dst_name: String,
        edge_type: Option<EdgeTypeT>,
        edge_type_name: Option<String>,
        weight: Option<WeightT>,
    ) -> Vec<String> {
        let mut line = vec![
            (
                match self.numeric_node_ids {
                    true => src.to_string(),
                    false => src_name,
                },
                self.sources_column_number,
            ),
            (
                match self.numeric_node_ids {
                    true => dst.to_string(),
                    false => dst_name,
                },
                self.destinations_column_number,
            ),
        ];

        if let Some(column_number) = &self.edge_types_column_number {
            line.push((
                if let (Some(edge_type), Some(edge_type_name)) = (edge_type, edge_type_name){
                    match self.numeric_edge_type_ids {
                        true => edge_type.to_string(),
                        false => edge_type_name,
                    }
                } else {
                    "".to_string()
                },
                *column_number,
            ));
        }

        if let Some(column_number) = &self.weights_column_number {
            line.push((
                weight.map_or("".to_string(), |w| w.to_string()),
                *column_number,
            ));
        }

        compose_lines(self.number_of_columns, line)
    }

    fn build_header(&self) -> Vec<(String, usize)> {
        // build the header
        let mut header = vec![
            (self.sources_column.clone(), self.sources_column_number),
            (
                self.destinations_column.clone(),
                self.destinations_column_number,
            ),
        ];

        if let (Some(edge_types_column), Some(edge_types_column_number)) =
            (&self.edge_types_column, self.edge_types_column_number)
        {
            header.push((edge_types_column.clone(), edge_types_column_number));
        }

        if let (Some(weights_column), Some(weights_column_number)) =
            (&self.weights_column, self.weights_column_number)
        {
            header.push((weights_column.clone(), weights_column_number));
        }

        header
    }

    /// Write edge list iterator to file.
    ///  
    /// # Arguments
    /// * `iterator`: impl Iterator<Item=_> - The iterator with the edge list to write to file.
    pub fn dump_iterator(
        &self,
        lines_number: Option<usize>,
        iterator: impl Iterator<
            Item = (
                EdgeT,
                NodeT,
                String,
                NodeT,
                String,
                Option<EdgeTypeT>,
                Option<String>,
                Option<WeightT>,
            ),
        >,
    ) -> Result<()> {
        self.writer.write_lines(
            lines_number,
            compose_lines(self.number_of_columns, self.build_header()),
            iterator.map(
                |(_, src, src_name, dst, dst_name, edge_type, edge_type_name, weight)| {
                    self.parse_line(
                        src,
                        src_name,
                        dst,
                        dst_name,
                        edge_type,
                        edge_type_name,
                        weight,
                    )
                },
            ),
        )
    }

    /// Write edge file from graph.
    ///  
    /// # Arguments
    /// * `graph`: &Graph - the graph to write out.
    pub fn dump_graph(&self, graph: &Graph) -> Result<()> {
        let directed: bool = self.directed.unwrap_or_else(|| graph.is_directed());
        self.dump_iterator(
            Some(graph.get_directed_edges_number() as usize),
            graph.iter_edge_node_names_and_edge_type_name_and_edge_weight(directed),
        )
    }
}
