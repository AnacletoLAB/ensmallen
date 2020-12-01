use super::*;

/// Structure that saves the reader specific to writing and reading a nodes csv file.
///
/// # Attributes
/// * reader: CSVFile - The common reader for reading and writing a csv.
/// * nodes_column_number: usize - The rank of the column with the nodes names. This parameter is mutually exclusive with nodes_column.
/// * node_types_column_number: Option<usize> - The rank of the column with the nodes types. This parameter is mutually exclusive with node_types_column.
/// * default_node_type: Option<String> - The node type to use if a node has node type or its node type is "".
#[derive(Clone)]
pub struct NodeFileReader {
    pub(crate) reader: CSVFileReader,
    pub(crate) default_node_type: Option<String>,
    pub(crate) nodes_column_number: usize,
    pub(crate) node_types_column_number: Option<usize>,
    pub(crate) numeric_node_ids: bool,
    pub(crate) numeric_node_type_ids: bool,
    pub(crate) skip_node_types_if_unavailable: bool,
}

impl NodeFileReader {
    /// Return new NodeFileReader object.
    ///
    /// # Arguments
    ///
    /// * reader: CSVFileParameters - Path where to store/load the file.
    ///
    pub fn new(path: String) -> Result<NodeFileReader, String> {
        Ok(NodeFileReader {
            reader: CSVFileReader::new(path)?,
            nodes_column_number: 0,
            default_node_type: None,
            node_types_column_number: None,
            numeric_node_ids: false,
            numeric_node_type_ids: false,
            skip_node_types_if_unavailable: false,
        })
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
    ) -> Result<NodeFileReader, String> {
        if let Some(column) = nodes_column {
            if column.is_empty() {
                return Err("The given node column is empty.".to_owned());
            }
            self.nodes_column_number = self.reader.get_column_number(column)?;
            if Some(self.nodes_column_number) == self.node_types_column_number {
                return Err("The node column is the same as the node type one.".to_string());
            }
        }
        Ok(self)
    }

    /// Set the column_number of the nodes.
    ///
    /// # Arguments
    ///
    /// * nodes_column_number: Option<usize> - The nodes column_number to use for the file.
    ///
    pub fn set_nodes_column_number(
        mut self,
        nodes_column_number: Option<usize>,
    ) -> Result<NodeFileReader, String> {
        if let Some(column) = nodes_column_number {
            let expected_number_of_elements = self.reader.get_elements_per_line()?;
            if column >= expected_number_of_elements {
                return Err(format!(
                    concat!(
                        "The nodes column number passed was {} but ",
                        "the first parsable line has {} values."
                    ),
                    column, expected_number_of_elements
                ));
            }
            self.nodes_column_number = column;
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
    ) -> Result<NodeFileReader, String> {
        if let Some(column) = nodes_type_column {
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
    ) -> Result<NodeFileReader, String> {
        if let Some(etcn) = &node_types_column_number {
            let expected_number_of_elements = self.reader.get_elements_per_line()?;
            if *etcn >= expected_number_of_elements {
                if !self.skip_node_types_if_unavailable {
                    return Err(format!(
                        concat!(
                            "The nodes types column number passed was {} but ",
                            "the first parsable line has {} values."
                        ),
                        etcn, expected_number_of_elements
                    ));
                }
            } else {
                self.node_types_column_number = node_types_column_number;
            }
        }
        Ok(self)
    }

    /// Set wether to automatically skip node_types if they are not avaitable instead of raising an exception.
    ///
    /// # Arguments
    ///
    /// * skip_node_types_if_unavailable: Option<bool> - Wether to skip node_types if they are not available.
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

    /// Set the default node type.
    ///
    /// # Arguments
    ///
    /// * default_node_type: Option<String> - The node type to use when node type is missing.
    ///
    pub fn set_default_node_type(mut self, default_node_type: Option<String>) -> NodeFileReader {
        self.default_node_type = default_node_type;
        self
    }

    /// Set the verbose.
    ///
    /// # Arguments
    ///
    /// * verbose: Option<bool> - Wethever to show the loading bar or not.
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
    /// * numeric_node_type_ids: Option<bool> - Wethever to convert numeric node type Ids to Node Type Ids.
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
    /// * numeric_node_ids: Option<bool> - Wethever to convert numeric node type Ids to Node Type Ids.
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
    /// * ignore_duplicates: Option<bool> - Wethever to ignore detected duplicates or raise exception.
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
    pub fn set_separator(mut self, separator: Option<String>) -> Result<NodeFileReader, String> {
        if let Some(sep) = separator {
            if sep.is_empty() {
                return Err("The separator cannot be empty.".to_owned());
            }
            self.reader.separator = sep;
        }
        Ok(self)
    }

    /// Set the header.
    ///
    /// # Arguments
    ///
    /// * header: Option<bool> - Wethever to expect an header or not.
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
    /// * rows_to_skip: Option<bool> - Wethever to show the loading bar or not.
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

    /// Return iterator of the lines of the node file.
    pub fn read_lines(
        &self,
    ) -> Result<impl Iterator<Item = Result<(String, Option<String>), String>> + '_, String> {
        if Some(self.nodes_column_number) == self.node_types_column_number {
            return Err("The node column is the same as the node type one.".to_string());
        }
        let expected_elements = self.reader.get_elements_per_line()?;
        if self.nodes_column_number >= expected_elements {
            return Err(format!(
                concat!(
                    "The nodes column number passed was {} but ",
                    "the first parsable line has {} values."
                ),
                self.nodes_column_number, expected_elements
            ));
        }
        Ok(self.reader.read_lines()?.map(move |values| match values {
            Ok(vals) => {
                let node_name = vals[self.nodes_column_number].to_owned();
                let node_type = if let Some(num) = self.node_types_column_number {
                    let mut node_type = vals[num].to_owned();
                    if node_type.is_empty() {
                        if let Some(dnt) = &self.default_node_type {
                            node_type = dnt.clone();
                        } else {
                            return Err(format!(
                                concat!(
                                    "Found empty node type but no default node ",
                                    "type to use was provided. ",
                                    "The node name is {node_name}.\n",
                                    "The path of the document was {path}.\n"
                                ),
                                node_name = node_name,
                                path = self.reader.path
                            ));
                        }
                    }
                    Some(node_type)
                } else {
                    None
                };
                Ok((node_name, node_type))
            }
            Err(e) => Err(e),
        }))
    }
}
