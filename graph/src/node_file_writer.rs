use super::*;

/// Structure that saves the parameters specific to writing and reading a nodes csv file.
///
/// # Attributes
/// * parameters: CSVFile - The common parameters for readin and writing a csv.
/// * nodes_column: String - The name of the nodes names column. This parameter is mutually exclusive with nodes_column_number.
/// * nodes_column_number: usize - The rank of the column with the nodes names. This parameter is mutually exclusive with nodes_column.
/// * node_types_column: String - The name of the nodes type column. This parameter is mutually exclusive with node_types_column_number.
/// * node_types_column_number: usize - The rank of the column with the nodes types. This parameter is mutually exclusive with node_types_column.
pub struct NodeFileWriter {
    pub(crate) parameters: CSVFileWriter,
    pub(crate) nodes_column: String,
    pub(crate) node_types_column: String,
    pub(crate) nodes_column_number: usize,
    pub(crate) node_types_column_number: usize,
}

impl NodeFileWriter {
    /// Return new NodeFileWriter object.
    ///
    /// # Arguments
    ///
    /// * parameters: CSVFileParameters - Path where to store/load the file.
    ///
    pub fn new(parameters: CSVFileWriter) -> NodeFileWriter {
        NodeFileWriter {
            parameters,
            nodes_column: "id".to_string(),
            nodes_column_number: 0,
            node_types_column: "category".to_string(),
            node_types_column_number: 1,
        }
    }

    /// Set the column of the nodes.
    ///
    /// # Arguments
    ///
    /// * nodes_column: Option<String> - The nodes column to use for the file.
    ///
    pub fn set_nodes_column(
        mut self,
        nodes_column: Option<String>,
    ) -> Result<NodeFileWriter, String> {
        if let Some(v) = nodes_column {
            self.nodes_column = v;
        }
        Ok(self)
    }

    /// Set the column of the nodes.
    ///
    /// # Arguments
    ///
    /// * node_types_column: Option<String> - The node types column to use for the file.
    ///
    pub fn set_node_types_column(
        mut self,
        nodes_type_column: Option<String>,
    ) -> Result<NodeFileWriter, String> {
        if let Some(v) = nodes_type_column {
            self.node_types_column = v;
        }
        Ok(self)
    }

    /// Set the column_number of the nodes.
    ///
    /// # Arguments
    ///
    /// * nodes_column_number: Option<usize> - The nodes column_number to use for the file.
    ///
    pub fn set_nodes_column_number(mut self, nodes_column_number: Option<usize>) -> NodeFileWriter {
        if let Some(v) = nodes_column_number {
            self.nodes_column_number = v;
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

    /// Read node file and returns graph builder data structures.
    ///  
    pub(crate) fn write_node_file(
        &self,
        graph: &Graph,
    ) -> Result<(), String> {
        // build the header
        let mut header = vec![(self.nodes_column.clone(), self.nodes_column_number)];

        if graph.has_node_types() {
            header.push((
                self.node_types_column.clone(),
                self.node_types_column_number,
            ));
        }

        let number_of_columns = 1 + header.iter().map(|(_, i)| i).max().unwrap();

        self.parameters.write_lines(
            graph.get_nodes_number() as u64,
            compose_lines(number_of_columns, header),
            (0..graph.get_nodes_number()).map(|index| {
                let mut line = vec![(graph.nodes.translate(index).to_string(), self.nodes_column_number)];

                if let Some(nt) = &graph.node_types {
                    line.push((
                        nt.translate(nt.ids[index]).to_string(),
                        self.node_types_column_number,
                    ));
                }
                compose_lines(number_of_columns, line)
            }),
        )
    }
}
