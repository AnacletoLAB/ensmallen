use super::*;
impl Graph {

	#[text_signature = "($self, verbose)"]
	/// TODO!: This binding was automatically generated
	/// Returns number a triple with (number of components, number of nodes of the smallest component, number of nodes of the biggest component )
	/// 
	/// Paramenters
	/// --------------
	/// verbose : bool,
	/// 	Whether to show a loading bar or not.
	fn get_connected_components_number(&self, verbose : bool) -> (NodeT, NodeT, NodeT) {
		self.graph.get_connected_components_number(verbose)
	}
	
	#[text_signature = "($self)"]
	/// TODO!: This binding was automatically generated
	/// Returns number of singleton nodes within the graph.
	fn get_singleton_nodes_number(&self) -> NodeT {
		self.graph.get_singleton_nodes_number()
	}
	
	#[text_signature = "($self)"]
	/// TODO!: This binding was automatically generated
	/// Returns number of singleton nodes with self-loops within the graph.
	fn get_singleton_nodes_with_selfloops_number(&self) -> NodeT {
		self.graph.get_singleton_nodes_with_selfloops_number()
	}
	
	#[text_signature = "($self)"]
	/// TODO!: This binding was automatically generated
	/// Returns number of not singleton nodes within the graph.
	fn get_not_singleton_nodes_number(&self) -> NodeT {
		self.graph.get_not_singleton_nodes_number()
	}
	
	#[text_signature = "($self)"]
	/// TODO!: This binding was automatically generated
	/// Returns density of the graph.
	fn get_density(&self) -> PyResult<f64> {
		pe!(self.graph.get_density())
	}
	
	#[text_signature = "($self)"]
	/// TODO!: This binding was automatically generated
	/// Returns the traps rate of the graph.
	/// 
	/// THIS IS EXPERIMENTAL AND MUST BE PROVEN!
	fn get_trap_nodes_rate(&self) -> f64 {
		self.graph.get_trap_nodes_rate()
	}
	
	#[text_signature = "($self)"]
	/// TODO!: This binding was automatically generated
	/// Returns mean node degree of the graph.
	fn get_node_degrees_mean(&self) -> PyResult<f64> {
		pe!(self.graph.get_node_degrees_mean())
	}
	
	#[text_signature = "($self)"]
	/// TODO!: This binding was automatically generated
	/// Returns number of undirected edges of the graph.
	fn get_undirected_edges_number(&self) -> EdgeT {
		self.graph.get_undirected_edges_number()
	}
	
	#[text_signature = "($self)"]
	/// TODO!: This binding was automatically generated
	/// Returns number of undirected edges of the graph.
	fn get_unique_undirected_edges_number(&self) -> EdgeT {
		self.graph.get_unique_undirected_edges_number()
	}
	
	#[text_signature = "($self)"]
	/// TODO!: This binding was automatically generated
	/// Returns number of edges of the graph.
	fn get_edges_number(&self) -> EdgeT {
		self.graph.get_edges_number()
	}
	
	#[text_signature = "($self)"]
	/// TODO!: This binding was automatically generated
	/// Returns number of unique edges of the graph.
	fn get_unique_edges_number(&self) -> EdgeT {
		self.graph.get_unique_edges_number()
	}
	
	#[text_signature = "($self)"]
	/// TODO!: This binding was automatically generated
	/// Returns median node degree of the graph
	fn get_node_degrees_median(&self) -> PyResult<NodeT> {
		pe!(self.graph.get_node_degrees_median())
	}
	
	#[text_signature = "($self)"]
	/// TODO!: This binding was automatically generated
	/// Returns maximum node degree of the graph.
	fn get_max_node_degree(&self) -> PyResult<NodeT> {
		pe!(self.graph.get_max_node_degree())
	}
	
	#[text_signature = "($self)"]
	/// TODO!: This binding was automatically generated
	/// Returns minimum node degree of the graph.
	fn get_min_node_degree(&self) -> PyResult<NodeT> {
		pe!(self.graph.get_min_node_degree())
	}
	
	#[text_signature = "($self)"]
	/// TODO!: This binding was automatically generated
	/// Returns mode node degree of the graph.
	fn get_node_degrees_mode(&self) -> PyResult<NodeT> {
		pe!(self.graph.get_node_degrees_mode())
	}
	
	#[text_signature = "($self)"]
	/// TODO!: This binding was automatically generated
	/// Returns number of self-loops, including also those in eventual multi-edges.
	fn get_selfloop_nodes_number(&self) -> EdgeT {
		self.graph.get_selfloop_nodes_number()
	}
	
	#[text_signature = "($self)"]
	/// TODO!: This binding was automatically generated
	/// Returns number of unique self-loops, excluding those in eventual multi-edges.
	fn get_unique_selfloop_number(&self) -> NodeT {
		self.graph.get_unique_selfloop_number()
	}
	
	#[text_signature = "($self)"]
	/// TODO!: This binding was automatically generated
	/// Returns rate of self-loops.
	fn get_selfloop_nodes_rate(&self) -> PyResult<f64> {
		pe!(self.graph.get_selfloop_nodes_rate())
	}
	
	#[text_signature = "($self)"]
	/// TODO!: This binding was automatically generated
	/// Return name of the graph.
	fn get_name(&self) -> String {
		self.graph.get_name()
	}
	
	#[text_signature = "($self)"]
	/// TODO!: This binding was automatically generated
	/// Return the number of traps (nodes without any outgoing edges that are not singletons)
	/// This also includes nodes with only a self-loops, therefore singletons with
	/// only a self-loops are not considered traps because you could make a walk on them.
	fn get_trap_nodes_number(&self) -> EdgeT {
		self.graph.get_trap_nodes_number()
	}
	
	#[text_signature = "($self, directed)"]
	/// TODO!: This binding was automatically generated
	/// Return vector of the non-unique source nodes.
	/// 
	/// Paramenters
	/// --------------
	/// directed : bool,
	/// 	Whether to filter out the undirected edges.
	fn get_sources(&self, directed : bool) -> Vec<NodeT> {
		self.graph.get_sources(directed)
	}
	
	#[text_signature = "($self, directed)"]
	/// TODO!: This binding was automatically generated
	/// Return vector of the non-unique source nodes names.
	/// 
	/// Paramenters
	/// --------------
	/// directed : bool,
	/// 	Whether to filter out the undirected edges.
	fn get_source_names(&self, directed : bool) -> Vec<String> {
		self.graph.get_source_names(directed)
	}
	
	#[text_signature = "($self, directed)"]
	/// TODO!: This binding was automatically generated
	/// Return vector on the (non unique) destination nodes of the graph.
	/// 
	/// Paramenters
	/// --------------
	/// directed : bool,
	/// 	Whether to filter out the undirected edges.
	fn get_destinations(&self, directed : bool) -> Vec<NodeT> {
		self.graph.get_destinations(directed)
	}
	
	#[text_signature = "($self, directed)"]
	/// TODO!: This binding was automatically generated
	/// Return vector of the non-unique destination nodes names.
	/// 
	/// Paramenters
	/// --------------
	/// directed : bool,
	/// 	Whether to filter out the undirected edges.
	fn get_destination_names(&self, directed : bool) -> Vec<String> {
		self.graph.get_destination_names(directed)
	}
	
	#[text_signature = "($self)"]
	/// TODO!: This binding was automatically generated
	/// Return vector with the sorted nodes names.
	fn get_node_names(&self) -> Vec<String> {
		self.graph.get_node_names()
	}
	
	#[text_signature = "($self)"]
	/// TODO!: This binding was automatically generated
	/// Return vector with the sorted nodes Ids.
	fn get_nodes(&self) -> Vec<NodeT> {
		self.graph.get_nodes()
	}
	
	#[text_signature = "($self)"]
	/// TODO!: This binding was automatically generated
	/// Return the edge types of the edges.
	fn get_edge_types(&self) -> PyResult<Vec<Option<EdgeTypeT>>> {
		pe!(self.graph.get_edge_types())
	}
	
	#[text_signature = "($self)"]
	/// TODO!: This binding was automatically generated
	/// Return the edge types names.
	fn get_edge_type_names(&self) -> Option<Vec<String>> {
		self.graph.get_edge_type_names()
	}
	
	#[text_signature = "($self)"]
	/// TODO!: This binding was automatically generated
	/// Return the weights of the graph edges.
	fn get_edge_weights(&self) -> PyResult<Vec<WeightT>> {
		pe!(self.graph.get_edge_weights())
	}
	
	#[text_signature = "($self)"]
	/// TODO!: This binding was automatically generated
	/// Return the minimum weight, if graph has weights.
	fn get_min_edge_weight(&self) -> PyResult<WeightT> {
		pe!(self.graph.get_min_edge_weight())
	}
	
	#[text_signature = "($self)"]
	/// TODO!: This binding was automatically generated
	/// Return the maximum weight, if graph has weights.
	fn get_max_edge_weight(&self) -> PyResult<WeightT> {
		pe!(self.graph.get_max_edge_weight())
	}
	
	#[text_signature = "($self)"]
	/// TODO!: This binding was automatically generated
	/// Return the node types of the graph nodes.
	fn get_node_type_ids(&self) -> PyResult<Vec<Option<Vec<NodeTypeT>>>> {
		pe!(self.graph.get_node_type_ids())
	}
	
	#[text_signature = "($self)"]
	/// TODO!: This binding was automatically generated
	/// Return the node types names.
	fn get_node_type_names(&self) -> PyResult<Vec<String>> {
		pe!(self.graph.get_node_type_names())
	}
	
	#[text_signature = "($self)"]
	/// TODO!: This binding was automatically generated
	/// Return number of the unique edges in the graph.
	fn get_unique_directed_edges_number(&self) -> EdgeT {
		self.graph.get_unique_directed_edges_number()
	}
	
	#[text_signature = "($self)"]
	/// TODO!: This binding was automatically generated
	/// Return the nodes mapping.
	fn get_nodes_mapping(&self) -> HashMap<String, NodeT> {
		self.graph.get_nodes_mapping()
	}
	
	#[text_signature = "($self, directed)"]
	/// TODO!: This binding was automatically generated
	/// Return vector with the sorted edge Ids.
	/// 
	/// Paramenters
	/// --------------
	/// directed : bool,
	/// 	Whether to filter out the undirected edges.
	fn get_edges(&self, directed : bool) -> Vec<Vec<NodeT>> {
		self.graph.get_edges(directed)
	}
	
	#[text_signature = "($self, directed)"]
	/// TODO!: This binding was automatically generated
	/// Return vector with the sorted edge names.
	/// 
	/// Paramenters
	/// --------------
	/// directed : bool,
	/// 	Whether to filter out the undirected edges.
	fn get_edge_node_names(&self, directed : bool) -> Vec<(String, String)> {
		self.graph.get_edge_node_names(directed)
	}
	
	#[text_signature = "($self)"]
	/// TODO!: This binding was automatically generated
	/// Returns number of unknown node types.
	fn get_unknown_node_types_number(&self) -> NodeT {
		self.graph.get_unknown_node_types_number()
	}
	
	#[text_signature = "($self)"]
	/// TODO!: This binding was automatically generated
	/// Returns minimum number of node types.
	fn get_minimum_node_types_number(&self) -> NodeT {
		self.graph.get_minimum_node_types_number()
	}
	
	#[text_signature = "($self)"]
	/// TODO!: This binding was automatically generated
	/// Returns number of unknown edge types.
	fn get_unknown_edge_types_number(&self) -> EdgeT {
		self.graph.get_unknown_edge_types_number()
	}
	
	#[text_signature = "($self)"]
	/// TODO!: This binding was automatically generated
	/// Returns minimum number of edge types.
	fn get_minimum_edge_types_number(&self) -> EdgeT {
		self.graph.get_minimum_edge_types_number()
	}
	
	#[text_signature = "($self)"]
	/// TODO!: This binding was automatically generated
	/// Returns number of nodes in the graph.
	fn get_nodes_number(&self) -> NodeT {
		self.graph.get_nodes_number()
	}
	
	#[text_signature = "($self, verbose)"]
	/// TODO!: This binding was automatically generated
	/// Return a vector with the components each node belongs to.
	/// 
	/// E.g. If we have two components `[0, 2, 3]` and `[1, 4, 5]` the result will look like
	/// `[0, 1, 0, 0, 1, 1]`
	/// 
	/// Paramenters
	/// --------------
	/// verbose : bool,
	/// 	Whether to show the loading bar.
	fn get_node_connected_component_ids(&self, verbose : bool) -> Vec<NodeT> {
		self.graph.get_node_connected_component_ids(verbose)
	}
	
	#[text_signature = "($self)"]
	/// TODO!: This binding was automatically generated
	/// Returns number of directed edges in the graph.
	fn get_directed_edges_number(&self) -> EdgeT {
		self.graph.get_directed_edges_number()
	}
	
	#[text_signature = "($self)"]
	/// TODO!: This binding was automatically generated
	/// Returns number of edge types in the graph.
	fn get_edge_types_number(&self) -> EdgeTypeT {
		self.graph.get_edge_types_number()
	}
	
	#[text_signature = "($self)"]
	/// TODO!: This binding was automatically generated
	/// Returns number of node types in the graph.
	fn get_node_types_number(&self) -> NodeTypeT {
		self.graph.get_node_types_number()
	}
	
	#[text_signature = "($self)"]
	/// TODO!: This binding was automatically generated
	/// Returns the degree of every node in the graph.
	fn get_node_degrees(&self) -> Vec<NodeT> {
		self.graph.get_node_degrees()
	}
	
	#[text_signature = "($self)"]
	/// TODO!: This binding was automatically generated
	/// Return set of nodes that are not singletons.
	fn get_not_singletons_node_ids(&self) -> Vec<NodeT> {
		self.graph.get_not_singletons_node_ids()
	}
	
	#[text_signature = "($self)"]
	/// TODO!: This binding was automatically generated
	/// Return mapping from instance not trap nodes to dense nodes.
	fn get_dense_nodes_mapping(&self) -> HashMap<NodeT, NodeT> {
		self.graph.get_dense_nodes_mapping()
	}
	
	#[text_signature = "($self)"]
	/// TODO!: This binding was automatically generated
	/// Return number of edges that have multigraph syblings.
	fn get_multigraph_edges_number(&self) -> EdgeT {
		self.graph.get_multigraph_edges_number()
	}
	
	#[text_signature = "($self)"]
	/// TODO!: This binding was automatically generated
	/// Return vector with node cumulative_node_degrees, that is the comulative node degree.
	fn get_cumulative_node_degrees(&self) -> Vec<EdgeT> {
		self.graph.get_cumulative_node_degrees()
	}
	
	#[text_signature = "($self)"]
	/// TODO!: This binding was automatically generated
	/// Returns number of the source nodes.
	fn get_unique_source_nodes_number(&self) -> NodeT {
		self.graph.get_unique_source_nodes_number()
	}
	
	#[text_signature = "($self)"]
	/// TODO!: This binding was automatically generated
	/// Returns edge type counts.
	fn get_edge_type_counter(&self) -> PyResult<Counter<EdgeTypeT, usize>> {
		pe!(self.graph.get_edge_type_counter())
	}
	
	#[text_signature = "($self)"]
	/// TODO!: This binding was automatically generated
	/// Returns edge type counts hashmap.
	fn get_edge_type_counts_hashmap(&self) -> PyResult<HashMap<EdgeTypeT, usize>> {
		pe!(self.graph.get_edge_type_counts_hashmap())
	}
	
	#[text_signature = "($self)"]
	/// TODO!: This binding was automatically generated
	/// Returns node type counts.
	fn get_node_type_counter(&self) -> PyResult<Counter<NodeTypeT, usize>> {
		pe!(self.graph.get_node_type_counter())
	}
	
	#[text_signature = "($self)"]
	/// TODO!: This binding was automatically generated
	/// Returns node type counts hashmap.
	fn get_node_type_counts_hashmap(&self) -> PyResult<HashMap<EdgeTypeT, usize>> {
		pe!(self.graph.get_node_type_counts_hashmap())
	}
	
}
