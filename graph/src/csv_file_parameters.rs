

pub struct EdgeFileReader {
    pub(crate) parameters: CSVFileReader,
    pub(crate) sources_column: String,
    pub(crate) destinations_column: String,
    pub(crate) edge_types_column: String,
    pub(crate) default_edge_type: Option<String>,
    pub(crate) weights_column: String,
    pub(crate) default_weight: Option<f64>,
    pub(crate) sources_column_number: usize,
    pub(crate) destinations_column_number: usize,
    pub(crate) edge_types_column_number: usize,
    pub(crate) weights_column_number: usize,
}

pub struct EdgeFileWriter {
    pub(crate) parameters: CSVFileReader,
    pub(crate) sources_column: String,
    pub(crate) destinations_column: String,
    pub(crate) edge_types_column: String,
    pub(crate) weights_column: String,
    pub(crate) sources_column_number: usize,
    pub(crate) destinations_column_number: usize,
    pub(crate) edge_types_column_number: usize,
    pub(crate) weights_column_number: usize,
}


impl NodeFileParameters {
    /// Return new NodeFileParameters object.
    ///
    /// # Arguments
    ///
    /// * parameters: CSVFileParameters - Path where to store/load the file.
    ///
    pub fn new(parameters: CSVFileParameters) -> NodeFileParameters {
        NodeFileParameters {
            parameters,
            consider_node_types: true,
            nodes_column: "id".to_string(),
            nodes_column_number: 0,
            node_types_column: "category".to_string(),
            default_node_type: None,
            node_types_column_number: 1
        }
    }

    /// Set wether to save or not the node types.
    ///
    /// # Arguments
    ///
    /// * consider_node_types: Option<bool> - The nodes column to use for the file.
    ///
    pub fn set_consider_node_types(mut self, consider_node_types: Option<bool>) -> NodeFileParameters {
        if let Some(v) = consider_node_types {
            self.consider_node_types = v;
        }
        self
    }

    /// Set the column of the nodes.
    ///
    /// # Arguments
    ///
    /// * nodes_column: Option<String> - The nodes column to use for the file.
    ///
    pub fn set_nodes_column(mut self, nodes_column: Option<String>) -> NodeFileParameters {
        if let Some(v) = nodes_column {
            self.nodes_column = v;
        }
        self
    }

    /// Set the column of the nodes.
    ///
    /// # Arguments
    ///
    /// * node_types_column: Option<String> - The node types column to use for the file.
    ///
    pub fn set_node_types_column(
        mut self,
        node_types_column: Option<String>,
    ) -> NodeFileParameters {
        if let Some(v) = node_types_column {
            self.node_types_column = v;
        }
        self
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
    ) -> NodeFileParameters {
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
    ) -> NodeFileParameters {
        if let Some(v) = node_types_column_number {
            self.node_types_column_number = v;
        }
        self
    }

    /// Set the default node type.
    ///
    /// # Arguments
    ///
    /// * default_node_type: Option<String> - The node type to use when node type is missing.
    ///
    pub fn set_default_node_type(
        mut self,
        default_node_type: Option<String>,
    ) -> NodeFileParameters {
        self.default_node_type = default_node_type;
        self
    }

    
}

impl EdgeFileParameters {
    /// Return new EdgeFileParameters object.
    ///
    /// # Arguments
    ///
    /// * parameters: CSVFileParameters - Path where to store/load the file.
    ///
    pub fn new(parameters: CSVFileParameters) -> EdgeFileParameters {
        EdgeFileParameters {
            parameters,
            consider_edge_types: true,
            consider_weights: true,
            sources_column: "subject".to_string(),
            destinations_column: "object".to_string(),
            weights_column: "weight".to_string(),
            default_weight: None,
            edge_types_column: "label".to_string(),
            default_edge_type: None,
            sources_column_number: 0,
            edge_types_column_number: 1,
            destinations_column_number: 2,
            weights_column_number: 3,
            ignore_self_loops: false
        }
    }

    /// Set wether to save or not the edge types.
    ///
    /// # Arguments
    ///
    /// * consider_edge_types: Option<bool>,
    ///     Boolean value representing if we should consider the edge types.
    ///
    pub fn set_consider_edge_types(mut self, consider_edge_types: Option<bool>) -> EdgeFileParameters {
        if let Some(v) = consider_edge_types {
            self.consider_edge_types = v;
        }
        self
    }

    /// Set wether to save or not the edge weights.
    ///
    /// # Arguments
    ///
    /// * consider_weights: Option<bool>,
    ///     Boolean value representing if we should consider the weights.
    ///
    pub fn set_consider_weights(mut self, consider_weights: Option<bool>) -> EdgeFileParameters {
        if let Some(v) = consider_weights {
            self.consider_weights = v;
        }
        self
    }

    /// Set the column from where to load the source nodes.
    ///
    /// # Arguments
    ///
    /// * sources_column: Option<String>,
    ///     Boolean value representing if we should consider the weights.
    ///
    pub fn set_sources_column(mut self, sources_column: Option<String>) -> EdgeFileParameters {
        if let Some(v) = sources_column {
            self.sources_column = v;
        }
        self
    }

    /// Set the column from where to load the destination nodes.
    ///
    /// # Arguments
    ///
    /// * destinations_column: Option<String>,
    ///     Boolean value representing if we should consider the weights.
    ///
    pub fn set_destinations_column(mut self, destinations_column: Option<String>) -> EdgeFileParameters {
        if let Some(v) = destinations_column {
            self.destinations_column = v;
        }
        self
    }

    /// Set the column from where to load the weight nodes.
    ///
    /// # Arguments
    ///
    /// * weights_column: Option<String>,
    ///     Boolean value representing if we should consider the weights.
    ///
    pub fn set_weights_column(mut self, weights_column: Option<String>) -> EdgeFileParameters {
        if let Some(v) = weights_column {
            self.weights_column = v;
        }
        self
    }

    /// Set the default weight.
    ///
    /// # Arguments
    ///
    /// * default_weight: Option<String> - The edge weight to use when weight is missing.
    ///
    pub fn set_default_node_type(
        mut self,
        default_weight: Option<f64>,
    ) -> EdgeFileParameters {
        self.default_weight = default_weight;
        self
    }

    /// Set the column from where to load the edge_type nodes.
    ///
    /// # Arguments
    ///
    /// * edge_types_column: Option<String>,
    ///     Column from where to load the edge types.
    ///
    pub fn set_edge_types_column(mut self, edge_types_column: Option<String>) -> EdgeFileParameters {
        if let Some(v) = edge_types_column {
            self.edge_types_column = v;
        }
        self
    }

    /// Set the default edge type.
    ///
    /// # Arguments
    ///
    /// * default_edge_type: Option<String> - The edge type to use when type is missing.
    ///
    pub fn set_default_edge_type(
        mut self,
        default_edge_type: Option<String>,
    ) -> EdgeFileParameters {
        self.default_edge_type = default_edge_type;
        self
    }

    /// Set the column from where to load the source nodes.
    ///
    /// # Arguments
    ///
    /// * sources_column_number: Option<usize>,
    ///     Boolean value representing if we should consider the weights.
    ///
    pub fn set_sources_column_number(mut self, sources_column_number: Option<usize>) -> EdgeFileParameters {
        if let Some(v) = sources_column_number {
            self.sources_column_number = v;
        }
        self
    }

    /// Set the column from where to load the destination nodes.
    ///
    /// # Arguments
    ///
    /// * destinations_column_number: Option<usize>,
    ///     Boolean value representing if we should consider the weights.
    ///
    pub fn set_destinations_column_number(mut self, destinations_column_number: Option<usize>) -> EdgeFileParameters {
        if let Some(v) = destinations_column_number {
            self.destinations_column_number = v;
        }
        self
    }

    /// Set the column from where to load the weight nodes.
    ///
    /// # Arguments
    ///
    /// * weights_column_number: Option<usize>,
    ///     Boolean value representing if we should consider the weights.
    ///
    pub fn set_weights_column_number(mut self, weights_column_number: Option<usize>) -> EdgeFileParameters {
        if let Some(v) = weights_column_number {
            self.weights_column_number = v;
        }
        self
    }

    /// Set the column from where to load the edge_type nodes.
    ///
    /// # Arguments
    ///
    /// * edge_types_column_number: Option<usize>,
    ///     Column from where to load the edge types.
    ///
    pub fn set_edge_types_column_number(mut self, edge_types_column_number: Option<usize>) -> EdgeFileParameters {
        if let Some(v) = edge_types_column_number {
            self.edge_types_column_number = v;
        }
        self
    }

    /// Set the ignore_self_loops.
    ///
    /// # Arguments
    ///
    /// * ignore_self_loops: Option<bool>,
    ///     Wethever to ignore self loops.
    ///
    pub fn set_ignore_self_loops(mut self, ignore_self_loops: Option<bool>) -> EdgeFileParameters {
        if let Some(v) = ignore_self_loops {
            self.ignore_self_loops = v;
        }
        self
    }
}
