use super::*;
use std::collections::{HashMap};

/// Structure that saves the parameters specific to writing and reading a nodes csv file.
///
/// # Attributes
/// * parameters: CSVFile - The common parameters for readin and writing a csv.
/// * nodes_column_number: usize - The rank of the column with the nodes names. This parameter is mutually exclusive with nodes_column.
/// * node_types_column_number: Option<usize> - The rank of the column with the nodes types. This parameter is mutually exclusive with node_types_column.
/// * default_node_type: Option<String> - The node type to use if a node has node type or its node type is "".
pub struct NodeFileReader {
    pub(crate) parameters: CSVFileReader,
    pub(crate) default_node_type: Option<String>,
    pub(crate) nodes_column_number: usize,
    pub(crate) node_types_column_number: Option<usize>,
    pub(crate) ignore_duplicated_nodes: bool
}

impl NodeFileReader {
    /// Return new NodeFileReader object.
    ///
    /// # Arguments
    ///
    /// * parameters: CSVFileParameters - Path where to store/load the file.
    ///
    pub fn new(parameters: CSVFileReader) -> NodeFileReader {
        NodeFileReader {
            parameters,
            nodes_column_number: 0,
            default_node_type: None,
            node_types_column_number: None,
            ignore_duplicated_nodes: false
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
    ) -> Result<NodeFileReader, String> {
        if let Some(column) = nodes_column {
            self.nodes_column_number = self.parameters.get_column_number(column)?;
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
            self.node_types_column_number = Some(self.parameters.get_column_number(column)?);
        }
        Ok(self)
    }

    /// Set the column_number of the nodes.
    ///
    /// # Arguments
    ///
    /// * nodes_column_number: Option<usize> - The nodes column_number to use for the file.
    ///
    pub fn set_nodes_column_number(mut self, nodes_column_number: Option<usize>) -> NodeFileReader {
        if let Some(column) = nodes_column_number {
            self.nodes_column_number = column;
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
    ) -> NodeFileReader {
        self.node_types_column_number = node_types_column_number;
        self
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
    
    /// Set if the reader should ignore or not duplicated nodes.
    ///
    /// # Arguments
    ///
    /// * ignore_duplicated_nodes: Option<bool> - if the reader should ignore or not duplicated nodes.
    ///
    pub fn set_ignore_duplicated_nodes(mut self, ignore_duplicated_nodes: Option<bool>) -> NodeFileReader {
        if let Some(i) = ignore_duplicated_nodes {
            self.ignore_duplicated_nodes = i;
        }
        self
    }
    
    /// Convert the vectorsof elements for each line othe csv to a tuple
    /// that is (node_name, node_type)
    /// This is a private method and only serves as an utility for read_node_file.
    fn read_lines(
        &self,
    ) -> Result<impl Iterator<Item = Result<(String, Option<String>), String>> + '_, String> {
        Ok(self.parameters.read_lines()?.map(move |values| match values {
            Ok(vals) => {
                let node_name = vals[self.nodes_column_number].to_owned();
                let node_type = if let Some(num) = self.node_types_column_number {
                    let mut node_type  = vals[num].to_owned();
                    if node_type.is_empty() {
                        if let Some(dnt) = &self.default_node_type {
                            node_type = dnt.clone();
                        } else {
                            return Err(format!(
                                concat!(
                                    "Found empty node type but no default node ",
                                    "type to use was provided.",
                                    "The node name is {node_name}.\n",
                                    "The path of the document was {path}.\n"
                                ),
                                node_name=node_name,
                                path=self.parameters.path
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

    /// Read node file and returns graph builder data structures.
    /// 
    /// Specifically, the returned objects are:
    /// * nodes_mapping: an hashmap from the node name to the node id.
    /// * node_reverse_mapping: vector of node names.
    /// * node_types_mapping: an hashmap from node types names to the node type ids.
    /// * node_types_reverse_mapping: vector of the node types names.
    /// * node_types: vector of the numeric node types ids.
    pub(crate) fn read_node_file(
        &self,
    ) -> Result<(
        HashMap<String, NodeT>,
        Vec<String>, 
        Vec<NodeTypeT>,
        HashMap<String, NodeTypeT>,
        Vec<String>
    ), String> {
        let mut nodes_mapping: HashMap<String, NodeT> = HashMap::new();
        let mut nodes_reverse_mapping: Vec<String> = Vec::new();
        let mut node_types: Vec<NodeTypeT> = Vec::new();
        let mut node_types_mapping: HashMap<String, NodeTypeT> = HashMap::new();
        let mut node_types_reverse_mapping: Vec<String> = Vec::new();

        for values in self.read_lines()? {
            let (node_name, node_type) = values?;
            // if the node is already mapped => duplicated line
            if nodes_mapping.contains_key(&node_name){
                if self.ignore_duplicated_nodes {
                    continue;
                }
                return Err(format!(
                    concat!(
                        "\nFound duplicated nodes!\n",
                        "The node is {node_name}.\n",
                        "The node type of the row is {node_type:?}.\n",
                        "The path of the document was {path}.\n",
                        "The library does not currently support multiple node types for a single node."
                    ),
                    node_name=node_name,
                    node_type = node_type,
                    path = self.parameters.path
                ));
            }
            nodes_mapping.insert(node_name, nodes_mapping.len());
            
            if let Some(ndt) = node_type {
                if !node_types_mapping.contains_key(&ndt) {
                    node_types_mapping.insert(ndt, node_types_reverse_mapping.len() as NodeTypeT);
                    node_types_reverse_mapping.push(ndt);
                }

                node_types.push(*node_types_mapping.get(&ndt).unwrap());
            }
        }

        Ok((
            nodes_mapping,
            nodes_reverse_mapping,
            node_types,
            node_types_mapping,
            node_types_reverse_mapping
        ))
    }
}
