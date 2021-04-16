use super::*;
impl Graph {

	#[text_signature = "($self, src, dst)"]
	/// TODO!: This binding was automatically generated
	/// 
	fn encode_edge(&self, src : NodeT, dst : NodeT) -> u64 {
		self.graph.encode_edge(src, dst)
	}
	
	#[text_signature = "($self, edge)"]
	/// TODO!: This binding was automatically generated
	/// 
	fn decode_edge(&self, edge : u64) -> (NodeT, NodeT) {
		self.graph.decode_edge(edge)
	}
	
	#[text_signature = "($self)"]
	/// TODO!: This binding was automatically generated
	/// Return maximum encodable edge number.
	fn get_max_encodable_edge_number(&self) -> EdgeT {
		self.graph.get_max_encodable_edge_number()
	}
	
}
