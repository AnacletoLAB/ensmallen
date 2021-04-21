use super::*;
impl Graph {

	#[text_signature = "($self)"]
	/// Returns edge type IDs counts hashmap.
	/// 
	///  # Raises
	///  * If there are no edge types in the current graph instance.
	///
	/// [Automatically generated binding]
	/// [Automatically generated documentation]
	fn get_edge_type_id_counts_hashmap(&self) -> PyResult<HashMap<EdgeTypeT, EdgeT>> {
		pe!(self.graph.get_edge_type_id_counts_hashmap())
	}
	
	#[text_signature = "($self)"]
	/// Returns edge type names counts hashmap.
	/// 
	///  # Raises
	///  * If there are no edge types in the current graph instance.
	///
	/// [Automatically generated binding]
	/// [Automatically generated documentation]
	fn get_edge_type_names_counts_hashmap(&self) -> PyResult<HashMap<String, EdgeT>> {
		pe!(self.graph.get_edge_type_names_counts_hashmap())
	}
	
	#[text_signature = "($self)"]
	/// Returns node type IDs counts hashmap.
	/// 
	///  # Raises
	///  * If there are no node types in the current graph instance.
	///
	/// [Automatically generated binding]
	/// [Automatically generated documentation]
	fn get_node_type_id_counts_hashmap(&self) -> PyResult<HashMap<NodeTypeT, NodeT>> {
		pe!(self.graph.get_node_type_id_counts_hashmap())
	}
	
	#[text_signature = "($self)"]
	/// Returns node type names counts hashmap.
	/// 
	///  # Raises
	///  * If there are no node types in the current graph instance.
	///
	/// [Automatically generated binding]
	/// [Automatically generated documentation]
	fn get_node_type_names_counts_hashmap(&self) -> PyResult<HashMap<String, NodeT>> {
		pe!(self.graph.get_node_type_names_counts_hashmap())
	}
	
	#[text_signature = "($self)"]
	/// Returns vector of singleton edge types IDs.
	/// 
	///  # Raises
	///  * If the graph does not have edge types.
	///
	/// [Automatically generated binding]
	/// [Automatically generated documentation]
	fn get_singleton_edge_type_ids(&self) -> PyResult<Vec<EdgeTypeT>> {
		pe!(self.graph.get_singleton_edge_type_ids())
	}
	
	#[text_signature = "($self)"]
	/// Returns vector of singleton edge types names.
	/// 
	///  # Raises
	///  * If the graph does not have edge types.
	///
	/// [Automatically generated binding]
	/// [Automatically generated documentation]
	fn get_singleton_edge_type_names(&self) -> PyResult<Vec<String>> {
		pe!(self.graph.get_singleton_edge_type_names())
	}
	
	#[text_signature = "($self)"]
	/// Returns number of singleton edge types.
	/// 
	///  # Raises
	///  * If the graph does not have edge types.
	///
	/// [Automatically generated binding]
	/// [Automatically generated documentation]
	fn get_singleton_edge_types_number(&self) -> PyResult<EdgeTypeT> {
		pe!(self.graph.get_singleton_edge_types_number())
	}
	
	#[text_signature = "($self)"]
	/// Returns vector of singleton node IDs of the graph.
	///
	/// [Automatically generated binding]
	/// [Automatically generated documentation]
	fn get_singleton_node_ids(&self) -> Vec<NodeT> {
		self.graph.get_singleton_node_ids()
	}
	
	#[text_signature = "($self)"]
	/// Returns vector of singleton node names of the graph.
	///
	/// [Automatically generated binding]
	/// [Automatically generated documentation]
	fn get_singleton_node_names(&self) -> Vec<String> {
		self.graph.get_singleton_node_names()
	}
	
	#[text_signature = "($self)"]
	/// Returns vector of singleton node types IDs.
	/// 
	///  # Raises
	///  * If the graph does not have node types.
	///
	/// [Automatically generated binding]
	/// [Automatically generated documentation]
	fn get_singleton_node_type_ids(&self) -> PyResult<Vec<NodeTypeT>> {
		pe!(self.graph.get_singleton_node_type_ids())
	}
	
	#[text_signature = "($self)"]
	/// Returns vector of singleton node types names.
	/// 
	///  # Raises
	///  * If the graph does not have node types.
	///
	/// [Automatically generated binding]
	/// [Automatically generated documentation]
	fn get_singleton_node_type_names(&self) -> PyResult<Vec<String>> {
		pe!(self.graph.get_singleton_node_type_names())
	}
	
	#[text_signature = "($self)"]
	/// Returns number of singleton node types.
	/// 
	///  # Raises
	///  * If the graph does not have node types.
	///
	/// [Automatically generated binding]
	/// [Automatically generated documentation]
	fn get_singleton_node_types_number(&self) -> PyResult<NodeTypeT> {
		pe!(self.graph.get_singleton_node_types_number())
	}
	
	#[text_signature = "($self)"]
	/// Returns vector of singleton_with_selfloops node IDs of the graph.
	///
	/// [Automatically generated binding]
	/// [Automatically generated documentation]
	fn get_singleton_with_selfloops_node_ids(&self) -> Vec<NodeT> {
		self.graph.get_singleton_with_selfloops_node_ids()
	}
	
	#[text_signature = "($self)"]
	/// Returns vector of singleton_with_selfloops node names of the graph.
	///
	/// [Automatically generated binding]
	/// [Automatically generated documentation]
	fn get_singleton_with_selfloops_node_names(&self) -> Vec<String> {
		self.graph.get_singleton_with_selfloops_node_names()
	}
	
	#[text_signature = "($self)"]
	/// Return the unique edge type IDs of the graph edges.
	///
	/// [Automatically generated binding]
	/// [Automatically generated documentation]
	fn get_unique_edge_type_ids(&self) -> PyResult<Vec<EdgeTypeT>> {
		pe!(self.graph.get_unique_edge_type_ids())
	}
	
	#[text_signature = "($self)"]
	/// Return the edge types names.
	///
	/// [Automatically generated binding]
	/// [Automatically generated documentation]
	fn get_unique_edge_type_names(&self) -> PyResult<Vec<String>> {
		pe!(self.graph.get_unique_edge_type_names())
	}
	
	#[text_signature = "($self)"]
	/// Return the unique node type IDs of the graph nodes.
	///
	/// [Automatically generated binding]
	/// [Automatically generated documentation]
	fn get_unique_node_type_ids(&self) -> PyResult<Vec<NodeTypeT>> {
		pe!(self.graph.get_unique_node_type_ids())
	}
	
	#[text_signature = "($self)"]
	/// Return the unique node types names.
	///
	/// [Automatically generated binding]
	/// [Automatically generated documentation]
	fn get_unique_node_type_names(&self) -> PyResult<Vec<String>> {
		pe!(self.graph.get_unique_node_type_names())
	}
	
	#[text_signature = "($self)"]
	/// Returns rate of unknown edge types over total edges number.
	///
	/// [Automatically generated binding]
	/// [Automatically generated documentation]
	fn get_unknown_edge_types_rate(&self) -> PyResult<f64> {
		pe!(self.graph.get_unknown_edge_types_rate())
	}
	
	#[text_signature = "($self)"]
	/// Returns rate of unknown node types over total nodes number.
	///
	/// [Automatically generated binding]
	/// [Automatically generated documentation]
	fn get_unknown_node_types_rate(&self) -> PyResult<f64> {
		pe!(self.graph.get_unknown_node_types_rate())
	}
	
}
