use rayon::iter::ParallelIterator;

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
    pub(crate) reader: Option<CSVFileReader>,
    pub(crate) default_node_type: Option<String>,
    pub(crate) nodes_column_number: Option<usize>,
    pub(crate) node_types_separator: Option<String>,
    pub(crate) node_types_column_number: Option<usize>,
    pub(crate) nodes_number: Option<NodeT>,
    pub(crate) minimum_node_id: Option<NodeT>,
    pub(crate) numeric_node_ids: bool,
    pub(crate) numeric_node_type_ids: bool,
    pub(crate) skip_node_types_if_unavailable: bool,
}

impl NodeFileReader {
    /// Return new NodeFileReader object.
    ///
    /// # Arguments
    ///
    /// * path: Option<String> - Optional path from where to load the node list.
    ///
    pub fn new(path: Option<String>) -> Result<NodeFileReader> {
        let has_path = path.is_some();
        Ok(NodeFileReader {
            reader: path.map_or(Ok::<_, String>(None), |path| {
                Ok(Some(CSVFileReader::new(path, "node list".to_owned())?))
            })?,
            default_node_type: None,
            nodes_column_number: None,
            node_types_separator: None,
            node_types_column_number: None,
            nodes_number: None,
            minimum_node_id: None,
            numeric_node_ids: !has_path,
            numeric_node_type_ids: false,
            skip_node_types_if_unavailable: false,
        })
    }

    /// Raises an error if the file reader was not created.
    fn must_have_reader(&self) -> Result<()> {
        if self.reader.is_none() {
            return Err(concat!(
                "This particular instance of the ",
                "node file reader was not created with a file."
            )
            .to_string());
        }
        Ok(())
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
    ) -> Result<NodeFileReader> {
        if let Some(column) = nodes_column {
            self.must_have_reader()?;
            let column = column.into();
            if column.is_empty() {
                return Err("The given node column is empty.".to_owned());
            }
            self.nodes_column_number = self
                .reader
                .as_ref()
                .map_or(Ok::<_, String>(None), |reader| {
                    Ok(Some(reader.get_column_number(column)?))
                })?;
        }
        Ok(self)
    }

    /// Set the column_number of the nodes.
    ///
    /// # Arguments
    ///
    /// * nodes_column_number: Option<usize> - The nodes column_number to use for the file.
    ///t
    pub fn set_nodes_column_number(
        mut self,
        nodes_column_number: Option<usize>,
    ) -> Result<NodeFileReader> {
        if let Some(column) = nodes_column_number {
            self.must_have_reader()?;
            self.nodes_column_number = Some(column);
        }
        Ok(self)
    }

    /// Set the name of the graph to be loaded.
    ///
    /// # Arguments
    ///
    /// * graph_name: String - The name of the graph to be loaded.
    ///
    pub(crate) fn set_graph_name(mut self, graph_name: String) -> NodeFileReader {
        self.reader
            .as_mut()
            .map(|reader| reader.graph_name = graph_name);
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
    ) -> Result<NodeFileReader> {
        if let Some(column) = nodes_type_column {
            self.must_have_reader()?;
            let column = column.into();
            if column.is_empty() {
                return Err("The given node types column is empty.".to_owned());
            }
            let column_number = self
                .reader
                .as_ref()
                .map(|reader| reader.get_column_number(column))
                .unwrap();
            match column_number {
                Ok(ecn) => {
                    self.node_types_column_number = Some(ecn);
                    Ok(())
                }
                Err(e) => {
                    if !self.skip_node_types_if_unavailable {
                        Err(e)
                    } else {
                        Ok(())
                    }
                }
            }?
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
    ) -> Result<NodeFileReader> {
        if let Some(column) = node_types_column_number {
            self.must_have_reader()?;
            self.node_types_column_number = Some(column);
        }
        Ok(self)
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
    ) -> Result<NodeFileReader> {
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
    pub fn set_comment_symbol(mut self, comment_symbol: Option<String>) -> Result<NodeFileReader> {
        if let Some(comment_symbol) = comment_symbol {
            self.must_have_reader()?;
            if comment_symbol.is_empty() {
                return Err("The given comment symbol is empty.".to_string());
            }
            if let Some(reader) = self.reader.as_mut(){
                reader.set_comment_symbol(comment_symbol)?;
            }
        }
        Ok(self)
    }

    /// Set whether the CSV is expected to be well written.
    ///
    /// # Arguments
    ///
    /// * csv_is_correct: Option<bool> - Whether you pinky swear the node list is correct.
    ///
    pub fn set_csv_is_correct(mut self, csv_is_correct: Option<bool>) -> Result<NodeFileReader> {
        if let Some(cic) = csv_is_correct {
            self.must_have_reader()?;
            self.reader
                .as_mut()
                .map(|reader| reader.csv_is_correct = cic);
        }
        Ok(self)
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
            self.reader.as_mut().map(|reader| reader.verbose = v);
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
    ) -> Result<NodeFileReader> {
        if let Some(nnti) = numeric_node_type_ids {
            self.must_have_reader()?;
            self.numeric_node_type_ids = nnti;
        }
        Ok(self)
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
    pub fn set_ignore_duplicates(
        mut self,
        ignore_duplicates: Option<bool>,
    ) -> Result<NodeFileReader> {
        if let Some(id) = ignore_duplicates {
            self.must_have_reader()?;
            self.reader.as_mut().map(|reader| reader.verbose = id);
        }
        Ok(self)
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
    ) -> Result<NodeFileReader> {
        if let Some(separator) = separator {
            self.must_have_reader()?;
            let separator = separator.into();
            if separator.is_empty() {
                return Err("The separator cannot be empty.".to_owned());
            }
            self.reader.as_mut().map(|reader| reader.set_separator(separator));
        } else if let Some(reader) = &mut self.reader {
            reader.set_separator(reader.detect_separator()?);
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
    /// TODO!: Add automatic separator detection
    pub fn set_node_types_separator<S: Into<String>>(
        mut self,
        node_types_separator: Option<S>,
    ) -> Result<NodeFileReader> {
        if let Some(sep) = node_types_separator {
            self.must_have_reader()?;
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
    pub fn set_header(mut self, header: Option<bool>) -> Result<NodeFileReader> {
        if let Some(header) = header {
            self.must_have_reader()?;
            if let Some(reader) = self.reader.as_mut(){
                reader.set_header(header)?;
            }
        }
        Ok(self)
    }

    /// Set number of rows to be skipped when starting to read file.
    ///
    /// # Arguments
    ///
    /// * rows_to_skip: Option<bool> - Whether to show the loading bar or not.
    ///
    pub fn set_rows_to_skip(mut self, rows_to_skip: Option<usize>) -> Result<NodeFileReader> {
        if let Some(rows_to_skip) = rows_to_skip {
            self.must_have_reader()?;
            if let Some(reader) = self.reader.as_mut(){
                reader.set_rows_to_skip(rows_to_skip)?;
            }
        }
        Ok(self)
    }

    /// Set the maximum number of rows to load from the file
    ///
    /// # Arguments
    /// * max_rows_number: Option<usize> - The edge type to use when edge type is missing.
    ///
    pub fn set_max_rows_number(mut self, max_rows_number: Option<usize>) -> Result<NodeFileReader> {
        if let Some(max_rows_number) = max_rows_number {
            self.must_have_reader()?;
            if let Some(reader) = self.reader.as_mut(){
                reader.set_max_rows_number(max_rows_number)?;
            }
        }
        Ok(self)
    }

    /// Set whether to load the node list in sequential or in parallel.
    ///
    /// # Arguments
    /// * parallel: Option<bool> - Whether to load the node list in sequential or parallel.
    ///
    pub fn set_parallel(mut self, parallel: Option<bool>) -> Result<NodeFileReader> {
        if let Some(parallel) = parallel {
            self.must_have_reader()?;
            self.reader
                .as_mut()
                .map(|reader| reader.parallel = parallel);
        }
        Ok(self)
    }

    /// Return boolean representing if the node types exist.
    pub fn has_node_types(&self) -> bool {
        self.default_node_type.is_some() || self.node_types_column_number.is_some()
    }

    /// Set the total number of expected nodes.
    ///
    /// # Arguments
    /// * nodes_number: Option<usize> - The number of nodes expected to be loaded.
    ///
    pub fn set_nodes_number(mut self, nodes_number: Option<NodeT>) -> NodeFileReader {
        self.nodes_number = nodes_number;
        self
    }

    /// Set the minimum node ID.
    ///
    /// # Arguments
    /// * minimum_node_id: Option<usize> - The minimum node ID to expect when loading numeric node IDs.
    ///
    pub fn set_minimum_node_id(mut self, minimum_node_id: Option<NodeT>) -> NodeFileReader {
        self.minimum_node_id = minimum_node_id;
        self
    }

    /// Parse a single line (vector of strings already splitted and fitered)
    ///
    /// # Arguments
    /// * `line_number`: usize - Line number in the node list.
    /// * `elements_in_line`: Vec<String> - Vector of the values of the line to be parsed
    fn parse_node_line(
        &self,
        line_number: usize,
        mut elements_in_line: Vec<Option<String>>,
    ) -> Result<(String, Option<Vec<String>>)> {
        // extract the values in reverse order
        // We start with the node types
        let maybe_node_types_string = if self.node_types_column_number.is_some() {
            elements_in_line
                .pop()
                .unwrap()
                .or_else(|| self.default_node_type.clone())
        } else {
            self.default_node_type.clone()
        };

        // Split given node types using the provided node type separator.
        let node_types =
            maybe_node_types_string.map(|node_types_string| match &self.node_types_separator {
                Some(sep) => node_types_string.split(sep).map(String::from).collect(),
                None => vec![node_types_string],
            });

        // Then we proceed with the node name
        let node_name = if self.nodes_column_number.is_some() {
            match elements_in_line.pop().unwrap() {
                Some(node_name) => node_name,
                None => {
                    return Err(format!(
                        concat!(
                            "While reading the provided node list, ",
                            "one of the provided node IDs is empty or None.\n",
                            "The number of the line with the error is {}."
                        ),
                        line_number
                    ))
                }
            }
        } else {
            line_number.to_string()
        };

        // Return tuple with string and list of node types
        Ok((node_name, node_types))
    }

    /// Return iterator of the lines of the node file.
    pub fn read_lines(
        &self,
    ) -> Option<
        Result<
            ItersWrapper<
                Result<(usize, (String, Option<Vec<String>>))>,
                impl Iterator<Item = Result<(usize, (String, Option<Vec<String>>))>> + '_,
                impl ParallelIterator<Item = Result<(usize, (String, Option<Vec<String>>))>> + '_,
            >,
        >,
    > {
        self.reader.as_ref().map(|reader| {
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
            let expected_number_of_elements = reader.get_elements_per_line()?;

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

            Ok(reader
                .read_lines(
                    [self.nodes_column_number, self.node_types_column_number]
                        .iter()
                        .filter_map(|&e| e)
                        .collect(),
                )?
                .map(move |line| match line {
                    Ok((line_number, elements_in_line)) => Ok((
                        line_number,
                        self.parse_node_line(line_number, elements_in_line)?,
                    )),
                    Err(e) => Err(e),
                }))
        })
    }
}
