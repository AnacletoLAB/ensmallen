use super::*;
use counter::Counter;
use itertools::Itertools;
use rayon::prelude::*;
use std::collections::HashMap;
use std::iter::once;
use vec_rand::sorted_unique_sub_sampling;

impl Graph {
    /// Return name of the graph.
    ///
    /// # Example
    /// ```rust
    /// # let graph = graph::test_utilities::load_ppi(true, true, true, true, false, false).unwrap();
    /// println!("The name of the current graph is {}.", graph.get_name());
    /// ```
    ///
    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    /// Return the number of traps (nodes without any outgoing edges that are not singletons)
    ///
    /// # Example
    /// ```rust
    /// # let graph = graph::test_utilities::load_ppi(true, true, true, true, false, false).unwrap();
    /// println!("There are {} trap nodes in the current graph.", graph.get_traps_number());
    /// ```
    ///
    pub fn get_traps_number(&self) -> EdgeT {
        self.not_singleton_nodes_number as EdgeT - self.unique_sources.len() as EdgeT
    }

    // Return whether the graph has trap nodes.
    ///
    /// # Example
    /// ```rust
    /// # let graph = graph::test_utilities::load_ppi(true, true, true, true, false, false).unwrap();
    /// if graph.has_traps(){
    ///     println!("There are {} trap nodes in the current graph.", graph.get_traps_number());
    /// } else {
    ///     println!("There are no trap nodes in the current graph.");
    /// }
    /// ```
    ///
    pub fn has_traps(&self) -> bool {
        self.get_traps_number() > 0
    }

    /// Returns boolean representing if graph is directed.
    ///
    /// # Example
    /// ```rust
    /// let directed_string_ppi = graph::test_utilities::load_ppi(true, true, true, true, false, false).unwrap();
    /// assert!(directed_string_ppi.is_directed());
    /// let undirected_string_ppi = graph::test_utilities::load_ppi(true, true, true, false, false, false).unwrap();
    /// assert!(!undirected_string_ppi.is_directed());
    /// ```
    ///
    pub fn is_directed(&self) -> bool {
        self.directed
    }

    /// Returns boolean representing whether graph has weights.
    ///
    /// # Example
    /// ```rust
    /// let weights_string_ppi = graph::test_utilities::load_ppi(true, true, true, true, false, false).unwrap();
    /// assert!(weights_string_ppi.has_weights());
    /// let unweights_string_ppi = graph::test_utilities::load_ppi(true, true, false, true, false, false).unwrap();
    /// assert!(!unweights_string_ppi.has_weights());
    /// ```
    ///
    pub fn has_weights(&self) -> bool {
        self.weights.is_some()
    }

    /// Returns boolean representing whether graph has edge types.
    ///
    /// # Example
    /// ```rust
    /// let string_ppi_with_edge_types = graph::test_utilities::load_ppi(true, true, true, true, false, false).unwrap();
    /// assert!(string_ppi_with_edge_types.has_edge_types());
    /// let string_ppi_without_edge_types = graph::test_utilities::load_ppi(true, false, true, true, false, false).unwrap();
    /// assert!(!string_ppi_without_edge_types.has_edge_types());
    /// ```
    ///
    pub fn has_edge_types(&self) -> bool {
        self.edge_types.is_some()
    }

    /// Returns boolean representing if graph has self-loops.
    pub fn has_selfloops(&self) -> bool {
        self.self_loop_number > 0
    }

    /// Returns boolean representing if given node is a singleton.
    ///
    /// The following works for traps and singletons.
    /// TODO: THIS IS SOMETHING TO BE GENERALIZED FOR DIRECTED GRAPHS.
    ///
    /// # Arguments
    ///
    /// `node_id`: NodeT - The node to be checked for.
    pub fn is_singleton(&self, node_id: NodeT) -> Result<bool, String> {
        Ok(self.has_singletons() && self.get_node_degree(node_id)? == 0)
    }

    /// Returns boolean representing if given node is a singleton with self-loops.
    ///
    /// # Arguments
    ///
    /// `node_id`: NodeT - The node to be checked for.
    pub fn is_singleton_with_self_loops(&self, node_id: NodeT) -> bool {
        self.has_singleton_nodes_with_self_loops_number()
            && self.get_neighbours_iter(node_id).all(|dst| dst == node_id)
    }

    /// Returns boolean representing if given node is a singleton.
    ///
    /// # Arguments
    /// `node_name`: &str - The node name to be checked for.
    pub fn is_singleton_by_node_name(&self, node_name: &str) -> Result<bool, String> {
        self.is_singleton(self.get_node_id(node_name)?)
    }

    /// Returns boolean representing if graph has singletons.
    pub fn has_singletons(&self) -> bool {
        self.get_singleton_nodes_number() > 0
    }

    /// Returns boolean representing if graph has singletons.
    pub fn has_singleton_nodes_with_self_loops_number(&self) -> bool {
        self.get_singleton_nodes_with_self_loops_number() > 0
    }

    /// Return vector of the non-unique source nodes.
    ///
    /// # Arguments
    /// * `directed`: bool, whether to filter out the undirected edges.
    pub fn get_sources(&self, directed: bool) -> Vec<NodeT> {
        self.get_sources_par_iter(directed).collect()
    }

    /// Return vector of the non-unique source nodes names.
    ///
    /// # Arguments
    /// * `directed`: bool, whether to filter out the undirected edges.
    pub fn get_source_names(&self, directed: bool) -> Vec<String> {
        self.get_sources_par_iter(directed)
            .map(|src| self.get_node_name(src).unwrap())
            .collect()
    }

    /// Return vector on the (non unique) destination nodes of the graph.
    ///
    /// # Arguments
    /// * `directed`: bool, whether to filter out the undirected edges.
    pub fn get_destinations(&self, directed: bool) -> Vec<NodeT> {
        self.get_destinations_par_iter(directed).collect()
    }

    /// Return vector of the non-unique destination nodes names.
    ///
    /// # Arguments
    /// * `directed`: bool, whether to filter out the undirected edges.
    pub fn get_destination_names(&self, directed: bool) -> Vec<String> {
        self.get_destinations_par_iter(directed)
            .map(|dst| self.get_node_name(dst).unwrap())
            .collect()
    }

    /// Return vector with the sorted nodes names.
    pub fn get_node_names(&self) -> Vec<String> {
        self.nodes.reverse_map.clone()
    }

    /// Return vector with the sorted nodes Ids.
    pub fn get_nodes(&self) -> Vec<NodeT> {
        (0..self.get_nodes_number()).collect()
    }

    /// Return vector with top k central node Ids.
    ///
    /// # Arguments
    ///
    /// * k: NodeT - Number of central nodes to extract.
    pub fn get_top_k_central_nodes(&self, k: NodeT) -> Vec<NodeT> {
        let mut nodes_degrees: Vec<(NodeT, NodeT)> = (0..self.get_nodes_number())
            .map(|node_id| (self.get_node_degree(node_id).unwrap(), node_id))
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
        self.get_top_k_central_nodes(k)
            .iter()
            .cloned()
            .map(|node_id| self.get_node_name(node_id).unwrap())
            .collect()
    }

    /// Return the edge types of the edges.
    pub fn get_edge_types(&self) -> Result<Vec<Option<EdgeTypeT>>, String> {
        if !self.has_edge_types(){
            return Err("The current graph instance does not have edge types!".to_string())
        }
        Ok(self.edge_types.as_ref().map(|ets| ets.ids.clone()).unwrap())
    }

    /// Return edge type name of given edge type.
    ///
    /// # Arguments
    /// * edge_type_id: EdgeTypeT - Id of the edge type.
    pub fn get_edge_type_name(&self, edge_type_id: EdgeTypeT) -> Result<&String, String> {
        self.edge_types
            .as_ref()
            .map_or(
                Err("Edge types not available for the current graph instance.".to_string()),
                |ets| ets.translate(edge_type_id))
        
    }

    /// Return the edge types names.
    pub fn get_edge_type_names(&self) -> Option<Vec<String>> {
        self.edge_types
            .as_ref()
            .map(|ets| ets.vocabulary.reverse_map.clone())
    }

    /// Return the node types of the nodes.
    pub fn get_node_types(&self) -> Result<Vec<Option<Vec<NodeTypeT>>>, String> {
        if !self.has_node_types(){
            return Err("The current graph instance does not have nodes!".to_string())
        }
        Ok(self.node_types.as_ref().map(|nts| nts.ids.clone()).unwrap())
    }

    /// Return node type name of given node type.
    ///
    /// # Arguments
    /// * node_type_id: Vec<NodeTypeT> - Id of the node type.
    pub fn translate_node_type_id(&self, node_type_id: NodeTypeT) -> Result<&String, String> {
        self.node_types
            .as_ref()
            .map_or(
                Err("Node types not available for the current graph instance.".to_string()),
                |nts| nts.translate(node_type_id)
            )
    }

    /// Return node type name of given node type.
    ///
    /// # Arguments
    /// * node_type_id: Vec<NodeTypeT> - Id of the node type.
    pub fn translate_node_type_id_vector(
        &self,
        node_type_id: Vec<NodeTypeT>,
    ) -> Result<Vec<String>, String> {
        self.node_types.as_ref().map_or(
            Err("Node types not available for the current graph instance.".to_string()), 
            |nts| {
                Ok(nts.translate_vector(node_type_id)?
                    .into_iter()
                    .map(String::to_owned)
                    .collect())
        })
    }

    /// Return the weights of the nodes.
    pub fn get_weights(&self) -> Result<Vec<WeightT>, String> {
        if !self.has_weights(){
            return Err("The current graph instance does not have weights!".to_string())
        }
        Ok(self.weights.clone().unwrap())
    }

    /// Return the node types names.
    pub fn get_node_type_names(&self) -> Option<Vec<String>> {
        self.node_types
            .as_ref()
            .map(|nts| nts.vocabulary.reverse_map.clone())
    }

    /// Return number of the unique edges in the graph.
    pub fn get_unique_directed_edges_number(&self) -> EdgeT {
        self.unique_edges_number
    }

    /// Return maximum encodable edge number.
    pub fn get_max_encodable_edge_number(&self) -> EdgeT {
        encode_max_edge(
            self.get_nodes_number(),
            get_node_bits(self.get_nodes_number()),
        )
    }

    /// Return the nodes mapping.
    pub fn get_nodes_mapping(&self) -> HashMap<String, NodeT> {
        self.nodes.map.clone()
    }

    /// Return vector with the sorted edge Ids.
    pub fn get_edges(&self, directed: bool) -> Vec<Vec<NodeT>> {
        self.get_edges_par_iter(directed)
            .map(|(_, src, dst)| vec![src, dst])
            .collect()
    }

    /// Return vector with the sorted edge names.
    pub fn get_edge_names(&self, directed: bool) -> Vec<(String, String)> {
        self.get_edges_par_string_iter(directed)
            .map(|(_, src, dst)| (src, dst))
            .collect()
    }

    /// Returns option with the edge type of the given edge id.
    pub(crate) fn get_unchecked_edge_type(&self, edge_id: EdgeT) -> Option<EdgeTypeT> {
        self.edge_types.as_ref().and_then(|ets| ets.ids[edge_id as usize])
    }

    /// Returns option with the weight of the given edge id.
    pub(crate) fn get_unchecked_edge_weight(&self, edge_id: EdgeT) -> Option<WeightT> {
        self.weights.as_ref().map(|ws| ws[edge_id as usize])
    }

    /// Returns option with the node type of the given node id.
    pub(crate) fn get_unchecked_node_type_id_by_node_id(&self, node_id: NodeT) -> Option<Vec<NodeTypeT>> {
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
    /// # Examples
    /// ```rust
    /// # let graph = graph::test_utilities::load_ppi(true, true, true, true, false, false).unwrap();
    /// println!("The node type id of node {} is {:?}", 0, graph.get_node_type_id_by_node_id(0));
    /// ```
    ///
    pub fn get_node_type_id_by_node_id(&self, node_id: NodeT) -> Result<Option<Vec<NodeTypeT>>, String> {
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
    /// println!("The edge type id of edge {} is {:?}", 0, graph.get_edge_type(0));
    /// ```
    pub fn get_edge_type(&self, edge_id: EdgeT) -> Result<Option<EdgeTypeT>, String> {
        if let Some(et) = &self.edge_types {
            return if edge_id <= et.ids.len() as EdgeT {
                Ok(self.get_unchecked_edge_type(edge_id))
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

    /// Returns option with the node type of the given node id.
    pub fn get_node_type_name(&self, node_id: NodeT) -> Result<Option<Vec<String>>, String> {
        match &self.node_types.is_some() {
            true => Ok(match self.get_unchecked_node_type_id_by_node_id(node_id) {
                Some(node_type_id) => Some(self.translate_node_type_id_vector(node_type_id)?),
                None => None,
            }),
            false => Err("Node types not available for the current graph instance.".to_string()),
        }
    }

    /// Returns option with the edge type of the given edge id.
    /// TODO: complete docstring and add example!
    /// TODO: THIS SHOULD RETURN A RESULT!
    pub fn get_edge_type_name_by_edge_id(&self, edge_id: EdgeT) -> Option<String> {
        self.edge_types.as_ref().and_then(|ets| {
            self.get_unchecked_edge_type(edge_id)
                .map(|et| ets.unchecked_translate(et))
        })
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
    /// assert!(weighted_graph.get_weight_by_edge_id(edge_id).is_ok());
    /// assert!(weighted_graph.get_weight_by_edge_id(unexistent_edge_id).is_err());
    /// assert!(unweighted_graph.get_weight_by_edge_id(edge_id).is_err());
    /// ```
    pub fn get_weight_by_edge_id(&self, edge_id: EdgeT) -> Result<WeightT, String> {
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
    /// assert!(weighted_graph.get_weight_by_node_ids(src, dst).is_ok());
    /// ```
    pub fn get_weight_by_node_ids(&self, src: NodeT, dst: NodeT) -> Result<WeightT, String> {
        self.get_weight_by_edge_id(self.get_edge_id_by_node_ids(src, dst)?)
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
    /// assert!(weighted_graph.get_weight_with_type_by_node_ids(src, dst, edge_type).is_ok());
    /// ```
    pub fn get_weight_with_type_by_node_ids(&self, src: NodeT, dst: NodeT, edge_type: Option<EdgeTypeT>) -> Result<WeightT, String> {
        self.get_weight_by_edge_id(self.get_edge_id_with_type_by_node_ids(src, dst, edge_type)?)
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
    /// assert!(weighted_graph.get_weight_with_type_by_node_names(src, dst, edge_type.as_ref()).is_ok());
    /// ```
    pub fn get_weight_with_type_by_node_names(&self, src: &str, dst: &str, edge_type: Option<&String>) -> Result<WeightT, String> {
        self.get_weight_by_edge_id(self.get_edge_id_with_type_by_node_names(src, dst, edge_type)?)
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
    /// assert!(weighted_graph.get_weight_by_node_names(src_name, dst_name).is_ok());
    /// ```
    pub fn get_weight_by_node_names(&self, src_name: &str, dst_name: &str) -> Result<WeightT, String> {
        self.get_weight_by_edge_id(self.get_edge_id_by_node_names(src_name, dst_name)?)
    }

    /// Returns result with the node name.
    pub fn get_node_name(&self, node_id: NodeT) -> Result<String, String> {
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
    pub fn get_node_id(&self, node_name: &str) -> Result<NodeT, String> {
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
    /// println!("The node type ID of node {} is {:?}.", node_name, graph.get_node_type_id_by_node_name(node_name).unwrap());
    /// ```
    pub fn get_node_type_id_by_node_name(&self, node_name: &str) -> Result<Option<Vec<NodeTypeT>>, String> {
        self.get_node_type_id_by_node_id(self.get_node_id(node_name)?)
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
    /// println!("The node type of node {} is {:?}", node_name, graph.get_node_type_name_by_node_name(node_name).unwrap());
    /// ```
    pub fn get_node_type_name_by_node_name(&self, node_name: &str) -> Result<Option<Vec<String>>, String> {
        self.get_node_type_name(self.get_node_id(node_name)?)
    }

    /// Returns whether the graph has the given node name.
    ///
    /// # Arguments
    /// 
    /// * `node_name`: &str - Name of the node.
    ///
    /// # Examples
    /// To check if a node appears in the graph you can use:
    /// ```rust
    /// # let graph = graph::test_utilities::load_ppi(true, true, true, true, false, false).unwrap();
    /// let node_name = "ENSP00000000233";
    /// let unexistent_node_name = "I_do_not_exist!";
    /// assert!(graph.has_node_by_name(node_name));
    /// assert!(!graph.has_node_by_name(unexistent_node_name));
    /// ```
    pub fn has_node_by_name(&self, node_name: &str) -> bool {
        self.get_node_id(node_name).is_ok()
    }

    /// Returns node id raising a panic if used unproperly.
    pub fn get_unchecked_node_id(&self, node_name: &str) -> NodeT {
        *self.nodes.get(node_name).unwrap()
    }

    /// Returns edge type id.
    /// TODO: CHECK IF THIS THING SHOULD BE PUBLIC!
    pub fn get_unchecked_edge_type_id(&self, edge_type: Option<&str>) -> Option<EdgeTypeT> {
        match (&self.edge_types, edge_type) {
            (Some(ets), Some(et)) => ets.get(et).copied(),
            _ => None,
        }
    }

    /// Returns option with the weight of the given edge id.
    /// TODO: CHECK IF THIS THING SHOULD BE PUBLIC!
    pub fn get_edge_weight(&self, edge_id: EdgeT) -> Option<WeightT> {
        self.get_unchecked_edge_weight(edge_id)
    }

    /// Returns boolean representing if graph has node types.
    pub fn has_node_types(&self) -> bool {
        self.node_types.is_some()
    }

    /// Returns boolean representing if graph has multilabel node types.
    pub fn has_multilabel_node_types(&self) -> bool {
        self.node_types
            .as_ref()
            .map_or(false, |nt| nt.is_multilabel())
    }

    /// Returns number of unknown node types.
    pub fn get_unknown_node_types_number(&self) -> NodeT {
        self.node_types
            .as_ref()
            .map_or(0, |nt| nt.get_unknown_count())
    }

    /// Returns minimum number of node types.
    pub fn get_minimum_node_types_number(&self) -> NodeT {
        self.node_types
            .as_ref()
            .map_or(0, |et| et.min_node_type_count())
    }

    /// Returns whether there are unknown node types.
    pub fn has_unknown_node_types(&self) -> bool {
        self.get_unknown_node_types_number() > 0
    }

    /// Returns number of unknown edge types.
    pub fn get_unknown_edge_types_number(&self) -> EdgeT {
        self.edge_types
            .as_ref()
            .map_or(0, |et| et.get_unknown_count())
    }

    /// Returns minimum number of edge types.
    pub fn get_minimum_edge_types_number(&self) -> EdgeT {
        self.edge_types
            .as_ref()
            .map_or(0, |et| et.min_edge_type_count())
    }

    /// Returns whether there are unknown edge types.
    pub fn has_unknown_edge_types(&self) -> bool {
        self.get_unknown_edge_types_number() > 0
    }

    /// Returns number of nodes in the graph.
    pub fn get_nodes_number(&self) -> NodeT {
        self.nodes.len() as NodeT
    }

    /// Return a vector with the components each node belongs to.
    ///
    /// E.g. If we have two components [0, 2, 3] and [1, 4, 5] the result will look like
    /// [0, 1, 0, 0, 1, 1]
    ///
    /// # Arguments
    /// * `verbose`: bool - wether to show the loading bar.
    pub fn get_node_components_vector(&self, verbose: bool) -> Vec<NodeT> {
        match self.directed {
            true => self.spanning_arborescence_kruskal(verbose).1,
            false => self.connected_components(verbose).unwrap().0,
        }
    }

    /// Returns number of directed edges in the graph.
    pub fn get_directed_edges_number(&self) -> EdgeT {
        self.edges.len() as EdgeT
    }

    /// Returns number of edge types in the graph.
    pub fn get_edge_types_number(&self) -> EdgeTypeT {
        self.edge_types.as_ref().map_or(0, |ets| ets.len() as EdgeTypeT)
    }

    /// Returns number of node types in the graph.
    pub fn get_node_types_number(&self) -> NodeTypeT {
        self.node_types.as_ref().map_or(0, |nts| nts.len() as NodeTypeT)
    }

    /// Returns the degree of every node in the graph.
    pub fn get_node_degrees(&self) -> Vec<NodeT> {
        (0..self.get_nodes_number())
            .into_par_iter()
            .map(|node| self.get_node_degree(node as NodeT).unwrap())
            .collect::<Vec<NodeT>>()
    }

    /// Return set of nodes that are not singletons.
    pub fn get_not_singletons(&self) -> Vec<NodeT> {
        self.get_edges_iter(false)
            .flat_map(|(_, src, dst)| once(src).chain(once(dst)))
            .unique()
            .collect()
    }

    /// Return mapping from instance not trap nodes to dense nodes.
    pub fn get_dense_node_mapping(&self) -> HashMap<NodeT, NodeT> {
        self.get_not_singletons()
            .iter()
            .cloned()
            .enumerate()
            .map(|(i, node)| (node as NodeT, i as NodeT))
            .collect()
    }

    /// Return number of edges of the given edge type without checks.
    /// 
    /// # Arguments
    /// 
    /// * edge_type: Option<EdgeTypeT> - The edge type to retrieve count of.
    ///
    pub(crate) fn get_unchecked_edge_count_by_edge_type(&self, edge_type: Option<EdgeTypeT>) -> EdgeT {
        match (&self.edge_types, edge_type) {
            (Some(ets), None) => ets.get_unknown_count(),
            (Some(ets), Some(et)) => ets.counts[et as usize],
            _ => unreachable!("The current graph instance does not have edge types!")
        }
    }

    /// Return number of nodes of the given node type without checks.
    /// 
    /// # Arguments
    /// 
    /// * node_type: Option<NodeTypeT> - The node type to retrieve count of.
    ///
    pub(crate) fn get_unchecked_node_count_by_node_type(&self, node_type: Option<NodeTypeT>) -> NodeT {
        match (&self.node_types, node_type) {
            (Some(nts), None) => nts.get_unknown_count(),
            (Some(nts), Some(nt)) => nts.counts[nt as usize],
            _ => unreachable!("The current graph instance does not have node types!")
        }
    }

    pub fn get_edge_count_by_edge_type(&self, edge_type: Option<EdgeTypeT>) -> Result<EdgeT, String> {
        if !self.has_edge_types() {
            return Err("Current graph does not have edge types!".to_owned());
        }
        if let Some(et) = &edge_type{
            if self.get_edge_types_number() <= *et {
                return Err(format!(
                    "Given edge type ID {} is bigger than number of edge types in the graph {}.",
                    self.get_edge_types_number(),
                    et
                ));
            }
        }
        Ok(self.get_unchecked_edge_count_by_edge_type(edge_type))
    }

    pub fn get_edge_type_id(&self, edge_type_name: Option<&str>) -> Result<Option<EdgeTypeT>, String> {
        match (&self.edge_types, edge_type_name) {
            (None, _) => Err("Current graph does not have edge types.".to_owned()),
            (Some(_), None) => Ok(None),
            (Some(ets), Some(etn)) => {
                match ets.get(etn) {
                    Some(edge_type_id) => Ok(Some(*edge_type_id)),
                    None => Err(format!(
                        "Given edge type name {} is not available in current graph.",
                        etn
                    )),
                }
            }
        }
    }

    pub fn get_edge_count_by_edge_type_name(&self, edge_type: Option<&str>) -> Result<EdgeT, String> {
        self.get_edge_count_by_edge_type(self.get_edge_type_id(edge_type)?)
    }

    pub fn get_node_type_id(&self, node_type_name: &str) -> Result<NodeTypeT, String> {
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

    pub fn get_node_count_by_node_type(&self, node_type: Option<NodeTypeT>) -> Result<NodeT, String> {
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
        Ok(self.get_unchecked_node_count_by_node_type(node_type))
    }

    pub fn get_node_count_by_node_type_name(&self, node_type_name: Option<&str>) -> Result<NodeT, String> {
        self.get_node_count_by_node_type(node_type_name.map_or(
            Ok::<_, String>(None), 
            |ntn| Ok(Some(self.get_node_type_id(ntn)?))
        )?)
    }

    /// Return if there are multiple edges between two nodes
    pub fn is_multigraph(&self) -> bool {
        self.get_multigraph_edges_number() > 0
    }

    /// Return number of edges that have multigraph syblings.
    pub fn get_multigraph_edges_number(&self) -> EdgeT {
        self.get_directed_edges_number() - self.unique_edges_number
    }

    pub fn get_outbounds(&self) -> Vec<EdgeT> {
        (0..self.get_nodes_number())
            .into_par_iter()
            .map(|src| self.get_unchecked_edge_id_from_tuple(src as NodeT + 1, 0))
            .collect()
    }

    /// TODO: add unchecked version of this method!
    /// TODO: add docstring and example!
    pub fn get_destination(&self, edge_id: EdgeT) -> Result<NodeT, String> {
        if edge_id >= self.get_directed_edges_number(){
            return Err(format!(
                "The edge ID {} is higher than the number of available directed edges {}.",
                edge_id,
                self.get_directed_edges_number()
            ));
        }
        Ok(match &self.destinations {
            Some(destinations) => destinations[edge_id as usize],
            None => self.get_node_ids_from_edge_id(edge_id).1,
        })
    }

    /// TODO: add docstring
    /// TODO: check how to uniform this method with the other similar ones!
    pub(crate) fn get_destinations_range(
        &self,
        min_edge_id: EdgeT,
        max_edge_id: EdgeT,
    ) -> impl Iterator<Item = NodeT> + '_ {
        (min_edge_id..max_edge_id).map(move |edge_id| self.get_destination(edge_id).unwrap())
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
    /// println!("The neighbours of the node {} are {:?}.", node_id, graph.get_node_neighbours_by_node_id(node_id).unwrap());
    /// let unavailable_node = 2349765432;
    /// assert!(graph.get_node_neighbours_by_node_id(unavailable_node).is_err());
    /// ```
    pub fn get_node_neighbours_by_node_id(&self, node_id: NodeT) -> Result<Vec<NodeT>, String> {
        if node_id >= self.get_nodes_number(){
            return Err(format!(
                "The node ID {} is higher than the number of available nodes {}.",
                node_id,
                self.get_nodes_number()
            ));
        }
        Ok(self.get_unchecked_destinations_range(node_id)
        .map(move |edge_id| self.get_destination(edge_id).unwrap()).collect())
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
    /// let node_id = 0;
    /// println!("The neighbours of the node {} are {:?}.", node_id, graph.get_node_neighbours_by_node_id(node_id).unwrap());
    /// ```
    pub fn get_node_neighbours_by_node_name(&self, node_name: &str) -> Result<Vec<NodeT>, String> {
        self.get_node_neighbours_by_node_id(self.get_node_id(node_name)?)
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
    /// println!("The neighbours of the node {} are {:?}.", node_name, graph.get_node_neighbours_name_by_node_name(node_name).unwrap());
    /// ```
    pub fn get_node_neighbours_name_by_node_name(&self, node_name: &str) -> Result<Vec<String>, String> {
        Ok(self.get_neighbours_names_iter(self.get_node_id(node_name)?).collect())
    }

    pub(crate) fn get_node_edges_and_destinations(
        &self,
        max_neighbours: Option<NodeT>,
        random_state: u64,
        node: NodeT,
    ) -> (EdgeT, EdgeT, Option<Vec<NodeT>>, Option<Vec<u64>>) {
        // We retrieve the range of edge ids, the minimum and maximum value.
        let (min_edge_id, max_edge_id) = self.get_destinations_min_max_edge_ids(node);

        // We check if subsampling is enabled and if so, if it makes sense:
        // that is, if the range of neighbours (max_edge_id-min_edge_id) is smaller
        // than the required sub-sampling we do not use it as it would be useless.
        if let Some(indices) = max_neighbours.and_then(|mn| {
            sorted_unique_sub_sampling(min_edge_id, max_edge_id, mn as u64, random_state).ok()
        }) {
            let destinations: Vec<NodeT> = match self
                .cached_destinations
                .as_ref()
                .and_then(|cds| cds.get(&node))
            {
                Some(dsts) => indices
                    .iter()
                    .map(|edge_id| dsts[(*edge_id - min_edge_id) as usize])
                    .collect(),
                None => indices
                    .iter()
                    .map(|edge_id| self.get_destination(*edge_id).unwrap())
                    .collect(),
            };
            return (min_edge_id, max_edge_id, Some(destinations), Some(indices));
        }

        // If the destinations are stored explicitly because the time-memory tradeoff is enabled we are done.
        if self.destinations.is_some() {
            return (min_edge_id, max_edge_id, None, None);
        }

        // Finally if we are using the cache without sub-sampling
        let destinations = match self
            .cached_destinations
            .as_ref()
            .map_or(false, |cds| cds.contains_key(&node))
        {
            true => None,
            false => Some(
                self.get_destinations_range(min_edge_id, max_edge_id)
                    .collect(),
            ),
        };
        (min_edge_id, max_edge_id, destinations, None)
    }

    pub(crate) fn get_destinations_slice<'a>(
        &'a self,
        min_edge_id: EdgeT,
        max_edge_id: EdgeT,
        node: NodeT,
        destinations: &'a Option<Vec<NodeT>>,
    ) -> &'a [NodeT] {
        match (&self.destinations, &self.cached_destinations, destinations) {
            (_, _, Some(dsts)) => &dsts.as_slice(),
            (Some(dsts), None, None) => &dsts[min_edge_id as usize..max_edge_id as usize],
            (None, Some(dsts), None) => dsts.get(&node).unwrap(),
            _ => unreachable!(
                "It is not possible to have both destinations and cached destinations at once."
            ),
        }
    }

    /// Return (subsampled) vector of destinations of given node.
    /// 
    /// If the max neighbours parameter is given, and is smaller than the 
    /// number of the neighbours of the given node, the subsampling
    /// mechanism is given.
    /// 
    /// # Arguments
    /// `node`: NodeT - Node whose neighbours are to return.
    /// `random_state`: u64 - Random state to subsample neighbours.
    /// `max_neighbours`: &Option<NodeT> - Optionally number of neighbours to consider.
    pub(crate) fn get_node_destinations(
        &self,
        node: NodeT,
        random_state: u64,
        max_neighbours: Option<NodeT>,
    )->Vec<NodeT>{
        let (min_edge_id, max_edge_id, destinations, _) =
            self.get_node_edges_and_destinations(max_neighbours, random_state, node);
        self.get_destinations_slice(min_edge_id, max_edge_id, node, &destinations).to_owned()
    }

    /// Return number of unique source nodes number.
    /// 
    /// # Example
    /// ```rust
    /// # let graph = graph::test_utilities::load_ppi(true, true, true, true, false, false).unwrap();
    /// println!("The graph has {} unique source nodes.", graph.get_unique_sources_number());
    /// ```
    pub fn get_unique_sources_number(&self) -> NodeT {
        self.unique_sources.len() as NodeT
    }

    /// Returns number of the source nodes.
    ///```rust
    /// # let graph = graph::test_utilities::load_ppi(true, true, true, true, false, false).unwrap();
    /// println!("The number of sources of the graph (not trap nodes) is {}", graph.get_source_nodes_number());
    /// ```
    pub fn get_source_nodes_number(&self) -> NodeT {
        self.unique_sources.len() as NodeT
    }

    /// Return edge ID without any checks for given tuple of nodes and edge type.
    /// 
    /// This method will cause a panic if used improperly when it is not certain
    /// that the edge exists.
    ///
    /// # Arguments
    /// `src`: NodeT - Source node of the edge. 
    /// `dst`: NodeT - Destination node of the edge.
    /// `edge_type`: Option<EdgeTypeT> - Edge Type of the edge.
    pub(crate) fn get_unchecked_edge_id(
        &self,
        src: NodeT,
        dst: NodeT,
        edge_type: Option<EdgeTypeT>,
    ) -> EdgeT {
        self.edge_types.as_ref().map_or_else(
            || self.get_unchecked_edge_id_from_tuple(src, dst),
            |ets| self
            .get_unchecked_edge_ids_range(src, dst)
            // The vectors of the edge types can only have one element.
            .find(|edge_id| ets.ids[*edge_id as usize] == edge_type)
            .unwrap()
        )
    }

    /// Return edge ID without any checks for given tuple of nodes and edge type.
    /// 
    /// This method will return an error if the graph does not contain the
    /// requested edge with edge type.
    ///
    /// # Arguments
    /// `src`: NodeT - Source node of the edge. 
    /// `dst`: NodeT - Destination node of the edge.
    /// `edge_type`: Option<EdgeTypeT> - Edge Type of the edge.
    ///
    pub fn get_edge_id_with_type_by_node_ids(
        &self,
        src: NodeT,
        dst: NodeT,
        edge_type: Option<EdgeTypeT>,
    ) -> Result<EdgeT, String> {
        let edge_id = self.edge_types.as_ref().map_or_else(|| self.get_edge_id_by_node_ids(src, dst).ok(), |ets| self.get_edge_ids(src, dst).and_then(|mut edge_ids| {
            edge_ids.find(|edge_id| ets.ids[*edge_id as usize] == edge_type)
        }));
        // TODO: change using a map_err!
        match edge_id{
            Some(e) => Ok(e),
            None => Err(
                format!(
                    concat!(
                        "The current graph instance does not contain the required edge composed of ",
                        "source node ID {}, destination node ID {} and edge ID {:?}."
                    ),
                    src, dst, edge_type
                )
            )
        }
    }

    // TODO: add docstring and example!
    pub fn has_edge(&self, src: NodeT, dst: NodeT) -> bool {
        self.get_edge_id_by_node_ids(src, dst).is_ok()
    }


    /// Returns boolean representing if edge passing between given nodes exists.
    ///
    /// # Arguments
    ///
    /// * src: NodeT - The source node of the edge.
    /// * dst: NodeT - The destination node of the edge.
    /// * edge_type: Option<EdgeTypeT> - The (optional) edge type.
    ///
    /// TODO: add example!
    pub fn has_edge_with_type(&self, src: NodeT, dst: NodeT, edge_type: Option<EdgeTypeT>) -> bool {
        self.get_edge_id_with_type_by_node_ids(src, dst, edge_type).is_ok()
    }

    // TODO: add docstring and example!
    pub fn get_edge_id_by_node_names(
        &self,
        src_name: &str,
        dst_name: &str
    ) -> Result<EdgeT, String> {
        // TODO REFACTOR CODE to be cleaner!
        let edge_id = if let (Some(src), Some(dst)) = (self.nodes.get(src_name), self.nodes.get(dst_name)) {
            self.get_edge_id_by_node_ids(*src, *dst).ok()
        } else {
            None
        };
        match edge_id {
            Some(e) => Ok(e),
            None => Err(
                format!(
                    concat!(
                        "The current graph instance does not contain the required edge composed of ",
                        "source node name {} and destination node name {}."
                    ),
                    src_name, dst_name
                )
            )
        }
    }

    // TODO: add docstring and example!
    pub fn has_edge_by_node_names(
        &self,
        src_name: &str,
        dst_name: &str
    ) -> bool {
        self.get_edge_id_by_node_names(src_name, dst_name).is_ok()
    }

    // TODO: add docstring and example!
    pub fn get_edge_id_with_type_by_node_names(
        &self,
        src_name: &str,
        dst_name: &str,
        edge_type_name: Option<&String>,
    ) -> Result<EdgeT, String> {
        if let (Some(src), Some(dst)) = (self.nodes.get(src_name), self.nodes.get(dst_name)) {
            self.get_edge_id_with_type_by_node_ids(*src, *dst, self.get_edge_type_id(edge_type_name.map(|x| x.as_str()))?)
        } else {
            Err(
                format!(
                    concat!(
                        "The current graph instance does not contain the required edge composed of ",
                        "source node name {}, destination node name {} and edge name {:?}."
                    ),
                    src_name, dst_name, edge_type_name
                )
            )
        }
    }

    /// Returns edge type counts.
    ///
    /// # Arguments
    ///
    /// None
    ///
    /// # Examples
    /// ```rust
    /// # let graph = graph::test_utilities::load_ppi(true, true, true, true, false, false).unwrap();
    /// for (edge_type_id, count) in graph.get_edge_type_counts().unwrap().iter() {
    ///     println!("edge type id {}: count: {}", edge_type_id, count);
    /// }
    /// ```
    pub fn get_edge_type_counts(&self) -> Result<Counter<EdgeTypeT, usize>, String> {
        if let Some(et) = &self.edge_types {
            Ok(Counter::init(
                et.ids.iter().filter_map(|edge_type| *edge_type),
            ))
        } else {
            Err(String::from(
                "Edge types are not defined for current graph instance.",
            ))
        }
    }

    /// Returns edge type counts hashmap.
    ///
    /// # Arguments
    ///
    /// None
    ///
    /// # Examples
    /// ```rust
    /// # let graph = graph::test_utilities::load_ppi(true, true, true, true, false, false).unwrap();
    /// for (edge_type_id, count) in graph.get_edge_type_counts().unwrap().iter() {
    ///     println!("edge type id {}: count: {}", edge_type_id, count);
    /// }
    /// ```
    pub fn get_edge_type_counts_hashmap(&self) -> Result<HashMap<EdgeTypeT, usize>, String> {
        Ok(self.get_edge_type_counts()?.into_map())
    }

    /// Return translated edge types from string to internal edge ID.
    ///
    /// # Arguments
    ///
    /// * `edge_types`: Vec<String> - Vector of edge types to be converted.
    pub fn translate_edge_types(
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
    pub fn translate_node_types(&self, node_types: Vec<Option<String>>) -> Result<Vec<Option<NodeTypeT>>, String> {
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

    /// Returns node type counts.
    ///
    /// # Arguments
    ///
    /// None
    ///
    /// ```rust
    /// # let graph = graph::test_utilities::load_ppi(true, true, true, true, false, false).unwrap();
    /// for (node_type_id, count) in graph.get_node_type_counts().unwrap().iter() {
    ///     println!("node type id {}: count: {}", node_type_id, count);
    /// }
    /// ```
    pub fn get_node_type_counts(&self) -> Result<Counter<NodeTypeT, usize>, String> {
        if let Some(nt) = &self.node_types {
            Ok(Counter::init(
                nt.ids
                    .iter()
                    .filter_map(|node_type| node_type.clone())
                    .flatten(),
            ))
        } else {
            Err(String::from(
                "Node types are not defined for current graph instance.",
            ))
        }
    }

    /// Returns node type counts hashmap.
    ///
    /// # Arguments
    ///
    /// None
    ///
    /// ```rust
    /// # let graph = graph::test_utilities::load_ppi(true, true, true, true, false, false).unwrap();
    /// for (node_type_id, count) in graph.get_node_type_counts().unwrap().iter() {
    ///     println!("node type id {}: count: {}", node_type_id, count);
    /// }
    /// ```
    pub fn get_node_type_counts_hashmap(&self) -> Result<HashMap<EdgeTypeT, usize>, String> {
        Ok(self.get_node_type_counts()?.into_map())
    }

    /// Returns boolean representing if edge passing between given nodes exists.
    ///
    /// # Arguments
    ///
    /// * src: String - The source node name of the edge.
    /// * dst: String - The destination node name of the edge.
    /// * edge_type: Option<String> - The (optional) edge type name.
    ///
    pub fn has_edge_with_type_by_node_names(
        &self,
        src_name: &str,
        dst_name: &str,
        edge_type_name: Option<&String>,
    ) -> bool {
        self.get_edge_id_with_type_by_node_names(src_name, dst_name, edge_type_name)
            .is_ok()
    }

    /// Returns boolean representing if node with given name and node type name exists in current graph.
    ///
    /// # Arguments
    ///
    /// * node_name: String - The node name.
    /// * node_type_name: String - The node type name.
    ///
    pub fn has_node_with_type_by_name(&self, node_name: &str, node_type_name: Option<Vec<String>>) -> bool {
        match self.get_node_id(node_name) {
            Err(_) => false,
            Ok(node_id) => {
                let our_node_types = self.get_node_type_name(node_id);
                match (our_node_types, node_type_name) {
                    (Err(_), None) => true,
                    (Ok(None), None) => true,
                    (Ok(Some(mut our_nts)), Some(mut other_nts)) => {
                        our_nts.sort();
                        other_nts.sort();
                        our_nts == other_nts
                    }
                    _ => false,
                }
            }
        }
    }


    /// Return range of outbound edges IDs for given Node.
    ///
    /// # Arguments
    ///
    /// * src: NodeT - Source node.
    /// * dst: NodeT - Destination node.
    ///
    pub(crate) fn get_unchecked_edge_types_min_max_edge_ids(
        &self,
        src: NodeT,
        dst: NodeT,
    ) -> (EdgeT, EdgeT) {
        (
            self.get_unchecked_edge_id_from_tuple(src, dst),
            self.get_unchecked_edge_id_from_tuple(src, dst + 1),
        )
    }

    /// Return range of outbound edges IDs for given Node.
    ///
    /// # Arguments
    ///
    /// * src: NodeT - Source node.
    /// * dst: NodeT - Destination node.
    ///
    pub(crate) fn get_unchecked_edge_types_number_from_tuple(
        &self,
        src: NodeT,
        dst: NodeT,
    ) -> EdgeT {
        let (min_edge_id, max_edge_id) = self.get_unchecked_edge_types_min_max_edge_ids(src, dst);
        max_edge_id - min_edge_id
    }

    /// Return range of outbound edges IDs for given Node.
    ///
    /// # Arguments
    ///
    /// * src: NodeT - Source node.
    /// * dst: NodeT - Destination node.
    ///
    pub(crate) fn get_edge_types_min_max_edge_ids(
        &self,
        src: NodeT,
        dst: NodeT,
    ) -> Option<(EdgeT, EdgeT)> {
        self.get_edge_id_by_node_ids(src, dst).ok().map(
            |min_edge|
            (min_edge, self.get_unchecked_edge_id_from_tuple(src, dst + 1))
        )
    }

    /// Return range of outbound edges IDs for given Node.
    ///
    /// # Arguments
    ///
    /// * src: NodeT - Node for which we need to compute the outbounds range.
    ///
    pub(crate) fn get_destinations_min_max_edge_ids(&self, src: NodeT) -> (EdgeT, EdgeT) {
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
                let min_edge_id: EdgeT = self.get_unchecked_edge_id_from_tuple(src, 0);
                (
                    min_edge_id,
                    match &self.cached_destinations {
                        Some(cds) => match cds.get(&src) {
                            Some(destinations) => destinations.len() as EdgeT + min_edge_id,
                            None => self.get_unchecked_edge_id_from_tuple(src + 1, 0),
                        },
                        None => self.get_unchecked_edge_id_from_tuple(src + 1, 0),
                    },
                )
            }
        }
    }

    /// Returns the number of outbound neighbours of given node.
    ///
    /// This is implemented as proposed by [S. Vigna here](http://vigna.di.unimi.it/ftp/papers/Broadword.pdf).
    ///
    /// # Arguments
    ///
    /// * `node_id` - Integer ID of the node.
    ///
    pub fn get_node_degree(&self, node_id: NodeT) -> Result<NodeT, String> {
        if node_id >= self.get_nodes_number(){
            return Err(format!(
                "The node ID {} is higher than the number of available nodes {}.",
                node_id,
                self.get_nodes_number()
            ));
        }
        let (min_edge_id, max_edge_id) = self.get_destinations_min_max_edge_ids(node_id);
        Ok((max_edge_id - min_edge_id) as NodeT)
    }

    /// Returns range of multigraph minimum and maximum edge ids with same source and destination nodes and different edge type.
    ///
    /// # Arguments
    /// 
    /// * `src` - Source node of the edge.
    /// * `dst` - Destination node of the edge.
    /// 
    pub(crate) fn get_unchecked_edge_ids_range(
        &self,
        src: NodeT,
        dst: NodeT,
    ) -> impl Iterator<Item = EdgeT> {
        let (min_edge_id, max_edge_id) = self.get_unchecked_edge_types_min_max_edge_ids(src, dst);
        min_edge_id..max_edge_id
    }

    /// Returns range of the edge ids of edges starting from the given source node.
    ///
    /// # Arguments
    /// 
    /// * `src` - Source node of the edge.
    /// 
    pub(crate) fn get_unchecked_destinations_range(&self, src: NodeT) -> impl Iterator<Item = EdgeT> {
        let (min_edge_id, max_edge_id) = self.get_destinations_min_max_edge_ids(src);
        min_edge_id..max_edge_id
    }

    /// Returns option of range of multigraph minimum and maximum edge ids with same source and destination nodes and different edge type.
    ///
    /// # Arguments
    /// 
    /// * `src` - Source node of the edge.
    /// 
    pub fn get_edge_ids(&self, src: NodeT, dst: NodeT) -> Option<impl Iterator<Item = EdgeT>> {
        self.get_edge_types_min_max_edge_ids(src, dst)
            .map(|(min_edge_id, max_edge_id)| min_edge_id..max_edge_id)
    }

    /// Returns edge_types associated to the given edge.
    /// A link is composed by all the edges that starts from src and ends at dst.
    ///
    /// # Arguments
    ///
    /// * `src`: NodeT - Integer ID of the source node.
    /// * `dst`: NodeT - Integer ID of the destination node.
    ///
    pub fn get_unchecked_link_edge_types(
        &self,
        src: NodeT,
        dst: NodeT,
    ) -> Option<Vec<Option<EdgeTypeT>>> {
        self.edge_types.as_ref().map(|ets| {
            self.get_unchecked_edge_ids_range(src, dst)
                .map(|edge_id| ets.ids[edge_id as usize])
                .collect()
        })
    }

    /// Returns weights associated to the given link.
    /// A link is composed by all the edges that starts from src and ends at dst.
    ///
    /// # Arguments
    ///
    /// * `src`: NodeT - Integer ID of the source node.
    /// * `dst`: NodeT - Integer ID of the destination node.
    ///
    pub fn get_unchecked_link_weights(&self, src: NodeT, dst: NodeT) -> Option<Vec<WeightT>> {
        self.weights.as_ref().map(|ws| {
            self.get_unchecked_edge_ids_range(src, dst)
                .map(|edge_id| ws[edge_id as usize])
                .collect()
        })
    }

    /// Returns boolean representing if given node is a trap.
    ///
    /// # Arguments
    ///
    /// * `node` - Integer ID of the node, if this is bigger that the number of nodes it will panic.
    ///
    pub fn is_node_trap(&self, node: NodeT) -> Result<bool, String> {
        Ok(self.get_node_degree(node)? == 0)
    }
    /// Returns boolean representing if given edge is a trap.
    ///
    /// # Arguments
    ///
    /// * `edge_id` - Integer ID of the edge, if this is bigger that the number of edges it will panic.
    ///
    pub fn is_edge_trap(&self, edge_id: EdgeT) -> Result<bool, String> {
        self.is_node_trap(self.get_destination(edge_id)?)
    }
}
