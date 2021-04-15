use super::*;

/// # Validators
/// The naming convention we follow is `validate_X`.
impl Graph {
    /// Validates provided node ID.
    ///
    /// # Arguments
    /// * node_id: NodeT - node ID to validate.
    ///
    /// # Example
    /// In order to validate a given node ID, you can use the following:
    ///
    /// ```rust
    /// # let graph = graph::test_utilities::load_ppi(true, true, true, true, false, false).unwrap();
    /// assert!(graph.validate_node_id(0).is_ok());
    /// assert!(graph.validate_node_id(100000000).is_err());
    /// ```
    pub fn validate_node_id(&self, node_id: NodeT) -> Result<NodeT, String> {
        if node_id >= self.get_nodes_number() {
            return Err(format!(
                "The given node id ({}) is higher than the number of nodes within the graph ({}).",
                node_id,
                self.get_nodes_number()
            ));
        }
        Ok(node_id)
    }

    /// Validates provided edge ID.
    ///
    /// # Arguments
    /// * edge_id: EdgeT - Edge ID to validate.
    ///
    /// # Example
    /// In order to validate a given edge ID, you can use the following:
    ///
    /// ```rust
    /// # let graph = graph::test_utilities::load_ppi(true, true, true, true, false, false).unwrap();
    /// assert!(graph.validate_edge_id(0).is_ok());
    /// assert!(graph.validate_edge_id(10000000000).is_err());
    /// ```
    pub fn validate_edge_id(&self, edge_id: EdgeT) -> Result<EdgeT, String> {
        if edge_id >= self.get_directed_edges_number() {
            return Err(format!(
                "The given edge id ({}) is higher than the number of edges within the graph ({}).",
                edge_id,
                self.get_directed_edges_number()
            ));
        }
        Ok(edge_id)
    }

    /// Raises an error if the graph does not have node types.
    ///
    /// # Example
    /// In order to validate a graph instance, you can use:
    ///
    /// ```rust
    /// # let graph_with_node_types = graph::test_utilities::load_ppi(true, true, true, true, false, false).unwrap();
    /// # let graph_without_node_types = graph::test_utilities::load_ppi(false, true, true, true, false, false).unwrap();
    /// assert!(graph_with_node_types.must_have_node_types().is_ok());
    /// assert!(graph_without_node_types.must_have_node_types().is_err());
    /// ```
    pub fn must_have_node_types(&self) -> Result<(), String> {
        if !self.has_node_types() {
            return Err("The current graph instance does not have node types.".to_string());
        }
        Ok(())
    }

    /// Raises an error if the graph does not have edge types.
    ///
    /// # Example
    /// In order to validate a graph instance, you can use:
    ///
    /// ```rust
    /// # let graph_with_edge_types = graph::test_utilities::load_ppi(false, true, true, true, false, false).unwrap();
    /// # let graph_without_edge_types = graph::test_utilities::load_ppi(false, false, true, true, false, false).unwrap();
    /// assert!(graph_with_edge_types.must_have_edge_types().is_ok());
    /// assert!(graph_without_edge_types.must_have_edge_types().is_err());
    /// ```
    pub fn must_have_edge_types(&self) -> Result<(), String> {
        if !self.has_edge_types() {
            return Err("The current graph instance does not have edge types.".to_string());
        }
        Ok(())
    }

    /// Raises an error if the graph does not have weights.
    ///
    /// # Example
    /// In order to validate a graph instance, you can use:
    ///
    /// ```rust
    /// # let graph_with_weights = graph::test_utilities::load_ppi(false, false, true, true, false, false).unwrap();
    /// # let graph_without_weights = graph::test_utilities::load_ppi(false, false, false, true, false, false).unwrap();
    /// assert!(graph_with_weights.must_have_weights().is_ok());
    /// assert!(graph_without_weights.must_have_weights().is_err());
    /// ```
    pub fn must_have_weights(&self) -> Result<(), String> {
        if !self.has_weights() {
            return Err("The current graph instance does not have weights.".to_string());
        }
        Ok(())
    }
}
