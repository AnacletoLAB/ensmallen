use super::*;
impl Graph {

	#[text_signature = "($self, random_state, undesired_edge_types, verbose)"]
	/// TODO!: This binding was automatically generated
	/// Returns set of edges composing a spanning tree and connected components.
	/// 
	/// The spanning tree is NOT minimal.
	/// The given random_state is NOT the root of the tree.
	/// 
	/// Paramenters
	/// --------------
	/// random_state : int,
	/// 	The random_state to use for the holdout,
	/// undesired_edge_types : Dict[int],
	/// 	Which edge types id to try to avoid.
	/// verbose : bool,
	/// 	Whether to show a loading bar or not.
	fn random_spanning_arborescence_kruskal(&self, random_state : EdgeT, undesired_edge_types : &Option<HashSet<Option<EdgeTypeT>>>, verbose : bool) -> (HashSet<(NodeT, NodeT)>, Vec<NodeT>, NodeT, NodeT, NodeT) {
		self.graph.random_spanning_arborescence_kruskal(random_state, undesired_edge_types, verbose)
	}
	
	#[text_signature = "($self, verbose)"]
	/// TODO!: This binding was automatically generated
	/// Returns consistent spanning arborescence using Kruskal.
	/// 
	/// The spanning tree is NOT minimal.
	/// 
	/// Paramenters
	/// --------------
	/// verbose : bool,
	/// 	Whether to show a loading bar or not.
	fn spanning_arborescence_kruskal(&self, verbose : bool) -> (HashSet<(NodeT, NodeT)>, Vec<NodeT>, NodeT, NodeT, NodeT) {
		self.graph.spanning_arborescence_kruskal(verbose)
	}
	
	#[text_signature = "($self, verbose)"]
	/// TODO!: This binding was automatically generated
	/// Compute the connected components building in parallel a spanning tree using [bader's algorithm](https://www.sciencedirect.com/science/article/abs/pii/S0743731505000882).
	/// **This works only for undirected graphs.**
	/// 
	/// This method is **not thread save and not deterministic** but by design of the algorithm this
	/// shouldn't matter but if we will encounter non-detemristic bugs here is where we want to look.
	/// 
	/// Returns (Components membership, components number, size of the smallest components, size of the biggest components).
	/// We assign to each node the index of its component, so nodes in the same components will have the same index.
	/// This component index is the returned Components membership vector.
	/// 
	/// Paramenters
	/// --------------
	/// verbose : bool,
	/// 	Whether to show a loading bar or not.
	fn connected_components(&self, verbose : bool) -> PyResult<(Vec<NodeT>, NodeT, NodeT, NodeT)> {
		pe!(self.graph.connected_components(verbose))
	}
	
}
