use super::*;
impl Graph {

	#[text_signature = "($self, edge_type_ids)"]
	/// Validates provided edge type IDs.
	/// 
	/// Parameters
	/// --------------
	/// edge_type_ids: List[Union[int], None],
	/// 	Vector of edge type IDs to validate.
	/// 
	///  # Raises
	///  * If there are no edge types in the graph.
	///
	/// [Automatically generated binding]
	/// [Automatically generated documentation]
	fn validate_edge_type_ids(&self, edge_type_ids : Vec<Option<EdgeTypeT>>) -> PyResult<Vec<Option<EdgeTypeT>>> {
		pe!(self.graph.validate_edge_type_ids(edge_type_ids))
	}
	
	#[text_signature = "($self, node_type_ids)"]
	/// Validates provided node type IDs.
	/// 
	/// Parameters
	/// --------------
	/// node_type_ids: List[Union[int], None],
	/// 	Vector of node type IDs to validate.
	/// 
	///  # Raises
	///  * If there are no node types in the graph.
	///
	/// [Automatically generated binding]
	/// [Automatically generated documentation]
	fn validate_node_type_ids(&self, node_type_ids : Vec<Option<NodeTypeT>>) -> PyResult<Vec<Option<NodeTypeT>>> {
		pe!(self.graph.validate_node_type_ids(node_type_ids))
	}
	
}
