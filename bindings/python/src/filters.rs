use super::*;
use graph::{EdgeT, EdgeTypeT, NodeT, NodeTypeT, WeightT};

#[pymethods]
impl EnsmallenGraph {
	#[text_signature = "($self, verbose)"]
	/// Returns new graph without parallel edges.
	/// 
	/// Parameters
	/// --------------
	/// verbose: bool,
	/// 	Whether to show a loading bar while building the graph.
	///
	/// [Automatically generated binding]
	/// [Automatically generated documentation]
	fn drop_parallel_edges(&self, verbose : bool) -> EnsmallenGraph {
		EnsmallenGraph{graph:self.graph.drop_parallel_edges(verbose)}
	}
	
    #[text_signature = "($self, verbose)"]
    /// Returns new graph without selfloops.
    ///
    /// Parameters
    /// --------------
    /// verbose: bool,
    /// 	Whether to show a loading bar while building the graph.
    ///
    /// [Automatically generated binding]
    /// [Automatically generated documentation]
    fn drop_selfloops(&self, verbose: bool) -> EnsmallenGraph {
        EnsmallenGraph {
            graph: self.graph.drop_selfloops(verbose),
        }
    }

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
    fn drop_singleton_nodes(&self, verbose: bool) -> EnsmallenGraph {
        EnsmallenGraph {
            graph: self.graph.drop_singleton_nodes(verbose),
        }
    }

    #[text_signature = "($self, node_ids_to_keep, node_ids_to_filter, node_type_ids_to_keep, node_type_ids_to_filter, node_type_id_to_keep, node_type_id_to_filter, edge_ids_to_keep, edge_ids_to_filter, edge_node_ids_to_keep, edge_node_ids_to_filter, edge_type_ids_to_keep, edge_type_ids_to_filter, min_edge_weight, max_edge_weight, filter_singletons, filter_selfloops, verbose)"]
    /// Returns a **NEW** Graph that does not have the required attributes.
    ///
    /// Parameters
    /// --------------
    /// node_ids_to_keep: Union[List[int], None],
    /// 	List of node IDs to keep during filtering.
    /// node_ids_to_filter: Union[List[int], None],
    /// 	List of node IDs to remove during filtering.
    /// node_type_ids_to_keep: Union[List[Option<Vec<int], None]>>,
    /// 	List of node type IDs to keep during filtering. The node types must match entirely the given node types vector provided.
    /// node_type_ids_to_filter: Union[List[Option<Vec<int], None]>>,
    /// 	List of node type IDs to remove during filtering. The node types must match entirely the given node types vector provided.
    /// node_type_id_to_keep: Union[List[Option<int], None]>,
    /// 	List of node type IDs to keep during filtering. Any of node types must match with one of the node types given.
    /// node_type_id_to_filter: Union[List[Option<int], None]>,
    /// 	List of node type IDs to remove during filtering. Any of node types must match with one of the node types given.
    /// edge_ids_to_keep: Union[List[int], None],
    /// 	List of edge IDs to keep during filtering.
    /// edge_ids_to_filter: Union[List[int], None],
    /// 	List of edge IDs to remove during filtering.
    /// edge_node_ids_to_keep: Union[List[(int, int)], None],
    /// 	List of tuple of node IDs to keep during filtering.
    /// edge_node_ids_to_filter: Union[List[(int, int)], None],
    /// 	List of tuple of node IDs to remove during filtering.
    /// edge_type_ids_to_keep: Union[List[Option<int], None]>,
    /// 	List of edge type IDs to keep during filtering.
    /// edge_type_ids_to_filter: Union[List[Option<int], None]>,
    /// 	List of edge type IDs to remove during filtering.
    /// min_edge_weight: Union[WeightT, None],
    /// 	Minimum edge weight. Values lower than this are removed.
    /// max_edge_weight: Union[WeightT, None],
    /// 	Maximum edge weight. Values higher than this are removed.
    /// filter_singletons: bool,
    /// 	Whether to filter out singletons.
    /// filter_selfloops: bool,
    /// 	Whether to filter out selfloops.
    /// filter_parallel_edges: bool,
    ///     Whether to filter out parallel edges.
    /// verbose: bool,
    /// 	Whether to show loading bar while building the graphs.
    ///
    ///  ## Implementation details
    ///
    ///  ### How the collapse of multigraphs is handled
    ///  We keep only the first edge when a multigraph is collapsed while removing
    ///  the edge types, in the order provided when first reading from the CSV file.
    ///
    ///  ### Generation of new singleton nodes when removing edges
    ///  Some of the remove operations allowed in this method might lead to the
    ///  generation of new singleton nodes that will not be handled within this
    ///  function call even if you provide the flag singletons to true, but you
    ///  will need to call the method again if you want to get reed of also those
    ///  newly created singleton nodes.
    ///
    /// [Automatically generated binding]
    /// [Automatically generated documentation]
    fn filter_from_ids(
        &self,
        node_ids_to_keep: Option<Vec<NodeT>>,
        node_ids_to_filter: Option<Vec<NodeT>>,
        node_type_ids_to_keep: Option<Vec<Option<Vec<NodeTypeT>>>>,
        node_type_ids_to_filter: Option<Vec<Option<Vec<NodeTypeT>>>>,
        node_type_id_to_keep: Option<Vec<Option<NodeTypeT>>>,
        node_type_id_to_filter: Option<Vec<Option<NodeTypeT>>>,
        edge_ids_to_keep: Option<Vec<EdgeT>>,
        edge_ids_to_filter: Option<Vec<EdgeT>>,
        edge_node_ids_to_keep: Option<Vec<(NodeT, NodeT)>>,
        edge_node_ids_to_filter: Option<Vec<(NodeT, NodeT)>>,
        edge_type_ids_to_keep: Option<Vec<Option<EdgeTypeT>>>,
        edge_type_ids_to_filter: Option<Vec<Option<EdgeTypeT>>>,
        min_edge_weight: Option<WeightT>,
        max_edge_weight: Option<WeightT>,
        filter_singletons: bool,
        filter_selfloops: bool,
        filter_parallel_edges: bool,
        verbose: bool,
    ) -> EnsmallenGraph {
        EnsmallenGraph {
            graph: self.graph.filter_from_ids(
                node_ids_to_keep,
                node_ids_to_filter,
                node_type_ids_to_keep,
                node_type_ids_to_filter,
                node_type_id_to_keep,
                node_type_id_to_filter,
                edge_ids_to_keep,
                edge_ids_to_filter,
                edge_node_ids_to_keep,
                edge_node_ids_to_filter,
                edge_type_ids_to_keep,
                edge_type_ids_to_filter,
                min_edge_weight,
                max_edge_weight,
                filter_singletons,
                filter_selfloops,
                filter_parallel_edges,
                verbose,
            ),
        }
    }

    #[text_signature = "($self, node_names_to_keep, node_names_to_filter, node_type_names_to_keep, node_type_names_to_filter, node_type_name_to_keep, node_type_name_to_filter, edge_node_names_to_keep, edge_node_names_to_filter, edge_type_names_to_keep, edge_type_names_to_filter, min_edge_weight, max_edge_weight, filter_singletons, filter_selfloops, verbose)"]
    /// Returns a **NEW** Graph that does not have the required attributes.
    ///
    /// Parameters
    /// --------------
    /// node_names_to_keep: Union[List[str], None],
    /// 	List of node names to keep during filtering.
    /// node_names_to_filter: Union[List[str], None],
    /// 	List of node names to remove during filtering.
    /// node_type_names_to_keep: Union[List[Option<Vec<str], None]>>,
    /// 	List of node type names to keep during filtering. The node types must match entirely the given node types vector provided.
    /// node_type_names_to_filter: Union[List[Option<Vec<str], None]>>,
    /// 	List of node type names to remove during filtering. The node types must match entirely the given node types vector provided.
    /// node_type_name_to_keep: Union[List[Option<str], None]>,
    /// 	List of node type name to keep during filtering. Any of node types must match with one of the node types given.
    /// node_type_name_to_filter: Union[List[Option<str], None]>,
    /// 	List of node type name to remove during filtering. Any of node types must match with one of the node types given.
    /// edge_node_names_to_keep: Union[List[(str, str)], None],
    /// 	List of tuple of node names to keep during filtering.
    /// edge_node_names_to_filter: Union[List[(str, str)], None],
    /// 	List of tuple of node names to remove during filtering.
    /// edge_type_names_to_keep: Union[List[Option<str], None]>,
    /// 	List of edge type names to keep during filtering.
    /// edge_type_names_to_filter: Union[List[Option<str], None]>,
    /// 	List of edge type names to remove during filtering.
    /// min_edge_weight: Union[WeightT, None],
    /// 	Minimum edge weight. Values lower than this are removed.
    /// max_edge_weight: Union[WeightT, None],
    /// 	Maximum edge weight. Values higher than this are removed.
    /// filter_singletons: bool,
    /// 	Whether to filter out singletons.
    /// filter_selfloops: bool,
    /// 	Whether to filter out selfloops.
    /// filter_parallel_edges: bool,
    ///     Whether to filter out parallel edges.
    /// verbose: bool,
    /// 	Whether to show loading bar while building the graphs.
    ///
    ///  ## Implementation details
    ///
    ///  ### How the collapse of multigraphs is handled
    ///  We keep only the first edge when a multigraph is collapsed while removing
    ///  the edge types, in the order provided when first reading from the CSV file.
    ///
    ///  ### Generation of new singleton nodes when removing edges
    ///  Some of the remove operations allowed in this method might lead to the
    ///  generation of new singleton nodes that will not be handled within this
    ///  function call even if you provide the flag singletons to true, but you
    ///  will need to call the method again if you want to get reed of also those
    ///  newly created singleton nodes.
    ///
    /// [Automatically generated binding]
    /// [Automatically generated documentation]
    fn filter_from_names(
        &self,
        node_names_to_keep: Option<Vec<&str>>,
        node_names_to_filter: Option<Vec<&str>>,
        node_type_names_to_keep: Option<Vec<Option<Vec<&str>>>>,
        node_type_names_to_filter: Option<Vec<Option<Vec<&str>>>>,
        node_type_name_to_keep: Option<Vec<Option<String>>>,
        node_type_name_to_filter: Option<Vec<Option<String>>>,
        edge_node_names_to_keep: Option<Vec<(&str, &str)>>,
        edge_node_names_to_filter: Option<Vec<(&str, &str)>>,
        edge_type_names_to_keep: Option<Vec<Option<String>>>,
        edge_type_names_to_filter: Option<Vec<Option<String>>>,
        min_edge_weight: Option<WeightT>,
        max_edge_weight: Option<WeightT>,
        filter_singletons: bool,
        filter_selfloops: bool,
        filter_parallel_edges: bool,
        verbose: bool,
    ) -> PyResult<EnsmallenGraph> {
        Ok(EnsmallenGraph {
            graph: pe!(self.graph.filter_from_names(
                node_names_to_keep,
                node_names_to_filter,
                node_type_names_to_keep,
                node_type_names_to_filter,
                node_type_name_to_keep,
                node_type_name_to_filter,
                edge_node_names_to_keep,
                edge_node_names_to_filter,
                edge_type_names_to_keep,
                edge_type_names_to_filter,
                min_edge_weight,
                max_edge_weight,
                filter_singletons,
                filter_selfloops,
                filter_parallel_edges,
                verbose
            ))?,
        })
    }
}
