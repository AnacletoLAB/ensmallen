use super::*;
impl Graph {

	#[text_signature = "($self, verbose)"]
	/// Returns new graph without singleton nodes.
	/// 
	///  A node is singleton when does not have neither incoming or outgoing edges.
	/// 
	/// Parameters
	/// --------------
	/// verbose: bool,
	/// 	Whether to show a loading bar while building the graph.
	///
	/// [Automatically generated binding]
	/// [Automatically generated documentation]
	fn drop_singleton_nodes(&self, verbose : bool) -> EnsmallenGraph {
		EnsmallenGraph{graph:self.graph.drop_singleton_nodes(verbose)}
	}
	
	#[text_signature = "($self, verbose)"]
	/// Returns new graph without singleton nodes with selfloops.
	/// 
	///  A node is singleton with selfloop when does not have neither incoming or outgoing edges.
	/// 
	/// Parameters
	/// --------------
	/// verbose: bool,
	/// 	Whether to show a loading bar while building the graph.
	///
	/// [Automatically generated binding]
	/// [Automatically generated documentation]
	fn drop_singleton_nodes_with_selfloops(&self, verbose : bool) -> EnsmallenGraph {
		EnsmallenGraph{graph:self.graph.drop_singleton_nodes_with_selfloops(verbose)}
	}
	
	#[text_signature = "($self, verbose)"]
	/// Returns new graph without unknown edge types and relative edges.
	/// 
	///  Note that this method will remove ALL edges labeled with unknown edge
	///  type!
	/// 
	/// Parameters
	/// --------------
	/// verbose: bool,
	/// 	Whether to show a loading bar while building the graph.
	///
	/// [Automatically generated binding]
	/// [Automatically generated documentation]
	fn drop_unknown_edge_types(&self, verbose : bool) -> EnsmallenGraph {
		EnsmallenGraph{graph:self.graph.drop_unknown_edge_types(verbose)}
	}
	
	#[text_signature = "($self, verbose)"]
	/// Returns new graph without unknown node types and relative nodes.
	/// 
	///  Note that this method will remove ALL nodes labeled with unknown node
	///  type!
	/// 
	/// Parameters
	/// --------------
	/// verbose: bool,
	/// 	Whether to show a loading bar while building the graph.
	///
	/// [Automatically generated binding]
	/// [Automatically generated documentation]
	fn drop_unknown_node_types(&self, verbose : bool) -> EnsmallenGraph {
		EnsmallenGraph{graph:self.graph.drop_unknown_node_types(verbose)}
	}
	
}
