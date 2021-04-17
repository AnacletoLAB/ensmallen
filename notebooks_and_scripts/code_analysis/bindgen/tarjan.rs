use super::*;
impl Graph {

	#[text_signature = "($self)"]
	/// Returns list of nodes of the various strongly connected components.
	/// 
	/// This is an implementation of Tarjan algorithm.
	///
	/// [Automatically generated binding]
	fn strongly_connected_components(&self) -> Vec<HashSet<NodeT>> {
		self.graph.strongly_connected_components()
	}
	
}
