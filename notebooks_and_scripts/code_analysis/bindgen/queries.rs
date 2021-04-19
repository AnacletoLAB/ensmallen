use super::*;
impl Graph {

	#[text_signature = "($self, edge_node_names)"]
	/// Returns result with the edge node IDs.
	/// 
	/// Parameters
	/// --------------
	/// edge_node_name: List[(str, str)],
	/// 	The node names whose node IDs is to be returned.
	/// 
	///  # Raises
	///  * When any of the given node name does not exists in the current graph.
	///
	/// [Automatically generated binding]
	/// [Automatically generated documentation]
	fn get_edge_node_ids_from_edge_node_names(&self, edge_node_names : Vec<(&str, &str)>) -> PyResult<Vec<(NodeT, NodeT)>> {
		pe!(self.graph.get_edge_node_ids_from_edge_node_names(edge_node_names))
	}
	
	#[text_signature = "($self, edge_node_ids)"]
	/// Returns result with the edge node names.
	/// 
	/// Parameters
	/// --------------
	/// edge_node_ids: List[(int, int)],
	/// 	The node names whose node names is to be returned.
	/// 
	///  # Raises
	///  * When any of the given node IDs does not exists in the current graph.
	///
	/// [Automatically generated binding]
	/// [Automatically generated documentation]
	fn get_edge_node_names_from_edge_node_ids(&self, edge_node_ids : Vec<(NodeT, NodeT)>) -> PyResult<Vec<(String, String)>> {
		pe!(self.graph.get_edge_node_names_from_edge_node_ids(edge_node_ids))
	}
	
	#[text_signature = "($self, node_type_names)"]
	/// Return translated node types from string to internal node ID.
	/// 
	/// Parameters
	/// --------------
	/// node_type_names: List[Union[Vec<str], None]>,
	/// 	Vector of node types to be converted.
	/// 
	///  # Raises
	///  * If the graph does not have node types.
	///  * If any of the given node type names do not exists in the graph.
	///
	/// [Automatically generated binding]
	/// [Automatically generated documentation]
	fn get_multiple_node_type_ids_from_node_type_names(&self, node_type_names : Vec<Option<Vec<&str>>>) -> PyResult<Vec<Option<Vec<NodeTypeT>>>> {
		pe!(self.graph.get_multiple_node_type_ids_from_node_type_names(node_type_names))
	}
	
	#[text_signature = "($self, node_names)"]
	/// Returns result with the node IDs.
	/// 
	/// Parameters
	/// --------------
	/// node_name: List[str],
	/// 	The node names whose node IDs is to be returned.
	/// 
	///  # Raises
	///  * When any of the given node name does not exists in the current graph.
	///
	/// [Automatically generated binding]
	/// [Automatically generated documentation]
	fn get_node_ids_from_node_names(&self, node_names : Vec<&str>) -> PyResult<Vec<NodeT>> {
		pe!(self.graph.get_node_ids_from_node_names(node_names))
	}
	
}
