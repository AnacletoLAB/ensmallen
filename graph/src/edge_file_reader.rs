use num_traits::Zero;
use rayon::iter::ParallelIterator;
use std::collections::HashMap;

use super::*;
/// Structure that saves the reader specific to writing and reading a nodes csv file.
#[derive(Clone)]
#[no_binding]
pub struct EdgeFileReader {
    pub(crate) reader: CSVFileReader,
    pub(crate) edge_ids_column_number: Option<usize>,
    pub(crate) sources_column_number: usize,
    pub(crate) destinations_column_number: usize,
    pub(crate) edge_types_column_number: Option<usize>,
    pub(crate) default_edge_type: Option<String>,
    pub(crate) weights_column_number: Option<usize>,
    pub(crate) default_weight: Option<WeightT>,
    pub(crate) numeric_edge_type_ids: bool,
    pub(crate) numeric_node_ids: bool,
    pub(crate) skip_weights_if_unavailable: bool,
    pub(crate) skip_edge_types_if_unavailable: bool,
    pub(crate) complete: Option<bool>,
    pub(crate) sorted: Option<bool>,
    pub(crate) number_of_edges: Option<EdgeT>,
    pub(crate) node_name_tokens_remapping: Option<HashMap<String, String>>,
}

impl EdgeFileReader {
    /// Return new EdgeFileReader object.
    ///
    /// # Arguments
    /// * `reader`: CSVFilereader - Path where to store/load the file.
    ///
    pub fn new<S: Into<String>>(path: S) -> Result<EdgeFileReader> {
        Ok(EdgeFileReader {
            reader: CSVFileReader::new(path, "edge list".to_owned())?,
            edge_ids_column_number: None,
            sources_column_number: 0,
            destinations_column_number: 1,
            edge_types_column_number: None,
            default_edge_type: None,
            weights_column_number: None,
            default_weight: None,
            numeric_edge_type_ids: false,
            numeric_node_ids: false,
            skip_weights_if_unavailable: false,
            skip_edge_types_if_unavailable: false,
            complete: None,
            sorted: None,
            number_of_edges: None,
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
    ) -> EdgeFileReader {
        self.node_name_tokens_remapping = node_name_tokens_remapping;
        self
    }

    /// Set the column of the edge IDs.
    ///
    /// # Arguments
    /// * `edge_ids_column`: Option<String> - The name of the edge id column to use for the file.
    ///
    pub fn set_edge_ids_column<S: Into<String>>(
        mut self,
        edge_ids_column: Option<S>,
    ) -> Result<EdgeFileReader> {
        if let Some(column) = edge_ids_column {
            let column = column.into();
            if column.is_empty() {
                return Err("The given edge ids column is empty.".to_owned());
            }

            let column_number = self.reader.get_column_number(column)?;

            self = self.set_edge_ids_column_number(Some(column_number))?;
        }
        Ok(self)
    }

    /// Set the edge id node column number.
    ///
    /// # Arguments
    /// * `edge_ids_column_number`: Option<usize> - The edge id column number to use for the file.
    ///
    pub fn set_edge_ids_column_number(
        mut self,
        edge_ids_column_number: Option<usize>,
    ) -> Result<EdgeFileReader> {
        if let Some(column) = edge_ids_column_number {
            let expected_elements = self.reader.get_elements_per_line()?;
            if column >= expected_elements {
                return Err(format!(
                    concat!(
                        "The edge ids column number passed was {} but ",
                        "the first parsable line has {} values."
                    ),
                    column, expected_elements
                ));
            }
            self.edge_ids_column_number = Some(column);
        }
        Ok(self)
    }

    /// Return the edge ids column number.
    pub fn get_edge_ids_column_number(&self) -> Option<usize> {
        self.edge_ids_column_number
    }

    /// Set the column of the source nodes.
    ///
    /// # Arguments
    /// * `sources_column`: Option<String> - The source nodes column to use for the file.
    ///
    pub fn set_sources_column<S: Into<String>>(
        mut self,
        sources_column: Option<S>,
    ) -> Result<EdgeFileReader> {
        if let Some(column) = sources_column {
            let column = column.into();
            if column.is_empty() {
                return Err("The given source nodes column name is empty.".to_owned());
            }

            let column = self.reader.get_column_number(column)?;
            self = self.set_sources_column_number(Some(column))?;
        }
        Ok(self)
    }

    /// Set the sources node column number.
    ///
    /// # Arguments
    /// * `sources_column_number`: Option<usize> - The sources column number to use for the file.
    ///
    pub fn set_sources_column_number(
        mut self,
        sources_column_number: Option<usize>,
    ) -> Result<EdgeFileReader> {
        if let Some(column) = sources_column_number {
            let expected_elements = self.reader.get_elements_per_line()?;
            if column >= expected_elements {
                return Err(format!(
                    concat!(
                        "The source column number passed was {} but ",
                        "the first parsable line has {} values."
                    ),
                    column, expected_elements
                ));
            }
            self.sources_column_number = column;
        }
        Ok(self)
    }

    /// Return the sources column number.
    pub fn get_sources_column_number(&self) -> usize {
        self.sources_column_number
    }

    /// Set the destination nodes column name.
    ///
    /// # Arguments
    /// * `destination_column`: Option<String> - The node types column to use for the file.
    ///
    pub fn set_destinations_column<S: Into<String>>(
        mut self,
        destinations_column: Option<S>,
    ) -> Result<EdgeFileReader> {
        if let Some(column) = destinations_column {
            let column = column.into();
            if column.is_empty() {
                return Err("The given destinations column is empty.".to_owned());
            }
            let column = self.reader.get_column_number(column)?;
            self = self.set_destinations_column_number(Some(column))?;
        }
        Ok(self)
    }

    /// Return the destinations column number.
    pub fn get_destinations_column_number(&self) -> usize {
        self.destinations_column_number
    }

    /// Set the column_number of the nodes.
    ///
    /// # Arguments
    /// * `destinations_column_number`: Option<usize> - The destinations column number to use for the file.
    ///
    pub fn set_destinations_column_number(
        mut self,
        destinations_column_number: Option<usize>,
    ) -> Result<EdgeFileReader> {
        if let Some(column) = destinations_column_number {
            let expected_elements = self.reader.get_elements_per_line()?;
            if column >= expected_elements {
                return Err(format!(
                    concat!(
                        "The destinations column number passed was {} but ",
                        "the first parsable line has {} values."
                    ),
                    column, expected_elements
                ));
            }
            self.destinations_column_number = column;
        }
        Ok(self)
    }

    /// Set the column of the nodes.
    ///
    /// # Arguments
    /// * `edge_type_column`: Option<String> - The node types column to use for the file.
    ///
    pub fn set_edge_types_column<S: Into<String>>(
        mut self,
        edge_type_column: Option<S>,
    ) -> Result<EdgeFileReader> {
        if let Some(column) = edge_type_column {
            let column = column.into();
            if column.is_empty() {
                return Err("The given edge types column is empty.".to_owned());
            }
            let column_number = self.reader.get_column_number(column);
            match (column_number, &self.skip_edge_types_if_unavailable) {
                (Ok(column_number), _) => {
                    self = self.set_edge_types_column_number(Some(column_number))?;
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
    /// * `edge_types_column_number`: Option<usize> - The edge_types column number to use for the file.
    ///
    pub fn set_edge_types_column_number(
        mut self,
        edge_types_column_number: Option<usize>,
    ) -> Result<EdgeFileReader> {
        if let Some(etcn) = &edge_types_column_number {
            let expected_elements = self.reader.get_elements_per_line()?;
            if *etcn >= expected_elements {
                if !self.skip_edge_types_if_unavailable {
                    return Err(format!(
                        concat!(
                            "The edge types column number passed was {} but ",
                            "the first parsable line has {} values."
                        ),
                        etcn, expected_elements
                    ));
                }
            } else {
                self.edge_types_column_number = edge_types_column_number;
            }
        }
        Ok(self)
    }

    /// Return the edge types column number.
    pub fn get_edge_types_column_number(&self) -> Option<usize> {
        self.edge_types_column_number
    }

    /// Set the column of the edge weights.
    ///
    /// # Arguments
    /// * `weights_column`: Option<String> - The edge weights column to use for the file.
    ///
    pub fn set_weights_column<S: Into<String>>(
        mut self,
        weights_column: Option<S>,
    ) -> Result<EdgeFileReader> {
        if let Some(column) = weights_column {
            let column = column.into();
            if column.is_empty() {
                return Err("The given edge weights column is empty.".to_owned());
            }
            match self.reader.get_column_number(column) {
                Ok(wcn) => {
                    self = self.set_weights_column_number(Some(wcn))?;
                }
                Err(e) => {
                    if !self.skip_weights_if_unavailable {
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
    /// * `weights_column_number`: Option<usize> - The weights column number to use for the file.
    ///
    pub fn set_weights_column_number(
        mut self,
        weights_column_number: Option<usize>,
    ) -> Result<EdgeFileReader> {
        if let Some(weights_column_number) = weights_column_number {
            let expected_elements = self.reader.get_elements_per_line()?;
            if weights_column_number >= expected_elements {
                if !self.skip_edge_types_if_unavailable {
                    return Err(format!(
                        concat!(
                            "The weights column number passed was {} but ",
                            "the first parsable line has {} values."
                        ),
                        weights_column_number, expected_elements
                    ));
                }
            } else {
                self.weights_column_number = Some(weights_column_number);
            }
        }
        Ok(self)
    }

    /// Return the edge weights column number.
    pub fn get_weights_column_number(&self) -> Option<usize> {
        self.weights_column_number
    }

    /// Set whether the current edge list is complete.
    ///
    /// # Arguments
    /// * complete: Option<bool> - Whether the edge list is complete.
    ///
    pub fn set_complete(mut self, complete: Option<bool>) -> EdgeFileReader {
        self.complete = complete;
        self
    }

    /// Set whether to load the current graph using the parallel reader or sequential reader.
    ///
    /// # Arguments
    /// * parallel: Option<bool> - Whether to read the edge list using a parallel or sequential reader.
    ///
    pub fn set_parallel(mut self, parallel: Option<bool>) -> EdgeFileReader {
        self.reader = self.reader.set_parallel(parallel);
        self
    }

    /// Set whether remove chevrons while reading elements.
    ///
    /// # Arguments
    /// * remove_chevrons: Option<bool> - Whether to remove chevrons while reading elements.
    ///
    pub fn set_remove_chevrons(mut self, remove_chevrons: Option<bool>) -> EdgeFileReader {
        self.reader = self.reader.set_remove_chevrons(remove_chevrons);
        self
    }

    /// Set whether remove spaces while reading elements.
    ///
    /// # Arguments
    /// * remove_spaces: Option<bool> - Whether to remove spaces while reading elements.
    ///
    pub fn set_remove_spaces(mut self, remove_spaces: Option<bool>) -> EdgeFileReader {
        self.reader = self.reader.set_remove_spaces(remove_spaces);
        self
    }

    /// Set whether the current edge list is sorted.
    ///
    /// # Arguments
    /// * sorted: Option<bool> - Whether the edge list is sorted.
    ///
    pub fn set_sorted(mut self, sorted: Option<bool>) -> EdgeFileReader {
        self.sorted = sorted;
        self
    }

    /// Set whether to automatically skip weights if they are not avaitable instead of raising an exception.
    ///
    /// # Arguments
    /// * skip_weights_if_unavailable: Option<bool> - Whether to skip weights if they are not available.
    ///
    pub fn set_skip_weights_if_unavailable(
        mut self,
        skip_weights_if_unavailable: Option<bool>,
    ) -> EdgeFileReader {
        if let Some(skip) = skip_weights_if_unavailable {
            self.skip_weights_if_unavailable = skip;
        }
        self
    }

    /// Set whether to automatically skip edge types if they are not avaitable instead of raising an exception.
    ///
    /// # Arguments
    /// * skip_edge_types_if_unavailable: Option<bool> - Whether to skip edge types if they are not available.
    ///
    pub fn set_skip_edge_types_if_unavailable(
        mut self,
        skip_edge_types_if_unavailable: Option<bool>,
    ) -> EdgeFileReader {
        if let Some(skip) = skip_edge_types_if_unavailable {
            self.skip_edge_types_if_unavailable = skip;
        }
        self
    }

    /// Set the default default_weight.
    ///
    /// # Arguments
    /// * `default_weight`: Option<WeightT> - The default_weight to use when default_weight is missing.
    ///
    pub fn set_default_weight(mut self, default_weight: Option<WeightT>) -> Result<EdgeFileReader> {
        if let Some(default_weight) = default_weight {
            if default_weight.is_zero() {
                return Err("The default weight cannot be zero.".to_string());
            }
            self.default_weight = Some(default_weight);
        }
        Ok(self)
    }

    /// Set the name of the graph to be loaded.
    ///
    /// # Arguments
    /// * graph_name: String - The name of the graph to be loaded.
    ///
    pub(crate) fn set_graph_name(mut self, graph_name: String) -> EdgeFileReader {
        self.reader.graph_name = graph_name;
        self
    }

    /// Set whether there may be duplicates in the provided edge list.
    ///
    /// # Arguments
    /// * may_have_duplicates: Option<bool> - Whether there may be duplicates in the provided edge list.
    ///
    pub fn set_may_have_duplicates(mut self, may_have_duplicates: Option<bool>) -> EdgeFileReader {
        self.reader.may_have_duplicates = may_have_duplicates;
        self
    }

    /// Set the default edge type.
    ///
    /// # Arguments
    /// * `default_edge_type`: Option<String> - The edge type to use when edge type is missing.
    ///
    pub fn set_default_edge_type<S: Into<String>>(
        mut self,
        default_edge_type: Option<S>,
    ) -> EdgeFileReader {
        self.default_edge_type = default_edge_type.map(|val| val.into());
        self
    }

    /// Set whether the CSV is expected to be well written.
    ///
    /// # Arguments
    /// * csv_is_correct: Option<bool> - Whether you pinky swear the edge list is correct.
    ///
    pub fn set_csv_is_correct(mut self, csv_is_correct: Option<bool>) -> EdgeFileReader {
        if let Some(cic) = csv_is_correct {
            self.reader.csv_is_correct = cic;
        }
        self
    }

    /// Set the comment symbol to use to skip the lines.
    ///
    /// # Arguments
    /// * comment_symbol: Option<String> - if the reader should ignore or not duplicated edges.
    ///
    pub fn set_comment_symbol(mut self, comment_symbol: Option<String>) -> Result<EdgeFileReader> {
        self.reader = self.reader.set_comment_symbol(comment_symbol)?;
        Ok(self)
    }

    /// Set the verbose.
    ///
    /// # Arguments
    /// * `verbose`: Option<bool> - Whether to show the loading bar or not.
    ///
    pub fn set_verbose(mut self, verbose: Option<bool>) -> EdgeFileReader {
        if let Some(v) = verbose {
            self.reader.verbose = v;
        }
        self
    }

    /// Set whether the edge types in the edge list are to be loaded as numeric.
    ///
    /// # Arguments
    /// * `numeric_id`: Option<bool> - Whether to convert numeric Ids to Node Id.
    ///
    pub fn set_numeric_edge_type_ids(
        mut self,
        numeric_edge_type_ids: Option<bool>,
    ) -> EdgeFileReader {
        if let Some(neti) = numeric_edge_type_ids {
            self.numeric_edge_type_ids = neti;
        }
        self
    }

    /// Set the numeric_id.
    ///
    /// # Arguments
    /// * `numeric_id`: Option<bool> - Whether to convert numeric Ids to Node Id.
    ///
    pub fn set_numeric_node_ids(mut self, numeric_node_ids: Option<bool>) -> EdgeFileReader {
        if let Some(nni) = numeric_node_ids {
            self.numeric_node_ids = nni;
        }
        self
    }

    /// Set the ignore_duplicates.
    ///
    /// # Arguments
    /// * ignore_duplicates: Option<bool> - Whether to ignore detected duplicates or raise exception.
    ///
    pub fn set_ignore_duplicates(mut self, ignore_duplicates: Option<bool>) -> EdgeFileReader {
        if let Some(v) = ignore_duplicates {
            self.reader.ignore_duplicates = v;
        }
        self
    }

    /// Set the separator.
    ///
    /// # Arguments
    /// * `separator`: Option<char> - The separator to use for the file.
    ///
    pub fn set_separator(mut self, separator: Option<char>) -> Result<EdgeFileReader> {
        self.reader = self.reader.set_separator(separator)?;
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
    ) -> EdgeFileReader {
        self.reader = self
            .reader
            .set_support_balanced_quotes(support_balanced_quotes);
        self
    }

    /// Return the CSV reader separator
    pub fn get_separator(&self) -> char {
        self.reader.separator.clone()
    }

    /// Set the header.
    ///
    /// # Arguments
    /// * header: Option<bool> - Whether to expect an header or not.
    ///
    pub fn set_header(mut self, header: Option<bool>) -> Result<EdgeFileReader> {
        self.reader = self.reader.set_header(header)?;
        Ok(self)
    }

    /// Return whether the file is set to be read as if it has an header.
    pub fn has_header(&self) -> bool {
        self.reader.header
    }

    /// Set number of rows to be skipped when starting to read file.
    ///
    /// # Arguments
    /// * rows_to_skip: Option<bool> - Whether to show the loading bar or not.
    ///
    pub fn set_rows_to_skip(mut self, rows_to_skip: Option<usize>) -> Result<EdgeFileReader> {
        self.reader = self.reader.set_rows_to_skip(rows_to_skip)?;
        Ok(self)
    }

    /// Set the maximum number of rows to load from the file
    ///
    /// # Arguments
    /// * max_rows_number: Option<usize> - The edge type to use when edge type is missing.
    ///
    pub fn set_max_rows_number(mut self, max_rows_number: Option<usize>) -> Result<EdgeFileReader> {
        self.reader = self.reader.set_max_rows_number(max_rows_number)?;
        Ok(self)
    }

    /// Return boolean representing if the edge types exist.
    pub fn has_edge_types(&self) -> bool {
        self.default_edge_type.is_some() || self.edge_types_column_number.is_some()
    }

    /// Return boolean representing if the weight types exist.
    pub fn has_edge_weights(&self) -> bool {
        self.default_weight.is_some() || self.weights_column_number.is_some()
    }

    /// Set the total number of expected edges.
    ///
    /// # Arguments
    /// * number_of_edges: Option<usize> - The number of edges expected to be loaded.
    ///
    pub fn set_number_of_edges(mut self, number_of_edges: Option<EdgeT>) -> EdgeFileReader {
        self.number_of_edges = number_of_edges;
        self
    }

    /// Returns the total number of lines to be skipped.
    ///
    /// # Arguments
    /// * `skip_header`: bool - Whether to skip the header.
    pub fn get_total_lines_to_skip(&self, skip_header: bool) -> Result<usize> {
        self.reader.get_total_lines_to_skip(skip_header)
    }

    /// Parse a single line (vector of strings already splitted and fitered)
    ///
    /// # Arguments
    /// * `line_number`: usize, Current line number.
    /// * `elements_in_line`: Vec<String> - Vector of the values of the line to be parsed
    fn parse_edge_line(
        &self,
        line_number: usize,
        mut elements_in_line: Vec<Option<String>>,
    ) -> Result<(usize, StringQuadruple)> {
        // extract the values in reverse order

        // First we start with the last, i.e. the weights
        let maybe_weight = if self.weights_column_number.is_some() {
            elements_in_line
                .pop()
                // We can unwrap because the check always happens in the CSV reader
                .unwrap()
                .map_or(Ok::<_, String>(self.default_weight), |candidate_weight| {
                    Ok(Some(parse_weight(candidate_weight.trim())?))
                })?
        } else {
            self.default_weight
        };
        // Next we handle the edge types
        let maybe_edge_types_string = if self.edge_types_column_number.is_some() {
            elements_in_line
                .pop()
                // We can unwrap because the check always happens in the CSV reader
                .unwrap()
                .or_else(|| self.default_edge_type.clone())
        } else {
            self.default_edge_type.clone()
        };

        // Next the destination nodes
        let maybe_destination_node_name = elements_in_line.pop().unwrap();
        // and the source node
        let maybe_source_node_name = elements_in_line.pop().unwrap();
        // We check that these values are actually provided
        if maybe_destination_node_name.is_none() {
            return Err(format!(
                concat!(
                    "While reading the provided edge list, we have encountered ",
                    "an undefined destination node, represented by a NaN or empty value. ",
                    "Such cases are often caused by unexpected separator symbol in ",
                    "other fields of the CSV file (for instance node descriptions) ",
                    "or just error in the prepreocessing pipeline. Do consider loading ",
                    "the edge list in Pandas, if possible, to debug and remove the ",
                    "missing value. ",
                    "The current line number is {}."
                ),
                line_number
            ));
        }
        if maybe_source_node_name.is_none() {
            return Err(format!(
                concat!(
                    "While reading the provided edge list, we have encountered ",
                    "an undefined source node, represented by a NaN or empty value. ",
                    "Such cases are often caused by unexpected separator symbol in ",
                    "other fields of the CSV file (for instance node descriptions) ",
                    "or just error in the prepreocessing pipeline. Do consider loading ",
                    "the edge list in Pandas, if possible, to debug and remove the ",
                    "missing value. ",
                    "The current line number is {}."
                ),
                line_number
            ));
        }

        // Finally we check if the edge ID was provided.
        let line_number = if self.edge_ids_column_number.is_some() {
            let maybe_edge_id = elements_in_line
                .pop()
                // We can unwrap because the check always happens in the CSV reader
                .unwrap();
            if maybe_edge_id.is_none() {
                return Err("The edge id cannot be undefined.".to_owned());
            }
            let edge_id = maybe_edge_id.unwrap();
            match edge_id.as_str().parse::<usize>() {
                Ok(edge_id) => Ok(edge_id),
                Err(_) => Err(format!(
                    concat!(
                        "Unable to pass the edge ID `{:?}` to ",
                        "a numeric value while reading line {}."
                    ),
                    edge_id, line_number
                )),
            }?
        } else {
            line_number
        };

        Ok((
            line_number,
            (
                maybe_source_node_name.unwrap(),
                maybe_destination_node_name.unwrap(),
                maybe_edge_types_string,
                maybe_weight.unwrap_or(WeightT::NAN),
            ),
        ))
    }

    /// Return iterator of rows of the edge file.
    pub fn read_lines(
        &self,
    ) -> Result<
        ItersWrapper<
            Result<(usize, StringQuadruple)>,
            impl Iterator<Item = Result<(usize, StringQuadruple)>> + '_,
            impl ParallelIterator<Item = Result<(usize, StringQuadruple)>> + '_,
        >,
    > {
        let columns_and_names = [
            (self.edge_ids_column_number, "edge ids"),
            (Some(self.sources_column_number), "sources"),
            (Some(self.destinations_column_number), "destinations"),
            (self.edge_types_column_number, "edge types"),
            (self.weights_column_number, "weights"),
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

        let expected_elements = self.reader.get_elements_per_line()?;
        if self.sources_column_number >= expected_elements {
            return Err(format!(
                concat!(
                    "The sources column number passed was {} but ",
                    "the first parsable line has {} values."
                ),
                self.sources_column_number, expected_elements
            ));
        }
        if self.destinations_column_number >= expected_elements {
            return Err(format!(
                concat!(
                    "The destinations column number passed was {} but ",
                    "the first parsable line has {} values."
                ),
                self.destinations_column_number, expected_elements
            ));
        }

        Ok(self
            .reader
            .read_lines(Some(
                [
                    self.edge_ids_column_number,
                    Some(self.sources_column_number),
                    Some(self.destinations_column_number),
                    self.edge_types_column_number,
                    self.weights_column_number,
                ]
                .iter()
                .filter_map(|&e| e)
                .collect(),
            ))?
            .map(move |line| match line {
                Ok((line_number, vals)) => self.parse_edge_line(line_number, vals),
                Err(e) => Err(e),
            }))
    }
}
