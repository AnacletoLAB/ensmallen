use super::*;
impl Graph {

	#[text_signature = "($self, edge_id)"]
	/// Returns option with the weight of the given edge id.
	/// 
	/// This method will raise a panic if the given edge ID is higher than
	/// the number of edges in the graph. Additionally, it will simply
	/// return None if there are no graph weights.
	/// 
	/// Paramenters
	/// --------------
	/// edge_id : int,
	/// 	The edge whose edge weight is to be returned.
	///
	/// [Automatically generated binding]
	/// [Automatically generated documentation]
	fn get_unchecked_edge_weight_from_edge_id(&self, edge_id : EdgeT) -> Option<WeightT> {
		self.graph.get_unchecked_edge_weight_from_edge_id(edge_id)
	}
	
	#[text_signature = "($self, node_name)"]
	/// Returns node id from given node name raising a panic if used unproperly.
	/// 
	/// Paramenters
	/// --------------
	/// node_name : str,
	/// 	The node name whose node ID is to be returned.
	///
	/// [Automatically generated binding]
	/// [Automatically generated documentation]
	fn get_unchecked_node_id_from_node_name(&self, node_name : &str) -> NodeT {
		self.graph.get_unchecked_node_id_from_node_name(node_name)
	}
	
	#[text_signature = "($self, edge_type_name)"]
	/// Return edge type ID corresponding to the given edge type name.
	/// 
	/// Paramenters
	/// --------------
	/// edge_type_name : str,
	/// 	The edge type name whose edge type ID is to be returned.
	///
	/// [Automatically generated binding]
	/// [Automatically generated documentation]
	fn get_unchecked_edge_type_id_from_edge_type_name(&self, edge_type_name : &str) -> Option<EdgeTypeT> {
		self.graph.get_unchecked_edge_type_id_from_edge_type_name(edge_type_name)
	}
	
	#[text_signature = "($self, edge_type_id)"]
	/// Return edge type ID corresponding to the given edge type name
	/// raising panic if edge type ID does not exists in current graph.
	/// 
	/// Paramenters
	/// --------------
	/// edge_type_id : int,
	/// 	The edge type naIDme whose edge type name is to be returned.
	///
	/// [Automatically generated binding]
	/// [Automatically generated documentation]
	fn get_unchecked_edge_type_name_from_edge_type_id(&self, edge_type_id : Option<EdgeTypeT>) -> Option<String> {
		self.graph.get_unchecked_edge_type_name_from_edge_type_id(edge_type_id)
	}
	
	#[text_signature = "($self, edge_type)"]
	/// Return number of edges of the given edge type without checks.
	/// 
	/// Paramenters
	/// --------------
	/// edge_type : int,
	/// 	The edge type to retrieve count of.
	///
	/// [Automatically generated binding]
	/// [Automatically generated documentation]
	fn get_unchecked_edge_count_from_edge_type_id(&self, edge_type : Option<EdgeTypeT>) -> EdgeT {
		self.graph.get_unchecked_edge_count_from_edge_type_id(edge_type)
	}
	
	#[text_signature = "($self, src, dst, edge_type)"]
	/// Return edge ID without any checks for given tuple of nodes and edge type.
	/// 
	/// This method will cause a panic if used improperly when it is not certain
	/// that the edge exists.
	/// 
	/// Paramenters
	/// --------------
	/// src : int,
	/// 	Source node of the edge.
	/// dst : int,
	/// 	Destination node of the edge.
	/// edge_type : int,
	/// 	Edge Type of the edge.
	///
	/// [Automatically generated binding]
	/// [Automatically generated documentation]
	fn get_unchecked_edge_id_from_node_ids_and_edge_type_id(&self, src : NodeT, dst : NodeT, edge_type : Option<EdgeTypeT>) -> EdgeT {
		self.graph.get_unchecked_edge_id_from_node_ids_and_edge_type_id(src, dst, edge_type)
	}
	
	#[text_signature = "($self, src, dst)"]
	/// Return range of outbound edges IDs for all the edges bewteen the given
	/// source and destination nodes.
	/// This operation is meaningfull only in a multigraph.
	/// 
	/// Paramenters
	/// --------------
	/// src : int,
	/// 	Source node.
	/// dst : int,
	/// 	Destination node.
	///
	/// [Automatically generated binding]
	/// [Automatically generated documentation]
	fn get_unchecked_minmax_edge_ids_from_node_ids(&self, src : NodeT, dst : NodeT) -> (EdgeT, EdgeT) {
		self.graph.get_unchecked_minmax_edge_ids_from_node_ids(src, dst)
	}
	
	#[text_signature = "($self, edge_id)"]
	/// Returns node IDs corresponding to given edge ID.
	/// 
	/// The method will panic if the given edge ID does not exists in the
	/// current graph instance.
	/// 
	/// Paramenters
	/// --------------
	/// edge_id : int,
	/// 	The edge ID whose source and destination node IDs are to e retrieved.
	///
	/// [Automatically generated binding]
	/// [Automatically generated documentation]
	fn get_unchecked_node_ids_from_edge_id(&self, edge_id : EdgeT) -> (NodeT, NodeT) {
		self.graph.get_unchecked_node_ids_from_edge_id(edge_id)
	}
	
	#[text_signature = "($self, edge_id)"]
	/// Returns node IDs corresponding to given edge ID.
	/// 
	/// Paramenters
	/// --------------
	/// edge_id : int,
	/// 	The edge ID whose source and destination node IDs are to e retrieved.
	///
	/// [Automatically generated binding]
	/// [Automatically generated documentation]
	fn get_node_ids_from_edge_id(&self, edge_id : EdgeT) -> PyResult<(NodeT, NodeT)> {
		pe!(self.graph.get_node_ids_from_edge_id(edge_id))
	}
	
	#[text_signature = "($self, src, dst)"]
	/// Returns edge ID corresponding to given source and destination node IDs.
	/// 
	/// The method will panic if the given source and destination node IDs do
	/// not correspond to an edge in this graph instance.
	/// 
	/// Paramenters
	/// --------------
	/// src : int,
	/// 	The source node ID.
	/// dst : int,
	/// 	The destination node ID.
	///
	/// [Automatically generated binding]
	/// [Automatically generated documentation]
	fn get_unchecked_edge_id_from_node_ids(&self, src : NodeT, dst : NodeT) -> EdgeT {
		self.graph.get_unchecked_edge_id_from_node_ids(src, dst)
	}
	
	#[text_signature = "($self, src, dst)"]
	/// Returns edge ID corresponding to given source and destination node IDs.
	/// 
	/// Paramenters
	/// --------------
	/// src : int,
	/// 	The source node ID.
	/// dst : int,
	/// 	The destination node ID.
	///
	/// [Automatically generated binding]
	/// [Automatically generated documentation]
	fn get_edge_id_from_node_ids(&self, src : NodeT, dst : NodeT) -> PyResult<EdgeT> {
		pe!(self.graph.get_edge_id_from_node_ids(src, dst))
	}
	
	#[text_signature = "($self, source_id)"]
	/// Returns edge ID corresponding to given source and destination node IDs.
	/// 
	/// Paramenters
	/// --------------
	/// source_id : int,
	/// 	The source node ID.
	///
	/// [Automatically generated binding]
	/// [Automatically generated documentation]
	fn get_unchecked_unique_source_node_id(&self, source_id : NodeT) -> NodeT {
		self.graph.get_unchecked_unique_source_node_id(source_id)
	}
	
	#[text_signature = "($self, edge_id)"]
	/// Return the src, dst, edge type of a given edge ID.
	/// 
	/// This method will raise a panic when an improper configuration is used.
	/// 
	/// Paramenters
	/// --------------
	/// edge_id : int,
	/// 	The edge ID whose source, destination and edge type are to be retrieved.
	///
	/// [Automatically generated binding]
	/// [Automatically generated documentation]
	fn get_unchecked_node_ids_and_edge_type_id_from_edge_id(&self, edge_id : EdgeT) -> (NodeT, NodeT, Option<EdgeTypeT>) {
		self.graph.get_unchecked_node_ids_and_edge_type_id_from_edge_id(edge_id)
	}
	
	#[text_signature = "($self, edge_id)"]
	/// Return the src, dst, edge type of a given edge ID.
	/// 
	/// Paramenters
	/// --------------
	/// edge_id : int,
	/// 	The edge ID whose source, destination and edge type are to be retrieved.
	///
	/// [Automatically generated binding]
	/// [Automatically generated documentation]
	fn get_node_ids_and_edge_type_id_from_edge_id(&self, edge_id : EdgeT) -> PyResult<(NodeT, NodeT, Option<EdgeTypeT>)> {
		pe!(self.graph.get_node_ids_and_edge_type_id_from_edge_id(edge_id))
	}
	
	#[text_signature = "($self, edge_id)"]
	/// Return the src, dst, edge type and weight of a given edge ID.
	/// 
	/// This method will raise a panic when an improper configuration is used.
	/// 
	/// Paramenters
	/// --------------
	/// edge_id : int,
	/// 	The edge ID whose source, destination, edge type and weight are to be retrieved.
	///
	/// [Automatically generated binding]
	/// [Automatically generated documentation]
	fn get_unchecked_node_ids_and_edge_type_id_and_edge_weight_from_edge_id(&self, edge_id : EdgeT) -> (NodeT, NodeT, Option<EdgeTypeT>, Option<WeightT>) {
		self.graph.get_unchecked_node_ids_and_edge_type_id_and_edge_weight_from_edge_id(edge_id)
	}
	
	#[text_signature = "($self, edge_id)"]
	/// Return the src, dst, edge type and weight of a given edge ID.
	/// 
	/// Paramenters
	/// --------------
	/// edge_id : int,
	/// 	The edge ID whose source, destination, edge type and weight are to be retrieved.
	///
	/// [Automatically generated binding]
	/// [Automatically generated documentation]
	fn get_node_ids_and_edge_type_id_and_edge_weight_from_edge_id(&self, edge_id : EdgeT) -> PyResult<(NodeT, NodeT, Option<EdgeTypeT>, Option<WeightT>)> {
		pe!(self.graph.get_node_ids_and_edge_type_id_and_edge_weight_from_edge_id(edge_id))
	}
	
	#[text_signature = "($self, k)"]
	/// Return vector with top k central node Ids.
	/// 
	/// If the k passed is bigger than the number of nodes this method will return
	/// all the nodes in the graph.
	/// 
	/// Paramenters
	/// --------------
	/// k : int,
	/// 	Number of central nodes to extract.
	/// TODO: This can be refactored to run faster!
	///
	/// [Automatically generated binding]
	/// [Automatically generated documentation]
	fn get_top_k_central_node_ids(&self, k : NodeT) -> Vec<NodeT> {
		self.graph.get_top_k_central_node_ids(k)
	}
	
	#[text_signature = "($self, node_id)"]
	/// Returns the number of outbound neighbours of given node.
	/// 
	/// The method will panic if the given node id is higher than the number of
	/// nodes in the graph.
	/// 
	/// Paramenters
	/// --------------
	/// node_id : int,
	/// 	Integer ID of the node.
	///
	/// [Automatically generated binding]
	/// [Automatically generated documentation]
	fn get_unchecked_node_degree_from_node_id(&self, node_id : NodeT) -> NodeT {
		self.graph.get_unchecked_node_degree_from_node_id(node_id)
	}
	
	#[text_signature = "($self, node_id)"]
	/// Returns the number of outbound neighbours of given node.
	/// 
	/// Paramenters
	/// --------------
	/// node_id : int,
	/// 	Integer ID of the node.
	///
	/// [Automatically generated binding]
	/// [Automatically generated documentation]
	fn get_node_degree_from_node_id(&self, node_id : NodeT) -> PyResult<NodeT> {
		pe!(self.graph.get_node_degree_from_node_id(node_id))
	}
	
	#[text_signature = "($self, k)"]
	/// Return vector with top k central node names.
	/// 
	/// Paramenters
	/// --------------
	/// k : int,
	/// 	Number of central nodes to extract.
	///
	/// [Automatically generated binding]
	/// [Automatically generated documentation]
	fn get_top_k_central_node_names(&self, k : NodeT) -> Vec<String> {
		self.graph.get_top_k_central_node_names(k)
	}
	
	#[text_signature = "($self, node_id)"]
	/// Returns option with vector of node types of given node.
	/// 
	/// This method will panic if the given node ID is greater than
	/// the number of nodes in the graph.
	/// Furthermore, if the graph does NOT have node types, it will NOT
	/// return neither an error or a panic.
	/// 
	/// Paramenters
	/// --------------
	/// node_id : int,
	/// 	node whose node type is to be returned.
	///
	/// [Automatically generated binding]
	/// [Automatically generated documentation]
	fn get_unchecked_node_type_id_from_node_id(&self, node_id : NodeT) -> Option<Vec<NodeTypeT>> {
		self.graph.get_unchecked_node_type_id_from_node_id(node_id)
	}
	
	#[text_signature = "($self, node_id)"]
	/// Returns node type of given node.
	/// 
	/// Paramenters
	/// --------------
	/// node_id : int,
	/// 	node whose node type is to be returned.
	///
	/// [Automatically generated binding]
	/// [Automatically generated documentation]
	fn get_node_type_id_from_node_id(&self, node_id : NodeT) -> PyResult<Option<Vec<NodeTypeT>>> {
		pe!(self.graph.get_node_type_id_from_node_id(node_id))
	}
	
	#[text_signature = "($self, edge_id)"]
	/// Returns edge type of given edge.
	/// 
	/// This method will panic if the given edge ID is greater than
	/// the number of edges in the graph.
	/// Furthermore, if the graph does NOT have edge types, it will NOT
	/// return neither an error or a panic.
	/// 
	/// Paramenters
	/// --------------
	/// edge_id : int,
	/// 	edge whose edge type is to be returned.
	///
	/// [Automatically generated binding]
	/// [Automatically generated documentation]
	fn get_unchecked_edge_type_id_from_edge_id(&self, edge_id : EdgeT) -> Option<EdgeTypeT> {
		self.graph.get_unchecked_edge_type_id_from_edge_id(edge_id)
	}
	
	#[text_signature = "($self, edge_id)"]
	/// Returns edge type of given edge.
	/// 
	/// Paramenters
	/// --------------
	/// edge_id : int,
	/// 	edge whose edge type is to be returned.
	///
	/// [Automatically generated binding]
	/// [Automatically generated documentation]
	fn get_edge_type_id_from_edge_id(&self, edge_id : EdgeT) -> PyResult<Option<EdgeTypeT>> {
		pe!(self.graph.get_edge_type_id_from_edge_id(edge_id))
	}
	
	#[text_signature = "($self, node_id)"]
	/// Returns result of option with the node type of the given node id.
	/// 
	/// Paramenters
	/// --------------
	/// node_id : int,
	/// 	The node ID whose node types are to be returned.
	///
	/// [Automatically generated binding]
	/// [Automatically generated documentation]
	fn get_node_type_name_from_node_id(&self, node_id : NodeT) -> PyResult<Option<Vec<String>>> {
		pe!(self.graph.get_node_type_name_from_node_id(node_id))
	}
	
	#[text_signature = "($self, edge_id)"]
	/// Returns option with the edge type of the given edge id.
	/// 
	/// Paramenters
	/// --------------
	/// edge_id : int,
	/// 	The edge ID whose edge type is to be returned.
	///
	/// [Automatically generated binding]
	/// [Automatically generated documentation]
	fn get_edge_type_name_from_edge_id(&self, edge_id : EdgeT) -> PyResult<Option<String>> {
		pe!(self.graph.get_edge_type_name_from_edge_id(edge_id))
	}
	
	#[text_signature = "($self, edge_type_id)"]
	/// Return edge type name of given edge type.
	/// 
	/// Paramenters
	/// --------------
	/// edge_type_id : int,
	/// 	Id of the edge type.
	///
	/// [Automatically generated binding]
	/// [Automatically generated documentation]
	fn get_edge_type_name_from_edge_type_id(&self, edge_type_id : EdgeTypeT) -> PyResult<String> {
		pe!(self.graph.get_edge_type_name_from_edge_type_id(edge_type_id))
	}
	
	#[text_signature = "($self, edge_id)"]
	/// Returns weight of the given edge id.
	/// 
	/// Paramenters
	/// --------------
	/// edge_id : int,
	/// 	The edge ID whose weight is to be returned.
	///
	/// [Automatically generated binding]
	/// [Automatically generated documentation]
	fn get_edge_weight_from_edge_id(&self, edge_id : EdgeT) -> PyResult<WeightT> {
		pe!(self.graph.get_edge_weight_from_edge_id(edge_id))
	}
	
	#[text_signature = "($self, src, dst)"]
	/// Returns weight of the given node ids.
	/// 
	/// Paramenters
	/// --------------
	/// src : int,
	/// 	The node ID of the source node.
	/// dst : int,
	/// 	The node ID of the destination node.
	///
	/// [Automatically generated binding]
	/// [Automatically generated documentation]
	fn get_edge_weight_from_node_ids(&self, src : NodeT, dst : NodeT) -> PyResult<WeightT> {
		pe!(self.graph.get_edge_weight_from_node_ids(src, dst))
	}
	
	#[text_signature = "($self, src, dst, edge_type)"]
	/// Returns weight of the given node ids and edge type.
	/// 
	/// Paramenters
	/// --------------
	/// src : int,
	/// 	The node ID of the source node.
	/// dst : int,
	/// 	The node ID of the destination node.
	/// edge_type : int,
	/// 	The edge type ID of the edge.
	///
	/// [Automatically generated binding]
	/// [Automatically generated documentation]
	fn get_edge_weight_from_node_ids_and_edge_type_id(&self, src : NodeT, dst : NodeT, edge_type : Option<EdgeTypeT>) -> PyResult<WeightT> {
		pe!(self.graph.get_edge_weight_from_node_ids_and_edge_type_id(src, dst, edge_type))
	}
	
	#[text_signature = "($self, src, dst, edge_type)"]
	/// Returns weight of the given node names and edge type.
	/// 
	/// Paramenters
	/// --------------
	/// src : str,
	/// 	The node name of the source node.
	/// dst : str,
	/// 	The node name of the destination node.
	/// edge_type : str,
	/// 	The edge type name of the edge.
	///
	/// [Automatically generated binding]
	/// [Automatically generated documentation]
	fn get_edge_weight_from_node_names_and_edge_type_name(&self, src : &str, dst : &str, edge_type : Option<&String>) -> PyResult<WeightT> {
		pe!(self.graph.get_edge_weight_from_node_names_and_edge_type_name(src, dst, edge_type))
	}
	
	#[text_signature = "($self, src_name, dst_name)"]
	/// Returns weight of the given node names.
	/// 
	/// Paramenters
	/// --------------
	/// src_name : str,
	/// 	The node name of the source node.
	/// dst_name : str,
	/// 	The node name of the destination node.
	///
	/// [Automatically generated binding]
	/// [Automatically generated documentation]
	fn get_edge_weight_from_node_names(&self, src_name : &str, dst_name : &str) -> PyResult<WeightT> {
		pe!(self.graph.get_edge_weight_from_node_names(src_name, dst_name))
	}
	
	#[text_signature = "($self, node_id)"]
	/// Returns result with the node name.
	/// 
	/// Paramenters
	/// --------------
	/// node_id : int,
	/// 	The node ID whose name is to be returned.
	///
	/// [Automatically generated binding]
	/// [Automatically generated documentation]
	fn get_unchecked_node_name_from_node_id(&self, node_id : NodeT) -> String {
		self.graph.get_unchecked_node_name_from_node_id(node_id)
	}
	
	#[text_signature = "($self, node_id)"]
	/// Returns result with the node name.
	/// 
	/// Paramenters
	/// --------------
	/// node_id : int,
	/// 	The node ID whose name is to be returned.
	///
	/// [Automatically generated binding]
	/// [Automatically generated documentation]
	fn get_node_name_from_node_id(&self, node_id : NodeT) -> PyResult<String> {
		pe!(self.graph.get_node_name_from_node_id(node_id))
	}
	
	#[text_signature = "($self, node_name)"]
	/// Returns result with the node id.
	/// 
	/// Paramenters
	/// --------------
	/// node_name : str,
	/// 	The node name whose node ID is to be returned.
	///
	/// [Automatically generated binding]
	/// [Automatically generated documentation]
	fn get_node_id_from_node_name(&self, node_name : &str) -> PyResult<NodeT> {
		pe!(self.graph.get_node_id_from_node_name(node_name))
	}
	
	#[text_signature = "($self, node_name)"]
	/// Return node type ID for the given node name if available.
	/// 
	/// Paramenters
	/// --------------
	/// node_name : str,
	/// 	Name of the node.
	///
	/// [Automatically generated binding]
	/// [Automatically generated documentation]
	fn get_node_type_id_from_node_name(&self, node_name : &str) -> PyResult<Option<Vec<NodeTypeT>>> {
		pe!(self.graph.get_node_type_id_from_node_name(node_name))
	}
	
	#[text_signature = "($self, node_name)"]
	/// Return node type name for the given node name if available.
	/// 
	/// Paramenters
	/// --------------
	/// node_name : str,
	/// 	Name of the node.
	///
	/// [Automatically generated binding]
	/// [Automatically generated documentation]
	fn get_node_type_name_from_node_name(&self, node_name : &str) -> PyResult<Option<Vec<String>>> {
		pe!(self.graph.get_node_type_name_from_node_name(node_name))
	}
	
	#[text_signature = "($self, edge_type_id)"]
	/// Return number of edges with given edge type ID.
	/// 
	/// If None is given as an edge type ID, the unknown edge type IDs
	/// will be returned.
	/// 
	/// Paramenters
	/// --------------
	/// edge_type_id : int,
	/// 	The edge type ID to count the edges of.
	///
	/// [Automatically generated binding]
	/// [Automatically generated documentation]
	fn get_edge_count_from_edge_type_id(&self, edge_type_id : Option<EdgeTypeT>) -> PyResult<EdgeT> {
		pe!(self.graph.get_edge_count_from_edge_type_id(edge_type_id))
	}
	
	#[text_signature = "($self, edge_type_name)"]
	/// Return edge type ID curresponding to given edge type name.
	/// 
	/// If None is given as an edge type ID, None is returned.
	/// 
	/// Paramenters
	/// --------------
	/// edge_type_name : str,
	/// 	The edge type name whose ID is to be returned.
	///
	/// [Automatically generated binding]
	/// [Automatically generated documentation]
	fn get_edge_type_id_from_edge_type_name(&self, edge_type_name : Option<&str>) -> PyResult<Option<EdgeTypeT>> {
		pe!(self.graph.get_edge_type_id_from_edge_type_name(edge_type_name))
	}
	
	#[text_signature = "($self, edge_type_name)"]
	/// Return number of edges with given edge type name.
	/// 
	/// If None is given as an edge type name, the unknown edge types
	/// will be returned.
	/// 
	/// Paramenters
	/// --------------
	/// edge_type_name : str,
	/// 	The edge type name to count the edges of.
	///
	/// [Automatically generated binding]
	/// [Automatically generated documentation]
	fn get_edge_count_from_edge_type_name(&self, edge_type_name : Option<&str>) -> PyResult<EdgeT> {
		pe!(self.graph.get_edge_count_from_edge_type_name(edge_type_name))
	}
	
	#[text_signature = "($self, node_type_name)"]
	/// Return node type ID curresponding to given node type name.
	/// 
	/// If None is given as an node type ID, None is returned.
	/// 
	/// Paramenters
	/// --------------
	/// node_type_name : str,
	/// 	The node type name whose ID is to be returned.
	///
	/// [Automatically generated binding]
	/// [Automatically generated documentation]
	fn get_node_type_id_from_node_type_name(&self, node_type_name : &str) -> PyResult<NodeTypeT> {
		pe!(self.graph.get_node_type_id_from_node_type_name(node_type_name))
	}
	
	#[text_signature = "($self, node_type_id)"]
	/// Return number of nodes with given node type ID.
	/// 
	/// If None is given as an node type ID, the unknown node types
	/// will be returned.
	/// 
	/// Paramenters
	/// --------------
	/// node_type_id : int,
	/// 	The node type ID to count the nodes of.
	///
	/// [Automatically generated binding]
	/// [Automatically generated documentation]
	fn get_node_count_from_node_type_id(&self, node_type_id : Option<NodeTypeT>) -> PyResult<NodeT> {
		pe!(self.graph.get_node_count_from_node_type_id(node_type_id))
	}
	
	#[text_signature = "($self, node_type_name)"]
	/// Return number of nodes with given node type name.
	/// 
	/// If None is given as an node type name, the unknown node types
	/// will be returned.
	/// 
	/// Paramenters
	/// --------------
	/// node_type_name : str,
	/// 	The node type name to count the nodes of.
	///
	/// [Automatically generated binding]
	/// [Automatically generated documentation]
	fn get_node_count_from_node_type_name(&self, node_type_name : Option<&str>) -> PyResult<NodeT> {
		pe!(self.graph.get_node_count_from_node_type_name(node_type_name))
	}
	
	#[text_signature = "($self, edge_id)"]
	/// Returns the destination of given edge id.
	/// 
	/// Paramenters
	/// --------------
	/// edge_id : int,
	/// 	The edge ID whose destination is to be retrieved.
	///
	/// [Automatically generated binding]
	/// [Automatically generated documentation]
	fn get_destination_node_id_from_edge_id(&self, edge_id : EdgeT) -> PyResult<NodeT> {
		pe!(self.graph.get_destination_node_id_from_edge_id(edge_id))
	}
	
	#[text_signature = "($self, node_id)"]
	/// Return vector of destinations for the given source node ID.
	/// 
	/// Paramenters
	/// --------------
	/// node_id : int,
	/// 	Node ID whose neighbours are to be retrieved.
	///
	/// [Automatically generated binding]
	/// [Automatically generated documentation]
	fn get_neighbour_node_ids_from_node_id(&self, node_id : NodeT) -> PyResult<Vec<NodeT>> {
		pe!(self.graph.get_neighbour_node_ids_from_node_id(node_id))
	}
	
	#[text_signature = "($self, node_name)"]
	/// Return vector of destinations for the given source node name.
	/// 
	/// Paramenters
	/// --------------
	/// node_name : str,
	/// 	Node ID whose neighbours are to be retrieved.
	///
	/// [Automatically generated binding]
	/// [Automatically generated documentation]
	fn get_neighbour_node_ids_from_node_name(&self, node_name : &str) -> PyResult<Vec<NodeT>> {
		pe!(self.graph.get_neighbour_node_ids_from_node_name(node_name))
	}
	
	#[text_signature = "($self, node_name)"]
	/// Return vector of destination names for the given source node name.
	/// 
	/// Paramenters
	/// --------------
	/// node_name : str,
	/// 	Node name whose neighbours are to be retrieved.
	///
	/// [Automatically generated binding]
	/// [Automatically generated documentation]
	fn get_neighbour_node_names_from_node_name(&self, node_name : &str) -> PyResult<Vec<String>> {
		pe!(self.graph.get_neighbour_node_names_from_node_name(node_name))
	}
	
	#[text_signature = "($self, src, dst)"]
	/// Return range of outbound edges IDs for all the edges bewteen the given
	/// source and destination nodes.
	/// This operation is meaningfull only in a multigraph.
	/// 
	/// Paramenters
	/// --------------
	/// src : int,
	/// 	Source node.
	/// dst : int,
	/// 	Destination node.
	///
	/// [Automatically generated binding]
	/// [Automatically generated documentation]
	fn get_minmax_edge_ids_from_node_ids(&self, src : NodeT, dst : NodeT) -> PyResult<(EdgeT, EdgeT)> {
		pe!(self.graph.get_minmax_edge_ids_from_node_ids(src, dst))
	}
	
	#[text_signature = "($self, src, dst, edge_type)"]
	/// Return edge ID for given tuple of nodes and edge type.
	/// 
	/// This method will return an error if the graph does not contain the
	/// requested edge with edge type.
	/// 
	/// Paramenters
	/// --------------
	/// src : int,
	/// 	Source node of the edge.
	/// dst : int,
	/// 	Destination node of the edge.
	/// edge_type : int,
	/// 	Edge Type of the edge.
	///
	/// [Automatically generated binding]
	/// [Automatically generated documentation]
	fn get_edge_id_from_node_ids_and_edge_type_id(&self, src : NodeT, dst : NodeT, edge_type : Option<EdgeTypeT>) -> PyResult<EdgeT> {
		pe!(self.graph.get_edge_id_from_node_ids_and_edge_type_id(src, dst, edge_type))
	}
	
	#[text_signature = "($self, src_name, dst_name)"]
	/// Return edge ID for given tuple of node names.
	/// 
	/// This method will return an error if the graph does not contain the
	/// requested edge with edge type.
	/// 
	/// Paramenters
	/// --------------
	/// src_name : str,
	/// 	Source node name of the edge.
	/// dst_name : str,
	/// 	Destination node name of the edge.
	///
	/// [Automatically generated binding]
	/// [Automatically generated documentation]
	fn get_edge_id_from_node_names(&self, src_name : &str, dst_name : &str) -> PyResult<EdgeT> {
		pe!(self.graph.get_edge_id_from_node_names(src_name, dst_name))
	}
	
	#[text_signature = "($self, src_name, dst_name, edge_type_name)"]
	/// Return edge ID for given tuple of node names and edge type name.
	/// 
	/// This method will return an error if the graph does not contain the
	/// requested edge with edge type.
	/// 
	/// Paramenters
	/// --------------
	/// src_name : str,
	/// 	Source node name of the edge.
	/// dst_name : str,
	/// 	Destination node name of the edge.
	/// edge_type_name : str,
	/// 	Edge type name.
	///
	/// [Automatically generated binding]
	/// [Automatically generated documentation]
	fn get_edge_id_from_node_names_and_edge_type_name(&self, src_name : &str, dst_name : &str, edge_type_name : Option<&String>) -> PyResult<EdgeT> {
		pe!(self.graph.get_edge_id_from_node_names_and_edge_type_name(src_name, dst_name, edge_type_name))
	}
	
	#[text_signature = "($self, edge_type_names)"]
	/// Return translated edge types from string to internal edge ID.
	/// 
	/// Paramenters
	/// --------------
	/// edge_type_names : List[str],
	/// 	Vector of edge types to be converted.
	///
	/// [Automatically generated binding]
	/// [Automatically generated documentation]
	fn get_edge_type_ids_from_edge_type_names(&self, edge_type_names : Vec<Option<String>>) -> PyResult<Vec<Option<EdgeTypeT>>> {
		pe!(self.graph.get_edge_type_ids_from_edge_type_names(edge_type_names))
	}
	
	#[text_signature = "($self, node_type_names)"]
	/// Return translated node types from string to internal node ID.
	/// 
	/// Paramenters
	/// --------------
	/// node_type_names : List[str],
	/// 	Vector of node types to be converted.
	///
	/// [Automatically generated binding]
	/// [Automatically generated documentation]
	fn get_node_type_ids_from_node_type_names(&self, node_type_names : Vec<Option<String>>) -> PyResult<Vec<Option<NodeTypeT>>> {
		pe!(self.graph.get_node_type_ids_from_node_type_names(node_type_names))
	}
	
	#[text_signature = "($self, src)"]
	/// Return range of outbound edges IDs which have as source the given Node.
	/// 
	/// The method will panic if the given source node ID is higher than
	/// the number of nodes in the graph.
	/// 
	/// Paramenters
	/// --------------
	/// src : int,
	/// 	Node for which we need to compute the cumulative_node_degrees range.
	///
	/// [Automatically generated binding]
	/// [Automatically generated documentation]
	fn get_unchecked_minmax_edge_ids_from_source_node_id(&self, src : NodeT) -> (EdgeT, EdgeT) {
		self.graph.get_unchecked_minmax_edge_ids_from_source_node_id(src)
	}
	
	#[text_signature = "($self, src)"]
	/// Return range of outbound edges IDs which have as source the given Node.
	/// 
	/// Paramenters
	/// --------------
	/// src : int,
	/// 	Node for which we need to compute the cumulative_node_degrees range.
	///
	/// [Automatically generated binding]
	/// [Automatically generated documentation]
	fn get_minmax_edge_ids_from_source_node_id(&self, src : NodeT) -> PyResult<(EdgeT, EdgeT)> {
		pe!(self.graph.get_minmax_edge_ids_from_source_node_id(src))
	}
	
	#[text_signature = "($self, node_type_id)"]
	/// Return node type name of given node type.
	/// 
	/// There is no need for a unchecked version since we will have to map
	/// on the note_types anyway.
	/// 
	/// Paramenters
	/// --------------
	/// node_type_id : int,
	/// 	Id of the node type.
	///
	/// [Automatically generated binding]
	/// [Automatically generated documentation]
	fn get_node_type_name_from_node_type_id(&self, node_type_id : NodeTypeT) -> PyResult<String> {
		pe!(self.graph.get_node_type_name_from_node_type_id(node_type_id))
	}
	
	#[text_signature = "($self, node_type_ids)"]
	/// Return node type name of given node type.
	/// 
	/// Paramenters
	/// --------------
	/// node_type_ids : List[int],
	/// 	Id of the node type.
	///
	/// [Automatically generated binding]
	/// [Automatically generated documentation]
	fn get_node_type_names_from_node_type_ids(&self, node_type_ids : Vec<NodeTypeT>) -> PyResult<Vec<String>> {
		pe!(self.graph.get_node_type_names_from_node_type_ids(node_type_ids))
	}
	
}
