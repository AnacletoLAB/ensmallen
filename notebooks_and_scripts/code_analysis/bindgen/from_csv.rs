use super::*;
impl Graph {

	#[text_signature = "($self, node_file_reader, directed, directed_edge_list, edges_number, nodes_number, name)"]
	/// Return graph renderized from given files.
	/// 
	/// Paramenters
	/// --------------
	/// edge_file_reader : EdgeFileReader,
	/// 	Reader of the edge file.
	/// node_file_reader : NodeFileReader,
	/// 	Reader of the node file.
	/// directed : bool,
	/// 	Whether the graph is to be read as directed or undirected.
	/// directed_edge_list : bool,
	/// 	Whether to read the edge list as directed.
	/// edges_number : int,
	/// 	Number of edges of the graph.
	/// nodes_number : int,
	/// 	Number of the nodes of the graph.
	/// name : S,
	/// 	Name of the graph.
	///
	/// [Automatically generated binding]
	fn from_sorted_csv(EdgeFileReader, node_file_reader : Option<NodeFileReader>, directed : bool, directed_edge_list : bool, edges_number : usize, nodes_number : NodeT, name : S) -> PyResult<Graph> {
		pe!(self.graph.from_sorted_csv(node_file_reader, directed, directed_edge_list, edges_number, nodes_number, name))
	}
	
	#[text_signature = "($self, node_file_reader, directed, directed_edge_list, name)"]
	/// Return graph renderized from given files.
	/// 
	/// Paramenters
	/// --------------
	/// edge_file_reader : EdgeFileReader,
	/// 	Reader of the edge file.
	/// node_file_reader : NodeFileReader,
	/// 	Reader of the node file.
	/// directed : bool,
	/// 	Whether the graph is to be read as directed or undirected.
	/// directed_edge_list : bool,
	/// 	Whether to read the edge list as directed.
	/// name : S,
	/// 	The name for the graph.
	///
	/// [Automatically generated binding]
	fn from_unsorted_csv(EdgeFileReader, node_file_reader : Option<NodeFileReader>, directed : bool, directed_edge_list : bool, name : S) -> PyResult<Graph> {
		pe!(self.graph.from_unsorted_csv(node_file_reader, directed, directed_edge_list, name))
	}
	
}
