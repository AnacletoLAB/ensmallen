use super::*;
use graph::{EdgeT, EdgeTypeT, NodeT, NodeTypeT, WeightT};

#[pymethods]
impl EnsmallenGraph {
	#[text_signature = "($self, verbose)"]
	/// Return rendered textual report of the graph.
	/// 
	/// Parameters
	/// --------------
	/// verbose: bool,
	/// 	Whether to show loading bar.
	///
	/// [Automatically generated binding]
	/// [Automatically generated documentation]
	fn textual_report(&self, verbose : bool) -> PyResult<String> {
		pe!(self.graph.textual_report(verbose))
	}
	
}
