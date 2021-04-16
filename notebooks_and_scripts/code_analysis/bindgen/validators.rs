use super::*;
impl Graph {

	#[text_signature = "($self, node_id)"]
	/// TODO!: This binding was automatically generated
	/// Validates provided node ID.
	/// 
	/// Paramenters
	/// --------------
	/// node_id : int,
	/// 	node ID to validate.
	fn validate_node_id(&self, node_id : NodeT) -> PyResult<NodeT> {
		pe!(self.graph.validate_node_id(node_id))
	}
	
	#[text_signature = "($self, edge_id)"]
	/// TODO!: This binding was automatically generated
	/// Validates provided edge ID.
	/// 
	/// Paramenters
	/// --------------
	/// edge_id : int,
	/// 	Edge ID to validate.
	fn validate_edge_id(&self, edge_id : EdgeT) -> PyResult<EdgeT> {
		pe!(self.graph.validate_edge_id(edge_id))
	}
	
	#[text_signature = "($self)"]
	/// TODO!: This binding was automatically generated
	/// Raises an error if the graph does not have node types.
	fn must_have_node_types(&self) -> PyResult<()> {
		pe!(self.graph.must_have_node_types())
	}
	
	#[text_signature = "($self)"]
	/// TODO!: This binding was automatically generated
	/// Raises an error if the graph does not have edge types.
	fn must_have_edge_types(&self) -> PyResult<()> {
		pe!(self.graph.must_have_edge_types())
	}
	
	#[text_signature = "($self)"]
	/// TODO!: This binding was automatically generated
	/// Raises an error if the graph does not have weights.
	fn must_have_edge_weights(&self) -> PyResult<()> {
		pe!(self.graph.must_have_edge_weights())
	}
	
}
