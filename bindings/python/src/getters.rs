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
    fn get_edge_types_number(&self) -> PyResult<EdgeTypeT> {
        pe!(self.graph.get_edge_types_number())
    }

    #[text_signature = "($self)"]
    /// Return the number of edges in the graph.
    ///
    /// This method will include, if found necessary by a missing value,
    /// also the default node type in the count of total node types.
    ///
    fn get_node_types_number(&self) -> PyResult<NodeTypeT> {
        pe!(self.graph.get_node_types_number())
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
    fn get_source_node_ids(&self, directed: Option<bool>) -> Py<PyArray1<NodeT>> {
        let gil = pyo3::Python::acquire_gil();
        to_ndarray_1d!(
            gil,
            self.graph.get_source_node_ids(directed.unwrap_or(true)),
            NodeT
        )
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
    fn get_destination_node_ids(&self, directed: Option<bool>) -> Py<PyArray1<NodeT>> {
        let gil = pyo3::Python::acquire_gil();
        to_ndarray_1d!(
            gil,
            self.graph
                .get_destination_node_ids(directed.unwrap_or(true)),
            NodeT
        )
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
    pub fn get_edge_node_ids(&self, directed: Option<bool>) -> PyResult<Py<PyArray2<NodeT>>> {
        let gil = pyo3::Python::acquire_gil();
        Ok(to_nparray_2d!(
            gil,
            self.graph.get_edge_node_ids(directed.unwrap_or(true)),
            NodeT
        ))
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
    pub fn get_node_ids(&self) -> Py<PyArray1<NodeT>> {
        let gil = pyo3::Python::acquire_gil();
        PyArray::from_vec(gil.python(), self.graph.get_node_ids())
            .cast::<NodeT>(false)
            .unwrap()
            .to_owned()
    }

    /// Return vector of weights.
    pub fn get_edge_weights(&self) -> PyResult<Py<PyArray1<WeightT>>> {
        let gil = pyo3::Python::acquire_gil();
        Ok(to_ndarray_1d!(
            gil,
            pe!(self.graph.get_edge_weights())?,
            WeightT
        ))
    }

    /// Return vector of node types_name.
    pub fn get_node_type_names(&self) -> PyResult<Vec<Option<Vec<String>>>> {
        pe!(self.graph.get_node_type_names())
    }

    /// Return vector of edge types_name.
    pub fn get_edge_type_names(&self) -> PyResult<Vec<Option<String>>> {
        pe!(self.graph.get_edge_type_names())
    }

    /// Return dictionary of strings to Ids representing the ndoes mapping.
    pub fn get_nodes_mapping(&self) -> HashMap<String, NodeT> {
        self.graph.get_nodes_mapping()
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
    fn get_edge_type_counts(&self) -> PyResult<HashMap<EdgeTypeT, EdgeT>> {
        pe!(self.graph.get_edge_type_id_counts_hashmap())
    }

    #[text_signature = "($self)"]
    /// Returns a boolean representing if the graph contains an edge that has
    /// source == destination.
    fn has_selfloops(&self) -> bool {
        self.graph.has_selfloops()
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
    #[text_signature = "($self, verbose)"]
    /// Returns number a triple with (number of components, number of nodes of the smallest component, number of nodes of the biggest component )
    ///
    /// Parameters
    /// --------------
    /// verbose: bool,
    /// 	Whether to show a loading bar or not.
    ///
    /// [Automatically generated binding]
    /// [Automatically generated documentation]
    fn get_connected_components_number(&self, verbose: bool) -> (NodeT, NodeT, NodeT) {
        self.graph.get_connected_components_number(verbose)
    }

    #[text_signature = "($self)"]
    /// Return vector with node cumulative_node_degrees, that is the comulative node degree.
    ///
    /// [Automatically generated binding]
    /// [Automatically generated documentation]
    fn get_cumulative_node_degrees(&self) -> Vec<EdgeT> {
        self.graph.get_cumulative_node_degrees()
    }

    #[text_signature = "($self)"]
    /// Return mapping from instance not trap nodes to dense nodes.
    ///
    /// [Automatically generated binding]
    /// [Automatically generated documentation]
    fn get_dense_nodes_mapping(&self) -> HashMap<NodeT, NodeT> {
        self.graph.get_dense_nodes_mapping()
    }

    #[text_signature = "($self)"]
    /// Returns density of the graph.
    ///
    /// [Automatically generated binding]
    /// [Automatically generated documentation]
    fn get_density(&self) -> PyResult<f64> {
        pe!(self.graph.get_density())
    }

    #[text_signature = "($self)"]
    /// Returns number of directed edges in the graph.
    ///
    /// [Automatically generated binding]
    /// [Automatically generated documentation]
    fn get_directed_edges_number(&self) -> EdgeT {
        self.graph.get_directed_edges_number()
    }

    #[text_signature = "($self, directed)"]
    /// Return vector with the sorted edge names.
    ///
    /// Parameters
    /// --------------
    /// directed: bool,
    /// 	Whether to filter out the undirected edges.
    ///
    /// [Automatically generated binding]
    /// [Automatically generated documentation]
    fn get_edge_node_names(&self, directed: bool) -> Vec<(String, String)> {
        self.graph.get_edge_node_names(directed)
    }

    #[text_signature = "($self)"]
    /// Returns edge type counts hashmap.
    ///
    /// [Automatically generated binding]
    /// [Automatically generated documentation]
    fn get_edge_type_id_counts_hashmap(&self) -> PyResult<HashMap<EdgeTypeT, EdgeT>> {
        pe!(self.graph.get_edge_type_id_counts_hashmap())
    }

    #[text_signature = "($self)"]
    /// Return the maximum weight, if graph has weights.
    ///
    /// [Automatically generated binding]
    /// [Automatically generated documentation]
    fn get_max_edge_weight(&self) -> PyResult<WeightT> {
        pe!(self.graph.get_max_edge_weight())
    }

    #[text_signature = "($self)"]
    /// Return the minimum weight, if graph has weights.
    ///
    /// [Automatically generated binding]
    /// [Automatically generated documentation]
    fn get_min_edge_weight(&self) -> PyResult<WeightT> {
        pe!(self.graph.get_min_edge_weight())
    }

    #[text_signature = "($self)"]
    /// Returns minimum number of edge types.
    ///
    /// [Automatically generated binding]
    /// [Automatically generated documentation]
    fn get_minimum_edge_types_number(&self) -> PyResult<EdgeT> {
        pe!(self.graph.get_minimum_edge_types_number())
    }

    #[text_signature = "($self)"]
    /// Returns minimum number of node types.
    ///
    /// [Automatically generated binding]
    /// [Automatically generated documentation]
    fn get_minimum_node_types_number(&self) -> PyResult<NodeT> {
        pe!(self.graph.get_minimum_node_types_number())
    }

    #[text_signature = "($self)"]
    /// Return number of edges that have multigraph syblings.
    ///
    /// [Automatically generated binding]
    /// [Automatically generated documentation]
    fn get_multigraph_edges_number(&self) -> EdgeT {
        self.graph.get_multigraph_edges_number()
    }

    #[text_signature = "($self, verbose)"]
    /// Return a vector with the components each node belongs to.
    ///
    ///  E.g. If we have two components `[0, 2, 3]` and `[1, 4, 5]` the result will look like
    ///  `[0, 1, 0, 0, 1, 1]`
    ///
    /// Parameters
    /// --------------
    /// verbose: bool,
    /// 	Whether to show the loading bar.
    ///
    /// [Automatically generated binding]
    /// [Automatically generated documentation]
    fn get_node_connected_component_ids(&self, verbose: bool) -> Vec<NodeT> {
        self.graph.get_node_connected_component_ids(verbose)
    }

    #[text_signature = "($self)"]
    /// Returns the degree of every node in the graph.
    ///
    /// [Automatically generated binding]
    /// [Automatically generated documentation]
    fn get_node_degrees(&self) -> Vec<NodeT> {
        self.graph.get_node_degrees()
    }

    #[text_signature = "($self)"]
    /// Return the node types of the graph nodes.
    ///
    /// [Automatically generated binding]
    /// [Automatically generated documentation]
    fn get_node_type_ids(&self) -> PyResult<Vec<Option<Vec<NodeTypeT>>>> {
        pe!(self.graph.get_node_type_ids())
    }

    #[text_signature = "($self)"]
    /// Return the edge types of the graph edges.
    ///
    /// [Automatically generated binding]
    /// [Automatically generated documentation]
    fn get_edge_type_ids(&self) -> PyResult<Vec<Option<EdgeTypeT>>> {
        pe!(self.graph.get_edge_type_ids())
    }

    #[text_signature = "($self)"]
    /// Returns number of not singleton nodes within the graph.
    ///
    /// [Automatically generated binding]
    /// [Automatically generated documentation]
    fn get_not_singleton_nodes_number(&self) -> NodeT {
        self.graph.get_not_singleton_nodes_number()
    }

    #[text_signature = "($self)"]
    /// Return set of nodes that are not singletons.
    ///
    /// [Automatically generated binding]
    /// [Automatically generated documentation]
    fn get_not_singletons_node_ids(&self) -> Vec<NodeT> {
        self.graph.get_not_singletons_node_ids()
    }

    #[text_signature = "($self)"]
    /// Returns number of self-loops, including also those in eventual multi-edges.
    ///
    /// [Automatically generated binding]
    /// [Automatically generated documentation]
    fn get_selfloop_nodes_number(&self) -> EdgeT {
        self.graph.get_selfloop_nodes_number()
    }

    #[text_signature = "($self)"]
    /// Returns rate of self-loops.
    ///
    /// [Automatically generated binding]
    /// [Automatically generated documentation]
    fn get_selfloop_nodes_rate(&self) -> PyResult<f64> {
        pe!(self.graph.get_selfloop_nodes_rate())
    }

    #[text_signature = "($self)"]
    /// Returns number of singleton nodes within the graph.
    ///
    /// [Automatically generated binding]
    /// [Automatically generated documentation]
    fn get_singleton_nodes_number(&self) -> NodeT {
        self.graph.get_singleton_nodes_number()
    }

    #[text_signature = "($self)"]
    /// Returns number of singleton nodes with self-loops within the graph.
    ///
    /// [Automatically generated binding]
    /// [Automatically generated documentation]
    fn get_singleton_nodes_with_selfloops_number(&self) -> NodeT {
        self.graph.get_singleton_nodes_with_selfloops_number()
    }

    #[text_signature = "($self)"]
    /// Return the number of traps (nodes without any outgoing edges that are not singletons)
    ///  This also includes nodes with only a self-loops, therefore singletons with
    ///  only a self-loops are not considered traps because you could make a walk on them.
    ///
    /// [Automatically generated binding]
    /// [Automatically generated documentation]
    fn get_trap_nodes_number(&self) -> EdgeT {
        self.graph.get_trap_nodes_number()
    }

    #[text_signature = "($self)"]
    /// Returns the traps rate of the graph.
    ///
    ///  THIS IS EXPERIMENTAL AND MUST BE PROVEN!
    ///
    /// [Automatically generated binding]
    /// [Automatically generated documentation]
    fn get_trap_nodes_rate(&self) -> f64 {
        self.graph.get_trap_nodes_rate()
    }

    #[text_signature = "($self)"]
    /// Return number of the unique edges in the graph.
    ///
    /// [Automatically generated binding]
    /// [Automatically generated documentation]
    fn get_unique_directed_edges_number(&self) -> EdgeT {
        self.graph.get_unique_directed_edges_number()
    }

    #[text_signature = "($self)"]
    /// Returns number of unique edges of the graph.
    ///
    /// [Automatically generated binding]
    /// [Automatically generated documentation]
    fn get_unique_edges_number(&self) -> EdgeT {
        self.graph.get_unique_edges_number()
    }

    #[text_signature = "($self)"]
    /// Returns number of unique self-loops, excluding those in eventual multi-edges.
    ///
    /// [Automatically generated binding]
    /// [Automatically generated documentation]
    fn get_unique_selfloop_number(&self) -> NodeT {
        self.graph.get_unique_selfloop_number()
    }

    #[text_signature = "($self)"]
    /// Returns number of undirected edges of the graph.
    ///
    /// [Automatically generated binding]
    /// [Automatically generated documentation]
    fn get_unique_undirected_edges_number(&self) -> EdgeT {
        self.graph.get_unique_undirected_edges_number()
    }

    #[text_signature = "($self)"]
    /// Returns number of unknown edge types.
    ///
    /// [Automatically generated binding]
    /// [Automatically generated documentation]
    fn get_unknown_edge_types_number(&self) -> PyResult<EdgeT> {
        pe!(self.graph.get_unknown_edge_types_number())
    }

    #[text_signature = "($self)"]
    /// Returns number of unknown node types.
    ///
    /// [Automatically generated binding]
    /// [Automatically generated documentation]
    fn get_unknown_node_types_number(&self) -> PyResult<NodeT> {
        pe!(self.graph.get_unknown_node_types_number())
    }

    #[text_signature = "($self)"]
    /// Returns edge type names counts hashmap.
    ///
    ///  # Raises
    ///  * If there are no edge types in the current graph instance.
    ///
    /// [Automatically generated binding]
    /// [Automatically generated documentation]
    fn get_edge_type_names_counts_hashmap(&self) -> PyResult<HashMap<String, EdgeT>> {
        pe!(self.graph.get_edge_type_names_counts_hashmap())
    }

    #[text_signature = "($self)"]
    /// Returns node type IDs counts hashmap.
    ///
    ///  # Raises
    ///  * If there are no node types in the current graph instance.
    ///
    /// [Automatically generated binding]
    /// [Automatically generated documentation]
    fn get_node_type_id_counts_hashmap(&self) -> PyResult<HashMap<NodeTypeT, NodeT>> {
        pe!(self.graph.get_node_type_id_counts_hashmap())
    }

    #[text_signature = "($self)"]
    /// Returns node type names counts hashmap.
    ///
    ///  # Raises
    ///  * If there are no node types in the current graph instance.
    ///
    /// [Automatically generated binding]
    /// [Automatically generated documentation]
    fn get_node_type_names_counts_hashmap(&self) -> PyResult<HashMap<String, NodeT>> {
        pe!(self.graph.get_node_type_names_counts_hashmap())
    }

    #[text_signature = "($self)"]
    /// Returns vector of singleton edge types IDs.
    ///
    ///  # Raises
    ///  * If the graph does not have edge types.
    ///
    /// [Automatically generated binding]
    /// [Automatically generated documentation]
    fn get_singleton_edge_type_ids(&self) -> PyResult<Vec<EdgeTypeT>> {
        pe!(self.graph.get_singleton_edge_type_ids())
    }

    #[text_signature = "($self)"]
    /// Returns vector of singleton edge types names.
    ///
    ///  # Raises
    ///  * If the graph does not have edge types.
    ///
    /// [Automatically generated binding]
    /// [Automatically generated documentation]
    fn get_singleton_edge_type_names(&self) -> PyResult<Vec<String>> {
        pe!(self.graph.get_singleton_edge_type_names())
    }

    #[text_signature = "($self)"]
    /// Returns number of singleton edge types.
    ///
    ///  # Raises
    ///  * If the graph does not have edge types.
    ///
    /// [Automatically generated binding]
    /// [Automatically generated documentation]
    fn get_singleton_edge_types_number(&self) -> PyResult<EdgeTypeT> {
        pe!(self.graph.get_singleton_edge_types_number())
    }

    #[text_signature = "($self)"]
    /// Returns vector of singleton node types IDs.
    ///
    ///  # Raises
    ///  * If the graph does not have node types.
    ///
    /// [Automatically generated binding]
    /// [Automatically generated documentation]
    fn get_singleton_node_type_ids(&self) -> PyResult<Vec<NodeTypeT>> {
        pe!(self.graph.get_singleton_node_type_ids())
    }

    #[text_signature = "($self)"]
    /// Returns vector of singleton node types names.
    ///
    ///  # Raises
    ///  * If the graph does not have node types.
    ///
    /// [Automatically generated binding]
    /// [Automatically generated documentation]
    fn get_singleton_node_type_names(&self) -> PyResult<Vec<String>> {
        pe!(self.graph.get_singleton_node_type_names())
    }

    #[text_signature = "($self)"]
    /// Returns number of singleton node types.
    ///
    ///  # Raises
    ///  * If the graph does not have node types.
    ///
    /// [Automatically generated binding]
    /// [Automatically generated documentation]
    fn get_singleton_node_types_number(&self) -> PyResult<NodeTypeT> {
        pe!(self.graph.get_singleton_node_types_number())
    }

    #[text_signature = "($self)"]
    /// Return the unique edge type IDs of the graph edges.
    ///
    /// [Automatically generated binding]
    /// [Automatically generated documentation]
    fn get_unique_edge_type_ids(&self) -> PyResult<Vec<EdgeTypeT>> {
        pe!(self.graph.get_unique_edge_type_ids())
    }

    #[text_signature = "($self)"]
    /// Return the edge types names.
    ///
    /// [Automatically generated binding]
    /// [Automatically generated documentation]
    fn get_unique_edge_type_names(&self) -> PyResult<Vec<String>> {
        pe!(self.graph.get_unique_edge_type_names())
    }

    #[text_signature = "($self)"]
    /// Return the unique node type IDs of the graph nodes.
    ///
    /// [Automatically generated binding]
    /// [Automatically generated documentation]
    fn get_unique_node_type_ids(&self) -> PyResult<Vec<NodeTypeT>> {
        pe!(self.graph.get_unique_node_type_ids())
    }

    #[text_signature = "($self)"]
    /// Return the unique node types names.
    ///
    /// [Automatically generated binding]
    /// [Automatically generated documentation]
    fn get_unique_node_type_names(&self) -> PyResult<Vec<String>> {
        pe!(self.graph.get_unique_node_type_names())
    }

    #[text_signature = "($self)"]
    /// Returns rate of unknown edge types over total edges number.
    ///
    /// [Automatically generated binding]
    /// [Automatically generated documentation]
    fn get_unknown_edge_types_rate(&self) -> PyResult<f64> {
        pe!(self.graph.get_unknown_edge_types_rate())
    }

    #[text_signature = "($self)"]
    /// Returns rate of unknown node types over total nodes number.
    ///
    /// [Automatically generated binding]
    /// [Automatically generated documentation]
    fn get_unknown_node_types_rate(&self) -> PyResult<f64> {
        pe!(self.graph.get_unknown_node_types_rate())
    }

    #[text_signature = "($self)"]
    /// Returns vector of singleton node IDs of the graph.
    ///
    /// [Automatically generated binding]
    /// [Automatically generated documentation]
    fn get_singleton_node_ids(&self) -> Vec<NodeT> {
        self.graph.get_singleton_node_ids()
    }

    #[text_signature = "($self)"]
    /// Returns vector of singleton node names of the graph.
    ///
    /// [Automatically generated binding]
    /// [Automatically generated documentation]
    fn get_singleton_node_names(&self) -> Vec<String> {
        self.graph.get_singleton_node_names()
    }

    #[text_signature = "($self)"]
    /// Returns vector of singleton_with_selfloops node IDs of the graph.
    ///
    /// [Automatically generated binding]
    /// [Automatically generated documentation]
    fn get_singleton_with_selfloops_node_ids(&self) -> Vec<NodeT> {
        self.graph.get_singleton_with_selfloops_node_ids()
    }

    #[text_signature = "($self)"]
    /// Returns vector of singleton_with_selfloops node names of the graph.
    ///
    /// [Automatically generated binding]
    /// [Automatically generated documentation]
    fn get_singleton_with_selfloops_node_names(&self) -> Vec<String> {
        self.graph.get_singleton_with_selfloops_node_names()
    }
}
