use super::*;

/// # Getters
/// The naming convention we follow is:
/// * `has_(.+)`
/// * `is_(.+)`
/// The naming convention for unchecked methods follows:
/// * `has_unchecked_(.+)`
/// * `is_unchecked_(.+)`.
impl Graph {
    /// Return if the graph has any nodes.
    ///
    /// # Example
    /// To check if the graph has nodes you can use:
    /// ```rust
    /// # let graph_with_nodes = graph::test_utilities::load_ppi(true, true, true, true, false, false).unwrap();
    /// # let empty_graph = graph::test_utilities::load_empty_graph(false);
    /// assert!(graph_with_nodes.has_nodes());
    /// assert!(!empty_graph.has_nodes());
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
    /// # let graph_with_edges = graph::test_utilities::load_ppi(true, true, true, true, false, false).unwrap();
    /// # let empty_graph = graph::test_utilities::load_empty_graph(false);
    /// assert!(graph_with_edges.has_edges());
    /// assert!(!empty_graph.has_edges());
    /// ```
    ///
    pub fn has_edges(&self) -> bool {
        self.get_edges_number() > 0
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
    /// assert!(weights_string_ppi.has_edge_weights());
    /// let unweights_string_ppi = graph::test_utilities::load_ppi(true, true, false, true, false, false).unwrap();
    /// assert!(!unweights_string_ppi.has_edge_weights());
    /// ```
    ///
    pub fn has_edge_weights(&self) -> bool {
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
        self.selfloop_number > 0
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
    pub fn has_singletons_with_selfloops(&self) -> bool {
        self.get_singleton_nodes_with_selfloops_number() > 0
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

    /// Returns whether there are unknown node types.
    pub fn has_unknown_node_types(&self) -> bool {
        self.get_unknown_node_types_number() > 0
    }

    /// Returns whether there are unknown edge types.
    pub fn has_unknown_edge_types(&self) -> bool {
        self.get_unknown_edge_types_number() > 0
    }

    /// Return if there are multiple edges between two nodes
    pub fn is_multigraph(&self) -> bool {
        self.get_multigraph_edges_number() > 0
    }
}
