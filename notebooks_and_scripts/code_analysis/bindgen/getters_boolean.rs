use super::*;
impl Graph {

	#[text_signature = "($self)"]
	/// Return whether the graph has any known edge type-related graph oddities.
	/// 
	///  # Raises
	///  * If the graph does not have edge types.
	///
	/// [Automatically generated binding]
	/// [Automatically generated documentation]
	fn has_edge_types_oddities(&self) -> PyResult<bool> {
		pe!(self.graph.has_edge_types_oddities())
	}
	
	#[text_signature = "($self)"]
	/// Returns whether the edges have an homogenous edge type.
	/// 
	///  # Raises
	///  * If the graph does not have edge types.
	///
	/// [Automatically generated binding]
	/// [Automatically generated documentation]
	fn has_homogeneous_edge_types(&self) -> PyResult<bool> {
		pe!(self.graph.has_homogeneous_edge_types())
	}
	
	#[text_signature = "($self)"]
	/// Returns whether the nodes have an homogenous node type.
	/// 
	///  # Raises
	///  * If the graph does not have node types.
	///
	/// [Automatically generated binding]
	/// [Automatically generated documentation]
	fn has_homogeneous_node_types(&self) -> PyResult<bool> {
		pe!(self.graph.has_homogeneous_node_types())
	}
	
	#[text_signature = "($self)"]
	/// Return whether the graph has any known node-related graph oddities.
	///
	/// [Automatically generated binding]
	/// [Automatically generated documentation]
	fn has_node_oddities(&self) -> bool {
		self.graph.has_node_oddities()
	}
	
	#[text_signature = "($self)"]
	/// Return whether the graph has any known node type-related graph oddities.
	/// 
	///  # Raises
	///  * If the graph does not have node types.
	///
	/// [Automatically generated binding]
	/// [Automatically generated documentation]
	fn has_node_types_oddities(&self) -> PyResult<bool> {
		pe!(self.graph.has_node_types_oddities())
	}
	
	#[text_signature = "($self)"]
	/// Returns whether there is at least singleton edge type, that is a edge type that only appears once.
	/// 
	///  # Raises
	///  * If the graph does not have edge types.
	///
	/// [Automatically generated binding]
	/// [Automatically generated documentation]
	fn has_singleton_edge_types(&self) -> PyResult<bool> {
		pe!(self.graph.has_singleton_edge_types())
	}
	
	#[text_signature = "($self)"]
	/// Returns whether there is at least singleton node type, that is a node type that only appears once.
	/// 
	///  # Raises
	///  * If the graph does not have node types.
	///
	/// [Automatically generated binding]
	/// [Automatically generated documentation]
	fn has_singleton_node_types(&self) -> PyResult<bool> {
		pe!(self.graph.has_singleton_node_types())
	}
	
	#[text_signature = "($self)"]
	/// Returns boolean representing if graph has singletons.
	///
	/// [Automatically generated binding]
	/// [Automatically generated documentation]
	fn has_singleton_nodes(&self) -> bool {
		self.graph.has_singleton_nodes()
	}
	
	#[text_signature = "($self)"]
	/// Returns boolean representing if graph has singletons.
	///
	/// [Automatically generated binding]
	/// [Automatically generated documentation]
	fn has_singleton_nodes_with_selfloops(&self) -> bool {
		self.graph.has_singleton_nodes_with_selfloops()
	}
	
}
