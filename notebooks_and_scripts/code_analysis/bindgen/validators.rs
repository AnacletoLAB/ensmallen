use super::*;
impl Graph {

	#[text_signature = "($self)"]
	/// Raises an error if the graph does not have edge types.
	///
	/// [Automatically generated binding]
	/// [Automatically generated documentation]
	fn must_not_be_multigraph(&self) -> PyResult<()> {
		pe!(self.graph.must_not_be_multigraph())
	}
	
}
