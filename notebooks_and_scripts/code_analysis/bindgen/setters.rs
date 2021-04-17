use super::*;
impl Graph {

	#[text_signature = "($self, name)"]
	/// Set the name of the graph.
	/// 
	/// Paramenters
	/// --------------
	/// name : str,
	/// 	Name of the graph.
	///
	/// [Automatically generated binding]
	/// [Automatically generated documentation]
	fn set_name(&mut self, name : String){
		self.graph.set_name(name)
	}
	
	#[text_signature = "($self, edge_type)"]
	/// Replace all edge types (if present) and set all the edge to edge_type.
	/// 
	/// Paramenters
	/// --------------
	/// edge_type : S,
	/// 	The edge type to assing to all the edges.
	///
	/// [Automatically generated binding]
	/// [Automatically generated documentation]
	fn set_all_edge_types(mut self, edge_type : S) -> PyResult<Graph> {
		pe!(self.graph.set_all_edge_types(edge_type))
	}
	
	#[text_signature = "($self, node_type)"]
	/// Replace all node types (if present) and set all the node to node_type.
	/// 
	/// Paramenters
	/// --------------
	/// node_type : S,
	/// 	The node type to assing to all the nodes.
	///
	/// [Automatically generated binding]
	/// [Automatically generated documentation]
	fn set_all_node_types(mut self, node_type : S) -> PyResult<Graph> {
		pe!(self.graph.set_all_node_types(node_type))
	}
	
}
