use super::*;
impl Graph {

	#[text_signature = "($self, node_names, node_types, edge_types, min_weight, max_weight, verbose)"]
	/// TODO!: This binding was automatically generated
	/// Return graph filtered by given weights range.
	/// 
	/// Paramenters
	/// --------------
	/// node_names : List[str],
	/// 	The node names to keep.
	/// node_types : List[Option<str]>,
	/// 	The node types to keep.
	/// edge_types : List[Option<str]>,
	/// 	The edge types to keep.
	/// min_weight : WeightT,
	/// 	Minimum weight to use to filter edges.
	/// max_weight : WeightT,
	/// 	Maximum weight to use to filter edges.
	/// verbose : bool,
	/// 	Whether to show the loading bar.
	fn filter(&self, node_names : Option<Vec<String>>, node_types : Option<Vec<Option<String>>>, edge_types : Option<Vec<Option<String>>>, min_weight : Option<WeightT>, max_weight : Option<WeightT>, verbose : bool) -> PyResult<Graph> {
		pe!(self.graph.filter(node_names, node_types, edge_types, min_weight, max_weight, verbose))
	}
	
}
