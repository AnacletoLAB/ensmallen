use super::*;
impl Graph {

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
	/// random_state : int,
	/// 	random_state to use to reproduce negative edge set.
	/// negatives_number : int,
	/// 	Number of negatives edges to include.
	/// seed_graph : Graph,
	/// 	Optional graph to use to filter the negative edges. The negative edges generated when this variable is provided will always have a node within this graph.
	/// only_from_same_component : bool,
	/// 	Whether to sample negative edges only from nodes that are from the same component.
	/// verbose : bool,
	/// 	Whether to show the loading bar.
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
	/// random_state : int,
	/// 	The random_state to use for the holdout,
	/// train_size : float,
	/// 	Rate target to reserve for training.
	/// edge_types : List[Option<str]>,
	/// 	Edge types to be selected for in the validation set.
	/// include_all_edge_types : bool,
	/// 	Whether to include all the edges between two nodes.
	/// verbose : bool,
	/// 	Whether to show the loading bar.
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
	/// random_state : int,
	/// 	The random_state to use for the holdout,
	/// train_size : float,
	/// 	rate target to reserve for training
	/// include_all_edge_types : bool,
	/// 	Whether to include all the edges between two nodes.
	/// edge_types : List[Option<str]>,
	/// 	The edges to include in validation set.
	/// min_number_overlaps : int,
	/// 	The minimum number of overlaps to include the edge into the validation set.
	/// verbose : bool,
	/// 	Whether to show the loading bar.
	fn random_holdout(&self, random_state : EdgeT, train_size : f64, include_all_edge_types : bool, edge_types : Option<Vec<Option<String>>>, min_number_overlaps : Option<EdgeT>, verbose : bool) -> PyResult<(Graph, Graph)> {
		pe!(self.graph.random_holdout(random_state, train_size, include_all_edge_types, edge_types, min_number_overlaps, verbose))
	}
	
	#[text_signature = "($self, train_size, use_stratification, random_state)"]
	/// TODO!: This binding was automatically generated
	/// Returns node-label holdout for training ML algorithms on the graph node labels.
	/// 
	/// Paramenters
	/// --------------
	/// train_size : float,
	/// 	rate target to reserve for training,
	/// use_stratification : bool,
	/// 	Whether to use node-label stratification,
	/// random_state : int,
	/// 	The random_state to use for the holdout,
	fn node_label_holdout(&self, train_size : f64, use_stratification : bool, random_state : EdgeT) -> PyResult<(Graph, Graph)> {
		pe!(self.graph.node_label_holdout(train_size, use_stratification, random_state))
	}
	
	#[text_signature = "($self, train_size, use_stratification, random_state)"]
	/// TODO!: This binding was automatically generated
	/// Returns edge-label holdout for training ML algorithms on the graph edge labels.
	/// This is commonly used for edge type prediction tasks.
	/// 
	/// This method returns two graphs, the train and the test one.
	/// The edges of the graph will be splitted in the train and test graphs according
	/// to the `train_size` argument.
	/// 
	/// If stratification is enabled, the train and test will have the same ratios of
	/// edge types.
	/// 
	/// Paramenters
	/// --------------
	/// train_size : float,
	/// 	rate target to reserve for training,
	/// use_stratification : bool,
	/// 	Whether to use edge-label stratification,
	/// random_state : int,
	/// 	The random_state to use for the holdout,
	fn edge_label_holdout(&self, train_size : f64, use_stratification : bool, random_state : EdgeT) -> PyResult<(Graph, Graph)> {
		pe!(self.graph.edge_label_holdout(train_size, use_stratification, random_state))
	}
	
	#[text_signature = "($self, random_state, nodes_number, verbose)"]
	/// TODO!: This binding was automatically generated
	/// Returns subgraph with given number of nodes.
	/// 
	/// **This method creates a subset of the graph starting from a random node
	/// sampled using given random_state and includes all neighbouring nodes until
	/// the required number of nodes is reached**. All the edges connecting any
	/// of the selected nodes are then inserted into this graph.
	/// 
	/// This is meant to execute distributed node embeddings.
	/// It may also sample singleton nodes.
	/// 
	/// Paramenters
	/// --------------
	/// random_state : int,
	/// 	Random random_state to use.
	/// nodes_number : int,
	/// 	Number of nodes to extract.
	/// verbose : bool,
	/// 	Whether to show the loading bar.
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
	/// k : int,
	/// 	The number of folds.
	/// k_index : int,
	/// 	Which fold to use for the validation.
	/// edge_types : List[Option<str]>,
	/// 	Edge types to be selected when computing the folds (All the edge types not listed here will be always be used in the training set).
	/// random_state : int,
	/// 	The random_state (seed) to use for the holdout,
	/// verbose : bool,
	/// 	Whether to show the loading bar.
	fn kfold(&self, k : EdgeT, k_index : u64, edge_types : Option<Vec<Option<String>>>, random_state : EdgeT, verbose : bool) -> PyResult<(Graph, Graph)> {
		pe!(self.graph.kfold(k, k_index, edge_types, random_state, verbose))
	}
	
}
