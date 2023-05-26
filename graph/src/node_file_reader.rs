use rayon::iter::ParallelIterator;
use std::collections::HashMap;

use super::*;

/// Structure that saves the reader specific to writing and reading a nodes csv file.
#[derive(Clone)]
#[no_binding]
pub struct NodeFileReader {
    pub(crate) reader: Option<CSVFileReader>,
    pub(crate) node_ids_column_number: Option<usize>,
    pub(crate) default_node_type: Option<String>,
    pub(crate) nodes_column_number: Option<usize>,
    pub(crate) node_types_separator: Option<char>,
    pub(crate) node_types_column_number: Option<usize>,
    pub(crate) number_of_nodes: Option<NodeT>,
    pub(crate) minimum_node_id: Option<NodeT>,
    pub(crate) numeric_node_ids: bool,
    pub(crate) numeric_node_type_ids: bool,
    pub(crate) skip_node_types_if_unavailable: bool,
    pub(crate) node_name_tokens_remapping: Option<HashMap<String, String>>,
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
            node_ids_column_number: None,
            default_node_type: None,
            nodes_column_number: None,
            node_types_separator: None,
            node_types_column_number: None,
            number_of_nodes: None,
            minimum_node_id: None,
            numeric_node_ids: !has_path,
            numeric_node_type_ids: false,
            skip_node_types_if_unavailable: false,
            node_name_tokens_remapping: None,
        })
    }

    /// Set the HashMap to be used to replace tokens in the node names.
    ///
    /// This is meant to be useful when the nodes include extremely long
    /// prefixes, such as in graphs like WikiData.
    ///
    /// # Arguments
    /// * `node_name_tokens_remapping`: Option<HashMap<String, String>> - Mapping of tokens to be used to simplify the node names.
    ///
    pub fn set_node_name_tokens_remapping(
        mut self,
        node_name_tokens_remapping: Option<HashMap<String, String>>,
    ) -> NodeFileReader {
        self.node_name_tokens_remapping = node_name_tokens_remapping;
        self
    }

    /// Set the column of the node IDs.
    ///
    /// # Arguments
    /// * node_ids_column: Option<String> - The name of the node id column to use for the file.
    ///
    pub fn set_node_ids_column<S: Into<String>>(
        mut self,
        node_ids_column: Option<S>,
    ) -> Result<NodeFileReader> {
        if let Some(column) = node_ids_column {
            self.must_have_reader()?;
            let column = column.into();
            if column.is_empty() {
                return Err("The given node types column is empty.".to_owned());
            }
            let column_number = self
                .reader
                .as_ref()
                .map_or(Ok::<_, String>(None), |reader| {
                    Ok(Some(reader.get_column_number(column)?))
                })?;
            self = self.set_node_ids_column_number(column_number)?;
        }
        Ok(self)
    }

    /// Set the node id node column number.
    ///
    /// # Arguments
    /// * `node_ids_column_number`: Option<usize> - The node id column number to use for the file.
    ///
    pub fn set_node_ids_column_number(
        mut self,
        node_ids_column_number: Option<usize>,
    ) -> Result<NodeFileReader> {
        if let Some(column) = node_ids_column_number {
            self.must_have_reader()?;
            if let Some(reader) = self.reader.as_mut() {
                let expected_elements = reader.get_elements_per_line()?;
                if column >= expected_elements {
                    return Err(format!(
                        concat!(
                            "The node id column number passed was {} but ",
                            "the first parsable line has {} values."
                        ),
                        column, expected_elements
                    ));
                }
            }
            self.node_ids_column_number = Some(column);
        }
        Ok(self)
    }

    /// Return the node ids column number.
    pub fn get_node_ids_column_number(&self) -> Option<usize> {
        self.node_ids_column_number
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
            let column_number = self
                .reader
                .as_ref()
                .map(|reader| reader.get_column_number(column))
                .unwrap();
            match (column_number, &self.skip_node_types_if_unavailable) {
                (Ok(column_number), _) => {
                    self = self.set_nodes_column_number(Some(column_number))?;
                    Ok(())
                }
                (Err(_), true) => Ok(()),
                (Err(e), false) => Err(e),
            }?;
        }
        Ok(self)
    }

    /// Set the column_number of the nodes.
    ///
    /// # Arguments
    ///
    /// * `nodes_column_number`: Option<usize> - The nodes column_number to use for the file.
    ///
    pub fn set_nodes_column_number(
        mut self,
        nodes_column_number: Option<usize>,
    ) -> Result<NodeFileReader> {
        if let Some(nodes_column_number) = nodes_column_number {
            self.must_have_reader()?;
            if let Some(reader) = self.reader.as_ref() {
                let expected_elements = reader.get_elements_per_line()?;
                if nodes_column_number >= expected_elements {
                    return Err(format!(
                        concat!(
                            "The nodes column number passed was {} but ",
                            "the first parsable line has {} values."
                        ),
                        nodes_column_number, expected_elements
                    ));
                } else {
                    self.nodes_column_number = Some(nodes_column_number);
                }
            }
        }
        Ok(self)
    }

    /// Return the nodes column number.
    pub fn get_nodes_column_number(&self) -> Option<usize> {
        self.nodes_column_number
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
        node_type_column: Option<S>,
    ) -> Result<NodeFileReader> {
        if let Some(column) = node_type_column {
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
            match (column_number, &self.skip_node_types_if_unavailable) {
                (Ok(column_number), _) => {
                    self = self.set_node_types_column_number(Some(column_number))?;
                    Ok(())
                }
                (Err(_), true) => Ok(()),
                (Err(e), false) => Err(e),
            }?;
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
        if let Some(node_types_column_number) = node_types_column_number {
            self.must_have_reader()?;
            if let Some(reader) = self.reader.as_ref() {
                let expected_elements = reader.get_elements_per_line()?;
                if node_types_column_number >= expected_elements {
                    if !self.skip_node_types_if_unavailable {
                        return Err(format!(
                            concat!(
                                "The node types column number passed was {} but ",
                                "the first parsable line has {} values."
                            ),
                            node_types_column_number, expected_elements
                        ));
                    }
                } else {
                    self.node_types_column_number = Some(node_types_column_number);
                }
            }
        }
        Ok(self)
    }

    /// Return the node types column number.
    pub fn get_node_types_column_number(&self) -> Option<usize> {
        self.node_types_column_number
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
        if comment_symbol.is_some() {
            self.must_have_reader()?;
        }
        self.reader = self.reader.map_or(Ok::<_, String>(None), |reader| {
            Ok(Some(reader.set_comment_symbol(comment_symbol)?))
        })?;
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

    /// Return whether the CSV was labelled as correct.
    pub fn is_csv_correct(&self) -> Result<bool> {
        self.must_have_reader()?;
        Ok(self
            .reader
            .as_ref()
            .map(|reader| reader.is_csv_correct())
            .unwrap())
    }

    /// Return whether the CSV was labelled to have numeric node IDs.
    pub fn has_numeric_node_ids(&self) -> bool {
        self.numeric_node_ids
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
    /// * separator: Option<char> - The separator to use for the file.
    ///
    pub fn set_separator(mut self, separator: Option<char>) -> Result<NodeFileReader> {
        if separator.is_some() {
            self.must_have_reader()?;
        }
        self.reader = self.reader.map_or(Ok::<_, String>(None), |reader| {
            Ok(Some(reader.set_separator(separator)?))
        })?;

        Ok(self)
    }

    /// Return separator used for the current node list file.
    pub fn get_separator(&self) -> Result<char> {
        self.must_have_reader()?;
        Ok(self
            .reader
            .as_ref()
            .map(|reader| reader.get_separator())
            .unwrap())
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
    ///
    /// * node_types_separator: Option<String> - The separator to use for the node types column.
    ///
    /// TODO!: Add automatic separator detection
    pub fn set_node_types_separator<S: Into<char>>(
        mut self,
        node_types_separator: Option<S>,
    ) -> Result<NodeFileReader> {
        if let Some(sep) = node_types_separator {
            self.must_have_reader()?;
            self.node_types_separator = Some(sep.into());
        }
        Ok(self)
    }

    /// Set whether to support the balanced quotes while reading the CSV, operation that will significantly slow down the execution.
    ///
    /// # Arguments
    /// * `support_balanced_quotes`: Option<bool> - Whether to support the balanced quotes while reading the CSV.
    ///
    pub fn set_support_balanced_quotes(
        mut self,
        support_balanced_quotes: Option<bool>,
    ) -> Result<NodeFileReader> {
        if support_balanced_quotes.is_some() {
            self.must_have_reader()?;
        }
        self.reader = self
            .reader
            .map(|reader| reader.set_support_balanced_quotes(support_balanced_quotes));
        Ok(self)
    }

    /// Return he node types separator used within this file.
    pub fn get_node_types_separator(&self) -> Option<char> {
        self.node_types_separator.clone()
    }

    /// Set the header.
    ///
    /// # Arguments
    ///
    /// * header: Option<bool> - Whether to expect an header or not.
    ///
    pub fn set_header(mut self, header: Option<bool>) -> Result<NodeFileReader> {
        if header.is_some() {
            self.must_have_reader()?;
        }
        self.reader = self.reader.map_or(Ok::<_, String>(None), |reader| {
            Ok(Some(reader.set_header(header)?))
        })?;
        Ok(self)
    }

    /// Return whether the reader is expected to include an header.
    pub fn has_header(&self) -> Result<bool> {
        self.must_have_reader()?;
        Ok(self
            .reader
            .as_ref()
            .map(|reader| reader.has_header())
            .unwrap())
    }

    /// Set number of rows to be skipped when starting to read file.
    ///
    /// # Arguments
    ///
    /// * rows_to_skip: Option<bool> - Whether to show the loading bar or not.
    ///
    pub fn set_rows_to_skip(mut self, rows_to_skip: Option<usize>) -> Result<NodeFileReader> {
        if rows_to_skip.is_some() {
            self.must_have_reader()?;
        }
        self.reader = self.reader.map_or(Ok::<_, String>(None), |reader| {
            Ok(Some(reader.set_rows_to_skip(rows_to_skip)?))
        })?;
        Ok(self)
    }

    /// Set the maximum number of rows to load from the file
    ///
    /// # Arguments
    /// * max_rows_number: Option<usize> - The edge type to use when edge type is missing.
    ///
    pub fn set_max_rows_number(mut self, max_rows_number: Option<usize>) -> Result<NodeFileReader> {
        if max_rows_number.is_some() {
            self.must_have_reader()?;
        }
        self.reader = self.reader.map_or(Ok::<_, String>(None), |reader| {
            Ok(Some(reader.set_max_rows_number(max_rows_number)?))
        })?;
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

    /// Set whether remove chevrons while reading elements.
    ///
    /// # Arguments
    /// * remove_chevrons: Option<bool> - Whether to remove chevrons while reading elements.
    ///
    pub fn set_remove_chevrons(mut self, remove_chevrons: Option<bool>) -> NodeFileReader {
        self.reader = self
            .reader
            .map(|reader| reader.set_remove_chevrons(remove_chevrons));
        self
    }

    /// Set whether remove spaces while reading elements.
    ///
    /// # Arguments
    /// * remove_spaces: Option<bool> - Whether to remove spaces while reading elements.
    ///
    pub fn set_remove_spaces(mut self, remove_spaces: Option<bool>) -> NodeFileReader {
        self.reader = self
            .reader
            .map(|reader| reader.set_remove_spaces(remove_spaces));
        self
    }

    /// Return boolean representing if the node types exist.
    pub fn has_node_types(&self) -> bool {
        self.default_node_type.is_some() || self.node_types_column_number.is_some()
    }

    /// Set the total number of expected nodes.
    ///
    /// # Arguments
    /// * number_of_nodes: Option<usize> - The number of nodes expected to be loaded.
    ///
    pub fn set_number_of_nodes(mut self, number_of_nodes: Option<NodeT>) -> NodeFileReader {
        self.number_of_nodes = number_of_nodes;
        self
    }

    /// Set the minimum node ID.
    ///
    /// # Arguments
    /// * `minimum_node_id`: Option<usize> - The minimum node ID to expect when loading numeric node IDs.
    ///
    pub fn set_minimum_node_id(mut self, minimum_node_id: Option<NodeT>) -> NodeFileReader {
        self.minimum_node_id = minimum_node_id;
        self
    }

    /// Get the minimum node ID.
    pub fn get_minimum_node_id(&self) -> Option<NodeT> {
        self.minimum_node_id
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
    ) -> Result<(usize, (String, Option<Vec<String>>))> {
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
                Some(sep) => node_types_string.split(*sep).map(String::from).collect(),
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

        // Finally we check if the node ID was provided.
        let line_number = if self.node_ids_column_number.is_some() {
            let maybe_node_id = elements_in_line
                .pop()
                // We can unwrap because the check always happens in the CSV reader
                .unwrap();
            if maybe_node_id.is_none() {
                return Err("The node id cannot be undefined.".to_owned());
            }
            let node_id = maybe_node_id.unwrap();
            match node_id.as_str().parse::<usize>() {
                Ok(node_id) => Ok(node_id),
                Err(_) => Err(format!(
                    concat!(
                        "Unable to pass the node ID `{:?}` to ",
                        "a numeric value while reading line {}."
                    ),
                    node_id, line_number
                )),
            }?
        } else {
            line_number
        };

        // Return tuple with string and list of node types
        Ok((line_number, (node_name, node_types)))
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
                return Err(format!(
                    concat!(
                        "Neither node names column nor node types column was provided ",
                        "therefore it is not clear what to load from the provided node ",
                        "list file.\n",
                        "Either do not provide this node list file or alternatively ",
                        "do provide the node names column and/or the node types column.\n",
                        "Note that if the node names column is not provided but the ",
                        "node types column is provided, the node names used will be the ",
                        "lines number.\n",
                        "The file path is {:?}."
                    ),
                    self.reader.as_ref().map(|reader| reader.path.clone())
                ));
            }

            let columns_and_names = [
                (self.node_ids_column_number, "node ids"),
                (self.nodes_column_number, "node names"),
                (self.node_types_column_number, "node types"),
            ];

            for (outer_column_number, outer_column_name) in columns_and_names.iter() {
                if let Some(outer_column_number) = outer_column_number {
                    for (inner_column_number, inner_column_name) in columns_and_names.iter() {
                        if outer_column_name == inner_column_name {
                            continue;
                        }
                        if let Some(inner_column_number) = inner_column_number {
                            if outer_column_number == inner_column_number {
                                return Err(format!(
                                    concat!(
                                        "The column number {} provided for column {} ",
                                        "is the same column number {} provided for column {}."
                                    ),
                                    outer_column_number,
                                    outer_column_name,
                                    inner_column_number,
                                    inner_column_name
                                ));
                            }
                        }
                    }
                }
            }

            // Retrieve the expected maximum number of columns.
            let expected_number_of_elements = reader.get_elements_per_line()?;

            // Check that the two columns do not have a value higher than the maximum amount.
            for column in [
                self.node_ids_column_number,
                self.nodes_column_number,
                self.node_types_column_number,
            ]
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
                .read_lines(Some(
                    [
                        self.node_ids_column_number,
                        self.nodes_column_number,
                        self.node_types_column_number,
                    ]
                    .iter()
                    .filter_map(|&e| e)
                    .collect(),
                ))?
                .map(move |line| match line {
                    Ok((line_number, elements_in_line)) => {
                        self.parse_node_line(line_number, elements_in_line)
                    }
                    Err(e) => Err(e),
                }))
        })
    }
}
