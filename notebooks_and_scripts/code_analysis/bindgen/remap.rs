use super::*;
impl Graph {

	#[text_signature = "($self, other)"]
	/// TODO!: This binding was automatically generated
	/// Return whether nodes are remappable to those of the given graph.
	/// 
	/// Paramenters
	/// --------------
	/// `other`: Graph - graph towards remap the nodes to.
	fn are_nodes_remappable(&self, other : &Graph) -> bool {
		self.graph.are_nodes_remappable(other)
	}
	
	#[text_signature = "($self, other, verbose)"]
	/// TODO!: This binding was automatically generated
	/// Return graph remapped towards nodes of the given graph.
	/// 
	/// Paramenters
	/// --------------
	/// other : Graph,
	/// 	The graph to remap towards.
	/// verbose : bool,
	/// 	Whether to show a loding bar.
	fn remap(&self, other : &Graph, verbose : bool) -> PyResult<Graph> {
		pe!(self.graph.remap(other, verbose))
	}
	
}
