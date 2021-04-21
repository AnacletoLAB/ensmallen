use super::*;
impl Graph {

	#[text_signature = "($self)"]
	/// Remove singleton edge types from all edges.
	/// 
	///  If any given edge remains with no edge type, that edge is labeled
	///  with edge type None. Note that the modification happens inplace.
	/// 
	///  # Raises
	///  * If the graph does not have edge types.
	///
	/// [Automatically generated binding]
	/// [Automatically generated documentation]
	fn remove_inplace_singleton_edge_types(&mut self) -> PyResult<&mut EnsmallenGraph> {
		Ok(EnsmallenGraph{graph:pe!(self.graph.remove_inplace_singleton_edge_types())})
	}
	
	#[text_signature = "($self)"]
	/// Remove singleton node types from all nodes.
	/// 
	///  If any given node remains with no node type, that node is labeled
	///  with node type None. Note that the modification happens inplace.
	/// 
	///  # Raises
	///  * If the graph does not have node types.
	///
	/// [Automatically generated binding]
	/// [Automatically generated documentation]
	fn remove_inplace_singleton_node_types(&mut self) -> PyResult<&mut EnsmallenGraph> {
		Ok(EnsmallenGraph{graph:pe!(self.graph.remove_inplace_singleton_node_types())})
	}
	
	#[text_signature = "($self)"]
	/// Remove singleton edge types from all edges.
	/// 
	///  If any given edge remains with no edge type, that edge is labeled
	///  with edge type None. Note that the modification DOES NOT happen inplace.
	/// 
	///  # Raises
	///  * If the graph does not have edge types.
	///
	/// [Automatically generated binding]
	/// [Automatically generated documentation]
	fn remove_singleton_edge_types(&mut self) -> PyResult<EnsmallenGraph> {
		Ok(EnsmallenGraph{graph:pe!(self.graph.remove_singleton_edge_types())})
	}
	
	#[text_signature = "($self)"]
	/// Remove singleton node types from all nodes.
	/// 
	///  If any given node remains with no node type, that node is labeled
	///  with node type None. Note that the modification DOES NOT happen inplace.
	/// 
	///  # Raises
	///  * If the graph does not have node types.
	///
	/// [Automatically generated binding]
	/// [Automatically generated documentation]
	fn remove_singleton_node_types(&mut self) -> PyResult<EnsmallenGraph> {
		Ok(EnsmallenGraph{graph:pe!(self.graph.remove_singleton_node_types())})
	}
	
}
