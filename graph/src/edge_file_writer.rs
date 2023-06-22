use super::*;
use indicatif::ProgressIterator;
use std::{fs::File, io::BufWriter};

/// Structure that saves the reader specific to writing and reading a nodes csv file.
///
/// # Attributes
#[no_binding]
pub struct EdgeFileWriter {
    pub(crate) writer: CSVFileWriter,
    pub(crate) edge_ids_column: Option<String>,
    pub(crate) edge_ids_column_number: Option<usize>,
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
    columns_are_dense: bool,
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
            edge_ids_column: None,
            edge_ids_column_number: None,
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
            // Note that this is not the
            // dense number of columns!
            number_of_columns: 2,
            columns_are_dense: true,
        }
    }

    // Return whether the columns are currently dense.
    fn are_columns_dense(&self) -> bool {
        let mut offset = 0;
        if self
            .edge_ids_column_number
            .map_or(false, |edge_ids_column_number| edge_ids_column_number != 0)
        {
            return false;
        }
        if self.edge_ids_column_number.is_some() {
            offset += 1;
        }
        if self.sources_column_number != offset || self.destinations_column_number != 1 + offset {
            return false;
        }
        if self
            .edge_types_column_number
            .map_or(false, |edge_types_column_number| {
                edge_types_column_number != 2 + offset
            })
        {
            return false;
        }
        if self.edge_types_column_number.is_some() {
            offset += 1;
        }
        if self
            .weights_column_number
            .map_or(false, |weights_column_number| {
                weights_column_number != 2 + offset
            })
        {
            return false;
        }
        return true;
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
            self.columns_are_dense = self.are_columns_dense();
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
            self.columns_are_dense = self.are_columns_dense();
        }
        self
    }

    /// Set the column of the edge types.
    ///
    /// # Arguments
    /// * edge_types_column: Option<String> - The edge types column to use for the file.
    ///
    pub fn set_edge_types_column(mut self, edge_types_column: Option<String>) -> EdgeFileWriter {
        self.edge_types_column = edge_types_column;
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
            self.columns_are_dense = self.are_columns_dense();
        }
        self
    }

    /// Set the column of the edge IDs.
    ///
    /// # Arguments
    /// * edge_ids_column: Option<String> - The edge IDs column to use for the file.
    ///
    pub fn set_edge_ids_column(mut self, edge_ids_column: Option<String>) -> EdgeFileWriter {
        self.edge_ids_column = edge_ids_column;
        self
    }

    /// Set the column number of the edge IDs.
    ///
    /// # Arguments
    /// * edge_ids_column_number: Option<usize> - The node types column to use for the file.
    ///
    pub fn set_edge_ids_column_number(
        mut self,
        edge_ids_column_number: Option<usize>,
    ) -> EdgeFileWriter {
        if let Some(column_number) = edge_ids_column_number {
            self.edge_ids_column_number = Some(column_number);
            self.number_of_columns = self.number_of_columns.max(column_number + 1);
            self.columns_are_dense = self.are_columns_dense();
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
            self.columns_are_dense = self.are_columns_dense();
        }
        self
    }

    /// Set the verbose.
    ///
    /// # Arguments
    /// * `verbose`: Option<bool> - Whether to show the loading bar or not.
    ///
    pub fn set_verbose(mut self, verbose: Option<bool>) -> EdgeFileWriter {
        self.writer = self.writer.set_verbose(verbose);
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
    pub fn set_numeric_edge_type_ids(
        mut self,
        numeric_edge_type_ids: Option<bool>,
    ) -> EdgeFileWriter {
        if let Some(nni) = numeric_edge_type_ids {
            self.numeric_edge_type_ids = nni;
        }
        self
    }

    /// Set the separator.
    ///
    /// # Arguments
    /// * separator: Option<char> - The separator to use for the file.
    ///
    pub fn set_separator(mut self, separator: Option<char>) -> Result<EdgeFileWriter> {
        self.writer = self.writer.set_separator(separator)?;
        Ok(self)
    }

    /// Set the header.
    ///
    /// # Arguments
    /// * header: Option<bool> - Whether to write out an header or not.
    ///
    pub fn set_header(mut self, header: Option<bool>) -> EdgeFileWriter {
        self.writer = self.writer.set_header(header);
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
        edge_id: EdgeT,
        src: NodeT,
        src_name: String,
        dst: NodeT,
        dst_name: String,
        edge_type: Option<EdgeTypeT>,
        edge_type_name: Option<String>,
        weight: Option<WeightT>,
    ) -> Vec<String> {
        let mut line = vec![];

        let mut positions = vec![];

        if let Some(edge_ids_column_number) = &self.edge_ids_column_number {
            line.push(edge_id.to_string());
            if !self.columns_are_dense {
                positions.push(*edge_ids_column_number);
            }
        }

        if self.numeric_node_ids {
            line.push(src.to_string());
            line.push(dst.to_string());
        } else {
            line.push(src_name);
            line.push(dst_name);
        };

        if !self.columns_are_dense {
            positions.push(self.sources_column_number);
            positions.push(self.destinations_column_number);
        }

        if let Some(column_number) = &self.edge_types_column_number {
            line.push(
                match self.numeric_edge_type_ids {
                    true => edge_type.map(|edge_type| edge_type.to_string()),
                    false => edge_type_name,
                }
                .unwrap_or("".to_string()),
            );
            if !self.columns_are_dense {
                positions.push(*column_number);
            }
        }

        if let Some(column_number) = &self.weights_column_number {
            line.push(weight.map_or("".to_string(), |w| w.to_string()));
            if self.columns_are_dense {
                positions.push(*column_number);
            }
        }

        if self.columns_are_dense {
            line
        } else {
            compose_lines(self.number_of_columns, line, positions)
        }
    }

    fn build_header(&self) -> (Vec<String>, Vec<usize>) {
        // build the header
        let mut header_values = vec![];
        let mut header_positions = vec![];

        if let (Some(edge_ids_column), Some(edge_ids_column_number)) =
            (&self.edge_ids_column, self.edge_ids_column_number)
        {
            header_values.push(edge_ids_column.clone());
            header_positions.push(edge_ids_column_number);
        }

        header_positions.push(self.sources_column_number.clone());
        header_positions.push(self.destinations_column_number.clone());

        header_values.push(self.sources_column.clone());
        header_values.push(self.destinations_column.clone());

        if let (Some(edge_types_column), Some(edge_types_column_number)) =
            (&self.edge_types_column, self.edge_types_column_number)
        {
            header_values.push(edge_types_column.clone());
            header_positions.push(edge_types_column_number);
        }

        if let (Some(weights_column), Some(weights_column_number)) =
            (&self.weights_column, self.weights_column_number)
        {
            header_values.push(weights_column.clone());
            header_positions.push(weights_column_number);
        }

        (header_values, header_positions)
    }

    pub fn start_writer(&self) -> Result<BufWriter<File>> {
        let (header_values, header_positions) = self.build_header();
        self.writer.start_writer(compose_lines(
            self.number_of_columns,
            header_values,
            header_positions,
        ))
    }

    /// Write the provided set of line elements to file.
    ///
    /// # Arguments
    /// `stream`: BufWriter<File> - The stream where to write the line
    ///
    /// # Raises
    /// * If some I/O error is encountered.
    pub fn write_line(
        &self,
        stream: &mut BufWriter<File>,
        edge_id: EdgeT,
        src: NodeT,
        src_name: String,
        dst: NodeT,
        dst_name: String,
        edge_type: Option<EdgeTypeT>,
        edge_type_name: Option<String>,
        weight: Option<WeightT>,
    ) -> Result<()> {
        self.writer.write_line(
            stream,
            self.parse_line(
                edge_id,
                src,
                src_name,
                dst,
                dst_name,
                edge_type,
                edge_type_name,
                weight,
            ),
        )
    }

    pub fn close_writer(&self, stream: BufWriter<File>) -> Result<()> {
        self.writer.close_writer(stream)
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
        let pb = get_loading_bar(
            self.writer.verbose && lines_number.is_some(),
            "Writing to edge list",
            lines_number.unwrap_or(0),
        );

        if self.edge_types_column.is_some() && self.edge_types_column_number.is_none()
        {
            return Err(format!(
                concat!(
                    "The edge types column number was not provided but ",
                    "the edge types column name was provided as {:?}.",
                ),
                self.edge_types_column
            ));
        }
        if self.weights_column.is_some() && self.weights_column_number.is_none() {
            return Err(format!(
                concat!(
                    "The weights column number was not provided but ",
                    "the weights column name was provided as {:?}.",
                ),
                self.weights_column
            ));
        }

        let mut stream = self.start_writer()?;
        for (edge_id, src, src_name, dst, dst_name, edge_type, edge_type_name, weight) in
            iterator.progress_with(pb)
        {
            self.write_line(
                &mut stream,
                edge_id,
                src,
                src_name,
                dst,
                dst_name,
                edge_type,
                edge_type_name,
                weight,
            )?;
        }
        self.close_writer(stream)
    }

    /// Write edge file from graph.
    ///  
    /// # Arguments
    /// * `graph`: &Graph - the graph to write out.
    pub fn dump_graph(self, graph: &Graph) -> Result<()> {
        let directed: bool = self.directed.unwrap_or_else(|| graph.is_directed());
        self.dump_iterator(
            Some(graph.get_number_of_directed_edges() as usize),
            graph.iter_edge_node_names_and_edge_type_name_and_edge_weight(directed),
        )
    }
}
