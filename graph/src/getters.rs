use super::*;
use counter::Counter;
use itertools::Itertools;
use rayon::prelude::*;
use std::collections::HashMap;
use std::iter::once;

/// # Getters
/// The naming convention we follow is `get_X_from_Y`.
/// The naming convention for unchecked methods follows `get_unchecked_X_from_Y`.
impl Graph {
    /// Return if the graph has any nodes.
    ///
    /// # Example
    /// To check if the graph has nodes you can use:
    /// ```rust
    /// # let graph = graph::test_utilities::load_ppi(true, true, true, true, false, false).unwrap();
    /// assert_eq!(graph.has_nodes(), true);
    /// ```
    ///
    pub fn has_nodes(&self) -> bool {
        self.get_nodes_number() > 0
    }

    /// Return if the graph has any edges.
    ///
    /// # Example
    /// To check if the current graph has edges you can use:
    /// ```rust
    /// # let graph = graph::test_utilities::load_ppi(true, true, true, true, false, false).unwrap();
    /// assert_eq!(graph.has_edges(), true);
    /// ```
    ///
    pub fn has_edges(&self) -> bool {
        self.get_edges_number() > 0
    }

    /// Return name of the graph.
    ///
    /// # Example
    /// To the retrieve the name of the current graph instance you can use:
    /// ```rust
    /// # let graph = graph::test_utilities::load_ppi(true, true, true, true, false, false).unwrap();
    /// assert_eq!(graph.get_name(), "STRING PPI".to_string());
    /// println!("The name of the current graph is {}.", graph.get_name());
    /// ```
    ///
    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    /// Return the number of traps (nodes without any outgoing edges that are not singletons)
    /// This also includes nodes with only a self-loops, therefore singletons with
    /// only a self-loops are not considered traps because you could make a walk on them.
    ///
    /// # Example
    /// ```rust
    /// # let graph = graph::test_utilities::load_ppi(true, true, true, true, false, false).unwrap();
    /// println!("There are {} trap nodes in the current graph.", graph.get_trap_nodes_number());
    /// ```
    ///
    pub fn get_trap_nodes_number(&self) -> EdgeT {
        (self.get_not_singleton_nodes_number() + self.get_singleton_nodes_with_self_loops_number()
            - self.get_unique_source_nodes_number()) as EdgeT
    }

    // Return whether the graph has trap nodes.
    ///
    /// # Example
    /// ```rust
    /// # let graph = graph::test_utilities::load_ppi(true, true, true, true, false, false).unwrap();
    /// if graph.has_trap_nodes(){
    ///     println!("There are {} trap nodes in the current graph.", graph.get_trap_nodes_number());
    /// } else {
    ///     println!("There are no trap nodes in the current graph.");
    /// }
    /// ```
    ///
    pub fn has_trap_nodes(&self) -> bool {
        self.get_trap_nodes_number() > 0
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
    ///
    /// # Example
    /// ```rust
    /// let string_ppi_with_selfloops = graph::test_utilities::load_ppi(true, true, true, true, false, false).unwrap();
    /// assert!(string_ppi_with_selfloops.has_selfloops());
    /// let string_ppi_without_selfloops = graph::test_utilities::load_ppi(true, false, true, true, false, true).unwrap();
    /// assert!(!string_ppi_without_selfloops.has_selfloops());
    /// ```
    ///
    pub fn has_selfloops(&self) -> bool {
        self.self_loop_number > 0
    }

    /// Returns boolean representing if graph has singletons.
    ///
    /// # Example
    /// ```rust
    /// # let graph_with_singletons = graph::test_utilities::load_ppi(true, true, true, false, false, false).unwrap();
    /// assert!(graph_with_singletons.has_singletons());
    /// let graph_without_singletons = graph_with_singletons.remove(
    ///     None, None, None, None, None, None, None, None, false, false, true, true, false, false,
    /// ).unwrap();
    /// assert!(!graph_without_singletons.has_singletons());
    /// ```
    pub fn has_singletons(&self) -> bool {
        self.get_singleton_nodes_number() > 0
    }

    /// Returns boolean representing if graph has singletons.
    pub fn has_singleton_nodes_with_self_loops(&self) -> bool {
        self.get_singleton_nodes_with_self_loops_number() > 0
    }

    /// Return vector of the non-unique source nodes.
    ///
    /// # Arguments
    /// * `directed`: bool, whether to filter out the undirected edges.
    pub fn get_sources(&self, directed: bool) -> Vec<NodeT> {
        self.par_iter_sources_ids(directed).collect()
    }

    /// Return vector of the non-unique source nodes names.
    ///
    /// # Arguments
    /// * `directed`: bool, whether to filter out the undirected edges.
    pub fn get_source_names(&self, directed: bool) -> Vec<String> {
        self.par_iter_sources_ids(directed)
            .map(|src| self.get_node_name_from_node_id(src).unwrap())
            .collect()
    }

    /// Return vector on the (non unique) destination nodes of the graph.
    ///
    /// # Arguments
    /// * `directed`: bool, whether to filter out the undirected edges.
    pub fn get_destinations(&self, directed: bool) -> Vec<NodeT> {
        self.par_iter_destinations_ids(directed).collect()
    }

    /// Return vector of the non-unique destination nodes names.
    ///
    /// # Arguments
    /// * `directed`: bool, whether to filter out the undirected edges.
    pub fn get_destination_names(&self, directed: bool) -> Vec<String> {
        self.par_iter_destinations_ids(directed)
            .map(|dst| self.get_node_name_from_node_id(dst).unwrap())
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

    /// Return the edge types of the edges.
    pub fn get_edge_types(&self) -> Result<Vec<Option<EdgeTypeT>>, String> {
        if !self.has_edge_types() {
            return Err("The current graph instance does not have edge types!".to_string());
        }
        Ok(self.edge_types.as_ref().map(|ets| ets.ids.clone()).unwrap())
    }

    /// Return the edge types names.
    pub fn get_edge_type_names(&self) -> Option<Vec<String>> {
        self.edge_types
            .as_ref()
            .map(|ets| ets.vocabulary.reverse_map.clone())
    }

    /// Return the node types of the nodes.
    pub fn get_node_types(&self) -> Result<Vec<Option<Vec<NodeTypeT>>>, String> {
        if !self.has_node_types() {
            return Err("The current graph instance does not have nodes!".to_string());
        }
        Ok(self.node_types.as_ref().map(|nts| nts.ids.clone()).unwrap())
    }

    /// Return the weights of the edges.
    pub fn get_weights(&self) -> Result<Vec<WeightT>, String> {
        if !self.has_weights() {
            return Err("The current graph instance does not have weights!".to_string());
        }
        Ok(self.weights.clone().unwrap())
    }

    /// Return the minimum weight, if graph has weights.
    pub fn get_min_weight(&self) -> Result<WeightT, String> {
        self.weights.as_ref().map_or(
            Err("The current graph instance does not have weights!".to_string()),
            |ws| {
                Ok(ws
                    .par_iter()
                    .cloned()
                    .reduce(|| f32::INFINITY, |a, b| a.min(b)))
            },
        )
    }

    /// Return the maximum weight, if graph has weights.
    pub fn get_max_weight(&self) -> Result<WeightT, String> {
        self.weights.as_ref().map_or(
            Err("The current graph instance does not have weights!".to_string()),
            |ws| {
                Ok(ws
                    .par_iter()
                    .cloned()
                    .reduce(|| f32::NEG_INFINITY, |a, b| a.max(b)))
            },
        )
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
        self.par_iter_edge_ids(directed)
            .map(|(_, src, dst)| vec![src, dst])
            .collect()
    }

    /// Return vector with the sorted edge names.
    pub fn get_edge_names(&self, directed: bool) -> Vec<(String, String)> {
        self.par_iter_edges(directed)
            .map(|(_, _, src_name, _, dst_name)| (src_name, dst_name))
            .collect()
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
    /// E.g. If we have two components `[0, 2, 3]` and `[1, 4, 5]` the result will look like
    /// `[0, 1, 0, 0, 1, 1]`
    ///
    /// # Arguments
    /// * `verbose`: bool - whether to show the loading bar.
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
        self.edge_types
            .as_ref()
            .map_or(0, |ets| ets.len() as EdgeTypeT)
    }

    /// Returns number of node types in the graph.
    pub fn get_node_types_number(&self) -> NodeTypeT {
        self.node_types
            .as_ref()
            .map_or(0, |nts| nts.len() as NodeTypeT)
    }

    /// Returns the degree of every node in the graph.
    pub fn get_node_degrees(&self) -> Vec<NodeT> {
        self.iter_node_degrees().collect()
    }

    /// Return set of nodes that are not singletons.
    // TODO: THIS METHOD CAN NOW BE WAY FASTER!
    pub fn get_not_singletons(&self) -> Vec<NodeT> {
        self.iter_edge_ids(false)
            .flat_map(|(_, src, dst)| once(src).chain(once(dst)))
            .unique()
            .collect()
    }

    /// Return mapping from instance not trap nodes to dense nodes.
    // TODO: REFACTOR THIS TO AVOID DOUBLE ITERATION!
    pub fn get_dense_node_mapping(&self) -> HashMap<NodeT, NodeT> {
        self.get_not_singletons()
            .into_iter()
            .enumerate()
            .map(|(i, node)| (node as NodeT, i as NodeT))
            .collect()
    }

    /// Return if there are multiple edges between two nodes
    pub fn is_multigraph(&self) -> bool {
        self.get_multigraph_edges_number() > 0
    }

    /// Return number of edges that have multigraph syblings.
    pub fn get_multigraph_edges_number(&self) -> EdgeT {
        self.get_directed_edges_number() - self.unique_edges_number
    }

    /// Return vector with node degrees
    /// TODO: USE CACHE!!!
    pub fn get_outbounds(&self) -> Vec<EdgeT> {
        (0..self.get_nodes_number())
            .into_par_iter()
            .map(|src| self.get_unchecked_edge_id_from_node_ids(src as NodeT + 1, 0))
            .collect()
    }

    /// Returns number of the source nodes.
    ///```rust
    /// # let graph = graph::test_utilities::load_ppi(true, true, true, true, false, false).unwrap();
    /// println!("The number of sources of the graph (not trap nodes) is {}", graph.get_unique_source_nodes_number());
    /// ```
    pub fn get_unique_source_nodes_number(&self) -> NodeT {
        self.unique_sources
            .as_ref()
            .map_or(self.get_nodes_number(), |x| x.len() as NodeT)
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
}
