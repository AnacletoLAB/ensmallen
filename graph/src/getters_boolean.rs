use super::*;
use rayon::iter::ParallelIterator;

/// # Boolean Getters
/// The naming convention we follow is:
/// * `/has_(.+)/`
/// * `/is_(.+)/`
///
/// The naming convention for unchecked methods follows:
/// * `/has_unchecked_(.+)/`
/// * `/is_unchecked_(.+)/`.
impl Graph {
    /// Return if graph has name that is not the default one.
    ///
    /// TODO: use a default for the default graph name!
    pub fn has_default_graph_name(&self) -> bool {
        self.get_name() == "Graph"
    }

    /// Return if the graph has any nodes.
    ///
    /// # Example
    /// To check if the graph has nodes you can use:
    /// ```rust
    /// # let graph_with_nodes = graph::test_utilities::load_ppi(true, true, true, true, false, false);
    /// # let empty_graph = graph::build_empty_graph(false, "Empty graph").unwrap();
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
    /// # let graph_with_edges = graph::test_utilities::load_ppi(true, true, true, true, false, false);
    /// # let empty_graph = graph::build_empty_graph(false, "Empty graph").unwrap();
    /// assert!(graph_with_edges.has_edges());
    /// assert!(!empty_graph.has_edges());
    /// ```
    ///
    pub fn has_edges(&self) -> bool {
        self.get_edges_number() > 0
    }

    /// Return whether the graph has trap nodes.
    ///
    /// # Example
    /// ```rust
    /// # let graph = graph::test_utilities::load_ppi(true, true, true, true, false, false);
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
    /// let directed_string_ppi = graph::test_utilities::load_ppi(true, true, true, true, false, false);
    /// assert!(directed_string_ppi.is_directed());
    /// let undirected_string_ppi = graph::test_utilities::load_ppi(true, true, true, false, false, false);
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
    /// let weights_string_ppi = graph::test_utilities::load_ppi(true, true, true, true, false, false);
    /// assert!(weights_string_ppi.has_edge_weights());
    /// let unweights_string_ppi = graph::test_utilities::load_ppi(true, true, false, true, false, false);
    /// assert!(!unweights_string_ppi.has_edge_weights());
    /// ```
    ///
    pub fn has_edge_weights(&self) -> bool {
        self.weights.is_some()
    }

    /// Returns whether graph has weights that can represent probabilities.
    pub fn has_edge_weights_representing_probabilities(&self) -> Result<bool> {
        Ok(self.get_mininum_edge_weight().clone()? > 0.0
            && self.get_maximum_edge_weight().clone()? <= 1.0)
    }

    /// Returns whether a graph has one or more weighted singleton nodes.
    ///
    /// A weighted singleton node is a node whose weighted node degree is 0.
    ///
    /// # Raises
    /// * If the graph does not contain edge weights.
    pub fn has_weighted_singleton_nodes(&self) -> Result<bool> {
        Ok(self.get_weighted_singleton_nodes_number().clone()? > 0)
    }

    /// Returns whether the graph has constant weights.
    ///
    /// # Implementative details
    /// If the minimum edge weight is closer than the maximum edge weight
    /// then the f32 epsilon we consider the weights functionally constant.
    ///
    /// # Raises
    /// * If the graph does not contain edge weights.
    pub fn has_constant_edge_weights(&self) -> Result<bool> {
        Ok(
            (self.get_maximum_edge_weight().clone()? - self.get_mininum_edge_weight().clone()?)
                .abs()
                < WeightT::EPSILON,
        )
    }

    /// Returns boolean representing whether graph has negative weights.
    ///
    /// # Example
    /// ```rust
    /// let weights_string_ppi = graph::test_utilities::load_ppi(true, true, true, true, false, false);
    /// assert!(weights_string_ppi.has_edge_weights());
    /// let unweights_string_ppi = graph::test_utilities::load_ppi(true, true, false, true, false, false);
    /// assert!(!unweights_string_ppi.has_edge_weights());
    /// ```
    ///
    /// # Raises
    /// * If the graph does not contain weights.
    pub fn has_negative_edge_weights(&self) -> Result<bool> {
        self.get_mininum_edge_weight()
            .clone()
            .map(|min_edge_weight| min_edge_weight < 0.0)
    }

    /// Returns boolean representing whether graph has edge types.
    ///
    /// # Example
    /// ```rust
    /// let string_ppi_with_edge_types = graph::test_utilities::load_ppi(true, true, true, true, false, false);
    /// assert!(string_ppi_with_edge_types.has_edge_types());
    /// let string_ppi_without_edge_types = graph::test_utilities::load_ppi(true, false, true, true, false, false);
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
    /// let string_ppi_with_selfloops = graph::test_utilities::load_ppi(true, true, true, true, false, false);
    /// assert!(string_ppi_with_selfloops.has_selfloops());
    /// let string_ppi_without_selfloops = string_ppi_with_selfloops.drop_selfloops();
    /// assert!(!string_ppi_without_selfloops.has_selfloops());
    /// ```
    ///
    pub fn has_selfloops(&self) -> bool {
        self.get_selfloops_number() > 0
    }

    /// Returns boolean representing if nodes which are nor singletons nor
    /// singletons with selfloops.
    ///
    /// # Example
    /// ```rust
    /// # let graph_with_singletons = graph::test_utilities::load_ppi(true, true, true, false, false, false);
    /// assert!(graph_with_singletons.has_disconnected_nodes());
    /// let graph_without_singletons = graph_with_singletons.drop_singleton_nodes();
    /// assert!(!graph_without_singletons.has_disconnected_nodes());
    /// ```
    pub fn has_disconnected_nodes(&self) -> bool {
        self.get_disconnected_nodes_number() > 0
    }

    /// Returns boolean representing if graph has singletons.
    ///
    /// # Example
    /// ```rust
    /// # let graph_with_singletons = graph::test_utilities::load_ppi(true, true, true, false, false, false);
    /// assert!(graph_with_singletons.has_singleton_nodes());
    /// let graph_without_singletons = graph_with_singletons.drop_singleton_nodes();
    /// assert!(!graph_without_singletons.has_singleton_nodes());
    /// ```
    pub fn has_singleton_nodes(&self) -> bool {
        self.get_singleton_nodes_number() > 0
    }

    /// Returns boolean representing if graph has singletons.
    pub fn has_singleton_nodes_with_selfloops(&self) -> bool {
        self.get_singleton_nodes_with_selfloops_number() > 0
    }

    #[cache_property(is_connected)]
    /// Returns whether the graph is connected.
    ///
    /// # Arguments
    /// * `verbose`: Option<bool> - Whether to show the loading bar while computing the connected components, if necessary.
    pub fn is_connected(&self, verbose: Option<bool>) -> bool {
        self.get_nodes_number() <= 1
            || !self.has_singleton_nodes()
                && !self.has_singleton_nodes_with_selfloops()
                && self.get_connected_components_number(verbose).0 == 1
    }

    /// Returns boolean representing if graph has node types.
    pub fn has_node_types(&self) -> bool {
        self.node_types.is_some()
    }

    /// Returns boolean representing if graph has multilabel node types.
    ///
    /// # Raises
    /// * If the graph does not have node types.
    pub fn has_multilabel_node_types(&self) -> Result<bool> {
        self.must_have_node_types()
            .map(|node_types| node_types.is_multilabel())
    }

    /// Returns whether there are unknown node types.
    ///
    /// # Raises
    /// * If the graph does not have node types.
    pub fn has_unknown_node_types(&self) -> Result<bool> {
        Ok(self.get_unknown_node_types_number()? > 0)
    }

    /// Returns whether there are known node types.
    ///
    /// # Raises
    /// * If the graph does not have node types.
    pub fn has_known_node_types(&self) -> Result<bool> {
        Ok(self.get_known_node_types_number()? > 0)
    }

    /// Returns whether there are unknown edge types.
    ///
    /// # Raises
    /// * If the graph does not have node types.
    pub fn has_unknown_edge_types(&self) -> Result<bool> {
        Ok(self.get_unknown_edge_types_number()? > 0)
    }

    /// Returns whether there are known edge types.
    ///
    /// # Raises
    /// * If the graph does not have edge types.
    pub fn has_known_edge_types(&self) -> Result<bool> {
        Ok(self.get_known_edge_types_number()? > 0)
    }

    /// Returns whether the nodes have an homogenous node type.
    ///
    /// # Raises
    /// * If the graph does not have node types.
    pub fn has_homogeneous_node_types(&self) -> Result<bool> {
        Ok(self
            .node_types
            .as_ref()
            .as_ref()
            .map_or(false, |node_type_ids| {
                node_type_ids
                    .counts
                    .iter()
                    .any(|&node_type_count| node_type_count == self.get_nodes_number())
            }))
    }

    /// Returns whether the edges have an homogenous edge type.
    ///
    /// # Raises
    /// * If the graph does not have edge types.
    pub fn has_homogeneous_edge_types(&self) -> Result<bool> {
        Ok(self.get_edge_types_number()? == 1)
    }

    /// Returns whether there is at least singleton node type, that is a node type that only appears once.
    ///
    /// # Raises
    /// * If the graph does not have node types.
    pub fn has_singleton_node_types(&self) -> Result<bool> {
        Ok(self.get_minimum_node_types_number()? == 1)
    }

    /// Return whether the graph has any known node-related graph oddities.
    pub fn has_node_oddities(&self) -> bool {
        [
            self.has_singleton_nodes(),
            self.has_singleton_nodes_with_selfloops(),
        ]
        .iter()
        .any(|value| *value)
    }

    /// Return whether the graph has any known node type-related graph oddities.
    ///
    /// # Raises
    /// * If the graph does not have node types.
    pub fn has_node_types_oddities(&self) -> Result<bool> {
        Ok([
            self.has_singleton_node_types()?,
            self.has_homogeneous_node_types()?,
            self.has_unknown_node_types()?,
        ]
        .iter()
        .any(|value| *value))
    }

    /// Returns whether there is at least singleton edge type, that is a edge type that only appears once.
    ///
    /// # Raises
    /// * If the graph does not have edge types.
    pub fn has_singleton_edge_types(&self) -> Result<bool> {
        Ok(self.get_minimum_edge_types_number()? == 1)
    }

    /// Return whether the graph has any known edge type-related graph oddities.
    ///
    /// # Raises
    /// * If the graph does not have edge types.
    pub fn has_edge_types_oddities(&self) -> Result<bool> {
        Ok([
            self.has_singleton_edge_types()?,
            self.has_homogeneous_edge_types()?,
            self.has_unknown_edge_types()?,
        ]
        .iter()
        .any(|value| *value))
    }

    /// Return if there are multiple edges between two nodes
    pub fn is_multigraph(&self) -> bool {
        self.get_parallel_edges_number() > 0
    }

    /// Return whether at least a node has a known ontology.
    pub fn has_node_ontologies(&self) -> bool {
        self.par_iter_node_ontologies()
            .any(|ontology| ontology.is_some())
    }

    /// Return whether at least a node has an unknown ontology.
    pub fn has_unknown_node_ontologies(&self) -> bool {
        self.par_iter_node_ontologies()
            .any(|ontology| ontology.is_none())
    }

    #[cache_property(nodes_sorted_by_decreasing_outbound_node_degree)]
    /// Returns whether the node IDs are sorted by decreasing outbound node degree.
    ///
    /// # Implications
    /// The implications of having a graph with node IDs sorted by the
    /// outbound node degrees are multiple.
    /// For instance, it makes it possible to create a NCE loss that
    /// is able to better approximate a complete Softmax by sampling
    /// the output labels using a Zipfian distribution, which is what
    /// most graphs follow.
    pub fn has_nodes_sorted_by_decreasing_outbound_node_degree(&self) -> bool {
        self.par_iter_node_ids().all(|node_id| unsafe {
            // If this is the first node, we just
            // return true.
            if node_id == 0 {
                return true;
            }
            // For the subsequent nodes we check two by two.
            // Since this is done in parallell, it should be
            // still be relatively efficient even though
            // the same thing in sequential could be done
            // via a simple scan.
            self.get_unchecked_node_degree_from_node_id(node_id)
                <= self.get_unchecked_node_degree_from_node_id(node_id - 1)
        })
    }

    #[cache_property(nodes_sorted_by_lexicographic_order)]
    /// Returns whether the node IDs are sorted by decreasing outbound node degree.
    ///
    /// # Implications
    /// The implications of having a graph with node IDs sorted by the
    /// lexicographic order are multiple.
    /// For instance, it makes it possible in some node keys distributions
    /// such as the names of websites to use this ordering for
    /// succinct data structures such as BVGraph.
    pub fn has_nodes_sorted_by_lexicographic_order(&self) -> bool {
        self.nodes.is_sorted_by_lexicographic_order()
    }

    /// Returns whether the graph contains the indentity matrix.
    ///
    /// # Implications
    /// The implications of having a graph containing teh identity
    /// matrix are that it is not required to add selfloops to the
    /// computation of the edge lists for kernels, like for instance
    /// the Laplacian kernel. This in turn, means that it is not necessary
    /// to sort the edge list in order to have it sorted, but it can
    /// be generated sorted in the first place.
    pub fn contains_identity_matrix(&self) -> bool {
        self.get_nodes_number() == self.get_unique_selfloops_number()
    }

    #[cache_property(nodes_sorted_by_increasing_outbound_node_degree)]
    /// Returns whether the node IDs are sorted by increasing outbound node degree.
    ///
    /// # Implications
    /// The implications of having a graph with node IDs sorted by the
    /// outbound node degrees are multiple.
    /// For instance, it makes it possible to create a NCE loss that
    /// is able to better approximate a complete Softmax by sampling
    /// the output labels using a Zipfian distribution, which is what
    /// most graphs follow.
    pub fn has_nodes_sorted_by_increasing_outbound_node_degree(&self) -> bool {
        self.par_iter_node_ids().all(|node_id| unsafe {
            // If this is the first node, we just
            // return true.
            if node_id == 0 {
                return true;
            }
            // For the subsequent nodes we check two by two.
            // Since this is done in parallell, it should be
            // still be relatively efficient even though
            // the same thing in sequential could be done
            // via a simple scan.
            self.get_unchecked_node_degree_from_node_id(node_id)
                >= self.get_unchecked_node_degree_from_node_id(node_id - 1)
        })
    }
}
