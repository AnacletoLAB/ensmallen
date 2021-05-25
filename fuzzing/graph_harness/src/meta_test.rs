use super::*;
use arbitrary::Arbitrary;
use std::collections::{HashSet, HashMap};
use rayon::iter::ParallelIterator;

struct Rng{
    seed: u64
}

impl Rng {
    pub fn new() -> Rng {
        Rng{
            seed: 0xbad5eed ^ unsafe{core::arch::x86_64::_rdtsc()}
        }
    }

    pub fn next(&mut self) -> u64 {
        let mut x = self.seed;
        x ^= x << 17;
        x ^= x >> 7;
        x ^= x << 13;
        self.seed = x;
        x
    }
}

#[derive(Arbitrary, Debug, Clone)]
pub struct RandomSpanningArborescenceKruskalParams {
	pub random_state : EdgeT,
	pub undesired_edge_types : Option<HashSet<Option<EdgeTypeT>>>,
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
pub struct ParIterNodeTypesCooccurrenceMatrixParams {
	pub maximal_distance : Option<usize>,
	pub k1 : Option<f64>,
	pub b : Option<f64>,
	pub verbose : Option<bool>,
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
pub struct HasNodeTypeIdParams {
	pub node_type_id : NodeTypeT,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct HasEdgeTypeIdParams {
	pub edge_type_id : EdgeTypeT,
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
pub struct GetPreferentialAttachmentParams {
	pub one : NodeT,
	pub two : NodeT,
	pub normalize : bool,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct GetJaccardCoefficientParams {
	pub one : NodeT,
	pub two : NodeT,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct GetAdamicAdarIndexParams {
	pub one : NodeT,
	pub two : NodeT,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct GetResourceAllocationIndexParams {
	pub one : NodeT,
	pub two : NodeT,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct EnableParams {
	pub vector_sources : bool,
	pub vector_destinations : bool,
	pub vector_cumulative_node_degrees : bool,
	pub cache_size : Option<f64>,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct GetNumberOfTrianglesParams {
	pub normalize : Option<bool>,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct GetNumberOfTrianglesPerNodeParams {
	pub normalize : Option<bool>,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct GetUnweightedEccentricityFromNodeIdParams {
	pub node_id : NodeT,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct GetWeightedEccentricityFromNodeIdParams {
	pub node_id : NodeT,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct GetBreathFirstSearchFromNodeIdsParams {
	pub src_node_id : NodeT,
	pub maybe_dst_node_id : Option<NodeT>,
	pub maybe_dst_node_ids : Option<Vec<u32>>,
	pub compute_distances : Option<bool>,
	pub compute_predecessors : Option<bool>,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct GetDijkstraFromNodeIdsParams {
	pub src_node_id : NodeT,
	pub maybe_dst_node_id : Option<NodeT>,
	pub maybe_dst_node_ids : Option<Vec<u32>>,
	pub compute_predecessors : Option<bool>,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct GetUnweightedDiameterParams {
	pub ignore_infinity : Option<bool>,
	pub verbose : Option<bool>,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct GetWeightedDiameterParams {
	pub ignore_infinity : Option<bool>,
	pub verbose : Option<bool>,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct IterEdgeIdsFromNodeIdsParams {
	pub src : NodeT,
	pub dst : NodeT,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct IterEdgeNodeIdsAndEdgeTypeIdFromEdgeTypeIdParams {
	pub edge_type_id : Option<EdgeTypeT>,
	pub directed : bool,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct IterNodeIdsAndNodeTypeIdsFromNodeTypeIdParams {
	pub node_type_id : Option<NodeTypeT>,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct IterNodeNamesAndNodeTypeNamesFromNodeTypeIdParams {
	pub node_type_id : Option<NodeTypeT>,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct IterEdgeNodeNamesAndEdgeTypeNameFromEdgeTypeIdParams {
	pub edge_type_id : Option<EdgeTypeT>,
	pub directed : bool,
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
pub struct GetNodeTypeNamesFromNodeIdParams {
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
pub struct GetEdgeNodeNamesFromEdgeNodeIdsParams {
	pub edge_node_ids : Vec<(NodeT, NodeT)>,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct GetEdgeCountFromEdgeTypeIdParams {
	pub edge_type_id : Option<EdgeTypeT>,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct GetNodeCountFromNodeTypeIdParams {
	pub node_type_id : Option<NodeTypeT>,
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
	pub edge_type_names : Vec<Option<String>>,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct GetNodeTypeIdsFromNodeTypeNamesParams {
	pub node_type_names : Vec<Option<String>>,
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
pub struct FilterFromIdsParams {
	pub node_ids_to_keep : Option<Vec<NodeT>>,
	pub node_ids_to_filter : Option<Vec<NodeT>>,
	pub node_type_ids_to_keep : Option<Vec<Option<Vec<NodeTypeT>>>>,
	pub node_type_ids_to_filter : Option<Vec<Option<Vec<NodeTypeT>>>>,
	pub node_type_id_to_keep : Option<Vec<Option<NodeTypeT>>>,
	pub node_type_id_to_filter : Option<Vec<Option<NodeTypeT>>>,
	pub edge_ids_to_keep : Option<Vec<EdgeT>>,
	pub edge_ids_to_filter : Option<Vec<EdgeT>>,
	pub edge_node_ids_to_keep : Option<Vec<(NodeT, NodeT)>>,
	pub edge_node_ids_to_filter : Option<Vec<(NodeT, NodeT)>>,
	pub edge_type_ids_to_keep : Option<Vec<Option<EdgeTypeT>>>,
	pub edge_type_ids_to_filter : Option<Vec<Option<EdgeTypeT>>>,
	pub min_edge_weight : Option<WeightT>,
	pub max_edge_weight : Option<WeightT>,
	pub filter_singleton_nodes : bool,
	pub filter_singleton_nodes_with_selfloop : bool,
	pub filter_selfloops : bool,
	pub filter_parallel_edges : bool,
	pub verbose : Option<bool>,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct DropUnknownNodeTypesParams {
	pub verbose : Option<bool>,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct DropUnknownEdgeTypesParams {
	pub verbose : Option<bool>,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct DropSingletonNodesParams {
	pub verbose : Option<bool>,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct DropSingletonNodesWithSelfloopsParams {
	pub verbose : Option<bool>,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct DropSelfloopsParams {
	pub verbose : Option<bool>,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct DropParallelEdgesParams {
	pub verbose : Option<bool>,
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
pub struct ValidateNodeTypeIdParams {
	pub node_type_id : Option<NodeTypeT>,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct ValidateNodeTypeIdsParams {
	pub node_type_ids : Vec<Option<NodeTypeT>>,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct ValidateEdgeTypeIdParams {
	pub edge_type_id : Option<EdgeTypeT>,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct ValidateEdgeTypeIdsParams {
	pub edge_type_ids : Vec<Option<EdgeTypeT>>,
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
pub struct ParIterUnweightedClosenessCentralityParams {
	pub verbose : Option<bool>,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct ParIterWeightedClosenessCentralityParams {
	pub verbose : Option<bool>,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct GetUnweightedClosenessCentralityParams {
	pub verbose : Option<bool>,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct GetWeightedClosenessCentralityParams {
	pub verbose : Option<bool>,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct ParIterUnweightedHarmonicCentralityParams {
	pub verbose : Option<bool>,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct ParIterWeightedHarmonicCentralityParams {
	pub verbose : Option<bool>,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct GetUnweightedHarmonicCentralityParams {
	pub verbose : Option<bool>,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct GetWeightedHarmonicCentralityParams {
	pub verbose : Option<bool>,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct GetStressCentralityParams {
	pub normalize : Option<bool>,
	pub verbose : Option<bool>,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct GetBetweennessCentralityParams {
	pub normalize : Option<bool>,
	pub verbose : Option<bool>,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct GetUnweightedEigenvectorCentralityParams {
	pub maximum_iterations_number : Option<u8>,
	pub tollerance : Option<f64>,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct GetWeightedEigenvectorCentralityParams {
	pub maximum_iterations_number : Option<u8>,
	pub tollerance : Option<f64>,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct SetNameParams {
	pub name : String,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct SetInplaceAllEdgeTypesParams {
	pub edge_type : String,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct SetAllEdgeTypesParams {
	pub edge_type : String,
	pub verbose : Option<bool>,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct SetInplaceAllNodeTypesParams {
	pub node_type : String,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct SetAllNodeTypesParams {
	pub node_type : String,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct RemoveInplaceNodeTypeIdParams {
	pub node_type_id : Vec<NodeTypeT>,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct RemoveInplaceEdgeTypeIdParams {
	pub edge_type_id : Vec<EdgeTypeT>,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct RemoveNodeTypeIdParams {
	pub node_type_id : NodeTypeT,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct RemoveEdgeTypeIdParams {
	pub edge_type_id : EdgeTypeT,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct RemoveEdgeTypesParams {
	pub verbose : Option<bool>,
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
pub struct ReplaceParams {
	pub node_name_mapping : Option<HashMap<String, String>>,
	pub node_type_name_mapping : Option<HashMap<String, String>>,
	pub node_type_names_mapping : Option<HashMap<Option<Vec<String>>, Option<Vec<String>>>>,
	pub edge_type_name_mapping : Option<HashMap<Option<String>, Option<String>>>,
	pub verbose : bool,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct ReplaceUnknownNodeTypesWithNodeTypeNameParams {
	pub node_type_names : Vec<String>,
	pub verbose : bool,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct ReplaceUnknownEdgeTypesWithEdgeTypeNameParams {
	pub edge_type_name : String,
	pub verbose : bool,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct GetNodeReportFromNodeIdParams {
	pub node_id : NodeT,
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
pub struct GetSourceNodeIdsParams {
	pub directed : bool,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct GetSourceNamesParams {
	pub directed : bool,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct GetDestinationNodeIdsParams {
	pub directed : bool,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct GetDestinationNamesParams {
	pub directed : bool,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct GetEdgeNodeIdsParams {
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
pub struct IsConnectedParams {
	pub verbose : bool,
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
	pub par_iter_node_types_cooccurrence_matrix: ParIterNodeTypesCooccurrenceMatrixParams,
	pub is_singleton_from_node_id: IsSingletonFromNodeIdParams,
	pub is_singleton_with_selfloops_from_node_id: IsSingletonWithSelfloopsFromNodeIdParams,
	pub has_node_type_id: HasNodeTypeIdParams,
	pub has_edge_type_id: HasEdgeTypeIdParams,
	pub has_edge_from_node_ids: HasEdgeFromNodeIdsParams,
	pub has_edge_from_node_ids_and_edge_type_id: HasEdgeFromNodeIdsAndEdgeTypeIdParams,
	pub is_trap_node_from_node_id: IsTrapNodeFromNodeIdParams,
	pub get_preferential_attachment: GetPreferentialAttachmentParams,
	pub get_jaccard_coefficient: GetJaccardCoefficientParams,
	pub get_adamic_adar_index: GetAdamicAdarIndexParams,
	pub get_resource_allocation_index: GetResourceAllocationIndexParams,
	pub enable: EnableParams,
	pub get_number_of_triangles: GetNumberOfTrianglesParams,
	pub get_number_of_triangles_per_node: GetNumberOfTrianglesPerNodeParams,
	pub get_unweighted_eccentricity_from_node_id: GetUnweightedEccentricityFromNodeIdParams,
	pub get_weighted_eccentricity_from_node_id: GetWeightedEccentricityFromNodeIdParams,
	pub get_breath_first_search_from_node_ids: GetBreathFirstSearchFromNodeIdsParams,
	pub get_dijkstra_from_node_ids: GetDijkstraFromNodeIdsParams,
	pub get_unweighted_diameter: GetUnweightedDiameterParams,
	pub get_weighted_diameter: GetWeightedDiameterParams,
	pub iter_edge_ids_from_node_ids: IterEdgeIdsFromNodeIdsParams,
	pub iter_edge_node_ids_and_edge_type_id_from_edge_type_id: IterEdgeNodeIdsAndEdgeTypeIdFromEdgeTypeIdParams,
	pub iter_node_ids_and_node_type_ids_from_node_type_id: IterNodeIdsAndNodeTypeIdsFromNodeTypeIdParams,
	pub iter_node_names_and_node_type_names_from_node_type_id: IterNodeNamesAndNodeTypeNamesFromNodeTypeIdParams,
	pub iter_edge_node_names_and_edge_type_name_from_edge_type_id: IterEdgeNodeNamesAndEdgeTypeNameFromEdgeTypeIdParams,
	pub get_node_ids_from_edge_id: GetNodeIdsFromEdgeIdParams,
	pub get_edge_id_from_node_ids: GetEdgeIdFromNodeIdsParams,
	pub get_node_ids_and_edge_type_id_from_edge_id: GetNodeIdsAndEdgeTypeIdFromEdgeIdParams,
	pub get_node_ids_and_edge_type_id_and_edge_weight_from_edge_id: GetNodeIdsAndEdgeTypeIdAndEdgeWeightFromEdgeIdParams,
	pub get_top_k_central_node_ids: GetTopKCentralNodeIdsParams,
	pub get_node_degree_from_node_id: GetNodeDegreeFromNodeIdParams,
	pub get_top_k_central_node_names: GetTopKCentralNodeNamesParams,
	pub get_node_type_id_from_node_id: GetNodeTypeIdFromNodeIdParams,
	pub get_edge_type_id_from_edge_id: GetEdgeTypeIdFromEdgeIdParams,
	pub get_node_type_names_from_node_id: GetNodeTypeNamesFromNodeIdParams,
	pub get_edge_type_name_from_edge_id: GetEdgeTypeNameFromEdgeIdParams,
	pub get_edge_type_name_from_edge_type_id: GetEdgeTypeNameFromEdgeTypeIdParams,
	pub get_edge_weight_from_edge_id: GetEdgeWeightFromEdgeIdParams,
	pub get_edge_weight_from_node_ids: GetEdgeWeightFromNodeIdsParams,
	pub get_edge_weight_from_node_ids_and_edge_type_id: GetEdgeWeightFromNodeIdsAndEdgeTypeIdParams,
	pub get_node_name_from_node_id: GetNodeNameFromNodeIdParams,
	pub get_edge_node_names_from_edge_node_ids: GetEdgeNodeNamesFromEdgeNodeIdsParams,
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
	pub filter_from_ids: FilterFromIdsParams,
	pub drop_unknown_node_types: DropUnknownNodeTypesParams,
	pub drop_unknown_edge_types: DropUnknownEdgeTypesParams,
	pub drop_singleton_nodes: DropSingletonNodesParams,
	pub drop_singleton_nodes_with_selfloops: DropSingletonNodesWithSelfloopsParams,
	pub drop_selfloops: DropSelfloopsParams,
	pub drop_parallel_edges: DropParallelEdgesParams,
	pub validate_node_id: ValidateNodeIdParams,
	pub validate_edge_id: ValidateEdgeIdParams,
	pub validate_node_type_id: ValidateNodeTypeIdParams,
	pub validate_node_type_ids: ValidateNodeTypeIdsParams,
	pub validate_edge_type_id: ValidateEdgeTypeIdParams,
	pub validate_edge_type_ids: ValidateEdgeTypeIdsParams,
	pub remove_components: RemoveComponentsParams,
	pub par_iter_unweighted_closeness_centrality: ParIterUnweightedClosenessCentralityParams,
	pub par_iter_weighted_closeness_centrality: ParIterWeightedClosenessCentralityParams,
	pub get_unweighted_closeness_centrality: GetUnweightedClosenessCentralityParams,
	pub get_weighted_closeness_centrality: GetWeightedClosenessCentralityParams,
	pub par_iter_unweighted_harmonic_centrality: ParIterUnweightedHarmonicCentralityParams,
	pub par_iter_weighted_harmonic_centrality: ParIterWeightedHarmonicCentralityParams,
	pub get_unweighted_harmonic_centrality: GetUnweightedHarmonicCentralityParams,
	pub get_weighted_harmonic_centrality: GetWeightedHarmonicCentralityParams,
	pub get_stress_centrality: GetStressCentralityParams,
	pub get_betweenness_centrality: GetBetweennessCentralityParams,
	pub get_unweighted_eigenvector_centrality: GetUnweightedEigenvectorCentralityParams,
	pub get_weighted_eigenvector_centrality: GetWeightedEigenvectorCentralityParams,
	pub set_name: SetNameParams,
	pub set_inplace_all_edge_types: SetInplaceAllEdgeTypesParams,
	pub set_all_edge_types: SetAllEdgeTypesParams,
	pub set_inplace_all_node_types: SetInplaceAllNodeTypesParams,
	pub set_all_node_types: SetAllNodeTypesParams,
	pub remove_inplace_node_type_id: RemoveInplaceNodeTypeIdParams,
	pub remove_inplace_edge_type_id: RemoveInplaceEdgeTypeIdParams,
	pub remove_node_type_id: RemoveNodeTypeIdParams,
	pub remove_edge_type_id: RemoveEdgeTypeIdParams,
	pub remove_edge_types: RemoveEdgeTypesParams,
	pub encode_edge: EncodeEdgeParams,
	pub decode_edge: DecodeEdgeParams,
	pub get_bipartite_edges: GetBipartiteEdgesParams,
	pub get_bipartite_edge_names: GetBipartiteEdgeNamesParams,
	pub get_star_edges: GetStarEdgesParams,
	pub get_star_edge_names: GetStarEdgeNamesParams,
	pub get_clique_edges: GetCliqueEdgesParams,
	pub get_clique_edge_names: GetCliqueEdgeNamesParams,
	pub replace: ReplaceParams,
	pub replace_unknown_node_types_with_node_type_name: ReplaceUnknownNodeTypesWithNodeTypeNameParams,
	pub replace_unknown_edge_types_with_edge_type_name: ReplaceUnknownEdgeTypesWithEdgeTypeNameParams,
	pub get_node_report_from_node_id: GetNodeReportFromNodeIdParams,
	pub textual_report: TextualReportParams,
	pub get_connected_components_number: GetConnectedComponentsNumberParams,
	pub get_source_node_ids: GetSourceNodeIdsParams,
	pub get_source_names: GetSourceNamesParams,
	pub get_destination_node_ids: GetDestinationNodeIdsParams,
	pub get_destination_names: GetDestinationNamesParams,
	pub get_edge_node_ids: GetEdgeNodeIdsParams,
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
	pub is_connected: IsConnectedParams,
	pub connected_holdout: ConnectedHoldoutParams,
	pub random_holdout: RandomHoldoutParams,
	pub node_label_holdout: NodeLabelHoldoutParams,
	pub edge_label_holdout: EdgeLabelHoldoutParams,
	pub random_subgraph: RandomSubgraphParams,
	pub kfold: KfoldParams,
    pub from_vec: FromVecHarnessParams,
}

pub fn meta_test(data: MetaParams) -> Result<(), String> {
    let panic_handler_data_before_load = data.clone();
    let data_copy_for_tests = data.clone();
    std::panic::set_hook(Box::new(move |info| {
        handle_panics_meta_test(Some(info), panic_handler_data_before_load.clone(), None);
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
        data.from_vec.verbose,
    )?;

    let mut rng = Rng::new();
    let mut trace = Vec::new();
    for _ in 0..10 {
        let data_for_current_test = data_copy_for_tests.clone();
        let data_for_panic_handler = data_copy_for_tests.clone();
        match rng.next() % 289 {
			0 => {
				trace.push(format!("random_spanning_arborescence_kruskal(random_state = {:?}, undesired_edge_types = {:?}, verbose = {:?})", data_for_current_test.random_spanning_arborescence_kruskal.random_state, &data_for_current_test.random_spanning_arborescence_kruskal.undesired_edge_types, data_for_current_test.random_spanning_arborescence_kruskal.verbose));
				
				let g_copy = graph.clone();
				let trace2 = trace.clone();
				std::panic::set_hook(Box::new(move |info| {
					handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
				}));
				graph.random_spanning_arborescence_kruskal(data_for_current_test.random_spanning_arborescence_kruskal.random_state, &data_for_current_test.random_spanning_arborescence_kruskal.undesired_edge_types, data_for_current_test.random_spanning_arborescence_kruskal.verbose);
			},
			1 => {
				trace.push(format!("spanning_arborescence_kruskal(verbose = {:?})", data_for_current_test.spanning_arborescence_kruskal.verbose));
				
				let g_copy = graph.clone();
				let trace2 = trace.clone();
				std::panic::set_hook(Box::new(move |info| {
					handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
				}));
				graph.spanning_arborescence_kruskal(data_for_current_test.spanning_arborescence_kruskal.verbose);
			},
			2 => {
				trace.push(format!("spanning_arborescence(verbose = {:?})", data_for_current_test.spanning_arborescence.verbose));
				
				let g_copy = graph.clone();
				let trace2 = trace.clone();
				std::panic::set_hook(Box::new(move |info| {
					handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
				}));
				let _ = graph.spanning_arborescence(data_for_current_test.spanning_arborescence.verbose);
			},
			3 => {
				trace.push(format!("connected_components(verbose = {:?})", data_for_current_test.connected_components.verbose));
				
				let g_copy = graph.clone();
				let trace2 = trace.clone();
				std::panic::set_hook(Box::new(move |info| {
					handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
				}));
				let _ = graph.connected_components(data_for_current_test.connected_components.verbose);
			},
			4 => {
				trace.push(format!("get_node_label_prediction_tuple_from_node_ids(node_ids = {:?}, random_state = {:?}, include_central_node = {:?}, offset = {:?}, max_neighbours = {:?})", data_for_current_test.get_node_label_prediction_tuple_from_node_ids.node_ids, data_for_current_test.get_node_label_prediction_tuple_from_node_ids.random_state, data_for_current_test.get_node_label_prediction_tuple_from_node_ids.include_central_node, data_for_current_test.get_node_label_prediction_tuple_from_node_ids.offset, data_for_current_test.get_node_label_prediction_tuple_from_node_ids.max_neighbours));
				
				let g_copy = graph.clone();
				let trace2 = trace.clone();
				std::panic::set_hook(Box::new(move |info| {
					handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
				}));
				let _ = graph.get_node_label_prediction_tuple_from_node_ids(data_for_current_test.get_node_label_prediction_tuple_from_node_ids.node_ids, data_for_current_test.get_node_label_prediction_tuple_from_node_ids.random_state, data_for_current_test.get_node_label_prediction_tuple_from_node_ids.include_central_node, data_for_current_test.get_node_label_prediction_tuple_from_node_ids.offset, data_for_current_test.get_node_label_prediction_tuple_from_node_ids.max_neighbours);
			},
			5 => {
				trace.push(format!("par_iter_node_types_cooccurrence_matrix(maximal_distance = {:?}, k1 = {:?}, b = {:?}, verbose = {:?})", data_for_current_test.par_iter_node_types_cooccurrence_matrix.maximal_distance, data_for_current_test.par_iter_node_types_cooccurrence_matrix.k1, data_for_current_test.par_iter_node_types_cooccurrence_matrix.b, data_for_current_test.par_iter_node_types_cooccurrence_matrix.verbose));
				
				let g_copy = graph.clone();
				let trace2 = trace.clone();
				std::panic::set_hook(Box::new(move |info| {
					handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
				}));
				let _ = graph.par_iter_node_types_cooccurrence_matrix(data_for_current_test.par_iter_node_types_cooccurrence_matrix.maximal_distance, data_for_current_test.par_iter_node_types_cooccurrence_matrix.k1, data_for_current_test.par_iter_node_types_cooccurrence_matrix.b, data_for_current_test.par_iter_node_types_cooccurrence_matrix.verbose);
			},
			6 => {
				trace.push(format!("is_singleton_from_node_id(node_id = {:?})", data_for_current_test.is_singleton_from_node_id.node_id));
				
				let g_copy = graph.clone();
				let trace2 = trace.clone();
				std::panic::set_hook(Box::new(move |info| {
					handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
				}));
				let _ = graph.is_singleton_from_node_id(data_for_current_test.is_singleton_from_node_id.node_id);
			},
			7 => {
				trace.push(format!("is_singleton_with_selfloops_from_node_id(node_id = {:?})", data_for_current_test.is_singleton_with_selfloops_from_node_id.node_id));
				
				let g_copy = graph.clone();
				let trace2 = trace.clone();
				std::panic::set_hook(Box::new(move |info| {
					handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
				}));
				graph.is_singleton_with_selfloops_from_node_id(data_for_current_test.is_singleton_with_selfloops_from_node_id.node_id);
			},
			8 => {
				trace.push(format!("has_node_type_id(node_type_id = {:?})", data_for_current_test.has_node_type_id.node_type_id));
				
				let g_copy = graph.clone();
				let trace2 = trace.clone();
				std::panic::set_hook(Box::new(move |info| {
					handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
				}));
				graph.has_node_type_id(data_for_current_test.has_node_type_id.node_type_id);
			},
			9 => {
				trace.push(format!("has_edge_type_id(edge_type_id = {:?})", data_for_current_test.has_edge_type_id.edge_type_id));
				
				let g_copy = graph.clone();
				let trace2 = trace.clone();
				std::panic::set_hook(Box::new(move |info| {
					handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
				}));
				graph.has_edge_type_id(data_for_current_test.has_edge_type_id.edge_type_id);
			},
			10 => {
				trace.push(format!("has_edge_from_node_ids(src = {:?}, dst = {:?})", data_for_current_test.has_edge_from_node_ids.src, data_for_current_test.has_edge_from_node_ids.dst));
				
				let g_copy = graph.clone();
				let trace2 = trace.clone();
				std::panic::set_hook(Box::new(move |info| {
					handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
				}));
				graph.has_edge_from_node_ids(data_for_current_test.has_edge_from_node_ids.src, data_for_current_test.has_edge_from_node_ids.dst);
			},
			11 => {
				trace.push(format!("has_edge_from_node_ids_and_edge_type_id(src = {:?}, dst = {:?}, edge_type = {:?})", data_for_current_test.has_edge_from_node_ids_and_edge_type_id.src, data_for_current_test.has_edge_from_node_ids_and_edge_type_id.dst, data_for_current_test.has_edge_from_node_ids_and_edge_type_id.edge_type));
				
				let g_copy = graph.clone();
				let trace2 = trace.clone();
				std::panic::set_hook(Box::new(move |info| {
					handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
				}));
				graph.has_edge_from_node_ids_and_edge_type_id(data_for_current_test.has_edge_from_node_ids_and_edge_type_id.src, data_for_current_test.has_edge_from_node_ids_and_edge_type_id.dst, data_for_current_test.has_edge_from_node_ids_and_edge_type_id.edge_type);
			},
			12 => {
				trace.push(format!("is_trap_node_from_node_id(node_id = {:?})", data_for_current_test.is_trap_node_from_node_id.node_id));
				
				let g_copy = graph.clone();
				let trace2 = trace.clone();
				std::panic::set_hook(Box::new(move |info| {
					handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
				}));
				let _ = graph.is_trap_node_from_node_id(data_for_current_test.is_trap_node_from_node_id.node_id);
			},
			13 => {
				trace.push(format!("get_preferential_attachment(one = {:?}, two = {:?}, normalize = {:?})", data_for_current_test.get_preferential_attachment.one, data_for_current_test.get_preferential_attachment.two, data_for_current_test.get_preferential_attachment.normalize));
				
				let g_copy = graph.clone();
				let trace2 = trace.clone();
				std::panic::set_hook(Box::new(move |info| {
					handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
				}));
				let _ = graph.get_preferential_attachment(data_for_current_test.get_preferential_attachment.one, data_for_current_test.get_preferential_attachment.two, data_for_current_test.get_preferential_attachment.normalize);
			},
			14 => {
				trace.push(format!("get_jaccard_coefficient(one = {:?}, two = {:?})", data_for_current_test.get_jaccard_coefficient.one, data_for_current_test.get_jaccard_coefficient.two));
				
				let g_copy = graph.clone();
				let trace2 = trace.clone();
				std::panic::set_hook(Box::new(move |info| {
					handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
				}));
				let _ = graph.get_jaccard_coefficient(data_for_current_test.get_jaccard_coefficient.one, data_for_current_test.get_jaccard_coefficient.two);
			},
			15 => {
				trace.push(format!("get_adamic_adar_index(one = {:?}, two = {:?})", data_for_current_test.get_adamic_adar_index.one, data_for_current_test.get_adamic_adar_index.two));
				
				let g_copy = graph.clone();
				let trace2 = trace.clone();
				std::panic::set_hook(Box::new(move |info| {
					handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
				}));
				let _ = graph.get_adamic_adar_index(data_for_current_test.get_adamic_adar_index.one, data_for_current_test.get_adamic_adar_index.two);
			},
			16 => {
				trace.push(format!("get_resource_allocation_index(one = {:?}, two = {:?})", data_for_current_test.get_resource_allocation_index.one, data_for_current_test.get_resource_allocation_index.two));
				
				let g_copy = graph.clone();
				let trace2 = trace.clone();
				std::panic::set_hook(Box::new(move |info| {
					handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
				}));
				let _ = graph.get_resource_allocation_index(data_for_current_test.get_resource_allocation_index.one, data_for_current_test.get_resource_allocation_index.two);
			},
			17 => {
				trace.push(format!("enable(vector_sources = {:?}, vector_destinations = {:?}, vector_cumulative_node_degrees = {:?}, cache_size = {:?})", data_for_current_test.enable.vector_sources, data_for_current_test.enable.vector_destinations, data_for_current_test.enable.vector_cumulative_node_degrees, data_for_current_test.enable.cache_size));
				
				let g_copy = graph.clone();
				let trace2 = trace.clone();
				std::panic::set_hook(Box::new(move |info| {
					handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
				}));
				let _ = graph.enable(data_for_current_test.enable.vector_sources, data_for_current_test.enable.vector_destinations, data_for_current_test.enable.vector_cumulative_node_degrees, data_for_current_test.enable.cache_size);
			},
			18 => {
				trace.push(format!("disable_all()", ));
				
				let g_copy = graph.clone();
				let trace2 = trace.clone();
				std::panic::set_hook(Box::new(move |info| {
					handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
				}));
				graph.disable_all();
			},
			19 => {
				trace.push(format!("get_number_of_triangles(normalize = {:?})", data_for_current_test.get_number_of_triangles.normalize));
				
				let g_copy = graph.clone();
				let trace2 = trace.clone();
				std::panic::set_hook(Box::new(move |info| {
					handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
				}));
				graph.get_number_of_triangles(data_for_current_test.get_number_of_triangles.normalize);
			},
			20 => {
				trace.push(format!("get_triads_number()", ));
				
				let g_copy = graph.clone();
				let trace2 = trace.clone();
				std::panic::set_hook(Box::new(move |info| {
					handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
				}));
				graph.get_triads_number();
			},
			21 => {
				trace.push(format!("get_transitivity()", ));
				
				let g_copy = graph.clone();
				let trace2 = trace.clone();
				std::panic::set_hook(Box::new(move |info| {
					handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
				}));
				graph.get_transitivity();
			},
			22 => {
				trace.push(format!("get_number_of_triangles_per_node(normalize = {:?})", data_for_current_test.get_number_of_triangles_per_node.normalize));
				
				let g_copy = graph.clone();
				let trace2 = trace.clone();
				std::panic::set_hook(Box::new(move |info| {
					handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
				}));
				graph.get_number_of_triangles_per_node(data_for_current_test.get_number_of_triangles_per_node.normalize);
			},
			23 => {
				trace.push(format!("iter_clustering_coefficient_per_node()", ));
				
				let g_copy = graph.clone();
				let trace2 = trace.clone();
				std::panic::set_hook(Box::new(move |info| {
					handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
				}));
				let _ = graph.iter_clustering_coefficient_per_node().collect::<Vec<_>>();
			},
			24 => {
				trace.push(format!("get_clustering_coefficient_per_node()", ));
				
				let g_copy = graph.clone();
				let trace2 = trace.clone();
				std::panic::set_hook(Box::new(move |info| {
					handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
				}));
				graph.get_clustering_coefficient_per_node();
			},
			25 => {
				trace.push(format!("get_clustering_coefficient()", ));
				
				let g_copy = graph.clone();
				let trace2 = trace.clone();
				std::panic::set_hook(Box::new(move |info| {
					handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
				}));
				graph.get_clustering_coefficient();
			},
			26 => {
				trace.push(format!("get_average_clustering_coefficient()", ));
				
				let g_copy = graph.clone();
				let trace2 = trace.clone();
				std::panic::set_hook(Box::new(move |info| {
					handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
				}));
				graph.get_average_clustering_coefficient();
			},
			27 => {
				trace.push(format!("get_unweighted_eccentricity_from_node_id(node_id = {:?})", data_for_current_test.get_unweighted_eccentricity_from_node_id.node_id));
				
				let g_copy = graph.clone();
				let trace2 = trace.clone();
				std::panic::set_hook(Box::new(move |info| {
					handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
				}));
				let _ = graph.get_unweighted_eccentricity_from_node_id(data_for_current_test.get_unweighted_eccentricity_from_node_id.node_id);
			},
			28 => {
				trace.push(format!("get_weighted_eccentricity_from_node_id(node_id = {:?})", data_for_current_test.get_weighted_eccentricity_from_node_id.node_id));
				
				let g_copy = graph.clone();
				let trace2 = trace.clone();
				std::panic::set_hook(Box::new(move |info| {
					handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
				}));
				let _ = graph.get_weighted_eccentricity_from_node_id(data_for_current_test.get_weighted_eccentricity_from_node_id.node_id);
			},
			29 => {
				trace.push(format!("get_breath_first_search_from_node_ids(src_node_id = {:?}, maybe_dst_node_id = {:?}, maybe_dst_node_ids = {:?}, compute_distances = {:?}, compute_predecessors = {:?})", data_for_current_test.get_breath_first_search_from_node_ids.src_node_id, data_for_current_test.get_breath_first_search_from_node_ids.maybe_dst_node_id, data_for_current_test.get_breath_first_search_from_node_ids.maybe_dst_node_ids, data_for_current_test.get_breath_first_search_from_node_ids.compute_distances, data_for_current_test.get_breath_first_search_from_node_ids.compute_predecessors));
				
				let g_copy = graph.clone();
				let trace2 = trace.clone();
				std::panic::set_hook(Box::new(move |info| {
					handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
				}));
				let _ = graph.get_breath_first_search_from_node_ids(data_for_current_test.get_breath_first_search_from_node_ids.src_node_id, data_for_current_test.get_breath_first_search_from_node_ids.maybe_dst_node_id, data_for_current_test.get_breath_first_search_from_node_ids.maybe_dst_node_ids.map(|x| {
					x.into_iter().collect()
				}), data_for_current_test.get_breath_first_search_from_node_ids.compute_distances, data_for_current_test.get_breath_first_search_from_node_ids.compute_predecessors);
			},
			30 => {
				trace.push(format!("get_dijkstra_from_node_ids(src_node_id = {:?}, maybe_dst_node_id = {:?}, maybe_dst_node_ids = {:?}, compute_predecessors = {:?})", data_for_current_test.get_dijkstra_from_node_ids.src_node_id, data_for_current_test.get_dijkstra_from_node_ids.maybe_dst_node_id, data_for_current_test.get_dijkstra_from_node_ids.maybe_dst_node_ids, data_for_current_test.get_dijkstra_from_node_ids.compute_predecessors));
				
				let g_copy = graph.clone();
				let trace2 = trace.clone();
				std::panic::set_hook(Box::new(move |info| {
					handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
				}));
				let _ = graph.get_dijkstra_from_node_ids(data_for_current_test.get_dijkstra_from_node_ids.src_node_id, data_for_current_test.get_dijkstra_from_node_ids.maybe_dst_node_id, data_for_current_test.get_dijkstra_from_node_ids.maybe_dst_node_ids.map(|x| x.into_iter().collect()), data_for_current_test.get_dijkstra_from_node_ids.compute_predecessors);
			},
			31 => {
				trace.push(format!("get_unweighted_diameter(ignore_infinity = {:?}, verbose = {:?})", data_for_current_test.get_unweighted_diameter.ignore_infinity, data_for_current_test.get_unweighted_diameter.verbose));
				
				let g_copy = graph.clone();
				let trace2 = trace.clone();
				std::panic::set_hook(Box::new(move |info| {
					handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
				}));
				let _ = graph.get_unweighted_diameter(data_for_current_test.get_unweighted_diameter.ignore_infinity, data_for_current_test.get_unweighted_diameter.verbose);
			},
			32 => {
				trace.push(format!("get_weighted_diameter(ignore_infinity = {:?}, verbose = {:?})", data_for_current_test.get_weighted_diameter.ignore_infinity, data_for_current_test.get_weighted_diameter.verbose));
				
				let g_copy = graph.clone();
				let trace2 = trace.clone();
				std::panic::set_hook(Box::new(move |info| {
					handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
				}));
				let _ = graph.get_weighted_diameter(data_for_current_test.get_weighted_diameter.ignore_infinity, data_for_current_test.get_weighted_diameter.verbose);
			},
			33 => {
				trace.push(format!("iter_edge_ids_from_node_ids(src = {:?}, dst = {:?})", data_for_current_test.iter_edge_ids_from_node_ids.src, data_for_current_test.iter_edge_ids_from_node_ids.dst));
				
				let g_copy = graph.clone();
				let trace2 = trace.clone();
				std::panic::set_hook(Box::new(move |info| {
					handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
				}));
				let _ = graph.iter_edge_ids_from_node_ids(data_for_current_test.iter_edge_ids_from_node_ids.src, data_for_current_test.iter_edge_ids_from_node_ids.dst);
			},
			34 => {
				trace.push(format!("iter_edge_node_ids_and_edge_type_id_from_edge_type_id(edge_type_id = {:?}, directed = {:?})", data_for_current_test.iter_edge_node_ids_and_edge_type_id_from_edge_type_id.edge_type_id, data_for_current_test.iter_edge_node_ids_and_edge_type_id_from_edge_type_id.directed));
				
				let g_copy = graph.clone();
				let trace2 = trace.clone();
				std::panic::set_hook(Box::new(move |info| {
					handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
				}));
				let _ = graph.iter_edge_node_ids_and_edge_type_id_from_edge_type_id(data_for_current_test.iter_edge_node_ids_and_edge_type_id_from_edge_type_id.edge_type_id, data_for_current_test.iter_edge_node_ids_and_edge_type_id_from_edge_type_id.directed);
			},
			35 => {
				trace.push(format!("iter_node_ids_and_node_type_ids_from_node_type_id(node_type_id = {:?})", data_for_current_test.iter_node_ids_and_node_type_ids_from_node_type_id.node_type_id));
				
				let g_copy = graph.clone();
				let trace2 = trace.clone();
				std::panic::set_hook(Box::new(move |info| {
					handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
				}));
				let _ = graph.iter_node_ids_and_node_type_ids_from_node_type_id(data_for_current_test.iter_node_ids_and_node_type_ids_from_node_type_id.node_type_id);
			},
			36 => {
				trace.push(format!("iter_node_names_and_node_type_names_from_node_type_id(node_type_id = {:?})", data_for_current_test.iter_node_names_and_node_type_names_from_node_type_id.node_type_id));
				
				let g_copy = graph.clone();
				let trace2 = trace.clone();
				std::panic::set_hook(Box::new(move |info| {
					handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
				}));
				let _ = graph.iter_node_names_and_node_type_names_from_node_type_id(data_for_current_test.iter_node_names_and_node_type_names_from_node_type_id.node_type_id);
			},
			37 => {
				trace.push(format!("iter_edge_node_names_and_edge_type_name_from_edge_type_id(edge_type_id = {:?}, directed = {:?})", data_for_current_test.iter_edge_node_names_and_edge_type_name_from_edge_type_id.edge_type_id, data_for_current_test.iter_edge_node_names_and_edge_type_name_from_edge_type_id.directed));
				
				let g_copy = graph.clone();
				let trace2 = trace.clone();
				std::panic::set_hook(Box::new(move |info| {
					handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
				}));
				let _ = graph.iter_edge_node_names_and_edge_type_name_from_edge_type_id(data_for_current_test.iter_edge_node_names_and_edge_type_name_from_edge_type_id.edge_type_id, data_for_current_test.iter_edge_node_names_and_edge_type_name_from_edge_type_id.directed);
			},
			38 => {
				trace.push(format!("strongly_connected_components()", ));
				
				let g_copy = graph.clone();
				let trace2 = trace.clone();
				std::panic::set_hook(Box::new(move |info| {
					handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
				}));
				graph.strongly_connected_components();
			},
			39 => {
				trace.push(format!("get_node_ids_from_edge_id(edge_id = {:?})", data_for_current_test.get_node_ids_from_edge_id.edge_id));
				
				let g_copy = graph.clone();
				let trace2 = trace.clone();
				std::panic::set_hook(Box::new(move |info| {
					handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
				}));
				let _ = graph.get_node_ids_from_edge_id(data_for_current_test.get_node_ids_from_edge_id.edge_id);
			},
			40 => {
				trace.push(format!("get_edge_id_from_node_ids(src = {:?}, dst = {:?})", data_for_current_test.get_edge_id_from_node_ids.src, data_for_current_test.get_edge_id_from_node_ids.dst));
				
				let g_copy = graph.clone();
				let trace2 = trace.clone();
				std::panic::set_hook(Box::new(move |info| {
					handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
				}));
				let _ = graph.get_edge_id_from_node_ids(data_for_current_test.get_edge_id_from_node_ids.src, data_for_current_test.get_edge_id_from_node_ids.dst);
			},
			41 => {
				trace.push(format!("get_node_ids_and_edge_type_id_from_edge_id(edge_id = {:?})", data_for_current_test.get_node_ids_and_edge_type_id_from_edge_id.edge_id));
				
				let g_copy = graph.clone();
				let trace2 = trace.clone();
				std::panic::set_hook(Box::new(move |info| {
					handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
				}));
				let _ = graph.get_node_ids_and_edge_type_id_from_edge_id(data_for_current_test.get_node_ids_and_edge_type_id_from_edge_id.edge_id);
			},
			42 => {
				trace.push(format!("get_node_ids_and_edge_type_id_and_edge_weight_from_edge_id(edge_id = {:?})", data_for_current_test.get_node_ids_and_edge_type_id_and_edge_weight_from_edge_id.edge_id));
				
				let g_copy = graph.clone();
				let trace2 = trace.clone();
				std::panic::set_hook(Box::new(move |info| {
					handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
				}));
				let _ = graph.get_node_ids_and_edge_type_id_and_edge_weight_from_edge_id(data_for_current_test.get_node_ids_and_edge_type_id_and_edge_weight_from_edge_id.edge_id);
			},
			43 => {
				trace.push(format!("get_top_k_central_node_ids(k = {:?})", data_for_current_test.get_top_k_central_node_ids.k));
				
				let g_copy = graph.clone();
				let trace2 = trace.clone();
				std::panic::set_hook(Box::new(move |info| {
					handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
				}));
				graph.get_top_k_central_node_ids(data_for_current_test.get_top_k_central_node_ids.k);
			},
			44 => {
				trace.push(format!("get_node_degree_from_node_id(node_id = {:?})", data_for_current_test.get_node_degree_from_node_id.node_id));
				
				let g_copy = graph.clone();
				let trace2 = trace.clone();
				std::panic::set_hook(Box::new(move |info| {
					handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
				}));
				let _ = graph.get_node_degree_from_node_id(data_for_current_test.get_node_degree_from_node_id.node_id);
			},
			45 => {
				trace.push(format!("get_top_k_central_node_names(k = {:?})", data_for_current_test.get_top_k_central_node_names.k));
				
				let g_copy = graph.clone();
				let trace2 = trace.clone();
				std::panic::set_hook(Box::new(move |info| {
					handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
				}));
				graph.get_top_k_central_node_names(data_for_current_test.get_top_k_central_node_names.k);
			},
			46 => {
				trace.push(format!("get_node_type_id_from_node_id(node_id = {:?})", data_for_current_test.get_node_type_id_from_node_id.node_id));
				
				let g_copy = graph.clone();
				let trace2 = trace.clone();
				std::panic::set_hook(Box::new(move |info| {
					handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
				}));
				let _ = graph.get_node_type_id_from_node_id(data_for_current_test.get_node_type_id_from_node_id.node_id);
			},
			47 => {
				trace.push(format!("get_edge_type_id_from_edge_id(edge_id = {:?})", data_for_current_test.get_edge_type_id_from_edge_id.edge_id));
				
				let g_copy = graph.clone();
				let trace2 = trace.clone();
				std::panic::set_hook(Box::new(move |info| {
					handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
				}));
				let _ = graph.get_edge_type_id_from_edge_id(data_for_current_test.get_edge_type_id_from_edge_id.edge_id);
			},
			48 => {
				trace.push(format!("get_node_type_names_from_node_id(node_id = {:?})", data_for_current_test.get_node_type_names_from_node_id.node_id));
				
				let g_copy = graph.clone();
				let trace2 = trace.clone();
				std::panic::set_hook(Box::new(move |info| {
					handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
				}));
				let _ = graph.get_node_type_names_from_node_id(data_for_current_test.get_node_type_names_from_node_id.node_id);
			},
			49 => {
				trace.push(format!("get_edge_type_name_from_edge_id(edge_id = {:?})", data_for_current_test.get_edge_type_name_from_edge_id.edge_id));
				
				let g_copy = graph.clone();
				let trace2 = trace.clone();
				std::panic::set_hook(Box::new(move |info| {
					handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
				}));
				let _ = graph.get_edge_type_name_from_edge_id(data_for_current_test.get_edge_type_name_from_edge_id.edge_id);
			},
			50 => {
				trace.push(format!("get_edge_type_name_from_edge_type_id(edge_type_id = {:?})", data_for_current_test.get_edge_type_name_from_edge_type_id.edge_type_id));
				
				let g_copy = graph.clone();
				let trace2 = trace.clone();
				std::panic::set_hook(Box::new(move |info| {
					handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
				}));
				let _ = graph.get_edge_type_name_from_edge_type_id(data_for_current_test.get_edge_type_name_from_edge_type_id.edge_type_id);
			},
			51 => {
				trace.push(format!("get_edge_weight_from_edge_id(edge_id = {:?})", data_for_current_test.get_edge_weight_from_edge_id.edge_id));
				
				let g_copy = graph.clone();
				let trace2 = trace.clone();
				std::panic::set_hook(Box::new(move |info| {
					handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
				}));
				let _ = graph.get_edge_weight_from_edge_id(data_for_current_test.get_edge_weight_from_edge_id.edge_id);
			},
			52 => {
				trace.push(format!("get_edge_weight_from_node_ids(src = {:?}, dst = {:?})", data_for_current_test.get_edge_weight_from_node_ids.src, data_for_current_test.get_edge_weight_from_node_ids.dst));
				
				let g_copy = graph.clone();
				let trace2 = trace.clone();
				std::panic::set_hook(Box::new(move |info| {
					handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
				}));
				let _ = graph.get_edge_weight_from_node_ids(data_for_current_test.get_edge_weight_from_node_ids.src, data_for_current_test.get_edge_weight_from_node_ids.dst);
			},
			53 => {
				trace.push(format!("get_edge_weight_from_node_ids_and_edge_type_id(src = {:?}, dst = {:?}, edge_type = {:?})", data_for_current_test.get_edge_weight_from_node_ids_and_edge_type_id.src, data_for_current_test.get_edge_weight_from_node_ids_and_edge_type_id.dst, data_for_current_test.get_edge_weight_from_node_ids_and_edge_type_id.edge_type));
				
				let g_copy = graph.clone();
				let trace2 = trace.clone();
				std::panic::set_hook(Box::new(move |info| {
					handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
				}));
				let _ = graph.get_edge_weight_from_node_ids_and_edge_type_id(data_for_current_test.get_edge_weight_from_node_ids_and_edge_type_id.src, data_for_current_test.get_edge_weight_from_node_ids_and_edge_type_id.dst, data_for_current_test.get_edge_weight_from_node_ids_and_edge_type_id.edge_type);
			},
			54 => {
				trace.push(format!("get_node_name_from_node_id(node_id = {:?})", data_for_current_test.get_node_name_from_node_id.node_id));
				
				let g_copy = graph.clone();
				let trace2 = trace.clone();
				std::panic::set_hook(Box::new(move |info| {
					handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
				}));
				let _ = graph.get_node_name_from_node_id(data_for_current_test.get_node_name_from_node_id.node_id);
			},
			55 => {
				trace.push(format!("get_edge_node_names_from_edge_node_ids(edge_node_ids = {:?})", data_for_current_test.get_edge_node_names_from_edge_node_ids.edge_node_ids));
				
				let g_copy = graph.clone();
				let trace2 = trace.clone();
				std::panic::set_hook(Box::new(move |info| {
					handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
				}));
				let _ = graph.get_edge_node_names_from_edge_node_ids(data_for_current_test.get_edge_node_names_from_edge_node_ids.edge_node_ids);
			},
			56 => {
				trace.push(format!("get_edge_count_from_edge_type_id(edge_type_id = {:?})", data_for_current_test.get_edge_count_from_edge_type_id.edge_type_id));
				
				let g_copy = graph.clone();
				let trace2 = trace.clone();
				std::panic::set_hook(Box::new(move |info| {
					handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
				}));
				let _ = graph.get_edge_count_from_edge_type_id(data_for_current_test.get_edge_count_from_edge_type_id.edge_type_id);
			},
			57 => {
				trace.push(format!("get_node_count_from_node_type_id(node_type_id = {:?})", data_for_current_test.get_node_count_from_node_type_id.node_type_id));
				
				let g_copy = graph.clone();
				let trace2 = trace.clone();
				std::panic::set_hook(Box::new(move |info| {
					handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
				}));
				let _ = graph.get_node_count_from_node_type_id(data_for_current_test.get_node_count_from_node_type_id.node_type_id);
			},
			58 => {
				trace.push(format!("get_destination_node_id_from_edge_id(edge_id = {:?})", data_for_current_test.get_destination_node_id_from_edge_id.edge_id));
				
				let g_copy = graph.clone();
				let trace2 = trace.clone();
				std::panic::set_hook(Box::new(move |info| {
					handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
				}));
				let _ = graph.get_destination_node_id_from_edge_id(data_for_current_test.get_destination_node_id_from_edge_id.edge_id);
			},
			59 => {
				trace.push(format!("get_neighbour_node_ids_from_node_id(node_id = {:?})", data_for_current_test.get_neighbour_node_ids_from_node_id.node_id));
				
				let g_copy = graph.clone();
				let trace2 = trace.clone();
				std::panic::set_hook(Box::new(move |info| {
					handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
				}));
				let _ = graph.get_neighbour_node_ids_from_node_id(data_for_current_test.get_neighbour_node_ids_from_node_id.node_id);
			},
			60 => {
				trace.push(format!("get_minmax_edge_ids_from_node_ids(src = {:?}, dst = {:?})", data_for_current_test.get_minmax_edge_ids_from_node_ids.src, data_for_current_test.get_minmax_edge_ids_from_node_ids.dst));
				
				let g_copy = graph.clone();
				let trace2 = trace.clone();
				std::panic::set_hook(Box::new(move |info| {
					handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
				}));
				let _ = graph.get_minmax_edge_ids_from_node_ids(data_for_current_test.get_minmax_edge_ids_from_node_ids.src, data_for_current_test.get_minmax_edge_ids_from_node_ids.dst);
			},
			61 => {
				trace.push(format!("get_edge_id_from_node_ids_and_edge_type_id(src = {:?}, dst = {:?}, edge_type = {:?})", data_for_current_test.get_edge_id_from_node_ids_and_edge_type_id.src, data_for_current_test.get_edge_id_from_node_ids_and_edge_type_id.dst, data_for_current_test.get_edge_id_from_node_ids_and_edge_type_id.edge_type));
				
				let g_copy = graph.clone();
				let trace2 = trace.clone();
				std::panic::set_hook(Box::new(move |info| {
					handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
				}));
				let _ = graph.get_edge_id_from_node_ids_and_edge_type_id(data_for_current_test.get_edge_id_from_node_ids_and_edge_type_id.src, data_for_current_test.get_edge_id_from_node_ids_and_edge_type_id.dst, data_for_current_test.get_edge_id_from_node_ids_and_edge_type_id.edge_type);
			},
			62 => {
				trace.push(format!("get_edge_type_ids_from_edge_type_names(edge_type_names = {:?})", data_for_current_test.get_edge_type_ids_from_edge_type_names.edge_type_names));
				
				let g_copy = graph.clone();
				let trace2 = trace.clone();
				std::panic::set_hook(Box::new(move |info| {
					handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
				}));
				let _ = graph.get_edge_type_ids_from_edge_type_names(data_for_current_test.get_edge_type_ids_from_edge_type_names.edge_type_names);
			},
			63 => {
				trace.push(format!("get_node_type_ids_from_node_type_names(node_type_names = {:?})", data_for_current_test.get_node_type_ids_from_node_type_names.node_type_names));
				
				let g_copy = graph.clone();
				let trace2 = trace.clone();
				std::panic::set_hook(Box::new(move |info| {
					handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
				}));
				let _ = graph.get_node_type_ids_from_node_type_names(data_for_current_test.get_node_type_ids_from_node_type_names.node_type_names);
			},
			64 => {
				trace.push(format!("get_minmax_edge_ids_from_source_node_id(src = {:?})", data_for_current_test.get_minmax_edge_ids_from_source_node_id.src));
				
				let g_copy = graph.clone();
				let trace2 = trace.clone();
				std::panic::set_hook(Box::new(move |info| {
					handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
				}));
				let _ = graph.get_minmax_edge_ids_from_source_node_id(data_for_current_test.get_minmax_edge_ids_from_source_node_id.src);
			},
			65 => {
				trace.push(format!("get_node_type_name_from_node_type_id(node_type_id = {:?})", data_for_current_test.get_node_type_name_from_node_type_id.node_type_id));
				
				let g_copy = graph.clone();
				let trace2 = trace.clone();
				std::panic::set_hook(Box::new(move |info| {
					handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
				}));
				let _ = graph.get_node_type_name_from_node_type_id(data_for_current_test.get_node_type_name_from_node_type_id.node_type_id);
			},
			66 => {
				trace.push(format!("filter_from_ids(node_ids_to_keep = {:?}, node_ids_to_filter = {:?}, node_type_ids_to_keep = {:?}, node_type_ids_to_filter = {:?}, node_type_id_to_keep = {:?}, node_type_id_to_filter = {:?}, edge_ids_to_keep = {:?}, edge_ids_to_filter = {:?}, edge_node_ids_to_keep = {:?}, edge_node_ids_to_filter = {:?}, edge_type_ids_to_keep = {:?}, edge_type_ids_to_filter = {:?}, min_edge_weight = {:?}, max_edge_weight = {:?}, filter_singleton_nodes = {:?}, filter_singleton_nodes_with_selfloop = {:?}, filter_selfloops = {:?}, filter_parallel_edges = {:?}, verbose = {:?})", data_for_current_test.filter_from_ids.node_ids_to_keep, data_for_current_test.filter_from_ids.node_ids_to_filter, data_for_current_test.filter_from_ids.node_type_ids_to_keep, data_for_current_test.filter_from_ids.node_type_ids_to_filter, data_for_current_test.filter_from_ids.node_type_id_to_keep, data_for_current_test.filter_from_ids.node_type_id_to_filter, data_for_current_test.filter_from_ids.edge_ids_to_keep, data_for_current_test.filter_from_ids.edge_ids_to_filter, data_for_current_test.filter_from_ids.edge_node_ids_to_keep, data_for_current_test.filter_from_ids.edge_node_ids_to_filter, data_for_current_test.filter_from_ids.edge_type_ids_to_keep, data_for_current_test.filter_from_ids.edge_type_ids_to_filter, data_for_current_test.filter_from_ids.min_edge_weight, data_for_current_test.filter_from_ids.max_edge_weight, data_for_current_test.filter_from_ids.filter_singleton_nodes, data_for_current_test.filter_from_ids.filter_singleton_nodes_with_selfloop, data_for_current_test.filter_from_ids.filter_selfloops, data_for_current_test.filter_from_ids.filter_parallel_edges, data_for_current_test.filter_from_ids.verbose));
				
				let g_copy = graph.clone();
				let trace2 = trace.clone();
				std::panic::set_hook(Box::new(move |info| {
					handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
				}));
				graph.filter_from_ids(data_for_current_test.filter_from_ids.node_ids_to_keep, data_for_current_test.filter_from_ids.node_ids_to_filter, data_for_current_test.filter_from_ids.node_type_ids_to_keep, data_for_current_test.filter_from_ids.node_type_ids_to_filter, data_for_current_test.filter_from_ids.node_type_id_to_keep, data_for_current_test.filter_from_ids.node_type_id_to_filter, data_for_current_test.filter_from_ids.edge_ids_to_keep, data_for_current_test.filter_from_ids.edge_ids_to_filter, data_for_current_test.filter_from_ids.edge_node_ids_to_keep, data_for_current_test.filter_from_ids.edge_node_ids_to_filter, data_for_current_test.filter_from_ids.edge_type_ids_to_keep, data_for_current_test.filter_from_ids.edge_type_ids_to_filter, data_for_current_test.filter_from_ids.min_edge_weight, data_for_current_test.filter_from_ids.max_edge_weight, data_for_current_test.filter_from_ids.filter_singleton_nodes, data_for_current_test.filter_from_ids.filter_singleton_nodes_with_selfloop, data_for_current_test.filter_from_ids.filter_selfloops, data_for_current_test.filter_from_ids.filter_parallel_edges, data_for_current_test.filter_from_ids.verbose);
			},
			67 => {
				trace.push(format!("drop_unknown_node_types(verbose = {:?})", data_for_current_test.drop_unknown_node_types.verbose));
				
				let g_copy = graph.clone();
				let trace2 = trace.clone();
				std::panic::set_hook(Box::new(move |info| {
					handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
				}));
				graph.drop_unknown_node_types(data_for_current_test.drop_unknown_node_types.verbose);
			},
			68 => {
				trace.push(format!("drop_unknown_edge_types(verbose = {:?})", data_for_current_test.drop_unknown_edge_types.verbose));
				
				let g_copy = graph.clone();
				let trace2 = trace.clone();
				std::panic::set_hook(Box::new(move |info| {
					handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
				}));
				graph.drop_unknown_edge_types(data_for_current_test.drop_unknown_edge_types.verbose);
			},
			69 => {
				trace.push(format!("drop_singleton_nodes(verbose = {:?})", data_for_current_test.drop_singleton_nodes.verbose));
				
				let g_copy = graph.clone();
				let trace2 = trace.clone();
				std::panic::set_hook(Box::new(move |info| {
					handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
				}));
				graph.drop_singleton_nodes(data_for_current_test.drop_singleton_nodes.verbose);
			},
			70 => {
				trace.push(format!("drop_singleton_nodes_with_selfloops(verbose = {:?})", data_for_current_test.drop_singleton_nodes_with_selfloops.verbose));
				
				let g_copy = graph.clone();
				let trace2 = trace.clone();
				std::panic::set_hook(Box::new(move |info| {
					handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
				}));
				graph.drop_singleton_nodes_with_selfloops(data_for_current_test.drop_singleton_nodes_with_selfloops.verbose);
			},
			71 => {
				trace.push(format!("drop_selfloops(verbose = {:?})", data_for_current_test.drop_selfloops.verbose));
				
				let g_copy = graph.clone();
				let trace2 = trace.clone();
				std::panic::set_hook(Box::new(move |info| {
					handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
				}));
				graph.drop_selfloops(data_for_current_test.drop_selfloops.verbose);
			},
			72 => {
				trace.push(format!("drop_parallel_edges(verbose = {:?})", data_for_current_test.drop_parallel_edges.verbose));
				
				let g_copy = graph.clone();
				let trace2 = trace.clone();
				std::panic::set_hook(Box::new(move |info| {
					handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
				}));
				graph.drop_parallel_edges(data_for_current_test.drop_parallel_edges.verbose);
			},
			73 => {
				trace.push(format!("validate_node_id(node_id = {:?})", data_for_current_test.validate_node_id.node_id));
				
				let g_copy = graph.clone();
				let trace2 = trace.clone();
				std::panic::set_hook(Box::new(move |info| {
					handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
				}));
				let _ = graph.validate_node_id(data_for_current_test.validate_node_id.node_id);
			},
			74 => {
				trace.push(format!("validate_edge_id(edge_id = {:?})", data_for_current_test.validate_edge_id.edge_id));
				
				let g_copy = graph.clone();
				let trace2 = trace.clone();
				std::panic::set_hook(Box::new(move |info| {
					handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
				}));
				let _ = graph.validate_edge_id(data_for_current_test.validate_edge_id.edge_id);
			},
			75 => {
				trace.push(format!("validate_node_type_id(node_type_id = {:?})", data_for_current_test.validate_node_type_id.node_type_id));
				
				let g_copy = graph.clone();
				let trace2 = trace.clone();
				std::panic::set_hook(Box::new(move |info| {
					handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
				}));
				let _ = graph.validate_node_type_id(data_for_current_test.validate_node_type_id.node_type_id);
			},
			76 => {
				trace.push(format!("validate_node_type_ids(node_type_ids = {:?})", data_for_current_test.validate_node_type_ids.node_type_ids));
				
				let g_copy = graph.clone();
				let trace2 = trace.clone();
				std::panic::set_hook(Box::new(move |info| {
					handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
				}));
				let _ = graph.validate_node_type_ids(data_for_current_test.validate_node_type_ids.node_type_ids);
			},
			77 => {
				trace.push(format!("validate_edge_type_id(edge_type_id = {:?})", data_for_current_test.validate_edge_type_id.edge_type_id));
				
				let g_copy = graph.clone();
				let trace2 = trace.clone();
				std::panic::set_hook(Box::new(move |info| {
					handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
				}));
				let _ = graph.validate_edge_type_id(data_for_current_test.validate_edge_type_id.edge_type_id);
			},
			78 => {
				trace.push(format!("validate_edge_type_ids(edge_type_ids = {:?})", data_for_current_test.validate_edge_type_ids.edge_type_ids));
				
				let g_copy = graph.clone();
				let trace2 = trace.clone();
				std::panic::set_hook(Box::new(move |info| {
					handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
				}));
				let _ = graph.validate_edge_type_ids(data_for_current_test.validate_edge_type_ids.edge_type_ids);
			},
			79 => {
				trace.push(format!("must_have_node_types()", ));
				
				let g_copy = graph.clone();
				let trace2 = trace.clone();
				std::panic::set_hook(Box::new(move |info| {
					handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
				}));
				let _ = graph.must_have_node_types();
			},
			80 => {
				trace.push(format!("must_have_edge_types()", ));
				
				let g_copy = graph.clone();
				let trace2 = trace.clone();
				std::panic::set_hook(Box::new(move |info| {
					handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
				}));
				let _ = graph.must_have_edge_types();
			},
			81 => {
				trace.push(format!("must_be_undirected()", ));
				
				let g_copy = graph.clone();
				let trace2 = trace.clone();
				std::panic::set_hook(Box::new(move |info| {
					handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
				}));
				let _ = graph.must_be_undirected();
			},
			82 => {
				trace.push(format!("must_be_multigraph()", ));
				
				let g_copy = graph.clone();
				let trace2 = trace.clone();
				std::panic::set_hook(Box::new(move |info| {
					handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
				}));
				let _ = graph.must_be_multigraph();
			},
			83 => {
				trace.push(format!("must_not_be_multigraph()", ));
				
				let g_copy = graph.clone();
				let trace2 = trace.clone();
				std::panic::set_hook(Box::new(move |info| {
					handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
				}));
				let _ = graph.must_not_be_multigraph();
			},
			84 => {
				trace.push(format!("must_have_edge_weights()", ));
				
				let g_copy = graph.clone();
				let trace2 = trace.clone();
				std::panic::set_hook(Box::new(move |info| {
					handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
				}));
				let _ = graph.must_have_edge_weights();
			},
			85 => {
				trace.push(format!("must_have_edges()", ));
				
				let g_copy = graph.clone();
				let trace2 = trace.clone();
				std::panic::set_hook(Box::new(move |info| {
					handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
				}));
				let _ = graph.must_have_edges();
			},
			86 => {
				trace.push(format!("must_have_nodes()", ));
				
				let g_copy = graph.clone();
				let trace2 = trace.clone();
				std::panic::set_hook(Box::new(move |info| {
					handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
				}));
				let _ = graph.must_have_nodes();
			},
			87 => {
				trace.push(format!("remove_components(node_names = {:?}, node_types = {:?}, edge_types = {:?}, minimum_component_size = {:?}, top_k_components = {:?}, verbose = {:?})", data_for_current_test.remove_components.node_names, data_for_current_test.remove_components.node_types, data_for_current_test.remove_components.edge_types, data_for_current_test.remove_components.minimum_component_size, data_for_current_test.remove_components.top_k_components, data_for_current_test.remove_components.verbose));
				
				let g_copy = graph.clone();
				let trace2 = trace.clone();
				std::panic::set_hook(Box::new(move |info| {
					handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
				}));
				let _ = graph.remove_components(data_for_current_test.remove_components.node_names, data_for_current_test.remove_components.node_types, data_for_current_test.remove_components.edge_types, data_for_current_test.remove_components.minimum_component_size, data_for_current_test.remove_components.top_k_components, data_for_current_test.remove_components.verbose);
			},
			88 => {
				trace.push(format!("iter_degree_centrality()", ));
				
				let g_copy = graph.clone();
				let trace2 = trace.clone();
				std::panic::set_hook(Box::new(move |info| {
					handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
				}));
				match graph.iter_degree_centrality() {
					Ok(it) => {it.collect::<Vec<_>>();},
					Err(_) => {},
				};
			},
			89 => {
				trace.push(format!("get_degree_centrality()", ));
				
				let g_copy = graph.clone();
				let trace2 = trace.clone();
				std::panic::set_hook(Box::new(move |info| {
					handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
				}));
				graph.get_degree_centrality();
			},
			90 => {
				trace.push(format!("par_iter_unweighted_closeness_centrality(verbose = {:?})", data_for_current_test.par_iter_unweighted_closeness_centrality.verbose));
				
				let g_copy = graph.clone();
				let trace2 = trace.clone();
				std::panic::set_hook(Box::new(move |info| {
					handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
				}));
				let _ = graph.par_iter_unweighted_closeness_centrality(data_for_current_test.par_iter_unweighted_closeness_centrality.verbose).collect::<Vec<_>>();
			},
			91 => {
				trace.push(format!("par_iter_weighted_closeness_centrality(verbose = {:?})", data_for_current_test.par_iter_weighted_closeness_centrality.verbose));
				
				let g_copy = graph.clone();
				let trace2 = trace.clone();
				std::panic::set_hook(Box::new(move |info| {
					handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
				}));
				let _ = graph.par_iter_weighted_closeness_centrality(data_for_current_test.par_iter_weighted_closeness_centrality.verbose).map(|x| x.collect::<Vec<_>>());
			},
			92 => {
				trace.push(format!("get_unweighted_closeness_centrality(verbose = {:?})", data_for_current_test.get_unweighted_closeness_centrality.verbose));
				
				let g_copy = graph.clone();
				let trace2 = trace.clone();
				std::panic::set_hook(Box::new(move |info| {
					handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
				}));
				graph.get_unweighted_closeness_centrality(data_for_current_test.get_unweighted_closeness_centrality.verbose);
			},
			93 => {
				trace.push(format!("get_weighted_closeness_centrality(verbose = {:?})", data_for_current_test.get_weighted_closeness_centrality.verbose));
				
				let g_copy = graph.clone();
				let trace2 = trace.clone();
				std::panic::set_hook(Box::new(move |info| {
					handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
				}));
				graph.get_weighted_closeness_centrality(data_for_current_test.get_weighted_closeness_centrality.verbose);
			},
			94 => {
				trace.push(format!("par_iter_unweighted_harmonic_centrality(verbose = {:?})", data_for_current_test.par_iter_unweighted_harmonic_centrality.verbose));
				
				let g_copy = graph.clone();
				let trace2 = trace.clone();
				std::panic::set_hook(Box::new(move |info| {
					handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
				}));
				let _ = graph.par_iter_unweighted_harmonic_centrality(data_for_current_test.par_iter_unweighted_harmonic_centrality.verbose).collect::<Vec<_>>();
			},
			95 => {
				trace.push(format!("par_iter_weighted_harmonic_centrality(verbose = {:?})", data_for_current_test.par_iter_weighted_harmonic_centrality.verbose));
				
				let g_copy = graph.clone();
				let trace2 = trace.clone();
				std::panic::set_hook(Box::new(move |info| {
					handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
				}));
				let _ = graph.par_iter_weighted_harmonic_centrality(data_for_current_test.par_iter_weighted_harmonic_centrality.verbose).map(|x| x.collect::<Vec<_>>());
			},
			96 => {
				trace.push(format!("get_unweighted_harmonic_centrality(verbose = {:?})", data_for_current_test.get_unweighted_harmonic_centrality.verbose));
				
				let g_copy = graph.clone();
				let trace2 = trace.clone();
				std::panic::set_hook(Box::new(move |info| {
					handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
				}));
				graph.get_unweighted_harmonic_centrality(data_for_current_test.get_unweighted_harmonic_centrality.verbose);
			},
			97 => {
				trace.push(format!("get_weighted_harmonic_centrality(verbose = {:?})", data_for_current_test.get_weighted_harmonic_centrality.verbose));
				
				let g_copy = graph.clone();
				let trace2 = trace.clone();
				std::panic::set_hook(Box::new(move |info| {
					handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
				}));
				graph.get_weighted_harmonic_centrality(data_for_current_test.get_weighted_harmonic_centrality.verbose);
			},
			98 => {
				trace.push(format!("get_stress_centrality(normalize = {:?}, verbose = {:?})", data_for_current_test.get_stress_centrality.normalize, data_for_current_test.get_stress_centrality.verbose));
				
				let g_copy = graph.clone();
				let trace2 = trace.clone();
				std::panic::set_hook(Box::new(move |info| {
					handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
				}));
				graph.get_stress_centrality(data_for_current_test.get_stress_centrality.normalize, data_for_current_test.get_stress_centrality.verbose);
			},
			99 => {
				trace.push(format!("get_betweenness_centrality(normalize = {:?}, verbose = {:?})", data_for_current_test.get_betweenness_centrality.normalize, data_for_current_test.get_betweenness_centrality.verbose));
				
				let g_copy = graph.clone();
				let trace2 = trace.clone();
				std::panic::set_hook(Box::new(move |info| {
					handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
				}));
				graph.get_betweenness_centrality(data_for_current_test.get_betweenness_centrality.normalize, data_for_current_test.get_betweenness_centrality.verbose);
			},
			100 => {
				trace.push(format!("get_unweighted_eigenvector_centrality(maximum_iterations_number = {:?}, tollerance = {:?})", data_for_current_test.get_unweighted_eigenvector_centrality.maximum_iterations_number, data_for_current_test.get_unweighted_eigenvector_centrality.tollerance));
				
				let g_copy = graph.clone();
				let trace2 = trace.clone();
				std::panic::set_hook(Box::new(move |info| {
					handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
				}));
				let _ = graph.get_unweighted_eigenvector_centrality(data_for_current_test.get_unweighted_eigenvector_centrality.maximum_iterations_number.map(|x| x as usize), data_for_current_test.get_unweighted_eigenvector_centrality.tollerance);
			},
			101 => {
				trace.push(format!("get_weighted_eigenvector_centrality(maximum_iterations_number = {:?}, tollerance = {:?})", data_for_current_test.get_weighted_eigenvector_centrality.maximum_iterations_number, data_for_current_test.get_weighted_eigenvector_centrality.tollerance));
				
				let g_copy = graph.clone();
				let trace2 = trace.clone();
				std::panic::set_hook(Box::new(move |info| {
					handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
				}));
				let _ = graph.get_weighted_eigenvector_centrality(data_for_current_test.get_weighted_eigenvector_centrality.maximum_iterations_number.map(|x| x as usize), data_for_current_test.get_weighted_eigenvector_centrality.tollerance);
			},
			102 => {
				trace.push(format!("set_name(name = {:?})", data_for_current_test.set_name.name));
				
				let g_copy = graph.clone();
				let trace2 = trace.clone();
				std::panic::set_hook(Box::new(move |info| {
					handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
				}));
				graph.set_name(data_for_current_test.set_name.name);
			},
			103 => {
				trace.push(format!("set_inplace_all_edge_types(edge_type = {:?})", data_for_current_test.set_inplace_all_edge_types.edge_type));
				
				let g_copy = graph.clone();
				let trace2 = trace.clone();
				std::panic::set_hook(Box::new(move |info| {
					handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
				}));
				let _ = graph.set_inplace_all_edge_types(data_for_current_test.set_inplace_all_edge_types.edge_type);
			},
			104 => {
				trace.push(format!("set_all_edge_types(edge_type = {:?}, verbose = {:?})", data_for_current_test.set_all_edge_types.edge_type, data_for_current_test.set_all_edge_types.verbose));
				
				let g_copy = graph.clone();
				let trace2 = trace.clone();
				std::panic::set_hook(Box::new(move |info| {
					handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
				}));
				let _ = graph.set_all_edge_types(data_for_current_test.set_all_edge_types.edge_type, data_for_current_test.set_all_edge_types.verbose);
			},
			105 => {
				trace.push(format!("set_inplace_all_node_types(node_type = {:?})", data_for_current_test.set_inplace_all_node_types.node_type));
				
				let g_copy = graph.clone();
				let trace2 = trace.clone();
				std::panic::set_hook(Box::new(move |info| {
					handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
				}));
				let _ = graph.set_inplace_all_node_types(data_for_current_test.set_inplace_all_node_types.node_type);
			},
			106 => {
				trace.push(format!("set_all_node_types(node_type = {:?})", data_for_current_test.set_all_node_types.node_type));
				
				let g_copy = graph.clone();
				let trace2 = trace.clone();
				std::panic::set_hook(Box::new(move |info| {
					handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
				}));
				let _ = graph.set_all_node_types(data_for_current_test.set_all_node_types.node_type);
			},
			107 => {
				trace.push(format!("remove_inplace_node_type_ids(node_type_id = {:?})", data_for_current_test.remove_inplace_node_type_id.node_type_id));
				
				let g_copy = graph.clone();
				let trace2 = trace.clone();
				std::panic::set_hook(Box::new(move |info| {
					handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
				}));
				let _ = graph.remove_inplace_node_type_ids(data_for_current_test.remove_inplace_node_type_id.node_type_id);
			},
			108 => {
				trace.push(format!("remove_inplace_singleton_node_types()", ));
				
				let g_copy = graph.clone();
				let trace2 = trace.clone();
				std::panic::set_hook(Box::new(move |info| {
					handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
				}));
				let _ = graph.remove_inplace_singleton_node_types();
			},
			109 => {
				trace.push(format!("remove_inplace_edge_type_ids(edge_type_id = {:?})", data_for_current_test.remove_inplace_edge_type_id.edge_type_id));
				
				let g_copy = graph.clone();
				let trace2 = trace.clone();
				std::panic::set_hook(Box::new(move |info| {
					handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
				}));
				let _ = graph.remove_inplace_edge_type_ids(data_for_current_test.remove_inplace_edge_type_id.edge_type_id);
			},
			110 => {
				trace.push(format!("remove_inplace_singleton_edge_types()", ));
				
				let g_copy = graph.clone();
				let trace2 = trace.clone();
				std::panic::set_hook(Box::new(move |info| {
					handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
				}));
				let _ = graph.remove_inplace_singleton_edge_types();
			},
			111 => {
				trace.push(format!("remove_node_type_id(node_type_id = {:?})", data_for_current_test.remove_node_type_id.node_type_id));
				
				let g_copy = graph.clone();
				let trace2 = trace.clone();
				std::panic::set_hook(Box::new(move |info| {
					handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
				}));
				let _ = graph.remove_node_type_id(data_for_current_test.remove_node_type_id.node_type_id);
			},
			112 => {
				trace.push(format!("remove_singleton_node_types()", ));
				
				let g_copy = graph.clone();
				let trace2 = trace.clone();
				std::panic::set_hook(Box::new(move |info| {
					handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
				}));
				let _ = graph.remove_singleton_node_types();
			},
			113 => {
				trace.push(format!("remove_edge_type_id(edge_type_id = {:?})", data_for_current_test.remove_edge_type_id.edge_type_id));
				
				let g_copy = graph.clone();
				let trace2 = trace.clone();
				std::panic::set_hook(Box::new(move |info| {
					handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
				}));
				let _ = graph.remove_edge_type_id(data_for_current_test.remove_edge_type_id.edge_type_id);
			},
			114 => {
				trace.push(format!("remove_singleton_edge_types()", ));
				
				let g_copy = graph.clone();
				let trace2 = trace.clone();
				std::panic::set_hook(Box::new(move |info| {
					handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
				}));
				let _ = graph.remove_singleton_edge_types();
			},
			115 => {
				trace.push(format!("remove_inplace_node_types()", ));
				
				let g_copy = graph.clone();
				let trace2 = trace.clone();
				std::panic::set_hook(Box::new(move |info| {
					handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
				}));
				let _ = graph.remove_inplace_node_types();
			},
			116 => {
				trace.push(format!("remove_node_types()", ));
				
				let g_copy = graph.clone();
				let trace2 = trace.clone();
				std::panic::set_hook(Box::new(move |info| {
					handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
				}));
				let _ = graph.remove_node_types();
			},
			117 => {
				trace.push(format!("remove_inplace_edge_types()", ));
				
				let g_copy = graph.clone();
				let trace2 = trace.clone();
				std::panic::set_hook(Box::new(move |info| {
					handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
				}));
				let _ = graph.remove_inplace_edge_types();
			},
			118 => {
				trace.push(format!("remove_edge_types(verbose = {:?})", data_for_current_test.remove_edge_types.verbose));
				
				let g_copy = graph.clone();
				let trace2 = trace.clone();
				std::panic::set_hook(Box::new(move |info| {
					handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
				}));
				let _ = graph.remove_edge_types(data_for_current_test.remove_edge_types.verbose);
			},
			119 => {
				trace.push(format!("remove_inplace_edge_weights()", ));
				
				let g_copy = graph.clone();
				let trace2 = trace.clone();
				std::panic::set_hook(Box::new(move |info| {
					handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
				}));
				let _ = graph.remove_inplace_edge_weights();
			},
			120 => {
				trace.push(format!("remove_edge_weights()", ));
				
				let g_copy = graph.clone();
				let trace2 = trace.clone();
				std::panic::set_hook(Box::new(move |info| {
					handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
				}));
				let _ = graph.remove_edge_weights();
			},
			121 => {
				trace.push(format!("encode_edge(src = {:?}, dst = {:?})", data_for_current_test.encode_edge.src, data_for_current_test.encode_edge.dst));
				
				let g_copy = graph.clone();
				let trace2 = trace.clone();
				std::panic::set_hook(Box::new(move |info| {
					handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
				}));
				graph.encode_edge(data_for_current_test.encode_edge.src, data_for_current_test.encode_edge.dst);
			},
			122 => {
				trace.push(format!("decode_edge(edge = {:?})", data_for_current_test.decode_edge.edge));
				
				let g_copy = graph.clone();
				let trace2 = trace.clone();
				std::panic::set_hook(Box::new(move |info| {
					handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
				}));
				graph.decode_edge(data_for_current_test.decode_edge.edge);
			},
			123 => {
				trace.push(format!("get_max_encodable_edge_number()", ));
				
				let g_copy = graph.clone();
				let trace2 = trace.clone();
				std::panic::set_hook(Box::new(move |info| {
					handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
				}));
				graph.get_max_encodable_edge_number();
			},
			124 => {
				trace.push(format!("get_bipartite_edges(removed_existing_edges = {:?}, first_nodes_set = {:?}, second_nodes_set = {:?}, first_node_types_set = {:?}, second_node_types_set = {:?})", data_for_current_test.get_bipartite_edges.removed_existing_edges, data_for_current_test.get_bipartite_edges.first_nodes_set, data_for_current_test.get_bipartite_edges.second_nodes_set, data_for_current_test.get_bipartite_edges.first_node_types_set, data_for_current_test.get_bipartite_edges.second_node_types_set));
				
				let g_copy = graph.clone();
				let trace2 = trace.clone();
				std::panic::set_hook(Box::new(move |info| {
					handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
				}));
				let _ = graph.get_bipartite_edges(data_for_current_test.get_bipartite_edges.removed_existing_edges, data_for_current_test.get_bipartite_edges.first_nodes_set, data_for_current_test.get_bipartite_edges.second_nodes_set, data_for_current_test.get_bipartite_edges.first_node_types_set, data_for_current_test.get_bipartite_edges.second_node_types_set);
			},
			125 => {
				trace.push(format!("get_bipartite_edge_names(removed_existing_edges = {:?}, first_nodes_set = {:?}, second_nodes_set = {:?}, first_node_types_set = {:?}, second_node_types_set = {:?})", data_for_current_test.get_bipartite_edge_names.removed_existing_edges, data_for_current_test.get_bipartite_edge_names.first_nodes_set, data_for_current_test.get_bipartite_edge_names.second_nodes_set, data_for_current_test.get_bipartite_edge_names.first_node_types_set, data_for_current_test.get_bipartite_edge_names.second_node_types_set));
				
				let g_copy = graph.clone();
				let trace2 = trace.clone();
				std::panic::set_hook(Box::new(move |info| {
					handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
				}));
				let _ = graph.get_bipartite_edge_names(data_for_current_test.get_bipartite_edge_names.removed_existing_edges, data_for_current_test.get_bipartite_edge_names.first_nodes_set, data_for_current_test.get_bipartite_edge_names.second_nodes_set, data_for_current_test.get_bipartite_edge_names.first_node_types_set, data_for_current_test.get_bipartite_edge_names.second_node_types_set);
			},
			126 => {
				trace.push(format!("get_star_edges(central_node = {:?}, removed_existing_edges = {:?}, star_points_nodes_set = {:?}, star_points_node_types_set = {:?})", data_for_current_test.get_star_edges.central_node, data_for_current_test.get_star_edges.removed_existing_edges, data_for_current_test.get_star_edges.star_points_nodes_set, data_for_current_test.get_star_edges.star_points_node_types_set));
				
				let g_copy = graph.clone();
				let trace2 = trace.clone();
				std::panic::set_hook(Box::new(move |info| {
					handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
				}));
				let _ = graph.get_star_edges(data_for_current_test.get_star_edges.central_node, data_for_current_test.get_star_edges.removed_existing_edges, data_for_current_test.get_star_edges.star_points_nodes_set, data_for_current_test.get_star_edges.star_points_node_types_set);
			},
			127 => {
				trace.push(format!("get_star_edge_names(central_node = {:?}, removed_existing_edges = {:?}, star_points_nodes_set = {:?}, star_points_node_types_set = {:?})", data_for_current_test.get_star_edge_names.central_node, data_for_current_test.get_star_edge_names.removed_existing_edges, data_for_current_test.get_star_edge_names.star_points_nodes_set, data_for_current_test.get_star_edge_names.star_points_node_types_set));
				
				let g_copy = graph.clone();
				let trace2 = trace.clone();
				std::panic::set_hook(Box::new(move |info| {
					handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
				}));
				let _ = graph.get_star_edge_names(data_for_current_test.get_star_edge_names.central_node, data_for_current_test.get_star_edge_names.removed_existing_edges, data_for_current_test.get_star_edge_names.star_points_nodes_set, data_for_current_test.get_star_edge_names.star_points_node_types_set);
			},
			128 => {
				trace.push(format!("get_clique_edges(directed = {:?}, allow_selfloops = {:?}, removed_existing_edges = {:?}, allow_node_type_set = {:?}, allow_node_set = {:?})", data_for_current_test.get_clique_edges.directed, data_for_current_test.get_clique_edges.allow_selfloops, data_for_current_test.get_clique_edges.removed_existing_edges, data_for_current_test.get_clique_edges.allow_node_type_set, data_for_current_test.get_clique_edges.allow_node_set));
				
				let g_copy = graph.clone();
				let trace2 = trace.clone();
				std::panic::set_hook(Box::new(move |info| {
					handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
				}));
				graph.get_clique_edges(data_for_current_test.get_clique_edges.directed, data_for_current_test.get_clique_edges.allow_selfloops, data_for_current_test.get_clique_edges.removed_existing_edges, data_for_current_test.get_clique_edges.allow_node_type_set, data_for_current_test.get_clique_edges.allow_node_set);
			},
			129 => {
				trace.push(format!("get_clique_edge_names(directed = {:?}, allow_selfloops = {:?}, removed_existing_edges = {:?}, allow_node_type_set = {:?}, allow_node_set = {:?})", data_for_current_test.get_clique_edge_names.directed, data_for_current_test.get_clique_edge_names.allow_selfloops, data_for_current_test.get_clique_edge_names.removed_existing_edges, data_for_current_test.get_clique_edge_names.allow_node_type_set, data_for_current_test.get_clique_edge_names.allow_node_set));
				
				let g_copy = graph.clone();
				let trace2 = trace.clone();
				std::panic::set_hook(Box::new(move |info| {
					handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
				}));
				graph.get_clique_edge_names(data_for_current_test.get_clique_edge_names.directed, data_for_current_test.get_clique_edge_names.allow_selfloops, data_for_current_test.get_clique_edge_names.removed_existing_edges, data_for_current_test.get_clique_edge_names.allow_node_type_set, data_for_current_test.get_clique_edge_names.allow_node_set);
			},
			130 => {
				trace.push(format!("replace(node_name_mapping = {:?}, node_type_name_mapping = {:?}, node_type_names_mapping = {:?}, edge_type_name_mapping = {:?}, verbose = {:?})", data_for_current_test.replace.node_name_mapping, data_for_current_test.replace.node_type_name_mapping, data_for_current_test.replace.node_type_names_mapping, data_for_current_test.replace.edge_type_name_mapping, data_for_current_test.replace.verbose));
				
				let g_copy = graph.clone();
				let trace2 = trace.clone();
				std::panic::set_hook(Box::new(move |info| {
					handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
				}));
				let _ = graph.replace(data_for_current_test.replace.node_name_mapping, data_for_current_test.replace.node_type_name_mapping, data_for_current_test.replace.node_type_names_mapping, data_for_current_test.replace.edge_type_name_mapping, data_for_current_test.replace.verbose);
			},
			131 => {
				trace.push(format!("replace_unknown_node_types_with_node_type_name(node_type_names = {:?}, verbose = {:?})", data_for_current_test.replace_unknown_node_types_with_node_type_name.node_type_names, data_for_current_test.replace_unknown_node_types_with_node_type_name.verbose));
				
				let g_copy = graph.clone();
				let trace2 = trace.clone();
				std::panic::set_hook(Box::new(move |info| {
					handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
				}));
				let _ = graph.replace_unknown_node_types_with_node_type_name(data_for_current_test.replace_unknown_node_types_with_node_type_name.node_type_names, data_for_current_test.replace_unknown_node_types_with_node_type_name.verbose);
			},
			132 => {
				trace.push(format!("replace_unknown_edge_types_with_edge_type_name(edge_type_name = {:?}, verbose = {:?})", data_for_current_test.replace_unknown_edge_types_with_edge_type_name.edge_type_name, data_for_current_test.replace_unknown_edge_types_with_edge_type_name.verbose));
				
				let g_copy = graph.clone();
				let trace2 = trace.clone();
				std::panic::set_hook(Box::new(move |info| {
					handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
				}));
				let _ = graph.replace_unknown_edge_types_with_edge_type_name(data_for_current_test.replace_unknown_edge_types_with_edge_type_name.edge_type_name, data_for_current_test.replace_unknown_edge_types_with_edge_type_name.verbose);
			},
			133 => {
				trace.push(format!("approximated_vertex_cover_bitvec()", ));
				
				let g_copy = graph.clone();
				let trace2 = trace.clone();
				std::panic::set_hook(Box::new(move |info| {
					handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
				}));
				graph.approximated_vertex_cover_bitvec();
			},
			134 => {
				trace.push(format!("approximated_vertex_cover_set()", ));
				
				let g_copy = graph.clone();
				let trace2 = trace.clone();
				std::panic::set_hook(Box::new(move |info| {
					handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
				}));
				graph.approximated_vertex_cover_set();
			},
			135 => {
				trace.push(format!("report()", ));
				
				let g_copy = graph.clone();
				let trace2 = trace.clone();
				std::panic::set_hook(Box::new(move |info| {
					handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
				}));
				graph.report();
			},
			136 => {
				trace.push(format!("get_node_report_from_node_id(node_id = {:?})", data_for_current_test.get_node_report_from_node_id.node_id));
				
				let g_copy = graph.clone();
				let trace2 = trace.clone();
				std::panic::set_hook(Box::new(move |info| {
					handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
				}));
				let _ = graph.get_node_report_from_node_id(data_for_current_test.get_node_report_from_node_id.node_id);
			},
			137 => {
				trace.push(format!("get_peculiarities_report_markdown()", ));
				
				let g_copy = graph.clone();
				let trace2 = trace.clone();
				std::panic::set_hook(Box::new(move |info| {
					handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
				}));
				graph.get_peculiarities_report_markdown();
			},
			138 => {
				trace.push(format!("textual_report(verbose = {:?})", data_for_current_test.textual_report.verbose));
				
				let g_copy = graph.clone();
				let trace2 = trace.clone();
				std::panic::set_hook(Box::new(move |info| {
					handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
				}));
				let _ = graph.textual_report(data_for_current_test.textual_report.verbose);
			},
			139 => {
				trace.push(format!("get_connected_components_number(verbose = {:?})", data_for_current_test.get_connected_components_number.verbose));
				
				let g_copy = graph.clone();
				let trace2 = trace.clone();
				std::panic::set_hook(Box::new(move |info| {
					handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
				}));
				graph.get_connected_components_number(data_for_current_test.get_connected_components_number.verbose);
			},
			140 => {
				trace.push(format!("get_singleton_nodes_number()", ));
				
				let g_copy = graph.clone();
				let trace2 = trace.clone();
				std::panic::set_hook(Box::new(move |info| {
					handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
				}));
				graph.get_singleton_nodes_number();
			},
			141 => {
				trace.push(format!("get_singleton_node_ids()", ));
				
				let g_copy = graph.clone();
				let trace2 = trace.clone();
				std::panic::set_hook(Box::new(move |info| {
					handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
				}));
				graph.get_singleton_node_ids();
			},
			142 => {
				trace.push(format!("get_singleton_node_names()", ));
				
				let g_copy = graph.clone();
				let trace2 = trace.clone();
				std::panic::set_hook(Box::new(move |info| {
					handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
				}));
				graph.get_singleton_node_names();
			},
			143 => {
				trace.push(format!("get_singleton_nodes_with_selfloops_number()", ));
				
				let g_copy = graph.clone();
				let trace2 = trace.clone();
				std::panic::set_hook(Box::new(move |info| {
					handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
				}));
				graph.get_singleton_nodes_with_selfloops_number();
			},
			144 => {
				trace.push(format!("get_singleton_with_selfloops_node_ids()", ));
				
				let g_copy = graph.clone();
				let trace2 = trace.clone();
				std::panic::set_hook(Box::new(move |info| {
					handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
				}));
				graph.get_singleton_with_selfloops_node_ids();
			},
			145 => {
				trace.push(format!("get_singleton_with_selfloops_node_names()", ));
				
				let g_copy = graph.clone();
				let trace2 = trace.clone();
				std::panic::set_hook(Box::new(move |info| {
					handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
				}));
				graph.get_singleton_with_selfloops_node_names();
			},
			146 => {
				trace.push(format!("get_disconnected_nodes_number()", ));
				
				let g_copy = graph.clone();
				let trace2 = trace.clone();
				std::panic::set_hook(Box::new(move |info| {
					handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
				}));
				graph.get_disconnected_nodes_number();
			},
			147 => {
				trace.push(format!("get_density()", ));
				
				let g_copy = graph.clone();
				let trace2 = trace.clone();
				std::panic::set_hook(Box::new(move |info| {
					handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
				}));
				let _ = graph.get_density();
			},
			148 => {
				trace.push(format!("get_trap_nodes_rate()", ));
				
				let g_copy = graph.clone();
				let trace2 = trace.clone();
				std::panic::set_hook(Box::new(move |info| {
					handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
				}));
				graph.get_trap_nodes_rate();
			},
			149 => {
				trace.push(format!("get_node_degrees_mean()", ));
				
				let g_copy = graph.clone();
				let trace2 = trace.clone();
				std::panic::set_hook(Box::new(move |info| {
					handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
				}));
				let _ = graph.get_node_degrees_mean();
			},
			150 => {
				trace.push(format!("get_undirected_edges_number()", ));
				
				let g_copy = graph.clone();
				let trace2 = trace.clone();
				std::panic::set_hook(Box::new(move |info| {
					handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
				}));
				graph.get_undirected_edges_number();
			},
			151 => {
				trace.push(format!("get_unique_undirected_edges_number()", ));
				
				let g_copy = graph.clone();
				let trace2 = trace.clone();
				std::panic::set_hook(Box::new(move |info| {
					handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
				}));
				graph.get_unique_undirected_edges_number();
			},
			152 => {
				trace.push(format!("get_edges_number()", ));
				
				let g_copy = graph.clone();
				let trace2 = trace.clone();
				std::panic::set_hook(Box::new(move |info| {
					handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
				}));
				graph.get_edges_number();
			},
			153 => {
				trace.push(format!("get_unique_edges_number()", ));
				
				let g_copy = graph.clone();
				let trace2 = trace.clone();
				std::panic::set_hook(Box::new(move |info| {
					handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
				}));
				graph.get_unique_edges_number();
			},
			154 => {
				trace.push(format!("get_node_degrees_median()", ));
				
				let g_copy = graph.clone();
				let trace2 = trace.clone();
				std::panic::set_hook(Box::new(move |info| {
					handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
				}));
				let _ = graph.get_node_degrees_median();
			},
			155 => {
				trace.push(format!("get_max_node_degree()", ));
				
				let g_copy = graph.clone();
				let trace2 = trace.clone();
				std::panic::set_hook(Box::new(move |info| {
					handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
				}));
				let _ = graph.get_max_node_degree();
			},
			156 => {
				trace.push(format!("get_argmax_node_degree()", ));
				
				let g_copy = graph.clone();
				let trace2 = trace.clone();
				std::panic::set_hook(Box::new(move |info| {
					handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
				}));
				let _ = graph.get_argmax_node_degree();
			},
			157 => {
				trace.push(format!("get_min_node_degree()", ));
				
				let g_copy = graph.clone();
				let trace2 = trace.clone();
				std::panic::set_hook(Box::new(move |info| {
					handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
				}));
				let _ = graph.get_min_node_degree();
			},
			158 => {
				trace.push(format!("get_node_degrees_mode()", ));
				
				let g_copy = graph.clone();
				let trace2 = trace.clone();
				std::panic::set_hook(Box::new(move |info| {
					handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
				}));
				let _ = graph.get_node_degrees_mode();
			},
			159 => {
				trace.push(format!("get_selfloop_nodes_number()", ));
				
				let g_copy = graph.clone();
				let trace2 = trace.clone();
				std::panic::set_hook(Box::new(move |info| {
					handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
				}));
				graph.get_selfloop_nodes_number();
			},
			160 => {
				trace.push(format!("get_unique_selfloop_number()", ));
				
				let g_copy = graph.clone();
				let trace2 = trace.clone();
				std::panic::set_hook(Box::new(move |info| {
					handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
				}));
				graph.get_unique_selfloop_number();
			},
			161 => {
				trace.push(format!("get_selfloop_nodes_rate()", ));
				
				let g_copy = graph.clone();
				let trace2 = trace.clone();
				std::panic::set_hook(Box::new(move |info| {
					handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
				}));
				let _ = graph.get_selfloop_nodes_rate();
			},
			162 => {
				trace.push(format!("get_name()", ));
				
				let g_copy = graph.clone();
				let trace2 = trace.clone();
				std::panic::set_hook(Box::new(move |info| {
					handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
				}));
				graph.get_name();
			},
			163 => {
				trace.push(format!("get_trap_nodes_number()", ));
				
				let g_copy = graph.clone();
				let trace2 = trace.clone();
				std::panic::set_hook(Box::new(move |info| {
					handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
				}));
				graph.get_trap_nodes_number();
			},
			164 => {
				trace.push(format!("get_source_node_ids(directed = {:?})", data_for_current_test.get_source_node_ids.directed));
				
				let g_copy = graph.clone();
				let trace2 = trace.clone();
				std::panic::set_hook(Box::new(move |info| {
					handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
				}));
				graph.get_source_node_ids(data_for_current_test.get_source_node_ids.directed);
			},
			165 => {
				trace.push(format!("get_source_names(directed = {:?})", data_for_current_test.get_source_names.directed));
				
				let g_copy = graph.clone();
				let trace2 = trace.clone();
				std::panic::set_hook(Box::new(move |info| {
					handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
				}));
				graph.get_source_names(data_for_current_test.get_source_names.directed);
			},
			166 => {
				trace.push(format!("get_destination_node_ids(directed = {:?})", data_for_current_test.get_destination_node_ids.directed));
				
				let g_copy = graph.clone();
				let trace2 = trace.clone();
				std::panic::set_hook(Box::new(move |info| {
					handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
				}));
				graph.get_destination_node_ids(data_for_current_test.get_destination_node_ids.directed);
			},
			167 => {
				trace.push(format!("get_destination_names(directed = {:?})", data_for_current_test.get_destination_names.directed));
				
				let g_copy = graph.clone();
				let trace2 = trace.clone();
				std::panic::set_hook(Box::new(move |info| {
					handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
				}));
				graph.get_destination_names(data_for_current_test.get_destination_names.directed);
			},
			168 => {
				trace.push(format!("get_node_names()", ));
				
				let g_copy = graph.clone();
				let trace2 = trace.clone();
				std::panic::set_hook(Box::new(move |info| {
					handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
				}));
				graph.get_node_names();
			},
			169 => {
				trace.push(format!("get_node_ids()", ));
				
				let g_copy = graph.clone();
				let trace2 = trace.clone();
				std::panic::set_hook(Box::new(move |info| {
					handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
				}));
				graph.get_node_ids();
			},
			170 => {
				trace.push(format!("get_edge_type_ids()", ));
				
				let g_copy = graph.clone();
				let trace2 = trace.clone();
				std::panic::set_hook(Box::new(move |info| {
					handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
				}));
				let _ = graph.get_edge_type_ids();
			},
			171 => {
				trace.push(format!("get_unique_edge_type_ids()", ));
				
				let g_copy = graph.clone();
				let trace2 = trace.clone();
				std::panic::set_hook(Box::new(move |info| {
					handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
				}));
				let _ = graph.get_unique_edge_type_ids();
			},
			172 => {
				trace.push(format!("get_edge_type_names()", ));
				
				let g_copy = graph.clone();
				let trace2 = trace.clone();
				std::panic::set_hook(Box::new(move |info| {
					handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
				}));
				let _ = graph.get_edge_type_names();
			},
			173 => {
				trace.push(format!("get_unique_edge_type_names()", ));
				
				let g_copy = graph.clone();
				let trace2 = trace.clone();
				std::panic::set_hook(Box::new(move |info| {
					handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
				}));
				let _ = graph.get_unique_edge_type_names();
			},
			174 => {
				trace.push(format!("get_edge_weights()", ));
				
				let g_copy = graph.clone();
				let trace2 = trace.clone();
				std::panic::set_hook(Box::new(move |info| {
					handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
				}));
				let _ = graph.get_edge_weights();
			},
			175 => {
				trace.push(format!("get_min_edge_weight()", ));
				
				let g_copy = graph.clone();
				let trace2 = trace.clone();
				std::panic::set_hook(Box::new(move |info| {
					handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
				}));
				let _ = graph.get_min_edge_weight();
			},
			176 => {
				trace.push(format!("get_max_edge_weight()", ));
				
				let g_copy = graph.clone();
				let trace2 = trace.clone();
				std::panic::set_hook(Box::new(move |info| {
					handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
				}));
				let _ = graph.get_max_edge_weight();
			},
			177 => {
				trace.push(format!("get_node_type_ids()", ));
				
				let g_copy = graph.clone();
				let trace2 = trace.clone();
				std::panic::set_hook(Box::new(move |info| {
					handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
				}));
				let _ = graph.get_node_type_ids();
			},
			178 => {
				trace.push(format!("get_node_type_names()", ));
				
				let g_copy = graph.clone();
				let trace2 = trace.clone();
				std::panic::set_hook(Box::new(move |info| {
					handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
				}));
				let _ = graph.get_node_type_names();
			},
			179 => {
				trace.push(format!("get_unique_node_type_ids()", ));
				
				let g_copy = graph.clone();
				let trace2 = trace.clone();
				std::panic::set_hook(Box::new(move |info| {
					handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
				}));
				let _ = graph.get_unique_node_type_ids();
			},
			180 => {
				trace.push(format!("get_unique_node_type_names()", ));
				
				let g_copy = graph.clone();
				let trace2 = trace.clone();
				std::panic::set_hook(Box::new(move |info| {
					handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
				}));
				let _ = graph.get_unique_node_type_names();
			},
			181 => {
				trace.push(format!("get_unique_directed_edges_number()", ));
				
				let g_copy = graph.clone();
				let trace2 = trace.clone();
				std::panic::set_hook(Box::new(move |info| {
					handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
				}));
				graph.get_unique_directed_edges_number();
			},
			182 => {
				trace.push(format!("get_nodes_mapping()", ));
				
				let g_copy = graph.clone();
				let trace2 = trace.clone();
				std::panic::set_hook(Box::new(move |info| {
					handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
				}));
				graph.get_nodes_mapping();
			},
			183 => {
				trace.push(format!("get_edge_node_ids(directed = {:?})", data_for_current_test.get_edge_node_ids.directed));
				
				let g_copy = graph.clone();
				let trace2 = trace.clone();
				std::panic::set_hook(Box::new(move |info| {
					handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
				}));
				graph.get_edge_node_ids(data_for_current_test.get_edge_node_ids.directed);
			},
			184 => {
				trace.push(format!("get_edge_node_names(directed = {:?})", data_for_current_test.get_edge_node_names.directed));
				
				let g_copy = graph.clone();
				let trace2 = trace.clone();
				std::panic::set_hook(Box::new(move |info| {
					handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
				}));
				graph.get_edge_node_names(data_for_current_test.get_edge_node_names.directed);
			},
			185 => {
				trace.push(format!("get_unknown_node_types_number()", ));
				
				let g_copy = graph.clone();
				let trace2 = trace.clone();
				std::panic::set_hook(Box::new(move |info| {
					handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
				}));
				let _ = graph.get_unknown_node_types_number();
			},
			186 => {
				trace.push(format!("get_unknown_node_types_rate()", ));
				
				let g_copy = graph.clone();
				let trace2 = trace.clone();
				std::panic::set_hook(Box::new(move |info| {
					handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
				}));
				let _ = graph.get_unknown_node_types_rate();
			},
			187 => {
				trace.push(format!("get_minimum_node_types_number()", ));
				
				let g_copy = graph.clone();
				let trace2 = trace.clone();
				std::panic::set_hook(Box::new(move |info| {
					handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
				}));
				let _ = graph.get_minimum_node_types_number();
			},
			188 => {
				trace.push(format!("get_singleton_node_types_number()", ));
				
				let g_copy = graph.clone();
				let trace2 = trace.clone();
				std::panic::set_hook(Box::new(move |info| {
					handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
				}));
				let _ = graph.get_singleton_node_types_number();
			},
			189 => {
				trace.push(format!("get_singleton_node_type_ids()", ));
				
				let g_copy = graph.clone();
				let trace2 = trace.clone();
				std::panic::set_hook(Box::new(move |info| {
					handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
				}));
				let _ = graph.get_singleton_node_type_ids();
			},
			190 => {
				trace.push(format!("get_singleton_node_type_names()", ));
				
				let g_copy = graph.clone();
				let trace2 = trace.clone();
				std::panic::set_hook(Box::new(move |info| {
					handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
				}));
				let _ = graph.get_singleton_node_type_names();
			},
			191 => {
				trace.push(format!("get_unknown_edge_types_number()", ));
				
				let g_copy = graph.clone();
				let trace2 = trace.clone();
				std::panic::set_hook(Box::new(move |info| {
					handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
				}));
				let _ = graph.get_unknown_edge_types_number();
			},
			192 => {
				trace.push(format!("get_unknown_edge_types_rate()", ));
				
				let g_copy = graph.clone();
				let trace2 = trace.clone();
				std::panic::set_hook(Box::new(move |info| {
					handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
				}));
				let _ = graph.get_unknown_edge_types_rate();
			},
			193 => {
				trace.push(format!("get_minimum_edge_types_number()", ));
				
				let g_copy = graph.clone();
				let trace2 = trace.clone();
				std::panic::set_hook(Box::new(move |info| {
					handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
				}));
				let _ = graph.get_minimum_edge_types_number();
			},
			194 => {
				trace.push(format!("get_singleton_edge_types_number()", ));
				
				let g_copy = graph.clone();
				let trace2 = trace.clone();
				std::panic::set_hook(Box::new(move |info| {
					handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
				}));
				let _ = graph.get_singleton_edge_types_number();
			},
			195 => {
				trace.push(format!("get_singleton_edge_type_ids()", ));
				
				let g_copy = graph.clone();
				let trace2 = trace.clone();
				std::panic::set_hook(Box::new(move |info| {
					handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
				}));
				let _ = graph.get_singleton_edge_type_ids();
			},
			196 => {
				trace.push(format!("get_singleton_edge_type_names()", ));
				
				let g_copy = graph.clone();
				let trace2 = trace.clone();
				std::panic::set_hook(Box::new(move |info| {
					handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
				}));
				let _ = graph.get_singleton_edge_type_names();
			},
			197 => {
				trace.push(format!("get_nodes_number()", ));
				
				let g_copy = graph.clone();
				let trace2 = trace.clone();
				std::panic::set_hook(Box::new(move |info| {
					handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
				}));
				graph.get_nodes_number();
			},
			198 => {
				trace.push(format!("get_node_connected_component_ids(verbose = {:?})", data_for_current_test.get_node_connected_component_ids.verbose));
				
				let g_copy = graph.clone();
				let trace2 = trace.clone();
				std::panic::set_hook(Box::new(move |info| {
					handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
				}));
				graph.get_node_connected_component_ids(data_for_current_test.get_node_connected_component_ids.verbose);
			},
			199 => {
				trace.push(format!("get_directed_edges_number()", ));
				
				let g_copy = graph.clone();
				let trace2 = trace.clone();
				std::panic::set_hook(Box::new(move |info| {
					handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
				}));
				graph.get_directed_edges_number();
			},
			200 => {
				trace.push(format!("get_edge_types_number()", ));
				
				let g_copy = graph.clone();
				let trace2 = trace.clone();
				std::panic::set_hook(Box::new(move |info| {
					handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
				}));
				let _ = graph.get_edge_types_number();
			},
			201 => {
				trace.push(format!("get_node_types_number()", ));
				
				let g_copy = graph.clone();
				let trace2 = trace.clone();
				std::panic::set_hook(Box::new(move |info| {
					handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
				}));
				let _ = graph.get_node_types_number();
			},
			202 => {
				trace.push(format!("get_node_degrees()", ));
				
				let g_copy = graph.clone();
				let trace2 = trace.clone();
				std::panic::set_hook(Box::new(move |info| {
					handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
				}));
				graph.get_node_degrees();
			},
			203 => {
				trace.push(format!("get_not_singletons_node_ids()", ));
				
				let g_copy = graph.clone();
				let trace2 = trace.clone();
				std::panic::set_hook(Box::new(move |info| {
					handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
				}));
				graph.get_not_singletons_node_ids();
			},
			204 => {
				trace.push(format!("get_dense_nodes_mapping()", ));
				
				let g_copy = graph.clone();
				let trace2 = trace.clone();
				std::panic::set_hook(Box::new(move |info| {
					handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
				}));
				graph.get_dense_nodes_mapping();
			},
			205 => {
				trace.push(format!("get_multigraph_edges_number()", ));
				
				let g_copy = graph.clone();
				let trace2 = trace.clone();
				std::panic::set_hook(Box::new(move |info| {
					handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
				}));
				graph.get_multigraph_edges_number();
			},
			206 => {
				trace.push(format!("get_cumulative_node_degrees()", ));
				
				let g_copy = graph.clone();
				let trace2 = trace.clone();
				std::panic::set_hook(Box::new(move |info| {
					handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
				}));
				graph.get_cumulative_node_degrees();
			},
			207 => {
				trace.push(format!("get_unique_source_nodes_number()", ));
				
				let g_copy = graph.clone();
				let trace2 = trace.clone();
				std::panic::set_hook(Box::new(move |info| {
					handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
				}));
				graph.get_unique_source_nodes_number();
			},
			208 => {
				trace.push(format!("get_edge_type_id_counts_hashmap()", ));
				
				let g_copy = graph.clone();
				let trace2 = trace.clone();
				std::panic::set_hook(Box::new(move |info| {
					handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
				}));
				let _ = graph.get_edge_type_id_counts_hashmap();
			},
			209 => {
				trace.push(format!("get_edge_type_names_counts_hashmap()", ));
				
				let g_copy = graph.clone();
				let trace2 = trace.clone();
				std::panic::set_hook(Box::new(move |info| {
					handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
				}));
				let _ = graph.get_edge_type_names_counts_hashmap();
			},
			210 => {
				trace.push(format!("get_node_type_id_counts_hashmap()", ));
				
				let g_copy = graph.clone();
				let trace2 = trace.clone();
				std::panic::set_hook(Box::new(move |info| {
					handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
				}));
				let _ = graph.get_node_type_id_counts_hashmap();
			},
			211 => {
				trace.push(format!("get_node_type_names_counts_hashmap()", ));
				
				let g_copy = graph.clone();
				let trace2 = trace.clone();
				std::panic::set_hook(Box::new(move |info| {
					handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
				}));
				let _ = graph.get_node_type_names_counts_hashmap();
			},
			212 => {
				trace.push(format!("iter_node_ids()", ));
				
				let g_copy = graph.clone();
				let trace2 = trace.clone();
				std::panic::set_hook(Box::new(move |info| {
					handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
				}));
				let _ = graph.iter_node_ids().collect::<Vec<_>>();
			},
			213 => {
				trace.push(format!("iter_node_names()", ));
				
				let g_copy = graph.clone();
				let trace2 = trace.clone();
				std::panic::set_hook(Box::new(move |info| {
					handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
				}));
				let _ = graph.iter_node_names().collect::<Vec<_>>();
			},
			214 => {
				trace.push(format!("iter_unique_node_type_ids()", ));
				
				let g_copy = graph.clone();
				let trace2 = trace.clone();
				std::panic::set_hook(Box::new(move |info| {
					handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
				}));
				let _ = graph.iter_unique_node_type_ids();
			},
			215 => {
				trace.push(format!("iter_node_type_counts()", ));
				
				let g_copy = graph.clone();
				let trace2 = trace.clone();
				std::panic::set_hook(Box::new(move |info| {
					handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
				}));
				let _ = graph.iter_node_type_counts();
			},
			216 => {
				trace.push(format!("iter_unique_node_type_ids_and_counts()", ));
				
				let g_copy = graph.clone();
				let trace2 = trace.clone();
				std::panic::set_hook(Box::new(move |info| {
					handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
				}));
				let _ = graph.iter_unique_node_type_ids_and_counts();
			},
			217 => {
				trace.push(format!("iter_unique_node_type_names()", ));
				
				let g_copy = graph.clone();
				let trace2 = trace.clone();
				std::panic::set_hook(Box::new(move |info| {
					handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
				}));
				let _ = graph.iter_unique_node_type_names();
			},
			218 => {
				trace.push(format!("iter_unique_node_type_names_and_counts()", ));
				
				let g_copy = graph.clone();
				let trace2 = trace.clone();
				std::panic::set_hook(Box::new(move |info| {
					handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
				}));
				let _ = graph.iter_unique_node_type_names_and_counts();
			},
			219 => {
				trace.push(format!("iter_unique_edge_type_ids()", ));
				
				let g_copy = graph.clone();
				let trace2 = trace.clone();
				std::panic::set_hook(Box::new(move |info| {
					handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
				}));
				let _ = graph.iter_unique_edge_type_ids();
			},
			220 => {
				trace.push(format!("iter_edge_type_counts()", ));
				
				let g_copy = graph.clone();
				let trace2 = trace.clone();
				std::panic::set_hook(Box::new(move |info| {
					handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
				}));
				let _ = graph.iter_edge_type_counts();
			},
			221 => {
				trace.push(format!("iter_unique_edge_type_ids_and_counts()", ));
				
				let g_copy = graph.clone();
				let trace2 = trace.clone();
				std::panic::set_hook(Box::new(move |info| {
					handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
				}));
				let _ = graph.iter_unique_edge_type_ids_and_counts();
			},
			222 => {
				trace.push(format!("iter_unique_edge_type_names_and_counts()", ));
				
				let g_copy = graph.clone();
				let trace2 = trace.clone();
				std::panic::set_hook(Box::new(move |info| {
					handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
				}));
				let _ = graph.iter_unique_edge_type_names_and_counts();
			},
			223 => {
				trace.push(format!("iter_unique_edge_type_names()", ));
				
				let g_copy = graph.clone();
				let trace2 = trace.clone();
				std::panic::set_hook(Box::new(move |info| {
					handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
				}));
				let _ = graph.iter_unique_edge_type_names();
			},
			224 => {
				trace.push(format!("par_iter_node_ids()", ));
				
				let g_copy = graph.clone();
				let trace2 = trace.clone();
				std::panic::set_hook(Box::new(move |info| {
					handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
				}));
				let _ = graph.par_iter_node_ids().collect::<Vec<_>>();
			},
			225 => {
				trace.push(format!("iter_node_degrees()", ));
				
				let g_copy = graph.clone();
				let trace2 = trace.clone();
				std::panic::set_hook(Box::new(move |info| {
					handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
				}));
				let _ = graph.iter_node_degrees().collect::<Vec<_>>();
			},
			226 => {
				trace.push(format!("par_iter_node_degrees()", ));
				
				let g_copy = graph.clone();
				let trace2 = trace.clone();
				std::panic::set_hook(Box::new(move |info| {
					handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
				}));
				let _ = graph.par_iter_node_degrees().collect::<Vec<_>>();
			},
			227 => {
				trace.push(format!("iter_connected_node_ids()", ));
				
				let g_copy = graph.clone();
				let trace2 = trace.clone();
				std::panic::set_hook(Box::new(move |info| {
					handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
				}));
				let _ = graph.iter_connected_node_ids().collect::<Vec<_>>();
			},
			228 => {
				trace.push(format!("iter_singleton_node_ids()", ));
				
				let g_copy = graph.clone();
				let trace2 = trace.clone();
				std::panic::set_hook(Box::new(move |info| {
					handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
				}));
				let _ = graph.iter_singleton_node_ids().collect::<Vec<_>>();
			},
			229 => {
				trace.push(format!("iter_singleton_node_names()", ));
				
				let g_copy = graph.clone();
				let trace2 = trace.clone();
				std::panic::set_hook(Box::new(move |info| {
					handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
				}));
				let _ = graph.iter_singleton_node_names().collect::<Vec<_>>();
			},
			230 => {
				trace.push(format!("iter_singleton_with_selfloops_node_ids()", ));
				
				let g_copy = graph.clone();
				let trace2 = trace.clone();
				std::panic::set_hook(Box::new(move |info| {
					handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
				}));
				let _ = graph.iter_singleton_with_selfloops_node_ids().collect::<Vec<_>>();
			},
			231 => {
				trace.push(format!("iter_singleton_with_selfloops_node_names()", ));
				
				let g_copy = graph.clone();
				let trace2 = trace.clone();
				std::panic::set_hook(Box::new(move |info| {
					handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
				}));
				let _ = graph.iter_singleton_with_selfloops_node_names().collect::<Vec<_>>();
			},
			232 => {
				trace.push(format!("iter_singleton_node_type_ids()", ));
				
				let g_copy = graph.clone();
				let trace2 = trace.clone();
				std::panic::set_hook(Box::new(move |info| {
					handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
				}));
				let _ = graph.iter_singleton_node_type_ids();
			},
			233 => {
				trace.push(format!("iter_singleton_edge_type_ids()", ));
				
				let g_copy = graph.clone();
				let trace2 = trace.clone();
				std::panic::set_hook(Box::new(move |info| {
					handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
				}));
				let _ = graph.iter_singleton_edge_type_ids();
			},
			234 => {
				trace.push(format!("iter_singleton_node_type_names()", ));
				
				let g_copy = graph.clone();
				let trace2 = trace.clone();
				std::panic::set_hook(Box::new(move |info| {
					handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
				}));
				let _ = graph.iter_singleton_node_type_names();
			},
			235 => {
				trace.push(format!("iter_singleton_edge_type_names()", ));
				
				let g_copy = graph.clone();
				let trace2 = trace.clone();
				std::panic::set_hook(Box::new(move |info| {
					handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
				}));
				let _ = graph.iter_singleton_edge_type_names();
			},
			236 => {
				trace.push(format!("iter_source_node_ids(directed = {:?})", data_for_current_test.iter_source_node_ids.directed));
				
				let g_copy = graph.clone();
				let trace2 = trace.clone();
				std::panic::set_hook(Box::new(move |info| {
					handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
				}));
				let _ = graph.iter_source_node_ids(data_for_current_test.iter_source_node_ids.directed).collect::<Vec<_>>();
			},
			237 => {
				trace.push(format!("iter_edge_weights()", ));
				
				let g_copy = graph.clone();
				let trace2 = trace.clone();
				std::panic::set_hook(Box::new(move |info| {
					handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
				}));
				let _ = graph.iter_edge_weights();
			},
			238 => {
				trace.push(format!("par_iter_edge_weights()", ));
				
				let g_copy = graph.clone();
				let trace2 = trace.clone();
				std::panic::set_hook(Box::new(move |info| {
					handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
				}));
				let _ = graph.par_iter_edge_weights();
			},
			239 => {
				trace.push(format!("par_iter_source_node_ids(directed = {:?})", data_for_current_test.par_iter_source_node_ids.directed));
				
				let g_copy = graph.clone();
				let trace2 = trace.clone();
				std::panic::set_hook(Box::new(move |info| {
					handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
				}));
				let _ = graph.par_iter_source_node_ids(data_for_current_test.par_iter_source_node_ids.directed).collect::<Vec<_>>();
			},
			240 => {
				trace.push(format!("iter_destination_node_ids(directed = {:?})", data_for_current_test.iter_destination_node_ids.directed));
				
				let g_copy = graph.clone();
				let trace2 = trace.clone();
				std::panic::set_hook(Box::new(move |info| {
					handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
				}));
				let _ = graph.iter_destination_node_ids(data_for_current_test.iter_destination_node_ids.directed).collect::<Vec<_>>();
			},
			241 => {
				trace.push(format!("par_iter_destination_node_ids(directed = {:?})", data_for_current_test.par_iter_destination_node_ids.directed));
				
				let g_copy = graph.clone();
				let trace2 = trace.clone();
				std::panic::set_hook(Box::new(move |info| {
					handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
				}));
				let _ = graph.par_iter_destination_node_ids(data_for_current_test.par_iter_destination_node_ids.directed).collect::<Vec<_>>();
			},
			242 => {
				trace.push(format!("iter_node_ids_and_node_type_ids()", ));
				
				let g_copy = graph.clone();
				let trace2 = trace.clone();
				std::panic::set_hook(Box::new(move |info| {
					handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
				}));
				let _ = graph.iter_node_ids_and_node_type_ids().collect::<Vec<_>>();
			},
			243 => {
				trace.push(format!("par_iter_node_ids_and_node_type_ids()", ));
				
				let g_copy = graph.clone();
				let trace2 = trace.clone();
				std::panic::set_hook(Box::new(move |info| {
					handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
				}));
				let _ = graph.par_iter_node_ids_and_node_type_ids().collect::<Vec<_>>();
			},
			244 => {
				trace.push(format!("iter_node_names_and_node_type_names()", ));
				
				let g_copy = graph.clone();
				let trace2 = trace.clone();
				std::panic::set_hook(Box::new(move |info| {
					handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
				}));
				let _ = graph.iter_node_names_and_node_type_names().collect::<Vec<_>>();
			},
			245 => {
				trace.push(format!("iter_edge_ids(directed = {:?})", data_for_current_test.iter_edge_ids.directed));
				
				let g_copy = graph.clone();
				let trace2 = trace.clone();
				std::panic::set_hook(Box::new(move |info| {
					handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
				}));
				let _ = graph.iter_edge_ids(data_for_current_test.iter_edge_ids.directed).collect::<Vec<_>>();
			},
			246 => {
				trace.push(format!("iter_edges(directed = {:?})", data_for_current_test.iter_edges.directed));
				
				let g_copy = graph.clone();
				let trace2 = trace.clone();
				std::panic::set_hook(Box::new(move |info| {
					handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
				}));
				let _ = graph.iter_edges(data_for_current_test.iter_edges.directed).collect::<Vec<_>>();
			},
			247 => {
				trace.push(format!("par_iter_edge_ids(directed = {:?})", data_for_current_test.par_iter_edge_ids.directed));
				
				let g_copy = graph.clone();
				let trace2 = trace.clone();
				std::panic::set_hook(Box::new(move |info| {
					handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
				}));
				let _ = graph.par_iter_edge_ids(data_for_current_test.par_iter_edge_ids.directed).collect::<Vec<_>>();
			},
			248 => {
				trace.push(format!("par_iter_edges(directed = {:?})", data_for_current_test.par_iter_edges.directed));
				
				let g_copy = graph.clone();
				let trace2 = trace.clone();
				std::panic::set_hook(Box::new(move |info| {
					handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
				}));
				let _ = graph.par_iter_edges(data_for_current_test.par_iter_edges.directed).collect::<Vec<_>>();
			},
			249 => {
				trace.push(format!("iter_edge_node_ids_and_edge_type_id(directed = {:?})", data_for_current_test.iter_edge_node_ids_and_edge_type_id.directed));
				
				let g_copy = graph.clone();
				let trace2 = trace.clone();
				std::panic::set_hook(Box::new(move |info| {
					handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
				}));
				let _ = graph.iter_edge_node_ids_and_edge_type_id(data_for_current_test.iter_edge_node_ids_and_edge_type_id.directed).collect::<Vec<_>>();
			},
			250 => {
				trace.push(format!("iter_edge_node_names_and_edge_type_name(directed = {:?})", data_for_current_test.iter_edge_node_names_and_edge_type_name.directed));
				
				let g_copy = graph.clone();
				let trace2 = trace.clone();
				std::panic::set_hook(Box::new(move |info| {
					handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
				}));
				let _ = graph.iter_edge_node_names_and_edge_type_name(data_for_current_test.iter_edge_node_names_and_edge_type_name.directed).collect::<Vec<_>>();
			},
			251 => {
				trace.push(format!("par_iter_edge_node_names_and_edge_type_name(directed = {:?})", data_for_current_test.par_iter_edge_node_names_and_edge_type_name.directed));
				
				let g_copy = graph.clone();
				let trace2 = trace.clone();
				std::panic::set_hook(Box::new(move |info| {
					handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
				}));
				let _ = graph.par_iter_edge_node_names_and_edge_type_name(data_for_current_test.par_iter_edge_node_names_and_edge_type_name.directed).collect::<Vec<_>>();
			},
			252 => {
				trace.push(format!("par_iter_edge_node_ids_and_edge_type_id(directed = {:?})", data_for_current_test.par_iter_edge_node_ids_and_edge_type_id.directed));
				
				let g_copy = graph.clone();
				let trace2 = trace.clone();
				std::panic::set_hook(Box::new(move |info| {
					handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
				}));
				let _ = graph.par_iter_edge_node_ids_and_edge_type_id(data_for_current_test.par_iter_edge_node_ids_and_edge_type_id.directed).collect::<Vec<_>>();
			},
			253 => {
				trace.push(format!("par_iter_edge_node_names_and_edge_type_name_and_edge_weight(directed = {:?})", data_for_current_test.par_iter_edge_node_names_and_edge_type_name_and_edge_weight.directed));
				
				let g_copy = graph.clone();
				let trace2 = trace.clone();
				std::panic::set_hook(Box::new(move |info| {
					handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
				}));
				let _ = graph.par_iter_edge_node_names_and_edge_type_name_and_edge_weight(data_for_current_test.par_iter_edge_node_names_and_edge_type_name_and_edge_weight.directed).collect::<Vec<_>>();
			},
			254 => {
				trace.push(format!("iter_edge_node_names_and_edge_type_name_and_edge_weight(directed = {:?})", data_for_current_test.iter_edge_node_names_and_edge_type_name_and_edge_weight.directed));
				
				let g_copy = graph.clone();
				let trace2 = trace.clone();
				std::panic::set_hook(Box::new(move |info| {
					handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
				}));
				let _ = graph.iter_edge_node_names_and_edge_type_name_and_edge_weight(data_for_current_test.iter_edge_node_names_and_edge_type_name_and_edge_weight.directed).collect::<Vec<_>>();
			},
			255 => {
				trace.push(format!("par_iter_edge_node_ids_and_edge_type_id_and_edge_weight(directed = {:?})", data_for_current_test.par_iter_edge_node_ids_and_edge_type_id_and_edge_weight.directed));
				
				let g_copy = graph.clone();
				let trace2 = trace.clone();
				std::panic::set_hook(Box::new(move |info| {
					handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
				}));
				let _ = graph.par_iter_edge_node_ids_and_edge_type_id_and_edge_weight(data_for_current_test.par_iter_edge_node_ids_and_edge_type_id_and_edge_weight.directed).collect::<Vec<_>>();
			},
			256 => {
				trace.push(format!("iter_edge_node_ids_and_edge_type_id_and_edge_weight(directed = {:?})", data_for_current_test.iter_edge_node_ids_and_edge_type_id_and_edge_weight.directed));
				
				let g_copy = graph.clone();
				let trace2 = trace.clone();
				std::panic::set_hook(Box::new(move |info| {
					handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
				}));
				let _ = graph.iter_edge_node_ids_and_edge_type_id_and_edge_weight(data_for_current_test.iter_edge_node_ids_and_edge_type_id_and_edge_weight.directed).collect::<Vec<_>>();
			},
			257 => {
				trace.push(format!("iter_unique_edge_node_ids(directed = {:?})", data_for_current_test.iter_unique_edge_node_ids.directed));
				
				let g_copy = graph.clone();
				let trace2 = trace.clone();
				std::panic::set_hook(Box::new(move |info| {
					handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
				}));
				let _ = graph.iter_unique_edge_node_ids(data_for_current_test.iter_unique_edge_node_ids.directed).collect::<Vec<_>>();
			},
			258 => {
				trace.push(format!("iter_unique_source_node_ids()", ));
				
				let g_copy = graph.clone();
				let trace2 = trace.clone();
				std::panic::set_hook(Box::new(move |info| {
					handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
				}));
				let _ = graph.iter_unique_source_node_ids().collect::<Vec<_>>();
			},
			259 => {
				trace.push(format!("has_nodes()", ));
				
				let g_copy = graph.clone();
				let trace2 = trace.clone();
				std::panic::set_hook(Box::new(move |info| {
					handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
				}));
				graph.has_nodes();
			},
			260 => {
				trace.push(format!("has_edges()", ));
				
				let g_copy = graph.clone();
				let trace2 = trace.clone();
				std::panic::set_hook(Box::new(move |info| {
					handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
				}));
				graph.has_edges();
			},
			261 => {
				trace.push(format!("has_trap_nodes()", ));
				
				let g_copy = graph.clone();
				let trace2 = trace.clone();
				std::panic::set_hook(Box::new(move |info| {
					handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
				}));
				graph.has_trap_nodes();
			},
			262 => {
				trace.push(format!("is_directed()", ));
				
				let g_copy = graph.clone();
				let trace2 = trace.clone();
				std::panic::set_hook(Box::new(move |info| {
					handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
				}));
				graph.is_directed();
			},
			263 => {
				trace.push(format!("has_edge_weights()", ));
				
				let g_copy = graph.clone();
				let trace2 = trace.clone();
				std::panic::set_hook(Box::new(move |info| {
					handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
				}));
				graph.has_edge_weights();
			},
			264 => {
				trace.push(format!("has_edge_types()", ));
				
				let g_copy = graph.clone();
				let trace2 = trace.clone();
				std::panic::set_hook(Box::new(move |info| {
					handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
				}));
				graph.has_edge_types();
			},
			265 => {
				trace.push(format!("has_selfloops()", ));
				
				let g_copy = graph.clone();
				let trace2 = trace.clone();
				std::panic::set_hook(Box::new(move |info| {
					handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
				}));
				graph.has_selfloops();
			},
			266 => {
				trace.push(format!("has_singleton_nodes()", ));
				
				let g_copy = graph.clone();
				let trace2 = trace.clone();
				std::panic::set_hook(Box::new(move |info| {
					handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
				}));
				graph.has_singleton_nodes();
			},
			267 => {
				trace.push(format!("has_singleton_nodes_with_selfloops()", ));
				
				let g_copy = graph.clone();
				let trace2 = trace.clone();
				std::panic::set_hook(Box::new(move |info| {
					handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
				}));
				graph.has_singleton_nodes_with_selfloops();
			},
			268 => {
				trace.push(format!("is_connected(verbose = {:?})", data_for_current_test.is_connected.verbose));
				
				let g_copy = graph.clone();
				let trace2 = trace.clone();
				std::panic::set_hook(Box::new(move |info| {
					handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
				}));
				graph.is_connected(data_for_current_test.is_connected.verbose);
			},
			269 => {
				trace.push(format!("has_node_types()", ));
				
				let g_copy = graph.clone();
				let trace2 = trace.clone();
				std::panic::set_hook(Box::new(move |info| {
					handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
				}));
				graph.has_node_types();
			},
			270 => {
				trace.push(format!("has_multilabel_node_types()", ));
				
				let g_copy = graph.clone();
				let trace2 = trace.clone();
				std::panic::set_hook(Box::new(move |info| {
					handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
				}));
				let _ = graph.has_multilabel_node_types();
			},
			271 => {
				trace.push(format!("has_unknown_node_types()", ));
				
				let g_copy = graph.clone();
				let trace2 = trace.clone();
				std::panic::set_hook(Box::new(move |info| {
					handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
				}));
				let _ = graph.has_unknown_node_types();
			},
			272 => {
				trace.push(format!("has_unknown_edge_types()", ));
				
				let g_copy = graph.clone();
				let trace2 = trace.clone();
				std::panic::set_hook(Box::new(move |info| {
					handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
				}));
				let _ = graph.has_unknown_edge_types();
			},
			273 => {
				trace.push(format!("has_homogeneous_node_types()", ));
				
				let g_copy = graph.clone();
				let trace2 = trace.clone();
				std::panic::set_hook(Box::new(move |info| {
					handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
				}));
				let _ = graph.has_homogeneous_node_types();
			},
			274 => {
				trace.push(format!("has_homogeneous_edge_types()", ));
				
				let g_copy = graph.clone();
				let trace2 = trace.clone();
				std::panic::set_hook(Box::new(move |info| {
					handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
				}));
				let _ = graph.has_homogeneous_edge_types();
			},
			275 => {
				trace.push(format!("has_singleton_node_types()", ));
				
				let g_copy = graph.clone();
				let trace2 = trace.clone();
				std::panic::set_hook(Box::new(move |info| {
					handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
				}));
				let _ = graph.has_singleton_node_types();
			},
			276 => {
				trace.push(format!("has_node_oddities()", ));
				
				let g_copy = graph.clone();
				let trace2 = trace.clone();
				std::panic::set_hook(Box::new(move |info| {
					handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
				}));
				graph.has_node_oddities();
			},
			277 => {
				trace.push(format!("has_node_types_oddities()", ));
				
				let g_copy = graph.clone();
				let trace2 = trace.clone();
				std::panic::set_hook(Box::new(move |info| {
					handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
				}));
				let _ = graph.has_node_types_oddities();
			},
			278 => {
				trace.push(format!("has_singleton_edge_types()", ));
				
				let g_copy = graph.clone();
				let trace2 = trace.clone();
				std::panic::set_hook(Box::new(move |info| {
					handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
				}));
				let _ = graph.has_singleton_edge_types();
			},
			279 => {
				trace.push(format!("has_edge_types_oddities()", ));
				
				let g_copy = graph.clone();
				let trace2 = trace.clone();
				std::panic::set_hook(Box::new(move |info| {
					handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
				}));
				let _ = graph.has_edge_types_oddities();
			},
			280 => {
				trace.push(format!("is_multigraph()", ));
				
				let g_copy = graph.clone();
				let trace2 = trace.clone();
				std::panic::set_hook(Box::new(move |info| {
					handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
				}));
				graph.is_multigraph();
			},
			281 => {
				trace.push(format!("compute_hash()", ));
				
				let g_copy = graph.clone();
				let trace2 = trace.clone();
				std::panic::set_hook(Box::new(move |info| {
					handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
				}));
				graph.compute_hash();
			},
			282 => {
				trace.push(format!("connected_holdout(random_state = {:?}, train_size = {:?}, edge_types = {:?}, include_all_edge_types = {:?}, verbose = {:?})", data_for_current_test.connected_holdout.random_state, data_for_current_test.connected_holdout.train_size, data_for_current_test.connected_holdout.edge_types, data_for_current_test.connected_holdout.include_all_edge_types, data_for_current_test.connected_holdout.verbose));
				
				let g_copy = graph.clone();
				let trace2 = trace.clone();
				std::panic::set_hook(Box::new(move |info| {
					handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
				}));
				let _ = graph.connected_holdout(data_for_current_test.connected_holdout.random_state, data_for_current_test.connected_holdout.train_size, data_for_current_test.connected_holdout.edge_types, data_for_current_test.connected_holdout.include_all_edge_types, data_for_current_test.connected_holdout.verbose);
			},
			283 => {
				trace.push(format!("random_holdout(random_state = {:?}, train_size = {:?}, include_all_edge_types = {:?}, edge_types = {:?}, min_number_overlaps = {:?}, verbose = {:?})", data_for_current_test.random_holdout.random_state, data_for_current_test.random_holdout.train_size, data_for_current_test.random_holdout.include_all_edge_types, data_for_current_test.random_holdout.edge_types, data_for_current_test.random_holdout.min_number_overlaps, data_for_current_test.random_holdout.verbose));
				
				let g_copy = graph.clone();
				let trace2 = trace.clone();
				std::panic::set_hook(Box::new(move |info| {
					handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
				}));
				let _ = graph.random_holdout(data_for_current_test.random_holdout.random_state, data_for_current_test.random_holdout.train_size, data_for_current_test.random_holdout.include_all_edge_types, data_for_current_test.random_holdout.edge_types, data_for_current_test.random_holdout.min_number_overlaps, data_for_current_test.random_holdout.verbose);
			},
			284 => {
				trace.push(format!("node_label_holdout(train_size = {:?}, use_stratification = {:?}, random_state = {:?})", data_for_current_test.node_label_holdout.train_size, data_for_current_test.node_label_holdout.use_stratification, data_for_current_test.node_label_holdout.random_state));
				
				let g_copy = graph.clone();
				let trace2 = trace.clone();
				std::panic::set_hook(Box::new(move |info| {
					handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
				}));
				let _ = graph.node_label_holdout(data_for_current_test.node_label_holdout.train_size, data_for_current_test.node_label_holdout.use_stratification, data_for_current_test.node_label_holdout.random_state);
			},
			285 => {
				trace.push(format!("edge_label_holdout(train_size = {:?}, use_stratification = {:?}, random_state = {:?})", data_for_current_test.edge_label_holdout.train_size, data_for_current_test.edge_label_holdout.use_stratification, data_for_current_test.edge_label_holdout.random_state));
				
				let g_copy = graph.clone();
				let trace2 = trace.clone();
				std::panic::set_hook(Box::new(move |info| {
					handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
				}));
				let _ = graph.edge_label_holdout(data_for_current_test.edge_label_holdout.train_size, data_for_current_test.edge_label_holdout.use_stratification, data_for_current_test.edge_label_holdout.random_state);
			},
			286 => {
				trace.push(format!("random_subgraph(random_state = {:?}, nodes_number = {:?}, verbose = {:?})", data_for_current_test.random_subgraph.random_state, data_for_current_test.random_subgraph.nodes_number, data_for_current_test.random_subgraph.verbose));
				
				let g_copy = graph.clone();
				let trace2 = trace.clone();
				std::panic::set_hook(Box::new(move |info| {
					handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
				}));
				let _ = graph.random_subgraph(data_for_current_test.random_subgraph.random_state, data_for_current_test.random_subgraph.nodes_number, data_for_current_test.random_subgraph.verbose);
			},
			287 => {
				trace.push(format!("kfold(k = {:?}, k_index = {:?}, edge_types = {:?}, random_state = {:?}, verbose = {:?})", data_for_current_test.kfold.k, data_for_current_test.kfold.k_index, data_for_current_test.kfold.edge_types, data_for_current_test.kfold.random_state, data_for_current_test.kfold.verbose));
				
				let g_copy = graph.clone();
				let trace2 = trace.clone();
				std::panic::set_hook(Box::new(move |info| {
					handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
				}));
				let _ = graph.kfold(data_for_current_test.kfold.k, data_for_current_test.kfold.k_index, data_for_current_test.kfold.edge_types, data_for_current_test.kfold.random_state, data_for_current_test.kfold.verbose);
			},
		288 => {let _ = graph::test_utilities::default_test_suite(&mut graph, false);}
            _ => unreachable!()
        }
    }
    
    let _ = graph::test_utilities::default_test_suite(&mut graph, false);

    Ok(())
}