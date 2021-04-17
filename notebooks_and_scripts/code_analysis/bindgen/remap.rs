use super::*;
impl Graph {

	#[text_signature = "($self, other)"]
	/// Return whether nodes are remappable to those of the given graph.
	/// 
	/// Paramenters
	/// --------------
	/// other : Graph,
	/// 	graph towards remap the nodes to.
	///
	/// [Automatically generated binding]
	/// [Automatically generated documentation]
	fn are_nodes_remappable(&self, other : &Graph) -> bool {
		self.graph.are_nodes_remappable(other)
	}
	
	#[text_signature = "($self, other, verbose)"]
	/// Return graph remapped towards nodes of the given graph.
	/// 
	/// Paramenters
	/// --------------
	/// other : Graph,
	/// 	The graph to remap towards.
	/// verbose : bool,
	/// 	Whether to show a loding bar.
	///
	/// [Automatically generated binding]
	/// [Automatically generated documentation]
	fn remap(&self, other : &Graph, verbose : bool) -> PyResult<Graph> {
		pe!(self.graph.remap(other, verbose))
	}
	
}
