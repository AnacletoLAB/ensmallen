use super::*;
impl Graph {

	#[text_signature = "($self, node_name_mapping, node_type_name_mapping, node_type_names_mapping, edge_type_name_mapping, verbose)"]
	/// Replace given node, node type and edge type names.
	/// 
	/// Parameters
	/// --------------
	/// node_name_mapping: Union[HashMap<str, str, None]>,
	/// 	The node names to replace.
	/// node_type_name_mapping: Union[HashMap<str, str, None]>,
	/// 	The node type names to replace.
	/// node_type_names_mapping: Union[HashMap<Option<List[str], None], Union[List[str], None]>>,
	/// 	The node type names (as vectors) to replace.
	/// edge_type_name_mapping: Union[HashMap<Option<str, None], Union[str, None]>>,
	/// 	The edge type names to replace.
	/// verbose: bool,
	/// 	Whether to show a loading bar.
	/// 
	///  # Raises
	///  * If the given node names mapping would lead to nodes duplication.
	///
	/// [Automatically generated binding]
	/// [Automatically generated documentation]
	fn replace(&self, node_name_mapping : Option<HashMap<String, String>>, node_type_name_mapping : Option<HashMap<String, String>>, node_type_names_mapping : Option<HashMap<Option<Vec<String>>, Option<Vec<String>>>>, edge_type_name_mapping : Option<HashMap<Option<String>, Option<String>>>, verbose : bool) -> PyResult<EnsmallenGraph> {
		Ok(EnsmallenGraph{graph:pe!(self.graph.replace(node_name_mapping, node_type_name_mapping, node_type_names_mapping, edge_type_name_mapping, verbose))})
	}
	
	#[text_signature = "($self, edge_type_name, verbose)"]
	/// Replace unknown edge types with given edge type name.
	/// 
	/// Parameters
	/// --------------
	/// edge_type_name: str,
	/// 	The edge type name to replace the unknown with.
	/// verbose: bool,
	/// 	Whether to show a loading bar.
	///
	/// [Automatically generated binding]
	/// [Automatically generated documentation]
	fn replace_unknown_edge_types_with_edge_type_name(&self, edge_type_name : String, verbose : bool) -> PyResult<EnsmallenGraph> {
		Ok(EnsmallenGraph{graph:pe!(self.graph.replace_unknown_edge_types_with_edge_type_name(edge_type_name, verbose))})
	}
	
	#[text_signature = "($self, node_type_names, verbose)"]
	/// Replace unknown node types with given node type.
	/// 
	/// Parameters
	/// --------------
	/// node_type_names: List[str],
	/// 	The node types to replace the unknown with.
	/// verbose: bool,
	/// 	Whether to show a loading bar.
	///
	/// [Automatically generated binding]
	/// [Automatically generated documentation]
	fn replace_unknown_node_types_with_node_type_name(&self, node_type_names : Vec<String>, verbose : bool) -> PyResult<EnsmallenGraph> {
		Ok(EnsmallenGraph{graph:pe!(self.graph.replace_unknown_node_types_with_node_type_name(node_type_names, verbose))})
	}
	
}
