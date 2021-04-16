use super::*;
use arbitrary::Arbitrary;
use std::collections::HashSet;
use rayon::iter::ParallelIterator;

#[derive(Arbitrary, Debug, Clone)]
pub struct RandomSpanningArborescenceKruskalParams {
	pub random_state : EdgeT,
	pub unwanted_edge_types : Option<HashSet<Option<EdgeTypeT>>>,
	pub verbose : bool,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct SpanningArborescenceKruskalParams {
	pub verbose : bool,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct SpanningArborescenceParams {
	pub verbose : bool,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct ConnectedComponentsParams {
	pub verbose : bool,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct GetNodeLabelPredictionTupleFromNodeIdsParams {
	pub node_ids : Vec<NodeT>,
	pub random_state : u64,
	pub include_central_node : bool,
	pub offset : NodeT,
	pub max_neighbours : Option<NodeT>,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct IsSingletonFromNodeIdParams {
	pub node_id : NodeT,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct IsSingletonWithSelfloopsFromNodeIdParams {
	pub node_id : NodeT,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct HasEdgeFromNodeIdsParams {
	pub src : NodeT,
	pub dst : NodeT,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct HasEdgeFromNodeIdsAndEdgeTypeIdParams {
	pub src : NodeT,
	pub dst : NodeT,
	pub edge_type : Option<EdgeTypeT>,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct IsTrapNodeFromNodeIdParams {
	pub node_id : NodeT,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct EnableParams {
	pub vector_sources : bool,
	pub vector_destinations : bool,
	pub vector_cumulative_node_degrees : bool,
	pub cache_size : Option<f64>,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct IterEdgeIdsFromNodeIdsParams {
	pub src : NodeT,
	pub dst : NodeT,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct GetNodeIdsFromEdgeIdParams {
	pub edge_id : EdgeT,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct GetEdgeIdFromNodeIdsParams {
	pub src : NodeT,
	pub dst : NodeT,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct GetNodeIdsAndEdgeTypeIdFromEdgeIdParams {
	pub edge_id : EdgeT,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct GetNodeIdsAndEdgeTypeIdAndEdgeWeightFromEdgeIdParams {
	pub edge_id : EdgeT,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct GetTopKCentralNodeIdsParams {
	pub k : NodeT,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct GetNodeDegreeFromNodeIdParams {
	pub node_id : NodeT,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct GetTopKCentralNodeNamesParams {
	pub k : NodeT,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct GetNodeTypeIdFromNodeIdParams {
	pub node_id : NodeT,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct GetEdgeTypeIdFromEdgeIdParams {
	pub edge_id : EdgeT,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct GetNodeTypeNameFromNodeIdParams {
	pub node_id : NodeT,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct GetEdgeTypeNameFromEdgeIdParams {
	pub edge_id : EdgeT,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct GetEdgeTypeNameFromEdgeTypeIdParams {
	pub edge_type_id : EdgeTypeT,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct GetEdgeWeightFromEdgeIdParams {
	pub edge_id : EdgeT,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct GetEdgeWeightFromNodeIdsParams {
	pub src : NodeT,
	pub dst : NodeT,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct GetEdgeWeightFromNodeIdsAndEdgeTypeIdParams {
	pub src : NodeT,
	pub dst : NodeT,
	pub edge_type : Option<EdgeTypeT>,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct GetNodeNameFromNodeIdParams {
	pub node_id : NodeT,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct GetEdgeCountFromEdgeTypeIdParams {
	pub edge_type : Option<EdgeTypeT>,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct GetNodeCountFromNodeTypeIdParams {
	pub node_type : Option<NodeTypeT>,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct GetDestinationNodeIdFromEdgeIdParams {
	pub edge_id : EdgeT,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct GetNeighbourNodeIdsFromNodeIdParams {
	pub node_id : NodeT,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct GetMinmaxEdgeIdsFromNodeIdsParams {
	pub src : NodeT,
	pub dst : NodeT,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct GetEdgeIdFromNodeIdsAndEdgeTypeIdParams {
	pub src : NodeT,
	pub dst : NodeT,
	pub edge_type : Option<EdgeTypeT>,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct GetEdgeTypeIdsFromEdgeTypeNamesParams {
	pub edge_types : Vec<Option<String>>,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct GetNodeTypeIdsFromNodeTypeNamesParams {
	pub node_types : Vec<Option<String>>,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct GetMinmaxEdgeIdsFromSourceNodeIdParams {
	pub src : NodeT,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct GetNodeTypeNameFromNodeTypeIdParams {
	pub node_type_id : NodeTypeT,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct GetNodeTypeNamesFromNodeTypeIdsParams {
	pub node_type_ids : Vec<NodeTypeT>,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct FilterParams {
	pub node_names : Option<Vec<String>>,
	pub node_types : Option<Vec<Option<String>>>,
	pub edge_types : Option<Vec<Option<String>>>,
	pub min_weight : Option<WeightT>,
	pub max_weight : Option<WeightT>,
	pub verbose : bool,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct ValidateNodeIdParams {
	pub node_id : NodeT,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct ValidateEdgeIdParams {
	pub edge_id : EdgeT,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct DegreesProductParams {
	pub one : NodeT,
	pub two : NodeT,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct JaccardIndexParams {
	pub one : NodeT,
	pub two : NodeT,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct AdamicAdarIndexParams {
	pub one : NodeT,
	pub two : NodeT,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct ResourceAllocationIndexParams {
	pub one : NodeT,
	pub two : NodeT,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct RemoveParams {
	pub allow_nodes_set : Option<HashSet<String>>,
	pub deny_nodes_set : Option<HashSet<String>>,
	pub allow_node_types_set : Option<HashSet<String>>,
	pub deny_node_types_set : Option<HashSet<String>>,
	pub allow_edge_set : Option<HashSet<EdgeT>>,
	pub deny_edge_set : Option<HashSet<EdgeT>>,
	pub allow_edge_types_set : Option<HashSet<String>>,
	pub deny_edge_types_set : Option<HashSet<String>>,
	pub weights : bool,
	pub node_types : bool,
	pub edge_types : bool,
	pub singletons : bool,
	pub selfloops : bool,
	pub verbose : bool,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct RemoveComponentsParams {
	pub node_names : Option<Vec<String>>,
	pub node_types : Option<Vec<Option<String>>>,
	pub edge_types : Option<Vec<Option<String>>>,
	pub minimum_component_size : Option<NodeT>,
	pub top_k_components : Option<NodeT>,
	pub verbose : bool,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct SetNameParams {
	pub name : String,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct SetAllEdgeTypesParams {
	pub edge_type : String,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct SetAllNodeTypesParams {
	pub node_type : String,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct EncodeEdgeParams {
	pub src : NodeT,
	pub dst : NodeT,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct DecodeEdgeParams {
	pub edge : u64,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct GetBipartiteEdgesParams {
	pub removed_existing_edges : Option<bool>,
	pub first_nodes_set : Option<HashSet<String>>,
	pub second_nodes_set : Option<HashSet<String>>,
	pub first_node_types_set : Option<HashSet<String>>,
	pub second_node_types_set : Option<HashSet<String>>,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct GetBipartiteEdgeNamesParams {
	pub removed_existing_edges : Option<bool>,
	pub first_nodes_set : Option<HashSet<String>>,
	pub second_nodes_set : Option<HashSet<String>>,
	pub first_node_types_set : Option<HashSet<String>>,
	pub second_node_types_set : Option<HashSet<String>>,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct GetStarEdgesParams {
	pub central_node : String,
	pub removed_existing_edges : Option<bool>,
	pub star_points_nodes_set : Option<HashSet<String>>,
	pub star_points_node_types_set : Option<HashSet<String>>,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct GetStarEdgeNamesParams {
	pub central_node : String,
	pub removed_existing_edges : Option<bool>,
	pub star_points_nodes_set : Option<HashSet<String>>,
	pub star_points_node_types_set : Option<HashSet<String>>,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct GetCliqueEdgesParams {
	pub directed : Option<bool>,
	pub allow_selfloops : Option<bool>,
	pub removed_existing_edges : Option<bool>,
	pub allow_node_type_set : Option<HashSet<String>>,
	pub allow_node_set : Option<HashSet<String>>,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct GetCliqueEdgeNamesParams {
	pub directed : Option<bool>,
	pub allow_selfloops : Option<bool>,
	pub removed_existing_edges : Option<bool>,
	pub allow_node_type_set : Option<HashSet<String>>,
	pub allow_node_set : Option<HashSet<String>>,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct TextualReportParams {
	pub verbose : bool,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct GetConnectedComponentsNumberParams {
	pub verbose : bool,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct GetSourcesParams {
	pub directed : bool,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct GetSourceNamesParams {
	pub directed : bool,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct GetDestinationsParams {
	pub directed : bool,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct GetDestinationNamesParams {
	pub directed : bool,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct GetEdgesParams {
	pub directed : bool,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct GetEdgeNodeNamesParams {
	pub directed : bool,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct GetNodeConnectedComponentIdsParams {
	pub verbose : bool,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct IterSourceNodeIdsParams {
	pub directed : bool,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct ParIterSourceNodeIdsParams {
	pub directed : bool,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct IterDestinationNodeIdsParams {
	pub directed : bool,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct ParIterDestinationNodeIdsParams {
	pub directed : bool,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct IterEdgeIdsParams {
	pub directed : bool,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct IterEdgesParams {
	pub directed : bool,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct ParIterEdgeIdsParams {
	pub directed : bool,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct ParIterEdgesParams {
	pub directed : bool,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct IterEdgeNodeIdsAndEdgeTypeIdParams {
	pub directed : bool,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct IterEdgeNodeNamesAndEdgeTypeNameParams {
	pub directed : bool,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct ParIterEdgeNodeNamesAndEdgeTypeNameParams {
	pub directed : bool,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct ParIterEdgeNodeIdsAndEdgeTypeIdParams {
	pub directed : bool,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct ParIterEdgeNodeNamesAndEdgeTypeNameAndEdgeWeightParams {
	pub directed : bool,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct IterEdgeNodeNamesAndEdgeTypeNameAndEdgeWeightParams {
	pub directed : bool,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct ParIterEdgeNodeIdsAndEdgeTypeIdAndEdgeWeightParams {
	pub directed : bool,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct IterEdgeNodeIdsAndEdgeTypeIdAndEdgeWeightParams {
	pub directed : bool,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct IterUniqueEdgeNodeIdsParams {
	pub directed : bool,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct ConnectedHoldoutParams {
	pub random_state : EdgeT,
	pub train_size : f64,
	pub edge_types : Option<Vec<Option<String>>>,
	pub include_all_edge_types : bool,
	pub verbose : bool,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct RandomHoldoutParams {
	pub random_state : EdgeT,
	pub train_size : f64,
	pub include_all_edge_types : bool,
	pub edge_types : Option<Vec<Option<String>>>,
	pub min_number_overlaps : Option<EdgeT>,
	pub verbose : bool,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct NodeLabelHoldoutParams {
	pub train_size : f64,
	pub use_stratification : bool,
	pub random_state : EdgeT,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct EdgeLabelHoldoutParams {
	pub train_size : f64,
	pub use_stratification : bool,
	pub random_state : EdgeT,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct RandomSubgraphParams {
	pub random_state : usize,
	pub nodes_number : NodeT,
	pub verbose : bool,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct KfoldParams {
	pub k : EdgeT,
	pub k_index : u64,
	pub edge_types : Option<Vec<Option<String>>>,
	pub random_state : EdgeT,
	pub verbose : bool,
}

#[derive(Arbitrary, Debug, Clone)]
pub struct MetaParams {
	pub random_spanning_arborescence_kruskal: RandomSpanningArborescenceKruskalParams,
	pub spanning_arborescence_kruskal: SpanningArborescenceKruskalParams,
	pub spanning_arborescence: SpanningArborescenceParams,
	pub connected_components: ConnectedComponentsParams,
	pub get_node_label_prediction_tuple_from_node_ids: GetNodeLabelPredictionTupleFromNodeIdsParams,
	pub is_singleton_from_node_id: IsSingletonFromNodeIdParams,
	pub is_singleton_with_selfloops_from_node_id: IsSingletonWithSelfloopsFromNodeIdParams,
	pub has_edge_from_node_ids: HasEdgeFromNodeIdsParams,
	pub has_edge_from_node_ids_and_edge_type_id: HasEdgeFromNodeIdsAndEdgeTypeIdParams,
	pub is_trap_node_from_node_id: IsTrapNodeFromNodeIdParams,
	pub enable: EnableParams,
	pub iter_edge_ids_from_node_ids: IterEdgeIdsFromNodeIdsParams,
	pub get_node_ids_from_edge_id: GetNodeIdsFromEdgeIdParams,
	pub get_edge_id_from_node_ids: GetEdgeIdFromNodeIdsParams,
	pub get_node_ids_and_edge_type_id_from_edge_id: GetNodeIdsAndEdgeTypeIdFromEdgeIdParams,
	pub get_node_ids_and_edge_type_id_and_edge_weight_from_edge_id: GetNodeIdsAndEdgeTypeIdAndEdgeWeightFromEdgeIdParams,
	pub get_top_k_central_node_ids: GetTopKCentralNodeIdsParams,
	pub get_node_degree_from_node_id: GetNodeDegreeFromNodeIdParams,
	pub get_top_k_central_node_names: GetTopKCentralNodeNamesParams,
	pub get_node_type_id_from_node_id: GetNodeTypeIdFromNodeIdParams,
	pub get_edge_type_id_from_edge_id: GetEdgeTypeIdFromEdgeIdParams,
	pub get_node_type_name_from_node_id: GetNodeTypeNameFromNodeIdParams,
	pub get_edge_type_name_from_edge_id: GetEdgeTypeNameFromEdgeIdParams,
	pub get_edge_type_name_from_edge_type_id: GetEdgeTypeNameFromEdgeTypeIdParams,
	pub get_edge_weight_from_edge_id: GetEdgeWeightFromEdgeIdParams,
	pub get_edge_weight_from_node_ids: GetEdgeWeightFromNodeIdsParams,
	pub get_edge_weight_from_node_ids_and_edge_type_id: GetEdgeWeightFromNodeIdsAndEdgeTypeIdParams,
	pub get_node_name_from_node_id: GetNodeNameFromNodeIdParams,
	pub get_edge_count_from_edge_type_id: GetEdgeCountFromEdgeTypeIdParams,
	pub get_node_count_from_node_type_id: GetNodeCountFromNodeTypeIdParams,
	pub get_destination_node_id_from_edge_id: GetDestinationNodeIdFromEdgeIdParams,
	pub get_neighbour_node_ids_from_node_id: GetNeighbourNodeIdsFromNodeIdParams,
	pub get_minmax_edge_ids_from_node_ids: GetMinmaxEdgeIdsFromNodeIdsParams,
	pub get_edge_id_from_node_ids_and_edge_type_id: GetEdgeIdFromNodeIdsAndEdgeTypeIdParams,
	pub get_edge_type_ids_from_edge_type_names: GetEdgeTypeIdsFromEdgeTypeNamesParams,
	pub get_node_type_ids_from_node_type_names: GetNodeTypeIdsFromNodeTypeNamesParams,
	pub get_minmax_edge_ids_from_source_node_id: GetMinmaxEdgeIdsFromSourceNodeIdParams,
	pub get_node_type_name_from_node_type_id: GetNodeTypeNameFromNodeTypeIdParams,
	pub get_node_type_names_from_node_type_ids: GetNodeTypeNamesFromNodeTypeIdsParams,
	pub filter: FilterParams,
	pub validate_node_id: ValidateNodeIdParams,
	pub validate_edge_id: ValidateEdgeIdParams,
	pub degrees_product: DegreesProductParams,
	pub jaccard_index: JaccardIndexParams,
	pub adamic_adar_index: AdamicAdarIndexParams,
	pub resource_allocation_index: ResourceAllocationIndexParams,
	pub remove: RemoveParams,
	pub remove_components: RemoveComponentsParams,
	pub set_name: SetNameParams,
	pub set_all_edge_types: SetAllEdgeTypesParams,
	pub set_all_node_types: SetAllNodeTypesParams,
	pub encode_edge: EncodeEdgeParams,
	pub decode_edge: DecodeEdgeParams,
	pub get_bipartite_edges: GetBipartiteEdgesParams,
	pub get_bipartite_edge_names: GetBipartiteEdgeNamesParams,
	pub get_star_edges: GetStarEdgesParams,
	pub get_star_edge_names: GetStarEdgeNamesParams,
	pub get_clique_edges: GetCliqueEdgesParams,
	pub get_clique_edge_names: GetCliqueEdgeNamesParams,
	pub textual_report: TextualReportParams,
	pub get_connected_components_number: GetConnectedComponentsNumberParams,
	pub get_sources: GetSourcesParams,
	pub get_source_names: GetSourceNamesParams,
	pub get_destinations: GetDestinationsParams,
	pub get_destination_names: GetDestinationNamesParams,
	pub get_edges: GetEdgesParams,
	pub get_edge_node_names: GetEdgeNodeNamesParams,
	pub get_node_connected_component_ids: GetNodeConnectedComponentIdsParams,
	pub iter_source_node_ids: IterSourceNodeIdsParams,
	pub par_iter_source_node_ids: ParIterSourceNodeIdsParams,
	pub iter_destination_node_ids: IterDestinationNodeIdsParams,
	pub par_iter_destination_node_ids: ParIterDestinationNodeIdsParams,
	pub iter_edge_ids: IterEdgeIdsParams,
	pub iter_edges: IterEdgesParams,
	pub par_iter_edge_ids: ParIterEdgeIdsParams,
	pub par_iter_edges: ParIterEdgesParams,
	pub iter_edge_node_ids_and_edge_type_id: IterEdgeNodeIdsAndEdgeTypeIdParams,
	pub iter_edge_node_names_and_edge_type_name: IterEdgeNodeNamesAndEdgeTypeNameParams,
	pub par_iter_edge_node_names_and_edge_type_name: ParIterEdgeNodeNamesAndEdgeTypeNameParams,
	pub par_iter_edge_node_ids_and_edge_type_id: ParIterEdgeNodeIdsAndEdgeTypeIdParams,
	pub par_iter_edge_node_names_and_edge_type_name_and_edge_weight: ParIterEdgeNodeNamesAndEdgeTypeNameAndEdgeWeightParams,
	pub iter_edge_node_names_and_edge_type_name_and_edge_weight: IterEdgeNodeNamesAndEdgeTypeNameAndEdgeWeightParams,
	pub par_iter_edge_node_ids_and_edge_type_id_and_edge_weight: ParIterEdgeNodeIdsAndEdgeTypeIdAndEdgeWeightParams,
	pub iter_edge_node_ids_and_edge_type_id_and_edge_weight: IterEdgeNodeIdsAndEdgeTypeIdAndEdgeWeightParams,
	pub iter_unique_edge_node_ids: IterUniqueEdgeNodeIdsParams,
	pub connected_holdout: ConnectedHoldoutParams,
	pub random_holdout: RandomHoldoutParams,
	pub node_label_holdout: NodeLabelHoldoutParams,
	pub edge_label_holdout: EdgeLabelHoldoutParams,
	pub random_subgraph: RandomSubgraphParams,
	pub kfold: KfoldParams,
    pub from_vec: FromVecHarnessParams,
}

pub fn meta_test(data: MetaParams) -> Result<(), String> {
    let data_copy = data.clone();
    let data_copy2 = data.clone();
    std::panic::set_hook(Box::new(move |info| {
        handle_panics_meta_test(Some(info), data_copy.clone(), None);
    }));

    let mut graph = graph::Graph::from_string_unsorted(
        data.from_vec.edges.into_iter(),
        data.from_vec.nodes.map(|ns| ns.into_iter()),
        data.from_vec.directed,
        data.from_vec.directed_edge_list,
        "MetaTest",
        data.from_vec.ignore_duplicated_nodes,
        false,
        data.from_vec.ignore_duplicated_edges,
        false,
        data.from_vec.verbose,
        data.from_vec.numeric_edge_types_ids,
        data.from_vec.numeric_node_ids,
        data.from_vec.numeric_edge_node_ids,
        data.from_vec.numeric_node_types_ids,
        data.from_vec.has_node_types,
        data.from_vec.has_edge_types,
        data.from_vec.has_edge_weights,
        true,
        true,
        true,
    )?;
    
    

    let g_copy = graph.clone();
    std::panic::set_hook(Box::new(move |info| {
        handle_panics_meta_test_once_loaded(Some(info), data_copy2.clone(), g_copy.clone());
    }));
    
	let _ = graph.iter_destination_node_ids(data.iter_destination_node_ids.directed).collect::<Vec<_>>();
	let _ = graph.iter_edge_ids(data.iter_edge_ids.directed).collect::<Vec<_>>();
	let _ = graph.iter_edge_node_ids_and_edge_type_id(data.iter_edge_node_ids_and_edge_type_id.directed).collect::<Vec<_>>();
	let _ = graph.iter_edge_node_ids_and_edge_type_id_and_edge_weight(data.iter_edge_node_ids_and_edge_type_id_and_edge_weight.directed).collect::<Vec<_>>();
	let _ = graph.iter_edge_node_names_and_edge_type_name(data.iter_edge_node_names_and_edge_type_name.directed).collect::<Vec<_>>();
	let _ = graph.iter_edge_node_names_and_edge_type_name_and_edge_weight(data.iter_edge_node_names_and_edge_type_name_and_edge_weight.directed).collect::<Vec<_>>();
	let _ = graph.iter_edges(data.iter_edges.directed).collect::<Vec<_>>();
	let _ = graph.iter_node_degrees().collect::<Vec<_>>();
	let _ = graph.iter_node_ids().collect::<Vec<_>>();
	let _ = graph.iter_node_ids_and_node_type_ids().collect::<Vec<_>>();
	let _ = graph.iter_nodes().collect::<Vec<_>>();
	let _ = graph.iter_non_singleton_node_ids().collect::<Vec<_>>();
	let _ = graph.iter_singleton_node_ids().collect::<Vec<_>>();
	let _ = graph.iter_singleton_with_selfloops_node_ids().collect::<Vec<_>>();
	let _ = graph.iter_source_node_ids(data.iter_source_node_ids.directed).collect::<Vec<_>>();
	let _ = graph.iter_unique_edge_node_ids(data.iter_unique_edge_node_ids.directed).collect::<Vec<_>>();
	let _ = graph.iter_unique_source_node_ids().collect::<Vec<_>>();
	let _ = graph.par_iter_destination_node_ids(data.par_iter_destination_node_ids.directed).collect::<Vec<_>>();
	let _ = graph.par_iter_edge_ids(data.par_iter_edge_ids.directed).collect::<Vec<_>>();
	let _ = graph.par_iter_edge_node_ids_and_edge_type_id(data.par_iter_edge_node_ids_and_edge_type_id.directed).collect::<Vec<_>>();
	let _ = graph.par_iter_edge_node_ids_and_edge_type_id_and_edge_weight(data.par_iter_edge_node_ids_and_edge_type_id_and_edge_weight.directed).collect::<Vec<_>>();
	let _ = graph.par_iter_edge_node_names_and_edge_type_name(data.par_iter_edge_node_names_and_edge_type_name.directed).collect::<Vec<_>>();
	let _ = graph.par_iter_edge_node_names_and_edge_type_name_and_edge_weight(data.par_iter_edge_node_names_and_edge_type_name_and_edge_weight.directed).collect::<Vec<_>>();
	let _ = graph.par_iter_edges(data.par_iter_edges.directed).collect::<Vec<_>>();
	let _ = graph.par_iter_node_degrees().collect::<Vec<_>>();
	let _ = graph.par_iter_node_ids().collect::<Vec<_>>();
	let _ = graph.par_iter_node_ids_and_node_type_ids().collect::<Vec<_>>();
	let _ = graph.par_iter_source_node_ids(data.par_iter_source_node_ids.directed).collect::<Vec<_>>();
	graph.compute_hash();
	graph.decode_edge(data.decode_edge.edge);
	graph.disable_all();
	graph.encode_edge(data.encode_edge.src, data.encode_edge.dst);
	graph.get_clique_edge_names(data.get_clique_edge_names.directed, data.get_clique_edge_names.allow_selfloops, data.get_clique_edge_names.removed_existing_edges, data.get_clique_edge_names.allow_node_type_set, data.get_clique_edge_names.allow_node_set);
	graph.get_clique_edges(data.get_clique_edges.directed, data.get_clique_edges.allow_selfloops, data.get_clique_edges.removed_existing_edges, data.get_clique_edges.allow_node_type_set, data.get_clique_edges.allow_node_set);
	graph.get_connected_components_number(data.get_connected_components_number.verbose);
	graph.get_cumulative_node_degrees();
	graph.get_dense_nodes_mapping();
	graph.get_destination_names(data.get_destination_names.directed);
	graph.get_destinations(data.get_destinations.directed);
	graph.get_directed_edges_number();
	graph.get_edge_node_names(data.get_edge_node_names.directed);
	graph.get_edge_type_names();
	graph.get_edge_types_number();
	graph.get_edges(data.get_edges.directed);
	graph.get_edges_number();
	graph.get_max_encodable_edge_number();
	graph.get_minimum_edge_types_number();
	graph.get_minimum_node_types_number();
	graph.get_multigraph_edges_number();
	graph.get_name();
	graph.get_node_connected_component_ids(data.get_node_connected_component_ids.verbose);
	graph.get_node_degrees();
	graph.get_node_names();
	graph.get_node_types_number();
	graph.get_nodes();
	graph.get_nodes_mapping();
	graph.get_nodes_number();
	graph.get_not_singleton_nodes_number();
	graph.get_not_singletons_node_ids();
	graph.get_selfloop_number();
	graph.get_singleton_nodes_number();
	graph.get_singleton_nodes_with_selfloops_number();
	graph.get_source_names(data.get_source_names.directed);
	graph.get_sources(data.get_sources.directed);
	graph.get_top_k_central_node_ids(data.get_top_k_central_node_ids.k);
	graph.get_top_k_central_node_names(data.get_top_k_central_node_names.k);
	graph.get_trap_nodes_number();
	graph.get_traps_rate();
	graph.get_undirected_edges_number();
	graph.get_unique_directed_edges_number();
	graph.get_unique_edges_number();
	graph.get_unique_selfloop_number();
	graph.get_unique_source_nodes_number();
	graph.get_unique_undirected_edges_number();
	graph.get_unknown_edge_types_number();
	graph.get_unknown_node_types_number();
	graph.has_edge_from_node_ids(data.has_edge_from_node_ids.src, data.has_edge_from_node_ids.dst);
	graph.has_edge_from_node_ids_and_edge_type_id(data.has_edge_from_node_ids_and_edge_type_id.src, data.has_edge_from_node_ids_and_edge_type_id.dst, data.has_edge_from_node_ids_and_edge_type_id.edge_type);
	graph.has_edge_types();
	graph.has_edge_weights();
	graph.has_edges();
	graph.has_multilabel_node_types();
	graph.has_node_types();
	graph.has_nodes();
	graph.has_selfloops();
	graph.has_singletons();
	graph.has_singletons_with_selfloops();
	graph.has_trap_nodes();
	graph.has_unknown_edge_types();
	graph.has_unknown_node_types();
	graph.is_directed();
	graph.is_multigraph();
	graph.is_singleton_with_selfloops_from_node_id(data.is_singleton_with_selfloops_from_node_id.node_id);
	graph.random_spanning_arborescence_kruskal(data.random_spanning_arborescence_kruskal.random_state, &data.random_spanning_arborescence_kruskal.unwanted_edge_types, data.random_spanning_arborescence_kruskal.verbose);
	graph.report();
	graph.set_name(data.set_name.name);
	graph.spanning_arborescence_kruskal(data.spanning_arborescence_kruskal.verbose);
	graph.strongly_connected_components();
	let _ = graph.adamic_adar_index(data.adamic_adar_index.one, data.adamic_adar_index.two);
	let _ = graph.connected_components(data.connected_components.verbose);
	let _ = graph.connected_holdout(data.connected_holdout.random_state, data.connected_holdout.train_size, data.connected_holdout.edge_types, data.connected_holdout.include_all_edge_types, data.connected_holdout.verbose);
	let _ = graph.degrees_product(data.degrees_product.one, data.degrees_product.two);
	let _ = graph.edge_label_holdout(data.edge_label_holdout.train_size, data.edge_label_holdout.use_stratification, data.edge_label_holdout.random_state);
	let _ = graph.enable(data.enable.vector_sources, data.enable.vector_destinations, data.enable.vector_cumulative_node_degrees, data.enable.cache_size);
	let _ = graph.filter(data.filter.node_names, data.filter.node_types, data.filter.edge_types, data.filter.min_weight, data.filter.max_weight, data.filter.verbose);
	let _ = graph.get_bipartite_edge_names(data.get_bipartite_edge_names.removed_existing_edges, data.get_bipartite_edge_names.first_nodes_set, data.get_bipartite_edge_names.second_nodes_set, data.get_bipartite_edge_names.first_node_types_set, data.get_bipartite_edge_names.second_node_types_set);
	let _ = graph.get_bipartite_edges(data.get_bipartite_edges.removed_existing_edges, data.get_bipartite_edges.first_nodes_set, data.get_bipartite_edges.second_nodes_set, data.get_bipartite_edges.first_node_types_set, data.get_bipartite_edges.second_node_types_set);
	let _ = graph.get_density();
	let _ = graph.get_destination_node_id_from_edge_id(data.get_destination_node_id_from_edge_id.edge_id);
	let _ = graph.get_edge_count_from_edge_type_id(data.get_edge_count_from_edge_type_id.edge_type);
	let _ = graph.get_edge_id_from_node_ids(data.get_edge_id_from_node_ids.src, data.get_edge_id_from_node_ids.dst);
	let _ = graph.get_edge_id_from_node_ids_and_edge_type_id(data.get_edge_id_from_node_ids_and_edge_type_id.src, data.get_edge_id_from_node_ids_and_edge_type_id.dst, data.get_edge_id_from_node_ids_and_edge_type_id.edge_type);
	let _ = graph.get_edge_type_counter();
	let _ = graph.get_edge_type_counts_hashmap();
	let _ = graph.get_edge_type_id_from_edge_id(data.get_edge_type_id_from_edge_id.edge_id);
	let _ = graph.get_edge_type_ids_from_edge_type_names(data.get_edge_type_ids_from_edge_type_names.edge_types);
	let _ = graph.get_edge_type_name_from_edge_id(data.get_edge_type_name_from_edge_id.edge_id);
	let _ = graph.get_edge_type_name_from_edge_type_id(data.get_edge_type_name_from_edge_type_id.edge_type_id);
	let _ = graph.get_edge_types();
	let _ = graph.get_edge_weight_from_edge_id(data.get_edge_weight_from_edge_id.edge_id);
	let _ = graph.get_edge_weight_from_node_ids(data.get_edge_weight_from_node_ids.src, data.get_edge_weight_from_node_ids.dst);
	let _ = graph.get_edge_weight_from_node_ids_and_edge_type_id(data.get_edge_weight_from_node_ids_and_edge_type_id.src, data.get_edge_weight_from_node_ids_and_edge_type_id.dst, data.get_edge_weight_from_node_ids_and_edge_type_id.edge_type);
	let _ = graph.get_edge_weights();
	let _ = graph.get_max_edge_weight();
	let _ = graph.get_max_node_degree();
	let _ = graph.get_min_edge_weight();
	let _ = graph.get_min_node_degree();
	let _ = graph.get_minmax_edge_ids_from_node_ids(data.get_minmax_edge_ids_from_node_ids.src, data.get_minmax_edge_ids_from_node_ids.dst);
	let _ = graph.get_minmax_edge_ids_from_source_node_id(data.get_minmax_edge_ids_from_source_node_id.src);
	let _ = graph.get_neighbour_node_ids_from_node_id(data.get_neighbour_node_ids_from_node_id.node_id);
	let _ = graph.get_node_count_from_node_type_id(data.get_node_count_from_node_type_id.node_type);
	let _ = graph.get_node_degree_from_node_id(data.get_node_degree_from_node_id.node_id);
	let _ = graph.get_node_degrees_mean();
	let _ = graph.get_node_degrees_median();
	let _ = graph.get_node_degrees_mode();
	let _ = graph.get_node_ids_and_edge_type_id_and_edge_weight_from_edge_id(data.get_node_ids_and_edge_type_id_and_edge_weight_from_edge_id.edge_id);
	let _ = graph.get_node_ids_and_edge_type_id_from_edge_id(data.get_node_ids_and_edge_type_id_from_edge_id.edge_id);
	let _ = graph.get_node_ids_from_edge_id(data.get_node_ids_from_edge_id.edge_id);
	let _ = graph.get_node_label_prediction_tuple_from_node_ids(data.get_node_label_prediction_tuple_from_node_ids.node_ids, data.get_node_label_prediction_tuple_from_node_ids.random_state, data.get_node_label_prediction_tuple_from_node_ids.include_central_node, data.get_node_label_prediction_tuple_from_node_ids.offset, data.get_node_label_prediction_tuple_from_node_ids.max_neighbours);
	let _ = graph.get_node_name_from_node_id(data.get_node_name_from_node_id.node_id);
	let _ = graph.get_node_type_counter();
	let _ = graph.get_node_type_counts_hashmap();
	let _ = graph.get_node_type_id_from_node_id(data.get_node_type_id_from_node_id.node_id);
	let _ = graph.get_node_type_ids();
	let _ = graph.get_node_type_ids_from_node_type_names(data.get_node_type_ids_from_node_type_names.node_types);
	let _ = graph.get_node_type_name_from_node_id(data.get_node_type_name_from_node_id.node_id);
	let _ = graph.get_node_type_name_from_node_type_id(data.get_node_type_name_from_node_type_id.node_type_id);
	let _ = graph.get_node_type_names();
	let _ = graph.get_node_type_names_from_node_type_ids(data.get_node_type_names_from_node_type_ids.node_type_ids);
	let _ = graph.get_selfloop_rate();
	let _ = graph.get_star_edge_names(data.get_star_edge_names.central_node, data.get_star_edge_names.removed_existing_edges, data.get_star_edge_names.star_points_nodes_set, data.get_star_edge_names.star_points_node_types_set);
	let _ = graph.get_star_edges(data.get_star_edges.central_node, data.get_star_edges.removed_existing_edges, data.get_star_edges.star_points_nodes_set, data.get_star_edges.star_points_node_types_set);
	let _ = graph.is_singleton_from_node_id(data.is_singleton_from_node_id.node_id);
	let _ = graph.is_trap_node_from_node_id(data.is_trap_node_from_node_id.node_id);
	let _ = graph.iter_edge_ids_from_node_ids(data.iter_edge_ids_from_node_ids.src, data.iter_edge_ids_from_node_ids.dst);
	let _ = graph.iter_edge_weights();
	let _ = graph.jaccard_index(data.jaccard_index.one, data.jaccard_index.two);
	let _ = graph.kfold(data.kfold.k, data.kfold.k_index, data.kfold.edge_types, data.kfold.random_state, data.kfold.verbose);
	let _ = graph.must_have_edge_types();
	let _ = graph.must_have_edge_weights();
	let _ = graph.must_have_node_types();
	let _ = graph.node_label_holdout(data.node_label_holdout.train_size, data.node_label_holdout.use_stratification, data.node_label_holdout.random_state);
	let _ = graph.par_iter_edge_weights();
	let _ = graph.random_holdout(data.random_holdout.random_state, data.random_holdout.train_size, data.random_holdout.include_all_edge_types, data.random_holdout.edge_types, data.random_holdout.min_number_overlaps, data.random_holdout.verbose);
	let _ = graph.random_subgraph(data.random_subgraph.random_state, data.random_subgraph.nodes_number, data.random_subgraph.verbose);
	let _ = graph.remove(data.remove.allow_nodes_set, data.remove.deny_nodes_set, data.remove.allow_node_types_set, data.remove.deny_node_types_set, data.remove.allow_edge_set, data.remove.deny_edge_set, data.remove.allow_edge_types_set, data.remove.deny_edge_types_set, data.remove.weights, data.remove.node_types, data.remove.edge_types, data.remove.singletons, data.remove.selfloops, data.remove.verbose);
	let _ = graph.remove_components(data.remove_components.node_names, data.remove_components.node_types, data.remove_components.edge_types, data.remove_components.minimum_component_size, data.remove_components.top_k_components, data.remove_components.verbose);
	let _ = graph.resource_allocation_index(data.resource_allocation_index.one, data.resource_allocation_index.two);
	let _ = graph.spanning_arborescence(data.spanning_arborescence.verbose);
	let _ = graph.textual_report(data.textual_report.verbose);
	let _ = graph.validate_edge_id(data.validate_edge_id.edge_id);
	let _ = graph.validate_node_id(data.validate_node_id.node_id);
	let mut graph = graph.set_all_edge_types(data.set_all_edge_types.edge_type)?;
	let mut graph = graph.set_all_node_types(data.set_all_node_types.node_type)?;

    Ok(())
}