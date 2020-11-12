use super::*;
use graph::{EdgeT, EdgeTypeT, NodeT, NodeTypeT, WeightT};
use numpy::{PyArray, PyArray1, PyArray2};
use std::collections::HashMap;

#[pymethods]
impl EnsmallenGraph {
    #[text_signature = "(self)"]
    /// Return the number of nodes in the graph.
    fn get_nodes_number(&self) -> NodeT {
        self.graph.get_nodes_number()
    }

    #[text_signature = "(self, k)"]
    /// Return List with top k central node Ids.
    ///
    /// Parameters
    /// -----------------
    /// k: int,
    ///      Number of central nodes to extract.
    ///
    /// Returns
    /// -----------------
    /// List of the top k central node Ids.
    fn get_top_k_central_nodes(&self, k: NodeT) -> Vec<NodeT> {
        self.graph.get_top_k_central_nodes(k)
    }

    #[text_signature = "(self, k)"]
    /// Return List with top k central node names.
    ///
    /// Parameters
    /// -----------------
    /// k: int,
    ///      Number of central nodes names to extract.
    ///
    /// Returns
    /// -----------------
    /// List of the top k central node names.
    fn get_top_k_central_node_names(&self, k: NodeT) -> Vec<String> {
        self.graph.get_top_k_central_node_names(k)
    }

    #[text_signature = "(self)"]
    /// Return the name of the graph.
    fn get_name(&self) -> String {
        self.graph.get_name()
    }

    #[text_signature = "(self)"]
    /// Return the number of edges in the graph.
    fn get_edges_number(&self) -> EdgeT {
        self.graph.get_edges_number()
    }

    #[text_signature = "(self)"]
    /// Return the number of undirected edges in the graph.
    fn get_undirected_edges_number(&self) -> EdgeT {
        self.graph.get_undirected_edges_number()
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

    #[text_signature = "(self, edge_type)"]
    /// Return the number of edges with the given edge type in the graph.
    /// 
    /// Parameters
    /// ---------------------
    /// edge_type: int,
    ///     Edge type ID for which to count the edges.
    /// 
    /// Raises
    /// ---------------------
    /// ValueError,
    ///     If the graph has no edge types.
    /// ValueError,
    ///     If the graph has not the given edge type.
    /// 
    /// Returns
    /// ---------------------
    /// Number of edges of given edge type.
    fn get_edge_count_by_edge_type(&self, edge_type: EdgeTypeT) -> PyResult<EdgeT> {
        pyex!(self.graph.get_edge_count_by_edge_type(edge_type))
    }

    #[text_signature = "(self, node_type)"]
    /// Return the number of nodes with the given node type in the graph.
    /// 
    /// Parameters
    /// ---------------------
    /// node_type: int,
    ///     Node type ID for which to number the nodes.
    /// 
    /// Raises
    /// ---------------------
    /// ValueError,
    ///     If the graph has no node types.
    /// ValueError,
    ///     If the graph has not the given node type.
    /// 
    /// Returns
    /// ---------------------
    /// Number of nodes of given node type.
    fn get_node_count_by_node_type(&self, node_type: NodeTypeT) -> PyResult<NodeT> {
        pyex!(self.graph.get_node_count_by_node_type(node_type))
    }

    #[text_signature = "(self, edge_type)"]
    /// Return the number of edges with the given edge type name in the graph.
    /// 
    /// Parameters
    /// ---------------------
    /// edge_type: str,
    ///     Edge type name for which to number the edges.
    /// 
    /// Raises
    /// ---------------------
    /// ValueError,
    ///     If the graph has no edge types.
    /// ValueError,
    ///     If the graph has not the given edge type.
    /// 
    /// Returns
    /// ---------------------
    /// Number of edges of given edge type.
    fn get_edge_count_by_edge_type_name(&self, edge_type: &str) -> PyResult<EdgeT> {
        pyex!(self.graph.get_edge_count_by_edge_type_name(edge_type))
    }

    #[text_signature = "(self, node_type)"]
    /// Return the number of nodes with the given node type name in the graph.
    /// 
    /// Parameters
    /// ---------------------
    /// node_type: str,
    ///     Node type name for which to number the nodes.
    /// 
    /// Raises
    /// ---------------------
    /// ValueError,
    ///     If the graph has no node types.
    /// ValueError,
    ///     If the graph has not the given node type.
    /// 
    /// Returns
    /// ---------------------
    /// Number of nodes of given node type.
    fn get_node_count_by_node_type_name(&self, node_type: &str) -> PyResult<NodeT> {
        pyex!(self.graph.get_node_count_by_node_type_name(node_type))
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

    #[text_signature = "($self, node_id)"]
    /// Return boolean representing singletons.
    ///
    /// Parameters
    /// ---------------------
    /// node_id: int,
    ///     Node ID to search if it's a singleton.
    ///
    /// Returns
    /// ----------------------------
    /// Boolean representing if given node is a singleton.
    ///
    fn is_singleton(&self, node_id: NodeT) -> bool {
        self.graph.is_singleton(node_id)
    }

    #[text_signature = "($self, node_name)"]
    /// Return boolean representing singletons.
    ///
    /// Parameters
    /// ---------------------
    /// node_name: str,
    ///     Node name to search if it's a singleton.
    ///
    /// Returns
    /// ----------------------------
    /// Boolean representing if given node is a singleton.
    ///
    fn is_singleton_string(&self, node_name: &str) -> PyResult<bool> {
        pyex!(self.graph.is_singleton_string(node_name))
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

    #[text_signature = "($self, src, dst, edge_type)"]
    /// Return boolean representing if given edge exists in graph.
    ///
    /// Parameters
    /// ---------------------
    /// src: int,
    ///     Node ID to use as source of given edge.
    /// dst: int,
    ///     Node ID to use as destination of given edge.
    /// edge_type: Union[None, int],
    ///     Edge type ID. (By deafult is None).
    ///
    /// Returns
    /// ----------------------------
    /// Boolean representing if given edge exists in graph.
    ///
    fn has_edge(&self, src: NodeT, dst: NodeT, edge_type: Option<EdgeTypeT>) -> bool {
        self.graph.has_edge(src, dst, edge_type)
    }

    #[text_signature = "($self, src, dst, edge_type)"]
    /// Return boolean representing if given edge exists in graph.
    ///
    /// Parameters
    /// ---------------------
    /// src: str,
    ///     Node name to use as source of given edge.
    /// dst: str,
    ///     Node name to use as destination of given edge.
    /// edge_type: Union[None, str],
    ///     Edge type name. (By deafult is None).
    ///
    /// Returns
    /// ----------------------------
    /// Boolean representing if given edge exists in graph.
    ///
    fn has_edge_string(&self, src: &str, dst: &str, edge_type: Option<String>) -> bool {
        self.graph.has_edge_string(&src, &dst, edge_type.as_ref())
    }

    #[text_signature = "($self, node_name, node_type)"]
    /// Return boolean representing if given node exists in graph.
    ///
    /// Parameters
    /// ---------------------
    /// node_name: str,
    ///     Name of the node.
    /// node_type: str = None,
    ///     Optional node type of the node.
    ///
    /// Returns
    /// ----------------------------
    /// Boolean representing if given node exists in graph.
    ///
    fn has_node_string(&self, node_name: &str, node_type: Option<String>) -> bool {
        self.graph.has_node_string(node_name, node_type)
    }

    #[text_signature = "($self, src, dst, edge_type)"]
    /// Return integer representing ID of the edge.
    ///
    /// Parameters
    /// ---------------------
    /// src: int,
    ///     Node ID to use as source of given edge.
    /// dst: int,
    ///     Node ID to use as destination of given edge.
    /// edge_type: Union[None, int],
    ///     Edge type ID. (By deafult is None).
    ///
    /// Returns
    /// ----------------------------
    /// Integer representing ID of the edge. It will return None when the edge does not exist.
    ///
    fn get_edge_id(&self, src: NodeT, dst: NodeT, edge_type: Option<EdgeTypeT>) -> Option<EdgeT> {
        self.graph.get_edge_id(src, dst, edge_type)
    }

    #[text_signature = "($self, src, dst, edge_type)"]
    /// Return integer representing ID of the edge.
    ///
    /// Parameters
    /// ---------------------
    /// src: str,
    ///     Node name to use as source of given edge.
    /// dst: str,
    ///     Node name to use as destination of given edge.
    /// edge_type: Union[None, str],
    ///     Edge type name. (By deafult is None).
    ///
    /// Returns
    /// ----------------------------
    /// Integer representing ID of the edge. It will return None when the edge does not exist.
    fn get_edge_id_string(&self, src: &str, dst: &str, edge_type: Option<String>) -> Option<EdgeT> {
        self.graph.get_edge_id_string(src, dst, edge_type.as_ref())
    }

    #[text_signature = "($self)"]
    /// Return mapping from instance not trap nodes to dense range of nodes.
    ///
    /// Returns
    /// ----------------------------
    /// Dict with mapping from not trap nodes to dense range of nodes.
    fn get_dense_node_mapping(&self) -> HashMap<NodeT, NodeT> {
        self.graph.get_dense_node_mapping()
    }

    #[text_signature = "($self)"]
    /// Return the number of source nodes.
    ///
    /// Returns
    /// ----------------------------
    /// Number of the source nodes.
    fn get_source_nodes_number(&self) -> NodeT {
        self.graph.get_source_nodes_number()
    }

    /// Return vector of the non-unique source nodes.
    ///
    /// Parameters
    /// --------------------------
    /// directed: bool,
    ///     Wethever to filter out the undirected edges.
    ///
    /// Returns
    /// --------------------------
    /// Numpy array with numeric sources Ids.
    pub fn get_sources(&self, directed: Option<bool>) -> PyResult<Py<PyArray1<NodeT>>> {
        let gil = pyo3::Python::acquire_gil();
        Ok(to_nparray_1d!(
            gil,
            self.graph.get_sources(directed.unwrap_or(true)),
            NodeT
        ))
    }

    /// Return vector on the (non unique) destination nodes of the graph.
    ///
    /// Parameters
    /// --------------------------
    /// directed: bool,
    ///     Wethever to filter out the undirected edges.
    ///
    /// Returns
    /// --------------------------
    /// Numpy array with numeric destination Ids.
    pub fn get_destinations(&self, directed: Option<bool>) -> PyResult<Py<PyArray1<NodeT>>> {
        let gil = pyo3::Python::acquire_gil();
        Ok(to_nparray_1d!(
            gil,
            self.graph.get_destinations(directed.unwrap_or(true)),
            NodeT
        ))
    }

    /// Return vector on the edges of the graph.
    ///
    /// Parameters
    /// --------------------------
    /// directed: bool,
    ///     Wethever to filter out the undirected edges.
    ///
    /// Returns
    /// --------------------------
    /// Numpy array with numeric source and destination Ids.
    pub fn get_edges(&self, directed: Option<bool>) -> PyResult<Py<PyArray2<NodeT>>> {
        let gil = pyo3::Python::acquire_gil();
        Ok(to_nparray_2d!(
            gil,
            self.graph.get_edges(directed.unwrap_or(true)),
            NodeT
        ))
    }

    /// Return list on the name of the edges of the graph.
    ///
    /// Parameters
    /// --------------------------
    /// directed: bool,
    ///     Wethever to filter out the undirected edges.
    ///
    /// Returns
    /// --------------------------
    /// Numpy array with numeric source and destination string names.
    pub fn get_edge_names(&self, directed: Option<bool>) -> Vec<(String, String)> {
        self.graph.get_edge_names(directed.unwrap_or(true))
    }

    /// Return vector of the non-unique source nodes names.
    ///
    /// Parameters
    /// --------------------------
    /// directed: bool,
    ///     wethever to filter out the undirected edges.
    pub fn get_source_names(&self, directed: Option<bool>) -> Vec<String> {
        self.graph.get_source_names(directed.unwrap_or(true))
    }

    /// Return vector on the (non unique) destination nodes of the graph.
    ///
    /// Parameters
    /// --------------------------
    /// directed: bool,
    ///     wethever to filter out the undirected edges.
    pub fn get_destination_names(&self, directed: Option<bool>) -> Vec<String> {
        self.graph.get_destination_names(directed.unwrap_or(true))
    }

    /// Return vector with the sorted nodes names.
    pub fn get_node_names(&self) -> Vec<String> {
        self.graph.get_node_names()
    }

    /// Return vector with the sorted nodes Ids.
    pub fn get_nodes(&self) -> PyResult<Py<PyArray1<NodeT>>> {
        let gil = pyo3::Python::acquire_gil();
        Ok(to_nparray_1d!(gil, self.graph.get_nodes(), NodeT))
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

    /// Return vector of node types_name.
    pub fn get_node_type_names(&self) -> Option<Vec<String>> {
        self.graph.get_node_type_names()
    }

    /// Return vector of edge types_name.
    pub fn get_edge_type_names(&self) -> Option<Vec<String>> {
        self.graph.get_edge_type_names()
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

    #[text_signature = "($self, node_id)"]
    /// Return the string name of the node.
    ///
    /// Parameters
    /// ---------------------
    /// node_id: int,
    ///     Numeric ID of the node.
    ///
    /// Returns
    /// ---------------------
    /// String name of the node.
    fn get_node_name(&self, node_id: NodeT) -> PyResult<String> {
        pyex!(self.graph.get_node_name(node_id))
    }

    #[text_signature = "($self, node_name)"]
    /// Return the node id curresponding to given string name.
    ///
    /// Parameters
    /// ---------------------
    /// node_name: str,
    ///     String name of the node.
    ///
    /// Returns
    /// ---------------------
    /// Node ID.
    fn get_node_id(&self, node_name: &str) -> PyResult<NodeT> {
        pyex!(self.graph.get_node_id(node_name))
    }

    #[text_signature = "($self)"]
    /// Return the count of how many time an edge type appears.
    fn get_edge_type_counts(&self) -> PyResult<HashMap<EdgeTypeT, usize>> {
        pyex!(self.graph.get_edge_type_counts_hashmap())
    }

    #[text_signature = "($self)"]
    /// Return the count of how many time an node type appears.
    fn get_node_type_counts(&self) -> PyResult<HashMap<EdgeTypeT, usize>> {
        pyex!(self.graph.get_node_type_counts_hashmap())
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
