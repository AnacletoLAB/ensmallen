use super::*;
use counter::Counter;
use itertools::Itertools;
use rayon::prelude::*;
use std::collections::HashMap;
use std::iter::once;

impl Graph {
    /// Return name of the graph.
    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    /// Return the number of traps (nodes without any outgoing edges that are not singletons)
    pub fn get_traps_number(&self) -> EdgeT {
        self.not_singleton_nodes_number as EdgeT - self.unique_sources.len() as EdgeT
    }

    // Return if the graph has traps or not
    pub fn has_traps(&self) -> bool {
        self.get_traps_number() > 0
    }

    /// Returns boolean representing if graph is directed.
    pub fn is_directed(&self) -> bool {
        self.directed
    }

    /// Returns boolean representing if graph has weights.
    pub fn has_weights(&self) -> bool {
        self.weights.is_some()
    }

    /// Returns boolean representing if graph has edge types.
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
    pub fn is_singleton(&self, node_id: NodeT) -> bool {
        self.has_singletons() && self.get_node_degree(node_id) == 0
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
    pub fn is_singleton_by_nide_name(&self, node_name: &str) -> Result<bool, String> {
        Ok(self.is_singleton(self.get_node_id(node_name)?))
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
            .map(|node_id| (self.get_node_degree(node_id), node_id))
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
    pub fn get_edge_types(&self) -> Option<Vec<Option<EdgeTypeT>>> {
        self.edge_types.as_ref().map(|ets| ets.ids.clone())
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
    pub fn get_node_types(&self) -> Option<Vec<Option<Vec<NodeTypeT>>>> {
        self.node_types.as_ref().map(|nts| nts.ids.clone())
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
    pub fn get_weights(&self) -> Option<Vec<WeightT>> {
        self.weights.clone()
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

    /// Returs option with the edge type of the given edge id.
    pub fn get_unchecked_edge_type(&self, edge_id: EdgeT) -> Option<EdgeTypeT> {
        match &self.edge_types {
            Some(ets) => ets.ids[edge_id as usize],
            None => None,
        }
    }

    /// Returs option with the weight of the given edge id.
    pub fn get_unchecked_edge_weight(&self, edge_id: EdgeT) -> Option<WeightT> {
        self.weights.as_ref().map(|ws| ws[edge_id as usize])
    }

    /// Returs option with the node type of the given node id.
    pub fn get_unchecked_node_type(&self, node_id: NodeT) -> Option<Vec<NodeTypeT>> {
        self.node_types
            .as_ref()
            .and_then(|nts| nts.ids[node_id as usize].clone())
    }

    /// Returns node type of given node.
    ///
    /// # Arguments
    ///
    /// * node_id: NodeT - node whose node type is to be returned.
    ///
    /// # Examples
    /// ```rust
    /// # let graph = graph::test_utilities::load_ppi(true, true, true, true, false, false).unwrap();
    /// println!("The node type id of node {} is {:?}", 0, graph.get_node_type_id_by_node_id(0).unwrap());
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

    /// Returs option with the node type of the given node id.
    pub fn get_node_type_name(&self, node_id: NodeT) -> Result<Option<Vec<String>>, String> {
        match &self.node_types.is_some() {
            true => Ok(match self.get_unchecked_node_type(node_id) {
                Some(node_type_id) => Some(self.translate_node_type_id_vector(node_type_id)?),
                None => None,
            }),
            false => Err("Node types not available for the current graph instance.".to_string()),
        }
    }

    /// Returs option with the edge type of the given edge id.
    pub fn get_edge_type_name_by_edge_id(&self, edge_id: EdgeT) -> Option<String> {
        self.edge_types.as_ref().and_then(|ets| {
            self.get_unchecked_edge_type(edge_id)
                .map(|et| ets.unchecked_translate(et))
        })
    }

    /// Returs result with the node name.
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

    /// Returs result with the node id.
    pub fn get_node_id(&self, node_name: &str) -> Result<NodeT, String> {
        match self.nodes.get(node_name) {
            Some(node_id) => Ok(*node_id),
            None => Err(format!(
                "Given node name {} is not available in current graph.",
                node_name
            )),
        }
    }

    pub fn get_node_type_id_by_node_name(&self, node_name: &str) -> Result<Option<Vec<NodeTypeT>>, String> {
        self.get_node_type_id_by_node_id(self.get_node_id(node_name)?)
    }

    pub fn get_node_type_name_by_node_name(&self, node_name: &str) -> Result<Option<Vec<String>>, String> {
        self.get_node_type_name(self.get_node_id(node_name)?)
    }

    /// Returs whether the graph has the given node name.
    pub fn has_node_by_name(&self, node_name: &str) -> bool {
        self.get_node_id(node_name).is_ok()
    }

    /// Returs node id raising a panic if used unproperly.
    pub fn get_unchecked_node_id(&self, node_name: &str) -> NodeT {
        *self.nodes.get(node_name).unwrap()
    }

    /// Returs edge type id.
    pub fn get_unchecked_edge_type_id(&self, edge_type: Option<&str>) -> Option<EdgeTypeT> {
        match (&self.edge_types, edge_type) {
            (Some(ets), Some(et)) => ets.get(et).copied(),
            _ => None,
        }
    }

    /// Returs option with the weight of the given edge id.
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
        if let Some(etm) = &self.edge_types {
            etm.len() as EdgeTypeT
        } else {
            0
        }
    }

    /// Returns number of node types in the graph.
    pub fn get_node_types_number(&self) -> NodeTypeT {
        if let Some(etm) = &self.node_types {
            etm.len() as NodeTypeT
        } else {
            0
        }
    }

    /// Returns the degree of every node in the graph.
    pub fn get_node_degrees(&self) -> Vec<NodeT> {
        (0..self.get_nodes_number())
            .into_par_iter()
            .map(|node| self.get_node_degree(node as NodeT))
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

    pub fn get_unchecked_edge_count_by_edge_type(&self, edge_type: Option<EdgeTypeT>) -> EdgeT {
        match (&self.edge_types, edge_type) {
            (None, _) => 0,
            (Some(ets), None) => ets.get_unknown_count(),
            (Some(ets), Some(et)) => ets.counts[et as usize],
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

    pub fn get_unchecked_node_count_by_node_type(&self, node_type: NodeTypeT) -> NodeT {
        match &self.node_types {
            None => 0,
            Some(nts) => nts.counts[node_type as usize],
        }
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

    pub fn get_node_count_by_node_type(&self, node_type: NodeTypeT) -> Result<NodeT, String> {
        if !self.has_node_types() {
            return Err("Current graph does not have node types!".to_owned());
        }
        if self.get_node_types_number() <= node_type {
            return Err(format!(
                "Given node type ID {} is bigger than number of node types in the graph {}.",
                self.get_node_types_number(),
                node_type
            ));
        }
        Ok(self.get_unchecked_node_count_by_node_type(node_type))
    }

    pub fn get_node_count_by_node_type_name(&self, node_type: &str) -> Result<NodeT, String> {
        self.get_node_count_by_node_type(self.get_node_type_id(node_type)?)
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

    pub fn get_destination(&self, edge_id: EdgeT) -> NodeT {
        match &self.destinations {
            Some(destinations) => destinations[edge_id as usize],
            None => self.get_edge_from_edge_id(edge_id).1,
        }
    }

    pub fn get_destinations_range(
        &self,
        min_edge_id: EdgeT,
        max_edge_id: EdgeT,
    ) -> impl Iterator<Item = NodeT> + '_ {
        (min_edge_id..max_edge_id).map(move |edge_id| self.get_destination(edge_id))
    }

    /// Return iterator over NodeT of destinations of the given node src.
    pub fn get_neighbours_iter(&self, src: NodeT) -> impl Iterator<Item = NodeT> + '_ {
        self.get_unchecked_destinations_range(src)
            .map(move |edge_id| self.get_destination(edge_id))
    }

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

    pub fn get_unchecked_edge_id(
        &self,
        src: NodeT,
        dst: NodeT,
        edge_type: Option<EdgeTypeT>,
    ) -> EdgeT {
        if let Some(ets) = &self.edge_types {
            return self
                .get_unchecked_edge_ids_range(src, dst)
                // The vectors of the edge types can only have one element.
                .find(|edge_id| ets.ids[*edge_id as usize] == edge_type)
                .unwrap();
        }
        self.get_unchecked_edge_id_from_tuple(src, dst)
    }

    pub fn get_edge_id(
        &self,
        src: NodeT,
        dst: NodeT,
        edge_type: Option<EdgeTypeT>,
    ) -> Result<EdgeT, String> {
        let edge_id = self.edge_types.as_ref().map_or_else(|| self.get_edge_id_by_node_ids(src, dst), |ets| self.get_edge_ids(src, dst).and_then(|mut edge_ids| {
            edge_ids.find(|edge_id| ets.ids[*edge_id as usize] == edge_type)
        }));
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

    pub fn has_edge(&self, src: NodeT, dst: NodeT) -> bool {
        self.get_edge_id_by_node_ids(src, dst).is_some()
    }


    /// Returns boolean representing if edge passing between given nodes exists.
    ///
    /// # Arguments
    ///
    /// * src: NodeT - The source node of the edge.
    /// * dst: NodeT - The destination node of the edge.
    /// * edge_type: Option<EdgeTypeT> - The (optional) edge type.
    ///
    pub fn has_edge_with_type(&self, src: NodeT, dst: NodeT, edge_type: Option<EdgeTypeT>) -> bool {
        self.get_edge_id(src, dst, edge_type).is_ok()
    }

    pub fn get_edge_id_by_node_names(
        &self,
        src_name: &str,
        dst_name: &str
    ) -> Result<EdgeT, String> {
        let edge_id = if let (Some(src), Some(dst)) = (self.nodes.get(src_name), self.nodes.get(dst_name)) {
            self.get_edge_id_by_node_ids(*src, *dst)
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

    pub fn has_edge_by_node_names(
        &self,
        src_name: &str,
        dst_name: &str
    ) -> bool {
        self.get_edge_id_by_node_names(src_name, dst_name).is_ok()
    }

    pub fn get_edge_id_with_type_by_node_names(
        &self,
        src_name: &str,
        dst_name: &str,
        edge_type_name: Option<&String>,
    ) -> Result<EdgeT, String> {
        if let (Some(src), Some(dst)) = (self.nodes.get(src_name), self.nodes.get(dst_name)) {
            self.get_edge_id(*src, *dst, self.get_edge_type_id(edge_type_name.map(|x| x.as_str()))?)
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
        self.get_edge_id_by_node_ids(src, dst).map(
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
    /// * `node` - Integer ID of the node.
    ///
    pub fn get_node_degree(&self, node: NodeT) -> NodeT {
        let (min_edge_id, max_edge_id) = self.get_destinations_min_max_edge_ids(node);
        (max_edge_id - min_edge_id) as NodeT
    }

    /// Returns range of multigraph minimum and maximum edge ids with same source and destination nodes and different edge type.
    ///
    /// # Arguments
    /// 
    /// * `src` - Source node of the edge.
    /// * `dst` - Destination node of the edge.
    /// 
    pub fn get_unchecked_edge_ids_range(
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
    pub fn get_unchecked_destinations_range(&self, src: NodeT) -> impl Iterator<Item = EdgeT> {
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
    pub fn is_node_trap(&self, node: NodeT) -> bool {
        self.get_node_degree(node) == 0
    }
    /// Returns boolean representing if given edge is a trap.
    ///
    /// # Arguments
    ///
    /// * `edge_id` - Integer ID of the edge, if this is bigger that the number of edges it will panic.
    ///
    pub fn is_edge_trap(&self, edge_id: EdgeT) -> bool {
        self.is_node_trap(self.get_destination(edge_id))
    }
}
