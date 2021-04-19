use super::*;
impl Graph {

	#[text_signature = "($self, verbose)"]
	/// Returns new graph without parallel edges.
	/// 
	/// Parameters
	/// --------------
	/// verbose: bool,
	/// 	Whether to show a loading bar while building the graph.
	///
	/// [Automatically generated binding]
	/// [Automatically generated documentation]
	fn drop_parallel_edges(&self, verbose : bool) -> EnsmallenGraph {
		EnsmallenGraph{graph:self.graph.drop_parallel_edges(verbose)}
	}
	
}
