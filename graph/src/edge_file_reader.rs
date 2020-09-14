use super::*;

fn parse_edge_weight(weight: Option<String>) -> Result<Option<WeightT>, String> {
    match weight {
        None => Ok(None),
        Some(w) => match w.parse::<WeightT>() {
            Ok(val) => match val.is_finite() && val > 0.0 {
                true => Ok(Some(val)),
                false => Err(format!(
                    "The value {} parsed as a weight as {} is either infinite or NaN or Zero.",
                    w, val
                )),
            },
            Err(_) => Err(format!("Cannot parse weight {} as a float.", w)),
        },
    }
}

/// Structure that saves the parameters specific to writing and reading a nodes csv file.
///
/// # Attributes
pub struct EdgeFileReader {
    pub(crate) parameters: CSVFileReader,
    pub(crate) sources_column_number: usize,
    pub(crate) destinations_column_number: usize,
    pub(crate) edge_types_column_number: Option<usize>,
    pub(crate) default_edge_type: Option<String>,
    pub(crate) weights_column_number: Option<usize>,
    pub(crate) default_weight: Option<WeightT>,
    pub(crate) skip_self_loops: bool,
    pub(crate) ignore_duplicated_edges: bool,
}

impl EdgeFileReader {
    /// Return new EdgeFileReader object.
    ///
    /// # Arguments
    ///
    /// * parameters: CSVFileParameters - Path where to store/load the file.
    ///
    pub fn new(parameters: CSVFileReader) -> EdgeFileReader {
        EdgeFileReader {
            parameters,
            sources_column_number: 0,
            destinations_column_number: 1,
            edge_types_column_number: None,
            default_edge_type: None,
            weights_column_number: None,
            default_weight: None,
            skip_self_loops: false,
            ignore_duplicated_edges: false,
        }
    }

    /// Set the column of the source nodes.
    ///
    /// # Arguments
    ///
    /// * sources_column: Option<String> - The source nodes column to use for the file.
    ///
    pub fn set_sources_column(
        mut self,
        sources_column: Option<String>,
    ) -> Result<EdgeFileReader, String> {
        if let Some(column) = sources_column {
            self.sources_column_number = self.parameters.get_column_number(column)?;
        }
        Ok(self)
    }

    /// Set the column of the nodes.
    ///
    /// # Arguments
    ///
    /// * destination_column: Option<String> - The node types column to use for the file.
    ///
    pub fn set_destinations_column(
        mut self,
        destinations_column: Option<String>,
    ) -> Result<EdgeFileReader, String> {
        if let Some(column) = destinations_column {
            self.destinations_column_number = self.parameters.get_column_number(column)?;
        }
        Ok(self)
    }

    /// Set the column of the nodes.
    ///
    /// # Arguments
    ///
    /// * destination_column: Option<String> - The node types column to use for the file.
    ///
    pub fn set_edge_types_column(
        mut self,
        edge_type_column: Option<String>,
    ) -> Result<EdgeFileReader, String> {
        if let Some(column) = edge_type_column {
            self.edge_types_column_number = Some(self.parameters.get_column_number(column)?);
        }
        Ok(self)
    }

    /// Set the column of the nodes.
    ///
    /// # Arguments
    ///
    /// * destination_column: Option<String> - The node types column to use for the file.
    ///
    pub fn set_weights_column(
        mut self,
        weights_column: Option<String>,
    ) -> Result<EdgeFileReader, String> {
        if let Some(column) = weights_column {
            self.weights_column_number = Some(self.parameters.get_column_number(column)?);
        }
        Ok(self)
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

    /// Set the default edge type.
    ///
    /// # Arguments
    ///
    /// * default_edge_type: Option<String> - The edge type to use when edge type is missing.
    ///
    pub fn set_default_edge_type(mut self, default_edge_type: Option<String>) -> EdgeFileReader {
        self.default_edge_type = default_edge_type;
        self
    }

    /// Set if the reader should ignore or not duplicated edges.
    ///
    /// # Arguments
    ///
    /// * ignore_duplicated_edges: Option<bool> - if the reader should ignore or not duplicated edges.
    ///
    pub fn set_ignore_duplicated_edges(
        mut self,
        ignore_duplicated_edges: Option<bool>,
    ) -> EdgeFileReader {
        if let Some(i) = ignore_duplicated_edges {
            self.ignore_duplicated_edges = i;
        }
        self
    }

    /// Set if the reader should ignore or not duplicated edges.
    ///
    /// # Arguments
    ///
    /// * skip_self_loops: Option<bool> - if the reader should ignore or not duplicated edges.
    ///
    pub fn set_skip_self_loops(mut self, skip_self_loops: Option<bool>) -> EdgeFileReader {
        if let Some(i) = skip_self_loops {
            self.skip_self_loops = i;
        }
        self
    }

    /// Parse a single line (vecotr of strings already splitted)
    /// # Arguments
    ///
    /// * vals: Vec<String> - Vector of the values of the line to be parsed
    fn parse_edge_line(
        &self,
        vals: Vec<String>,
    ) -> Result<(String, String, Option<String>, Option<WeightT>), String> {
        // exctract the values
        let source_node_name = vals[self.sources_column_number].to_owned();
        let destination_node_name = vals[self.destinations_column_number].to_owned();
        // extract the edge type if present
        let edge_type: Option<String> = match self.edge_types_column_number {
            None => Ok(None),
            Some(idx) => {
                let curr = vals[idx].to_owned();
                if !curr.is_empty() {
                    Ok(Some(curr))
                } else if let Some(def) = &self.default_edge_type {
                    Ok(Some(def.clone()))
                } else {
                    Err(format!(
                        concat!(
                            "Found empty edge type but no default edge ",
                            "type to use was provided.",
                            "The source node name is {source_node_name}.\n",
                            "The destination node name is {destination_node_name}.\n",
                            "The path of the document was {path}.\n"
                        ),
                        source_node_name = source_node_name,
                        destination_node_name = destination_node_name,
                        path = self.parameters.path
                    ))
                }
            }
        }?;
        // extract the weights
        let edge_weight = match self.weights_column_number {
            None => Ok(None),
            Some(idx) => {
                let curr = vals[idx].to_owned();
                if !curr.is_empty() {
                    match parse_edge_weight(Some(curr)) {
                        Ok(v) => Ok(v),
                        Err(e) => Err(e),
                    }
                } else if let Some(def) = &self.default_weight {
                    Ok(Some(*def))
                } else {
                    Err(format!(
                        concat!(
                            "Found empty weight but no default wright ",
                            "to use was provided.",
                            "The source node name is {source_node_name}.\n",
                            "The destination node name is {destination_node_name}.\n",
                            "The path of the document was {path}.\n"
                        ),
                        source_node_name = source_node_name,
                        destination_node_name = destination_node_name,
                        path = self.parameters.path
                    ))
                }
            }
        }?;

        Ok((
            source_node_name,
            destination_node_name,
            edge_type,
            edge_weight,
        ))
    }

    /// Return iterator of rows of the edge file.
    pub fn read_lines(
        &self,
    ) -> Result<
        impl Iterator<Item = Result<(String, String, Option<String>, Option<WeightT>), String>> + '_,
        String,
    > {
        Ok(self
            .parameters
            .read_lines()?
            .map(move |values| match values {
                Ok(vals) => self.parse_edge_line(vals),
                Err(e) => Err(e),
            }))
    }
}
