use itertools::Itertools;

use super::*;
/// Structure that saves the reader specific to writing and reading a nodes csv file.
///
/// # Attributes
#[derive(Clone)]
pub struct EdgeFileReader {
    pub(crate) reader: CSVFileReader,
    pub(crate) sources_column_number: usize,
    pub(crate) destinations_column_number: usize,
    pub(crate) edge_types_column_number: Option<usize>,
    pub(crate) default_edge_type: Option<String>,
    pub(crate) weights_column_number: Option<usize>,
    pub(crate) default_weight: Option<WeightT>,
    pub(crate) skip_selfloops: bool,
    pub(crate) numeric_edge_type_ids: bool,
    pub(crate) numeric_node_ids: bool,
    pub(crate) skip_weights_if_unavailable: bool,
    pub(crate) skip_edge_types_if_unavailable: bool,
    pub(crate) might_have_singletons_with_selfloops: bool,
    pub(crate) might_have_trap_nodes: bool,
}

impl EdgeFileReader {
    /// Return new EdgeFileReader object.
    ///
    /// # Arguments
    ///
    /// * reader: CSVFilereader - Path where to store/load the file.
    ///
    pub fn new<S: Into<String>>(path: S) -> Result<EdgeFileReader, String> {
        Ok(EdgeFileReader {
            reader: CSVFileReader::new(path, "edge list".to_owned())?,
            sources_column_number: 0,
            destinations_column_number: 1,
            edge_types_column_number: None,
            default_edge_type: None,
            weights_column_number: None,
            default_weight: None,
            skip_selfloops: false,
            numeric_edge_type_ids: false,
            numeric_node_ids: false,
            skip_weights_if_unavailable: false,
            skip_edge_types_if_unavailable: false,
            might_have_singletons_with_selfloops: true,
            might_have_trap_nodes: true,
        })
    }

    /// Set the column of the source nodes.
    ///
    /// # Arguments
    ///
    /// * sources_column: Option<String> - The source nodes column to use for the file.
    ///
    pub fn set_sources_column<S: Into<String>>(
        mut self,
        sources_column: Option<S>,
    ) -> Result<EdgeFileReader, String> {
        if let Some(column) = sources_column {
            let column = column.into();
            if column.is_empty() {
                return Err("The given node types column is empty.".to_owned());
            }

            match self.reader.get_column_number(column) {
                Ok(ecn) => {
                    self = self.set_sources_column_number(Some(ecn))?;
                }
                Err(e) => {
                    if !self.skip_edge_types_if_unavailable {
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
    /// * sources_column_number: Option<usize> - The sources column number to use for the file.
    ///
    pub fn set_sources_column_number(
        mut self,
        sources_column_number: Option<usize>,
    ) -> Result<EdgeFileReader, String> {
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

    /// Set the column of the nodes.
    ///
    /// # Arguments
    ///
    /// * destination_column: Option<String> - The node types column to use for the file.
    ///
    pub fn set_destinations_column<S: Into<String>>(
        mut self,
        destinations_column: Option<S>,
    ) -> Result<EdgeFileReader, String> {
        if let Some(column) = destinations_column {
            let column = column.into();
            if column.is_empty() {
                return Err("The given node types column is empty.".to_owned());
            }
            match self.reader.get_column_number(column) {
                Ok(ecn) => {
                    self = self.set_destinations_column_number(Some(ecn))?;
                }
                Err(e) => {
                    if !self.skip_edge_types_if_unavailable {
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
    /// * destinations_column_number: Option<usize> - The destinations column number to use for the file.
    ///
    pub fn set_destinations_column_number(
        mut self,
        destinations_column_number: Option<usize>,
    ) -> Result<EdgeFileReader, String> {
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
    ///
    /// * destination_column: Option<String> - The node types column to use for the file.
    ///
    pub fn set_edge_types_column<S: Into<String>>(
        mut self,
        edge_type_column: Option<S>,
    ) -> Result<EdgeFileReader, String> {
        if let Some(column) = edge_type_column {
            let column = column.into();
            if column.is_empty() {
                return Err("The given node types column is empty.".to_owned());
            }
            match self.reader.get_column_number(column) {
                Ok(ecn) => {
                    self = self.set_edge_types_column_number(Some(ecn))?;
                }
                Err(e) => {
                    if !self.skip_edge_types_if_unavailable {
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
    /// * edge_types_column_number: Option<usize> - The edge_types column number to use for the file.
    ///
    pub fn set_edge_types_column_number(
        mut self,
        edge_types_column_number: Option<usize>,
    ) -> Result<EdgeFileReader, String> {
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

    /// Set the column of the edge weights.
    ///
    /// # Arguments
    ///
    /// * weights_column: Option<String> - The edge weights column to use for the file.
    ///
    pub fn set_weights_column<S: Into<String>>(
        mut self,
        weights_column: Option<S>,
    ) -> Result<EdgeFileReader, String> {
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
    ///
    /// * weights_column_number: Option<usize> - The weights column number to use for the file.
    ///
    pub fn set_weights_column_number(
        mut self,
        weights_column_number: Option<usize>,
    ) -> Result<EdgeFileReader, String> {
        if let Some(wcn) = &weights_column_number {
            let expected_elements = self.reader.get_elements_per_line()?;
            if *wcn >= expected_elements {
                if !self.skip_edge_types_if_unavailable {
                    return Err(format!(
                        concat!(
                            "The weights column number passed was {} but ",
                            "the first parsable line has {} values."
                        ),
                        wcn, expected_elements
                    ));
                }
            } else {
                self.weights_column_number = weights_column_number;
            }
        }
        Ok(self)
    }

    /// Set whether to automatically skip weights if they are not avaitable instead of raising an exception.
    ///
    /// # Arguments
    ///
    /// * skip_weights_if_unavailable: Option<bool> - whether to skip weights if they are not available.
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
    ///
    /// * skip_edge_types_if_unavailable: Option<bool> - whether to skip edge types if they are not available.
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
    ///
    /// * default_weight: Option<WeightT> - The default_weight to use when default_weight is missing.
    ///
    pub fn set_default_weight(mut self, default_weight: Option<WeightT>) -> EdgeFileReader {
        self.default_weight = default_weight;
        self
    }

    /// Set the name of the graph to be loaded.
    ///
    /// # Arguments
    ///
    /// * graph_name: String - The name of the graph to be loaded.
    ///
    pub(crate) fn set_graph_name(mut self, graph_name: String) -> EdgeFileReader {
        self.reader.graph_name = graph_name;
        self
    }

    /// Set the default edge type.
    ///
    /// # Arguments
    ///
    /// * default_edge_type: Option<String> - The edge type to use when edge type is missing.
    ///
    pub fn set_default_edge_type<S: Into<String>>(
        mut self,
        default_edge_type: Option<S>,
    ) -> EdgeFileReader {
        self.default_edge_type = default_edge_type.map(|val| val.into());
        self
    }

    /// Set whether should ignore or not selfloops.
    ///
    /// # Arguments
    ///
    /// * `skip_selfloops`: Option<bool> - whether should ignore or not selfloops.
    ///
    pub fn set_skip_selfloops(mut self, skip_selfloops: Option<bool>) -> EdgeFileReader {
        if let Some(ssl) = skip_selfloops {
            self.skip_selfloops = ssl;
            self.might_have_singletons_with_selfloops = !ssl;
        }
        self
    }

    /// Set whether the CSV is expected to be well written.
    ///
    /// # Arguments
    ///
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
    ///
    /// * comment_symbol: Option<String> - if the reader should ignore or not duplicated edges.
    ///
    pub fn set_comment_symbol(
        mut self,
        comment_symbol: Option<String>,
    ) -> Result<EdgeFileReader, String> {
        if let Some(cs) = comment_symbol {
            if cs.is_empty() {
                return Err("The given comment symbol is empty.".to_string());
            }
            self.reader.comment_symbol = Some(cs);
        }
        Ok(self)
    }

    /// Set the verbose.
    ///
    /// # Arguments
    ///
    /// * `verbose`: Option<bool> - whether to show the loading bar or not.
    ///
    pub fn set_verbose(mut self, verbose: Option<bool>) -> EdgeFileReader {
        if let Some(v) = verbose {
            self.reader.verbose = v;
        }
        self
    }

    /// Set whether you pinky promise that this graph has singletons with self-loops or not.
    ///
    /// # Arguments
    ///
    /// * might_have_singletons_with_selfloops: Option<bool> - Whether this graph has singletons with self-loops.
    ///
    pub fn set_might_have_singletons_with_selfloops(
        mut self,
        might_have_singletons_with_selfloops: Option<bool>,
    ) -> EdgeFileReader {
        if let Some(skip) = might_have_singletons_with_selfloops {
            self.might_have_singletons_with_selfloops = !self.skip_selfloops && skip;
        }
        self
    }

    /// Set whether you pinky promise that this graph has trap nodes or not.
    ///
    /// # Arguments
    ///
    /// * might_have_trap_nodes: Option<bool> - Whether this graph has trap nodes with self-loops.
    ///
    pub fn set_might_have_trap_nodes(
        mut self,
        might_have_trap_nodes: Option<bool>,
    ) -> EdgeFileReader {
        if let Some(skip) = might_have_trap_nodes {
            self.might_have_trap_nodes = skip;
        }
        self
    }

    ///
    /// * numeric_id: Option<bool> - whether to convert numeric Ids to Node Id.
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
    ///
    /// * numeric_id: Option<bool> - whether to convert numeric Ids to Node Id.
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
    ///
    /// * ignore_duplicates: Option<bool> - whether to ignore detected duplicates or raise exception.
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
    ///
    /// * separator: Option<String> - The separator to use for the file.
    ///
    pub fn set_separator<S: Into<String>>(
        mut self,
        separator: Option<S>,
    ) -> Result<EdgeFileReader, String> {
        if let Some(sep) = separator {
            let sep = sep.into();
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
    /// * header: Option<bool> - whether to expect an header or not.
    ///
    pub fn set_header(mut self, header: Option<bool>) -> EdgeFileReader {
        if let Some(v) = header {
            self.reader.header = v;
        }
        self
    }

    /// Set number of rows to be skipped when starting to read file.
    ///
    /// # Arguments
    ///
    /// * rows_to_skip: Option<bool> - whether to show the loading bar or not.
    ///
    pub fn set_rows_to_skip(mut self, rows_to_skip: Option<usize>) -> EdgeFileReader {
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
    pub fn set_max_rows_number(mut self, max_rows_number: Option<u64>) -> EdgeFileReader {
        self.reader.max_rows_number = max_rows_number;
        self
    }

    /// Return boolean representing if the edge types exist.
    pub fn has_edge_types(&self) -> bool {
        self.default_edge_type.is_some() || self.edge_types_column_number.is_some()
    }

    /// Return boolean representing if the weight types exist.
    pub fn has_edge_weights(&self) -> bool {
        self.default_weight.is_some() || self.weights_column_number.is_some()
    }

    /// Parse a single line (vecotr of strings already splitted)
    /// # Arguments
    ///
    /// * vals: Vec<String> - Vector of the values of the line to be parsed
    fn parse_edge_line(&self, vals: Vec<Option<String>>) -> Result<StringQuadruple, String> {
        // extract the values
        let maybe_source_node_name = vals[self.sources_column_number].clone();
        let maybe_destination_node_name = vals[self.destinations_column_number].clone();
        if maybe_source_node_name.is_none() || maybe_destination_node_name.is_none() {
            return Err("Either the source or destination node ID are undefined.".to_string());
        }

        let source_node_name = maybe_source_node_name.unwrap();
        let destination_node_name = maybe_destination_node_name.unwrap();

        // Handle the extraction of the edge types.
        let maybe_edge_types_string = match self.edge_types_column_number {
            Some(column) => match vals[column].to_owned() {
                Some(edge_type) => Some(edge_type),
                None => self.default_edge_type.clone(),
            },
            None => self.default_edge_type.clone(),
        };

        // Handle the extraction of the weights.
        let maybe_weight_string = match self.weights_column_number {
            Some(column) => match vals[column].to_owned() {
                Some(w) => Some(parse_weight(w)?),
                None => self.default_weight,
            },
            None => self.default_weight,
        };

        Ok((
            source_node_name,
            destination_node_name,
            maybe_edge_types_string,
            maybe_weight_string,
        ))
    }

    /// Return iterator of rows of the edge file.
    pub fn read_lines(
        &self,
    ) -> Result<impl Iterator<Item = Result<StringQuadruple, String>> + '_, String> {
        if self.destinations_column_number == self.sources_column_number {
            return Err("The destinations column is the same as the sources one.".to_string());
        }
        if Some(self.destinations_column_number) == self.weights_column_number {
            return Err("The destinations column is the same as the weights one.".to_string());
        }
        if Some(self.sources_column_number) == self.weights_column_number {
            return Err("The sources column is the same as the weights one.".to_string());
        }
        if Some(self.sources_column_number) == self.edge_types_column_number {
            return Err("The sources column is the same as the edge types one.".to_string());
        }
        if Some(self.destinations_column_number) == self.edge_types_column_number {
            return Err("The destinations column is the same as the edge types one.".to_string());
        }
        if self.weights_column_number.is_some()
            && self.weights_column_number == self.edge_types_column_number
        {
            return Err("The weights column is the same as the edge types one.".to_string());
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
            .read_lines()?
            .map(move |values| match values {
                Ok(vals) => self.parse_edge_line(vals),
                Err(e) => Err(e),
            })
            .filter_ok(move |(source_node_name, destination_node_name, _, _)| {
                !self.skip_selfloops || source_node_name != destination_node_name
            }))
    }
}
