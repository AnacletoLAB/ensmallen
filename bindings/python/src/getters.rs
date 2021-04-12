use super::*;
use graph::{EdgeT, EdgeTypeT, NodeT, NodeTypeT, WeightT};
use numpy::{PyArray, PyArray1, PyArray2};
use std::collections::HashMap;

#[pymethods]
impl EnsmallenGraph {
    #[text_signature = "($self)"]
    /// Return the number of nodes in the graph.
    fn get_nodes_number(&self) -> NodeT {
        self.graph.get_nodes_number()
    }

    #[text_signature = "($self, k)"]
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
    fn get_top_k_central_nodes_ids(&self, k: NodeT) -> Vec<NodeT> {
        self.graph.get_top_k_central_nodes_ids(k)
    }

    #[text_signature = "($self, k)"]
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

    #[text_signature = "($self)"]
    /// Return the name of the graph.
    fn get_name(&self) -> String {
        self.graph.get_name()
    }

    #[text_signature = "($self)"]
    /// Return the number of edges in the graph.
    fn get_edges_number(&self) -> EdgeT {
        self.graph.get_edges_number()
    }

    #[text_signature = "($self)"]
    /// Return the number of undirected edges in the graph.
    fn get_undirected_edges_number(&self) -> EdgeT {
        self.graph.get_undirected_edges_number()
    }

    #[text_signature = "($self)"]
    /// Return the number of edges types in the graph.
    ///
    /// This method will include, if found necessary by a missing value,
    /// also the default edge type in the count of total edge types.
    ///
    fn get_edge_types_number(&self) -> EdgeTypeT {
        self.graph.get_edge_types_number()
    }

    #[text_signature = "($self)"]
    /// Return the number of edges in the graph.
    ///
    /// This method will include, if found necessary by a missing value,
    /// also the default node type in the count of total node types.
    ///
    fn get_node_types_number(&self) -> NodeTypeT {
        self.graph.get_node_types_number()
    }

    #[text_signature = "($self, edge_type)"]
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
    fn  get_edge_count_by_edge_type_id(&self, edge_type: Option<EdgeTypeT>) -> PyResult<EdgeT> {
        pe!(self.graph.get_edge_count_by_edge_type_id(edge_type))
    }

    #[text_signature = "($self, node_type)"]
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
    fn get_node_count_by_node_type_id(&self, node_type: Option<NodeTypeT>) -> PyResult<NodeT> {
        pe!(self.graph.get_node_count_by_node_type_id(node_type))
    }

    #[text_signature = "($self, edge_type)"]
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
    fn get_edge_count_by_edge_type_name(&self, edge_type: Option<&str>) -> PyResult<EdgeT> {
        pe!(self.graph.get_edge_count_by_edge_type_name(edge_type))
    }

    #[text_signature = "($self, node_type)"]
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
    fn get_node_count_by_node_type_name(&self, node_type: Option<&str>) -> PyResult<NodeT> {
        pe!(self.graph.get_node_count_by_node_type_name(node_type))
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
    fn is_node_trap_by_node_id(&self, node: NodeT) -> PyResult<bool> {
        pe!(self.graph.is_node_trap_by_node_id(node))
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
    fn is_singleton_by_node_id(&self, node_id: NodeT) -> PyResult<bool> {
        pe!(self.graph.is_singleton_by_node_id(node_id))
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
    fn is_singleton_by_node_name(&self, node_name: &str) -> PyResult<bool> {
        pe!(self.graph.is_singleton_by_node_name(node_name))
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
    fn has_edge_by_node_ids(&self, src: NodeT, dst: NodeT) -> bool {
        self.graph.has_edge_by_node_ids(src, dst)
    }

    #[text_signature = "($self, src, dst, edge_type)"]
    /// Return boolean representing if given edge with type exists in graph.
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
    fn has_edge_with_type_by_node_ids(&self, src: NodeT, dst: NodeT, edge_type: Option<EdgeTypeT>) -> bool {
        self.graph.has_edge_with_type_by_node_ids(src, dst, edge_type)
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
    fn has_edge_with_type_by_node_names(
        &self,
        src: &str,
        dst: &str,
        edge_type: Option<String>,
    ) -> bool {
        self.graph
            .has_edge_with_type_by_node_names(&src, &dst, edge_type.as_ref())
    }

    #[text_signature = "($self, node_name)"]
    /// Return boolean representing if given node exists in graph.
    ///
    /// Parameters
    /// ---------------------
    /// node_name: str,
    ///     Name of the node.
    ///
    /// Returns
    /// ----------------------------
    /// Boolean representing if given node exists in graph.
    ///
    fn has_node_by_node_name(&self, node_name: &str) -> bool {
        self.graph.has_node_by_node_name(node_name)
    }

    #[text_signature = "($self, node_name, node_type)"]
    /// Return boolean representing if given node with type exists in graph.
    ///
    /// Parameters
    /// ---------------------
    /// node_name: str,
    ///     Name of the node.
    /// node_type: List[str] = None,
    ///     Optional node type of the node.
    ///
    /// Returns
    /// ----------------------------
    /// Boolean representing if given node exists in graph.
    ///
    fn has_node_with_type_by_node_name(&self, node_name: &str, node_type: Option<Vec<String>>) -> bool {
        self.graph.has_node_with_type_by_node_name(node_name, node_type)
    }

    #[text_signature = "($self, src, dst)"]
    /// Return integer representing ID of the edge.
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
    /// Integer representing ID of the edge.
    ///
    fn get_edge_id_by_node_ids(&self, src: NodeT, dst: NodeT) -> PyResult<EdgeT> {
        pe!(self.graph.get_edge_id_by_node_ids(src, dst))
    }

    #[text_signature = "($self, src, dst, edge_type)"]
    /// Return integer representing ID of the edge with type.
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
    /// Integer representing ID of the edge.
    ///
    fn get_edge_id_with_type_by_node_ids(
        &self,
        src: NodeT,
        dst: NodeT,
        edge_type: Option<EdgeTypeT>,
    ) -> PyResult<EdgeT> {
        pe!(self
            .graph
            .get_edge_id_with_type_by_node_ids(src, dst, edge_type))
    }

    #[text_signature = "($self, src, dst)"]
    /// Return integer representing ID of the edge.
    ///
    /// Parameters
    /// ---------------------
    /// src: str,
    ///     Node name to use as source of given edge.
    /// dst: str,
    ///     Node name to use as destination of given edge.
    ///
    /// Returns
    /// ----------------------------
    /// Integer representing ID of the edge. It will return None when the edge does not exist.
    fn get_edge_id_by_node_names(&self, src: &str, dst: &str) -> PyResult<EdgeT> {
        pe!(self.graph.get_edge_id_by_node_names(src, dst))
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
    fn get_edge_id_with_type_by_node_names(
        &self,
        src: &str,
        dst: &str,
        edge_type: Option<String>,
    ) -> PyResult<EdgeT> {
        pe!(self
            .graph
            .get_edge_id_with_type_by_node_names(src, dst, edge_type.as_ref()))
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
    fn get_unique_source_nodes_number(&self) -> NodeT {
        self.graph.get_unique_source_nodes_number()
    }

    #[text_signature = "($self, directed)"]
    /// Return vector of the non-unique source nodes.
    ///
    /// Parameters
    /// --------------------------
    /// directed: bool,
    ///     whether to filter out the undirected edges.
    ///
    /// Returns
    /// --------------------------
    /// Numpy array with numeric sources Ids.
    fn get_sources(&self, directed: Option<bool>) -> Py<PyArray1<NodeT>> {
        let gil = pyo3::Python::acquire_gil();
        to_ndarray_1d!(gil, self.graph.get_sources(directed.unwrap_or(true)), NodeT)
    }

    #[text_signature = "($self, directed)"]
    /// Return vector on the (non unique) destination nodes of the graph.
    ///
    /// Parameters
    /// --------------------------
    /// directed: bool,
    ///     whether to filter out the undirected edges.
    ///
    /// Returns
    /// --------------------------
    /// Numpy array with numeric destination Ids.
    fn get_destinations(&self, directed: Option<bool>) -> Py<PyArray1<NodeT>> {
        let gil = pyo3::Python::acquire_gil();
        to_ndarray_1d!(gil, self.graph.get_destinations(directed.unwrap_or(true)), NodeT)
    }

    #[text_signature = "($self, directed)"]
    /// Return vector on the edges of the graph.
    ///
    /// Parameters
    /// --------------------------
    /// directed: bool,
    ///     whether to filter out the undirected edges.
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

    #[text_signature = "($self, directed)"]
    /// Return list on the name of the edges of the graph.
    ///
    /// Parameters
    /// --------------------------
    /// directed: bool,
    ///     whether to filter out the undirected edges.
    ///
    /// Returns
    /// --------------------------
    /// Numpy array with numeric source and destination string names.
    pub fn get_edge_names(&self, directed: Option<bool>) -> Vec<(String, String)> {
        self.graph.get_edge_names(directed.unwrap_or(true))
    }

    #[text_signature = "($self, directed)"]
    /// Return vector of the non-unique source nodes names.
    ///
    /// Parameters
    /// --------------------------
    /// directed: bool,
    ///     whether to filter out the undirected edges.
    pub fn get_source_names(&self, directed: Option<bool>) -> Vec<String> {
        self.graph.get_source_names(directed.unwrap_or(true))
    }

    #[text_signature = "($self, directed)"]
    /// Return vector on the (non unique) destination nodes of the graph.
    ///
    /// Parameters
    /// --------------------------
    /// directed: bool,
    ///     whether to filter out the undirected edges.
    pub fn get_destination_names(&self, directed: Option<bool>) -> Vec<String> {
        self.graph.get_destination_names(directed.unwrap_or(true))
    }

    /// Return vector with the sorted nodes names.
    pub fn get_node_names(&self) -> Vec<String> {
        self.graph.get_node_names()
    }

    /// Return vector with the sorted nodes Ids.
    pub fn get_nodes(&self) -> Py<PyArray1<NodeT>> {
        let gil = pyo3::Python::acquire_gil();
        PyArray::from_vec(gil.python(), self.graph.get_nodes())
            .cast::<NodeT>(false)
            .unwrap()
            .to_owned()
    }

    /// Return vector of node types.
    pub fn get_node_types(&self) -> PyResult<Vec<Option<Vec<NodeTypeT>>>> {
        pe!(self.graph.get_node_types())
    }

    /// Return vector of edge types.
    pub fn get_edge_types(&self) -> PyResult<Vec<Option<EdgeTypeT>>> {
        pe!(self.graph.get_edge_types())
    }

    /// Return vector of weights.
    pub fn get_weights(&self) -> PyResult<Py<PyArray1<WeightT>>> {
        let gil = pyo3::Python::acquire_gil();
        Ok(to_ndarray_1d!(gil, pe!(self.graph.get_weights())?, WeightT))
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
    fn get_edge_type_id_by_edge_id(&self, edge_id: EdgeT) -> PyResult<Option<EdgeTypeT>> {
        pe!(self.graph.get_edge_type_id_by_edge_id(edge_id))
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
    fn get_node_type_id_by_node_id(&self, node_id: NodeT) -> PyResult<Option<Vec<NodeTypeT>>> {
        pe!(self.graph.get_node_type_id_by_node_id(node_id))
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
    fn get_node_name_by_node_id(&self, node_id: NodeT) -> PyResult<String> {
        pe!(self.graph.get_node_name_by_node_id(node_id))
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
    fn get_node_id_by_node_name(&self, node_name: &str) -> PyResult<NodeT> {
        pe!(self.graph.get_node_id_by_node_name(node_name))
    }

    #[text_signature = "($self)"]
    /// Return dictionary count of how many time an edge type appears.
    ///
    /// The dictionary looks like the following:
    ///
    /// {
    ///    edge_type_id: count_of_edge_types    
    /// }
    ///
    fn get_edge_type_counts(&self) -> PyResult<HashMap<EdgeTypeT, usize>> {
        pe!(self.graph.get_edge_type_counts_hashmap())
    }

    #[text_signature = "($self)"]
    /// Return dictionary count of how many time an node type appears.
    ///
    /// The dictionary looks like the following:
    ///
    /// {
    ///    node_type_id: count_of_node_types    
    /// }
    ///
    fn get_node_type_counts(&self) -> PyResult<HashMap<EdgeTypeT, usize>> {
        pe!(self.graph.get_node_type_counts_hashmap())
    }

    #[text_signature = "($self, node_name)"]
    /// Return node type ID for the given node name if available.
    ///
    /// Arguments
    /// -------------------
    /// node_name: str,
    ///     Name of the node.
    ///
    /// Returns
    /// -------------------
    /// Vector of the node type IDs or None if unknown.
    pub fn get_node_type_id_by_node_name(&self, node_name: &str) -> PyResult<Option<Vec<NodeTypeT>>> {
        pe!(self.graph.get_node_type_id_by_node_name(node_name))
    }

    #[text_signature = "($self, node_name)"]
    /// Return node type name for the given node name if available.
    ///
    /// Arguments
    /// -------------------
    /// node_name: str,
    ///     Name of the node.
    ///
    /// Returns
    /// -------------------
    /// Vector of the node type names or None if unknown.
    pub fn get_node_type_name_by_node_name(&self, node_name: &str) -> PyResult<Option<Vec<String>>> {
        pe!(self.graph.get_node_type_name_by_node_name(node_name))
    }

    #[text_signature = "($self, edge_id)"]
    /// Return the weight curresponding to the edge composed by the given edge ids.
    ///
    /// Parameters
    /// ---------------------
    /// edge_id: int,
    ///     ID of the edge.
    ///
    /// Returns
    /// ---------------------
    /// Weight of the edge.
    ///
    /// Raises
    /// ---------------------
    /// ValueError,
    ///     If the current graph instance does not have weights.
    /// ValueError,
    ///     If the given edge_id does not exist.
    pub fn get_weight_by_edge_id(&self, edge_id: EdgeT) -> PyResult<WeightT> {
        pe!(self.graph.get_weight_by_edge_id(edge_id))
    }

    #[text_signature = "($self, src, dst)"]
    /// Return the weight curresponding to the edge composed by the given node ids.
    ///
    /// Parameters
    /// ---------------------
    /// src: int,
    ///     id of the source node.
    /// dst: int,
    ///     id of the destination node.
    ///
    /// Returns
    /// ---------------------
    /// Weight of the edge.
    ///
    /// Raises
    /// ---------------------
    /// ValueError,
    ///     If the current graph instance does not have weights.
    /// ValueError,
    ///     If one or more of the given node ids do not exist in the graph.
    /// ValueError,
    ///     If the edge composed of the given node ids does not exist.
    pub fn get_weight_by_node_ids(&self, src: NodeT, dst: NodeT) -> PyResult<WeightT> {
        pe!(self.graph.get_weight_by_node_ids(src, dst))
    }

    #[text_signature = "($self, src_name, dst_name)"]
    /// Return the weight curresponding to the edge composed by the given node names.
    ///
    /// Parameters
    /// ---------------------
    /// src_name: str,
    ///     Name of the source node.
    /// dst_name: str,
    ///     Name of the destination node.
    ///
    /// Returns
    /// ---------------------
    /// Weight of the edge.
    ///
    /// Raises
    /// ---------------------
    /// ValueError,
    ///     If the current graph instance does not have weights.
    /// ValueError,
    ///     If one or more of the given node names do not exist in the graph.
    /// ValueError,
    ///     If the edge composed of the given node names does not exist.
    pub fn get_weight_by_node_names(&self, src_name: &str, dst_name: &str) -> PyResult<WeightT> {
        pe!(self.graph.get_weight_by_node_names(src_name, dst_name))
    }

    #[text_signature = "($self, node_id)"]
    /// Return vector of destination IDs for the given source node ID.
    ///
    /// Parameters
    /// ----------------
    /// node_id: int,
    ///     Node ID whose neighbours are to be retrieved.
    ///
    /// Returns
    /// ----------------
    /// Vector of the node IDs of the neighbours of given node.
    pub fn get_node_neighbours_by_node_id(&self, node_id: NodeT) -> PyResult<Vec<NodeT>> {
        pe!(self.graph.get_node_neighbours_by_node_id(node_id))
    }

    #[text_signature = "($self, node_name)"]
    /// Return vector of destination IDs for the given source node name.
    ///
    /// Parameters
    /// ----------------
    /// node_name: str,
    ///     Node name whose neighbours are to be retrieved.
    ///
    /// Returns
    /// ----------------
    /// Vector of the node IDs of the neighbours of given node.
    pub fn get_node_neighbours_by_node_name(&self, node_name: &str) -> PyResult<Vec<NodeT>> {
        pe!(self.graph.get_node_neighbours_by_node_name(node_name))
    }

    #[text_signature = "($self, node_name)"]
    /// Return vector of destination names for the given source node name.
    ///
    /// Parameters
    /// ----------------
    /// node_name: str,
    ///     Node name whose neighbours are to be retrieved.
    ///
    /// Returns
    /// ----------------
    /// Vector of the node names of the neighbours of given node.
    pub fn get_node_neighbours_name_by_node_name(&self, node_name: &str) -> PyResult<Vec<String>> {
        pe!(self.graph.get_node_neighbours_name_by_node_name(node_name))
    }

    #[text_signature = "($self)"]
    /// Returns a boolean representing if the graph contains an edge that has
    /// source == destination.
    fn has_selfloops(&self) -> bool {
        self.graph.has_selfloops()
    }

    #[text_signature = "($self)"]
    /// Returns true if the graph has weights.
    fn has_weights(&self) -> bool {
        self.graph.has_weights()
    }

    #[text_signature = "($self)"]
    /// Returns true if the graph has node types.
    fn has_node_types(&self) -> bool {
        self.graph.has_node_types()
    }

    #[text_signature = "($self)"]
    /// Returns true if the graph has edge types.
    fn has_edge_types(&self) -> bool {
        self.graph.has_edge_types()
    }
}
