use super::*;

/// # Validators
/// The naming convention we follow is:
/// * `/validate_(.+)/`
/// * `/must_have_(.+)/`
/// * `/must_be_(.+)/`
/// * `/must_not_be_(.+)/`
impl Graph {
    /// Validates provided node ID.
    ///
    /// # Arguments
    /// * `node_id`: NodeT - node ID to validate.
    ///
    /// # Example
    /// In order to validate a given node ID, you can use the following:
    ///
    /// ```rust
    /// # let graph = graph::test_utilities::load_ppi(true, true, true, true, false, false);
    /// assert!(graph.validate_node_id(0).is_ok());
    /// assert!(graph.validate_node_id(100000000).is_err());
    /// ```
    ///
    /// # Raises
    /// * If the given node ID does not exists in the graph.
    pub fn validate_node_id(&self, node_id: NodeT) -> Result<NodeT> {
        if node_id >= self.get_nodes_number() {
            return Err(format!(
                "The given node id ({}) is higher than the number of nodes within the graph ({}).",
                node_id,
                self.get_nodes_number()
            ));
        }
        Ok(node_id)
    }

    /// Validates all provided node IDs.
    ///
    /// # Arguments
    /// * `node_ids`: Vec<NodeT> - node IDs to validate.
    ///
    /// # Example
    /// In order to validate the given node IDs, you can use the following:
    ///
    /// ```rust
    /// # let graph = graph::test_utilities::load_ppi(true, true, true, true, false, false);
    /// assert!(graph.validate_node_ids(vec![0, 1, 2]).is_ok());
    /// assert!(graph.validate_node_ids(vec![100000000, u32::MAX]).is_err());
    /// ```
    ///
    /// # Raises
    /// * If any of the given node ID does not exists in the graph.
    pub fn validate_node_ids(&self, node_ids: Vec<NodeT>) -> Result<Vec<NodeT>> {
        node_ids
            .into_iter()
            .map(|node_id| self.validate_node_id(node_id))
            .collect()
    }

    /// Validates provided edge ID.
    ///
    /// # Arguments
    /// * `edge_id`: EdgeT - Edge ID to validate.
    ///
    /// # Example
    /// In order to validate a given edge ID, you can use the following:
    ///
    /// ```rust
    /// # let graph = graph::test_utilities::load_ppi(true, true, true, true, false, false);
    /// assert!(graph.validate_edge_id(0).is_ok());
    /// assert!(graph.validate_edge_id(10000000000).is_err());
    /// ```
    ///
    /// # Raises
    /// * If the given edge ID does not exists in the graph.
    pub fn validate_edge_id(&self, edge_id: EdgeT) -> Result<EdgeT> {
        if edge_id >= self.get_directed_edges_number() {
            return Err(format!(
                "The given edge id ({}) is higher than the number of edges within the graph ({}).",
                edge_id,
                self.get_directed_edges_number()
            ));
        }
        Ok(edge_id)
    }

    /// Validates provided edge IDs.
    ///
    /// # Arguments
    /// * `edge_ids`: Vec<EdgeT> - Edge IDs to validate.
    ///
    /// # Example
    /// In order to validate a given edge ID, you can use the following:
    ///
    /// ```rust
    /// # let graph = graph::test_utilities::load_ppi(true, true, true, true, false, false);
    /// assert!(graph.validate_edge_ids(vec![0, 1, 2]).is_ok());
    /// assert!(graph.validate_edge_ids(vec![10000000000, u64::MAX]).is_err());
    /// ```
    ///
    /// # Raises
    /// * If any of the given edge ID does not exists in the graph.
    pub fn validate_edge_ids(&self, edge_ids: Vec<EdgeT>) -> Result<Vec<EdgeT>> {
        edge_ids
            .into_iter()
            .map(|edge_id| self.validate_edge_id(edge_id))
            .collect()
    }

    /// Raises an error if the graph contains unknown node types.
    ///
    /// # Raises
    /// * If the graph does not contain node types.
    /// * If the graph contains unknown node types.
    pub fn must_not_contain_unknown_node_types(&self) -> Result<()> {
        if self.has_unknown_node_types()? {
            return Err("The graph contains unknown node types.".to_string());
        }
        Ok(())
    }

    /// Raises an error if the graph contains unknown edge types.
    ///
    /// # Raises
    /// * If the graph does not contain edge types.
    /// * If the graph contains unknown edge types.
    pub fn must_not_contain_unknown_edge_types(&self) -> Result<()> {
        if self.has_unknown_edge_types()? {
            return Err("The graph contains unknown edge types.".to_string());
        }
        Ok(())
    }

    /// Validates provided node type ID.
    ///
    /// # Arguments
    /// * `node_type_id`: Option<NodeTypeT> - Node type ID to validate.
    ///
    /// # Example
    /// In order to validate a given node type ID, you can use the following:
    ///
    /// ```rust
    /// # let graph = graph::test_utilities::load_ppi(true, true, true, true, false, false);
    /// assert!(graph.validate_node_type_id(Some(0)).is_ok());
    /// assert!(graph.validate_node_type_id(Some(1000)).is_err());
    /// ```
    ///
    /// # Raises
    /// * If the given node type ID does not exists in the graph.
    pub fn validate_node_type_id(
        &self,
        node_type_id: Option<NodeTypeT>,
    ) -> Result<Option<NodeTypeT>> {
        self.get_node_types_number().and_then(|node_types_number| {
            node_type_id.map_or_else( || if !self.has_unknown_node_types()?{
                Err(
                    "An unknown node type was given but the graph does not contain unknown node types.".to_string()
                )
            } else {
                Ok(None)
            }, |nti| {
                if node_types_number <= nti {
                    Err(format!(
                        "Given node type ID {} is bigger than number of node types in the graph {}.",
                        nti,
                        node_types_number
                    ))
                } else {
                    Ok(Some(nti))
                }
            })
        })
    }

    /// Validates provided node type IDs.
    ///
    /// # Arguments
    /// * `node_type_ids`: Vec<Option<NodeTypeT>> - Vector of node type IDs to validate.
    ///
    /// # Raises
    /// * If there are no node types in the graph.
    pub fn validate_node_type_ids(
        &self,
        node_type_ids: Vec<Option<NodeTypeT>>,
    ) -> Result<Vec<Option<NodeTypeT>>> {
        self.must_have_node_types()?;
        node_type_ids
            .into_iter()
            .map(|node_type| self.validate_node_type_id(node_type))
            .collect()
    }

    /// Validates provided edge type ID.
    ///
    /// # Arguments
    /// * `edge_type_id`: Option<EdgeTypeT> - edge type ID to validate.
    ///
    /// # Example
    /// In order to validate a given edge type ID, you can use the following:
    ///
    /// ```rust
    /// # let graph = graph::test_utilities::load_ppi(true, true, true, true, false, false);
    /// assert!(graph.validate_edge_type_id(Some(0)).is_ok());
    /// assert!(graph.validate_edge_type_id(Some(1000)).is_err());
    /// ```
    ///
    /// # Raises
    /// * If the given edge type ID does not exists in the graph.
    pub fn validate_edge_type_id(
        &self,
        edge_type_id: Option<EdgeTypeT>,
    ) -> Result<Option<EdgeTypeT>> {
        self.get_edge_types_number().and_then(|edge_types_number| {
            edge_type_id.map_or_else( || if !self.has_unknown_edge_types()?{
                Err(
                    "An unknown edge type was given but the graph does not contain unknown edge types.".to_string()
                )
            } else {
                Ok(None)
            }, |eti| {
                if edge_types_number <= eti {
                    Err(format!(
                        "Given edge type ID {} is bigger than number of edge types in the graph {}.",
                        eti,
                        edge_types_number
                    ))
                } else {
                    Ok(Some(eti))
                }
            })
        })
    }

    /// Validates provided edge type IDs.
    ///
    /// # Arguments
    /// * `edge_type_ids`: Vec<Option<EdgeTypeT>> - Vector of edge type IDs to validate.
    ///
    /// # Raises
    /// * If there are no edge types in the graph.
    pub fn validate_edge_type_ids(
        &self,
        edge_type_ids: Vec<Option<EdgeTypeT>>,
    ) -> Result<Vec<Option<EdgeTypeT>>> {
        self.must_have_edge_types()?;
        edge_type_ids
            .into_iter()
            .map(|edge_type| self.validate_edge_type_id(edge_type))
            .collect()
    }

    #[no_binding]
    /// Raises an error if the graph does not have node types.
    ///
    /// # Example
    /// In order to validate a graph instance, you can use:
    ///
    /// ```rust
    /// # let graph_with_node_types = graph::test_utilities::load_ppi(true, true, true, true, false, false);
    /// # let graph_without_node_types = graph::test_utilities::load_ppi(false, true, true, true, false, false);
    /// assert!(graph_with_node_types.must_have_node_types().is_ok());
    /// assert!(graph_without_node_types.must_have_node_types().is_err());
    /// ```
    ///
    /// # Raises
    /// * If the graph does not contain node types.
    pub fn must_have_node_types(&self) -> Result<&NodeTypeVocabulary> {
        if !self.has_node_types() {
            return Err("The current graph instance does not have node types.".to_string());
        }
        Ok(self.node_types.as_ref().unwrap())
    }

    #[no_binding]
    /// Raises an error if the graph does not have edge types.
    ///
    /// # Example
    /// In order to validate a graph instance, you can use:
    ///
    /// ```rust
    /// # let graph_with_edge_types = graph::test_utilities::load_ppi(false, true, true, true, false, false);
    /// # let graph_without_edge_types = graph::test_utilities::load_ppi(false, false, true, true, false, false);
    /// assert!(graph_with_edge_types.must_have_edge_types().is_ok());
    /// assert!(graph_without_edge_types.must_have_edge_types().is_err());
    /// ```
    ///
    /// # Raises
    /// * If there are no edge types in the graph.
    pub fn must_have_edge_types(&self) -> Result<&EdgeTypeVocabulary> {
        if !self.has_edge_types() {
            return Err("The current graph instance does not have edge types.".to_string());
        }
        Ok(self.edge_types.as_ref().unwrap())
    }

    /// Raises an error if the graph does not have edge types.
    ///
    /// # Example
    /// In order to validate a graph instance, you can use:
    ///
    /// ```rust
    /// # let undirecte_graph = graph::test_utilities::load_ppi(false, false, false, false, false, false);
    /// # let directed_graph = graph::test_utilities::load_ppi(false, false, true, true, false, false);
    /// assert!(undirecte_graph.must_be_undirected().is_ok());
    /// assert!(directed_graph.must_be_undirected().is_err());
    /// ```
    ///
    /// # Raises
    /// * If the graph is directed.
    pub fn must_be_undirected(&self) -> Result<()> {
        if self.is_directed() {
            return Err("The current graph instance is not undirected.".to_string());
        }
        Ok(())
    }

    /// Raises an error if the graph does not have edge types.
    ///
    /// # Example
    /// In order to validate a graph instance, you can use:
    ///
    /// ```rust
    /// # let multigraph = graph::test_utilities::load_ppi(false, true, false, false, false, false);
    /// # let homogeneous = graph::test_utilities::load_ppi(false, false, false, false, false, false);
    /// assert!(multigraph.must_be_multigraph().is_ok());
    /// assert!(homogeneous.must_be_multigraph().is_err());
    /// ```
    ///
    /// # Raises
    /// * If the graph is not a multigraph.
    pub fn must_be_multigraph(&self) -> Result<()> {
        if !self.is_multigraph() {
            return Err(
                "The current graph instance must be a multigraph to run this method.".to_string(),
            );
        }
        Ok(())
    }

    /// Raises an error if the graph does not have edge types.
    ///
    /// # Example
    /// In order to validate a graph instance, you can use:
    ///
    /// ```rust
    /// # let multigraph = graph::test_utilities::load_ppi(false, true, false, false, false, false);
    /// # let homogeneous = graph::test_utilities::load_ppi(false, false, false, false, false, false);
    /// assert!(multigraph.must_not_be_multigraph().is_err());
    /// assert!(homogeneous.must_not_be_multigraph().is_ok());
    /// ```
    ///
    /// # Raises
    /// * If the graph is a multigraph.
    pub fn must_not_be_multigraph(&self) -> Result<()> {
        if self.is_multigraph() {
            return Err(
                "The current graph instance must not be a multigraph to run this method."
                    .to_string(),
            );
        }
        Ok(())
    }

    #[no_binding]
    /// Raises an error if the graph does not have weights.
    ///
    /// # Example
    /// In order to validate a graph instance, you can use:
    ///
    /// ```rust
    /// # let graph_with_weights = graph::test_utilities::load_ppi(false, false, true, true, false, false);
    /// # let graph_without_weights = graph::test_utilities::load_ppi(false, false, false, true, false, false);
    /// assert!(graph_with_weights.must_have_edge_weights().is_ok());
    /// assert!(graph_without_weights.must_have_edge_weights().is_err());
    /// ```
    ///
    /// # Raises
    /// * If the graph does not have edge weights.
    pub fn must_have_edge_weights(&self) -> Result<&Vec<WeightT>> {
        if !self.has_edge_weights() {
            return Err("The current graph instance does not have weights.".to_string());
        }
        Ok(self.weights.as_ref().unwrap())
    }

    #[no_binding]
    /// Raises an error if the graph does not have known node types.
    ///
    /// # Raises
    /// * If the graph does not contain any known node types.
    pub fn must_have_known_node_types(&self) -> Result<()> {
        if !self.has_known_node_types()? {
            return Err("The current graph instance does contain any known node type.".to_string());
        }
        Ok(())
    }

    #[no_binding]
    /// Raises an error if the graph does not have unknown node types.
    ///
    /// # Raises
    /// * If the graph does not contain any unknown node types.
    pub fn must_have_unknown_node_types(&self) -> Result<()> {
        if !self.has_unknown_node_types()? {
            return Err(concat!(
                "The current graph instance does contain any unknown node type.\n",
                "Possibly you have forgotten to execute a node-label holdout?"
            )
            .to_string());
        }
        Ok(())
    }

    #[no_binding]
    /// Raises an error if the graph does not have known edge types.
    ///
    /// # Raises
    /// * If the graph does not contain any known edge types.
    pub fn must_have_known_edge_types(&self) -> Result<()> {
        if !self.has_known_edge_types()? {
            return Err("The current graph instance does contain any known edge type.".to_string());
        }
        Ok(())
    }

    #[no_binding]
    /// Raises an error if the graph does not have unknown edge types.
    ///
    /// # Raises
    /// * If the graph does not contain any unknown edge types.
    pub fn must_have_unknown_edge_types(&self) -> Result<()> {
        if !self.has_unknown_edge_types()? {
            return Err(concat!(
                "The current graph instance does contain any unknown edge type.\n",
                "Possibly you have forgotten to execute a edge-label holdout?"
            )
            .to_string());
        }
        Ok(())
    }

    #[no_binding]
    /// Raises an error if the graph does not have weights.
    ///
    /// # Raises
    /// * If the graph does not have edge weights.
    pub fn must_have_edge_weights_representing_probabilities(&self) -> Result<&Vec<WeightT>> {
        if !self.has_edge_weights_representing_probabilities()? {
            return Err(
                "The current graph instance does not contain weights representing probabilities."
                    .to_string(),
            );
        }
        Ok(self.weights.as_ref().unwrap())
    }

    #[no_binding]
    /// Raises an error if the graph has negative edge weights.
    ///
    /// # Example
    /// In order to validate a graph instance, you can use:
    ///
    /// ```rust
    /// # let graph_with_weights = graph::test_utilities::load_ppi(false, false, true, false, false, false);
    /// # let graph_with_negative_weights = graph_with_weights.get_laplacian_transformed_graph();
    /// assert!(graph_with_weights.must_have_positive_edge_weights().is_ok());
    /// assert!(graph_with_negative_weights.must_have_positive_edge_weights().is_err());
    /// ```
    ///
    /// # Raises
    /// * If the graph does not contain edge weights.
    /// * If the graph contains negative edge weights.
    pub fn must_have_positive_edge_weights(&self) -> Result<&Vec<WeightT>> {
        if self.has_negative_edge_weights()? {
            return Err("The current graph instance contains negative edge weights.".to_string());
        }
        Ok(self.weights.as_ref().unwrap())
    }

    /// Raises an error if the graph contains zero weighted degree.
    ///
    /// # Raises
    /// * If the graph does not have edges.
    pub fn must_not_contain_weighted_singleton_nodes(&self) -> Result<()> {
        if self.has_weighted_singleton_nodes()? {
            return Err(concat!(
                "The current graph instance contains weighted ",
                "singleton nodes, that is nodes with weighted degree zero."
            )
            .to_string());
        }
        Ok(())
    }

    /// Raises an error if the graph has a maximal weighted
    ///
    /// # Example
    /// In order to validate a graph instance, you can use:
    ///
    /// ```rust
    /// # let graph_with_edges = graph::test_utilities::load_ppi(false, false, true, true, false, false);
    /// # let graph_without_edges = graph::build_empty_graph(false, "Empty graph").unwrap();
    /// assert!(graph_with_edges.must_have_edges().is_ok());
    /// assert!(graph_without_edges.must_have_edges().is_err());
    /// ```
    ///
    /// # Raises
    /// * If the graph does not have edges.
    pub fn must_have_edges(&self) -> Result<()> {
        if !self.has_edges() {
            return Err("The current graph instance does not have any edge.".to_string());
        }
        Ok(())
    }

    /// Raises an error if the graph does not have any node.
    ///
    /// # Example
    /// In order to validate a graph instance, you can use:
    ///
    /// ```rust
    /// # let graph_with_nodes = graph::test_utilities::load_ppi(false, false, true, true, false, false);
    /// # let graph_without_nodes = graph::build_empty_graph(false, "Empty graph").unwrap();
    /// assert!(graph_with_nodes.must_have_nodes().is_ok());
    /// assert!(graph_without_nodes.must_have_nodes().is_err());
    /// ```
    ///
    /// # Raises
    /// * If the graph does not have nodes.
    pub fn must_have_nodes(&self) -> Result<()> {
        if !self.has_nodes() {
            return Err("The current graph instance does not have any node.".to_string());
        }
        Ok(())
    }

    /// Raises an error if the graph is not connected.
    ///
    /// # Raises
    /// * If the graph is not connected.
    pub fn must_be_connected(&self) -> Result<()> {
        if !self.is_connected(None) {
            return Err("The current graph instance is not connected.".to_string());
        }
        Ok(())
    }
}
