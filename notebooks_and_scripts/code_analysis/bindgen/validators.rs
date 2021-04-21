use super::*;
impl Graph {

	#[text_signature = "($self)"]
	/// Raises an error if the graph does not have edge types.
	///
	/// [Automatically generated binding]
	/// [Automatically generated documentation]
	fn must_have_edge_types(&self) -> PyResult<&EdgeTypeVocabulary> {
		pe!(self.graph.must_have_edge_types())
	}
	
	#[text_signature = "($self)"]
	/// Raises an error if the graph does not have weights.
	///
	/// [Automatically generated binding]
	/// [Automatically generated documentation]
	fn must_have_edge_weights(&self) -> PyResult<&Vec<WeightT>> {
		pe!(self.graph.must_have_edge_weights())
	}
	
	#[text_signature = "($self)"]
	/// Raises an error if the graph does not have node types.
	///
	/// [Automatically generated binding]
	/// [Automatically generated documentation]
	fn must_have_node_types(&self) -> PyResult<&NodeTypeVocabulary> {
		pe!(self.graph.must_have_node_types())
	}
	
}
