use super::*;
impl Graph {

	#[text_signature = "($self)"]
	/// Returns vector of singleton node IDs of the graph.
	///
	/// [Automatically generated binding]
	/// [Automatically generated documentation]
	fn get_singleton_node_ids(&self) -> Vec<NodeT> {
		self.graph.get_singleton_node_ids()
	}
	
	#[text_signature = "($self)"]
	/// Returns vector of singleton node names of the graph.
	///
	/// [Automatically generated binding]
	/// [Automatically generated documentation]
	fn get_singleton_node_names(&self) -> Vec<String> {
		self.graph.get_singleton_node_names()
	}
	
	#[text_signature = "($self)"]
	/// Returns vector of singleton_with_selfloops node IDs of the graph.
	///
	/// [Automatically generated binding]
	/// [Automatically generated documentation]
	fn get_singleton_with_selfloops_node_ids(&self) -> Vec<NodeT> {
		self.graph.get_singleton_with_selfloops_node_ids()
	}
	
	#[text_signature = "($self)"]
	/// Returns vector of singleton_with_selfloops node names of the graph.
	///
	/// [Automatically generated binding]
	/// [Automatically generated documentation]
	fn get_singleton_with_selfloops_node_names(&self) -> Vec<String> {
		self.graph.get_singleton_with_selfloops_node_names()
	}
	
}
