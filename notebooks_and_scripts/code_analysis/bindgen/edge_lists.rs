use super::*;
impl Graph {

	#[text_signature = "($self, removed_existing_edges, first_nodes_set, second_nodes_set, first_node_types_set, second_node_types_set)"]
	/// TODO!: This binding was automatically generated
	/// Return vector of tuple of Node IDs that form the edges of the required bipartite graph.
	/// 
	/// Paramenters
	/// --------------
	/// removed_existing_edges : bool,
	/// 	Whether to filter out the existing edges. By default, true.
	/// first_nodes_set : Dict[str],
	/// 	Optional set of nodes to use to create the first set of nodes of the graph.
	/// second_nodes_set : Dict[str],
	/// 	Optional set of nodes to use to create the second set of nodes of the graph.
	/// first_node_types_set : Dict[str],
	/// 	Optional set of node types to create the first set of nodes of the graph.
	/// second_node_types_set : Dict[str],
	/// 	Optional set of node types to create the second set of nodes of the graph.
	fn get_bipartite_edges(&self, removed_existing_edges : Option<bool>, first_nodes_set : Option<HashSet<String>>, second_nodes_set : Option<HashSet<String>>, first_node_types_set : Option<HashSet<String>>, second_node_types_set : Option<HashSet<String>>) -> PyResult<Vec<Vec<NodeT>>> {
		pe!(self.graph.get_bipartite_edges(removed_existing_edges, first_nodes_set, second_nodes_set, first_node_types_set, second_node_types_set))
	}
	
	#[text_signature = "($self, removed_existing_edges, first_nodes_set, second_nodes_set, first_node_types_set, second_node_types_set)"]
	/// TODO!: This binding was automatically generated
	/// Return vector of tuple of Node IDs that form the edges of the required bipartite graph.
	/// 
	/// Paramenters
	/// --------------
	/// removed_existing_edges : bool,
	/// 	Whether to filter out the existing edges. By default, true.
	/// first_nodes_set : Dict[str],
	/// 	Optional set of nodes to use to create the first set of nodes of the graph.
	/// second_nodes_set : Dict[str],
	/// 	Optional set of nodes to use to create the second set of nodes of the graph.
	/// first_node_types_set : Dict[str],
	/// 	Optional set of node types to create the first set of nodes of the graph.
	/// second_node_types_set : Dict[str],
	/// 	Optional set of node types to create the second set of nodes of the graph.
	fn get_bipartite_edge_names(&self, removed_existing_edges : Option<bool>, first_nodes_set : Option<HashSet<String>>, second_nodes_set : Option<HashSet<String>>, first_node_types_set : Option<HashSet<String>>, second_node_types_set : Option<HashSet<String>>) -> PyResult<Vec<Vec<String>>> {
		pe!(self.graph.get_bipartite_edge_names(removed_existing_edges, first_nodes_set, second_nodes_set, first_node_types_set, second_node_types_set))
	}
	
	#[text_signature = "($self, central_node, removed_existing_edges, star_points_nodes_set, star_points_node_types_set)"]
	/// TODO!: This binding was automatically generated
	/// Return vector of tuple of Node IDs that form the edges of the required star.
	/// 
	/// Paramenters
	/// --------------
	/// central_node : str,
	/// 	Name of the node to use as center of the star.
	/// removed_existing_edges : bool,
	/// 	Whether to filter out the existing edges. By default, true.
	/// star_points_nodes_set : Dict[str],
	/// 	Optional set of nodes to use to create the set of star points.
	/// star_points_node_types_set : Dict[str],
	/// 	Optional set of node types to create the set of star points.
	fn get_star_edges(&self, central_node : String, removed_existing_edges : Option<bool>, star_points_nodes_set : Option<HashSet<String>>, star_points_node_types_set : Option<HashSet<String>>) -> PyResult<Vec<Vec<NodeT>>> {
		pe!(self.graph.get_star_edges(central_node, removed_existing_edges, star_points_nodes_set, star_points_node_types_set))
	}
	
	#[text_signature = "($self, central_node, removed_existing_edges, star_points_nodes_set, star_points_node_types_set)"]
	/// TODO!: This binding was automatically generated
	/// Return vector of tuple of Node names that form the edges of the required star.
	/// 
	/// Paramenters
	/// --------------
	/// central_node : str,
	/// 	Name of the node to use as center of the star.
	/// removed_existing_edges : bool,
	/// 	Whether to filter out the existing edges. By default, true.
	/// star_points_nodes_set : Dict[str],
	/// 	Optional set of nodes to use to create the set of star points.
	/// star_points_node_types_set : Dict[str],
	/// 	Optional set of node types to create the set of star points.
	fn get_star_edge_names(&self, central_node : String, removed_existing_edges : Option<bool>, star_points_nodes_set : Option<HashSet<String>>, star_points_node_types_set : Option<HashSet<String>>) -> PyResult<Vec<Vec<String>>> {
		pe!(self.graph.get_star_edge_names(central_node, removed_existing_edges, star_points_nodes_set, star_points_node_types_set))
	}
	
	#[text_signature = "($self, directed, allow_selfloops, removed_existing_edges, allow_node_type_set, allow_node_set)"]
	/// TODO!: This binding was automatically generated
	/// Return vector of tuple of Node IDs that form the edges of the required clique.
	/// 
	/// Paramenters
	/// --------------
	/// directed : bool,
	/// 	Whether to return the edges as directed or undirected. By default, equal to the graph.
	/// allow_selfloops : bool,
	/// 	Whether to allow self-loops in the clique. By default, equal to the graph.
	/// removed_existing_edges : bool,
	/// 	Whether to filter out the existing edges. By default, true.
	/// allow_node_type_set : Dict[str],
	/// 	Node types to include in the clique.
	/// allow_node_set : Dict[str],
	/// 	Nodes to include i the clique.
	fn get_clique_edges(&self, directed : Option<bool>, allow_selfloops : Option<bool>, removed_existing_edges : Option<bool>, allow_node_type_set : Option<HashSet<String>>, allow_node_set : Option<HashSet<String>>) -> Vec<Vec<NodeT>> {
		self.graph.get_clique_edges(directed, allow_selfloops, removed_existing_edges, allow_node_type_set, allow_node_set)
	}
	
	#[text_signature = "($self, directed, allow_selfloops, removed_existing_edges, allow_node_type_set, allow_node_set)"]
	/// TODO!: This binding was automatically generated
	/// Return vector of tuple of Node names that form the edges of the required clique.
	/// 
	/// Paramenters
	/// --------------
	/// directed : bool,
	/// 	Whether to return the edges as directed or undirected. By default, equal to the graph.
	/// allow_selfloops : bool,
	/// 	Whether to allow self-loops in the clique. By default, equal to the graph.
	/// removed_existing_edges : bool,
	/// 	Whether to filter out the existing edges. By default, true.
	/// allow_node_type_set : Dict[str],
	/// 	Node types to include in the clique.
	/// allow_node_set : Dict[str],
	/// 	Nodes to include i the clique.
	fn get_clique_edge_names(&self, directed : Option<bool>, allow_selfloops : Option<bool>, removed_existing_edges : Option<bool>, allow_node_type_set : Option<HashSet<String>>, allow_node_set : Option<HashSet<String>>) -> Vec<Vec<String>> {
		self.graph.get_clique_edge_names(directed, allow_selfloops, removed_existing_edges, allow_node_type_set, allow_node_set)
	}
	
}
