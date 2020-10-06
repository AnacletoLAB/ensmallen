use super::*;
use graph::{EdgeT, EdgeTypeT, NodeT, NodeTypeT, WeightT};
use numpy::{PyArray, PyArray1};
use std::collections::HashMap;

#[pymethods]
impl EnsmallenGraph {
    #[text_signature = "(self)"]
    /// Return the number of nodes in the graph.
    fn get_nodes_number(&self) -> NodeT {
        self.graph.get_nodes_number()
    }

    #[text_signature = "(self)"]
    /// Return the number of edges in the graph.
    fn get_edges_number(&self) -> EdgeT {
        self.graph.get_edges_number()
    }

    #[text_signature = "(self)"]
    /// Return the number of edges types in the graph.
    ///
    /// This method will include, if found necessary by a missing value,
    /// also the default edge type in the count of total edge types.
    ///
    fn get_edge_types_number(&self) -> EdgeTypeT {
        self.graph.get_edge_types_number()
    }

    #[text_signature = "(self)"]
    /// Return the number of edges in the graph.
    ///
    /// This method will include, if found necessary by a missing value,
    /// also the default node type in the count of total node types.
    ///
    fn get_node_types_number(&self) -> NodeTypeT {
        self.graph.get_node_types_number()
    }

    #[text_signature = "($self, node)"]
    /// Return boolean representing if given node is a trap.
    ///
    /// A trap node is a node with no outbounds edges.
    ///
    /// Parameters
    /// ---------------------
    /// node: int,
    ///     Node ID to search if it's a trap.
    ///
    /// Returns
    /// ----------------------------
    /// Boolean representing if given node is a trap.
    ///
    fn is_node_trap(&self, node: NodeT) -> bool {
        self.graph.is_node_trap(node)
    }

    #[text_signature = "($self, edge)"]
    /// Return boolean representing if given edge is a trap.
    ///
    /// A trap edge is a edge with a destination node that is a trap node.
    ///
    /// Parameters
    /// ---------------------
    /// node: int,
    ///     Node ID to search if it's a trap.
    ///
    /// Returns
    /// ----------------------------
    /// Boolean representing if given edge is a trap.
    ///
    fn is_edge_trap(&self, edge: EdgeT) -> bool {
        self.graph.is_edge_trap(edge)
    }

    #[text_signature = "($self, src, dst)"]
    /// Return boolean representing if given edge exists in graph.
    ///
    /// Parameters
    /// ---------------------
    /// src: int,
    ///     Node ID to use as source of given edge.
    /// dst: int,
    ///     Node ID to use as destination of given edge.
    ///
    /// Returns
    /// ----------------------------
    /// Boolean representing if given edge exists in graph.
    ///
    fn has_edge(&self, src: NodeT, dst: NodeT, edge_type: Option<EdgeTypeT>) -> bool {
        self.graph.has_edge(src, dst, edge_type)
    }

    #[text_signature = "($self)"]
    /// Return mapping from instance not trap nodes to dense range of nodes.
    ///
    /// Returns
    /// ----------------------------
    /// Dict with mapping from not trap nodes to dense range of nodes.
    ///
    fn get_dense_node_mapping(&self) -> HashMap<NodeT, NodeT> {
        self.graph.get_dense_node_mapping()
    }

    #[text_signature = "($self)"]
    /// Return the number of source nodes.
    ///
    /// Returns
    /// ----------------------------
    /// Number of the source nodes.
    ///
    fn get_source_nodes_number(&self) -> NodeT {
        self.graph.get_source_nodes_number()
    }

    /// Return vector of the non-unique source nodes.
    pub fn get_sources(&self) -> PyResult<Py<PyArray1<NodeT>>> {
        let gil = pyo3::Python::acquire_gil();
        Ok(to_nparray_1d!(gil, self.graph.get_sources(), NodeT))
    }

    /// Return vector on the (non unique) destination nodes of the graph.
    pub fn get_destinations(&self) -> PyResult<Py<PyArray1<NodeT>>> {
        let gil = pyo3::Python::acquire_gil();
        Ok(to_nparray_1d!(gil, self.graph.get_destinations(), NodeT))
    }

    /// Return vector of strings representing the node Ids reverse mapping.
    pub fn get_nodes_reverse_mapping(&self) -> Vec<String> {
        self.graph.get_nodes_reverse_mapping()
    }

    /// Return vector of node types.
    pub fn get_node_types(&self) -> PyResult<Py<PyArray1<NodeTypeT>>> {
        pyex!(match self.graph.get_node_types() {
            Some(values) => {
                let gil = pyo3::Python::acquire_gil();
                Ok(to_nparray_1d!(gil, values, NodeTypeT))
            }
            None => Err("Graph does not have node types."),
        })
    }

    /// Return vector of edge types.
    pub fn get_edge_types(&self) -> PyResult<Py<PyArray1<EdgeTypeT>>> {
        pyex!(match self.graph.get_edge_types() {
            Some(values) => {
                let gil = pyo3::Python::acquire_gil();
                Ok(to_nparray_1d!(gil, values, EdgeTypeT))
            }
            None => Err("Graph does not have edge types."),
        })
    }

    /// Return vector of weights.
    pub fn get_weights(&self) -> PyResult<Py<PyArray1<WeightT>>> {
        pyex!(match self.graph.get_weights() {
            Some(values) => {
                let gil = pyo3::Python::acquire_gil();
                Ok(to_nparray_1d!(gil, values, WeightT))
            }
            None => Err("Graph does not have weights."),
        })
    }

    /// Return vector of node types_reverse_mapping.
    pub fn get_node_types_reverse_mapping(&self) -> Option<Vec<String>> {
        self.graph.get_node_types_reverse_mapping()
    }

    /// Return vector of edge types_reverse_mapping.
    pub fn get_edge_types_reverse_mapping(&self) -> Option<Vec<String>> {
        self.graph.get_edge_types_reverse_mapping()
    }

    /// Return dictionary of strings to Ids representing the ndoes mapping.
    pub fn get_nodes_mapping(&self) -> HashMap<String, NodeT> {
        self.graph.get_nodes_mapping()
    }

    #[text_signature = "($self, edge_id)"]
    /// Return the id of the edge type of the edge.
    ///
    /// Parameters
    /// ---------------------
    /// edge_id: int,
    ///     Numeric ID of the edge.
    ///
    /// Returns
    /// ---------------------
    /// Id of the edge type of the edge.
    fn get_edge_type(&self, edge_id: EdgeT) -> PyResult<EdgeTypeT> {
        pyex!(self.graph.get_edge_type(edge_id))
    }

    #[text_signature = "($self, node_id)"]
    /// Return the id of the node type of the node.
    ///
    /// Parameters
    /// ---------------------
    /// node_id: int,
    ///     Numeric ID of the node.
    ///
    /// Returns
    /// ---------------------
    /// Id of the node type of the node.
    fn get_node_type(&self, node_id: NodeT) -> PyResult<NodeTypeT> {
        pyex!(self.graph.get_node_type(node_id))
    }

    #[text_signature = "($self)"]
    /// Return the count of how many time an edge type appears.
    fn get_edge_type_counts(&self) -> PyResult<HashMap<EdgeTypeT, usize>> {
        pyex!(self.graph.get_edge_type_counts())
    }

    #[text_signature = "($self)"]
    /// Return the count of how many time an node type appears.
    fn get_node_type_counts(&self) -> PyResult<HashMap<EdgeTypeT, usize>> {
        pyex!(self.graph.get_node_type_counts())
    }

    #[text_signature = "(self)"]
    /// Returns a boolean representing if the graph contains an edge that has
    /// source == destination.
    fn has_selfloops(&self) -> bool {
        self.graph.has_selfloops()
    }

    #[text_signature = "(self)"]
    /// Returns true if the graph has weights.
    fn has_weights(&self) -> bool {
        self.graph.has_weights()
    }

    #[text_signature = "(self)"]
    /// Returns true if the graph has node types.
    fn has_node_types(&self) -> bool {
        self.graph.has_node_types()
    }

    #[text_signature = "(self)"]
    /// Returns true if the graph has edge types.
    fn has_edge_types(&self) -> bool {
        self.graph.has_edge_types()
    }
}
