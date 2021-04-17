use super::*;
impl Graph {

	#[text_signature = "($self, node_id)"]
	/// Returns boolean representing if given node is a singleton.
	/// 
	/// Paramenters
	/// --------------
	/// node_id : int,
	/// 	The node to be checked for.
	///
	/// [Automatically generated binding]
	fn is_unchecked_singleton_from_node_id(&self, node_id : NodeT) -> bool {
		self.graph.is_unchecked_singleton_from_node_id(node_id)
	}
	
	#[text_signature = "($self, node_id)"]
	/// Returns boolean representing if given node is a singleton.
	/// 
	/// Paramenters
	/// --------------
	/// node_id : int,
	/// 	The node to be checked for.
	///
	/// [Automatically generated binding]
	fn is_singleton_from_node_id(&self, node_id : NodeT) -> PyResult<bool> {
		pe!(self.graph.is_singleton_from_node_id(node_id))
	}
	
	#[text_signature = "($self, node_id)"]
	/// Returns boolean representing if given node is a singleton with self-loops.
	/// 
	/// Paramenters
	/// --------------
	/// node_id : int,
	/// 	The node to be checked for.
	///
	/// [Automatically generated binding]
	fn is_singleton_with_selfloops_from_node_id(&self, node_id : NodeT) -> bool {
		self.graph.is_singleton_with_selfloops_from_node_id(node_id)
	}
	
	#[text_signature = "($self, node_name)"]
	/// Returns boolean representing if given node is a singleton.
	/// 
	/// Paramenters
	/// --------------
	/// node_name : str,
	/// 	The node name to be checked for.
	///
	/// [Automatically generated binding]
	fn is_singleton_from_node_name(&self, node_name : &str) -> PyResult<bool> {
		pe!(self.graph.is_singleton_from_node_name(node_name))
	}
	
	#[text_signature = "($self, node_name)"]
	/// Returns whether the graph has the given node name.
	/// 
	/// Paramenters
	/// --------------
	/// node_name : str,
	/// 	Name of the node.
	///
	/// [Automatically generated binding]
	fn has_node_from_node_name(&self, node_name : &str) -> bool {
		self.graph.has_node_from_node_name(node_name)
	}
	
	#[text_signature = "($self, src, dst)"]
	/// Returns whether edge passing between given node ids exists.
	/// 
	/// Paramenters
	/// --------------
	/// src : int,
	/// 	Source node id.
	/// dst : int,
	/// 	Destination node id.
	///
	/// [Automatically generated binding]
	fn has_edge_from_node_ids(&self, src : NodeT, dst : NodeT) -> bool {
		self.graph.has_edge_from_node_ids(src, dst)
	}
	
	#[text_signature = "($self, src, dst, edge_type)"]
	/// Returns whether edge with the given type passing between given nodes exists.
	/// 
	/// Paramenters
	/// --------------
	/// src : int,
	/// 	The source node of the edge.
	/// dst : int,
	/// 	The destination node of the edge.
	/// edge_type : int,
	/// 	The (optional) edge type.
	///
	/// [Automatically generated binding]
	fn has_edge_from_node_ids_and_edge_type_id(&self, src : NodeT, dst : NodeT, edge_type : Option<EdgeTypeT>) -> bool {
		self.graph.has_edge_from_node_ids_and_edge_type_id(src, dst, edge_type)
	}
	
	#[text_signature = "($self, node_id)"]
	/// Returns boolean representing if given node is a trap.
	/// 
	/// If the provided node_id is higher than the number of nodes in the graph,
	/// the method will panic.
	/// 
	/// Paramenters
	/// --------------
	/// node_id : int,
	/// 	Integer ID of the node, if this is bigger that the number of nodes it will panic.
	///
	/// [Automatically generated binding]
	fn is_unchecked_trap_node_from_node_id(&self, node_id : NodeT) -> bool {
		self.graph.is_unchecked_trap_node_from_node_id(node_id)
	}
	
	#[text_signature = "($self, node_id)"]
	/// Returns boolean representing if given node is a trap.
	/// 
	/// Paramenters
	/// --------------
	/// node_id : int,
	/// 	Integer ID of the node, if this is bigger that the number of nodes it will panic.
	///
	/// [Automatically generated binding]
	fn is_trap_node_from_node_id(&self, node_id : NodeT) -> PyResult<bool> {
		pe!(self.graph.is_trap_node_from_node_id(node_id))
	}
	
	#[text_signature = "($self, node_name, node_type_name)"]
	/// Returns whether the given node name and node type name exist in current graph.
	/// 
	/// Paramenters
	/// --------------
	/// node_name : str,
	/// 	The node name.
	/// node_type_name : List[str],
	/// 	The node types name.
	///
	/// [Automatically generated binding]
	fn has_node_from_node_name_and_node_type_name(&self, node_name : &str, node_type_name : Option<Vec<String>>) -> bool {
		self.graph.has_node_from_node_name_and_node_type_name(node_name, node_type_name)
	}
	
	#[text_signature = "($self, src_name, dst_name)"]
	/// Returns whether if edge passing between given nodes exists.
	/// 
	/// Paramenters
	/// --------------
	/// src_name : str,
	/// 	The source node name of the edge.
	/// dst_name : str,
	/// 	The destination node name of the edge.
	///
	/// [Automatically generated binding]
	fn has_edge_from_node_names(&self, src_name : &str, dst_name : &str) -> bool {
		self.graph.has_edge_from_node_names(src_name, dst_name)
	}
	
	#[text_signature = "($self, src_name, dst_name, edge_type_name)"]
	/// Returns whether if edge with type passing between given nodes exists.
	/// 
	/// Paramenters
	/// --------------
	/// src_name : str,
	/// 	The source node name of the edge.
	/// dst_name : str,
	/// 	The destination node name of the edge.
	/// edge_type_name : str,
	/// 	The (optional) edge type name.
	///
	/// [Automatically generated binding]
	fn has_edge_from_node_names_and_edge_type_name(&self, src_name : &str, dst_name : &str, edge_type_name : Option<&String>) -> bool {
		self.graph.has_edge_from_node_names_and_edge_type_name(src_name, dst_name, edge_type_name)
	}
	
}
