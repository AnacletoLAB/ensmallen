use super::*;
impl Graph {

	#[text_signature = "($self, node_name)"]
	/// Returns the number of outbound neighbours of given node name.
	/// 
	/// Parameters
	/// --------------
	/// node_name: str,
	/// 	Integer ID of the node.
	/// 
	///  # Raises
	///  * If the given node name does not exist in the graph.
	///
	/// [Automatically generated binding]
	/// [Automatically generated documentation]
	fn get_node_degree_from_node_name(&self, node_name : &str) -> PyResult<NodeT> {
		pe!(self.graph.get_node_degree_from_node_name(node_name))
	}
	
	#[text_signature = "($self, node_id)"]
	/// Returns result of option with the node type of the given node id.
	/// 
	/// Parameters
	/// --------------
	/// node_id: int,
	/// 	The node ID whose node types are to be returned.
	///
	/// [Automatically generated binding]
	/// [Automatically generated documentation]
	fn get_node_type_names_from_node_id(&self, node_id : NodeT) -> PyResult<Option<Vec<String>>> {
		pe!(self.graph.get_node_type_names_from_node_id(node_id))
	}
	
}
