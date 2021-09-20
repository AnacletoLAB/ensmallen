use super::*;
use rayon::prelude::*;

/// # Queries
/// The naming convention we follow is:
/// * `/get_(.+?)_from_(.+)/`
/// * `/get_(.+?)_from_(.+)_unchecked/`
impl Graph {
    #[no_inverse_method]
    /// Returns option with the weight of the given edge id.
    ///
    /// This method will raise a panic if the given edge ID is higher than
    /// the number of edges in the graph. Additionally, it will simply
    /// return None if there are no graph weights.
    ///
    /// # Arguments
    /// * `edge_id`: EdgeT - The edge whose edge weight is to be returned.
    ///
    /// # Safety
    /// If the given edge ID does not exists in the graph this method will panic.
    pub unsafe fn get_unchecked_edge_weight_from_edge_id(&self, edge_id: EdgeT) -> Option<WeightT> {
        self.weights.as_ref().map(|ws| ws[edge_id as usize])
    }

    /// Returns option with the weight of the given node ids.
    ///
    /// This method will raise a panic if the given node IDs are higher than
    /// the number of nodes in the graph.
    ///
    /// # Arguments
    /// * `src`: NodeT - The source node ID.
    /// * `dst`: NodeT - The destination node ID.
    ///
    /// # Safety
    /// If either of the two given node IDs does not exists in the graph.
    pub unsafe fn get_unchecked_edge_weight_from_node_ids(
        &self,
        src: NodeT,
        dst: NodeT,
    ) -> WeightT {
        self.get_unchecked_edge_weight_from_edge_id(
            self.get_unchecked_edge_id_from_node_ids(src, dst),
        )
        .unwrap_unchecked()
    }

    /// Returns node id from given node name raising a panic if used unproperly.
    ///
    /// # Arguments
    /// * `node_name`: &str - The node name whose node ID is to be returned.
    ///
    /// # Safety
    /// If the given node name does not exists in the considered graph the method will panic.
    pub unsafe fn get_unchecked_node_id_from_node_name(&self, node_name: &str) -> NodeT {
        self.nodes.get(node_name).unwrap()
    }

    /// Return edge type ID corresponding to the given edge type name.
    ///
    /// # Arguments
    /// * `edge_type_name`: &str - The edge type name whose edge type ID is to be returned.
    ///
    /// # Safety
    /// If the given edge type name does not exists in the considered graph the method will panic.
    pub unsafe fn get_unchecked_edge_type_id_from_edge_type_name(
        &self,
        edge_type_name: &str,
    ) -> Option<EdgeTypeT> {
        self.edge_types
            .as_ref()
            .and_then(|ets| ets.get(edge_type_name))
    }

    /// Return edge type ID corresponding to the given edge type name
    /// raising panic if edge type ID does not exists in current graph.
    ///
    /// # Arguments
    /// * `edge_type_id`: Option<EdgeTypeT> - The edge type naIDme whose edge type name is to be returned.
    ///
    /// # Safety
    /// If the given edge type ID does not exist in the graph the method will panic.
    pub unsafe fn get_unchecked_edge_type_name_from_edge_type_id(
        &self,
        edge_type_id: Option<EdgeTypeT>,
    ) -> Option<String> {
        match (&self.edge_types, edge_type_id) {
            (Some(ets), Some(et)) => Some(ets.unchecked_translate(et)),
            _ => None,
        }
    }

    /// Return number of edges of the given edge type without checks.
    ///
    /// # Arguments
    /// * `edge_type`: Option<EdgeTypeT> - The edge type to retrieve count of.
    ///
    /// # Safety
    /// If the given edge type ID does not exist in the graph the method will panic.
    pub unsafe fn get_unchecked_edge_count_from_edge_type_id(
        &self,
        edge_type: Option<EdgeTypeT>,
    ) -> EdgeT {
        match (&self.edge_types, edge_type) {
            (Some(ets), None) => ets.get_unknown_count(),
            (Some(ets), Some(et)) => ets.counts[et as usize],
            _ => unreachable!("The current graph instance does not have edge types!"),
        }
    }

    /// Return number of nodes of the given node type without checks.
    ///
    /// # Arguments
    /// * node_type: Option<NodeTypeT> - The node type to retrieve count of.
    pub(crate) fn get_unchecked_node_count_from_node_type_id(
        &self,
        node_type: Option<NodeTypeT>,
    ) -> NodeT {
        match (&self.node_types, node_type) {
            (Some(nts), None) => nts.get_unknown_count(),
            (Some(nts), Some(nt)) => nts.counts[nt as usize],
            _ => unreachable!("The current graph instance does not have node types!"),
        }
    }

    /// Return edge ID without any checks for given tuple of nodes and edge type.
    ///
    /// # Arguments
    /// * `src`: NodeT - Source node of the edge.
    /// * `dst`: NodeT - Destination node of the edge.
    /// * `edge_type`: Option<EdgeTypeT> - Edge Type of the edge.
    ///
    /// # Safety
    /// If the given node IDs or edge type does not exists in the graph this method will panic.
    pub unsafe fn get_unchecked_edge_id_from_node_ids_and_edge_type_id(
        &self,
        src: NodeT,
        dst: NodeT,
        edge_type: Option<EdgeTypeT>,
    ) -> EdgeT {
        self.edge_types.as_ref().map_or_else(
            || self.get_unchecked_edge_id_from_node_ids(src, dst),
            |ets| {
                self.iter_unchecked_edge_ids_from_node_ids(src, dst)
                    // The vectors of the edge types can only have one element.
                    .find(|edge_id| ets.ids[*edge_id as usize] == edge_type)
                    .unwrap()
            },
        )
    }

    /// Return range of outbound edges IDs for all the edges bewteen the given
    /// source and destination nodes.
    /// This operation is meaningfull only in a multigraph.
    ///
    /// # Arguments
    ///
    /// * `src`: NodeT - Source node.
    /// * `dst`: NodeT - Destination node.
    ///
    /// # Safety
    /// If the given node type IDs do not exist in the graph this method will panic.
    pub unsafe fn get_unchecked_minmax_edge_ids_from_node_ids(
        &self,
        src: NodeT,
        dst: NodeT,
    ) -> (EdgeT, EdgeT) {
        (
            self.get_unchecked_edge_id_from_node_ids(src, dst),
            self.get_unchecked_edge_id_from_node_ids(src, dst + 1),
        )
    }

    /// Return the number of edges between the given source and destination nodes.
    ///
    /// This might be thought as the degree of an edge in a multigraph.
    /// On non-multigraph this trivially return 1 on existing edges and 0 on
    /// the non-existing ones.
    ///
    /// # Arguments
    ///
    /// * `src`: NodeT - Source node.
    /// * `dst`: NodeT - Destination node.
    ///
    pub(crate) fn get_unchecked_edge_degree_from_node_ids(&self, src: NodeT, dst: NodeT) -> EdgeT {
        let (min_edge_id, max_edge_id) =
            unsafe { self.get_unchecked_minmax_edge_ids_from_node_ids(src, dst) };
        max_edge_id - min_edge_id
    }

    #[inline(always)]
    /// Returns node IDs corresponding to given edge ID.
    ///
    /// The method will panic if the given edge ID does not exists in the
    /// current graph instance.
    ///
    /// # Arguments
    /// * `edge_id`: EdgeT - The edge ID whose source and destination node IDs are to e retrieved.
    ///
    /// # Example
    /// To retrieve the source and destination node IDs of a given edge ID you can use the following:
    ///
    /// ```rust
    /// # let graph = graph::test_utilities::load_ppi(true, true, true, true, false, false);
    /// let edge_id = 0;
    /// let (src, dst) = unsafe { graph.get_unchecked_node_ids_from_edge_id(edge_id) };
    /// println!("The edge with ID {} has source node ID {} and destination node ID {}.", edge_id, src, dst);
    /// ```
    ///
    /// # Safety
    /// If the given edge ID does not exist in the current graph the method will raise a panic.
    pub unsafe fn get_unchecked_node_ids_from_edge_id(&self, edge_id: EdgeT) -> (NodeT, NodeT) {
        if let (Some(sources), Some(destinations)) = (&self.sources, &self.destinations) {
            return (sources[edge_id as usize], destinations[edge_id as usize]);
        }
        self.decode_edge(self.edges.unchecked_select(edge_id))
    }

    /// Returns node names corresponding to given edge ID.
    ///
    /// # Arguments
    /// * `edge_id`: EdgeT - The edge ID whose source and destination node IDs are to e retrieved.
    ///
    /// # Safety
    /// If the given edge ID does not exist in the current graph the method will raise a panic.
    pub unsafe fn get_unchecked_node_names_from_edge_id(&self, edge_id: EdgeT) -> (String, String) {
        let (src, dst) = self.get_unchecked_node_ids_from_edge_id(edge_id);
        (
            self.get_unchecked_node_name_from_node_id(src),
            self.get_unchecked_node_name_from_node_id(dst),
        )
    }

    /// Returns the source of given edge id without making any boundary check.
    ///
    /// # Arguments
    /// * `edge_id`: EdgeT - The edge ID whose source is to be retrieved.
    ///
    /// # Safety
    /// If the given edge ID does not exist in the current graph the method will cause an out of bounds.
    pub unsafe fn get_unchecked_source_node_id_from_edge_id(&self, edge_id: EdgeT) -> NodeT {
        self.sources.as_ref().map_or_else(
            || self.get_unchecked_node_ids_from_edge_id(edge_id).0,
            |srscs| srscs[edge_id as usize],
        )
    }

    /// Returns the destination of given edge id without making any boundary check.
    ///
    /// # Arguments
    /// * `edge_id`: EdgeT - The edge ID whose destination is to be retrieved.
    ///
    /// # Safety
    /// If the given edge ID does not exist in the current graph the method will cause an out of bounds.
    pub unsafe fn get_unchecked_destination_node_id_from_edge_id(&self, edge_id: EdgeT) -> NodeT {
        self.destinations.as_ref().map_or_else(
            || self.get_unchecked_node_ids_from_edge_id(edge_id).1,
            |dsts| dsts[edge_id as usize],
        )
    }

    /// Returns source node ID corresponding to given edge ID.
    ///
    /// # Arguments
    /// * `edge_id`: EdgeT - The edge ID whose source node ID is to be retrieved.
    ///
    /// # Raises
    /// * If the given edge ID does not exist in the current graph.
    pub fn get_source_node_id_from_edge_id(&self, edge_id: EdgeT) -> Result<NodeT> {
        self.validate_edge_id(edge_id)
            .map(|edge_id| unsafe { self.get_unchecked_source_node_id_from_edge_id(edge_id) })
    }

    /// Returns destination node ID corresponding to given edge ID.
    ///
    /// # Arguments
    /// * `edge_id`: EdgeT - The edge ID whose destination node ID is to be retrieved.
    ///
    /// # Raises
    /// * If the given edge ID does not exist in the current graph.
    pub fn get_destination_node_id_from_edge_id(&self, edge_id: EdgeT) -> Result<NodeT> {
        self.validate_edge_id(edge_id)
            .map(|edge_id| unsafe { self.get_unchecked_destination_node_id_from_edge_id(edge_id) })
    }

    /// Returns source node name corresponding to given edge ID.
    ///
    /// # Arguments
    /// * `edge_id`: EdgeT - The edge ID whose source node name is to be retrieved.
    ///
    /// # Safety
    /// If the given edge ID does not exist in the current graph the method will raise a panic.
    pub unsafe fn get_unchecked_source_node_name_from_edge_id(&self, edge_id: EdgeT) -> String {
        self.get_unchecked_node_name_from_node_id(
            self.get_unchecked_source_node_id_from_edge_id(edge_id),
        )
    }

    /// Returns destination node name corresponding to given edge ID.
    ///
    /// # Arguments
    /// * `edge_id`: EdgeT - The edge ID whose destination node name is to be retrieved.
    ///
    /// # Safety
    /// If the given edge ID does not exist in the current graph the method will raise a panic.
    pub unsafe fn get_unchecked_destination_node_name_from_edge_id(
        &self,
        edge_id: EdgeT,
    ) -> String {
        self.get_unchecked_node_name_from_node_id(
            self.get_unchecked_destination_node_id_from_edge_id(edge_id),
        )
    }

    /// Returns source node name corresponding to given edge ID.
    ///
    /// # Arguments
    /// * `edge_id`: EdgeT - The edge ID whose source node name is to be retrieved.
    ///
    /// # Raises
    /// If the given edge ID does not exist in the current graph.
    pub fn get_source_node_name_from_edge_id(&self, edge_id: EdgeT) -> Result<String> {
        self.validate_edge_id(edge_id)
            .map(|edge_id| unsafe { self.get_unchecked_source_node_name_from_edge_id(edge_id) })
    }

    /// Returns destination node name corresponding to given edge ID.
    ///
    /// # Arguments
    /// * `edge_id`: EdgeT - The edge ID whose destination node name is to be retrieved.
    ///
    /// # Raises
    /// If the given edge ID does not exist in the current graph.
    pub fn get_destination_node_name_from_edge_id(&self, edge_id: EdgeT) -> Result<String> {
        self.validate_edge_id(edge_id).map(|edge_id| unsafe {
            self.get_unchecked_destination_node_name_from_edge_id(edge_id)
        })
    }

    /// Returns node names corresponding to given edge ID.
    ///
    /// # Arguments
    /// * `edge_id`: EdgeT - The edge ID whose source and destination node IDs are to e retrieved.
    ///
    pub fn get_node_names_from_edge_id(&self, edge_id: EdgeT) -> Result<(String, String)> {
        self.validate_edge_id(edge_id)
            .map(|edge_id| unsafe { self.get_unchecked_node_names_from_edge_id(edge_id) })
    }

    /// Returns node names corresponding to given edge ID.
    ///
    /// # Arguments
    /// * `edge_id`: EdgeT - The edge ID whose source and destination node IDs are to e retrieved.
    ///
    /// # Example
    /// To retrieve the source and destination node IDs of a given edge ID you can use the following:
    ///
    /// ```rust
    /// # let graph = graph::test_utilities::load_ppi(true, true, true, true, false, false);
    /// assert!(graph.get_node_ids_from_edge_id(0).is_ok());
    /// assert!(graph.get_node_ids_from_edge_id(10000000000).is_err());
    /// ```
    pub fn get_node_ids_from_edge_id(&self, edge_id: EdgeT) -> Result<(NodeT, NodeT)> {
        self.validate_edge_id(edge_id)
            .map(|edge_id| unsafe { self.get_unchecked_node_ids_from_edge_id(edge_id) })
    }

    #[inline(always)]
    /// Returns edge ID corresponding to given source and destination node IDs.
    ///
    /// The method will panic if the given source and destination node IDs do
    /// not correspond to an edge in this graph instance.
    ///
    /// # Arguments
    /// * `src`: NodeT - The source node ID.
    /// * `dst`: NodeT - The destination node ID.
    ///
    /// # Example
    /// To retrieve the edge ID curresponding to the given source and destination node IDs you can use the following:
    ///
    /// ```rust
    /// # let graph = graph::test_utilities::load_ppi(false, true, true, true, false, false);
    /// let src = 0;
    /// let dst = 1;
    /// let edge_id = unsafe { graph.get_unchecked_edge_id_from_node_ids(src, dst) };
    /// println!("The source node ID {} and destination node ID {} corrrespond to the edge with ID {}.", src, dst, edge_id);
    /// ```
    ///
    /// # Safety
    /// If any of the given node IDs do not exist in the graph the method will panic.
    pub unsafe fn get_unchecked_edge_id_from_node_ids(&self, src: NodeT, dst: NodeT) -> EdgeT {
        self.edges.unchecked_rank(self.encode_edge(src, dst)) as EdgeT
    }

    #[inline(always)]
    /// Returns edge ID corresponding to given source and destination node IDs.
    ///
    /// # Arguments
    /// * `src`: NodeT - The source node ID.
    /// * `dst`: NodeT - The destination node ID.
    ///
    /// # Example
    /// To retrieve the edge ID curresponding to the given source and destination node IDs you can use the following:
    ///
    /// ```rust
    /// # let graph = graph::test_utilities::load_ppi(false, true, true, true, false, false);
    /// assert!(graph.get_edge_id_from_node_ids(0, 1).is_ok());
    /// assert!(graph.get_edge_id_from_node_ids(0, 100000000).is_err());
    /// ```
    pub fn get_edge_id_from_node_ids(&self, src: NodeT, dst: NodeT) -> Result<EdgeT> {
        match self
            .edges
            .rank(self.encode_edge(src, dst))
            .map(|value| value as EdgeT) {
                Some(edge_id) => Ok(edge_id),
                None => Err(format!("The edge composed by the source node {} and destination node {} does not exist in this graph.", src, dst))
            }
    }

    #[inline(always)]
    /// Returns edge ID corresponding to given source and destination node IDs.
    ///
    /// # Arguments
    /// * `source_id`: NodeT - The source node ID.
    ///
    /// # Example
    /// To retrieve the edge ID curresponding to the given source and destination node IDs you can use the following:
    ///
    /// ```rust
    /// # let graph_without_singletons = graph::test_utilities::load_ppi(false, true, true, false, false, false);
    /// for node_id in graph_without_singletons.iter_node_ids(){
    ///     assert_eq!(
    ///         unsafe { graph_without_singletons.get_unchecked_unique_source_node_id(node_id)},
    ///         node_id,
    ///         "The expected node ID does not match the obtained node ID."
    ///     );
    /// }
    /// ```
    ///
    /// # Safety
    /// If the given source node ID does not exist in the current graph the method will panic.
    pub unsafe fn get_unchecked_unique_source_node_id(&self, source_id: NodeT) -> NodeT {
        self.unique_sources
            .as_ref()
            .map_or(source_id, |unique_sources| {
                unique_sources.unchecked_select(source_id as u64) as NodeT
            })
    }

    /// Return the src, dst, edge type of a given edge ID.
    ///
    /// This method will raise a panic when an improper configuration is used.
    ///
    /// # Arguments
    /// * `edge_id`: EdgeT - The edge ID whose source, destination and edge type are to be retrieved.
    ///
    /// # Example
    /// In order to retrieve a given edge ID informations, you can use the following:
    ///
    /// ```rust
    /// # let graph = graph::test_utilities::load_ppi(true, true, true, true, false, false);
    /// let edge_id = 0;
    /// let (src, dst, edge_type) = unsafe { graph.get_unchecked_node_ids_and_edge_type_id_from_edge_id(edge_id) };
    /// println!("The edge with ID {} has source node ID {}, destination node ID {} and edge type ID {:?}", edge_id, src, dst, edge_type);
    /// ```
    ///
    /// # Safety
    /// If the given edge ID does not exist in the current graph the method will raise a panic.
    pub unsafe fn get_unchecked_node_ids_and_edge_type_id_from_edge_id(
        &self,
        edge_id: EdgeT,
    ) -> (NodeT, NodeT, Option<EdgeTypeT>) {
        let (src, dst) = self.get_unchecked_node_ids_from_edge_id(edge_id);
        (
            src,
            dst,
            self.get_unchecked_edge_type_id_from_edge_id(edge_id),
        )
    }

    /// Return the src, dst, edge type of a given edge ID.
    ///
    /// # Arguments
    /// * `edge_id`: EdgeT - The edge ID whose source, destination and edge type are to be retrieved.
    ///
    /// # Example
    /// In order to retrieve a given edge ID informations, you can use the following:
    ///
    /// ```rust
    /// # let graph = graph::test_utilities::load_ppi(true, true, true, true, false, false);
    /// assert!(graph.get_node_ids_and_edge_type_id_from_edge_id(0).is_ok());
    /// assert!(graph.get_node_ids_and_edge_type_id_from_edge_id(10000000000).is_err());
    /// ```
    pub fn get_node_ids_and_edge_type_id_from_edge_id(
        &self,
        edge_id: EdgeT,
    ) -> Result<(NodeT, NodeT, Option<EdgeTypeT>)> {
        self.validate_edge_id(edge_id).map(|edge_id| unsafe {
            self.get_unchecked_node_ids_and_edge_type_id_from_edge_id(edge_id)
        })
    }

    /// Return the src, dst, edge type and weight of a given edge ID.
    ///
    /// This method will raise a panic when an improper configuration is used.
    ///
    /// # Arguments
    /// * `edge_id`: EdgeT - The edge ID whose source, destination, edge type and weight are to be retrieved.
    ///
    /// # Example
    /// In order to retrieve a given edge ID informations, you can use the following:
    ///
    /// ```rust
    /// # let graph = graph::test_utilities::load_ppi(true, true, true, true, false, false);
    /// let edge_id = 0;
    /// let (src, dst, edge_type, weight) = unsafe { graph.get_unchecked_node_ids_and_edge_type_id_and_edge_weight_from_edge_id(edge_id) };
    /// println!("The edge with ID {} has source node ID {}, destination node ID {}, edge type ID {:?} and weight {:?}.", edge_id, src, dst, edge_type, weight);
    /// ```
    ///
    /// # Safety
    /// If the given edge ID does not exist in the current graph the method will raise a panic.
    pub unsafe fn get_unchecked_node_ids_and_edge_type_id_and_edge_weight_from_edge_id(
        &self,
        edge_id: EdgeT,
    ) -> (NodeT, NodeT, Option<EdgeTypeT>, Option<WeightT>) {
        let (src, dst, edge_type) =
            self.get_unchecked_node_ids_and_edge_type_id_from_edge_id(edge_id);
        (
            src,
            dst,
            edge_type,
            self.get_unchecked_edge_weight_from_edge_id(edge_id),
        )
    }

    /// Return the src, dst, edge type and weight of a given edge ID.
    ///
    /// # Arguments
    /// * `edge_id`: EdgeT - The edge ID whose source, destination, edge type and weight are to be retrieved.
    ///
    /// # Example
    /// In order to retrieve a given edge ID informations, you can use the following:
    ///
    /// ```rust
    /// # let graph = graph::test_utilities::load_ppi(true, true, true, true, false, false);
    /// assert!(graph.get_node_ids_and_edge_type_id_and_edge_weight_from_edge_id(0).is_ok());
    /// assert!(graph.get_node_ids_and_edge_type_id_and_edge_weight_from_edge_id(10000000000).is_err());
    /// ```
    pub fn get_node_ids_and_edge_type_id_and_edge_weight_from_edge_id(
        &self,
        edge_id: EdgeT,
    ) -> Result<(NodeT, NodeT, Option<EdgeTypeT>, Option<WeightT>)> {
        self.validate_edge_id(edge_id).map(|edge_id| unsafe {
            self.get_unchecked_node_ids_and_edge_type_id_and_edge_weight_from_edge_id(edge_id)
        })
    }

    /// Return vector with unweighted top k central node Ids.
    ///
    /// If the k passed is bigger than the number of nodes this method will return
    /// all the nodes in the graph.
    ///
    /// # Arguments
    /// * `k`: NodeT - Number of central nodes to extract.
    ///
    /// # Raises
    /// * If the given value k is zero.
    /// * If the graph has no nodes.
    pub fn get_top_k_central_node_ids(&self, k: NodeT) -> Result<Vec<NodeT>> {
        if k == 0 {
            return Err(
                "K must be strictly a positive integer value greater than zero.".to_string(),
            );
        }
        if !self.has_nodes() {
            return Err("The node degrees are not well defined in an empty graph.".to_string());
        }
        let mut node_ids_and_degrees = vec![(0, 0); self.get_nodes_number() as usize];

        self.par_iter_node_degrees()
            .enumerate()
            .collect_into_vec(&mut node_ids_and_degrees);

        node_ids_and_degrees.par_sort_unstable_by(|(_, a), (_, b)| b.cmp(a));
        Ok(node_ids_and_degrees
            .into_iter()
            .take(k as usize)
            .map(|(node_id, _)| node_id as NodeT)
            .collect())
    }

    /// Return vector with weighted top k central node Ids.
    ///
    /// If the k passed is bigger than the number of nodes this method will return
    /// all the nodes in the graph.
    ///
    /// # Arguments
    /// * `k`: NodeT - Number of central nodes to extract.
    ///
    /// # Raises
    /// * If the current graph instance does not contain edge weights.
    /// * If the given value k is zero.
    ///
    /// TODO! Sort the returned values!
    pub fn get_weighted_top_k_central_node_ids(&self, k: NodeT) -> Result<Vec<NodeT>> {
        self.must_have_edge_weights()?;
        if k == 0 {
            return Err(
                "K must be strictly a positive integer value greater than zero.".to_string(),
            );
        }
        if !self.has_nodes() {
            return Err(
                "The weighted node degrees are not well defined in an empty graph.".to_string(),
            );
        }
        let k = k.min(self.get_nodes_number());
        let mut most_central_node_degrees = vec![0.0; k as usize];
        let mut most_central_node_ids = vec![0; k as usize];
        self.iter_node_ids().for_each(|node_id| unsafe {
            let degree = self.get_unchecked_weighted_node_degree_from_node_id(node_id);
            let (argmin, min_degree) = most_central_node_degrees
                .iter_mut()
                .enumerate()
                .min_by(|(_, node_degree_one), (_, node_degree_two)| {
                    (**node_degree_one).partial_cmp(*node_degree_two).unwrap()
                })
                .unwrap();
            if *min_degree <= degree {
                *min_degree = degree;
                most_central_node_ids[argmin] = node_id;
            }
        });

        Ok(most_central_node_ids)
    }

    /// Returns the number of outbound neighbours of given node.
    ///
    /// # Arguments
    ///
    /// * `node_id`: NodeT - Integer ID of the node.
    ///
    /// # Safety
    /// If the given node ID does not exist in the current graph the method will raise a panic.
    pub unsafe fn get_unchecked_node_degree_from_node_id(&self, node_id: NodeT) -> NodeT {
        let (min_edge_id, max_edge_id) =
            self.get_unchecked_minmax_edge_ids_from_source_node_id(node_id);
        (max_edge_id - min_edge_id) as NodeT
    }

    /// Returns the weighted sum of outbound neighbours of given node.
    ///
    /// The method will panic if the given node id is higher than the number of
    /// nodes in the graph.
    ///
    /// # Arguments
    ///
    /// * `node_id`: NodeT - Integer ID of the node.
    ///
    /// # Safety
    /// If the given node ID does not exist in the current graph the method will raise a panic.
    pub unsafe fn get_unchecked_weighted_node_degree_from_node_id(&self, node_id: NodeT) -> f64 {
        self.iter_unchecked_edge_weights_from_source_node_id(node_id)
            .map(|w| w as f64)
            .sum::<f64>()
    }

    /// Returns the number of outbound neighbours of given node ID.
    ///
    /// # Arguments
    /// * `node_id`: NodeT - Integer ID of the node.
    ///
    pub fn get_node_degree_from_node_id(&self, node_id: NodeT) -> Result<NodeT> {
        self.validate_node_id(node_id)
            .map(|node_id| unsafe { self.get_unchecked_node_degree_from_node_id(node_id) })
    }

    /// Returns the comulative node degree up to the given node.
    ///
    /// # Arguments
    /// * `node_id`: NodeT - Integer ID of the node.
    ///
    /// # Safety
    /// If the given node ID does not exist in the current graph the method will raise a panic.
    pub unsafe fn get_unchecked_comulative_node_degree_from_node_id(
        &self,
        node_id: NodeT,
    ) -> EdgeT {
        self.get_unchecked_edge_id_from_node_ids(node_id + 1, 0)
    }

    /// Returns the comulative node degree up to the given node.
    ///
    /// # Arguments
    /// * `node_id`: NodeT - Integer ID of the node.
    pub fn get_comulative_node_degree_from_node_id(&self, node_id: NodeT) -> Result<EdgeT> {
        self.validate_node_id(node_id).map(|node_id| unsafe {
            self.get_unchecked_comulative_node_degree_from_node_id(node_id)
        })
    }

    /// Returns the reciprocal squared root node degree up to the given node.
    ///
    /// # Arguments
    /// * `node_id`: NodeT - Integer ID of the node.
    ///
    /// # Safety
    /// If the given node ID does not exist in the current graph the method will raise a panic.
    pub unsafe fn get_unchecked_reciprocal_sqrt_degree_from_node_id(
        &self,
        node_id: NodeT,
    ) -> WeightT {
        (1.0 / (self.get_unchecked_node_degree_from_node_id(node_id) as f64).sqrt()) as WeightT
    }

    /// Returns the reciprocal squared root node degree up to the given node.
    ///
    /// # Arguments
    /// * `node_id`: NodeT - Integer ID of the node.
    pub fn get_reciprocal_sqrt_degree_from_node_id(&self, node_id: NodeT) -> Result<WeightT> {
        self.validate_node_id(node_id).map(|node_id| unsafe {
            self.get_unchecked_reciprocal_sqrt_degree_from_node_id(node_id)
        })
    }

    /// Return vector with reciprocal squared root degree of the provided nodes.
    ///
    /// # Arguments
    /// * `node_ids`: &[NodeT] - The vector of node IDs whose reciprocal squared root degree is to be retrieved.
    ///
    /// # Safety
    /// This method makes the assumption that the provided node IDs exist in the graph, that is
    /// they are not higher than the number of nodes in the graph.
    pub unsafe fn get_unchecked_reciprocal_sqrt_degrees_from_node_ids(
        &self,
        node_ids: &[NodeT],
    ) -> Vec<WeightT> {
        let mut reciprocal_sqrt_degrees = vec![0.0; node_ids.len()];
        if let Some(cached_reciprocal_sqrt_degrees) = self.reciprocal_sqrt_degrees.as_ref() {
            node_ids
                .par_iter()
                .map(|&node_id| cached_reciprocal_sqrt_degrees[node_id as usize])
                .collect_into_vec(&mut reciprocal_sqrt_degrees);
        } else {
            node_ids
                .par_iter()
                .map(|&node_id| self.get_unchecked_reciprocal_sqrt_degree_from_node_id(node_id))
                .collect_into_vec(&mut reciprocal_sqrt_degrees);
        }
        reciprocal_sqrt_degrees
    }

    /// Returns the weighted sum of outbound neighbours of given node ID.
    ///
    /// # Arguments
    /// * `node_id`: NodeT - Integer ID of the node.
    ///
    pub fn get_weighted_node_degree_from_node_id(&self, node_id: NodeT) -> Result<f64> {
        self.must_have_edge_weights()?;

        self.validate_node_id(node_id)
            .map(|node_id| unsafe { self.get_unchecked_weighted_node_degree_from_node_id(node_id) })
    }

    /// Returns the number of outbound neighbours of given node name.
    ///
    /// # Arguments
    /// * `node_name`: &str - Integer ID of the node.
    ///
    /// # Raises
    /// * If the given node name does not exist in the graph.
    pub fn get_node_degree_from_node_name(&self, node_name: &str) -> Result<NodeT> {
        Ok(unsafe {
            self.get_unchecked_node_degree_from_node_id(self.get_node_id_from_node_name(node_name)?)
        })
    }

    /// Return vector with top k central node names.
    ///
    /// # Arguments
    ///
    /// * `k`: NodeT - Number of central nodes to extract.
    pub fn get_top_k_central_node_names(&self, k: NodeT) -> Result<Vec<String>> {
        self.get_top_k_central_node_ids(k).map(|x| {
            x.into_iter()
                .map(|node_id| unsafe { self.get_unchecked_node_name_from_node_id(node_id) })
                .collect()
        })
    }

    /// Returns option with vector of node types of given node.
    ///
    /// This method will panic if the given node ID is greater than
    /// the number of nodes in the graph.
    /// Furthermore, if the graph does NOT have node types, it will NOT
    /// return neither an error or a panic.
    ///
    /// # Arguments
    ///
    /// * `node_id`: NodeT - node whose node type is to be returned.
    ///
    /// # Example
    /// ```rust
    /// # let graph = graph::test_utilities::load_ppi(true, true, true, true, false, false);
    /// println!("The node type id of node {} is {:?}", 0, unsafe{ graph.get_unchecked_node_type_id_from_node_id(0) });
    /// ```
    ///
    /// # Safety
    /// Even though the method will return an option when the node types are
    /// not available for the current graph, the behaviour is undefined.
    pub unsafe fn get_unchecked_node_type_id_from_node_id(
        &self,
        node_id: NodeT,
    ) -> Option<Vec<NodeTypeT>> {
        self.node_types
            .as_ref()
            .and_then(|nts| nts.ids[node_id as usize].clone())
    }

    /// Returns node type of given node.
    ///
    /// # Arguments
    ///
    /// * `node_id`: NodeT - node whose node type is to be returned.
    ///
    /// # Example
    /// ```rust
    /// # let graph = graph::test_utilities::load_ppi(true, true, true, true, false, false);
    /// println!("The node type id of node {} is {:?}", 0, graph.get_node_type_ids_from_node_id(0));
    /// ```
    ///
    pub fn get_node_type_ids_from_node_id(&self, node_id: NodeT) -> Result<Option<Vec<NodeTypeT>>> {
        self.must_have_node_types()?;
        self.validate_node_id(node_id)
            .map(|node_id| unsafe { self.get_unchecked_node_type_id_from_node_id(node_id) })
    }

    /// Returns edge type of given edge.
    ///
    /// This method will panic if the given edge ID is greater than
    /// the number of edges in the graph.
    /// Furthermore, if the graph does NOT have edge types, it will NOT
    /// return neither an error or a panic.
    ///
    /// # Arguments
    ///
    /// * `edge_id`: EdgeT - edge whose edge type is to be returned.
    ///
    /// # Example
    /// ```rust
    /// # let graph = graph::test_utilities::load_ppi(true, true, true, true, false, false);

    /// assert_eq!(unsafe{ graph.get_unchecked_edge_type_id_from_edge_id(0) }, Some(0));
    /// ```
    ///
    /// # Safety
    /// If the given edge ID does not exist in the current graph the method will raise a panic.
    pub unsafe fn get_unchecked_edge_type_id_from_edge_id(
        &self,
        edge_id: EdgeT,
    ) -> Option<EdgeTypeT> {
        self.edge_types
            .as_ref()
            .and_then(|ets| ets.ids[edge_id as usize])
    }

    /// Returns edge type of given edge.
    ///
    /// # Arguments
    ///
    /// * `edge_id`: EdgeT - edge whose edge type is to be returned.
    ///
    /// # Example
    /// ```rust
    /// # let graph = graph::test_utilities::load_ppi(true, true, true, true, false, false);
    /// # let graph_without_edge_types = graph::test_utilities::load_ppi(true, false, true, true, false, false);
    /// assert_eq!(graph.get_edge_type_id_from_edge_id(0).unwrap(), Some(0));
    /// assert!(graph_without_edge_types.get_edge_type_id_from_edge_id(0).is_err());
    /// ```
    pub fn get_edge_type_id_from_edge_id(&self, edge_id: EdgeT) -> Result<Option<EdgeTypeT>> {
        self.must_have_edge_types()?;
        self.validate_edge_id(edge_id)
            .map(|edge_id| unsafe { self.get_unchecked_edge_type_id_from_edge_id(edge_id) })
    }

    /// Returns result of option with the node type of the given node id.
    ///
    /// # Arguments
    /// * `node_id`: NodeT - The node ID whose node types are to be returned.
    ///
    /// # Safety
    /// This method will return an iterator of None values when the graph
    /// does not contain node types.
    pub unsafe fn get_unchecked_node_type_names_from_node_id(
        &self,
        node_id: NodeT,
    ) -> Option<Vec<String>> {
        self.get_unchecked_node_type_id_from_node_id(node_id)
            .map(|node_type_ids| {
                self.get_unchecked_node_type_names_from_node_type_ids(node_type_ids)
            })
    }

    /// Returns result of option with the node type of the given node id.
    ///
    /// # Arguments
    /// * `node_id`: NodeT - The node ID whose node types are to be returned.
    ///
    /// # Raises
    /// * If the node types are not available for the current graph instance.
    pub fn get_node_type_names_from_node_id(&self, node_id: NodeT) -> Result<Option<Vec<String>>> {
        self.must_have_node_types()?;
        Ok(self
            .get_node_type_ids_from_node_id(node_id)?
            .map(|node_type_ids| unsafe {
                self.get_unchecked_node_type_names_from_node_type_ids(node_type_ids)
            }))
    }

    /// Returns result of option with the node type of the given node name.
    ///
    /// # Arguments
    /// * `node_name`: &str - The node name whose node types are to be returned.
    pub fn get_node_type_names_from_node_name(
        &self,
        node_name: &str,
    ) -> Result<Option<Vec<String>>> {
        self.get_node_type_names_from_node_id(self.get_node_id_from_node_name(node_name)?)
    }

    /// Returns option with the edge type of the given edge id.
    ///
    /// # Arguments
    /// * `edge_id`: EdgeT - The edge ID whose edge type is to be returned.
    pub fn get_edge_type_name_from_edge_id(&self, edge_id: EdgeT) -> Result<Option<String>> {
        self.get_edge_type_id_from_edge_id(edge_id)?
            .map_or(Ok(None), |x| {
                Ok(Some(self.get_edge_type_name_from_edge_type_id(x)?))
            })
    }

    /// Return edge type name of given edge type.
    ///
    /// # Arguments
    /// * `edge_type_id`: EdgeTypeT - Id of the edge type.
    pub fn get_edge_type_name_from_edge_type_id(&self, edge_type_id: EdgeTypeT) -> Result<String> {
        self.must_have_edge_types()?;
        self.edge_types
            .as_ref()
            .map(|ets| ets.translate(edge_type_id))
            .unwrap()
    }

    /// Returns weight of the given edge id.
    ///
    /// # Arguments
    /// * `edge_id`: EdgeT - The edge ID whose weight is to be returned.
    ///
    /// # Example
    /// To get the weight of a given `edge_id` you can run:
    /// ```rust
    /// # let weighted_graph = graph::test_utilities::load_ppi(true, true, true, true, false, false);
    /// # let unweighted_graph = graph::test_utilities::load_ppi(true, true, false, true, false, false);
    /// let edge_id = 0;
    /// let unexistent_edge_id = 123456789;
    /// assert!(weighted_graph.get_edge_weight_from_edge_id(edge_id).is_ok());
    /// assert!(weighted_graph.get_edge_weight_from_edge_id(unexistent_edge_id).is_err());
    /// assert!(unweighted_graph.get_edge_weight_from_edge_id(edge_id).is_err());
    /// ```
    pub fn get_edge_weight_from_edge_id(&self, edge_id: EdgeT) -> Result<WeightT> {
        self.must_have_edge_weights()?;
        self.weights.as_ref().map(
            |weights| weights.get(edge_id as usize).map_or(
                Err(format!(
                    "The given edge_id {} is higher than the number of available directed edges {}.",
                    edge_id,
                    self.get_directed_edges_number()
                )),
                |value| Ok(*value)
            )
        ).unwrap()
    }

    /// Returns weight of the given node ids.
    ///
    /// # Arguments
    /// * `src`: NodeT - The node ID of the source node.
    /// * `dst`: NodeT - The node ID of the destination node.
    ///
    /// # Example
    /// To get the weight of a given `src` and `dst` you can run:
    /// ```rust
    /// # let weighted_graph = graph::test_utilities::load_ppi(false, true, true, true, false, false);
    /// let src = 0;
    /// let dst = 1;
    /// assert!(weighted_graph.get_edge_weight_from_node_ids(src, dst).is_ok());
    /// ```
    pub fn get_edge_weight_from_node_ids(&self, src: NodeT, dst: NodeT) -> Result<WeightT> {
        self.get_edge_weight_from_edge_id(self.get_edge_id_from_node_ids(src, dst)?)
    }

    /// Returns weight of the given node ids and edge type.
    ///
    /// # Arguments
    /// * `src`: NodeT - The node ID of the source node.
    /// * `dst`: NodeT - The node ID of the destination node.
    /// * `edge_type`: Option<EdgeTypeT> - The edge type ID of the edge.
    ///
    /// # Example
    /// To get the weight of a given `src` and `dst` and `edge_type` you can run:
    /// ```rust
    /// # let weighted_graph = graph::test_utilities::load_ppi(false, true, true, true, false, false);
    /// let src = 0;
    /// let dst = 1;
    /// let edge_type = Some(0);
    /// assert!(weighted_graph.get_edge_weight_from_node_ids_and_edge_type_id(src, dst, edge_type).is_ok());
    /// ```
    pub fn get_edge_weight_from_node_ids_and_edge_type_id(
        &self,
        src: NodeT,
        dst: NodeT,
        edge_type: Option<EdgeTypeT>,
    ) -> Result<WeightT> {
        self.get_edge_weight_from_edge_id(
            self.get_edge_id_from_node_ids_and_edge_type_id(src, dst, edge_type)?,
        )
    }

    /// Returns weight of the given node names and edge type.
    ///
    /// # Arguments
    /// * `src`: &str - The node name of the source node.
    /// * `dst`: &str - The node name of the destination node.
    /// * `edge_type`: Option<&str> - The edge type name of the edge.
    ///
    /// # Example
    /// To get the weight of a given `src` and `dst` and `edge_type` you can run:
    /// ```rust
    /// # let weighted_graph = graph::test_utilities::load_ppi(false, true, true, true, false, false);
    /// let src = "ENSP00000000233";
    /// let dst = "ENSP00000432568";
    /// let edge_type = Some("red");
    /// assert!(weighted_graph.get_edge_weight_from_node_names_and_edge_type_name(src, dst, edge_type).is_ok());
    /// ```
    pub fn get_edge_weight_from_node_names_and_edge_type_name(
        &self,
        src: &str,
        dst: &str,
        edge_type: Option<&str>,
    ) -> Result<WeightT> {
        self.get_edge_weight_from_edge_id(
            self.get_edge_id_from_node_names_and_edge_type_name(src, dst, edge_type)?,
        )
    }

    /// Returns weight of the given node names.
    ///
    /// # Arguments
    /// * `src_name`: &str - The node name of the source node.
    /// * `dst_name`: &str - The node name of the destination node.
    ///
    /// # Example
    /// To get the weight of a given `src_name` and `dst_name` you can run:
    /// ```rust
    /// # let weighted_graph = graph::test_utilities::load_ppi(false, true, true, true, false, false);
    /// let src_name = "ENSP00000000233";
    /// let dst_name = "ENSP00000432568";
    /// assert!(weighted_graph.get_edge_weight_from_node_names(src_name, dst_name).is_ok());
    /// ```
    pub fn get_edge_weight_from_node_names(
        &self,
        src_name: &str,
        dst_name: &str,
    ) -> Result<WeightT> {
        self.get_edge_weight_from_edge_id(self.get_edge_id_from_node_names(src_name, dst_name)?)
    }

    /// Returns result with the node name.
    ///
    /// # Arguments
    /// * `node_id`: NodeT - The node ID whose name is to be returned.
    ///
    /// # Example
    /// To get the name of a node you can use:
    /// ```rust
    /// # let graph = graph::test_utilities::load_ppi(true, true, true, true, false, false);
    /// assert_eq!(unsafe { graph.get_unchecked_node_name_from_node_id(0) }, "ENSG00000004059".to_string());
    /// ```
    ///
    /// # Safety
    /// If the given node ID does not exist in the current graph the method will raise a panic.
    pub unsafe fn get_unchecked_node_name_from_node_id(&self, node_id: NodeT) -> String {
        self.nodes.unchecked_translate(node_id)
    }

    /// Returns result with the node name.
    ///
    /// # Arguments
    /// * `node_id`: NodeT - The node ID whose name is to be returned.
    ///
    /// # Example
    /// To get the name of a node you can use:
    /// ```rust
    /// # let graph = graph::test_utilities::load_ppi(true, true, true, true, false, false);
    /// assert!(graph.get_node_name_from_node_id(0).is_ok());
    /// ```
    pub fn get_node_name_from_node_id(&self, node_id: NodeT) -> Result<String> {
        self.validate_node_id(node_id)
            .map(|node_id| unsafe { self.get_unchecked_node_name_from_node_id(node_id) })
    }

    /// Returns result with the node ID.
    ///
    /// # Arguments
    /// * `node_name`: &str - The node name whose node ID is to be returned.
    ///
    /// # Raises
    /// * When the given node name does not exists in the current graph.
    pub fn get_node_id_from_node_name(&self, node_name: &str) -> Result<NodeT> {
        match self.nodes.get(node_name) {
            Some(node_id) => Ok(node_id),
            None => Err(format!(
                "Given node name {} is not available in current graph.",
                node_name
            )),
        }
    }

    /// Returns result with the node IDs.
    ///
    /// # Arguments
    /// * `node_names`: Vec<&str> - The node names whose node IDs is to be returned.
    ///
    /// # Raises
    /// * When any of the given node name does not exists in the current graph.
    pub fn get_node_ids_from_node_names(&self, node_names: Vec<&str>) -> Result<Vec<NodeT>> {
        node_names
            .into_iter()
            .map(|node_name| self.get_node_id_from_node_name(node_name))
            .collect::<Result<Vec<NodeT>>>()
    }

    /// Returns result with the edge node IDs.
    ///
    /// # Arguments
    /// * `edge_node_names`: Vec<(&str, &str)> - The node names whose node IDs is to be returned.
    ///
    /// # Raises
    /// * When any of the given node name does not exists in the current graph.
    pub fn get_edge_node_ids_from_edge_node_names(
        &self,
        edge_node_names: Vec<(&str, &str)>,
    ) -> Result<Vec<(NodeT, NodeT)>> {
        edge_node_names
            .into_iter()
            .map(|(src_name, dst_name)| {
                Ok((
                    self.get_node_id_from_node_name(src_name)?,
                    self.get_node_id_from_node_name(dst_name)?,
                ))
            })
            .collect::<Result<Vec<(NodeT, NodeT)>>>()
    }

    /// Returns result with the edge node names.
    ///
    /// # Arguments
    /// * `edge_node_ids`: Vec<(NodeT, NodeT)> - The node names whose node names is to be returned.
    ///
    /// # Raises
    /// * When any of the given node IDs does not exists in the current graph.
    pub fn get_edge_node_names_from_edge_node_ids(
        &self,
        edge_node_ids: Vec<(NodeT, NodeT)>,
    ) -> Result<Vec<(String, String)>> {
        edge_node_ids
            .into_iter()
            .map(|(src_name, dst_name)| {
                Ok((
                    self.get_node_name_from_node_id(src_name)?,
                    self.get_node_name_from_node_id(dst_name)?,
                ))
            })
            .collect::<Result<Vec<(String, String)>>>()
    }

    /// Return node type ID for the given node name if available.
    ///
    /// # Arguments
    ///
    /// * `node_name`: &str - Name of the node.
    ///
    /// # Example
    /// To get the node type ID for a given node name you can run:
    /// ```rust
    /// # let graph = graph::test_utilities::load_ppi(true, true, true, true, false, false);
    /// let node_name = "ENSP00000000233";
    /// println!("The node type ID of node {} is {:?}.", node_name, graph.get_node_type_ids_from_node_name(node_name).unwrap());
    /// ```
    pub fn get_node_type_ids_from_node_name(
        &self,
        node_name: &str,
    ) -> Result<Option<Vec<NodeTypeT>>> {
        self.get_node_type_ids_from_node_id(self.get_node_id_from_node_name(node_name)?)
    }

    /// Return node type name for the given node name if available.
    ///
    /// # Arguments
    ///
    /// * `node_name`: &str - Name of the node.
    ///
    /// # Example
    /// To get the node type name for a given node name you can run:
    /// ```rust
    /// # let graph = graph::test_utilities::load_ppi(true, true, true, true, false, false);
    /// let node_name = "ENSP00000000233";
    /// println!("The node type of node {} is {:?}", node_name, graph.get_node_type_name_from_node_name(node_name).unwrap());
    /// ```
    pub fn get_node_type_name_from_node_name(
        &self,
        node_name: &str,
    ) -> Result<Option<Vec<String>>> {
        self.get_node_type_names_from_node_id(self.get_node_id_from_node_name(node_name)?)
    }

    /// Return number of edges with given edge type ID.
    ///
    /// If None is given as an edge type ID, the unknown edge type IDs
    /// will be returned.
    ///
    /// # Arguments
    /// * `edge_type_id`: Option<EdgeTypeT> - The edge type ID to count the edges of.
    ///
    pub fn get_edge_count_from_edge_type_id(
        &self,
        edge_type_id: Option<EdgeTypeT>,
    ) -> Result<EdgeT> {
        self.validate_edge_type_id(edge_type_id)
            .map(|edge_type_id| unsafe {
                self.get_unchecked_edge_count_from_edge_type_id(edge_type_id)
            })
    }

    /// Return edge type ID curresponding to given edge type name.
    ///
    /// If None is given as an edge type ID, None is returned.
    ///
    /// # Arguments
    /// * `edge_type_name`: Option<&str> - The edge type name whose ID is to be returned.
    ///
    /// TODO: refactor this method using new validation methods.
    pub fn get_edge_type_id_from_edge_type_name(
        &self,
        edge_type_name: Option<&str>,
    ) -> Result<Option<EdgeTypeT>> {
        match (&self.edge_types, edge_type_name) {
            (None, _) => Err("Current graph does not have edge types.".to_owned()),
            (Some(_), None) => Ok(None),
            (Some(ets), Some(etn)) => match ets.get(etn) {
                Some(edge_type_id) => Ok(Some(edge_type_id)),
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
    /// * `edge_type_name`: Option<&str> - The edge type name to count the edges of.
    ///
    pub fn get_edge_count_from_edge_type_name(
        &self,
        edge_type_name: Option<&str>,
    ) -> Result<EdgeT> {
        self.get_edge_count_from_edge_type_id(
            self.get_edge_type_id_from_edge_type_name(edge_type_name)?,
        )
    }

    /// Return node type ID curresponding to given node type name.
    ///
    /// If None is given as an node type ID, None is returned.
    ///
    /// # Arguments
    /// * `node_type_name`: &str - The node type name whose ID is to be returned.
    ///
    pub fn get_node_type_id_from_node_type_name(&self, node_type_name: &str) -> Result<NodeTypeT> {
        self.must_have_node_types()?;
        self.node_types
            .as_ref()
            .map(|nts| {
                nts.get(node_type_name).map_or_else(
                    || {
                        Err(
                            format!(
                            concat!(
                                "The given node type name {} does not exists in the current graph instance.\n",
                                "The supported node types are {:?}."
                            ),
                            node_type_name,
                            self.get_unique_node_type_names()
                        )
                    )
                    },
                    |node_type_id| Ok(node_type_id),
                )
            })
            .unwrap()
    }

    /// Return number of nodes with given node type ID.
    ///
    /// If None is given as an node type ID, the unknown node types
    /// will be returned.
    ///
    /// # Arguments
    /// * `node_type_id`: Option<NodeTypeT> - The node type ID to count the nodes of.
    ///
    pub fn get_node_count_from_node_type_id(
        &self,
        node_type_id: Option<NodeTypeT>,
    ) -> Result<NodeT> {
        self.validate_node_type_id(node_type_id)
            .map(|node_type_id| self.get_unchecked_node_count_from_node_type_id(node_type_id))
    }

    /// Return number of nodes with given node type name.
    ///
    /// If None is given as an node type name, the unknown node types
    /// will be returned.
    ///
    /// # Arguments
    /// * `node_type_name`: Option<&str> - The node type name to count the nodes of.
    ///
    pub fn get_node_count_from_node_type_name(
        &self,
        node_type_name: Option<&str>,
    ) -> Result<NodeT> {
        self.get_node_count_from_node_type_id(
            node_type_name.map_or(Ok::<_, String>(None), |ntn| {
                Ok(Some(self.get_node_type_id_from_node_type_name(ntn)?))
            })?,
        )
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
    /// # let graph = graph::test_utilities::load_ppi(true, true, true, true, false, false);
    /// # let node_id = 0;
    /// println!("The neighbours of the node {} are {:?}.", node_id, graph.get_neighbour_node_ids_from_node_id(node_id).unwrap());
    /// let unavailable_node = 2349765432;
    /// assert!(graph.get_neighbour_node_ids_from_node_id(unavailable_node).is_err());
    /// ```
    pub fn get_neighbour_node_ids_from_node_id(&self, node_id: NodeT) -> Result<Vec<NodeT>> {
        self.validate_node_id(node_id).map(|node_id| {
            unsafe { self.iter_unchecked_neighbour_node_ids_from_source_node_id(node_id) }.collect()
        })
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
    /// # let graph = graph::test_utilities::load_ppi(true, true, true, true, false, false);
    /// let node_name = "ENSP00000000233";
    /// println!("The neighbours of the node {} are {:?}.", node_name, graph.get_neighbour_node_ids_from_node_name(node_name).unwrap());
    /// ```
    pub fn get_neighbour_node_ids_from_node_name(&self, node_name: &str) -> Result<Vec<NodeT>> {
        self.get_neighbour_node_ids_from_node_id(self.get_node_id_from_node_name(node_name)?)
    }

    /// Return vector of destination names for the given source node name.
    ///
    /// # Arguments
    ///
    /// * `node_name`: &str - Node name whose neighbours are to be retrieved.
    ///
    /// # Example
    /// To retrieve the neighbours of a given node `src` you can use:
    ///
    /// ```rust
    /// # let graph = graph::test_utilities::load_ppi(true, true, true, true, false, false);
    /// let node_name = "ENSP00000000233";
    /// println!("The neighbours of the node {} are {:?}.", node_name, graph.get_neighbour_node_names_from_node_name(node_name).unwrap());
    /// ```
    pub fn get_neighbour_node_names_from_node_name(&self, node_name: &str) -> Result<Vec<String>> {
        Ok(unsafe {
            self.iter_unchecked_neighbour_node_names_from_source_node_id(
                self.get_node_id_from_node_name(node_name)?,
            )
        }
        .collect())
    }

    /// Return range of outbound edges IDs for all the edges bewteen the given
    /// source and destination nodes.
    /// This operation is meaningfull only in a multigraph.
    ///
    /// # Arguments
    ///
    /// * `src`: NodeT - Source node.
    /// * `dst`: NodeT - Destination node.
    ///
    pub fn get_minmax_edge_ids_from_node_ids(
        &self,
        src: NodeT,
        dst: NodeT,
    ) -> Result<(EdgeT, EdgeT)> {
        Ok((self.get_edge_id_from_node_ids(src, dst)?, unsafe {
            self.get_unchecked_edge_id_from_node_ids(src, dst + 1)
        }))
    }

    /// Return edge ID for given tuple of nodes and edge type.
    ///
    /// This method will return an error if the graph does not contain the
    /// requested edge with edge type.
    ///
    /// # Arguments
    /// * `src`: NodeT - Source node of the edge.
    /// * `dst`: NodeT - Destination node of the edge.
    /// * `edge_type`: Option<EdgeTypeT> - Edge Type of the edge.
    ///
    pub fn get_edge_id_from_node_ids_and_edge_type_id(
        &self,
        src: NodeT,
        dst: NodeT,
        edge_type: Option<EdgeTypeT>,
    ) -> Result<EdgeT> {
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
    /// * `src_name`: &str - Source node name of the edge.
    /// * `dst_name`: &str - Destination node name of the edge.
    ///
    pub fn get_edge_id_from_node_names(&self, src_name: &str, dst_name: &str) -> Result<EdgeT> {
        match (self.nodes.get(src_name), self.nodes.get(dst_name)) {
            (Some(src), Some(dst)) => self.get_edge_id_from_node_ids(src, dst).ok(),
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
    /// * `src_name`: &str - Source node name of the edge.
    /// * `dst_name`: &str - Destination node name of the edge.
    /// * `edge_type_name`: Option<&str> - Edge type name.
    ///
    pub fn get_edge_id_from_node_names_and_edge_type_name(
        &self,
        src_name: &str,
        dst_name: &str,
        edge_type_name: Option<&str>,
    ) -> Result<EdgeT> {
        match (self.nodes.get(src_name), self.nodes.get(dst_name)) {
            (Some(src), Some(dst)) => self
                .get_edge_id_from_node_ids_and_edge_type_id(
                    src,
                    dst,
                    self.get_edge_type_id_from_edge_type_name(edge_type_name)?,
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
    /// * `edge_type_names`: Vec<Option<String>> - Vector of edge types to be converted.
    pub fn get_edge_type_ids_from_edge_type_names(
        &self,
        edge_type_names: Vec<Option<String>>,
    ) -> Result<Vec<Option<EdgeTypeT>>> {
        edge_type_names
            .iter()
            .map(|edge_type_name| match edge_type_name {
                None => Ok(None),
                Some(edge_type_name) => {
                    self.get_edge_type_id_from_edge_type_name(Some(edge_type_name))
                }
            })
            .collect::<Result<Vec<Option<EdgeTypeT>>>>()
    }

    /// Return translated node types from string to internal node ID.
    ///
    /// # Arguments
    ///
    /// * `node_type_names`: Vec<Option<String>> - Vector of node types to be converted.
    pub fn get_node_type_ids_from_node_type_names(
        &self,
        node_type_names: Vec<Option<String>>,
    ) -> Result<Vec<Option<NodeTypeT>>> {
        self.must_have_node_types()?;
        node_type_names
            .iter()
            .map(|node_type_name| match node_type_name {
                None => Ok(None),
                Some(node_type_name) => self
                    .get_node_type_id_from_node_type_name(node_type_name)
                    .map(Some),
            })
            .collect::<Result<Vec<Option<NodeTypeT>>>>()
    }

    /// Return translated node types from string to internal node ID.
    ///
    /// # Arguments
    ///
    /// * `node_type_names`: Vec<Option<Vec<&str>>> - Vector of node types to be converted.
    ///
    /// # Raises
    /// * If the graph does not have node types.
    /// * If any of the given node type names do not exists in the graph.
    pub fn get_multiple_node_type_ids_from_node_type_names(
        &self,
        node_type_names: Vec<Option<Vec<&str>>>,
    ) -> Result<Vec<Option<Vec<NodeTypeT>>>> {
        self.must_have_node_types()?;
        node_type_names
            .iter()
            .map(|maybe_node_type_names| {
                maybe_node_type_names
                    .as_ref()
                    .map_or(Ok::<_, String>(None), |node_type_names| {
                        Ok(Some(
                            node_type_names
                                .iter()
                                .map(|node_type_name| {
                                    self.get_node_type_id_from_node_type_name(node_type_name)
                                })
                                .collect::<Result<Vec<NodeTypeT>>>()?,
                        ))
                    })
            })
            .collect::<Result<Vec<Option<Vec<NodeTypeT>>>>>()
    }

    /// Return range of outbound edges IDs which have as source the given Node.
    ///
    /// The method will panic if the given source node ID is higher than
    /// the number of nodes in the graph.
    ///
    /// # Arguments
    ///
    /// * `src`: NodeT - Node for which we need to compute the cumulative_node_degrees range.
    ///
    /// # Safety
    /// If the given node ID does not exist in the current graph the method will raise a panic.
    pub unsafe fn get_unchecked_minmax_edge_ids_from_source_node_id(
        &self,
        src: NodeT,
    ) -> (EdgeT, EdgeT) {
        match &self.cumulative_node_degrees {
            Some(cumulative_node_degrees) => {
                let min_edge_id = if src == 0 {
                    0
                } else {
                    cumulative_node_degrees[src as usize - 1]
                };
                (min_edge_id, cumulative_node_degrees[src as usize])
            }
            None => {
                let min_edge_id: EdgeT = self.get_unchecked_edge_id_from_node_ids(src, 0);
                (
                    min_edge_id,
                    self.get_unchecked_edge_id_from_node_ids(src + 1, 0),
                )
            }
        }
    }

    /// Return range of outbound edges IDs which have as source the given Node.
    ///
    /// # Arguments
    ///
    /// * `src`: NodeT - Node for which we need to compute the cumulative_node_degrees range.
    ///
    pub fn get_minmax_edge_ids_from_source_node_id(&self, src: NodeT) -> Result<(EdgeT, EdgeT)> {
        self.validate_node_id(src)
            .map(|src| unsafe { self.get_unchecked_minmax_edge_ids_from_source_node_id(src) })
    }

    /// Return node type name of given node type.
    ///
    /// There is no need for a unchecked version since we will have to map
    /// on the note_types anyway.
    ///
    /// # Arguments
    /// * `node_type_id`: NodeTypeT - Id of the node type.
    pub fn get_node_type_name_from_node_type_id(&self, node_type_id: NodeTypeT) -> Result<String> {
        self.must_have_node_types()?;
        self.node_types
            .as_ref()
            .map(|nts| nts.translate(node_type_id))
            .unwrap()
    }

    /// Return node type name of given node type.
    ///
    /// # Arguments
    /// * `node_type_ids`: Vec<NodeTypeT> - Id of the node type.
    ///
    /// # Safety
    /// The method will panic if the graph does not contain node types.
    pub unsafe fn get_unchecked_node_type_names_from_node_type_ids(
        &self,
        node_type_ids: Vec<NodeTypeT>,
    ) -> Vec<String> {
        self.node_types
            .as_ref()
            .map(|nts| nts.unchecked_translate_vector(node_type_ids))
            .unwrap_unchecked()
    }
}
