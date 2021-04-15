use super::*;
use rayon::prelude::*;

/// # Queries
/// The naming convention we follow is `get_X_from_Y`.
impl Graph {
    #[inline(always)]
    /// Returns node IDs corresponding to given edge ID.
    ///
    /// The method will panic if the given edge ID does not exists in the
    /// current graph instance.
    ///
    /// # Arguments
    /// `edge_id`: EdgeT - The edge ID whose source and destination node IDs are to e retrieved.
    ///
    /// # Example
    /// To retrieve the source and destination node IDs of a given edge ID you can use the following:
    ///
    /// ```rust
    /// # let graph = graph::test_utilities::load_ppi(true, true, true, true, false, false).unwrap();
    /// let edge_id = 0;
    /// let (src, dst) = graph.get_unchecked_node_ids_from_edge_id(edge_id);
    /// println!("The edge with ID {} has source node ID {} and destination node ID {}.", edge_id, src, dst);
    /// ```
    pub fn get_unchecked_node_ids_from_edge_id(&self, edge_id: EdgeT) -> (NodeT, NodeT) {
        if let (Some(sources), Some(destinations)) = (&self.sources, &self.destinations) {
            return (sources[edge_id as usize], destinations[edge_id as usize]);
        }
        self.decode_edge(self.edges.unchecked_select(edge_id))
    }

    #[inline(always)]
    /// Returns node IDs corresponding to given edge ID.
    ///
    /// # Arguments
    /// `edge_id`: EdgeT - The edge ID whose source and destination node IDs are to e retrieved.
    ///
    /// # Example
    /// To retrieve the source and destination node IDs of a given edge ID you can use the following:
    ///
    /// ```rust
    /// # let graph = graph::test_utilities::load_ppi(true, true, true, true, false, false).unwrap();
    /// assert!(graph.get_node_ids_and_type_from_edge_id(0).is_ok());
    /// assert!(graph.get_node_ids_and_type_from_edge_id(10000000000).is_err());
    /// ```
    pub fn get_node_ids_from_edge_id(&self, edge_id: EdgeT) -> Result<(NodeT, NodeT), String> {
        self.validate_edge_id(edge_id)
            .map(|edge_id| self.get_unchecked_node_ids_from_edge_id(edge_id))
    }

    #[inline(always)]
    /// Returns edge ID corresponding to given source and destination node IDs.
    ///
    /// The method will panic if the given source and destination node IDs do
    /// not correspond to an edge in this graph instance.
    ///
    /// # Arguments
    /// `src`: NodeT - The source node ID.
    /// `dst`: NodeT - The destination node ID.
    ///
    /// # Example
    /// To retrieve the edge ID curresponding to the given source and destination node IDs you can use the following:
    ///
    /// ```rust
    /// # let graph = graph::test_utilities::load_ppi(false, true, true, true, false, false).unwrap();
    /// let src = 0;
    /// let dst = 1;
    /// let edge_id = graph.get_unchecked_edge_id_from_node_ids(src, dst);
    /// println!("The source node ID {} and destination node ID {} corrrespond to the edge with ID {}.", src, dst, edge_id);
    /// ```
    pub fn get_unchecked_edge_id_from_node_ids(&self, src: NodeT, dst: NodeT) -> EdgeT {
        self.edges.unchecked_rank(self.encode_edge(src, dst)) as EdgeT
    }

    #[inline(always)]
    /// Returns edge ID corresponding to given source and destination node IDs.
    ///
    /// # Arguments
    /// `src`: NodeT - The source node ID.
    /// `dst`: NodeT - The destination node ID.
    ///
    /// # Example
    /// To retrieve the edge ID curresponding to the given source and destination node IDs you can use the following:
    ///
    /// ```rust
    /// # let graph = graph::test_utilities::load_ppi(false, true, true, true, false, false).unwrap();
    /// assert!(graph.get_edge_id_from_node_ids(0, 1).is_ok());
    /// assert!(graph.get_edge_id_from_node_ids(0, 100000000).is_err());
    /// ```
    pub fn get_edge_id_from_node_ids(&self, src: NodeT, dst: NodeT) -> Result<EdgeT, String> {
        match self
            .edges
            .rank(self.encode_edge(src, dst))
            .map(|value| value as EdgeT) {
                Some(edge_id) => Ok(edge_id),
                None => Err(format!("The edge composed by the source node {} and destination node {} does not exist in this graph.", src, dst))
            }
    }

    #[inline(always)]
    pub(crate) fn get_unique_source(&self, source_id: NodeT) -> NodeT {
        self.unique_sources
            .as_ref()
            .map_or(source_id, |x| x.unchecked_select(source_id as u64) as NodeT)
    }

    /// Return the src, dst, edge type of a given edge ID.
    ///
    /// This method will raise a panic when an improper configuration is used.
    ///
    /// # Arguments
    /// `edge_id`: EdgeT  - The edge ID whose source, destination and edge type are to be retrieved.
    ///
    /// # Example
    /// In order to retrieve a given edge ID informations, you can use the following:
    ///
    /// ```rust
    /// # let graph = graph::test_utilities::load_ppi(true, true, true, true, false, false).unwrap();
    /// let edge_id = 0;
    /// let (src, dst, edge_type) = graph.get_unchecked_node_ids_and_type_from_edge_id(edge_id);
    /// println!("The edge with ID {} has source node ID {}, destination node ID {} and edge type ID {:?}", edge_id, src, dst, edge_type);
    /// ```
    pub fn get_unchecked_node_ids_and_type_from_edge_id(
        &self,
        edge_id: EdgeT,
    ) -> (NodeT, NodeT, Option<EdgeTypeT>) {
        let (src, dst) = self.get_unchecked_node_ids_from_edge_id(edge_id);
        (src, dst, self.get_unchecked_edge_type_from_edge_id(edge_id))
    }

    /// Return the src, dst, edge type of a given edge ID.
    ///
    /// # Arguments
    /// `edge_id`: EdgeT  - The edge ID whose source, destination and edge type are to be retrieved.
    ///
    /// # Example
    /// In order to retrieve a given edge ID informations, you can use the following:
    ///
    /// ```rust
    /// # let graph = graph::test_utilities::load_ppi(true, true, true, true, false, false).unwrap();
    /// assert!(graph.get_node_ids_and_type_from_edge_id(0).is_ok());
    /// assert!(graph.get_node_ids_and_type_from_edge_id(10000000000).is_err());
    /// ```
    pub fn get_node_ids_and_type_from_edge_id(
        &self,
        edge_id: EdgeT,
    ) -> Result<(NodeT, NodeT, Option<EdgeTypeT>), String> {
        if edge_id >= self.get_directed_edges_number() {
            Err(format!(
                "The given edge id ({}) is higher than the edges of the graph ({}).",
                edge_id,
                self.get_directed_edges_number()
            ))
        } else {
            Ok(self.get_unchecked_node_ids_and_type_from_edge_id(edge_id))
        }
    }

    /// Return the src, dst, edge type and weight of a given edge ID.
    ///
    /// This method will raise a panic when an improper configuration is used.
    ///
    /// # Arguments
    /// `edge_id`: EdgeT  - The edge ID whose source, destination, edge type and weight are to be retrieved.
    ///
    /// # Example
    /// In order to retrieve a given edge ID informations, you can use the following:
    ///
    /// ```rust
    /// # let graph = graph::test_utilities::load_ppi(true, true, true, true, false, false).unwrap();
    /// let edge_id = 0;
    /// let (src, dst, edge_type, weight) = graph.get_unchecked_node_ids_type_and_weight_from_edge_id(edge_id);
    /// println!("The edge with ID {} has source node ID {}, destination node ID {}, edge type ID {:?} and weight {:?}.", edge_id, src, dst, edge_type, weight);
    /// ```
    pub fn get_unchecked_node_ids_type_and_weight_from_edge_id(
        &self,
        edge_id: EdgeT,
    ) -> (NodeT, NodeT, Option<EdgeTypeT>, Option<WeightT>) {
        let (src, dst, edge_type) = self.get_unchecked_node_ids_and_type_from_edge_id(edge_id);
        (
            src,
            dst,
            edge_type,
            self.get_unchecked_weight_from_edge_id(edge_id),
        )
    }

    /// Return vector with top k central node Ids.
    ///
    /// # Arguments
    ///
    /// * k: NodeT - Number of central nodes to extract.
    pub fn get_top_k_central_nodes_ids(&self, k: NodeT) -> Vec<NodeT> {
        let mut nodes_degrees: Vec<(NodeT, NodeT)> = (0..self.get_nodes_number())
            .map(|node_id| (self.get_node_degree_from_node_id(node_id).unwrap(), node_id))
            .collect();
        nodes_degrees.par_sort_unstable();
        nodes_degrees.reverse();
        nodes_degrees[0..k as usize]
            .iter()
            .map(|(_, node_id)| *node_id)
            .collect()
    }

    /// Return vector with top k central node names.
    ///
    /// # Arguments
    ///
    /// * k: NodeT - Number of central nodes to extract.
    pub fn get_top_k_central_node_names(&self, k: NodeT) -> Vec<String> {
        self.get_top_k_central_nodes_ids(k)
            .into_iter()
            .map(|node_id| self.get_node_name_from_node_id(node_id).unwrap())
            .collect()
    }

    /// Returns node type of given node.
    ///
    /// # Arguments
    ///
    /// * `node_id`: NodeT - node whose node type is to be returned.
    ///
    /// # Examples
    /// ```rust
    /// # let graph = graph::test_utilities::load_ppi(true, true, true, true, false, false).unwrap();
    /// println!("The node type id of node {} is {:?}", 0, graph.get_node_type_id_from_node_id(0));
    /// ```
    ///
    pub fn get_node_type_id_from_node_id(
        &self,
        node_id: NodeT,
    ) -> Result<Option<Vec<NodeTypeT>>, String> {
        if let Some(nt) = &self.node_types {
            return if node_id <= nt.ids.len() as NodeT {
                Ok(nt.ids[node_id as usize].clone())
            } else {
                Err(format!(
                    "The node_index {} is too big for the node_types vector which has len {}",
                    node_id,
                    nt.ids.len()
                ))
            };
        }

        Err(String::from(
            "Node types are not defined for current graph instance.",
        ))
    }

    /// Returns edge type of given edge.
    ///
    /// # Arguments
    ///
    /// * edge_id: EdgeT - edge whose edge type is to be returned.
    ///
    /// # Examples
    /// ```rust
    /// # let graph = graph::test_utilities::load_ppi(true, true, true, true, false, false).unwrap();
    /// println!("The edge type id of edge {} is {:?}", 0, graph.get_edge_type_id_from_edge_id(0));
    /// ```
    pub fn get_edge_type_id_from_edge_id(&self, edge_id: EdgeT) -> Result<Option<EdgeTypeT>, String> {
        if let Some(et) = &self.edge_types {
            return if edge_id <= et.ids.len() as EdgeT {
                Ok(self.get_unchecked_edge_type_from_edge_id(edge_id))
            } else {
                Err(format!(
                    "The edge_index {} is too big for the edge_types vector which has len {}",
                    edge_id,
                    et.ids.len()
                ))
            };
        }
        Err(String::from(
            "Edge types are not defined for current graph instance.",
        ))
    }

    /// Returns result of option with the node type of the given node id.
    ///
    /// # Arguments
    /// `node_id`: NodeT - The node ID whose node types are to be returned.
    pub fn get_node_type_name_from_node_id(
        &self,
        node_id: NodeT,
    ) -> Result<Option<Vec<String>>, String> {
        if self.node_types.is_some() {
            Ok(self
                .get_node_type_id_from_node_id(node_id)?
                .and_then(|node_type_ids| {
                    // This unwrap cannot fail because it is surely a vector
                    // of node type IDs from the current graph instance.
                    self.get_node_type_names_from_node_type_ids(node_type_ids)
                        .ok()
                }))
        } else {
            Err("Node types not available for the current graph instance.".to_string())
        }
    }

    /// Returns option with the edge type of the given edge id.
    ///
    /// # Arguments
    /// `edge_id`: EdgeT - The edge ID whose edge type is to be returned.
    pub fn get_edge_type_name_from_edge_id(&self, edge_id: EdgeT) -> Result<Option<String>, String> {
        self.get_edge_type_id_from_edge_id(edge_id)?
            .map_or(Ok(None), |x| {
                Ok(Some(self.get_edge_type_name_from_edge_type_id(x)?))
            })
    }

    /// Return edge type name of given edge type.
    ///
    /// # Arguments
    /// * edge_type_id: EdgeTypeT - Id of the edge type.
    pub fn get_edge_type_name_from_edge_type_id(
        &self,
        edge_type_id: EdgeTypeT,
    ) -> Result<String, String> {
        self.edge_types.as_ref().map_or(
            Err("Edge types not available for the current graph instance.".to_string()),
            |ets| ets.translate(edge_type_id),
        )
    }

    /// Returns weight of the given edge id.
    ///
    /// # Arguments
    /// * `edge_id`: EdgeT - The edge ID whose weight is to be returned.
    ///
    /// # Examples
    /// To get the weight of a given `edge_id` you can run:
    /// ```rust
    /// # let weighted_graph = graph::test_utilities::load_ppi(true, true, true, true, false, false).unwrap();
    /// # let unweighted_graph = graph::test_utilities::load_ppi(true, true, false, true, false, false).unwrap();
    /// let edge_id = 0;
    /// let unexistent_edge_id = 123456789;
    /// assert!(weighted_graph.get_weight_from_edge_id(edge_id).is_ok());
    /// assert!(weighted_graph.get_weight_from_edge_id(unexistent_edge_id).is_err());
    /// assert!(unweighted_graph.get_weight_from_edge_id(edge_id).is_err());
    /// ```
    pub fn get_weight_from_edge_id(&self, edge_id: EdgeT) -> Result<WeightT, String> {
        self.weights.as_ref().map_or(
            Err("The current graph instance does not have weights!".to_string()),
            |weights| weights.get(edge_id as usize).map_or(
                Err(format!(
                    "The given edge_id {} is higher than the number of available directed edges {}.",
                    edge_id,
                    self.get_directed_edges_number()
                )),
                |value| Ok(*value)
            )
        )
    }

    /// Returns weight of the given node ids.
    ///
    /// # Arguments
    /// * `src`: NodeT - The node ID of the source node.
    /// * `dst`: NodeT - The node ID of the destination node.
    ///
    /// # Examples
    /// To get the weight of a given `src` and `dst` you can run:
    /// ```rust
    /// # let weighted_graph = graph::test_utilities::load_ppi(false, true, true, true, false, false).unwrap();
    /// let src = 0;
    /// let dst = 1;
    /// assert!(weighted_graph.get_weight_from_node_ids(src, dst).is_ok());
    /// ```
    pub fn get_weight_from_node_ids(&self, src: NodeT, dst: NodeT) -> Result<WeightT, String> {
        self.get_weight_from_edge_id(self.get_edge_id_from_node_ids(src, dst)?)
    }

    /// Returns weight of the given node ids and edge type.
    ///
    /// # Arguments
    /// * `src`: NodeT - The node ID of the source node.
    /// * `dst`: NodeT - The node ID of the destination node.
    /// * `edge_type`: Option<EdgeTypeT> - The edge type ID of the edge.
    ///
    /// # Examples
    /// To get the weight of a given `src` and `dst` and `edge_type` you can run:
    /// ```rust
    /// # let weighted_graph = graph::test_utilities::load_ppi(false, true, true, true, false, false).unwrap();
    /// let src = 0;
    /// let dst = 1;
    /// let edge_type = Some(0);
    /// assert!(weighted_graph.get_weight_with_type_from_node_ids(src, dst, edge_type).is_ok());
    /// ```
    pub fn get_weight_with_type_from_node_ids(
        &self,
        src: NodeT,
        dst: NodeT,
        edge_type: Option<EdgeTypeT>,
    ) -> Result<WeightT, String> {
        self.get_weight_from_edge_id(self.get_edge_id_with_type_from_node_ids(src, dst, edge_type)?)
    }

    /// Returns weight of the given node names and edge type.
    ///
    /// # Arguments
    /// * `src`: &str - The node name of the source node.
    /// * `dst`: &str - The node name of the destination node.
    /// * `edge_type`: Option<&String> - The edge type name of the edge.
    ///
    /// # Examples
    /// To get the weight of a given `src` and `dst` and `edge_type` you can run:
    /// ```rust
    /// # let weighted_graph = graph::test_utilities::load_ppi(false, true, true, true, false, false).unwrap();
    /// let src = "ENSP00000000233";
    /// let dst = "ENSP00000432568";
    /// let edge_type = Some("red".to_string());
    /// assert!(weighted_graph.get_weight_with_type_from_node_names(src, dst, edge_type.as_ref()).is_ok());
    /// ```
    pub fn get_weight_with_type_from_node_names(
        &self,
        src: &str,
        dst: &str,
        edge_type: Option<&String>,
    ) -> Result<WeightT, String> {
        self.get_weight_from_edge_id(self.get_edge_id_with_type_from_node_names(src, dst, edge_type)?)
    }

    /// Returns weight of the given node names.
    ///
    /// # Arguments
    /// * `src_name`: &str - The node name of the source node.
    /// * `dst_name`: &str - The node name of the destination node.
    ///
    /// # Examples
    /// To get the weight of a given `src_name` and `dst_name` you can run:
    /// ```rust
    /// # let weighted_graph = graph::test_utilities::load_ppi(false, true, true, true, false, false).unwrap();
    /// let src_name = "ENSP00000000233";
    /// let dst_name = "ENSP00000432568";
    /// assert!(weighted_graph.get_weight_from_node_names(src_name, dst_name).is_ok());
    /// ```
    pub fn get_weight_from_node_names(
        &self,
        src_name: &str,
        dst_name: &str,
    ) -> Result<WeightT, String> {
        self.get_weight_from_edge_id(self.get_edge_id_from_node_names(src_name, dst_name)?)
    }

    /// Returns result with the node name.
    pub fn get_node_name_from_node_id(&self, node_id: NodeT) -> Result<String, String> {
        match node_id < self.get_nodes_number() {
            true => Ok(self.nodes.unchecked_translate(node_id)),
            false => Err(format!(
                "Given node_id {} is greater than number of nodes in the graph ({}).",
                node_id,
                self.get_nodes_number()
            )),
        }
    }

    /// Returns result with the node id.
    pub fn get_node_id_from_node_name(&self, node_name: &str) -> Result<NodeT, String> {
        match self.nodes.get(node_name) {
            Some(node_id) => Ok(*node_id),
            None => Err(format!(
                "Given node name {} is not available in current graph.",
                node_name
            )),
        }
    }

    /// Return node type ID for the given node name if available.
    ///
    /// # Arguments
    ///
    /// * `node_name`: &str - Name of the node.
    ///
    /// # Examples
    /// To get the node type ID for a given node name you can run:
    /// ```rust
    /// # let graph = graph::test_utilities::load_ppi(true, true, true, true, false, false).unwrap();
    /// let node_name = "ENSP00000000233";
    /// println!("The node type ID of node {} is {:?}.", node_name, graph.get_node_type_id_from_node_name(node_name).unwrap());
    /// ```
    pub fn get_node_type_id_from_node_name(
        &self,
        node_name: &str,
    ) -> Result<Option<Vec<NodeTypeT>>, String> {
        self.get_node_type_id_from_node_id(self.get_node_id_from_node_name(node_name)?)
    }

    /// Return node type name for the given node name if available.
    ///
    /// # Arguments
    ///
    /// * `node_name`: &str - Name of the node.
    ///
    /// # Examples
    /// To get the node type name for a given node name you can run:
    /// ```rust
    /// # let graph = graph::test_utilities::load_ppi(true, true, true, true, false, false).unwrap();
    /// let node_name = "ENSP00000000233";
    /// println!("The node type of node {} is {:?}", node_name, graph.get_node_type_name_from_node_name(node_name).unwrap());
    /// ```
    pub fn get_node_type_name_from_node_name(
        &self,
        node_name: &str,
    ) -> Result<Option<Vec<String>>, String> {
        self.get_node_type_name_from_node_id(self.get_node_id_from_node_name(node_name)?)
    }

    /// Return number of edges with given edge type ID.
    ///
    /// If None is given as an edge type ID, the unknown edge type IDs
    /// will be returned.
    ///
    /// # Arguments
    /// edge_type: Option<EdgeTypeT> - The edge type ID to count the edges of.
    ///
    pub fn get_edge_count_from_edge_type_id(
        &self,
        edge_type: Option<EdgeTypeT>,
    ) -> Result<EdgeT, String> {
        if !self.has_edge_types() {
            return Err("Current graph does not have edge types!".to_owned());
        }
        if let Some(et) = &edge_type {
            if self.get_edge_types_number() <= *et {
                return Err(format!(
                    "Given edge type ID {} is bigger than number of edge types in the graph {}.",
                    self.get_edge_types_number(),
                    et
                ));
            }
        }
        Ok(self.get_unchecked_edge_count_from_edge_type_id(edge_type))
    }

    /// Return edge type ID curresponding to given edge type name.
    ///
    /// If None is given as an edge type ID, None is returned.
    ///
    /// # Arguments
    /// edge_type: Option<&str> - The edge type name whose ID is to be returned.
    ///
    pub fn get_edge_type_id_from_edge_type_name(
        &self,
        edge_type_name: Option<&str>,
    ) -> Result<Option<EdgeTypeT>, String> {
        match (&self.edge_types, edge_type_name) {
            (None, _) => Err("Current graph does not have edge types.".to_owned()),
            (Some(_), None) => Ok(None),
            (Some(ets), Some(etn)) => match ets.get(etn) {
                Some(edge_type_id) => Ok(Some(*edge_type_id)),
                None => Err(format!(
                    "Given edge type name {} is not available in current graph.",
                    etn
                )),
            },
        }
    }

    /// Return number of edges with given edge type name.
    ///
    /// If None is given as an edge type name, the unknown edge types
    /// will be returned.
    ///
    /// # Arguments
    /// edge_type: Option<&str> - The edge type name to count the edges of.
    ///
    pub fn get_edge_count_from_edge_type_name(
        &self,
        edge_type: Option<&str>,
    ) -> Result<EdgeT, String> {
        self.get_edge_count_from_edge_type_id(self.get_edge_type_id_from_edge_type_name(edge_type)?)
    }

    /// Return node type ID curresponding to given node type name.
    ///
    /// If None is given as an node type ID, None is returned.
    ///
    /// # Arguments
    /// node_type: Option<&str> - The node type name whose ID is to be returned.
    ///
    pub fn get_node_type_id_from_node_type_name(
        &self,
        node_type_name: &str,
    ) -> Result<NodeTypeT, String> {
        if let Some(ets) = &self.node_types {
            return match ets.get(node_type_name) {
                Some(node_type_id) => Ok(*node_type_id),
                None => Err(format!(
                    "Given node type name {} is not available in current graph.",
                    node_type_name
                )),
            };
        }
        Err("Current graph does not have node types.".to_owned())
    }

    /// Return number of nodes with given node type ID.
    ///
    /// If None is given as an node type ID, the unknown node types
    /// will be returned.
    ///
    /// # Arguments
    /// node_type: Option<NodeTypeT> - The node type ID to count the nodes of.
    ///
    pub fn get_node_count_from_node_type_id(
        &self,
        node_type: Option<NodeTypeT>,
    ) -> Result<NodeT, String> {
        if !self.has_node_types() {
            return Err("Current graph does not have node types!".to_owned());
        }
        if node_type.map_or(false, |nt| self.get_node_types_number() <= nt) {
            return Err(format!(
                "Given node type ID {:?} is bigger than number of node types in the graph {}.",
                node_type,
                self.get_node_types_number()
            ));
        }
        Ok(self.get_unchecked_node_count_from_node_type_id(node_type))
    }

    /// Return number of nodes with given node type name.
    ///
    /// If None is given as an node type name, the unknown node types
    /// will be returned.
    ///
    /// # Arguments
    /// node_type: Option<&str> - The node type name to count the nodes of.
    ///
    pub fn get_node_count_from_node_type_name(
        &self,
        node_type_name: Option<&str>,
    ) -> Result<NodeT, String> {
        self.get_node_count_from_node_type_id(
            node_type_name.map_or(Ok::<_, String>(None), |ntn| {
                Ok(Some(self.get_node_type_id_from_node_type_name(ntn)?))
            })?,
        )
    }

    /// Returns the destination of given edge id without making any boundary check.
    ///
    /// # Arguments
    ///
    /// `edge_id`: EdgeT - The edge ID whose destination is to be retrieved.
    pub(crate) fn get_unchecked_destination_node_id_from_edge_id(&self, edge_id: EdgeT) -> NodeT {
        self.destinations.as_ref().map_or_else(
            || self.get_unchecked_node_ids_from_edge_id(edge_id).1,
            |dsts| dsts[edge_id as usize],
        )
    }

    /// Returns the destination of given edge id.
    ///
    /// # Arguments
    ///
    /// `edge_id`: EdgeT - The edge ID whose destination is to be retrieved.
    pub fn get_destination_node_id_from_edge_id(&self, edge_id: EdgeT) -> Result<NodeT, String> {
        if edge_id >= self.get_directed_edges_number() {
            return Err(format!(
                "The edge ID {} is higher than the number of available directed edges {}.",
                edge_id,
                self.get_directed_edges_number()
            ));
        }
        Ok(self.get_unchecked_destination_node_id_from_edge_id(edge_id))
    }

    /// Return vector of destinations for the given source node ID.
    ///
    /// # Arguments
    ///
    /// * `node_id`: NodeT - Node ID whose neighbours are to be retrieved.
    ///
    /// # Example
    /// To retrieve the neighbours of a given node `src` you can use:
    ///
    /// ```rust
    /// # let graph = graph::test_utilities::load_ppi(true, true, true, true, false, false).unwrap();
    /// # let node_id = 0;
    /// println!("The neighbours of the node {} are {:?}.", node_id, graph.get_node_neighbours_from_node_id(node_id).unwrap());
    /// let unavailable_node = 2349765432;
    /// assert!(graph.get_node_neighbours_from_node_id(unavailable_node).is_err());
    /// ```
    pub fn get_node_neighbours_from_node_id(&self, node_id: NodeT) -> Result<Vec<NodeT>, String> {
        if node_id >= self.get_nodes_number() {
            return Err(format!(
                "The node ID {} is higher than the number of available nodes {}.",
                node_id,
                self.get_nodes_number()
            ));
        }
        Ok(self.iter_node_neighbours_ids(node_id).collect())
    }

    /// Return vector of destinations for the given source node name.
    ///
    /// # Arguments
    ///
    /// * `node_name`: &str - Node ID whose neighbours are to be retrieved.
    ///
    /// # Example
    /// To retrieve the neighbours of a given node `src` you can use:
    ///
    /// ```rust
    /// # let graph = graph::test_utilities::load_ppi(true, true, true, true, false, false).unwrap();
    /// let node_name = "ENSP00000000233";
    /// println!("The neighbours of the node {} are {:?}.", node_name, graph.get_node_neighbour_ids_from_node_name(node_name).unwrap());
    /// ```
    pub fn get_node_neighbour_ids_from_node_name(
        &self,
        node_name: &str,
    ) -> Result<Vec<NodeT>, String> {
        self.get_node_neighbours_from_node_id(self.get_node_id_from_node_name(node_name)?)
    }

    /// Return vector of destination names for the given source node name.
    ///
    /// # Arguments
    ///
    /// * `node_id`: NodeT - Node ID whose neighbours are to be retrieved.
    ///
    /// # Example
    /// To retrieve the neighbours of a given node `src` you can use:
    ///
    /// ```rust
    /// # let graph = graph::test_utilities::load_ppi(true, true, true, true, false, false).unwrap();
    /// let node_name = "ENSP00000000233";
    /// println!("The neighbours of the node {} are {:?}.", node_name, graph.get_node_neighbour_names_from_node_name(node_name).unwrap());
    /// ```
    pub fn get_node_neighbour_names_from_node_name(
        &self,
        node_name: &str,
    ) -> Result<Vec<String>, String> {
        Ok(self
            .iter_node_neighbours(self.get_node_id_from_node_name(node_name)?)
            .collect())
    }

    /// Return range of outbound edges IDs for all the edges bewteen the given
    /// source and destination nodes.
    /// This operation is meaningfull only in a multigraph.
    ///
    /// # Arguments
    ///
    /// * src: NodeT - Source node.
    /// * dst: NodeT - Destination node.
    ///
    pub fn get_minmax_edge_ids_from_node_ids(
        &self,
        src: NodeT,
        dst: NodeT,
    ) -> Result<(EdgeT, EdgeT), String> {
        Ok((
            self.get_edge_id_from_node_ids(src, dst)?,
            self.get_unchecked_edge_id_from_node_ids(src, dst + 1),
        ))
    }

    /// Return edge ID for given tuple of nodes and edge type.
    ///
    /// This method will return an error if the graph does not contain the
    /// requested edge with edge type.
    ///
    /// # Arguments
    /// `src`: NodeT - Source node of the edge.
    /// `dst`: NodeT - Destination node of the edge.
    /// `edge_type`: Option<EdgeTypeT> - Edge Type of the edge.
    ///
    pub fn get_edge_id_with_type_from_node_ids(
        &self,
        src: NodeT,
        dst: NodeT,
        edge_type: Option<EdgeTypeT>,
    ) -> Result<EdgeT, String> {
        self.edge_types
            .as_ref()
            .map_or_else(
                || self.get_edge_id_from_node_ids(src, dst).ok(),
                |ets| {
                    self.iter_edge_ids_from_node_ids(src, dst)
                        .ok()
                        .and_then(|mut edge_ids| {
                            edge_ids.find(|edge_id| ets.ids[*edge_id as usize] == edge_type)
                        })
                },
            )
            .ok_or_else(|| {
                format!(
                    concat!(
                    "The current graph instance does not contain the required edge composed of ",
                    "source node ID {}, destination node ID {} and edge ID {:?}."
                ),
                    src, dst, edge_type
                )
            })
    }

    /// Return edge ID for given tuple of node names.
    ///
    /// This method will return an error if the graph does not contain the
    /// requested edge with edge type.
    ///
    /// # Arguments
    /// `src_name`: &str - Source node name of the edge.
    /// `dst_name`: &str - Destination node name of the edge.
    ///
    pub fn get_edge_id_from_node_names(
        &self,
        src_name: &str,
        dst_name: &str,
    ) -> Result<EdgeT, String> {
        match (self.nodes.get(src_name), self.nodes.get(dst_name)) {
            (Some(src), Some(dst)) => self.get_edge_id_from_node_ids(*src, *dst).ok(),
            _ => None,
        }
        .ok_or_else(|| {
            format!(
                concat!(
                    "The current graph instance does not contain the required edge composed of ",
                    "source node name {} and destination node name {}."
                ),
                src_name, dst_name
            )
        })
    }

    /// Return edge ID for given tuple of node names and edge type name.
    ///
    /// This method will return an error if the graph does not contain the
    /// requested edge with edge type.
    ///
    /// # Arguments
    /// `src_name`: &str - Source node name of the edge.
    /// `dst_name`: &str - Destination node name of the edge.
    /// `edge_type_name`: Option<&String> - Edge type name.
    ///
    pub fn get_edge_id_with_type_from_node_names(
        &self,
        src_name: &str,
        dst_name: &str,
        edge_type_name: Option<&String>,
    ) -> Result<EdgeT, String> {
        match (self.nodes.get(src_name), self.nodes.get(dst_name)) {
            (Some(src), Some(dst)) => self
                .get_edge_id_with_type_from_node_ids(
                    *src,
                    *dst,
                    self.get_edge_type_id_from_edge_type_name(edge_type_name.map(|x| x.as_str()))?,
                )
                .ok(),
            _ => None,
        }
        .ok_or_else(|| {
            format!(
                concat!(
                    "The current graph instance does not contain the required edge composed of ",
                    "source node name {}, destination node name {} and edge name {:?}."
                ),
                src_name, dst_name, edge_type_name
            )
        })
    }

    /// Return translated edge types from string to internal edge ID.
    ///
    /// # Arguments
    ///
    /// * `edge_types`: Vec<String> - Vector of edge types to be converted.
    pub fn get_edge_type_ids_from_edge_type_names(
        &self,
        edge_types: Vec<Option<String>>,
    ) -> Result<Vec<Option<EdgeTypeT>>, String> {
        match &self.edge_types {
                None => Err(String::from("Current graph does not have edge types.")),
                Some(ets) => {
                    edge_types
                    .iter()
                    .map(|edge_type_name|
                        match edge_type_name {
                            None=> Ok(None),
                            Some(et) => {
                                match ets.get(et) {
                                    Some(edge_type_id) => Ok(Some(*edge_type_id)),
                                    None => Err(format!(
                                        "The edge type {} does not exist in current graph. The available edge types are {}.",
                                        et,
                                        ets.keys().join(", ")
                                    ))
                                }
                            }
                        }
                    )
                .collect::<Result<Vec<Option<EdgeTypeT>>, String>>()
            }
        }
    }

    /// Return translated node types from string to internal node ID.
    ///
    /// # Arguments
    ///
    /// * `node_types`: Vec<String> - Vector of node types to be converted.
    pub fn get_node_type_ids_from_node_type_names(
        &self,
        node_types: Vec<Option<String>>,
    ) -> Result<Vec<Option<NodeTypeT>>, String> {
        match &self.node_types {
            None => Err(String::from("Current graph does not have node types.")),
            Some(nts) => {
                node_types
                .iter()
                .map(|node_type_name|
                    match node_type_name {
                        None => Ok(None),
                        Some(nt) => {
                            match nts.get(nt) {
                                Some(node_type_id) => Ok(Some(*node_type_id)),
                                None => Err(format!(
                                    "The node type {} does not exist in current graph. The available node types are {}.",
                                    nt,
                                    nts.keys().join(", ")
                                )),
                            }
                        }
                    })
                .collect::<Result<Vec<Option<NodeTypeT>>, String>>()
            }
        }
    }

    /// Return range of outbound edges IDs which have as source the given Node.
    ///
    /// # Arguments
    ///
    /// * src: NodeT - Node for which we need to compute the outbounds range.
    ///
    pub(crate) fn get_minmax_edge_ids_from_source_node_id(&self, src: NodeT) -> (EdgeT, EdgeT) {
        match &self.outbounds {
            Some(outbounds) => {
                let min_edge_id = if src == 0 {
                    0
                } else {
                    outbounds[src as usize - 1]
                };
                (min_edge_id, outbounds[src as usize])
            }
            None => {
                let min_edge_id: EdgeT = self.get_unchecked_edge_id_from_node_ids(src, 0);
                (
                    min_edge_id,
                    match &self.cached_destinations {
                        Some(cds) => match cds.get(&src) {
                            Some(destinations) => destinations.len() as EdgeT + min_edge_id,
                            None => self.get_unchecked_edge_id_from_node_ids(src + 1, 0),
                        },
                        None => self.get_unchecked_edge_id_from_node_ids(src + 1, 0),
                    },
                )
            }
        }
    }

    /// Return node type name of given node type.
    ///
    /// There is no need for a unchecked version since we will have to map
    /// on the note_types anyway.
    ///
    /// # Arguments
    /// * node_type_id: Vec<NodeTypeT> - Id of the node type.
    pub fn get_node_type_name_from_node_type_id(
        &self,
        node_type_id: NodeTypeT,
    ) -> Result<String, String> {
        self.node_types.as_ref().map_or(
            Err("Node types not available for the current graph instance.".to_string()),
            |nts| nts.translate(node_type_id),
        )
    }

    /// Return node type name of given node type.
    ///
    /// # Arguments
    /// * node_type_ids: Vec<NodeTypeT> - Id of the node type.
    pub fn get_node_type_names_from_node_type_ids(
        &self,
        node_type_ids: Vec<NodeTypeT>,
    ) -> Result<Vec<String>, String> {
        self.node_types.as_ref().map_or(
            Err("Node types not available for the current graph instance.".to_string()),
            |nts| nts.translate_vector(node_type_ids),
        )
    }

    /// Returns the number of outbound neighbours of given node.
    ///
    /// This is implemented as proposed by [S. Vigna here](http://vigna.di.unimi.it/ftp/papers/Broadword.pdf).
    ///
    /// # Arguments
    ///
    /// * `node_id` - Integer ID of the node.
    ///
    pub fn get_node_degree_from_node_id(&self, node_id: NodeT) -> Result<NodeT, String> {
        if node_id >= self.get_nodes_number() {
            return Err(format!(
                "The node ID {} is higher than the number of available nodes {}.",
                node_id,
                self.get_nodes_number()
            ));
        }
        let (min_edge_id, max_edge_id) = self.get_minmax_edge_ids_from_source_node_id(node_id);
        Ok((max_edge_id - min_edge_id) as NodeT)
    }
}
