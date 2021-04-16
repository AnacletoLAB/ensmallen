use super::*;
impl Graph {

	#[text_signature = "($self)"]
	/// TODO!: This binding was automatically generated
	/// Return if the graph has any nodes.
	fn has_nodes(&self) -> bool {
		self.graph.has_nodes()
	}
	
	#[text_signature = "($self)"]
	/// TODO!: This binding was automatically generated
	/// Return if the graph has any edges.
	fn has_edges(&self) -> bool {
		self.graph.has_edges()
	}
	
	#[text_signature = "($self)"]
	/// TODO!: This binding was automatically generated
	/// 
	fn has_trap_nodes(&self) -> bool {
		self.graph.has_trap_nodes()
	}
	
	#[text_signature = "($self)"]
	/// TODO!: This binding was automatically generated
	/// Returns boolean representing if graph is directed.
	fn is_directed(&self) -> bool {
		self.graph.is_directed()
	}
	
	#[text_signature = "($self)"]
	/// TODO!: This binding was automatically generated
	/// Returns boolean representing whether graph has weights.
	fn has_edge_weights(&self) -> bool {
		self.graph.has_edge_weights()
	}
	
	#[text_signature = "($self)"]
	/// TODO!: This binding was automatically generated
	/// Returns boolean representing whether graph has edge types.
	fn has_edge_types(&self) -> bool {
		self.graph.has_edge_types()
	}
	
	#[text_signature = "($self)"]
	/// TODO!: This binding was automatically generated
	/// Returns boolean representing if graph has self-loops.
	fn has_selfloops(&self) -> bool {
		self.graph.has_selfloops()
	}
	
	#[text_signature = "($self)"]
	/// TODO!: This binding was automatically generated
	/// Returns boolean representing if graph has singletons.
	fn has_singletons(&self) -> bool {
		self.graph.has_singletons()
	}
	
	#[text_signature = "($self)"]
	/// TODO!: This binding was automatically generated
	/// Returns boolean representing if graph has singletons.
	fn has_singletons_with_selfloops(&self) -> bool {
		self.graph.has_singletons_with_selfloops()
	}
	
	#[text_signature = "($self)"]
	/// TODO!: This binding was automatically generated
	/// Returns boolean representing if graph has node types.
	fn has_node_types(&self) -> bool {
		self.graph.has_node_types()
	}
	
	#[text_signature = "($self)"]
	/// TODO!: This binding was automatically generated
	/// Returns boolean representing if graph has multilabel node types.
	fn has_multilabel_node_types(&self) -> bool {
		self.graph.has_multilabel_node_types()
	}
	
	#[text_signature = "($self)"]
	/// TODO!: This binding was automatically generated
	/// Returns whether there are unknown node types.
	fn has_unknown_node_types(&self) -> bool {
		self.graph.has_unknown_node_types()
	}
	
	#[text_signature = "($self)"]
	/// TODO!: This binding was automatically generated
	/// Returns whether there are unknown edge types.
	fn has_unknown_edge_types(&self) -> bool {
		self.graph.has_unknown_edge_types()
	}
	
	#[text_signature = "($self)"]
	/// TODO!: This binding was automatically generated
	/// Return if there are multiple edges between two nodes
	fn is_multigraph(&self) -> bool {
		self.graph.is_multigraph()
	}
	
}
