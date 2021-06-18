use super::*;

/// Structure that saves the reader specific to writing and reading a nodes csv file.
///
/// # Attributes
/// * reader: CSVFile - The common reader for reading and writing a csv.
/// * default_node_type: Option<String> - The node type to use if a node has node type or its node type is "".
/// * nodes_column_number: Option<usize> - The rank of the column with the nodes names. This parameter is mutually exclusive with nodes_column.
/// * node_types_separator: Option<String> - Separator to split the node types.
/// * node_types_column_number: Option<usize> - The rank of the column with the nodes types. This parameter is mutually exclusive with node_types_column.
/// * numeric_node_ids: bool - Whether to load the node IDs as numeric.
/// * numeric_node_type_ids: bool - Whether to load the node type IDs as numeric.
/// * skip_node_types_if_unavailable: bool - Whether to skip attempting to load the node types if column is unavailable.
///
#[derive(Clone)]
pub struct NodeFileReader {
    pub(crate) reader: CSVFileReader,
    pub(crate) default_node_type: Option<String>,
    pub(crate) nodes_column_number: Option<usize>,
    pub(crate) node_types_separator: Option<String>,
    pub(crate) node_types_column_number: Option<usize>,
    pub(crate) numeric_node_ids: bool,
    pub(crate) numeric_node_type_ids: bool,
    pub(crate) skip_node_types_if_unavailable: bool,
    pub(crate) might_contain_singletons: bool,
}

impl NodeFileReader {
    /// Return new NodeFileReader object.
    ///
    /// # Arguments
    ///
    /// * reader: CSVFileParameters - Path where to store/load the file.
    ///
    pub fn new<S: Into<String>>(path: S) -> Result<NodeFileReader, String> {
        Ok(NodeFileReader {
            reader: CSVFileReader::new(path, "node list".to_owned())?,
            default_node_type: None,
            nodes_column_number: None,
            node_types_separator: None,
            node_types_column_number: None,
            numeric_node_ids: false,
            numeric_node_type_ids: false,
            skip_node_types_if_unavailable: false,
            might_contain_singletons: true,
        })
    }

    /// Set the column of the nodes.
    ///
    /// # Arguments
    ///
    /// * nodes_column: Option<String> - The nodes column to use for the file.
    ///
    pub fn set_nodes_column<S: Into<String>>(
        mut self,
        nodes_column: Option<S>,
    ) -> Result<NodeFileReader, String> {
        if let Some(column) = nodes_column {
            let column = column.into();
            if column.is_empty() {
                return Err("The given node column is empty.".to_owned());
            }
            self.nodes_column_number = Some(self.reader.get_column_number(column)?);
        }
        Ok(self)
    }

    /// Set the column_number of the nodes.
    ///
    /// # Arguments
    ///
    /// * nodes_column_number: Option<usize> - The nodes column_number to use for the file.
    ///t
    pub fn set_nodes_column_number(mut self, nodes_column_number: Option<usize>) -> NodeFileReader {
        self.nodes_column_number = nodes_column_number;
        self
    }

    /// Set the name of the graph to be loaded.
    ///
    /// # Arguments
    ///
    /// * graph_name: String - The name of the graph to be loaded.
    ///
    pub(crate) fn set_graph_name(mut self, graph_name: String) -> NodeFileReader {
        self.reader.graph_name = graph_name;
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
    ) -> Result<NodeFileReader, String> {
        if let Some(column) = nodes_type_column {
            let column = column.into();
            if column.is_empty() {
                return Err("The given node types column is empty.".to_owned());
            }
            match self.reader.get_column_number(column) {
                Ok(ecn) => {
                    self.node_types_column_number = Some(ecn);
                }
                Err(e) => {
                    if !self.skip_node_types_if_unavailable {
                        return Err(e);
                    }
                }
            }
        }
        Ok(self)
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
    ) -> NodeFileReader {
        self.node_types_column_number = node_types_column_number;
        self
    }

    /// Set whether to automatically skip node_types if they are not avaitable instead of raising an exception.
    ///
    /// # Arguments
    ///
    /// * skip_node_types_if_unavailable: Option<bool> - Whether to skip node_types if they are not available.
    ///
    pub fn set_skip_node_types_if_unavailable(
        mut self,
        skip_node_types_if_unavailable: Option<bool>,
    ) -> Result<NodeFileReader, String> {
        if let Some(skip) = skip_node_types_if_unavailable {
            self.skip_node_types_if_unavailable = skip;
        }
        Ok(self)
    }

    /// Set whether you pinky promise that this graph has singletons or not.
    ///
    /// # Arguments
    ///
    /// * `might_contain_singletons`: Option<bool> - Whether this graph has singletons.
    ///
    pub fn set_might_contain_singleton_nodes(
        mut self,
        might_contain_singletons: Option<bool>,
    ) -> Result<NodeFileReader, String> {
        if let Some(skip) = might_contain_singletons {
            self.might_contain_singletons = skip;
        }
        Ok(self)
    }

    /// Set the comment symbol to use to skip the lines.
    ///
    /// # Arguments
    ///
    /// * comment_symbol: Option<String> - if the reader should ignore or not duplicated edges.
    ///
    pub fn set_comment_symbol(
        mut self,
        comment_symbol: Option<String>,
    ) -> Result<NodeFileReader, String> {
        if let Some(cs) = comment_symbol {
            if cs.is_empty() {
                return Err("The given comment symbol is empty.".to_string());
            }
            self.reader.comment_symbol = Some(cs);
        }
        Ok(self)
    }

    /// Set whether the CSV is expected to be well written.
    ///
    /// # Arguments
    ///
    /// * csv_is_correct: Option<bool> - Whether you pinky swear the node list is correct.
    ///
    pub fn set_csv_is_correct(mut self, csv_is_correct: Option<bool>) -> NodeFileReader {
        if let Some(cic) = csv_is_correct {
            self.reader.csv_is_correct = cic;
        }
        self
    }

    /// Set the default node type.
    ///
    /// # Arguments
    ///
    /// * default_node_type: Option<String> - The node type to use when node type is missing.
    ///
    pub fn set_default_node_type<S: Into<String>>(
        mut self,
        default_node_type: Option<S>,
    ) -> NodeFileReader {
        self.default_node_type = default_node_type.map(|val| val.into());
        self
    }

    /// Set the verbose.
    ///
    /// # Arguments
    ///
    /// * `verbose`: Option<bool> - Whether to show the loading bar or not.
    ///
    pub fn set_verbose(mut self, verbose: Option<bool>) -> NodeFileReader {
        if let Some(v) = verbose {
            self.reader.verbose = v;
        }
        self
    }

    /// Set the numeric_id.
    ///
    /// # Arguments
    ///
    /// * numeric_node_type_ids: Option<bool> - Whether to convert numeric node type Ids to Node Type Ids.
    ///
    pub fn set_numeric_node_type_ids(
        mut self,
        numeric_node_type_ids: Option<bool>,
    ) -> NodeFileReader {
        if let Some(nnti) = numeric_node_type_ids {
            self.numeric_node_type_ids = nnti;
        }
        self
    }

    /// Set the numeric_id.
    ///
    /// # Arguments
    ///
    /// * numeric_node_ids: Option<bool> - Whether to convert numeric node type Ids to Node Type Ids.
    ///
    pub fn set_numeric_node_ids(mut self, numeric_node_ids: Option<bool>) -> NodeFileReader {
        if let Some(nni) = numeric_node_ids {
            self.numeric_node_ids = nni;
        }
        self
    }

    /// Set the ignore_duplicates.
    ///
    /// # Arguments
    ///
    /// * ignore_duplicates: Option<bool> - Whether to ignore detected duplicates or raise exception.
    ///
    pub fn set_ignore_duplicates(mut self, ignore_duplicates: Option<bool>) -> NodeFileReader {
        if let Some(v) = ignore_duplicates {
            self.reader.ignore_duplicates = v;
        }
        self
    }

    /// Set the separator.
    ///
    /// # Arguments
    ///
    /// * separator: Option<String> - The separator to use for the file.
    ///
    pub fn set_separator<S: Into<String>>(
        mut self,
        separator: Option<S>,
    ) -> Result<NodeFileReader, String> {
        if let Some(sep) = separator {
            let sep = sep.into();
            if sep.is_empty() {
                return Err("The separator cannot be empty.".to_owned());
            }
            self.reader.separator = sep;
        }
        Ok(self)
    }

    /// Set the node types separator.
    ///
    /// In the following example we show a column of node IDs and
    /// a column of node types.
    ///
    /// ```bash
    /// node_id_columns node_types
    /// node_A node_type_1|node_type_2
    /// node_B node_type_2
    /// ```  
    ///
    /// # Arguments
    ///
    /// * node_types_separator: Option<String> - The separator to use for the node types column.
    ///
    pub fn set_node_types_separator<S: Into<String>>(
        mut self,
        node_types_separator: Option<S>,
    ) -> Result<NodeFileReader, String> {
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
    /// * header: Option<bool> - Whether to expect an header or not.
    ///
    pub fn set_header(mut self, header: Option<bool>) -> NodeFileReader {
        if let Some(v) = header {
            self.reader.header = v;
        }
        self
    }

    /// Set number of rows to be skipped when starting to read file.
    ///
    /// # Arguments
    ///
    /// * rows_to_skip: Option<bool> - Whether to show the loading bar or not.
    ///
    pub fn set_rows_to_skip(mut self, rows_to_skip: Option<usize>) -> NodeFileReader {
        if let Some(v) = rows_to_skip {
            self.reader.rows_to_skip = v;
        }
        self
    }

    /// Set the maximum number of rows to load from the file
    ///
    /// # Arguments
    ///
    /// * max_rows_number: Option<u64> - The edge type to use when edge type is missing.
    ///
    pub fn set_max_rows_number(mut self, max_rows_number: Option<u64>) -> NodeFileReader {
        self.reader.max_rows_number = max_rows_number;
        self
    }

    /// Return boolean representing if the node types exist.
    pub fn has_node_types(&self) -> bool {
        self.default_node_type.is_some() || self.node_types_column_number.is_some()
    }

    /// Return iterator of the lines of the node file.
    pub fn read_lines(
        &self,
    ) -> Result<impl Iterator<Item = Result<(String, Option<Vec<String>>), String>> + '_, String>
    {
        // Validating that at least a column was given.
        if [self.nodes_column_number, self.node_types_column_number]
            .iter()
            .all(|val| val.is_none())
        {
            return Err("Neither nodes ID column or node types column were given!".to_string());
        }

        // Check that the two columns do not have the same value.
        if self.nodes_column_number == self.node_types_column_number {
            return Err("The node column is the same as the node type one.".to_string());
        }

        // Retrieve the expected maximum number of columns.
        let expected_number_of_elements = self.reader.get_elements_per_line()?;

        // Check that the two columns do not have a value higher than the maximum amount.
        for column in [self.nodes_column_number, self.node_types_column_number]
            .iter()
            .filter_map(|maybe_column| *maybe_column)
        {
            if column >= expected_number_of_elements {
                return Err(format!(
                    concat!(
                        "A column number passed was {} but ",
                        "the first parsable line has {} values."
                    ),
                    column, expected_number_of_elements
                ));
            }
        }

        Ok(self
            .reader
            .read_lines()?
            .enumerate()
            .map(move |(line_number, values)| match values {
                Ok(vals) => {
                    let node_name = match self.nodes_column_number {
                        Some(column) => match vals[column].to_owned() {
                            Some(node_name) => node_name,
                            None => {
                                return Err(
                                    "One of the provided node IDs is empty or None.".to_owned()
                                )
                            }
                        },
                        None => line_number.to_string(),
                    };
                    let maybe_node_types_string = match self.node_types_column_number {
                        Some(column) => match vals[column].to_owned() {
                            Some(node_type) => Some(node_type),
                            None => self.default_node_type.clone(),
                        },
                        None => self.default_node_type.clone(),
                    };

                    // Split given node types using the provided node type separator.
                    let node_types = match maybe_node_types_string {
                        Some(string) => match &self.node_types_separator {
                            Some(sep) => Some(string.split(sep).map(String::from).collect()),
                            None => Some(vec![string]),
                        },
                        None => None,
                    };

                    // Return tuple with string and list of node types
                    Ok((node_name, node_types))
                }
                Err(e) => Err(e),
            }))
    }
}
