use super::*;
impl Graph {

	#[text_signature = "($self, other)"]
	/// Return whether nodes are remappable to those of the given graph.
	/// 
	/// Parameters
	/// --------------
	/// other: Graph,
	/// 	graph towards remap the nodes to.
	///
	/// [Automatically generated binding]
	/// [Automatically generated documentation]
	fn are_nodes_remappable(&self, other : &Graph) -> bool {
		self.graph.are_nodes_remappable(other)
	}
	
}
