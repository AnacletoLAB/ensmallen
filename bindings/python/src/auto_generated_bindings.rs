use super::*;

#[pymethods]
impl EnsmallenGraph {
    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Return total edge weights, if graph has weights.
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the graph does not contain edge weights.
    ///
    pub fn get_total_edge_weights() -> Result<f64> {
        self.graph.get_total_edge_weights()
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Return the minimum weight, if graph has weights.
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the graph does not contain edge weights.
    ///
    pub fn get_mininum_edge_weight() -> Result<WeightT> {
        self.graph.get_mininum_edge_weight()
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Return the maximum weight, if graph has weights.
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the graph does not contain edge weights.
    ///
    pub fn get_maximum_edge_weight() -> Result<WeightT> {
        self.graph.get_maximum_edge_weight()
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Return the maximum node degree.
    ///
    /// Safety
    /// ------
    /// The method will return an undefined value (0) when the graph
    /// does not contain nodes. In those cases the value is not properly
    /// defined.
    pub fn get_unchecked_maximum_node_degree() -> NodeT {
        self.graph.get_unchecked_maximum_node_degree()
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Return the minimum node degree.
    ///
    /// Safety
    /// ------
    /// The method will return an undefined value (0) when the graph
    /// does not contain nodes. In those cases the value is not properly
    /// defined.
    pub fn get_unchecked_minimum_node_degree() -> NodeT {
        self.graph.get_unchecked_minimum_node_degree()
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Return the maximum weighted node degree
    pub fn get_weighted_maximum_node_degree() -> Result<f64> {
        self.graph.get_weighted_maximum_node_degree()
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Return the minimum weighted node degree
    pub fn get_weighted_minimum_node_degree() -> Result<f64> {
        self.graph.get_weighted_minimum_node_degree()
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Return the number of weighted singleton nodes, i.e. nodes with weighted node degree equal to zero
    pub fn get_weighted_singleton_nodes_number() -> Result<NodeT> {
        self.graph.get_weighted_singleton_nodes_number()
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Returns number of self-loops, including also those in eventual multi-edges.
    pub fn get_selfloops_number() -> EdgeT {
        self.graph.get_selfloops_number()
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Returns number of unique self-loops, excluding those in eventual multi-edges.
    pub fn get_unique_selfloop_number() -> NodeT {
        self.graph.get_unique_selfloop_number()
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Returns number of singleton nodes with self-loops within the graph.
    pub fn get_singleton_nodes_with_selfloops_number() -> NodeT {
        self.graph.get_singleton_nodes_with_selfloops_number()
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Returns number of not singleton nodes within the graph.
    pub fn get_connected_nodes_number() -> NodeT {
        self.graph.get_connected_nodes_number()
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Returns a bitvector of the non singleton nodes within the graph.
    pub fn get_connected_nodes() -> ConcurrentBitVec {
        self.graph.get_connected_nodes()
    }
}
