use super::*;
impl Graph {

	#[text_signature = "($self, random_state, undesired_edge_types, verbose)"]
	/// Returns set of edges composing a spanning tree and connected components.
	/// 
	///  The spanning tree is NOT minimal.
	///  The given random_state is NOT the root of the tree.
	/// 
	/// Parameters
	/// --------------
	/// random_state: int,
	/// 	The random_state to use for the holdout,
	/// undesired_edge_types: Union[Dict[Option<int, None]]>,
	/// 	Which edge types id to try to avoid.
	/// verbose: bool,
	/// 	Whether to show a loading bar or not.
	///
	/// [Automatically generated binding]
	/// [Automatically generated documentation]
	fn random_spanning_arborescence_kruskal(&self, random_state : EdgeT, undesired_edge_types : &Option<HashSet<Option<EdgeTypeT>>>, verbose : bool) -> (HashSet<(NodeT, NodeT)>, Vec<NodeT>, NodeT, NodeT, NodeT) {
		self.graph.random_spanning_arborescence_kruskal(random_state, undesired_edge_types, verbose)
	}
	
	#[text_signature = "($self, verbose)"]
	/// Returns consistent spanning arborescence using Kruskal.
	/// 
	///  The spanning tree is NOT minimal.
	/// 
	/// Parameters
	/// --------------
	/// verbose: bool,
	/// 	Whether to show a loading bar or not.
	///
	/// [Automatically generated binding]
	/// [Automatically generated documentation]
	fn spanning_arborescence_kruskal(&self, verbose : bool) -> (HashSet<(NodeT, NodeT)>, Vec<NodeT>, NodeT, NodeT, NodeT) {
		self.graph.spanning_arborescence_kruskal(verbose)
	}
	
}
