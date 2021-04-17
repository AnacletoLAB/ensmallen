use super::*;
impl Graph {

	#[text_signature = "($self, allow_nodes_set, deny_nodes_set, allow_node_types_set, deny_node_types_set, allow_edge_set, deny_edge_set, allow_edge_types_set, deny_edge_types_set, weights, node_types, edge_types, singletons, selfloops, verbose)"]
	/// Returns a **NEW** Graph that does not have the required attributes.
	/// 
	/// Paramenters
	/// --------------
	/// allow_nodes_set : Dict[str],
	/// 	Optional set of nodes names to keep.
	/// deny_nodes_set : Dict[str],
	/// 	Optional set of nodes names to remove.
	/// allow_node_types_set : Dict[str],
	/// 	Optional set of node type names to keep.
	/// deny_node_types_set : Dict[str],
	/// 	Optional set of node type names to remove.
	/// allow_edge_set : Dict[int],
	/// 	Optional set of numeric edge IDs to keep.
	/// deny_edge_set : Dict[int],
	/// 	Optional set of numeric edge IDs to remove.
	/// allow_edge_types_set : Dict[str],
	/// 	Optional set of edge type names to keep.
	/// deny_edge_types_set : Dict[str],
	/// 	Optional set of edge type names to remove.
	/// weights : bool,
	/// 	Whether to remove the weights.
	/// node_types : bool,
	/// 	Whether to remove the node types.
	/// edge_types : bool,
	/// 	Whether to remove the edge types.
	/// singletons : bool,
	/// 	Whether to remove the singleton nodes.
	/// selfloops : bool,
	/// 	Whether to remove edges with self-loops.
	/// verbose : bool,
	/// 	Whether to show a loading bar while building the graph.
	/// 
	/// ## Implementation details
	/// 
	/// ### How the collapse of multigraphs is handled
	/// We keep only the first edge when a multigraph is collapsed while removing
	/// the edge types, in the order provided when first reading from the CSV file.
	/// 
	/// ### Generation of new singleton nodes when removeping edges
	/// Some of the remove operations allowed in this method might lead to the
	/// generation of new singleton nodes that will not be handled within this
	/// function call even if you provide the flag singletons to true, but you
	/// will need to call the method again if you want to get reed of also those
	/// newly created singleton nodes.
	///
	/// [Automatically generated binding]
	fn remove(&self, allow_nodes_set : Option<HashSet<String>>, deny_nodes_set : Option<HashSet<String>>, allow_node_types_set : Option<HashSet<String>>, deny_node_types_set : Option<HashSet<String>>, allow_edge_set : Option<HashSet<EdgeT>>, deny_edge_set : Option<HashSet<EdgeT>>, allow_edge_types_set : Option<HashSet<String>>, deny_edge_types_set : Option<HashSet<String>>, weights : bool, node_types : bool, edge_types : bool, singletons : bool, selfloops : bool, verbose : bool) -> PyResult<Graph> {
		pe!(self.graph.remove(allow_nodes_set, deny_nodes_set, allow_node_types_set, deny_node_types_set, allow_edge_set, deny_edge_set, allow_edge_types_set, deny_edge_types_set, weights, node_types, edge_types, singletons, selfloops, verbose))
	}
	
	#[text_signature = "($self, node_names, node_types, edge_types, minimum_component_size, top_k_components, verbose)"]
	/// remove all the components that are not connected to interesting
	/// nodes and edges.
	/// 
	/// Paramenters
	/// --------------
	/// node_names : List[str],
	/// 	The name of the nodes of which components to keep.
	/// node_types : List[Option<str]>,
	/// 	The types of the nodes of which components to keep.
	/// edge_types : List[Option<str]>,
	/// 	The types of the edges of which components to keep.
	/// minimum_component_size : int,
	/// 	Optional, Minimum size of the components to keep.
	/// top_k_components : int,
	/// 	Optional, number of components to keep sorted by number of nodes.
	/// verbose : bool,
	/// 	Whether to show the loading bar.
	///
	/// [Automatically generated binding]
	fn remove_components(&self, node_names : Option<Vec<String>>, node_types : Option<Vec<Option<String>>>, edge_types : Option<Vec<Option<String>>>, minimum_component_size : Option<NodeT>, top_k_components : Option<NodeT>, verbose : bool) -> PyResult<Graph> {
		pe!(self.graph.remove_components(node_names, node_types, edge_types, minimum_component_size, top_k_components, verbose))
	}
	
}
