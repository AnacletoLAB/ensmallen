use super::*;
impl Graph {

	#[text_signature = "($self, random_state, unwanted_edge_types, verbose)"]
	/// TODO!: This binding was automatically generated
	fn random_spanning_arborescence_kruskal(&self, random_state : EdgeT, unwanted_edge_types : &Option<HashSet<Option<EdgeTypeT>>>, verbose : bool) -> (HashSet<(NodeT, NodeT)>, Vec<NodeT>, NodeT, NodeT, NodeT) {
		self.graph.random_spanning_arborescence_kruskal(random_state, unwanted_edge_types, verbose)
	}
	
	#[text_signature = "($self, verbose)"]
	/// TODO!: This binding was automatically generated
	/// Returns set of edges composing a spanning tree.
	/// 
	/// This is the implementaiton of [A Fast, Parallel Spanning Tree Algorithm for Symmetric Multiprocessors (SMPs)](https://smartech.gatech.edu/bitstream/handle/1853/14355/GT-CSE-06-01.pdf)
	/// by David A. Bader and Guojing Cong.
	fn spanning_arborescence_kruskal(&self, verbose : bool) -> (HashSet<(NodeT, NodeT)>, Vec<NodeT>, NodeT, NodeT, NodeT) {
		self.graph.spanning_arborescence_kruskal(verbose)
	}
	
	#[text_signature = "($self, other)"]
	/// TODO!: This binding was automatically generated
	/// Return graph remapped towards nodes of the given graph.
	/// 
	/// # Arguments
	/// 
	/// * other: &Graph - The graph to remap towards.
	/// * verbose: bool - whether to show a loding bar.
	/// 
	/// # Example
	/// A graph is always remappable to itself:
	/// ```rust
	/// # let graph = graph::test_utilities::load_ppi(true, true, true, true, false, false).unwrap();
	/// assert_eq!(graph, graph.remap(&graph, false).unwrap());
	/// ```
	/// 
	fn are_nodes_remappable(self, other : &Graph) -> bool {
		self.graph.are_nodes_remappable(other)
	}
	
	#[text_signature = "($self, node_id)"]
	/// TODO!: This binding was automatically generated
	/// Returns boolean representing if given node is a singleton.
	/// 
	/// # Arguments
	/// `node_name`: &str - The node name to be checked for.
	fn is_singleton_with_self_loops_by_node_id(self, node_id : NodeT) -> bool {
		self.graph.is_singleton_with_self_loops_by_node_id(node_id)
	}
	
	#[text_signature = "($self, src_name, dst_name)"]
	/// TODO!: This binding was automatically generated
	/// Return a roaringbitmap with the node ids to keep.
	/// 
	/// If both node\_names and node\_types are specified the result will be the
	/// union of both queries.
	/// 
	/// # Arguments
	/// * `node_names` - The nodes to keep as strings
	/// * `node_types` - The nodes types to keep as strings
	/// 
	fn has_edge_by_node_names(self, src_name : &str, dst_name : &str) -> bool {
		self.graph.has_edge_by_node_names(src_name, dst_name)
	}
	
	#[text_signature = "($self, src_name, dst_name)"]
	/// TODO!: This binding was automatically generated
	/// Return a roaringbitmap with the node ids to keep.
	/// 
	/// If both node\_names and node\_types are specified the result will be the
	/// union of both queries.
	/// 
	/// # Arguments
	/// * `node_names` - The nodes to keep as strings
	/// * `node_types` - The nodes types to keep as strings
	/// 
	fn has_edge_by_node_names(self, src_name : &str, dst_name : &str) -> bool {
		self.graph.has_edge_by_node_names(src_name, dst_name)
	}
	
	#[text_signature = "($self, edge_id)"]
	/// TODO!: This binding was automatically generated
	/// Return the src, dst, edge type of a given edge id
	fn get_edge_quadruple(&self, edge_id : EdgeT) -> (NodeT, NodeT, Option<EdgeTypeT>, Option<WeightT>) {
		self.graph.get_edge_quadruple(edge_id)
	}
	
	#[text_signature = "($self, edge_id)"]
	/// TODO!: This binding was automatically generated
	/// Return vector with top k central node Ids.
	/// 
	/// # Arguments
	/// 
	/// * k: NodeT - Number of central nodes to extract.
	fn get_edge_triple(self, edge_id : EdgeT) -> (NodeT, NodeT, Option<EdgeTypeT>) {
		self.graph.get_edge_triple(edge_id)
	}
	
	#[text_signature = "($self, node_id)"]
	/// TODO!: This binding was automatically generated
	/// Returns option with the edge type of the given edge id.
	/// TODO: complete docstring and add example!
	/// TODO: THIS SHOULD RETURN A RESULT!
	fn get_node_type_name_by_node_id(&self, node_id : NodeT) -> PyResult<Option<Vec<String>>> {
		pe!(self.graph.get_node_type_name_by_node_id(node_id))
	}
	
	#[text_signature = "($self, edge_id)"]
	/// TODO!: This binding was automatically generated
	/// Return edge type name of given edge type.
	/// 
	/// # Arguments
	/// * edge_type_id: EdgeTypeT - Id of the edge type.
	fn get_edge_type_name_by_edge_id(self, edge_id : EdgeT) -> PyResult<Option<String>> {
		pe!(self.graph.get_edge_type_name_by_edge_id(edge_id))
	}
	
	#[text_signature = "($self, edge_type_id)"]
	/// TODO!: This binding was automatically generated
	/// Returns weight of the given edge id.
	/// 
	/// # Arguments
	/// * `edge_id`: EdgeT - The edge ID whose weight is to be returned.
	/// 
	/// # Examples
	/// To get the weight of a given `edge_id` you can run:
	/// ```rust
	/// # let weighted_graph = graph::test_utilities::load_ppi(true, true, true, true, false, false).unwrap();
	/// # let unweighted_graph = graph::test_utilities::load_ppi(true, true, false, true, false, false).unwrap();
	/// let edge_id = 0;
	/// let unexistent_edge_id = 123456789;
	/// assert!(weighted_graph.get_weight_by_edge_id(edge_id).is_ok());
	/// assert!(weighted_graph.get_weight_by_edge_id(unexistent_edge_id).is_err());
	/// assert!(unweighted_graph.get_weight_by_edge_id(edge_id).is_err());
	/// ```
	fn get_edge_type_name_by_edge_type_id(&self, edge_type_id : EdgeTypeT) -> PyResult<String> {
		pe!(self.graph.get_edge_type_name_by_edge_type_id(edge_type_id))
	}
	
	#[text_signature = "($self, src, dst, edge_type)"]
	/// TODO!: This binding was automatically generated
	/// Returns weight of the given node names and edge type.
	/// 
	/// # Arguments
	/// * `src`: &str - The node name of the source node.
	/// * `dst`: &str - The node name of the destination node.
	/// * `edge_type`: Option<&String> - The edge type name of the edge.
	/// 
	/// # Examples
	/// To get the weight of a given `src` and `dst` and `edge_type` you can run:
	/// ```rust
	/// # let weighted_graph = graph::test_utilities::load_ppi(false, true, true, true, false, false).unwrap();
	/// let src = "ENSP00000000233";
	/// let dst = "ENSP00000432568";
	/// let edge_type = Some("red".to_string());
	/// assert!(weighted_graph.get_weight_with_type_by_node_names(src, dst, edge_type.as_ref()).is_ok());
	/// ```
	fn get_weight_with_type_by_node_ids(&self, src : NodeT, dst : NodeT, edge_type : Option<EdgeTypeT>) -> PyResult<WeightT> {
		pe!(self.graph.get_weight_with_type_by_node_ids(src, dst, edge_type))
	}
	
	#[text_signature = "($self, src, dst, edge_type)"]
	/// TODO!: This binding was automatically generated
	/// Returns weight of the given node names.
	/// 
	/// # Arguments
	/// * `src_name`: &str - The node name of the source node.
	/// * `dst_name`: &str - The node name of the destination node.
	/// 
	/// # Examples
	/// To get the weight of a given `src_name` and `dst_name` you can run:
	/// ```rust
	/// # let weighted_graph = graph::test_utilities::load_ppi(false, true, true, true, false, false).unwrap();
	/// let src_name = "ENSP00000000233";
	/// let dst_name = "ENSP00000432568";
	/// assert!(weighted_graph.get_weight_by_node_names(src_name, dst_name).is_ok());
	/// ```
	fn get_weight_with_type_by_node_names(&self, src : &str, dst : &str, edge_type : Option<&String>) -> PyResult<WeightT> {
		pe!(self.graph.get_weight_with_type_by_node_names(src, dst, edge_type))
	}
	
	#[text_signature = "($self, edge_type_name)"]
	/// TODO!: This binding was automatically generated
	/// TODO: add doc
	fn get_edge_type_id_by_edge_type_name(&self, edge_type_name : Option<&str>) -> PyResult<Option<EdgeTypeT>> {
		pe!(self.graph.get_edge_type_id_by_edge_type_name(edge_type_name))
	}
	
	#[text_signature = "($self, node_type_name)"]
	/// TODO!: This binding was automatically generated
	/// TODO: add doc
	fn get_node_type_id_by_node_type_name(&self, node_type_name : &str) -> PyResult<NodeTypeT> {
		pe!(self.graph.get_node_type_id_by_node_type_name(node_type_name))
	}
	
	#[text_signature = "($self, edge_id)"]
	/// TODO!: This binding was automatically generated
	/// Return vector of destinations for the given source node ID.
	/// 
	/// # Arguments
	/// 
	/// * `node_id`: NodeT - Node ID whose neighbours are to be retrieved.
	/// 
	/// # Example
	/// To retrieve the neighbours of a given node `src` you can use:
	/// 
	/// ```rust
	/// # let graph = graph::test_utilities::load_ppi(true, true, true, true, false, false).unwrap();
	/// # let node_id = 0;
	/// println!("The neighbours of the node {} are {:?}.", node_id, graph.get_node_neighbours_by_node_id(node_id).unwrap());
	/// let unavailable_node = 2349765432;
	/// assert!(graph.get_node_neighbours_by_node_id(unavailable_node).is_err());
	/// ```
	fn get_destination_node_id_by_edge_id(self, edge_id : EdgeT) -> PyResult<NodeT> {
		pe!(self.graph.get_destination_node_id_by_edge_id(edge_id))
	}
	
	#[text_signature = "($self, edge_types)"]
	/// TODO!: This binding was automatically generated
	/// Return translated node types from string to internal node ID.
	/// 
	/// # Arguments
	/// 
	/// * `node_types`: Vec<String> - Vector of node types to be converted.
	fn get_edge_type_ids_by_edge_type_names(&self, edge_types : Vec<Option<String>>) -> PyResult<Vec<Option<EdgeTypeT>>> {
		pe!(self.graph.get_edge_type_ids_by_edge_type_names(edge_types))
	}
	
	#[text_signature = "($self, node_types)"]
	/// TODO!: This binding was automatically generated
	/// Return range of outbound edges IDs for all the edges bewteen the given
	/// source and destination nodes.
	/// This operation is meaningfull only in a multigraph.
	/// 
	/// # Arguments
	/// 
	/// * src: NodeT - Source node.
	/// * dst: NodeT - Destination node.
	/// 
	fn get_node_type_ids_by_node_type_names(&self, node_types : Vec<Option<String>>) -> PyResult<Vec<Option<NodeTypeT>>> {
		pe!(self.graph.get_node_type_ids_by_node_type_names(node_types))
	}
	
	#[text_signature = "($self, src, dst)"]
	/// TODO!: This binding was automatically generated
	/// Return node type name of given node type.
	/// 
	/// There is no need for a unchecked version since we will have to map
	/// on the note_types anyway.
	/// 
	/// # Arguments
	/// * node_type_id: Vec<NodeTypeT> - Id of the node type.
	fn get_edge_ids_by_node_ids(&self, src : NodeT, dst : NodeT) -> Option<impl Iterator<Item = EdgeT>> {
		self.graph.get_edge_ids_by_node_ids(src, dst)
	}
	
	#[text_signature = "($self, node_type_id)"]
	/// TODO!: This binding was automatically generated
	/// Return node type name of given node type.
	/// 
	/// # Arguments
	/// * node_type_ids: Vec<NodeTypeT> - Id of the node type.
	fn get_node_type_name_by_node_type_id(&self, node_type_id : NodeTypeT) -> PyResult<String> {
		pe!(self.graph.get_node_type_name_by_node_type_id(node_type_id))
	}
	
	#[text_signature = "($self, node_type_ids)"]
	/// TODO!: This binding was automatically generated
	/// Returns the number of outbound neighbours of given node.
	/// 
	/// This is implemented as proposed by [S. Vigna here](http://vigna.di.unimi.it/ftp/papers/Broadword.pdf).
	/// 
	/// # Arguments
	/// 
	/// * `node_id` - Integer ID of the node.
	/// 
	fn get_node_type_names_by_node_type_ids(&self, node_type_ids : Vec<NodeTypeT>) -> PyResult<Vec<String>> {
		pe!(self.graph.get_node_type_names_by_node_type_ids(node_type_ids))
	}
	
	#[text_signature = "($self)"]
	/// TODO!: This binding was automatically generated
	/// Returns number of edges of the graph.
	/// ```rust
	/// # let graph = graph::test_utilities::load_ppi(true, true, true, true, false, false).unwrap();
	/// println!("The number of edges of the graph is  {}", graph.get_edges_number());
	/// ```
	fn get_unique_undirected_edges_number(self) -> EdgeT {
		self.graph.get_unique_undirected_edges_number()
	}
	
	#[text_signature = "($self)"]
	/// TODO!: This binding was automatically generated
	/// Returns median node degree of the graph
	/// ```rust
	/// # let graph = graph::test_utilities::load_ppi(true, true, true, true, false, false).unwrap();
	/// println!("The median node degree of the graph is  {}", graph.get_node_degrees_median().unwrap());
	/// ```
	fn get_unique_edges_number(self) -> EdgeT {
		self.graph.get_unique_edges_number()
	}
	
	#[text_signature = "($self)"]
	/// TODO!: This binding was automatically generated
	/// Returns number of unique self-loops, excluding those in eventual multi-edges.
	/// ```rust
	/// # let graph = graph::test_utilities::load_ppi(true, true, true, true, false, false).unwrap();
	/// println!("The number of unique self-loops in the graph is  {}", graph.get_unique_self_loop_number());
	/// ```
	fn get_self_loop_number(self) -> EdgeT {
		self.graph.get_self_loop_number()
	}
	
	#[text_signature = "($self)"]
	/// TODO!: This binding was automatically generated
	/// Returns rate of self-loops.
	/// ```rust
	/// # let graph = graph::test_utilities::load_ppi(true, true, true, true, false, false).unwrap();
	/// println!("The rate of self-loops in the graph is  {}", graph.get_self_loop_rate());
	/// ```
	fn get_unique_self_loop_number(self) -> NodeT {
		self.graph.get_unique_self_loop_number()
	}
	
	#[text_signature = "($self)"]
	/// TODO!: This binding was automatically generated
	/// Returns number a triple with (number of components, number of nodes of the smallest component, number of nodes of the biggest component )
	fn get_self_loop_rate(self) -> f64 {
		self.graph.get_self_loop_rate()
	}
	
	#[text_signature = "($self)"]
	/// TODO!: This binding was automatically generated
	/// Returns number of singleton nodes with self-loops within the graph.
	/// ```rust
	/// # let graph = graph::test_utilities::load_ppi(true, true, true, true, false, false).unwrap();
	/// println!("The graph contains {} singleton nodes with self-loops", graph.get_singleton_nodes_with_self_loops_number());
	/// ```
	fn get_singleton_nodes_number(self) -> NodeT {
		self.graph.get_singleton_nodes_number()
	}
	
	#[text_signature = "($self)"]
	/// TODO!: This binding was automatically generated
	/// Returns number of not singleton nodes within the graph.
	/// ```rust
	/// # let graph = graph::test_utilities::load_ppi(true, true, true, true, false, false).unwrap();
	/// println!("The graph contains {} not singleton nodes", graph.get_not_singleton_nodes_number());
	/// ```
	fn get_singleton_nodes_with_self_loops_number(self) -> NodeT {
		self.graph.get_singleton_nodes_with_self_loops_number()
	}
	
	#[text_signature = "($self)"]
	/// TODO!: This binding was automatically generated
	/// Returns density of the graph.
	/// ```rust
	/// # let graph = graph::test_utilities::load_ppi(true, true, true, true, false, false).unwrap();
	/// println!("The graph density is {}", graph.get_density().unwrap());
	/// ```
	fn get_not_singleton_nodes_number(self) -> NodeT {
		self.graph.get_not_singleton_nodes_number()
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
	/// 
	/// ```rust
	/// # let graph = graph::test_utilities::load_ppi(true, true, true, true, false, false).unwrap();
	/// graph.report();
	/// ```
	fn get_density(self) -> PyResult<f64> {
		pe!(self.graph.get_density())
	}
	
	#[text_signature = "($self, verbose)"]
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
	/// # Arguments
	/// * `allow_nodes_set`: Option<HashSet<String>> - Optional set of nodes names to keep.
	/// * `deny_nodes_set`: Option<HashSet<String>> - Optional set of nodes names to remove.
	/// * `allow_node_types_set`: Option<HashSet<String>> - Optional set of node type names to keep.
	/// * `deny_node_types_set`: Option<HashSet<String>> - Optional set of node type names to remove.
	/// * `allow_edge_set`: Option<HashSet<EdgeT>>- Optional set of numeric edge IDs to keep.
	/// * `deny_edge_set`: Option<HashSet<EdgeT>>- Optional set of numeric edge IDs to remove.
	/// * `allow_edge_types_set`: Option<HashSet<String>> - Optional set of edge type names to keep.
	/// * `deny_edge_types_set`: Option<HashSet<String>> - Optional set of edge type names to remove.
	/// * `weights`: bool - whether to remove the weights.
	/// * `node_types`: bool - whether to remove the node types.
	/// * `edge_types`: bool - whether to remove the edge types.
	/// * `singletons`: bool - whether to remove the singleton nodes.
	/// * `selfloops`: bool - whether to remove edges with self-loops.
	/// * `verbose`: bool - whether to show a loading bar while building the graph.
	/// 
	fn textual_report(self, verbose : bool) -> PyResult<String> {
		pe!(self.graph.textual_report(verbose))
	}
	
	#[text_signature = "($self, verbose)"]
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
	/// # Arguments
	/// * `allow_nodes_set`: Option<HashSet<String>> - Optional set of nodes names to keep.
	/// * `deny_nodes_set`: Option<HashSet<String>> - Optional set of nodes names to remove.
	/// * `allow_node_types_set`: Option<HashSet<String>> - Optional set of node type names to keep.
	/// * `deny_node_types_set`: Option<HashSet<String>> - Optional set of node type names to remove.
	/// * `allow_edge_set`: Option<HashSet<EdgeT>>- Optional set of numeric edge IDs to keep.
	/// * `deny_edge_set`: Option<HashSet<EdgeT>>- Optional set of numeric edge IDs to remove.
	/// * `allow_edge_types_set`: Option<HashSet<String>> - Optional set of edge type names to keep.
	/// * `deny_edge_types_set`: Option<HashSet<String>> - Optional set of edge type names to remove.
	/// * `weights`: bool - whether to remove the weights.
	/// * `node_types`: bool - whether to remove the node types.
	/// * `edge_types`: bool - whether to remove the edge types.
	/// * `singletons`: bool - whether to remove the singleton nodes.
	/// * `selfloops`: bool - whether to remove edges with self-loops.
	/// * `verbose`: bool - whether to show a loading bar while building the graph.
	/// 
	fn textual_report(self, verbose : bool) -> PyResult<String> {
		pe!(self.graph.textual_report(verbose))
	}
	
	#[text_signature = "($self, src, dst)"]
	/// TODO!: This binding was automatically generated
	fn encode_edge(self, src : NodeT, dst : NodeT) -> u64 {
		self.graph.encode_edge(src, dst)
	}
	
	#[text_signature = "($self, edge)"]
	/// TODO!: This binding was automatically generated
	fn decode_edge(self, edge : u64) -> (NodeT, NodeT) {
		self.graph.decode_edge(edge)
	}
	
	#[text_signature = "($self)"]
	/// TODO!: This binding was automatically generated
	/// Return if the graph has any edges.
	/// 
	/// # Example
	/// To check if the current graph has edges you can use:
	/// ```rust
	/// # let graph = graph::test_utilities::load_ppi(true, true, true, true, false, false).unwrap();
	/// assert_eq!(graph.has_edges(), true);
	/// ```
	/// 
	fn has_nodes(self) -> bool {
		self.graph.has_nodes()
	}
	
	#[text_signature = "($self)"]
	/// TODO!: This binding was automatically generated
	/// Return name of the graph.
	/// 
	/// # Example
	/// To the retrieve the name of the current graph instance you can use:
	/// ```rust
	/// # let graph = graph::test_utilities::load_ppi(true, true, true, true, false, false).unwrap();
	/// assert_eq!(graph.get_name(), "STRING PPI".to_string());
	/// println!("The name of the current graph is {}.", graph.get_name());
	/// ```
	/// 
	fn has_edges(self) -> bool {
		self.graph.has_edges()
	}
	
	#[text_signature = "($self)"]
	/// TODO!: This binding was automatically generated
	/// 
	/// # Example
	/// ```rust
	/// # let graph = graph::test_utilities::load_ppi(true, true, true, true, false, false).unwrap();
	/// if graph.has_trap_nodes(){
	/// println!("There are {} trap nodes in the current graph.", graph.get_trap_nodes_number());
	/// } else {
	/// println!("There are no trap nodes in the current graph.");
	/// }
	/// ```
	/// 
	fn get_trap_nodes_number(self) -> EdgeT {
		self.graph.get_trap_nodes_number()
	}
	
	#[text_signature = "($self)"]
	/// TODO!: This binding was automatically generated
	/// Returns boolean representing if graph is directed.
	/// 
	/// # Example
	/// ```rust
	/// let directed_string_ppi = graph::test_utilities::load_ppi(true, true, true, true, false, false).unwrap();
	/// assert!(directed_string_ppi.is_directed());
	/// let undirected_string_ppi = graph::test_utilities::load_ppi(true, true, true, false, false, false).unwrap();
	/// assert!(!undirected_string_ppi.is_directed());
	/// ```
	/// 
	fn has_trap_nodes(self) -> bool {
		self.graph.has_trap_nodes()
	}
	
	#[text_signature = "($self)"]
	/// TODO!: This binding was automatically generated
	/// Returns boolean representing whether graph has weights.
	/// 
	/// # Example
	/// ```rust
	/// let weights_string_ppi = graph::test_utilities::load_ppi(true, true, true, true, false, false).unwrap();
	/// assert!(weights_string_ppi.has_weights());
	/// let unweights_string_ppi = graph::test_utilities::load_ppi(true, true, false, true, false, false).unwrap();
	/// assert!(!unweights_string_ppi.has_weights());
	/// ```
	/// 
	fn is_directed(self) -> bool {
		self.graph.is_directed()
	}
	
	#[text_signature = "($self)"]
	/// TODO!: This binding was automatically generated
	/// Returns boolean representing if graph has singletons.
	fn has_singletons(self) -> bool {
		self.graph.has_singletons()
	}
	
	#[text_signature = "($self)"]
	/// TODO!: This binding was automatically generated
	/// Return vector of the non-unique source nodes.
	/// 
	/// # Arguments
	/// * `directed`: bool, whether to filter out the undirected edges.
	fn has_singleton_nodes_with_self_loops(self) -> bool {
		self.graph.has_singleton_nodes_with_self_loops()
	}
	
	#[text_signature = "($self)"]
	/// TODO!: This binding was automatically generated
	/// Return the maximum weight, if graph has weights.
	fn get_min_weight(self) -> PyResult<WeightT> {
		pe!(self.graph.get_min_weight())
	}
	
	#[text_signature = "($self)"]
	/// TODO!: This binding was automatically generated
	/// Return the node types names.
	fn get_max_weight(self) -> PyResult<WeightT> {
		pe!(self.graph.get_max_weight())
	}
	
	#[text_signature = "($self)"]
	/// TODO!: This binding was automatically generated
	/// Return maximum encodable edge number.
	fn get_unique_directed_edges_number(self) -> EdgeT {
		self.graph.get_unique_directed_edges_number()
	}
	
	#[text_signature = "($self)"]
	/// TODO!: This binding was automatically generated
	/// Return the nodes mapping.
	fn get_max_encodable_edge_number(self) -> EdgeT {
		self.graph.get_max_encodable_edge_number()
	}
	
	#[text_signature = "($self)"]
	/// TODO!: This binding was automatically generated
	/// Returns number of unknown node types.
	fn has_multilabel_node_types(self) -> bool {
		self.graph.has_multilabel_node_types()
	}
	
	#[text_signature = "($self)"]
	/// TODO!: This binding was automatically generated
	/// Returns minimum number of node types.
	fn get_unknown_node_types_number(self) -> NodeT {
		self.graph.get_unknown_node_types_number()
	}
	
	#[text_signature = "($self)"]
	/// TODO!: This binding was automatically generated
	/// Returns whether there are unknown node types.
	fn get_minimum_node_types_number(self) -> NodeT {
		self.graph.get_minimum_node_types_number()
	}
	
	#[text_signature = "($self)"]
	/// TODO!: This binding was automatically generated
	/// Returns number of unknown edge types.
	fn has_unknown_node_types(self) -> bool {
		self.graph.has_unknown_node_types()
	}
	
	#[text_signature = "($self)"]
	/// TODO!: This binding was automatically generated
	/// Returns minimum number of edge types.
	fn get_unknown_edge_types_number(self) -> EdgeT {
		self.graph.get_unknown_edge_types_number()
	}
	
	#[text_signature = "($self)"]
	/// TODO!: This binding was automatically generated
	/// Returns whether there are unknown edge types.
	fn get_minimum_edge_types_number(self) -> EdgeT {
		self.graph.get_minimum_edge_types_number()
	}
	
	#[text_signature = "($self)"]
	/// TODO!: This binding was automatically generated
	/// Returns number of nodes in the graph.
	fn has_unknown_edge_types(self) -> bool {
		self.graph.has_unknown_edge_types()
	}
	
	#[text_signature = "($self, verbose)"]
	/// TODO!: This binding was automatically generated
	/// Returns number of directed edges in the graph.
	fn get_node_components_vector(self, verbose : bool) -> Vec<NodeT> {
		self.graph.get_node_components_vector(verbose)
	}
	
	#[text_signature = "($self)"]
	/// TODO!: This binding was automatically generated
	/// Returns number of edge types in the graph.
	fn get_directed_edges_number(self) -> EdgeT {
		self.graph.get_directed_edges_number()
	}
	
	#[text_signature = "($self)"]
	/// TODO!: This binding was automatically generated
	/// Return set of nodes that are not singletons.
	fn get_node_degrees(self) -> Vec<NodeT> {
		self.graph.get_node_degrees()
	}
	
	#[text_signature = "($self)"]
	/// TODO!: This binding was automatically generated
	/// Return mapping from instance not trap nodes to dense nodes.
	fn get_not_singletons(self) -> Vec<NodeT> {
		self.graph.get_not_singletons()
	}
	
	#[text_signature = "($self)"]
	/// TODO!: This binding was automatically generated
	/// Return number of edges that have multigraph syblings.
	fn is_multigraph(self) -> bool {
		self.graph.is_multigraph()
	}
	
	#[text_signature = "($self)"]
	/// TODO!: This binding was automatically generated
	/// Return vector with node degrees
	fn get_multigraph_edges_number(self) -> EdgeT {
		self.graph.get_multigraph_edges_number()
	}
	
	#[text_signature = "($self)"]
	/// TODO!: This binding was automatically generated
	/// Returns number of the source nodes.
	/// ```rust
	/// # let graph = graph::test_utilities::load_ppi(true, true, true, true, false, false).unwrap();
	/// println!("The number of sources of the graph (not trap nodes) is {}", graph.get_unique_source_nodes_number());
	/// ```
	fn get_outbounds(self) -> Vec<EdgeT> {
		self.graph.get_outbounds()
	}
	
	#[text_signature = "($self)"]
	/// TODO!: This binding was automatically generated
	/// Returns node type counts.
	/// 
	/// # Arguments
	/// 
	/// None
	/// 
	/// ```rust
	/// # let graph = graph::test_utilities::load_ppi(true, true, true, true, false, false).unwrap();
	/// for (node_type_id, count) in graph.get_node_type_counts().unwrap().iter() {
	/// println!("node type id {}: count: {}", node_type_id, count);
	/// }
	/// ```
	fn get_edge_type_counts_hashmap(self) -> PyResult<HashMap<EdgeTypeT, usize>> {
		pe!(self.graph.get_edge_type_counts_hashmap())
	}
	
	#[text_signature = "($self)"]
	/// TODO!: This binding was automatically generated
	/// Returns range of the edge ids of edges starting from the given source node.
	/// 
	/// # Arguments
	/// 
	/// * `src` - Source node of the edge.
	/// 
	fn get_node_type_counts_hashmap(self) -> PyResult<HashMap<EdgeTypeT, usize>> {
		pe!(self.graph.get_node_type_counts_hashmap())
	}
	
	#[text_signature = "($self)"]
	/// TODO!: This binding was automatically generated
	/// Returns range of the edge ids of edges starting from the given source node.
	/// 
	/// # Arguments
	/// 
	/// * `src` - Source node of the edge.
	/// 
	fn get_node_type_counts_hashmap(self) -> PyResult<HashMap<EdgeTypeT, usize>> {
		pe!(self.graph.get_node_type_counts_hashmap())
	}
	
	#[text_signature = "($self)"]
	/// TODO!: This binding was automatically generated
	fn compute_hash(self) -> u64 {
		self.graph.compute_hash()
	}
	
	#[text_signature = "($self)"]
	/// TODO!: This binding was automatically generated
	fn compute_hash(self) -> u64 {
		self.graph.compute_hash()
	}
	
	#[text_signature = "($self, nodes_iterator, directed, directed_edge_list, name, ignore_duplicated_nodes, node_list_is_correct, ignore_duplicated_edges, edge_list_is_correct, verbose, numeric_edge_type_ids, numeric_node_ids, numeric_edge_node_ids, numeric_node_types_ids, has_node_types, has_edge_types, has_weights, might_have_singletons, might_have_singletons_with_selfloops, might_have_trap_nodes)"]
	/// TODO!: This binding was automatically generated
	/// Create new Graph object from unsorted source.
	/// 
	/// # Arguments
	/// 
	/// * edges_iterator: impl Iterator<Item = Result<StringQuadruple, String>>,
	/// Iterator of the edges.
	/// * nodes_iterator: Option<impl Iterator<Item = Result<(String, Option<String>), String>>>,
	/// Iterator of the nodes.
	/// * directed: bool,
	/// Wether the graph should be directed or undirected.
	/// * ignore_duplicated_nodes: bool,
	/// Wether to ignore duplicated nodes or to raise a proper exception.
	/// * ignore_duplicated_edges: bool,
	/// Wether to ignore duplicated edges or to raise a proper exception.
	/// * skip_self_loops: bool,
	/// Wether to skip self loops while reading the the edges iterator.
	fn from_string_unsorted(impl Iterator<Item = Result<StringQuadruple, String>>, nodes_iterator : Option<impl Iterator<Item = Result<(String, Option<Vec<String>>), String>>>, directed : bool, directed_edge_list : bool, name : S, ignore_duplicated_nodes : bool, node_list_is_correct : bool, ignore_duplicated_edges : bool, edge_list_is_correct : bool, verbose : bool, numeric_edge_type_ids : bool, numeric_node_ids : bool, numeric_edge_node_ids : bool, numeric_node_types_ids : bool, has_node_types : bool, has_edge_types : bool, has_weights : bool, might_have_singletons : bool, might_have_singletons_with_selfloops : bool, might_have_trap_nodes : bool) -> PyResult<Graph> {
		pe!(self.graph.from_string_unsorted(nodes_iterator, directed, directed_edge_list, name, ignore_duplicated_nodes, node_list_is_correct, ignore_duplicated_edges, edge_list_is_correct, verbose, numeric_edge_type_ids, numeric_node_ids, numeric_edge_node_ids, numeric_node_types_ids, has_node_types, has_edge_types, has_weights, might_have_singletons, might_have_singletons_with_selfloops, might_have_trap_nodes))
	}
	
	#[text_signature = "($self, nodes, node_types, edge_types_vocabulary, directed, name, ignore_duplicated_edges, has_edge_types, has_weights, verbose, might_have_singletons, might_have_singletons_with_selfloops, might_have_trap_nodes)"]
	/// TODO!: This binding was automatically generated
	/// Create new Graph object from sorted sources.
	fn from_integer_unsorted(impl Iterator<
	            Item = Result<(NodeT, NodeT, Option<NodeTypeT>, Option<WeightT>), String>,
	        >, nodes : Vocabulary<NodeT>, node_types : Option<NodeTypeVocabulary>, edge_types_vocabulary : Option<Vocabulary<EdgeTypeT>>, directed : bool, name : String, ignore_duplicated_edges : bool, has_edge_types : bool, has_weights : bool, verbose : bool, might_have_singletons : bool, might_have_singletons_with_selfloops : bool, might_have_trap_nodes : bool) -> PyResult<Graph> {
		pe!(self.graph.from_integer_unsorted(nodes, node_types, edge_types_vocabulary, directed, name, ignore_duplicated_edges, has_edge_types, has_weights, verbose, might_have_singletons, might_have_singletons_with_selfloops, might_have_trap_nodes))
	}
	
	#[text_signature = "($self, nodes_iterator, directed, directed_edge_list, ignore_duplicated_nodes, node_list_is_correct, ignore_duplicated_edges, edge_list_is_correct, edges_number, nodes_number, numeric_edge_type_ids, numeric_node_ids, numeric_edge_node_ids, numeric_node_types_ids, has_node_types, has_edge_types, has_weights, might_have_singletons, might_have_singletons_with_selfloops, might_have_trap_nodes, name)"]
	/// TODO!: This binding was automatically generated
	/// Returns Graph with given amount of negative edges as positive edges.
	/// 
	/// The graph generated may be used as a testing negatives partition to be
	/// fed into the argument "graph_to_avoid" of the link_prediction or the
	/// skipgrams algorithm.
	/// 
	/// 
	/// # Arguments
	/// 
	/// * `random_state`: EdgeT - random_state to use to reproduce negative edge set.
	/// * `negatives_number`: EdgeT - Number of negatives edges to include.
	/// * `seed_graph`: Option<Graph> - Optional graph to use to filter the negative edges. The negative edges generated when this variable is provided will always have a node within this graph.
	/// * `only_from_same_component`: bool - whether to sample negative edges only from nodes that are from the same component.
	/// * `verbose`: bool - whether to show the loading bar.
	/// 
	fn from_string_sorted(impl Iterator<Item = Result<StringQuadruple, String>>, nodes_iterator : Option<impl Iterator<Item = Result<(String, Option<Vec<String>>), String>>>, directed : bool, directed_edge_list : bool, ignore_duplicated_nodes : bool, node_list_is_correct : bool, ignore_duplicated_edges : bool, edge_list_is_correct : bool, edges_number : usize, nodes_number : NodeT, numeric_edge_type_ids : bool, numeric_node_ids : bool, numeric_edge_node_ids : bool, numeric_node_types_ids : bool, has_node_types : bool, has_edge_types : bool, has_weights : bool, might_have_singletons : bool, might_have_singletons_with_selfloops : bool, might_have_trap_nodes : bool, name : S) -> PyResult<Graph> {
		pe!(self.graph.from_string_sorted(nodes_iterator, directed, directed_edge_list, ignore_duplicated_nodes, node_list_is_correct, ignore_duplicated_edges, edge_list_is_correct, edges_number, nodes_number, numeric_edge_type_ids, numeric_node_ids, numeric_edge_node_ids, numeric_node_types_ids, has_node_types, has_edge_types, has_weights, might_have_singletons, might_have_singletons_with_selfloops, might_have_trap_nodes, name))
	}
	
	#[text_signature = "($self, nodes_iterator, directed, directed_edge_list, ignore_duplicated_nodes, node_list_is_correct, ignore_duplicated_edges, edge_list_is_correct, edges_number, nodes_number, numeric_edge_type_ids, numeric_node_ids, numeric_edge_node_ids, numeric_node_types_ids, has_node_types, has_edge_types, has_weights, might_have_singletons, might_have_singletons_with_selfloops, might_have_trap_nodes, name)"]
	/// TODO!: This binding was automatically generated
	/// Returns Graph with given amount of negative edges as positive edges.
	/// 
	/// The graph generated may be used as a testing negatives partition to be
	/// fed into the argument "graph_to_avoid" of the link_prediction or the
	/// skipgrams algorithm.
	/// 
	/// 
	/// # Arguments
	/// 
	/// * `random_state`: EdgeT - random_state to use to reproduce negative edge set.
	/// * `negatives_number`: EdgeT - Number of negatives edges to include.
	/// * `seed_graph`: Option<Graph> - Optional graph to use to filter the negative edges. The negative edges generated when this variable is provided will always have a node within this graph.
	/// * `only_from_same_component`: bool - whether to sample negative edges only from nodes that are from the same component.
	/// * `verbose`: bool - whether to show the loading bar.
	/// 
	fn from_string_sorted(impl Iterator<Item = Result<StringQuadruple, String>>, nodes_iterator : Option<impl Iterator<Item = Result<(String, Option<Vec<String>>), String>>>, directed : bool, directed_edge_list : bool, ignore_duplicated_nodes : bool, node_list_is_correct : bool, ignore_duplicated_edges : bool, edge_list_is_correct : bool, edges_number : usize, nodes_number : NodeT, numeric_edge_type_ids : bool, numeric_node_ids : bool, numeric_edge_node_ids : bool, numeric_node_types_ids : bool, has_node_types : bool, has_edge_types : bool, has_weights : bool, might_have_singletons : bool, might_have_singletons_with_selfloops : bool, might_have_trap_nodes : bool, name : S) -> PyResult<Graph> {
		pe!(self.graph.from_string_sorted(nodes_iterator, directed, directed_edge_list, ignore_duplicated_nodes, node_list_is_correct, ignore_duplicated_edges, edge_list_is_correct, edges_number, nodes_number, numeric_edge_type_ids, numeric_node_ids, numeric_edge_node_ids, numeric_node_types_ids, has_node_types, has_edge_types, has_weights, might_have_singletons, might_have_singletons_with_selfloops, might_have_trap_nodes, name))
	}
	
}
