use super::*;
impl Graph {

	#[text_signature = "($self, edge_type_name)"]
	/// Remove given edge type name from all edges.
	/// 
	///  If any given edge remains with no edge type, that edge is labeled
	///  with edge type None. Note that the modification happens inplace.
	/// 
	/// Parameters
	/// --------------
	/// edge_type_name: str,
	/// 	The edge type ID to remove.
	/// 
	///  # Raises
	///  * If the graph does not have edge types.
	///  * If the given edge type name does not exists in the graph.
	///
	/// [Automatically generated binding]
	/// [Automatically generated documentation]
	fn remove_inplace_edge_type_name(&mut self, edge_type_name : &str) -> PyResult<&mut EnsmallenGraph> {
		Ok(EnsmallenGraph{graph:pe!(self.graph.remove_inplace_edge_type_name(edge_type_name))})
	}
	
	#[text_signature = "($self)"]
	/// Remove edge types from the graph.
	/// 
	///  Note that the modification happens inplace.
	/// 
	///  # Raises
	///  * If the graph does not have edge types.
	///  * If the graph is a multigraph.
	///
	/// [Automatically generated binding]
	/// [Automatically generated documentation]
	fn remove_inplace_edge_types(&mut self) -> PyResult<&EnsmallenGraph> {
		Ok(EnsmallenGraph{graph:pe!(self.graph.remove_inplace_edge_types())})
	}
	
	#[text_signature = "($self)"]
	/// Remove edge weights from the graph.
	/// 
	///  Note that the modification happens inplace.
	/// 
	///  # Raises
	///  * If the graph does not have edge weights.
	///
	/// [Automatically generated binding]
	/// [Automatically generated documentation]
	fn remove_inplace_edge_weights(&mut self) -> PyResult<&EnsmallenGraph> {
		Ok(EnsmallenGraph{graph:pe!(self.graph.remove_inplace_edge_weights())})
	}
	
	#[text_signature = "($self, node_type_id)"]
	/// Remove given node type ID from all nodes.
	/// 
	///  If any given node remains with no node type, that node is labeled
	///  with node type None. Note that the modification happens inplace.
	/// 
	/// Parameters
	/// --------------
	/// node_type_id: int,
	/// 	The node type ID to remove.
	/// 
	///  # Raises
	///  * If the graph does not have node types.
	///  * If the given node type ID does not exists in the graph.
	///
	/// [Automatically generated binding]
	/// [Automatically generated documentation]
	fn remove_inplace_node_type_id(&mut self, node_type_id : NodeTypeT) -> PyResult<&EnsmallenGraph> {
		Ok(EnsmallenGraph{graph:pe!(self.graph.remove_inplace_node_type_id(node_type_id))})
	}
	
	#[text_signature = "($self, node_type_name)"]
	/// Remove given node type name from all nodes.
	/// 
	///  If any given node remains with no node type, that node is labeled
	///  with node type None. Note that the modification happens inplace.
	/// 
	/// Parameters
	/// --------------
	/// node_type_name: str,
	/// 	The node type ID to remove.
	/// 
	///  # Raises
	///  * If the graph does not have node types.
	///  * If the given node type name does not exists in the graph.
	///
	/// [Automatically generated binding]
	/// [Automatically generated documentation]
	fn remove_inplace_node_type_name(&mut self, node_type_name : &str) -> PyResult<&EnsmallenGraph> {
		Ok(EnsmallenGraph{graph:pe!(self.graph.remove_inplace_node_type_name(node_type_name))})
	}
	
	#[text_signature = "($self)"]
	/// Remove node types from the graph.
	/// 
	///  Note that the modification happens inplace.
	/// 
	///  # Raises
	///  * If the graph does not have node types.
	///
	/// [Automatically generated binding]
	/// [Automatically generated documentation]
	fn remove_inplace_node_types(&mut self) -> PyResult<&EnsmallenGraph> {
		Ok(EnsmallenGraph{graph:pe!(self.graph.remove_inplace_node_types())})
	}
	
	#[text_signature = "($self, edge_type)"]
	/// Replace all edge types (if present) and set all the edge to edge_type.
	/// 
	///  This happens INPLACE, that is edits the current graph instance.
	/// 
	/// Parameters
	/// --------------
	/// edge_type: S,
	/// 	The edge type to assing to all the edges.
	///
	/// [Automatically generated binding]
	/// [Automatically generated documentation]
	fn set_inplace_all_edge_types(&mut self, edge_type : S) -> PyResult<&EnsmallenGraph> {
		Ok(EnsmallenGraph{graph:pe!(self.graph.set_inplace_all_edge_types(edge_type))})
	}
	
	#[text_signature = "($self, node_type)"]
	/// Replace all node types (if present) and set all the node to node_type.
	/// 
	/// Parameters
	/// --------------
	/// node_type: S,
	/// 	The node type to assing to all the nodes.
	///
	/// [Automatically generated binding]
	/// [Automatically generated documentation]
	fn set_inplace_all_node_types(&mut self, node_type : S) -> PyResult<&EnsmallenGraph> {
		Ok(EnsmallenGraph{graph:pe!(self.graph.set_inplace_all_node_types(node_type))})
	}
	
}
