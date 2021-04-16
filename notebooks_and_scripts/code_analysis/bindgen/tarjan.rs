use super::*;
impl Graph {

	#[text_signature = "($self)"]
	/// TODO!: This binding was automatically generated
	/// Returns list of nodes of the various strongly connected components.
	/// 
	/// This is an implementation of Tarjan algorithm.
	fn strongly_connected_components(&self) -> Vec<HashSet<NodeT>> {
		self.graph.strongly_connected_components()
	}
	
}
