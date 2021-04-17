use super::*;
impl Graph {

	#[text_signature = "($self, other)"]
	/// Return true if given graph has any edge overlapping with current graph.
	/// 
	/// Paramenters
	/// --------------
	/// other : Graph,
	/// 	The graph to check against.
	///
	/// [Automatically generated binding]
	fn overlaps(&self, other : &Graph) -> PyResult<bool> {
		pe!(self.graph.overlaps(other))
	}
	
	#[text_signature = "($self, other)"]
	/// Return true if given graph edges are all contained within current graph.
	/// 
	/// Paramenters
	/// --------------
	/// other : Graph,
	/// 	The graph to check against.
	///
	/// [Automatically generated binding]
	fn contains(&self, other : &Graph) -> PyResult<bool> {
		pe!(self.graph.contains(other))
	}
	
}
