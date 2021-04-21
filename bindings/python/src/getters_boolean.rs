use super::*;

#[pymethods]
impl EnsmallenGraph {
	#[text_signature = "($self)"]
	/// Returns boolean representing whether graph has weights.
	///
	/// [Automatically generated binding]
	/// [Automatically generated documentation]
	fn has_edge_weights(&self) -> bool {
		self.graph.has_edge_weights()
	}
	
	#[text_signature = "($self)"]
	/// Return if the graph has any edges.
	///
	/// [Automatically generated binding]
	/// [Automatically generated documentation]
	fn has_edges(&self) -> bool {
		self.graph.has_edges()
	}
	
	#[text_signature = "($self)"]
	/// Returns boolean representing if graph has multilabel node types.
	///
	/// [Automatically generated binding]
	/// [Automatically generated documentation]
	fn has_multilabel_node_types(&self) -> bool {
		self.graph.has_multilabel_node_types()
	}
	
	#[text_signature = "($self)"]
	/// Return if the graph has any nodes.
	///
	/// [Automatically generated binding]
	/// [Automatically generated documentation]
	fn has_nodes(&self) -> bool {
		self.graph.has_nodes()
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
	
	#[text_signature = "($self)"]
	/// 
	///
	/// [Automatically generated binding]
	/// [Automatically generated documentation]
	fn has_trap_nodes(&self) -> bool {
		self.graph.has_trap_nodes()
	}
	
	#[text_signature = "($self)"]
	/// Returns whether there are unknown edge types.
	///
	/// [Automatically generated binding]
	/// [Automatically generated documentation]
	fn has_unknown_edge_types(&self) -> bool {
		self.graph.has_unknown_edge_types()
	}
	
	#[text_signature = "($self)"]
	/// Returns whether there are unknown node types.
	///
	/// [Automatically generated binding]
	/// [Automatically generated documentation]
	fn has_unknown_node_types(&self) -> bool {
		self.graph.has_unknown_node_types()
	}
	
	#[text_signature = "($self)"]
	/// Returns boolean representing if graph is directed.
	///
	/// [Automatically generated binding]
	/// [Automatically generated documentation]
	fn is_directed(&self) -> bool {
		self.graph.is_directed()
	}
	
	#[text_signature = "($self)"]
	/// Return if there are multiple edges between two nodes
	///
	/// [Automatically generated binding]
	/// [Automatically generated documentation]
	fn is_multigraph(&self) -> bool {
		self.graph.is_multigraph()
	}
	
}
