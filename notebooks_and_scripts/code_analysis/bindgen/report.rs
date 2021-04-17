use super::*;
impl Graph {

	#[text_signature = "($self)"]
	/// Returns report relative to the graph metrics
	/// 
	/// The report includes a few useful metrics like:
	/// 
	/// * degrees_median: the median degree of the nodes.
	/// * degrees_mean: the mean degree of the nodes.
	/// * degrees_mode: the mode degree of the nodes.
	/// * min_degree: the max degree of the nodes.
	/// * max_degree: the min degree of the nodes.
	/// * nodes_number: the number of nodes in the graph.
	/// * edges_number: the number of edges in the graph.
	/// * unique_node_types_number: the number of different node types in the graph.
	/// * unique_edge_types_number: the number of different edge types in the graph.
	/// * traps_rate: probability to end up in a trap when starting into any given node.
	/// * selfloops_rate: pecentage of edges that are selfloops.
	/// * bidirectional_rate: rate of edges that are bidirectional.
	///
	/// [Automatically generated binding]
	/// [Automatically generated documentation]
	fn report(&self) -> DefaultHashMap<&str, String> {
		self.graph.report()
	}
	
	#[text_signature = "($self, other, verbose)"]
	/// Return rendered textual report about the graph overlaps.
	/// 
	/// Paramenters
	/// --------------
	/// other : Graph,
	/// 	graph to create overlap report with.
	/// verbose : bool,
	/// 	Whether to shor the loading bars.
	///
	/// [Automatically generated binding]
	/// [Automatically generated documentation]
	fn overlap_textual_report(&self, other : &Graph, verbose : bool) -> PyResult<String> {
		pe!(self.graph.overlap_textual_report(other, verbose))
	}
	
	#[text_signature = "($self, verbose)"]
	/// Return rendered textual report of the graph.
	/// 
	/// Paramenters
	/// --------------
	/// verbose : bool,
	/// 	Whether to show loading bar.
	///
	/// [Automatically generated binding]
	/// [Automatically generated documentation]
	fn textual_report(&self, verbose : bool) -> PyResult<String> {
		pe!(self.graph.textual_report(verbose))
	}
	
}
