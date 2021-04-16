use super::*;
use counter::Counter;
use rayon::prelude::*;
use std::collections::HashMap;

/// # Getters
/// The naming convention we follow is `get_X_from_Y`.
/// The naming convention for unchecked methods follows `get_unchecked_X_from_Y`.
impl Graph {
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
        (self.get_not_singleton_nodes_number() + self.get_singleton_nodes_with_selfloops_number()
            - self.get_unique_source_nodes_number()) as EdgeT
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
            .map(|src| self.get_unchecked_node_name_from_node_id(src))
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
            .map(|dst| self.get_unchecked_node_name_from_node_id(dst))
            .collect()
    }

    /// Return vector with the sorted nodes names.
    pub fn get_node_names(&self) -> Vec<String> {
        self.nodes.reverse_map.clone()
    }

    /// Return vector with the sorted nodes Ids.
    pub fn get_nodes(&self) -> Vec<NodeT> {
        self.iter_node_ids().collect()
    }

    /// Return the edge types of the edges.
    pub fn get_edge_types(&self) -> Result<Vec<Option<EdgeTypeT>>, String> {
        self.must_have_edge_types()?;
        Ok(self.edge_types.as_ref().map(|ets| ets.ids.clone()).unwrap())
    }

    /// Return the edge types names.
    pub fn get_edge_type_names(&self) -> Option<Vec<String>> {
        self.edge_types
            .as_ref()
            .map(|ets| ets.vocabulary.reverse_map.clone())
    }

    /// Return the weights of the graph edges.
    ///
    /// # Example
    /// To get an the graph weights you can use:
    /// ```rust
    /// # let graph_with_weights = graph::test_utilities::load_ppi(false, false, true, true, false, false).unwrap();
    /// # let graph_without_weights = graph::test_utilities::load_ppi(false, false, false, true, false, false).unwrap();
    /// assert!(graph_with_weights.get_weights().is_ok());
    /// assert!(graph_without_weights.get_weights().is_err());
    /// println!("The graph weights are {:?}.", graph_with_weights.get_weights());
    /// ```
    pub fn get_weights(&self) -> Result<Vec<WeightT>, String> {
        self.must_have_edge_weights()?;
        Ok(self.weights.clone().unwrap())
    }

    /// Return the minimum weight, if graph has weights.
    ///
    /// # Example
    /// To get the minimum edge weight you can use:
    /// ```rust
    /// # let graph_with_weights = graph::test_utilities::load_ppi(false, false, true, true, false, false).unwrap();
    /// # let graph_without_weights = graph::test_utilities::load_ppi(false, false, false, true, false, false).unwrap();
    /// assert!(graph_with_weights.get_min_weight().is_ok());
    /// assert!(graph_without_weights.get_min_weight().is_err());
    /// println!("The graph minimum weight is {:?}.", graph_with_weights.get_min_weight());
    /// ```
    pub fn get_min_weight(&self) -> Result<WeightT, String> {
        Ok(self
            .par_iter_weights()?
            .reduce(|| f32::INFINITY, |a, b| a.min(b)))
    }

    /// Return the maximum weight, if graph has weights.
    ///
    /// # Example
    /// To get the maximum edge weight you can use:
    /// ```rust
    /// # let graph_with_weights = graph::test_utilities::load_ppi(false, false, true, true, false, false).unwrap();
    /// # let graph_without_weights = graph::test_utilities::load_ppi(false, false, false, true, false, false).unwrap();
    /// assert!(graph_with_weights.get_max_weight().is_ok());
    /// assert!(graph_without_weights.get_max_weight().is_err());
    /// println!("The graph maximum weight is {:?}.", graph_with_weights.get_max_weight());
    /// ```
    pub fn get_max_weight(&self) -> Result<WeightT, String> {
        Ok(self
            .par_iter_weights()?
            .reduce(|| f32::NEG_INFINITY, |a, b| a.max(b)))
    }

    /// Return the node types of the graph nodes.
    ///
    /// # Example
    /// To retrieve the node type IDs of the graph nodes you can use:
    /// ```rust
    /// # let graph_with_node_types = graph::test_utilities::load_ppi(true, true, true, true, false, false).unwrap();
    /// # let graph_without_node_types = graph::test_utilities::load_ppi(false, true, true, true, false, false).unwrap();
    /// assert!(graph_with_node_types.get_node_types_ids().is_ok());
    /// assert!(graph_without_node_types.get_node_types_ids().is_err());
    /// println!("The graph node types are {:?}", graph_with_node_types.get_node_types_ids());
    /// ```
    ///
    pub fn get_node_types_ids(&self) -> Result<Vec<Option<Vec<NodeTypeT>>>, String> {
        self.must_have_node_types()?;
        Ok(self.node_types.as_ref().map(|nts| nts.ids.clone()).unwrap())
    }

    /// Return the node types names.
    ///
    /// # Example
    /// To retrieve the node type names of the graph nodes you can use:
    /// ```rust
    /// # let graph_with_node_types = graph::test_utilities::load_ppi(true, true, true, true, false, false).unwrap();
    /// # let graph_without_node_types = graph::test_utilities::load_ppi(false, true, true, true, false, false).unwrap();
    /// assert!(graph_with_node_types.get_node_type_names().is_ok());
    /// assert!(graph_without_node_types.get_node_type_names().is_err());
    /// println!("The graph node types are {:?}", graph_with_node_types.get_node_type_names());
    /// ```
    ///
    pub fn get_node_type_names(&self) -> Result<Vec<String>, String> {
        self.must_have_node_types()?;
        Ok(self
            .node_types
            .as_ref()
            .map(|nts| nts.vocabulary.reverse_map.clone())
            .unwrap())
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
    pub fn get_not_singletons(&self) -> Vec<NodeT> {
        self.iter_non_singleton_node_ids().collect()
    }

    /// Return mapping from instance not trap nodes to dense nodes.
    pub fn get_dense_node_mapping(&self) -> HashMap<NodeT, NodeT> {
        self.iter_non_singleton_node_ids()
            .enumerate()
            .map(|(i, node)| (node as NodeT, i as NodeT))
            .collect()
    }

    /// Return number of edges that have multigraph syblings.
    pub fn get_multigraph_edges_number(&self) -> EdgeT {
        self.get_directed_edges_number() - self.unique_edges_number
    }

    /// Return vector with node outbounds, that is the comulative node degree.
    pub fn get_outbounds(&self) -> Vec<EdgeT> {
        self.outbounds.as_ref().map_or_else(
            || {
                self.par_iter_node_ids()
                    .map(|src| self.get_unchecked_edge_id_from_node_ids(src + 1, 0))
                    .collect()
            },
            |outbounds| outbounds.clone(),
        )
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
