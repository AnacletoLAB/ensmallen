#[warn(unused_macros)]
use super::*;
use std::collections::{HashMap, HashSet};

fn parse_edge_weight(weight: Option<String>) -> Result<Option<WeightT>, String> {
    match weight {
        None => Ok(None),
        Some(w) => match w.parse::<WeightT>() {
            Ok(val) => match val.is_finite() {
                true => Ok(Some(val)),
                false => Err(format!(
                    "The value {} parsed as a weight as {} is either infinite or NaN.",
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
                let mut curr = vals[idx].to_owned();
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
                let mut curr = vals[idx].to_owned();
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

    /// Convert the vectorsof elements for each line othe csv to a tuple
    /// that is (node_name, node_type)
    /// This is a private method and only serves as an utility for read_node_file.
    fn read_lines(
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

    /// Read node file and returns graph builder data structures.
    ///
    /// Specifically, the returned objects are:
    /// * nodes_mapping: an hashmap from the node name to the node id.
    /// * node_reverse_mapping: vector of node names.
    /// * node_types_mapping: an hashmap from node types names to the node type ids.
    /// * edge_type_reverse_mapping: vector of the node types names.
    /// * node_types: vector of the numeric node types ids.
    pub(crate) fn read_edge_file(
        &self,
        nodes_mapping: &mut HashMap<String, NodeT>,
        nodes_reverse_mapping: &mut Vec<String>,
    ) -> Result<(
        Vec<NodeT>,
        Vec<NodeT>,
        HashMap<(NodeT, NodeT), EdgeMetadata>,
        HashMap<String, EdgeTypeT>,
        Vec<String>,
        Vec<EdgeTypeT>,
        Vec<WeightT>
    ), String> {

        let empty_nodes_mapping : bool = nodes_mapping.is_empty();
        let mut sources: Vec<NodeT> = Vec::new();
        let mut destinations: Vec<NodeT> = Vec::new();
        let mut edge_type_mapping: HashMap<String, NodeTypeT> = HashMap::new();
        let mut edge_type_reverse_mapping: Vec<String> = Vec::new();
        let mut edge_types: Vec<NodeTypeT> = Vec::new();
        let mut weights: Vec<WeightT> = Vec::new();
        let mut unique_edges: HashMap<(NodeT, NodeT), EdgeMetadata> = HashMap::new();
        for values in self.read_lines()? {
            let (source_node_name, destination_node_name, edge_type, edge_weight) = values?;
            // Check if we need to skip self-loops
            if self.skip_self_loops && source_node_name == destination_node_name {
                // If current edge is a self-loop and we need to skip them we skip.
                continue;
            }
            // Handle missing node IDs when no node file was provided
            for node_name in &[source_node_name, destination_node_name]{
                if !nodes_mapping.contains_key(node_name){
                    if empty_nodes_mapping {
                        nodes_mapping.insert(node_name.clone(), nodes_mapping.len());
                    } else {
                        return Err(
                            format!(
                                concat!(
                                    "In the edge file was found the node {} ",
                                    "which is not present in the given node file." 
                                ),
                                node_name
                            )
                        )
                    }
                }
            }
            // Retrieve the node IDs
            let source_node_id = nodes_mapping.get(&source_node_name).unwrap();
            let destinations_node_id = nodes_mapping.get(&destination_node_name).unwrap();
            // Retrieve the edge type id if it was given.
            let edge_type_id = if let Some(et) = edge_type {
                if !edge_type_mapping.contains_key(&et) {
                    edge_type_mapping.insert(et, edge_type_reverse_mapping.len() as NodeTypeT);
                    edge_type_reverse_mapping.push(et);
                }
                edge_type_mapping.get(&et)
            } else {
                None
            };

            // Get the metadata of the edge and if it's not present, add it
            let edge_metadata = unique_edges.entry(
                (*source_node_id, *destinations_node_id)
            ).or_insert_with(|| EdgeMetadata{
                edge_id: unique_edges.len(),
                edge_types: HashSet::new()
            });
            
            // if the node is already mapped => duplicated line
            if let Some(eti) = edge_type_id{
                if edge_metadata.edge_types.contains(eti) {
                    if self.ignore_duplicated_edges {
                        continue;
                    } 
                    return Err(format!(
                        concat!(
                            "\nFound duplicated edges!\n",
                            "The source node is {source} and the destination node is {destination}.\n",
                            "The edge type of the row is {edge_type:?}.\n",
                            "The path of the document was {path}."
                        ),
                        source=source_node_name,
                        destination=destination_node_name,
                        edge_type=edge_type,
                        path=self.parameters.path
                    ));  
                }  
                // add the edge type in the metadata
                edge_metadata.edge_types.insert(*eti);
            }
            // update the vectors 
            sources.push(*source_node_id);
            destinations.push(*destinations_node_id);
                            
            if let Some(et) = edge_type_id {
                edge_types.push(*et);
            }
            if let Some(w) = edge_weight {
                weights.push(w);
            }
        }

        Ok((
            sources,
            destinations,
            unique_edges,
            edge_type_mapping,
            edge_type_reverse_mapping,
            edge_types,
            weights
        ))
    }
}
