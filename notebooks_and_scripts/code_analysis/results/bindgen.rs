use super::*;
impl Graph {

	#[text_signature = "($self, random_state, unwanted_edge_types, verbose)"]
	/// TODO!: This binding was automatically generated
	/// Returns set of edges composing a spanning tree and connected components.
	/// 
	/// The spanning tree is NOT minimal.
	/// The given random_state is NOT the root of the tree.
	/// 
	/// Paramenters
	/// --------------
	/// 
	///  `random_state : int,
	/// 	The random_state to use for the holdout,
	///  `include_all_edge_types : bool,
	/// 	whether to include all the edges between two nodes.
	///  `unwanted_edge_types : Dict[int],
	/// 	Which edge types id to try to avoid.
	///  `verbose : bool,
	/// 	whether to show a loading bar or not.
	fn random_spanning_arborescence_kruskal(&self, random_state : EdgeT, unwanted_edge_types : &Option<HashSet<Option<EdgeTypeT>>>, verbose : bool) -> (HashSet<(NodeT, NodeT)>, Vec<NodeT>, NodeT, NodeT, NodeT) {
		self.graph.random_spanning_arborescence_kruskal(random_state, unwanted_edge_types, verbose)
	}
	
	#[text_signature = "($self, verbose)"]
	/// TODO!: This binding was automatically generated
	/// 
	fn spanning_arborescence_kruskal(&self, verbose : bool) -> (HashSet<(NodeT, NodeT)>, Vec<NodeT>, NodeT, NodeT, NodeT) {
		self.graph.spanning_arborescence_kruskal(verbose)
	}
	
	#[text_signature = "($self, verbose)"]
	/// TODO!: This binding was automatically generated
	/// Returns set of edges composing a spanning tree.
	/// 
	/// This is the implementaiton of [A Fast, Parallel Spanning Tree Algorithm for Symmetric Multiprocessors (SMPs)](https://smartech.gatech.edu/bitstream/handle/1853/14355/GT-CSE-06-01.pdf)
	/// by David A. Bader and Guojing Cong.
	fn spanning_arborescence(&self, verbose : bool) -> PyResult<(usize, impl Iterator<Item = (NodeT, NodeT)> + '_)> {
		pe!(self.graph.spanning_arborescence(verbose))
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
	/// Example:
	fn connected_components(&self, verbose : bool) -> PyResult<(Vec<NodeT>, NodeT, NodeT, NodeT)> {
		pe!(self.graph.connected_components(verbose))
	}
	
	#[text_signature = "($self, other)"]
	/// TODO!: This binding was automatically generated
	/// Return true if given graph has any edge overlapping with current graph.
	/// 
	/// Paramenters
	/// --------------
	/// 
	///  other : Graph,
	/// 	The graph to check against.
	fn overlaps(self, other : &Graph) -> PyResult<bool> {
		pe!(self.graph.overlaps(other))
	}
	
	#[text_signature = "($self, other)"]
	/// TODO!: This binding was automatically generated
	/// Return true if given graph edges are all contained within current graph.
	/// 
	/// Paramenters
	/// --------------
	/// 
	///  other : Graph,
	/// 	The graph to check against.
	fn contains(self, other : &Graph) -> PyResult<bool> {
		pe!(self.graph.contains(other))
	}
	
	#[text_signature = "($self, walk_parameters, quantity, window_size)"]
	/// TODO!: This binding was automatically generated
	/// Return training batches for Node2Vec models.
	/// 
	/// The batch is composed of a tuple as the following:
	/// 
	/// - (Contexts indices, central nodes indices): the tuple of nodes
	/// 
	/// This does not provide any output value as the model uses NCE loss
	/// and basically the central nodes that are fed as inputs work as the
	/// outputs value.
	/// 
	/// Paramenters
	/// --------------
	/// 
	///  walk_parameters : WalksParameters,
	/// 	the weighted walks parameters.
	///  quantity : usize,
	/// 	Number of nodes to consider.
	///  window_size : usize,
	/// 	Window size to consider for the sequences.
	fn node2vec(&'a self, walk_parameters : &'a WalksParameters, quantity : NodeT, window_size : usize) -> PyResult<impl ParallelIterator<Item = (Vec<NodeT>, NodeT)> + 'a> {
		pe!(self.graph.node2vec(walk_parameters, quantity, window_size))
	}
	
	#[text_signature = "($self, walks_parameters, window_size, verbose)"]
	/// TODO!: This binding was automatically generated
	/// Return triple with CSR representation of cooccurrence matrix.
	/// 
	/// The first vector has the sources, the second vector the destinations
	/// and the third one contains the min-max normalized frequencies.
	/// 
	/// Paramenters
	/// --------------
	/// 
	///  parameters : WalksParameters,
	/// 	the walks parameters.
	///  window_size : usize,
	/// 	Window size to consider for the sequences.
	/// * `verbose`: bool -
	/// whether to show the progress bars.
	/// The default behaviour is false.
	fn cooccurence_matrix(&'a self, walks_parameters : &'a WalksParameters, window_size : usize, verbose : bool) -> PyResult<(usize, impl Iterator<Item=(NodeT, NodeT, f64)> + 'a)> {
		pe!(self.graph.cooccurence_matrix(walks_parameters, window_size, verbose))
	}
	
	#[text_signature = "($self, node_ids, random_state, include_central_node, offset, max_neighbours)"]
	/// TODO!: This binding was automatically generated
	/// Return iterator over neighbours for the given node IDs, optionally including given the node IDs, and node type.
	/// 
	/// This method is meant to be used to predict node labels using the NoLaN model.
	/// 
	/// If you need to predict the node label of a node, not during training,
	/// use `max_neighbours=None`.
	/// 
	/// Paramenters
	/// --------------
	/// 
	///  `node_ids : List[int],
	/// 	The node ID to retrieve neighbours for.
	///  `random_state : u64,
	/// 	The random state to use to extract the neighbours.
	///  `include_central_node : bool,
	/// 	Whether to include the node ID in the returned iterator.
	///  `offset : int,
	/// 	Offset for padding porposes.
	///  `max_neighbours : int,
	/// 	Number of maximum neighbours to consider.
	fn get_node_label_prediction_tuple_by_node_ids(&self, node_ids : Vec<NodeT>, random_state : u64, include_central_node : bool, offset : NodeT, max_neighbours : Option<NodeT>) -> PyResult<
	        impl Iterator<Item = (impl Iterator<Item = NodeT> + '_, Option<Vec<NodeTypeT>>)> + '_,
	        String> {
		pe!(self.graph.get_node_label_prediction_tuple_by_node_ids(node_ids, random_state, include_central_node, offset, max_neighbours))
	}
	
	#[text_signature = "($self, idx, batch_size, normalize, negative_samples, avoid_false_negatives, maximal_sampling_attempts, graph_to_avoid)"]
	/// TODO!: This binding was automatically generated
	/// Returns triple with the degrees of source nodes, destination nodes and labels for training model for link prediction.
	/// This method is just for setting the lowerbound on the simplest possible model.
	/// 
	/// Paramenters
	/// --------------
	/// 
	///  idx : u64,
	/// 	The index of the batch to generate, behaves like a random random_state,
	///  batch_size : usize,
	/// 	The maximal size of the batch to generate,
	///  normalize : bool,
	/// 	Divide the degrees by the max, this way the values are in [0, 1],
	///  negative_samples : f64,
	/// 	The component of netagetive samples to use,
	///  avoid_false_negatives : bool,
	/// 	whether to remove the false negatives when generated.
	/// - It should be left to false, as it has very limited impact on the training, but enabling this will slow things down.
	///  maximal_sampling_attempts : usize,
	/// 	Number of attempts to execute to sample the negative edges.
	///  graph_to_avoid : Graph,
	/// 	The graph whose edges are to be avoided during the generation of false negatives,
	fn link_prediction_degrees(&'a self, idx : u64, batch_size : usize, normalize : bool, negative_samples : f64, avoid_false_negatives : bool, maximal_sampling_attempts : usize, graph_to_avoid : &'a Option<&Graph>) -> PyResult<impl ParallelIterator<Item = (usize, f64, f64, bool)> + 'a> {
		pe!(self.graph.link_prediction_degrees(idx, batch_size, normalize, negative_samples, avoid_false_negatives, maximal_sampling_attempts, graph_to_avoid))
	}
	
	#[text_signature = "($self, idx, batch_size, negative_samples, avoid_false_negatives, maximal_sampling_attempts, graph_to_avoid)"]
	/// TODO!: This binding was automatically generated
	/// Returns triple with the ids of source nodes, destination nodes and labels for training model for link prediction.
	/// 
	/// Paramenters
	/// --------------
	/// 
	///  idx : u64,
	/// 	The index of the batch to generate, behaves like a random random_state,
	///  batch_size : usize,
	/// 	The maximal size of the batch to generate,
	///  negative_samples : f64,
	/// 	The component of netagetive samples to use,
	///  avoid_false_negatives : bool,
	/// 	whether to remove the false negatives when generated.
	/// - It should be left to false, as it has very limited impact on the training, but enabling this will slow things down.
	///  maximal_sampling_attempts : usize,
	/// 	Number of attempts to execute to sample the negative edges.
	///  graph_to_avoid : Graph,
	/// 	The graph whose edges are to be avoided during the generation of false negatives,
	fn link_prediction_ids(&'a self, idx : u64, batch_size : usize, negative_samples : f64, avoid_false_negatives : bool, maximal_sampling_attempts : usize, graph_to_avoid : &'a Option<&Graph>) -> PyResult<impl ParallelIterator<Item = (usize, NodeT, NodeT, bool)> + 'a> {
		pe!(self.graph.link_prediction_ids(idx, batch_size, negative_samples, avoid_false_negatives, maximal_sampling_attempts, graph_to_avoid))
	}
	
	#[text_signature = "($self, other)"]
	/// TODO!: This binding was automatically generated
	/// Return whether nodes are remappable to those of the given graph.
	/// 
	/// Paramenters
	/// --------------
	/// other: Graph - graph towards remap the nodes to.
	fn are_nodes_remappable(self, other : &Graph) -> bool {
		self.graph.are_nodes_remappable(other)
	}
	
	#[text_signature = "($self, other, verbose)"]
	/// TODO!: This binding was automatically generated
	/// Return graph remapped towards nodes of the given graph.
	/// 
	/// Paramenters
	/// --------------
	/// 
	///  other : Graph,
	/// 	The graph to remap towards.
	///  verbose : bool,
	/// 	whether to show a loding bar.
	fn remap(self, other : &Graph, verbose : bool) -> PyResult<Graph> {
		pe!(self.graph.remap(other, verbose))
	}
	
	#[text_signature = "($self, node_id)"]
	/// TODO!: This binding was automatically generated
	/// Returns boolean representing if given node is a singleton.
	/// 
	/// Paramenters
	/// --------------
	/// 
	/// `node_id`: int - The node to be checked for.
	fn is_singleton_by_node_id(self, node_id : NodeT) -> PyResult<bool> {
		pe!(self.graph.is_singleton_by_node_id(node_id))
	}
	
	#[text_signature = "($self, node_id)"]
	/// TODO!: This binding was automatically generated
	/// Returns boolean representing if given node is a singleton with self-loops.
	/// 
	/// Paramenters
	/// --------------
	/// 
	/// `node_id`: int - The node to be checked for.
	fn is_singleton_with_selfloops_by_node_id(self, node_id : NodeT) -> bool {
		self.graph.is_singleton_with_selfloops_by_node_id(node_id)
	}
	
	#[text_signature = "($self, node_name)"]
	/// TODO!: This binding was automatically generated
	/// Returns boolean representing if given node is a singleton.
	/// 
	/// Paramenters
	/// --------------
	/// `node_name`: str - The node name to be checked for.
	fn is_singleton_by_node_name(self, node_name : &str) -> PyResult<bool> {
		pe!(self.graph.is_singleton_by_node_name(node_name))
	}
	
	#[text_signature = "($self, node_name)"]
	/// TODO!: This binding was automatically generated
	/// Returns whether the graph has the given node name.
	/// 
	/// Paramenters
	/// --------------
	/// 
	///  `node_name : str,
	/// 	Name of the node.
	fn has_node_by_node_name(self, node_name : &str) -> bool {
		self.graph.has_node_by_node_name(node_name)
	}
	
	#[text_signature = "($self, src, dst)"]
	/// TODO!: This binding was automatically generated
	/// Returns whether edge passing between given node ids exists.
	/// 
	/// Paramenters
	/// --------------
	/// 
	///  `src : int,
	/// 	Source node id.
	///  `dst : int,
	/// 	Destination node id.
	fn has_edge_by_node_ids(self, src : NodeT, dst : NodeT) -> bool {
		self.graph.has_edge_by_node_ids(src, dst)
	}
	
	#[text_signature = "($self, src, dst, edge_type)"]
	/// TODO!: This binding was automatically generated
	/// Returns whether edge with the given type passing between given nodes exists.
	/// 
	/// Paramenters
	/// --------------
	/// 
	///  src : int,
	/// 	The source node of the edge.
	///  dst : int,
	/// 	The destination node of the edge.
	///  edge_type : int,
	/// 	The (optional) edge type.
	fn has_edge_with_type_by_node_ids(&self, src : NodeT, dst : NodeT, edge_type : Option<EdgeTypeT>) -> bool {
		self.graph.has_edge_with_type_by_node_ids(src, dst, edge_type)
	}
	
	#[text_signature = "($self, node_id)"]
	/// TODO!: This binding was automatically generated
	/// Returns boolean representing if given node is a trap.
	/// 
	/// Paramenters
	/// --------------
	/// 
	/// * `node_id`: NodeT - Integer ID of the node, if this is bigger that the number of nodes it will panic.
	fn is_trap_node_by_node_id(self, node_id : NodeT) -> PyResult<bool> {
		pe!(self.graph.is_trap_node_by_node_id(node_id))
	}
	
	#[text_signature = "($self, node_name, node_type_name)"]
	/// TODO!: This binding was automatically generated
	/// Returns whether the given node name and node type name exist in current graph.
	/// 
	/// Paramenters
	/// --------------
	/// 
	///  node_name : str,
	/// 	The node name.
	///  node_type_name : str,
	/// 	The node type name.
	fn has_node_with_type_by_node_name(&self, node_name : &str, node_type_name : Option<Vec<String>>) -> bool {
		self.graph.has_node_with_type_by_node_name(node_name, node_type_name)
	}
	
	#[text_signature = "($self, src_name, dst_name)"]
	/// TODO!: This binding was automatically generated
	/// Returns whether if edge passing between given nodes exists.
	/// 
	/// Paramenters
	/// --------------
	/// 
	///  src : str,
	/// 	The source node name of the edge.
	///  dst : str,
	/// 	The destination node name of the edge.
	fn has_edge_by_node_names(self, src_name : &str, dst_name : &str) -> bool {
		self.graph.has_edge_by_node_names(src_name, dst_name)
	}
	
	#[text_signature = "($self, src_name, dst_name, edge_type_name)"]
	/// TODO!: This binding was automatically generated
	/// Returns whether if edge with type passing between given nodes exists.
	/// 
	/// Paramenters
	/// --------------
	/// 
	///  src : str,
	/// 	The source node name of the edge.
	///  dst : str,
	/// 	The destination node name of the edge.
	///  edge_type : str,
	/// 	The (optional) edge type name.
	fn has_edge_with_type_by_node_names(&self, src_name : &str, dst_name : &str, edge_type_name : Option<&String>) -> bool {
		self.graph.has_edge_with_type_by_node_names(src_name, dst_name, edge_type_name)
	}
	
	#[text_signature = "($self, vector_sources, vector_destinations, vector_outbounds, cache_size)"]
	/// TODO!: This binding was automatically generated
	/// Enable extra perks that buys you time as you accept to spend more memory.
	/// 
	/// Paramenters
	/// --------------
	/// * `vector_sources`: bool - whether to cache sources into a vector for faster walks.
	/// * `vector_destinations`: bool - whether to cache destinations into a vector for faster walks.
	/// * `vector_outbounds`: bool - whether to cache outbounds into a vector for faster walks.
	/// * `cache_size`: f64, percentage of nodes destinations to cache. This cannot be used with the vector destinations.
	fn enable(&mut self, vector_sources : bool, vector_destinations : bool, vector_outbounds : bool, cache_size : Option<f64>) -> PyResult<()> {
		pe!(self.graph.enable(vector_sources, vector_destinations, vector_outbounds, cache_size))
	}
	
	#[text_signature = "($self)"]
	/// TODO!: This binding was automatically generated
	/// Disable all extra perks, reducing memory impact but incresing time requirements.
	fn disable_all(mut self){
		self.graph.disable_all()
	}
	
	#[text_signature = "($self)"]
	/// TODO!: This binding was automatically generated
	/// Returns list of nodes of the various strongly connected components.
	/// 
	/// This is an implementation of Tarjan algorithm.
	fn strongly_connected_components(self) -> Vec<HashSet<NodeT>> {
		self.graph.strongly_connected_components()
	}
	
	#[text_signature = "($self, edge_id)"]
	/// TODO!: This binding was automatically generated
	/// Return the src, dst, edge type and weight of a given edge id
	fn get_edge_quadruple(&self, edge_id : EdgeT) -> (NodeT, NodeT, Option<EdgeTypeT>, Option<WeightT>) {
		self.graph.get_edge_quadruple(edge_id)
	}
	
	#[text_signature = "($self, edge_id)"]
	/// TODO!: This binding was automatically generated
	/// Return the src, dst, edge type of a given edge id
	fn get_edge_triple(self, edge_id : EdgeT) -> (NodeT, NodeT, Option<EdgeTypeT>) {
		self.graph.get_edge_triple(edge_id)
	}
	
	#[text_signature = "($self, k)"]
	/// TODO!: This binding was automatically generated
	/// Return vector with top k central node Ids.
	/// 
	/// Paramenters
	/// --------------
	/// 
	///  k : int,
	/// 	Number of central nodes to extract.
	fn get_top_k_central_nodes_ids(self, k : NodeT) -> Vec<NodeT> {
		self.graph.get_top_k_central_nodes_ids(k)
	}
	
	#[text_signature = "($self, k)"]
	/// TODO!: This binding was automatically generated
	/// Return vector with top k central node names.
	/// 
	/// Paramenters
	/// --------------
	/// 
	///  k : int,
	/// 	Number of central nodes to extract.
	fn get_top_k_central_node_names(self, k : NodeT) -> Vec<String> {
		self.graph.get_top_k_central_node_names(k)
	}
	
	#[text_signature = "($self, node_id)"]
	/// TODO!: This binding was automatically generated
	/// Returns node type of given node.
	/// 
	/// Paramenters
	/// --------------
	/// 
	///  `node_id : int,
	/// 	node whose node type is to be returned.
	fn get_node_type_id_by_node_id(&self, node_id : NodeT) -> PyResult<Option<Vec<NodeTypeT>>> {
		pe!(self.graph.get_node_type_id_by_node_id(node_id))
	}
	
	#[text_signature = "($self, edge_id)"]
	/// TODO!: This binding was automatically generated
	/// Returns edge type of given edge.
	/// 
	/// Paramenters
	/// --------------
	/// 
	///  edge_id : int,
	/// 	edge whose edge type is to be returned.
	fn get_edge_type_id_by_edge_id(self, edge_id : EdgeT) -> PyResult<Option<EdgeTypeT>> {
		pe!(self.graph.get_edge_type_id_by_edge_id(edge_id))
	}
	
	#[text_signature = "($self, node_id)"]
	/// TODO!: This binding was automatically generated
	/// Returns result of option with the node type of the given node id.
	/// 
	/// Paramenters
	/// --------------
	/// `node_id`: int - The node ID whose node types are to be returned.
	fn get_node_type_name_by_node_id(&self, node_id : NodeT) -> PyResult<Option<Vec<String>>> {
		pe!(self.graph.get_node_type_name_by_node_id(node_id))
	}
	
	#[text_signature = "($self, edge_id)"]
	/// TODO!: This binding was automatically generated
	/// Returns option with the edge type of the given edge id.
	/// 
	/// Paramenters
	/// --------------
	/// `edge_id`: int - The edge ID whose edge type is to be returned.
	fn get_edge_type_name_by_edge_id(self, edge_id : EdgeT) -> PyResult<Option<String>> {
		pe!(self.graph.get_edge_type_name_by_edge_id(edge_id))
	}
	
	#[text_signature = "($self, edge_type_id)"]
	/// TODO!: This binding was automatically generated
	/// Return edge type name of given edge type.
	/// 
	/// Paramenters
	/// --------------
	///  edge_type_id : int,
	/// 	Id of the edge type.
	fn get_edge_type_name_by_edge_type_id(&self, edge_type_id : EdgeTypeT) -> PyResult<String> {
		pe!(self.graph.get_edge_type_name_by_edge_type_id(edge_type_id))
	}
	
	#[text_signature = "($self, edge_id)"]
	/// TODO!: This binding was automatically generated
	/// Returns weight of the given edge id.
	/// 
	/// Paramenters
	/// --------------
	///  `edge_id : int,
	/// 	The edge ID whose weight is to be returned.
	fn get_weight_by_edge_id(self, edge_id : EdgeT) -> PyResult<WeightT> {
		pe!(self.graph.get_weight_by_edge_id(edge_id))
	}
	
	#[text_signature = "($self, src, dst)"]
	/// TODO!: This binding was automatically generated
	/// Returns weight of the given node ids.
	/// 
	/// Paramenters
	/// --------------
	///  `src : int,
	/// 	The node ID of the source node.
	///  `dst : int,
	/// 	The node ID of the destination node.
	fn get_weight_by_node_ids(self, src : NodeT, dst : NodeT) -> PyResult<WeightT> {
		pe!(self.graph.get_weight_by_node_ids(src, dst))
	}
	
	#[text_signature = "($self, src, dst, edge_type)"]
	/// TODO!: This binding was automatically generated
	/// Returns weight of the given node ids and edge type.
	/// 
	/// Paramenters
	/// --------------
	///  `src : int,
	/// 	The node ID of the source node.
	///  `dst : int,
	/// 	The node ID of the destination node.
	///  `edge_type : int,
	/// 	The edge type ID of the edge.
	fn get_weight_with_type_by_node_ids(&self, src : NodeT, dst : NodeT, edge_type : Option<EdgeTypeT>) -> PyResult<WeightT> {
		pe!(self.graph.get_weight_with_type_by_node_ids(src, dst, edge_type))
	}
	
	#[text_signature = "($self, src, dst, edge_type)"]
	/// TODO!: This binding was automatically generated
	/// Returns weight of the given node names and edge type.
	/// 
	/// Paramenters
	/// --------------
	///  `src : str,
	/// 	The node name of the source node.
	///  `dst : str,
	/// 	The node name of the destination node.
	///  `edge_type : str,
	/// 	The edge type name of the edge.
	fn get_weight_with_type_by_node_names(&self, src : &str, dst : &str, edge_type : Option<&String>) -> PyResult<WeightT> {
		pe!(self.graph.get_weight_with_type_by_node_names(src, dst, edge_type))
	}
	
	#[text_signature = "($self, src_name, dst_name)"]
	/// TODO!: This binding was automatically generated
	/// Returns weight of the given node names.
	/// 
	/// Paramenters
	/// --------------
	///  `src_name : str,
	/// 	The node name of the source node.
	///  `dst_name : str,
	/// 	The node name of the destination node.
	fn get_weight_by_node_names(&self, src_name : &str, dst_name : &str) -> PyResult<WeightT> {
		pe!(self.graph.get_weight_by_node_names(src_name, dst_name))
	}
	
	#[text_signature = "($self, node_id)"]
	/// TODO!: This binding was automatically generated
	/// Returns result with the node name.
	fn get_node_name_by_node_id(self, node_id : NodeT) -> PyResult<String> {
		pe!(self.graph.get_node_name_by_node_id(node_id))
	}
	
	#[text_signature = "($self, node_name)"]
	/// TODO!: This binding was automatically generated
	/// Returns result with the node id.
	fn get_node_id_by_node_name(self, node_name : &str) -> PyResult<NodeT> {
		pe!(self.graph.get_node_id_by_node_name(node_name))
	}
	
	#[text_signature = "($self, node_name)"]
	/// TODO!: This binding was automatically generated
	/// Return node type ID for the given node name if available.
	/// 
	/// Paramenters
	/// --------------
	/// 
	///  `node_name : str,
	/// 	Name of the node.
	fn get_node_type_id_by_node_name(&self, node_name : &str) -> PyResult<Option<Vec<NodeTypeT>>> {
		pe!(self.graph.get_node_type_id_by_node_name(node_name))
	}
	
	#[text_signature = "($self, node_name)"]
	/// TODO!: This binding was automatically generated
	/// Return node type name for the given node name if available.
	/// 
	/// Paramenters
	/// --------------
	/// 
	///  `node_name : str,
	/// 	Name of the node.
	fn get_node_type_name_by_node_name(&self, node_name : &str) -> PyResult<Option<Vec<String>>> {
		pe!(self.graph.get_node_type_name_by_node_name(node_name))
	}
	
	#[text_signature = "($self, edge_type)"]
	/// TODO!: This binding was automatically generated
	/// Return number of edges with given edge type ID.
	/// 
	/// If None is given as an edge type ID, the unknown edge type IDs
	/// will be returned.
	/// 
	/// Paramenters
	/// --------------
	/// edge_type: int - The edge type ID to count the edges of.
	fn get_edge_count_by_edge_type_id(&self, edge_type : Option<EdgeTypeT>) -> PyResult<EdgeT> {
		pe!(self.graph.get_edge_count_by_edge_type_id(edge_type))
	}
	
	#[text_signature = "($self, edge_type_name)"]
	/// TODO!: This binding was automatically generated
	/// Return edge type ID curresponding to given edge type name.
	/// 
	/// If None is given as an edge type ID, None is returned.
	/// 
	/// Paramenters
	/// --------------
	/// edge_type: str - The edge type name whose ID is to be returned.
	fn get_edge_type_id_by_edge_type_name(&self, edge_type_name : Option<&str>) -> PyResult<Option<EdgeTypeT>> {
		pe!(self.graph.get_edge_type_id_by_edge_type_name(edge_type_name))
	}
	
	#[text_signature = "($self, edge_type)"]
	/// TODO!: This binding was automatically generated
	/// Return number of edges with given edge type name.
	/// 
	/// If None is given as an edge type name, the unknown edge types
	/// will be returned.
	/// 
	/// Paramenters
	/// --------------
	/// edge_type: str - The edge type name to count the edges of.
	fn get_edge_count_by_edge_type_name(&self, edge_type : Option<&str>) -> PyResult<EdgeT> {
		pe!(self.graph.get_edge_count_by_edge_type_name(edge_type))
	}
	
	#[text_signature = "($self, node_type_name)"]
	/// TODO!: This binding was automatically generated
	/// Return node type ID curresponding to given node type name.
	/// 
	/// If None is given as an node type ID, None is returned.
	/// 
	/// Paramenters
	/// --------------
	/// node_type: str - The node type name whose ID is to be returned.
	fn get_node_type_id_by_node_type_name(&self, node_type_name : &str) -> PyResult<NodeTypeT> {
		pe!(self.graph.get_node_type_id_by_node_type_name(node_type_name))
	}
	
	#[text_signature = "($self, node_type)"]
	/// TODO!: This binding was automatically generated
	/// Return number of nodes with given node type ID.
	/// 
	/// If None is given as an node type ID, the unknown node types
	/// will be returned.
	/// 
	/// Paramenters
	/// --------------
	/// node_type: int - The node type ID to count the nodes of.
	fn get_node_count_by_node_type_id(&self, node_type : Option<NodeTypeT>) -> PyResult<NodeT> {
		pe!(self.graph.get_node_count_by_node_type_id(node_type))
	}
	
	#[text_signature = "($self, node_type_name)"]
	/// TODO!: This binding was automatically generated
	/// Return number of nodes with given node type name.
	/// 
	/// If None is given as an node type name, the unknown node types
	/// will be returned.
	/// 
	/// Paramenters
	/// --------------
	/// node_type: str - The node type name to count the nodes of.
	fn get_node_count_by_node_type_name(&self, node_type_name : Option<&str>) -> PyResult<NodeT> {
		pe!(self.graph.get_node_count_by_node_type_name(node_type_name))
	}
	
	#[text_signature = "($self, edge_id)"]
	/// TODO!: This binding was automatically generated
	/// Returns the destination of given edge id.
	/// 
	/// Paramenters
	/// --------------
	/// 
	/// `edge_id`: int - The edge ID whose destination is to be retrieved.
	fn get_destination_node_id_by_edge_id(self, edge_id : EdgeT) -> PyResult<NodeT> {
		pe!(self.graph.get_destination_node_id_by_edge_id(edge_id))
	}
	
	#[text_signature = "($self, node_id)"]
	/// TODO!: This binding was automatically generated
	/// Return vector of destinations for the given source node ID.
	/// 
	/// Paramenters
	/// --------------
	/// 
	///  `node_id : int,
	/// 	Node ID whose neighbours are to be retrieved.
	fn get_node_neighbours_by_node_id(self, node_id : NodeT) -> PyResult<Vec<NodeT>> {
		pe!(self.graph.get_node_neighbours_by_node_id(node_id))
	}
	
	#[text_signature = "($self, node_name)"]
	/// TODO!: This binding was automatically generated
	/// Return vector of destinations for the given source node name.
	/// 
	/// Paramenters
	/// --------------
	/// 
	///  `node_name : str,
	/// 	Node ID whose neighbours are to be retrieved.
	fn get_neighbour_node_ids_by_node_name(self, node_name : &str) -> PyResult<Vec<NodeT>> {
		pe!(self.graph.get_neighbour_node_ids_by_node_name(node_name))
	}
	
	#[text_signature = "($self, node_name)"]
	/// TODO!: This binding was automatically generated
	/// Return vector of destination names for the given source node name.
	/// 
	/// Paramenters
	/// --------------
	/// 
	///  `node_id : int,
	/// 	Node ID whose neighbours are to be retrieved.
	fn get_neighbour_node_names_by_node_name(&self, node_name : &str) -> PyResult<Vec<String>> {
		pe!(self.graph.get_neighbour_node_names_by_node_name(node_name))
	}
	
	#[text_signature = "($self, src, dst, edge_type)"]
	/// TODO!: This binding was automatically generated
	/// Return edge ID for given tuple of nodes and edge type.
	/// 
	/// This method will return an error if the graph does not contain the
	/// requested edge with edge type.
	/// 
	/// Paramenters
	/// --------------
	/// `src`: int - Source node of the edge.
	/// `dst`: int - Destination node of the edge.
	/// `edge_type`: int - Edge Type of the edge.
	fn get_edge_id_with_type_by_node_ids(&self, src : NodeT, dst : NodeT, edge_type : Option<EdgeTypeT>) -> PyResult<EdgeT> {
		pe!(self.graph.get_edge_id_with_type_by_node_ids(src, dst, edge_type))
	}
	
	#[text_signature = "($self, src_name, dst_name)"]
	/// TODO!: This binding was automatically generated
	/// Return edge ID for given tuple of node names.
	/// 
	/// This method will return an error if the graph does not contain the
	/// requested edge with edge type.
	/// 
	/// Paramenters
	/// --------------
	/// `src_name`: str - Source node name of the edge.
	/// `dst_name`: str - Destination node name of the edge.
	fn get_edge_id_by_node_names(&self, src_name : &str, dst_name : &str) -> PyResult<EdgeT> {
		pe!(self.graph.get_edge_id_by_node_names(src_name, dst_name))
	}
	
	#[text_signature = "($self, src_name, dst_name, edge_type_name)"]
	/// TODO!: This binding was automatically generated
	/// Return edge ID for given tuple of node names and edge type name.
	/// 
	/// This method will return an error if the graph does not contain the
	/// requested edge with edge type.
	/// 
	/// Paramenters
	/// --------------
	/// `src_name`: str - Source node name of the edge.
	/// `dst_name`: str - Destination node name of the edge.
	/// `edge_type_name`: str - Edge type name.
	fn get_edge_id_with_type_by_node_names(&self, src_name : &str, dst_name : &str, edge_type_name : Option<&String>) -> PyResult<EdgeT> {
		pe!(self.graph.get_edge_id_with_type_by_node_names(src_name, dst_name, edge_type_name))
	}
	
	#[text_signature = "($self, edge_types)"]
	/// TODO!: This binding was automatically generated
	/// Return translated edge types from string to internal edge ID.
	/// 
	/// Paramenters
	/// --------------
	/// 
	///  `edge_types : List[str],
	/// 	Vector of edge types to be converted.
	fn get_edge_type_ids_by_edge_type_names(&self, edge_types : Vec<Option<String>>) -> PyResult<Vec<Option<EdgeTypeT>>> {
		pe!(self.graph.get_edge_type_ids_by_edge_type_names(edge_types))
	}
	
	#[text_signature = "($self, node_types)"]
	/// TODO!: This binding was automatically generated
	/// Return translated node types from string to internal node ID.
	/// 
	/// Paramenters
	/// --------------
	/// 
	///  `node_types : List[str],
	/// 	Vector of node types to be converted.
	fn get_node_type_ids_by_node_type_names(&self, node_types : Vec<Option<String>>) -> PyResult<Vec<Option<NodeTypeT>>> {
		pe!(self.graph.get_node_type_ids_by_node_type_names(node_types))
	}
	
	#[text_signature = "($self, node_type_id)"]
	/// TODO!: This binding was automatically generated
	/// Return node type name of given node type.
	/// 
	/// There is no need for a unchecked version since we will have to map
	/// on the note_types anyway.
	/// 
	/// Paramenters
	/// --------------
	///  node_type_id : List[int],
	/// 	Id of the node type.
	fn get_node_type_name_by_node_type_id(&self, node_type_id : NodeTypeT) -> PyResult<String> {
		pe!(self.graph.get_node_type_name_by_node_type_id(node_type_id))
	}
	
	#[text_signature = "($self, node_type_ids)"]
	/// TODO!: This binding was automatically generated
	/// Return node type name of given node type.
	/// 
	/// Paramenters
	/// --------------
	///  node_type_ids : List[int],
	/// 	Id of the node type.
	fn get_node_type_names_by_node_type_ids(&self, node_type_ids : Vec<NodeTypeT>) -> PyResult<Vec<String>> {
		pe!(self.graph.get_node_type_names_by_node_type_ids(node_type_ids))
	}
	
	#[text_signature = "($self, node_id)"]
	/// TODO!: This binding was automatically generated
	/// Returns the number of outbound neighbours of given node.
	/// 
	/// This is implemented as proposed by [S. Vigna here](http://vigna.di.unimi.it/ftp/papers/Broadword.pdf).
	/// 
	/// Paramenters
	/// --------------
	/// 
	/// * `node_id`: NodeT - Integer ID of the node.
	fn get_node_degree_by_node_id(self, node_id : NodeT) -> PyResult<NodeT> {
		pe!(self.graph.get_node_degree_by_node_id(node_id))
	}
	
	#[text_signature = "($self, node_names, node_types, edge_types, min_weight, max_weight, verbose)"]
	/// TODO!: This binding was automatically generated
	/// Return graph filtered by given weights range.
	/// 
	/// Paramenters
	/// --------------
	/// 
	///  node_names : List[str],
	/// 	The node names to keep.
	///  node_types : List[str],
	/// 	The node types to keep.
	///  edge_types : List[str],
	/// 	The edge types to keep.
	///  min_weight : WeightT,
	/// 	Minimum weight to use to filter edges.
	///  max_weight : WeightT,
	/// 	Maximum weight to use to filter edges.
	///  verbose : bool,
	/// 	whether to show the loading bar.
	fn filter(&self, node_names : Option<Vec<String>>, node_types : Option<Vec<Option<String>>>, edge_types : Option<Vec<Option<String>>>, min_weight : Option<WeightT>, max_weight : Option<WeightT>, verbose : bool) -> PyResult<Graph> {
		pe!(self.graph.filter(node_names, node_types, edge_types, min_weight, max_weight, verbose))
	}
	
	#[text_signature = "($self, one, two)"]
	/// TODO!: This binding was automatically generated
	/// Returns product of degrees of given nodes.
	/// 
	/// Paramenters
	/// --------------
	/// 
	/// * `one`: NodeT - Integer ID of the first node.
	/// * `two`: NodeT - Integer ID of the second node.
	fn degrees_product(self, one : NodeT, two : NodeT) -> PyResult<usize> {
		pe!(self.graph.degrees_product(one, two))
	}
	
	#[text_signature = "($self, one, two)"]
	/// TODO!: This binding was automatically generated
	/// Returns the Jaccard index for the two given nodes.
	/// 
	/// Paramenters
	/// --------------
	/// 
	/// * `one`: NodeT - Integer ID of the first node.
	/// * `two`: NodeT - Integer ID of the second node.
	/// 
	/// # References
	/// [D. Liben-Nowell, J. Kleinberg.
	/// The Link Prediction Problem for Social Networks (2004).](http://www.cs.cornell.edu/home/kleinber/link-pred.pdf)
	fn jaccard_index(self, one : NodeT, two : NodeT) -> PyResult<f64> {
		pe!(self.graph.jaccard_index(one, two))
	}
	
	#[text_signature = "($self, one, two)"]
	/// TODO!: This binding was automatically generated
	/// Returns the Adamic/Adar Index for the given pair of nodes.
	/// 
	/// Paramenters
	/// --------------:
	/// 
	/// * `one`: NodeT - Integer ID of the first node.
	/// * `two`: NodeT - Integer ID of the second node.
	/// 
	/// # Implementation details
	/// Since the Adamic/Adar Index is only defined for graph not containing
	/// node traps (nodes without any outbound edge) and must support all kind
	/// of graphs, the sinks node are excluded from
	/// the computation because they would result in an infinity.
	/// 
	/// # References
	/// [D. Liben-Nowell, J. Kleinberg.
	/// The Link Prediction Problem for Social Networks (2004).](http://www.cs.cornell.edu/home/kleinber/link-pred.pdf)
	fn adamic_adar_index(self, one : NodeT, two : NodeT) -> PyResult<f64> {
		pe!(self.graph.adamic_adar_index(one, two))
	}
	
	#[text_signature = "($self, one, two)"]
	/// TODO!: This binding was automatically generated
	/// Returns the Resource Allocation Index for the given pair of nodes.
	/// 
	/// Paramenters
	/// --------------:
	/// 
	/// * `one`: NodeT - Integer ID of the first node.
	/// * `two`: NodeT - Integer ID of the second node.
	/// 
	/// # References
	/// [T. Zhou, L. Lu, Y.-C. Zhang.
	/// Predicting missing links via local information.
	/// Eur. Phys. J. B 71 (2009) 623.](http://arxiv.org/pdf/0901.0553.pdf)
	/// 
	/// # Implementation details
	/// Since the Resource Allocation Index is only defined for graph not
	/// containing node traps (nodes without any outbound edge) and
	/// must support all kind of graphs, the sinks node are excluded from
	/// the computation because they would result in an infinity.
	fn resource_allocation_index(self, one : NodeT, two : NodeT) -> PyResult<f64> {
		pe!(self.graph.resource_allocation_index(one, two))
	}
	
	#[text_signature = "($self)"]
	/// TODO!: This binding was automatically generated
	/// Returns the traps rate of the graph.
	/// 
	/// THIS IS EXPERIMENTAL AND MUST BE PROVEN!
	fn get_traps_rate(self) -> f64 {
		self.graph.get_traps_rate()
	}
	
	#[text_signature = "($self)"]
	/// TODO!: This binding was automatically generated
	/// Returns mean node degree of the graph.
	fn get_node_degrees_mean(self) -> PyResult<f64> {
		pe!(self.graph.get_node_degrees_mean())
	}
	
	#[text_signature = "($self)"]
	/// TODO!: This binding was automatically generated
	/// Returns number of undirected edges of the graph.
	fn get_undirected_edges_number(self) -> EdgeT {
		self.graph.get_undirected_edges_number()
	}
	
	#[text_signature = "($self)"]
	/// TODO!: This binding was automatically generated
	/// Returns number of undirected edges of the graph.
	fn get_unique_undirected_edges_number(self) -> EdgeT {
		self.graph.get_unique_undirected_edges_number()
	}
	
	#[text_signature = "($self)"]
	/// TODO!: This binding was automatically generated
	/// Returns number of edges of the graph.
	fn get_edges_number(self) -> EdgeT {
		self.graph.get_edges_number()
	}
	
	#[text_signature = "($self)"]
	/// TODO!: This binding was automatically generated
	/// Returns number of unique edges of the graph.
	fn get_unique_edges_number(self) -> EdgeT {
		self.graph.get_unique_edges_number()
	}
	
	#[text_signature = "($self)"]
	/// TODO!: This binding was automatically generated
	/// Returns median node degree of the graph
	fn get_node_degrees_median(self) -> PyResult<NodeT> {
		pe!(self.graph.get_node_degrees_median())
	}
	
	#[text_signature = "($self)"]
	/// TODO!: This binding was automatically generated
	/// Returns maximum node degree of the graph
	fn get_max_node_degree(self) -> PyResult<NodeT> {
		pe!(self.graph.get_max_node_degree())
	}
	
	#[text_signature = "($self)"]
	/// TODO!: This binding was automatically generated
	/// Returns minimum node degree of the graph
	fn get_min_node_degree(self) -> PyResult<NodeT> {
		pe!(self.graph.get_min_node_degree())
	}
	
	#[text_signature = "($self)"]
	/// TODO!: This binding was automatically generated
	/// Returns mode node degree of the graph
	fn get_node_degrees_mode(self) -> PyResult<NodeT> {
		pe!(self.graph.get_node_degrees_mode())
	}
	
	#[text_signature = "($self)"]
	/// TODO!: This binding was automatically generated
	/// Returns number of self-loops, including also those in eventual multi-edges.
	fn get_selfloop_number(self) -> EdgeT {
		self.graph.get_selfloop_number()
	}
	
	#[text_signature = "($self)"]
	/// TODO!: This binding was automatically generated
	/// Returns number of unique self-loops, excluding those in eventual multi-edges.
	fn get_unique_selfloop_number(self) -> NodeT {
		self.graph.get_unique_selfloop_number()
	}
	
	#[text_signature = "($self)"]
	/// TODO!: This binding was automatically generated
	/// Returns rate of self-loops.
	fn get_selfloop_rate(self) -> PyResult<f64> {
		pe!(self.graph.get_selfloop_rate())
	}
	
	#[text_signature = "($self, verbose)"]
	/// TODO!: This binding was automatically generated
	/// Returns number a triple with (number of components, number of nodes of the smallest component, number of nodes of the biggest component )
	fn connected_components_number(self, verbose : bool) -> (NodeT, NodeT, NodeT) {
		self.graph.connected_components_number(verbose)
	}
	
	#[text_signature = "($self)"]
	/// TODO!: This binding was automatically generated
	/// Returns number of singleton nodes within the graph.
	fn get_singleton_nodes_number(self) -> NodeT {
		self.graph.get_singleton_nodes_number()
	}
	
	#[text_signature = "($self)"]
	/// TODO!: This binding was automatically generated
	/// Returns number of singleton nodes with self-loops within the graph.
	fn get_singleton_nodes_with_selfloops_number(self) -> NodeT {
		self.graph.get_singleton_nodes_with_selfloops_number()
	}
	
	#[text_signature = "($self)"]
	/// TODO!: This binding was automatically generated
	/// Returns number of not singleton nodes within the graph.
	fn get_not_singleton_nodes_number(self) -> NodeT {
		self.graph.get_not_singleton_nodes_number()
	}
	
	#[text_signature = "($self)"]
	/// TODO!: This binding was automatically generated
	/// Returns density of the graph.
	fn get_density(self) -> PyResult<f64> {
		pe!(self.graph.get_density())
	}
	
	#[text_signature = "($self)"]
	/// TODO!: This binding was automatically generated
	/// Returns report relative to the graph metrics
	/// 
	/// The report includes a few useful metrics like:
	/// 
	/// * degrees_median: the median degree of the nodes.
	/// * degrees_mean: the mean degree of the nodes.
	/// * degrees_mode: the mode degree of the nodes.
	/// * min_degree: the max degree of the nodes.
	/// * max_degree: the min degree of the nodes.
	/// * nodes_number: the number of nodes in the graph.
	/// * edges_number: the number of edges in the graph.
	/// * unique_node_types_number: the number of different node types in the graph.
	/// * unique_edge_types_number: the number of different edge types in the graph.
	/// * traps_rate: probability to end up in a trap when starting into any given node.
	/// * selfloops_rate: pecentage of edges that are selfloops.
	/// * bidirectional_rate: rate of edges that are bidirectional.
	fn report(self) -> DefaultHashMap<&str, String> {
		self.graph.report()
	}
	
	#[text_signature = "($self, other, verbose)"]
	/// TODO!: This binding was automatically generated
	/// Return rendered textual report about the graph overlaps.
	/// 
	/// Paramenters
	/// --------------
	/// 
	/// - `other`: Graph - graph to create overlap report with.
	/// - `verbose`: bool - whether to shor the loading bars.
	fn overlap_textual_report(self, other : &Graph, verbose : bool) -> PyResult<String> {
		pe!(self.graph.overlap_textual_report(other, verbose))
	}
	
	#[text_signature = "($self, verbose)"]
	/// TODO!: This binding was automatically generated
	/// Return rendered textual report of the graph.
	fn textual_report(self, verbose : bool) -> PyResult<String> {
		pe!(self.graph.textual_report(verbose))
	}
	
	#[text_signature = "($self, allow_nodes_set, deny_nodes_set, allow_node_types_set, deny_node_types_set, allow_edge_set, deny_edge_set, allow_edge_types_set, deny_edge_types_set, weights, node_types, edge_types, singletons, selfloops, verbose)"]
	/// TODO!: This binding was automatically generated
	/// Returns a **NEW** Graph that does not have the required attributes.
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
	/// Paramenters
	/// --------------
	///  `allow_nodes_set : Dict[str],
	/// 	Optional set of nodes names to keep.
	///  `deny_nodes_set : Dict[str],
	/// 	Optional set of nodes names to remove.
	///  `allow_node_types_set : Dict[str],
	/// 	Optional set of node type names to keep.
	///  `deny_node_types_set : Dict[str],
	/// 	Optional set of node type names to remove.
	///  `allow_edge_set : Dict[int],
	/// 	Optional set of numeric edge IDs to keep.
	///  `deny_edge_set : Dict[int],
	/// 	Optional set of numeric edge IDs to remove.
	///  `allow_edge_types_set : Dict[str],
	/// 	Optional set of edge type names to keep.
	///  `deny_edge_types_set : Dict[str],
	/// 	Optional set of edge type names to remove.
	///  `weights : bool,
	/// 	whether to remove the weights.
	///  `node_types : bool,
	/// 	whether to remove the node types.
	///  `edge_types : bool,
	/// 	whether to remove the edge types.
	///  `singletons : bool,
	/// 	whether to remove the singleton nodes.
	///  `selfloops : bool,
	/// 	whether to remove edges with self-loops.
	///  `verbose : bool,
	/// 	whether to show a loading bar while building the graph.
	fn remove(&self, allow_nodes_set : Option<HashSet<String>>, deny_nodes_set : Option<HashSet<String>>, allow_node_types_set : Option<HashSet<String>>, deny_node_types_set : Option<HashSet<String>>, allow_edge_set : Option<HashSet<EdgeT>>, deny_edge_set : Option<HashSet<EdgeT>>, allow_edge_types_set : Option<HashSet<String>>, deny_edge_types_set : Option<HashSet<String>>, weights : bool, node_types : bool, edge_types : bool, singletons : bool, selfloops : bool, verbose : bool) -> PyResult<Graph> {
		pe!(self.graph.remove(allow_nodes_set, deny_nodes_set, allow_node_types_set, deny_node_types_set, allow_edge_set, deny_edge_set, allow_edge_types_set, deny_edge_types_set, weights, node_types, edge_types, singletons, selfloops, verbose))
	}
	
	#[text_signature = "($self, node_names, node_types, edge_types, minimum_component_size, top_k_components, verbose)"]
	/// TODO!: This binding was automatically generated
	/// remove all the components that are not connected to interesting
	/// nodes and edges.
	/// 
	/// Paramenters
	/// --------------
	///  `node_names : List[str],
	/// 	The name of the nodes of which components to keep.
	///  `node_types : List[str],
	/// 	The types of the nodes of which components to keep.
	///  `edge_types : List[str],
	/// 	The types of the edges of which components to keep.
	///  `minimum_component_size : int,
	/// 	Optional, Minimum size of the components to keep.
	///  `top_k_components : int,
	/// 	Optional, number of components to keep sorted by number of nodes.
	///  `verbose : bool,
	/// 	whether to show the loading bar.
	fn remove_components(&self, node_names : Option<Vec<String>>, node_types : Option<Vec<Option<String>>>, edge_types : Option<Vec<Option<String>>>, minimum_component_size : Option<NodeT>, top_k_components : Option<NodeT>, verbose : bool) -> PyResult<Graph> {
		pe!(self.graph.remove_components(node_names, node_types, edge_types, minimum_component_size, top_k_components, verbose))
	}
	
	#[text_signature = "($self, name)"]
	/// TODO!: This binding was automatically generated
	/// Set the name of the graph.
	/// 
	/// Paramenters
	/// --------------
	/// 
	///  name : str,
	/// 	Name of the graph.
	fn set_name(mut self, name : String){
		self.graph.set_name(name)
	}
	
	#[text_signature = "($self, edge_type)"]
	/// TODO!: This binding was automatically generated
	/// Replace all edge types (if present) and set all the edge to edge_type.
	/// 
	/// Paramenters
	/// --------------
	/// - `edge_type`: str - The edge type to assing to all the edges.
	fn set_all_edge_types(ut self, edge_type : S) -> Graph {
		self.graph.set_all_edge_types(edge_type)
	}
	
	#[text_signature = "($self, node_type)"]
	/// TODO!: This binding was automatically generated
	/// Replace all node types (if present) and set all the node to node_type.
	/// 
	/// Paramenters
	/// --------------
	/// - `node_type`: str - The node type to assing to all the nodes.
	fn set_all_node_types(ut self, node_type : S) -> Graph {
		self.graph.set_all_node_types(node_type)
	}
	
	#[text_signature = "($self, src, dst)"]
	/// TODO!: This binding was automatically generated
	/// 
	fn encode_edge(self, src : NodeT, dst : NodeT) -> u64 {
		self.graph.encode_edge(src, dst)
	}
	
	#[text_signature = "($self, edge)"]
	/// TODO!: This binding was automatically generated
	/// 
	fn decode_edge(self, edge : u64) -> (NodeT, NodeT) {
		self.graph.decode_edge(edge)
	}
	
	#[text_signature = "($self, src, dst)"]
	/// TODO!: This binding was automatically generated
	/// 
	fn get_edge_id_by_node_ids(self, src : NodeT, dst : NodeT) -> PyResult<EdgeT> {
		pe!(self.graph.get_edge_id_by_node_ids(src, dst))
	}
	
	#[text_signature = "($self, removed_existing_edges, first_nodes_set, second_nodes_set, first_node_types_set, second_node_types_set)"]
	/// TODO!: This binding was automatically generated
	/// Return vector of tuple of Node IDs that form the edges of the required bipartite graph.
	/// 
	/// Paramenters
	/// --------------
	/// `removed_existing_edges`: bool - whether to filter out the existing edges. By default, true.
	/// `first_nodes_set`: HashMap<str> - Optional set of nodes to use to create the first set of nodes of the graph.
	/// `second_nodes_set`: HashMap<str> - Optional set of nodes to use to create the second set of nodes of the graph.
	/// `first_node_types_set`: HashMap<str> - Optional set of node types to create the first set of nodes of the graph.
	/// `second_node_types_set`: HashMap<str> - Optional set of node types to create the second set of nodes of the graph.
	fn get_bipartite_edges(&self, removed_existing_edges : Option<bool>, first_nodes_set : Option<HashSet<String>>, second_nodes_set : Option<HashSet<String>>, first_node_types_set : Option<HashSet<String>>, second_node_types_set : Option<HashSet<String>>) -> PyResult<Vec<Vec<NodeT>>> {
		pe!(self.graph.get_bipartite_edges(removed_existing_edges, first_nodes_set, second_nodes_set, first_node_types_set, second_node_types_set))
	}
	
	#[text_signature = "($self, removed_existing_edges, first_nodes_set, second_nodes_set, first_node_types_set, second_node_types_set)"]
	/// TODO!: This binding was automatically generated
	/// Return vector of tuple of Node IDs that form the edges of the required bipartite graph.
	/// 
	/// Paramenters
	/// --------------
	/// `removed_existing_edges`: bool - whether to filter out the existing edges. By default, true.
	/// `first_nodes_set`: HashMap<str> - Optional set of nodes to use to create the first set of nodes of the graph.
	/// `second_nodes_set`: HashMap<str> - Optional set of nodes to use to create the second set of nodes of the graph.
	/// `first_node_types_set`: HashMap<str> - Optional set of node types to create the first set of nodes of the graph.
	/// `second_node_types_set`: HashMap<str> - Optional set of node types to create the second set of nodes of the graph.
	fn get_bipartite_edge_names(&self, removed_existing_edges : Option<bool>, first_nodes_set : Option<HashSet<String>>, second_nodes_set : Option<HashSet<String>>, first_node_types_set : Option<HashSet<String>>, second_node_types_set : Option<HashSet<String>>) -> PyResult<Vec<Vec<String>>> {
		pe!(self.graph.get_bipartite_edge_names(removed_existing_edges, first_nodes_set, second_nodes_set, first_node_types_set, second_node_types_set))
	}
	
	#[text_signature = "($self, central_node, removed_existing_edges, star_points_nodes_set, star_points_node_types_set)"]
	/// TODO!: This binding was automatically generated
	/// Return vector of tuple of Node IDs that form the edges of the required star.
	/// 
	/// Paramenters
	/// --------------
	/// `central_node`: str - Name of the node to use as center of the star.
	/// `removed_existing_edges`: bool - whether to filter out the existing edges. By default, true.
	/// `star_points_nodes_set`: HashMap<str> - Optional set of nodes to use to create the set of star points.
	/// `star_points_node_types_set`: HashMap<str> - Optional set of node types to create the set of star points.
	fn get_star_edges(&self, central_node : String, removed_existing_edges : Option<bool>, star_points_nodes_set : Option<HashSet<String>>, star_points_node_types_set : Option<HashSet<String>>) -> PyResult<Vec<Vec<NodeT>>> {
		pe!(self.graph.get_star_edges(central_node, removed_existing_edges, star_points_nodes_set, star_points_node_types_set))
	}
	
	#[text_signature = "($self, central_node, removed_existing_edges, star_points_nodes_set, star_points_node_types_set)"]
	/// TODO!: This binding was automatically generated
	/// Return vector of tuple of Node names that form the edges of the required star.
	/// 
	/// Paramenters
	/// --------------
	/// `central_node`: str - Name of the node to use as center of the star.
	/// `removed_existing_edges`: bool - whether to filter out the existing edges. By default, true.
	/// `star_points_nodes_set`: HashMap<str> - Optional set of nodes to use to create the set of star points.
	/// `star_points_node_types_set`: HashMap<str> - Optional set of node types to create the set of star points.
	fn get_star_edge_names(&self, central_node : String, removed_existing_edges : Option<bool>, star_points_nodes_set : Option<HashSet<String>>, star_points_node_types_set : Option<HashSet<String>>) -> PyResult<Vec<Vec<String>>> {
		pe!(self.graph.get_star_edge_names(central_node, removed_existing_edges, star_points_nodes_set, star_points_node_types_set))
	}
	
	#[text_signature = "($self, directed, allow_selfloops, removed_existing_edges, allow_node_type_set, allow_node_set)"]
	/// TODO!: This binding was automatically generated
	/// Return vector of tuple of Node IDs that form the edges of the required clique.
	/// 
	/// Paramenters
	/// --------------
	/// `directed`: bool - whether to return the edges as directed or undirected. By default, equal to the graph.
	/// `allow_selfloops`: bool - whether to allow self-loops in the clique. By default, equal to the graph.
	/// `removed_existing_edges`: bool - whether to filter out the existing edges. By default, true.
	/// `allow_node_type_set`: Dict[str] - Node types to include in the clique.
	/// `allow_node_set`: Dict[str] - Nodes to include i the clique.
	fn get_clique_edges(&self, directed : Option<bool>, allow_selfloops : Option<bool>, removed_existing_edges : Option<bool>, allow_node_type_set : Option<HashSet<String>>, allow_node_set : Option<HashSet<String>>) -> Vec<Vec<NodeT>> {
		self.graph.get_clique_edges(directed, allow_selfloops, removed_existing_edges, allow_node_type_set, allow_node_set)
	}
	
	#[text_signature = "($self, directed, allow_selfloops, removed_existing_edges, allow_node_type_set, allow_node_set)"]
	/// TODO!: This binding was automatically generated
	/// Return vector of tuple of Node names that form the edges of the required clique.
	/// 
	/// Paramenters
	/// --------------
	/// `directed`: bool - whether to return the edges as directed or undirected. By default, equal to the graph.
	/// `allow_selfloops`: bool - whether to allow self-loops in the clique. By default, equal to the graph.
	/// `removed_existing_edges`: bool - whether to filter out the existing edges. By default, true.
	/// `allow_node_type_set`: Dict[str] - Node types to include in the clique.
	/// `allow_node_set`: Dict[str] - Nodes to include i the clique.
	fn get_clique_edge_names(&self, directed : Option<bool>, allow_selfloops : Option<bool>, removed_existing_edges : Option<bool>, allow_node_type_set : Option<HashSet<String>>, allow_node_set : Option<HashSet<String>>) -> Vec<Vec<String>> {
		self.graph.get_clique_edge_names(directed, allow_selfloops, removed_existing_edges, allow_node_type_set, allow_node_set)
	}
	
	#[text_signature = "($self)"]
	/// TODO!: This binding was automatically generated
	/// Return if the graph has any nodes.
	fn has_nodes(self) -> bool {
		self.graph.has_nodes()
	}
	
	#[text_signature = "($self)"]
	/// TODO!: This binding was automatically generated
	/// Return if the graph has any edges.
	fn has_edges(self) -> bool {
		self.graph.has_edges()
	}
	
	#[text_signature = "($self)"]
	/// TODO!: This binding was automatically generated
	/// Return name of the graph.
	fn get_name(self) -> String {
		self.graph.get_name()
	}
	
	#[text_signature = "($self)"]
	/// TODO!: This binding was automatically generated
	/// Return the number of traps (nodes without any outgoing edges that are not singletons)
	/// This also includes nodes with only a self-loops, therefore singletons with
	/// only a self-loops are not considered traps because you could make a walk on them.
	fn get_trap_nodes_number(self) -> EdgeT {
		self.graph.get_trap_nodes_number()
	}
	
	#[text_signature = "($self)"]
	/// TODO!: This binding was automatically generated
	/// 
	fn has_trap_nodes(self) -> bool {
		self.graph.has_trap_nodes()
	}
	
	#[text_signature = "($self)"]
	/// TODO!: This binding was automatically generated
	/// Returns boolean representing if graph is directed.
	fn is_directed(self) -> bool {
		self.graph.is_directed()
	}
	
	#[text_signature = "($self)"]
	/// TODO!: This binding was automatically generated
	/// Returns boolean representing whether graph has weights.
	fn has_edge_weights(self) -> bool {
		self.graph.has_edge_weights()
	}
	
	#[text_signature = "($self)"]
	/// TODO!: This binding was automatically generated
	/// Returns boolean representing whether graph has edge types.
	fn has_edge_types(self) -> bool {
		self.graph.has_edge_types()
	}
	
	#[text_signature = "($self)"]
	/// TODO!: This binding was automatically generated
	/// Returns boolean representing if graph has self-loops.
	fn has_selfloops(self) -> bool {
		self.graph.has_selfloops()
	}
	
	#[text_signature = "($self)"]
	/// TODO!: This binding was automatically generated
	/// Returns boolean representing if graph has singletons.
	fn has_singletons(self) -> bool {
		self.graph.has_singletons()
	}
	
	#[text_signature = "($self)"]
	/// TODO!: This binding was automatically generated
	/// Returns boolean representing if graph has singletons.
	fn has_singletons_with_selfloops(self) -> bool {
		self.graph.has_singletons_with_selfloops()
	}
	
	#[text_signature = "($self, directed)"]
	/// TODO!: This binding was automatically generated
	/// Return vector of the non-unique source nodes.
	/// 
	/// Paramenters
	/// --------------
	/// * `directed`: bool - whether to filter out the undirected edges.
	fn get_sources(self, directed : bool) -> Vec<NodeT> {
		self.graph.get_sources(directed)
	}
	
	#[text_signature = "($self, directed)"]
	/// TODO!: This binding was automatically generated
	/// Return vector of the non-unique source nodes names.
	/// 
	/// Paramenters
	/// --------------
	/// * `directed`: bool - whether to filter out the undirected edges.
	fn get_source_names(self, directed : bool) -> Vec<String> {
		self.graph.get_source_names(directed)
	}
	
	#[text_signature = "($self, directed)"]
	/// TODO!: This binding was automatically generated
	/// Return vector on the (non unique) destination nodes of the graph.
	/// 
	/// Paramenters
	/// --------------
	/// * `directed`: bool - whether to filter out the undirected edges.
	fn get_destinations(self, directed : bool) -> Vec<NodeT> {
		self.graph.get_destinations(directed)
	}
	
	#[text_signature = "($self, directed)"]
	/// TODO!: This binding was automatically generated
	/// Return vector of the non-unique destination nodes names.
	/// 
	/// Paramenters
	/// --------------
	/// * `directed`: bool - whether to filter out the undirected edges.
	fn get_destination_names(self, directed : bool) -> Vec<String> {
		self.graph.get_destination_names(directed)
	}
	
	#[text_signature = "($self)"]
	/// TODO!: This binding was automatically generated
	/// Return vector with the sorted nodes names.
	fn get_node_names(self) -> Vec<String> {
		self.graph.get_node_names()
	}
	
	#[text_signature = "($self)"]
	/// TODO!: This binding was automatically generated
	/// Return vector with the sorted nodes Ids.
	fn get_nodes(self) -> Vec<NodeT> {
		self.graph.get_nodes()
	}
	
	#[text_signature = "($self)"]
	/// TODO!: This binding was automatically generated
	/// Return the edge types of the edges.
	fn get_edge_types(self) -> PyResult<Vec<Option<EdgeTypeT>>> {
		pe!(self.graph.get_edge_types())
	}
	
	#[text_signature = "($self)"]
	/// TODO!: This binding was automatically generated
	/// Return the edge types names.
	fn get_edge_type_names(self) -> Option<Vec<String>> {
		self.graph.get_edge_type_names()
	}
	
	#[text_signature = "($self)"]
	/// TODO!: This binding was automatically generated
	/// Return the node types of the nodes.
	fn get_node_types(self) -> PyResult<Vec<Option<Vec<NodeTypeT>>>> {
		pe!(self.graph.get_node_types())
	}
	
	#[text_signature = "($self)"]
	/// TODO!: This binding was automatically generated
	/// Return the weights of the edges.
	fn get_weights(self) -> PyResult<Vec<WeightT>> {
		pe!(self.graph.get_weights())
	}
	
	#[text_signature = "($self)"]
	/// TODO!: This binding was automatically generated
	/// Return the minimum weight, if graph has weights.
	fn get_min_weight(self) -> PyResult<WeightT> {
		pe!(self.graph.get_min_weight())
	}
	
	#[text_signature = "($self)"]
	/// TODO!: This binding was automatically generated
	/// Return the maximum weight, if graph has weights.
	fn get_max_weight(self) -> PyResult<WeightT> {
		pe!(self.graph.get_max_weight())
	}
	
	#[text_signature = "($self)"]
	/// TODO!: This binding was automatically generated
	/// Return the node types names.
	fn get_node_type_names(self) -> Option<Vec<String>> {
		self.graph.get_node_type_names()
	}
	
	#[text_signature = "($self)"]
	/// TODO!: This binding was automatically generated
	/// Return number of the unique edges in the graph.
	fn get_unique_directed_edges_number(self) -> EdgeT {
		self.graph.get_unique_directed_edges_number()
	}
	
	#[text_signature = "($self)"]
	/// TODO!: This binding was automatically generated
	/// Return maximum encodable edge number.
	fn get_max_encodable_edge_number(self) -> EdgeT {
		self.graph.get_max_encodable_edge_number()
	}
	
	#[text_signature = "($self)"]
	/// TODO!: This binding was automatically generated
	/// Return the nodes mapping.
	fn get_nodes_mapping(self) -> HashMap<String, NodeT> {
		self.graph.get_nodes_mapping()
	}
	
	#[text_signature = "($self, directed)"]
	/// TODO!: This binding was automatically generated
	/// Return vector with the sorted edge Ids.
	fn get_edges(self, directed : bool) -> Vec<Vec<NodeT>> {
		self.graph.get_edges(directed)
	}
	
	#[text_signature = "($self, directed)"]
	/// TODO!: This binding was automatically generated
	/// Return vector with the sorted edge names.
	fn get_edge_names(self, directed : bool) -> Vec<(String, String)> {
		self.graph.get_edge_names(directed)
	}
	
	#[text_signature = "($self)"]
	/// TODO!: This binding was automatically generated
	/// Returns boolean representing if graph has node types.
	fn has_node_types(self) -> bool {
		self.graph.has_node_types()
	}
	
	#[text_signature = "($self)"]
	/// TODO!: This binding was automatically generated
	/// Returns boolean representing if graph has multilabel node types.
	fn has_multilabel_node_types(self) -> bool {
		self.graph.has_multilabel_node_types()
	}
	
	#[text_signature = "($self)"]
	/// TODO!: This binding was automatically generated
	/// Returns number of unknown node types.
	fn get_unknown_node_types_number(self) -> NodeT {
		self.graph.get_unknown_node_types_number()
	}
	
	#[text_signature = "($self)"]
	/// TODO!: This binding was automatically generated
	/// Returns minimum number of node types.
	fn get_minimum_node_types_number(self) -> NodeT {
		self.graph.get_minimum_node_types_number()
	}
	
	#[text_signature = "($self)"]
	/// TODO!: This binding was automatically generated
	/// Returns whether there are unknown node types.
	fn has_unknown_node_types(self) -> bool {
		self.graph.has_unknown_node_types()
	}
	
	#[text_signature = "($self)"]
	/// TODO!: This binding was automatically generated
	/// Returns number of unknown edge types.
	fn get_unknown_edge_types_number(self) -> EdgeT {
		self.graph.get_unknown_edge_types_number()
	}
	
	#[text_signature = "($self)"]
	/// TODO!: This binding was automatically generated
	/// Returns minimum number of edge types.
	fn get_minimum_edge_types_number(self) -> EdgeT {
		self.graph.get_minimum_edge_types_number()
	}
	
	#[text_signature = "($self)"]
	/// TODO!: This binding was automatically generated
	/// Returns whether there are unknown edge types.
	fn has_unknown_edge_types(self) -> bool {
		self.graph.has_unknown_edge_types()
	}
	
	#[text_signature = "($self)"]
	/// TODO!: This binding was automatically generated
	/// Returns number of nodes in the graph.
	fn get_nodes_number(self) -> NodeT {
		self.graph.get_nodes_number()
	}
	
	#[text_signature = "($self, verbose)"]
	/// TODO!: This binding was automatically generated
	/// Return a vector with the components each node belongs to.
	/// 
	/// E.g. If we have two components `[0, 2, 3]` and `[1, 4, 5]` the result will look like
	/// `[0, 1, 0, 0, 1, 1]`
	/// 
	/// Paramenters
	/// --------------
	///  `verbose : bool,
	/// 	whether to show the loading bar.
	fn get_node_components_vector(self, verbose : bool) -> Vec<NodeT> {
		self.graph.get_node_components_vector(verbose)
	}
	
	#[text_signature = "($self)"]
	/// TODO!: This binding was automatically generated
	/// Returns number of directed edges in the graph.
	fn get_directed_edges_number(self) -> EdgeT {
		self.graph.get_directed_edges_number()
	}
	
	#[text_signature = "($self)"]
	/// TODO!: This binding was automatically generated
	/// Returns number of edge types in the graph.
	fn get_edge_types_number(self) -> EdgeTypeT {
		self.graph.get_edge_types_number()
	}
	
	#[text_signature = "($self)"]
	/// TODO!: This binding was automatically generated
	/// Returns number of node types in the graph.
	fn get_node_types_number(self) -> NodeTypeT {
		self.graph.get_node_types_number()
	}
	
	#[text_signature = "($self)"]
	/// TODO!: This binding was automatically generated
	/// Returns the degree of every node in the graph.
	fn get_node_degrees(self) -> Vec<NodeT> {
		self.graph.get_node_degrees()
	}
	
	#[text_signature = "($self)"]
	/// TODO!: This binding was automatically generated
	/// Return set of nodes that are not singletons.
	fn get_not_singletons(self) -> Vec<NodeT> {
		self.graph.get_not_singletons()
	}
	
	#[text_signature = "($self)"]
	/// TODO!: This binding was automatically generated
	/// Return mapping from instance not trap nodes to dense nodes.
	fn get_dense_node_mapping(self) -> HashMap<NodeT, NodeT> {
		self.graph.get_dense_node_mapping()
	}
	
	#[text_signature = "($self)"]
	/// TODO!: This binding was automatically generated
	/// Return if there are multiple edges between two nodes
	fn is_multigraph(self) -> bool {
		self.graph.is_multigraph()
	}
	
	#[text_signature = "($self)"]
	/// TODO!: This binding was automatically generated
	/// Return number of edges that have multigraph syblings.
	fn get_multigraph_edges_number(self) -> EdgeT {
		self.graph.get_multigraph_edges_number()
	}
	
	#[text_signature = "($self)"]
	/// TODO!: This binding was automatically generated
	/// Return vector with node degrees
	fn get_outbounds(self) -> Vec<EdgeT> {
		self.graph.get_outbounds()
	}
	
	#[text_signature = "($self)"]
	/// TODO!: This binding was automatically generated
	/// Returns number of the source nodes.
	fn get_unique_source_nodes_number(self) -> NodeT {
		self.graph.get_unique_source_nodes_number()
	}
	
	#[text_signature = "($self)"]
	/// TODO!: This binding was automatically generated
	/// Returns edge type counts.
	/// 
	/// Paramenters
	/// --------------
	/// 
	/// None
	fn get_edge_type_counts(self) -> PyResult<Counter<EdgeTypeT, usize>> {
		pe!(self.graph.get_edge_type_counts())
	}
	
	#[text_signature = "($self)"]
	/// TODO!: This binding was automatically generated
	/// Returns edge type counts hashmap.
	/// 
	/// Paramenters
	/// --------------
	/// 
	/// None
	fn get_edge_type_counts_hashmap(self) -> PyResult<HashMap<EdgeTypeT, usize>> {
		pe!(self.graph.get_edge_type_counts_hashmap())
	}
	
	#[text_signature = "($self)"]
	/// TODO!: This binding was automatically generated
	/// Returns node type counts.
	/// 
	/// Paramenters
	/// --------------
	/// 
	/// None
	fn get_node_type_counts(self) -> PyResult<Counter<NodeTypeT, usize>> {
		pe!(self.graph.get_node_type_counts())
	}
	
	#[text_signature = "($self)"]
	/// TODO!: This binding was automatically generated
	/// Returns node type counts hashmap.
	/// 
	/// Paramenters
	/// --------------
	/// 
	/// None
	fn get_node_type_counts_hashmap(self) -> PyResult<HashMap<EdgeTypeT, usize>> {
		pe!(self.graph.get_node_type_counts_hashmap())
	}
	
	#[text_signature = "($self, node_file_reader, directed, directed_edge_list, edges_number, nodes_number, name)"]
	/// TODO!: This binding was automatically generated
	/// Return graph renderized from given files.
	/// 
	/// Paramenters
	/// --------------
	/// 
	///  `edge_file_reader : EdgeFileReader,
	/// 	Reader of the edge file.
	///  `node_file_reader : NodeFileReader,
	/// 	Reader of the node file.
	///  `directed : bool,
	/// 	whether the graph is to be read as directed or undirected.
	///  `directed_edge_list : bool,
	/// 	whether to read the edge list as directed.
	///  `edges_number : usize,
	/// 	Number of edges of the graph.
	///  `nodes_number : int,
	/// 	Number of the nodes of the graph.
	///  `name : S,
	/// 	Name of the graph.
	fn from_sorted_csv(EdgeFileReader, node_file_reader : Option<NodeFileReader>, directed : bool, directed_edge_list : bool, edges_number : usize, nodes_number : NodeT, name : S) -> PyResult<Graph> {
		pe!(self.graph.from_sorted_csv(node_file_reader, directed, directed_edge_list, edges_number, nodes_number, name))
	}
	
	#[text_signature = "($self, node_file_reader, directed, directed_edge_list, name)"]
	/// TODO!: This binding was automatically generated
	/// Return graph renderized from given files.
	/// 
	/// Paramenters
	/// --------------
	/// 
	///  `edge_file_reader : EdgeFileReader,
	/// 	Reader of the edge file.
	///  `node_file_reader : NodeFileReader,
	/// 	Reader of the node file.
	///  `directed : bool,
	/// 	whether the graph is to be read as directed or undirected.
	///  `directed_edge_list : bool,
	/// 	whether to read the edge list as directed.
	fn from_unsorted_csv(EdgeFileReader, node_file_reader : Option<NodeFileReader>, directed : bool, directed_edge_list : bool, name : S) -> PyResult<Graph> {
		pe!(self.graph.from_unsorted_csv(node_file_reader, directed, directed_edge_list, name))
	}
	
	#[text_signature = "($self)"]
	/// TODO!: This binding was automatically generated
	/// 
	fn compute_hash(self) -> u64 {
		self.graph.compute_hash()
	}
	
	#[text_signature = "($self, nodes_iterator, directed, directed_edge_list, name, ignore_duplicated_nodes, node_list_is_correct, ignore_duplicated_edges, edge_list_is_correct, verbose, numeric_edge_type_ids, numeric_node_ids, numeric_edge_node_ids, numeric_node_types_ids, has_node_types, has_edge_types, has_edge_weights, might_have_singletons, might_have_singletons_with_selfloops, might_have_trap_nodes)"]
	/// TODO!: This binding was automatically generated
	/// Create new Graph object from unsorted source.
	/// 
	/// Paramenters
	/// --------------
	/// 
	/// TODO: UPDATE THE DOCSTRING!
	/// 
	/// * `edges_iterator`: impl Iterator<Item = Result<strQuadruple, str>>,
	/// Iterator of the edges.
	/// * `nodes_iterator`: impl Iterator<Item = Result<(str, Option<str), str>>>,
	/// Iterator of the nodes.
	/// * `directed`: bool -
	/// Wether the graph should be directed or undirected.
	/// * `ignore_duplicated_nodes`: bool -
	/// Wether to ignore duplicated nodes or to raise a proper exception.
	/// * `ignore_duplicated_edges`: bool -
	/// Wether to ignore duplicated edges or to raise a proper exception.
	/// * `skip_selfloops`: bool -
	/// Wether to skip self loops while reading the the edges iterator.
	fn from_string_unsorted(impl Iterator<Item = Result<StringQuadruple, String>>, nodes_iterator : Option<impl Iterator<Item = Result<(String, Option<Vec<String>>), String>>>, directed : bool, directed_edge_list : bool, name : S, ignore_duplicated_nodes : bool, node_list_is_correct : bool, ignore_duplicated_edges : bool, edge_list_is_correct : bool, verbose : bool, numeric_edge_type_ids : bool, numeric_node_ids : bool, numeric_edge_node_ids : bool, numeric_node_types_ids : bool, has_node_types : bool, has_edge_types : bool, has_edge_weights : bool, might_have_singletons : bool, might_have_singletons_with_selfloops : bool, might_have_trap_nodes : bool) -> PyResult<Graph> {
		pe!(self.graph.from_string_unsorted(nodes_iterator, directed, directed_edge_list, name, ignore_duplicated_nodes, node_list_is_correct, ignore_duplicated_edges, edge_list_is_correct, verbose, numeric_edge_type_ids, numeric_node_ids, numeric_edge_node_ids, numeric_node_types_ids, has_node_types, has_edge_types, has_edge_weights, might_have_singletons, might_have_singletons_with_selfloops, might_have_trap_nodes))
	}
	
	#[text_signature = "($self, nodes, node_types, edge_types_vocabulary, directed, name, ignore_duplicated_edges, has_edge_types, has_edge_weights, verbose, might_have_singletons, might_have_singletons_with_selfloops, might_have_trap_nodes)"]
	/// TODO!: This binding was automatically generated
	/// Create new Graph object from unsorted source.
	/// 
	/// Paramenters
	/// --------------
	/// 
	/// * `edges_iterator`: impl Iterator<Item = Result<strQuadruple, str>>,
	/// Iterator of the edges.
	/// * `nodes_iterator`: impl Iterator<Item = Result<(str, Option<str), str>>>,
	/// Iterator of the nodes.
	/// * `directed`: bool -
	/// Wether the graph should be directed or undirected.
	/// * `ignore_duplicated_nodes`: bool -
	/// Wether to ignore duplicated nodes or to raise a proper exception.
	/// * `ignore_duplicated_edges`: bool -
	/// Wether to ignore duplicated edges or to raise a proper exception.
	/// * `skip_selfloops`: bool -
	/// Wether to skip self loops while reading the the edges iterator.
	fn from_integer_unsorted(impl Iterator<
	            Item = Result<(NodeT, NodeT, Option<NodeTypeT>, Option<WeightT>), String>,
	        >, nodes : Vocabulary<NodeT>, node_types : Option<NodeTypeVocabulary>, edge_types_vocabulary : Option<Vocabulary<EdgeTypeT>>, directed : bool, name : String, ignore_duplicated_edges : bool, has_edge_types : bool, has_edge_weights : bool, verbose : bool, might_have_singletons : bool, might_have_singletons_with_selfloops : bool, might_have_trap_nodes : bool) -> PyResult<Graph> {
		pe!(self.graph.from_integer_unsorted(nodes, node_types, edge_types_vocabulary, directed, name, ignore_duplicated_edges, has_edge_types, has_edge_weights, verbose, might_have_singletons, might_have_singletons_with_selfloops, might_have_trap_nodes))
	}
	
	#[text_signature = "($self, nodes_iterator, directed, directed_edge_list, ignore_duplicated_nodes, node_list_is_correct, ignore_duplicated_edges, edge_list_is_correct, edges_number, nodes_number, numeric_edge_type_ids, numeric_node_ids, numeric_edge_node_ids, numeric_node_types_ids, has_node_types, has_edge_types, has_edge_weights, might_have_singletons, might_have_singletons_with_selfloops, might_have_trap_nodes, name)"]
	/// TODO!: This binding was automatically generated
	/// Create new Graph object from sorted sources.
	fn from_string_sorted(impl Iterator<Item = Result<StringQuadruple, String>>, nodes_iterator : Option<impl Iterator<Item = Result<(String, Option<Vec<String>>), String>>>, directed : bool, directed_edge_list : bool, ignore_duplicated_nodes : bool, node_list_is_correct : bool, ignore_duplicated_edges : bool, edge_list_is_correct : bool, edges_number : usize, nodes_number : NodeT, numeric_edge_type_ids : bool, numeric_node_ids : bool, numeric_edge_node_ids : bool, numeric_node_types_ids : bool, has_node_types : bool, has_edge_types : bool, has_edge_weights : bool, might_have_singletons : bool, might_have_singletons_with_selfloops : bool, might_have_trap_nodes : bool, name : S) -> PyResult<Graph> {
		pe!(self.graph.from_string_sorted(nodes_iterator, directed, directed_edge_list, ignore_duplicated_nodes, node_list_is_correct, ignore_duplicated_edges, edge_list_is_correct, edges_number, nodes_number, numeric_edge_type_ids, numeric_node_ids, numeric_edge_node_ids, numeric_node_types_ids, has_node_types, has_edge_types, has_edge_weights, might_have_singletons, might_have_singletons_with_selfloops, might_have_trap_nodes, name))
	}
	
	#[text_signature = "($self, random_state, negatives_number, seed_graph, only_from_same_component, verbose)"]
	/// TODO!: This binding was automatically generated
	/// Returns Graph with given amount of negative edges as positive edges.
	/// 
	/// The graph generated may be used as a testing negatives partition to be
	/// fed into the argument "graph_to_avoid" of the link_prediction or the
	/// skipgrams algorithm.
	/// 
	/// 
	/// Paramenters
	/// --------------
	/// 
	///  `random_state : int,
	/// 	random_state to use to reproduce negative edge set.
	///  `negatives_number : int,
	/// 	Number of negatives edges to include.
	///  `seed_graph : Graph,
	/// 	Optional graph to use to filter the negative edges. The negative edges generated when this variable is provided will always have a node within this graph.
	///  `only_from_same_component : bool,
	/// 	whether to sample negative edges only from nodes that are from the same component.
	///  `verbose : bool,
	/// 	whether to show the loading bar.
	fn sample_negatives(&self, random_state : EdgeT, negatives_number : EdgeT, seed_graph : Option<&Graph>, only_from_same_component : bool, verbose : bool) -> PyResult<Graph> {
		pe!(self.graph.sample_negatives(random_state, negatives_number, seed_graph, only_from_same_component, verbose))
	}
	
	#[text_signature = "($self, random_state, train_size, edge_types, include_all_edge_types, verbose)"]
	/// TODO!: This binding was automatically generated
	/// Returns holdout for training ML algorithms on the graph structure.
	/// 
	/// The holdouts returned are a tuple of graphs. The first one, which
	/// is the training graph, is garanteed to have the same number of
	/// graph components as the initial graph. The second graph is the graph
	/// meant for testing or validation of the algorithm, and has no garantee
	/// to be connected. It will have at most (1-train_size) edges,
	/// as the bound of connectivity which is required for the training graph
	/// may lead to more edges being left into the training partition.
	/// 
	/// In the option where a list of edge types has been provided, these
	/// edge types will be those put into the validation set.
	/// 
	/// Paramenters
	/// --------------
	/// 
	///  `random_state : int,
	/// 	The random_state to use for the holdout,
	///  `train_size : f64,
	/// 	Rate target to reserve for training.
	///  `edge_types : List[str],
	/// 	Edge types to be selected for in the validation set.
	///  `include_all_edge_types : bool,
	/// 	whether to include all the edges between two nodes.
	///  `verbose : bool,
	/// 	whether to show the loading bar.
	fn connected_holdout(&self, random_state : EdgeT, train_size : f64, edge_types : Option<Vec<Option<String>>>, include_all_edge_types : bool, verbose : bool) -> PyResult<(Graph, Graph)> {
		pe!(self.graph.connected_holdout(random_state, train_size, edge_types, include_all_edge_types, verbose))
	}
	
	#[text_signature = "($self, random_state, train_size, include_all_edge_types, edge_types, min_number_overlaps, verbose)"]
	/// TODO!: This binding was automatically generated
	/// Returns random holdout for training ML algorithms on the graph edges.
	/// 
	/// The holdouts returned are a tuple of graphs. In neither holdouts the
	/// graph connectivity is necessarily preserved. To maintain that, use
	/// the method `connected_holdout`.
	/// 
	/// Paramenters
	/// --------------
	/// 
	///  `random_state : int,
	/// 	The random_state to use for the holdout,
	///  `train_size : f64,
	/// 	rate target to reserve for training
	///  `include_all_edge_types : bool,
	/// 	whether to include all the edges between two nodes.
	///  `edge_types : List[str],
	/// 	The edges to include in validation set.
	///  `min_number_overlaps : usize,
	/// 	The minimum number of overlaps to include the edge into the validation set.
	///  `verbose : bool,
	/// 	whether to show the loading bar.
	fn random_holdout(&self, random_state : EdgeT, train_size : f64, include_all_edge_types : bool, edge_types : Option<Vec<Option<String>>>, min_number_overlaps : Option<EdgeT>, verbose : bool) -> PyResult<(Graph, Graph)> {
		pe!(self.graph.random_holdout(random_state, train_size, include_all_edge_types, edge_types, min_number_overlaps, verbose))
	}
	
	#[text_signature = "($self, train_size, use_stratification, random_state)"]
	/// TODO!: This binding was automatically generated
	/// Returns node-label holdout for training ML algorithms on the graph node labels.
	/// 
	/// Paramenters
	/// --------------
	/// 
	///  `train_size : f64,
	/// 	rate target to reserve for training,
	///  `use_stratification : bool,
	/// 	Whether to use node-label stratification,
	///  `random_state : int,
	/// 	The random_state to use for the holdout,
	fn node_label_holdout(&self, train_size : f64, use_stratification : bool, random_state : EdgeT) -> PyResult<(Graph, Graph)> {
		pe!(self.graph.node_label_holdout(train_size, use_stratification, random_state))
	}
	
	#[text_signature = "($self, train_size, use_stratification, random_state)"]
	/// TODO!: This binding was automatically generated
	/// Returns edge-label holdout for training ML algorithms on the graph edge labels.
	/// 
	/// Paramenters
	/// --------------
	/// 
	///  `train_size : f64,
	/// 	rate target to reserve for training,
	///  `use_stratification : bool,
	/// 	Whether to use edge-label stratification,
	///  `random_state : int,
	/// 	The random_state to use for the holdout,
	fn edge_label_holdout(&self, train_size : f64, use_stratification : bool, random_state : EdgeT) -> PyResult<(Graph, Graph)> {
		pe!(self.graph.edge_label_holdout(train_size, use_stratification, random_state))
	}
	
	#[text_signature = "($self, random_state, nodes_number, verbose)"]
	/// TODO!: This binding was automatically generated
	/// Returns subgraph with given number of nodes.
	/// 
	/// This method creates a subset of the graph starting from a random node
	/// sampled using given random_state and includes all neighbouring nodes until
	/// the required number of nodes is reached. All the edges connecting any
	/// of the selected nodes are then inserted into this graph.
	/// 
	/// This is meant to execute distributed node embeddings.
	/// It may also sample singleton nodes.
	/// 
	/// Paramenters
	/// --------------
	/// 
	///  `random_state : usize,
	/// 	Random random_state to use.
	///  `nodes_number : usize,
	/// 	Number of nodes to extract.
	///  `verbose : bool,
	/// 	whether to show the loading bar.
	fn random_subgraph(&self, random_state : usize, nodes_number : NodeT, verbose : bool) -> PyResult<Graph> {
		pe!(self.graph.random_subgraph(random_state, nodes_number, verbose))
	}
	
	#[text_signature = "($self, k, k_index, edge_types, random_state, verbose)"]
	/// TODO!: This binding was automatically generated
	/// Returns train and test graph following kfold validation scheme.
	/// 
	/// The edges are splitted into k chunks. The k_index-th chunk is used to build
	/// the validation graph, all the other edges create the training graph.
	/// 
	/// Paramenters
	/// --------------
	/// 
	///  `edge_types : List[str],
	/// 	Edge types to be selected when computing the folds
	/// (All the edge types not listed here will be always be used in the training set).
	///  `k : u64,
	/// 	The number of folds.
	///  `k_index : u64,
	/// 	Which fold to use for the validation.
	///  `random_state : int,
	/// 	The random_state (seed) to use for the holdout,
	///  `verbose : bool,
	/// 	whether to show the loading bar.
	fn kfold(&self, k : EdgeT, k_index : u64, edge_types : Option<Vec<Option<String>>>, random_state : EdgeT, verbose : bool) -> PyResult<(Graph, Graph)> {
		pe!(self.graph.kfold(k, k_index, edge_types, random_state, verbose))
	}
	
}
