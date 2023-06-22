use super::*;
use indicatif::ProgressIterator;
use itertools::Itertools;
use std::{fs::File, io::BufWriter};

/// Structure that saves the writer specific to writing and reading a nodes csv file.
///
/// # Attributes
/// * `writer`: CSVFileWriter - The common writer for readin and writing a csv.
/// * `node_ids_column`: Option<String> - The name of the node ids column.
/// * `node_ids_column_number`: Option<usize> - The rank of the column with the nodes ids.
/// * `nodes_column`: String - The name of the nodes names column.
/// * `nodes_column_number`: usize - The rank of the column with the nodes names.
/// * `node_types_column`: String - The name of the nodes type column.
/// * `node_types_column_number`: usize - The rank of the column with the nodes types.
/// * `node_types_separator`: Option<String> - Separator to split the node types.
/// * `node_descriptions_column`: Option<String> - The name of the nodes descriptions column.
/// * `node_descriptions_column_number`: Option<usize> - The rank of the column with the nodes descriptions.
/// * `numeric_node_type_ids`: bool - Whether the node type IDs should be written out as numeric.
/// * `number_of_columns`: usize - The number of columns to be written out.
/// * `columns_are_dense`: bool - Whether the columns can be assumed to be dense, that is without gaps in between. This information can be used for a faster writer implementation.
#[no_binding]
pub struct NodeFileWriter {
    pub(crate) writer: CSVFileWriter,
    pub(crate) node_ids_column: Option<String>,
    pub(crate) node_ids_column_number: Option<usize>,
    pub(crate) nodes_column: String,
    pub(crate) nodes_column_number: usize,
    pub(crate) node_types_column: Option<String>,
    pub(crate) node_types_column_number: Option<usize>,
    pub(crate) node_types_separator: Option<String>,
    pub(crate) node_descriptions_column: Option<String>,
    pub(crate) node_descriptions_column_number: Option<usize>,
    pub(crate) numeric_node_type_ids: bool,
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
            nodes_column: "node_name".to_string(),
            nodes_column_number: 0,
            node_types_column: None,
            node_types_column_number: None,
            node_types_separator: Some("|".to_string()),
            node_descriptions_column: None,
            node_descriptions_column_number: None,
            numeric_node_type_ids: false,
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
        if self.nodes_column_number != offset {
            return false;
        }
        offset += 1;

        if self
            .node_types_column_number
            .as_ref()
            .map_or(false, |&ntcn| ntcn != offset)
        {
            return false;
        }
        if self.node_types_column_number.is_some() {
            offset += 1;
        }
        if self
            .node_descriptions_column_number
            .as_ref()
            .map_or(false, |&ntcn| ntcn != offset)
        {
            return false;
        }
        true
    }

    /// Set the column of the nodes.
    ///
    /// # Arguments
    /// * `nodes_column`: Option<String> - The nodes column to use for the file.
    ///
    pub fn set_nodes_column<S: Into<String>>(mut self, nodes_column: Option<S>) -> NodeFileWriter {
        if let Some(column) = nodes_column {
            self.nodes_column = column.into();
        }
        self
    }

    /// Set the column of the node types.
    ///
    /// # Arguments
    /// * `node_types_column`: Option<String> - The node types column to use for the file.
    ///
    pub fn set_node_types_column<S: Into<String>>(
        mut self,
        node_type_column: Option<S>,
    ) -> NodeFileWriter {
        if let Some(column) = node_type_column {
            self.node_types_column = Some(column.into());
        }
        self
    }

    /// Set the column of the node descriptions.
    ///
    /// # Arguments
    /// * `node_descriptions_column`: Option<String> - The node descriptions column to use for the file.
    ///
    pub fn set_node_descriptions_column<S: Into<String>>(
        mut self,
        node_descriptions_column: Option<S>,
    ) -> NodeFileWriter {
        if let Some(column) = node_descriptions_column {
            self.node_descriptions_column = Some(column.into());
        }
        self
    }

    /// Set the column_number of the nodes.
    ///
    /// # Arguments
    /// * `nodes_column_number`: Option<usize> - The nodes column_number to use for the file.
    ///
    pub fn set_nodes_column_number(mut self, nodes_column_number: Option<usize>) -> NodeFileWriter {
        if let Some(column_number) = nodes_column_number {
            self.nodes_column_number = column_number;
            self.number_of_columns = self.number_of_columns.max(column_number + 1);
            self.columns_are_dense = self.are_columns_dense();
        }
        self
    }

    /// Set the column_number of the node types.
    ///
    /// # Arguments
    /// * `node_types_column_number`: Option<usize> - The node types column_number to use for the file.
    ///
    pub fn set_node_types_column_number(
        mut self,
        node_types_column_number: Option<usize>,
    ) -> NodeFileWriter {
        if let Some(column_number) = node_types_column_number {
            self.node_types_column_number = Some(column_number);
            self.number_of_columns = self.number_of_columns.max(column_number + 1);
            self.columns_are_dense = self.are_columns_dense();
            if self.node_types_column.is_none() {
                self.node_types_column = Some(column_number.to_string());
            }
        }
        self
    }

    /// Set the column_number of the node descriptions.
    ///
    /// # Arguments
    /// * `node_descriptions_column_number`: Option<usize> - The node descriptions column_number to use for the file.
    ///
    pub fn set_node_descriptions_column_number(
        mut self,
        node_descriptions_column_number: Option<usize>,
    ) -> NodeFileWriter {
        if let Some(column_number) = node_descriptions_column_number {
            self.node_descriptions_column_number = Some(column_number);
            self.number_of_columns = self.number_of_columns.max(column_number + 1);
            self.columns_are_dense = self.are_columns_dense();
            if self.node_descriptions_column.is_none() {
                self.node_descriptions_column = Some(column_number.to_string());
            }
        }
        self
    }

    /// Set the column of the node IDs.
    ///
    /// # Arguments
    /// * `node_ids_column`: Option<String> - The node IDs column to use for the file.
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
            if self.node_ids_column.is_none() {
                self.node_ids_column = Some(column_number.to_string());
            }
        }
        self
    }

    /// Set whether the node type IDs are to be treated as numeric.
    ///
    /// # Arguments
    /// * `numeric_node_type_ids`: Option<bool> - Whether the node type IDs are to be treated as numeric.
    ///
    pub fn set_numeric_node_type_ids(
        mut self,
        numeric_node_type_ids: Option<bool>,
    ) -> NodeFileWriter {
        if let Some(nni) = numeric_node_type_ids {
            self.numeric_node_type_ids = nni;
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
    /// * separator: Option<char> - The separator to use for the file.
    ///
    pub fn set_separator(mut self, separator: Option<char>) -> Result<NodeFileWriter> {
        self.writer = self.writer.set_separator(separator)?;
        Ok(self)
    }

    /// Set the node types separator.
    ///
    /// In the following example we show a column of node IDs and
    /// a column of node types.
    ///
    /// ```bash
    /// node_ids_columns node_types
    /// node_A node_type_1|node_type_2
    /// node_B node_type_2
    /// ```  
    ///
    /// # Arguments
    /// * node_types_separator: Option<String> - The separator to use for the node types column.
    ///
    pub fn set_node_types_separator<S: Into<String>>(
        mut self,
        node_types_separator: Option<S>,
    ) -> Result<NodeFileWriter> {
        if let Some(sep) = node_types_separator {
            let sep = sep.into();
            if sep.is_empty() {
                return Err("The node type separator cannot be empty.".to_owned());
            }
            self.node_types_separator = Some(sep);
        }
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

        if let (Some(node_descriptions_column), Some(node_descriptions_column_number)) = (
            &self.node_descriptions_column,
            self.node_descriptions_column_number,
        ) {
            header_values.push(node_descriptions_column.clone());
            header_positions.push(node_descriptions_column_number);
        }

        (header_values, header_positions)
    }

    pub fn parse_line(
        &self,
        node_id: NodeT,
        node_name: String,
        node_type_ids: Option<Vec<NodeTypeT>>,
        node_type_names: Option<Vec<String>>,
        node_description: Option<String>,
    ) -> Vec<String> {
        let mut line = vec![];
        let mut positions = vec![];

        if let Some(node_ids_column_number) = &self.node_ids_column_number {
            line.push(node_id.to_string());
            if !self.columns_are_dense {
                positions.push(*node_ids_column_number);
            }
        }

        line.push(node_name);

        if !self.columns_are_dense {
            positions.push(self.nodes_column_number);
        }

        if let Some(column_number) = &self.node_types_column_number {
            if self.numeric_node_type_ids {
                line.push(match (node_type_ids, &self.node_types_separator) {
                    (None, _) => "".to_string(),
                    (Some(ntns), Some(sep)) => ntns
                        .iter()
                        .map(|node_type_id| node_type_id.to_string())
                        .join(sep),
                    (Some(ntns), None) => ntns[0].to_string(),
                });
            } else {
                line.push(match (node_type_names, &self.node_types_separator) {
                    (None, _) => "".to_string(),
                    (Some(ntns), Some(sep)) => ntns.join(sep),
                    (Some(mut ntns), None) => ntns.pop().unwrap(),
                });
            }
            if !self.columns_are_dense {
                positions.push(*column_number);
            }
        }

        if let Some(node_descriptions_column_number) = &self.node_descriptions_column_number {
            line.push(node_description.unwrap_or("".to_string()));
            if !self.columns_are_dense {
                positions.push(*node_descriptions_column_number);
            }
        }

        if self.columns_are_dense {
            line
        } else {
            compose_lines(self.number_of_columns, line, positions)
        }
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
    /// `node_id`: NodeT - Node ID of the node.
    /// `node_name`: String - Name of the node.
    /// `node_type_ids`: Option<Vec<NodeTypeT>> - Numeric IDs of the node types of the node.
    /// `node_type_names`: Option<Vec<String>> - Names of the node types of the node.
    /// `node_description`: Option<String> - Description of the node.
    ///
    /// # Raises
    /// * If some I/O error is encountered.
    pub fn write_line(
        &self,
        stream: &mut BufWriter<File>,
        node_id: NodeT,
        node_name: String,
        node_type_ids: Option<Vec<NodeTypeT>>,
        node_type_names: Option<Vec<String>>,
        node_description: Option<String>,
    ) -> Result<()> {
        self.writer.write_line(
            stream,
            self.parse_line(
                node_id,
                node_name,
                node_type_ids,
                node_type_names,
                node_description,
            ),
        )
    }

    pub fn close_writer(&self, stream: BufWriter<File>) -> Result<()> {
        self.writer.close_writer(stream)
    }

    /// Write edge list iterator to file.
    ///  
    /// # Arguments
    /// * `lines_number`: Option<usize> - The number of lines in the file.
    /// * `iterator`: impl Iterator<Item=_> - The iterator with the edge list to write to file.
    pub fn dump_iterator(
        &self,
        lines_number: Option<usize>,
        iterator: impl Iterator<Item = (NodeT, String, Option<Vec<NodeTypeT>>, Option<Vec<String>>)>,
    ) -> Result<()> {
        let pb = get_loading_bar(
            self.writer.verbose && lines_number.is_some(),
            "Writing to node list",
            lines_number.unwrap_or(0),
        );
        if self.node_types_column.is_some() && self.node_types_column_number.is_none() {
            return Err(format!(
                concat!(
                    "The node types column number was not provided but ",
                    "the node types column name was provided as {:?}.",
                ),
                self.node_types_column
            ));
        }
        if self.node_descriptions_column.is_some() && self.node_descriptions_column_number.is_none()
        {
            return Err(format!(
                concat!(
                    "The node descriptions column number was not provided but ",
                    "the node descriptions column name was provided as {:?}.",
                ),
                self.node_descriptions_column
            ));
        }
        let mut stream = self.start_writer()?;
        for (node_id, node_name, node_type_ids, node_type_names) in iterator.progress_with(pb) {
            self.write_line(
                &mut stream,
                node_id,
                node_name,
                node_type_ids,
                node_type_names,
                None,
            )?;
        }
        self.close_writer(stream)
    }

    /// Write nodes to file.
    ///
    /// # Arguments
    ///
    /// * `graph`: &Graph, reference to graph to use.
    pub fn dump_graph(&self, graph: &Graph) -> Result<()> {
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
        self.dump_iterator(
            Some(graph.get_number_of_nodes() as usize),
            graph.iter_node_names_and_node_type_names().map(
                |(node_id, node_name, node_type_ids, node_type_names)| {
                    (
                        node_id,
                        node_name,
                        node_type_ids.map(|x| x.to_vec()),
                        node_type_names,
                    )
                },
            ),
        )
    }
}
