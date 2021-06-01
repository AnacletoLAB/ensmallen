
use super::*;
use arbitrary::Arbitrary;
use std::collections::{HashSet, HashMap};
use rayon::iter::ParallelIterator;

struct Rng{
    seed: u64
}

impl Rng {
    pub fn new(seed: u64) -> Rng {
        Rng{
            seed: seed,
        }
    }

    pub fn next(&mut self) -> u64 {
        let mut x = self.seed;
        x = x.wrapping_add(0xbadf00ddeadbeef);
        x ^= x << 17;
        x ^= x >> 7;
        x ^= x << 13;
        self.seed = x;
        x
    }
}


#[derive(Arbitrary, Debug, Clone)]
pub struct SpanningArborescenceKruskal {
    pub verbose : Option<bool>,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct SpanningArborescence {
    pub verbose : Option<bool>,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct ConnectedComponents {
    pub verbose : Option<bool>,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct GetNodeLabelPredictionTupleFromNodeIds {
    pub node_ids : Vec<NodeT>,
    pub random_state : u64,
    pub include_central_node : bool,
    pub offset : NodeT,
    pub max_neighbours : Option<NodeT>,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct ParIterUncheckedEdgePredictionMetrics {
    pub source_node_ids : Vec<NodeT>,
    pub destination_node_ids : Vec<NodeT>,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct GetOkapiBm25NodeFeaturePropagation {
    pub features : Vec<Vec<f64>>,
    pub iterations : Option<usize>,
    pub maximal_distance : Option<usize>,
    pub k1 : Option<f64>,
    pub b : Option<f64>,
    pub include_central_node : Option<bool>,
    pub verbose : Option<bool>,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct GetOkapiBm25NodeLabelPropagation {
    pub iterations : Option<usize>,
    pub maximal_distance : Option<usize>,
    pub k1 : Option<f64>,
    pub b : Option<f64>,
    pub verbose : Option<bool>,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct ToDot {
    pub use_node_names : Option<bool>,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct IsUncheckedSingletonFromNodeId {
    pub node_id : NodeT,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct IsUncheckedDisconnectedFromNodeId {
    pub node_id : NodeT,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct IsUncheckedConnectedFromNodeId {
    pub node_id : NodeT,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct IsSingletonFromNodeId {
    pub node_id : NodeT,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct IsSingletonWithSelfloopsFromNodeId {
    pub node_id : NodeT,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct HasNodeTypeId {
    pub node_type_id : NodeTypeT,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct HasEdgeTypeId {
    pub edge_type_id : EdgeTypeT,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct HasEdgeFromNodeIds {
    pub src : NodeT,
    pub dst : NodeT,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct HasSelfloopFromNodeId {
    pub node_id : NodeT,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct HasEdgeFromNodeIdsAndEdgeTypeId {
    pub src : NodeT,
    pub dst : NodeT,
    pub edge_type : Option<EdgeTypeT>,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct IsUncheckedTrapNodeFromNodeId {
    pub node_id : NodeT,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct IsTrapNodeFromNodeId {
    pub node_id : NodeT,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct GetUncheckedPreferentialAttachment {
    pub one : NodeT,
    pub two : NodeT,
    pub normalize : bool,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct GetPreferentialAttachment {
    pub one : NodeT,
    pub two : NodeT,
    pub normalize : bool,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct GetUncheckedJaccardCoefficient {
    pub one : NodeT,
    pub two : NodeT,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct GetJaccardCoefficient {
    pub one : NodeT,
    pub two : NodeT,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct GetUncheckedAdamicAdarIndex {
    pub one : NodeT,
    pub two : NodeT,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct GetAdamicAdarIndex {
    pub one : NodeT,
    pub two : NodeT,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct GetUncheckedResourceAllocationIndex {
    pub one : NodeT,
    pub two : NodeT,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct GetResourceAllocationIndex {
    pub one : NodeT,
    pub two : NodeT,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct Enable {
    pub vector_sources : Option<bool>,
    pub vector_destinations : Option<bool>,
    pub vector_cumulative_node_degrees : Option<bool>,
    pub cache_size : Option<f64>,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct GetUnweightedNumberOfTriangles {
    pub normalize : Option<bool>,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct GetUnweightedNumberOfTrianglesPerNode {
    pub normalize : Option<bool>,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct GetUncheckedBreathFirstSearchFromNodeIds {
    pub src_node_id : NodeT,
    pub maybe_dst_node_id : Option<NodeT>,
    pub maybe_dst_node_ids : Option<Vec<NodeT>>,
    pub compute_distances : Option<bool>,
    pub compute_predecessors : Option<bool>,
    pub compute_visited : Option<bool>,
    pub maximal_depth : Option<NodeT>,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct GetUncheckedUnweightedMinimumPathNodeIdsFromNodeIds {
    pub src_node_id : NodeT,
    pub dst_node_id : NodeT,
    pub maximal_depth : Option<NodeT>,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct GetUncheckedUnweightedMinimumPathNodeNamesFromNodeIds {
    pub src_node_id : NodeT,
    pub dst_node_id : NodeT,
    pub maximal_depth : Option<NodeT>,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct GetUnweightedMinimumPathNodeIdsFromNodeIds {
    pub src_node_id : NodeT,
    pub dst_node_id : NodeT,
    pub maximal_depth : Option<NodeT>,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct GetUncheckedUnweightedKShortestPathNodeIdsFromNodeIds {
    pub src_node_id : NodeT,
    pub dst_node_id : NodeT,
    pub k : usize,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct GetUnweightedKShortestPathNodeIdsFromNodeIds {
    pub src_node_id : NodeT,
    pub dst_node_id : NodeT,
    pub k : usize,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct GetUncheckedUnweightedEccentricityFromNodeId {
    pub node_id : NodeT,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct GetUncheckedWeightedEccentricityFromNodeId {
    pub node_id : NodeT,
    pub use_edge_weights_as_probabilities : Option<bool>,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct GetUnweightedEccentricityFromNodeId {
    pub node_id : NodeT,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct GetWeightedEccentricityFromNodeId {
    pub node_id : NodeT,
    pub use_edge_weights_as_probabilities : Option<bool>,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct GetUncheckedDijkstraFromNodeIds {
    pub src_node_id : NodeT,
    pub maybe_dst_node_id : Option<NodeT>,
    pub maybe_dst_node_ids : Option<Vec<NodeT>>,
    pub compute_predecessors : Option<bool>,
    pub maximal_depth : Option<NodeT>,
    pub use_edge_weights_as_probabilities : Option<bool>,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct GetUncheckedWeightedMinimumPathNodeIdsFromNodeIds {
    pub src_node_id : NodeT,
    pub dst_node_id : NodeT,
    pub use_edge_weights_as_probabilities : Option<bool>,
    pub maximal_depth : Option<NodeT>,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct GetUncheckedWeightedMinimumPathNodeNamesFromNodeIds {
    pub src_node_id : NodeT,
    pub dst_node_id : NodeT,
    pub use_edge_weights_as_probabilities : Option<bool>,
    pub maximal_depth : Option<NodeT>,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct GetWeightedMinimumPathNodeIdsFromNodeIds {
    pub src_node_id : NodeT,
    pub dst_node_id : NodeT,
    pub use_edge_weights_as_probabilities : Option<bool>,
    pub maximal_depth : Option<NodeT>,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct GetBreathFirstSearchFromNodeIds {
    pub src_node_id : NodeT,
    pub maybe_dst_node_id : Option<NodeT>,
    pub maybe_dst_node_ids : Option<Vec<NodeT>>,
    pub compute_distances : Option<bool>,
    pub compute_predecessors : Option<bool>,
    pub compute_visited : Option<bool>,
    pub maximal_depth : Option<NodeT>,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct GetDijkstraFromNodeIds {
    pub src_node_id : NodeT,
    pub maybe_dst_node_id : Option<NodeT>,
    pub maybe_dst_node_ids : Option<Vec<NodeT>>,
    pub compute_predecessors : Option<bool>,
    pub maximal_depth : Option<NodeT>,
    pub use_edge_weights_as_probabilities : Option<bool>,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct GetUnweightedDiameter {
    pub ignore_infinity : Option<bool>,
    pub verbose : Option<bool>,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct GetWeightedDiameter {
    pub ignore_infinity : Option<bool>,
    pub use_edge_weights_as_probabilities : Option<bool>,
    pub verbose : Option<bool>,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct IterUncheckedEdgeIdsFromSourceNodeId {
    pub src : NodeT,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct IterUncheckedEdgeWeightsFromSourceNodeId {
    pub source_node_id : NodeT,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct ParIterUncheckedEdgeIdsFromSourceNodeId {
    pub src : NodeT,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct IterUncheckedEdgeIdsFromNodeIds {
    pub src : NodeT,
    pub dst : NodeT,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct IterUncheckedNeighbourNodeIdsFromSourceNodeId {
    pub src : NodeT,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct IterUncheckedNeighbourNodeIdsIntersectionFromSourceNodeIds {
    pub first_src_node_id : NodeT,
    pub second_src_node_id : NodeT,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct IterUncheckedNeighbourNodeIdsUnionFromSourceNodeIds {
    pub first_src_node_id : NodeT,
    pub second_src_node_id : NodeT,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct IterUncheckedNeighbourNodeIdsDifferenceFromSourceNodeIds {
    pub first_src_node_id : NodeT,
    pub second_src_node_id : NodeT,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct IterUncheckedNeighbourNodeNamesFromSourceNodeId {
    pub src : NodeT,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct IterEdgeIdsFromNodeIds {
    pub src : NodeT,
    pub dst : NodeT,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct IterEdgeNodeIdsAndEdgeTypeIdFromEdgeTypeId {
    pub edge_type_id : Option<EdgeTypeT>,
    pub directed : bool,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct IterNodeIdsAndNodeTypeIdsFromNodeTypeId {
    pub node_type_id : Option<NodeTypeT>,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct IterNodeNamesAndNodeTypeNamesFromNodeTypeId {
    pub node_type_id : Option<NodeTypeT>,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct IterEdgeNodeNamesAndEdgeTypeNameFromEdgeTypeId {
    pub edge_type_id : Option<EdgeTypeT>,
    pub directed : bool,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct GetTransitiveClosure {
    pub iterations : Option<NodeT>,
    pub verbose : Option<bool>,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct GetUnweightedAllShortestPaths {
    pub iterations : Option<NodeT>,
    pub verbose : Option<bool>,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct GetWeightedAllShortestPaths {
    pub iterations : Option<NodeT>,
    pub use_edge_weights_as_probabilities : Option<bool>,
    pub verbose : Option<bool>,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct GetUncheckedEdgeWeightFromEdgeId {
    pub edge_id : EdgeT,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct GetUncheckedEdgeWeightFromNodeIds {
    pub src : NodeT,
    pub dst : NodeT,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct GetUncheckedEdgeTypeNameFromEdgeTypeId {
    pub edge_type_id : Option<EdgeTypeT>,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct GetUncheckedEdgeCountFromEdgeTypeId {
    pub edge_type : Option<EdgeTypeT>,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct GetUncheckedEdgeIdFromNodeIdsAndEdgeTypeId {
    pub src : NodeT,
    pub dst : NodeT,
    pub edge_type : Option<EdgeTypeT>,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct GetUncheckedMinmaxEdgeIdsFromNodeIds {
    pub src : NodeT,
    pub dst : NodeT,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct GetUncheckedNodeIdsFromEdgeId {
    pub edge_id : EdgeT,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct GetNodeIdsFromEdgeId {
    pub edge_id : EdgeT,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct GetUncheckedEdgeIdFromNodeIds {
    pub src : NodeT,
    pub dst : NodeT,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct GetEdgeIdFromNodeIds {
    pub src : NodeT,
    pub dst : NodeT,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct GetUncheckedUniqueSourceNodeId {
    pub source_id : NodeT,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct GetUncheckedNodeIdsAndEdgeTypeIdFromEdgeId {
    pub edge_id : EdgeT,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct GetNodeIdsAndEdgeTypeIdFromEdgeId {
    pub edge_id : EdgeT,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct GetUncheckedNodeIdsAndEdgeTypeIdAndEdgeWeightFromEdgeId {
    pub edge_id : EdgeT,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct GetNodeIdsAndEdgeTypeIdAndEdgeWeightFromEdgeId {
    pub edge_id : EdgeT,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct GetTopKCentralNodeIds {
    pub k : NodeT,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct GetUncheckedUnweightedNodeDegreeFromNodeId {
    pub node_id : NodeT,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct GetUncheckedWeightedNodeDegreeFromNodeId {
    pub node_id : NodeT,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct GetUnweightedNodeDegreeFromNodeId {
    pub node_id : NodeT,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct GetWeightedNodeDegreeFromNodeId {
    pub node_id : NodeT,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct GetTopKCentralNodeNames {
    pub k : NodeT,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct GetUncheckedNodeTypeIdFromNodeId {
    pub node_id : NodeT,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct GetNodeTypeIdFromNodeId {
    pub node_id : NodeT,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct GetUncheckedEdgeTypeIdFromEdgeId {
    pub edge_id : EdgeT,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct GetEdgeTypeIdFromEdgeId {
    pub edge_id : EdgeT,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct GetUncheckedNodeTypeNamesFromNodeId {
    pub node_id : NodeT,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct GetNodeTypeNamesFromNodeId {
    pub node_id : NodeT,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct GetEdgeTypeNameFromEdgeId {
    pub edge_id : EdgeT,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct GetEdgeTypeNameFromEdgeTypeId {
    pub edge_type_id : EdgeTypeT,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct GetEdgeWeightFromEdgeId {
    pub edge_id : EdgeT,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct GetEdgeWeightFromNodeIds {
    pub src : NodeT,
    pub dst : NodeT,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct GetEdgeWeightFromNodeIdsAndEdgeTypeId {
    pub src : NodeT,
    pub dst : NodeT,
    pub edge_type : Option<EdgeTypeT>,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct GetUncheckedNodeNameFromNodeId {
    pub node_id : NodeT,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct GetNodeNameFromNodeId {
    pub node_id : NodeT,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct GetEdgeNodeNamesFromEdgeNodeIds {
    pub edge_node_ids : Vec<(NodeT, NodeT)>,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct GetEdgeCountFromEdgeTypeId {
    pub edge_type_id : Option<EdgeTypeT>,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct GetNodeCountFromNodeTypeId {
    pub node_type_id : Option<NodeTypeT>,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct GetDestinationNodeIdFromEdgeId {
    pub edge_id : EdgeT,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct GetNeighbourNodeIdsFromNodeId {
    pub node_id : NodeT,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct GetMinmaxEdgeIdsFromNodeIds {
    pub src : NodeT,
    pub dst : NodeT,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct GetEdgeIdFromNodeIdsAndEdgeTypeId {
    pub src : NodeT,
    pub dst : NodeT,
    pub edge_type : Option<EdgeTypeT>,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct GetUncheckedMinmaxEdgeIdsFromSourceNodeId {
    pub src : NodeT,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct GetMinmaxEdgeIdsFromSourceNodeId {
    pub src : NodeT,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct GetNodeTypeNameFromNodeTypeId {
    pub node_type_id : NodeTypeT,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct GetUncheckedNodeTypeNamesFromNodeTypeIds {
    pub node_type_ids : Vec<NodeTypeT>,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct FilterFromIds {
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
    pub filter_singleton_nodes : Option<bool>,
    pub filter_singleton_nodes_with_selfloop : Option<bool>,
    pub filter_selfloops : Option<bool>,
    pub filter_parallel_edges : Option<bool>,
    pub verbose : Option<bool>,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct DropUnknownNodeTypes {
    pub verbose : Option<bool>,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct DropUnknownEdgeTypes {
    pub verbose : Option<bool>,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct DropSingletonNodes {
    pub verbose : Option<bool>,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct DropSingletonNodesWithSelfloops {
    pub verbose : Option<bool>,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct DropSelfloops {
    pub verbose : Option<bool>,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct DropParallelEdges {
    pub verbose : Option<bool>,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct ValidateNodeId {
    pub node_id : NodeT,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct ValidateNodeIds {
    pub node_ids : Vec<NodeT>,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct ValidateEdgeId {
    pub edge_id : EdgeT,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct ValidateEdgeIds {
    pub edge_ids : Vec<EdgeT>,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct ValidateNodeTypeId {
    pub node_type_id : Option<NodeTypeT>,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct ValidateNodeTypeIds {
    pub node_type_ids : Vec<Option<NodeTypeT>>,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct ValidateEdgeTypeId {
    pub edge_type_id : Option<EdgeTypeT>,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct ValidateEdgeTypeIds {
    pub edge_type_ids : Vec<Option<EdgeTypeT>>,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct GetUncheckedUnweightedClosenessCentralityFromNodeId {
    pub node_id : NodeT,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct GetUncheckedWeightedClosenessCentralityFromNodeId {
    pub node_id : NodeT,
    pub use_edge_weights_as_probabilities : bool,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct ParIterUnweightedClosenessCentrality {
    pub verbose : Option<bool>,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct ParIterWeightedClosenessCentrality {
    pub use_edge_weights_as_probabilities : Option<bool>,
    pub verbose : Option<bool>,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct GetUnweightedClosenessCentrality {
    pub verbose : Option<bool>,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct GetWeightedClosenessCentrality {
    pub use_edge_weights_as_probabilities : Option<bool>,
    pub verbose : Option<bool>,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct GetUncheckedUnweightedHarmonicCentralityFromNodeId {
    pub node_id : NodeT,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct GetUncheckedWeightedHarmonicCentralityFromNodeId {
    pub node_id : NodeT,
    pub use_edge_weights_as_probabilities : bool,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct ParIterUnweightedHarmonicCentrality {
    pub verbose : Option<bool>,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct ParIterWeightedHarmonicCentrality {
    pub use_edge_weights_as_probabilities : Option<bool>,
    pub verbose : Option<bool>,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct GetUnweightedHarmonicCentrality {
    pub verbose : Option<bool>,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct GetWeightedHarmonicCentrality {
    pub use_edge_weights_as_probabilities : Option<bool>,
    pub verbose : Option<bool>,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct GetStressCentrality {
    pub normalize : Option<bool>,
    pub verbose : Option<bool>,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct GetBetweennessCentrality {
    pub normalize : Option<bool>,
    pub verbose : Option<bool>,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct GetUnweightedEigenvectorCentrality {
    pub maximum_iterations_number : Option<usize>,
    pub tollerance : Option<f64>,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct GetWeightedEigenvectorCentrality {
    pub maximum_iterations_number : Option<usize>,
    pub tollerance : Option<f64>,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct RemoveInplaceNodeTypeIds {
    pub node_type_ids_to_remove : Vec<NodeTypeT>,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct RemoveInplaceEdgeTypeIds {
    pub edge_type_ids_to_remove : Vec<EdgeTypeT>,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct RemoveNodeTypeId {
    pub node_type_id : NodeTypeT,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct RemoveEdgeTypeId {
    pub edge_type_id : EdgeTypeT,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct RemoveEdgeTypes {
    pub verbose : Option<bool>,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct EncodeEdge {
    pub src : NodeT,
    pub dst : NodeT,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct DecodeEdge {
    pub edge : u64,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct GetNodeReportFromNodeId {
    pub node_id : NodeT,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct TextualReport {
    pub verbose : Option<bool>,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct GetConnectedComponentsNumber {
    pub verbose : Option<bool>,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct GetSourceNodeIds {
    pub directed : bool,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct GetSourceNames {
    pub directed : bool,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct GetDestinationNodeIds {
    pub directed : bool,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct GetDestinationNames {
    pub directed : bool,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct GetEdgeNodeIds {
    pub directed : bool,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct GetEdgeNodeNames {
    pub directed : bool,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct GetNodeConnectedComponentIds {
    pub verbose : Option<bool>,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct GetDenseWeightedAdjacencyMatrix {
    pub weight : Option<WeightT>,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct IterSourceNodeIds {
    pub directed : bool,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct ParIterSourceNodeIds {
    pub directed : bool,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct IterDestinationNodeIds {
    pub directed : bool,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct ParIterDestinationNodeIds {
    pub directed : bool,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct IterEdgeNodeIds {
    pub directed : bool,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct IterEdges {
    pub directed : bool,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct ParIterEdgeNodeIds {
    pub directed : bool,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct ParIterEdges {
    pub directed : bool,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct IterEdgeNodeIdsAndEdgeTypeId {
    pub directed : bool,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct IterEdgeNodeNamesAndEdgeTypeName {
    pub directed : bool,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct ParIterEdgeNodeNamesAndEdgeTypeName {
    pub directed : bool,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct ParIterEdgeNodeIdsAndEdgeTypeId {
    pub directed : bool,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct ParIterEdgeNodeNamesAndEdgeTypeNameAndEdgeWeight {
    pub directed : bool,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct IterEdgeNodeNamesAndEdgeTypeNameAndEdgeWeight {
    pub directed : bool,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct ParIterEdgeNodeIdsAndEdgeTypeIdAndEdgeWeight {
    pub directed : bool,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct IterEdgeNodeIdsAndEdgeTypeIdAndEdgeWeight {
    pub directed : bool,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct IterUniqueEdgeNodeIds {
    pub directed : bool,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct IterEdgeNodeIdsWithUnknownEdgeTypes {
    pub directed : bool,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct IterEdgeNodeIdsWithKnownEdgeTypes {
    pub directed : bool,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct ParIterEdgeNodeIdsWithUnknownEdgeTypes {
    pub directed : bool,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct ParIterEdgeNodeIdsWithKnownEdgeTypes {
    pub directed : bool,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct GetUnweightedLaplacianTransformedGraph {
    pub verbose : Option<bool>,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct GetUnweightedRandomWalkNormalizedLaplacianTransformedGraph {
    pub verbose : Option<bool>,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct GetUnweightedSymmetricNormalizedLaplacianTransformedGraph {
    pub verbose : Option<bool>,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct GetUnweightedSymmetricNormalizedTransformedGraph {
    pub verbose : Option<bool>,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct GetWeightedLaplacianTransformedGraph {
    pub verbose : Option<bool>,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct GetWeightedSymmetricNormalizedLaplacianTransformedGraph {
    pub verbose : Option<bool>,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct GetWeightedSymmetricNormalizedTransformedGraph {
    pub verbose : Option<bool>,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct GetWeightedRandomWalkNormalizedLaplacianTransformedGraph {
    pub verbose : Option<bool>,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct IsConnected {
    pub verbose : Option<bool>,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct NodeLabelHoldout {
    pub train_size : f64,
    pub use_stratification : Option<bool>,
    pub random_state : Option<EdgeT>,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct EdgeLabelHoldout {
    pub train_size : f64,
    pub use_stratification : Option<bool>,
    pub random_state : Option<EdgeT>,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct RandomSubgraph {
    pub nodes_number : NodeT,
    pub random_state : Option<usize>,
    pub verbose : Option<bool>,
}



#[derive(Arbitrary, Debug, Clone)]
pub struct MetaParams {
    pub seed: u64,
    pub spanningarborescencekruskal : SpanningArborescenceKruskal,
    pub spanningarborescence : SpanningArborescence,
    pub connectedcomponents : ConnectedComponents,
    pub getnodelabelpredictiontuplefromnodeids : GetNodeLabelPredictionTupleFromNodeIds,
    pub pariteruncheckededgepredictionmetrics : ParIterUncheckedEdgePredictionMetrics,
    pub getokapibm25nodefeaturepropagation : GetOkapiBm25NodeFeaturePropagation,
    pub getokapibm25nodelabelpropagation : GetOkapiBm25NodeLabelPropagation,
    pub todot : ToDot,
    pub isuncheckedsingletonfromnodeid : IsUncheckedSingletonFromNodeId,
    pub isuncheckeddisconnectedfromnodeid : IsUncheckedDisconnectedFromNodeId,
    pub isuncheckedconnectedfromnodeid : IsUncheckedConnectedFromNodeId,
    pub issingletonfromnodeid : IsSingletonFromNodeId,
    pub issingletonwithselfloopsfromnodeid : IsSingletonWithSelfloopsFromNodeId,
    pub hasnodetypeid : HasNodeTypeId,
    pub hasedgetypeid : HasEdgeTypeId,
    pub hasedgefromnodeids : HasEdgeFromNodeIds,
    pub hasselfloopfromnodeid : HasSelfloopFromNodeId,
    pub hasedgefromnodeidsandedgetypeid : HasEdgeFromNodeIdsAndEdgeTypeId,
    pub isuncheckedtrapnodefromnodeid : IsUncheckedTrapNodeFromNodeId,
    pub istrapnodefromnodeid : IsTrapNodeFromNodeId,


    pub getuncheckedpreferentialattachment : GetUncheckedPreferentialAttachment,
    pub getpreferentialattachment : GetPreferentialAttachment,
    pub getuncheckedjaccardcoefficient : GetUncheckedJaccardCoefficient,
    pub getjaccardcoefficient : GetJaccardCoefficient,
    pub getuncheckedadamicadarindex : GetUncheckedAdamicAdarIndex,
    pub getadamicadarindex : GetAdamicAdarIndex,
    pub getuncheckedresourceallocationindex : GetUncheckedResourceAllocationIndex,
    pub getresourceallocationindex : GetResourceAllocationIndex,
    pub enable : Enable,

    pub getunweightednumberoftriangles : GetUnweightedNumberOfTriangles,



    pub getunweightednumberoftrianglespernode : GetUnweightedNumberOfTrianglesPerNode,




    pub getuncheckedbreathfirstsearchfromnodeids : GetUncheckedBreathFirstSearchFromNodeIds,
    pub getuncheckedunweightedminimumpathnodeidsfromnodeids : GetUncheckedUnweightedMinimumPathNodeIdsFromNodeIds,
    pub getuncheckedunweightedminimumpathnodenamesfromnodeids : GetUncheckedUnweightedMinimumPathNodeNamesFromNodeIds,
    pub getunweightedminimumpathnodeidsfromnodeids : GetUnweightedMinimumPathNodeIdsFromNodeIds,
    pub getuncheckedunweightedkshortestpathnodeidsfromnodeids : GetUncheckedUnweightedKShortestPathNodeIdsFromNodeIds,
    pub getunweightedkshortestpathnodeidsfromnodeids : GetUnweightedKShortestPathNodeIdsFromNodeIds,
    pub getuncheckedunweightedeccentricityfromnodeid : GetUncheckedUnweightedEccentricityFromNodeId,
    pub getuncheckedweightedeccentricityfromnodeid : GetUncheckedWeightedEccentricityFromNodeId,
    pub getunweightedeccentricityfromnodeid : GetUnweightedEccentricityFromNodeId,
    pub getweightedeccentricityfromnodeid : GetWeightedEccentricityFromNodeId,
    pub getuncheckeddijkstrafromnodeids : GetUncheckedDijkstraFromNodeIds,
    pub getuncheckedweightedminimumpathnodeidsfromnodeids : GetUncheckedWeightedMinimumPathNodeIdsFromNodeIds,
    pub getuncheckedweightedminimumpathnodenamesfromnodeids : GetUncheckedWeightedMinimumPathNodeNamesFromNodeIds,
    pub getweightedminimumpathnodeidsfromnodeids : GetWeightedMinimumPathNodeIdsFromNodeIds,
    pub getbreathfirstsearchfromnodeids : GetBreathFirstSearchFromNodeIds,
    pub getdijkstrafromnodeids : GetDijkstraFromNodeIds,
    pub getunweighteddiameter : GetUnweightedDiameter,
    pub getweighteddiameter : GetWeightedDiameter,
    pub iteruncheckededgeidsfromsourcenodeid : IterUncheckedEdgeIdsFromSourceNodeId,
    pub iteruncheckededgeweightsfromsourcenodeid : IterUncheckedEdgeWeightsFromSourceNodeId,
    pub pariteruncheckededgeidsfromsourcenodeid : ParIterUncheckedEdgeIdsFromSourceNodeId,
    pub iteruncheckededgeidsfromnodeids : IterUncheckedEdgeIdsFromNodeIds,
    pub iteruncheckedneighbournodeidsfromsourcenodeid : IterUncheckedNeighbourNodeIdsFromSourceNodeId,
    pub iteruncheckedneighbournodeidsintersectionfromsourcenodeids : IterUncheckedNeighbourNodeIdsIntersectionFromSourceNodeIds,
    pub iteruncheckedneighbournodeidsunionfromsourcenodeids : IterUncheckedNeighbourNodeIdsUnionFromSourceNodeIds,
    pub iteruncheckedneighbournodeidsdifferencefromsourcenodeids : IterUncheckedNeighbourNodeIdsDifferenceFromSourceNodeIds,
    pub iteruncheckedneighbournodenamesfromsourcenodeid : IterUncheckedNeighbourNodeNamesFromSourceNodeId,
    pub iteredgeidsfromnodeids : IterEdgeIdsFromNodeIds,
    pub iteredgenodeidsandedgetypeidfromedgetypeid : IterEdgeNodeIdsAndEdgeTypeIdFromEdgeTypeId,
    pub iternodeidsandnodetypeidsfromnodetypeid : IterNodeIdsAndNodeTypeIdsFromNodeTypeId,
    pub iternodenamesandnodetypenamesfromnodetypeid : IterNodeNamesAndNodeTypeNamesFromNodeTypeId,
    pub iteredgenodenamesandedgetypenamefromedgetypeid : IterEdgeNodeNamesAndEdgeTypeNameFromEdgeTypeId,
    pub gettransitiveclosure : GetTransitiveClosure,
    pub getunweightedallshortestpaths : GetUnweightedAllShortestPaths,
    pub getweightedallshortestpaths : GetWeightedAllShortestPaths,

    pub getuncheckededgeweightfromedgeid : GetUncheckedEdgeWeightFromEdgeId,
    pub getuncheckededgeweightfromnodeids : GetUncheckedEdgeWeightFromNodeIds,
    pub getuncheckededgetypenamefromedgetypeid : GetUncheckedEdgeTypeNameFromEdgeTypeId,
    pub getuncheckededgecountfromedgetypeid : GetUncheckedEdgeCountFromEdgeTypeId,
    pub getuncheckededgeidfromnodeidsandedgetypeid : GetUncheckedEdgeIdFromNodeIdsAndEdgeTypeId,
    pub getuncheckedminmaxedgeidsfromnodeids : GetUncheckedMinmaxEdgeIdsFromNodeIds,
    pub getuncheckednodeidsfromedgeid : GetUncheckedNodeIdsFromEdgeId,
    pub getnodeidsfromedgeid : GetNodeIdsFromEdgeId,
    pub getuncheckededgeidfromnodeids : GetUncheckedEdgeIdFromNodeIds,
    pub getedgeidfromnodeids : GetEdgeIdFromNodeIds,
    pub getuncheckeduniquesourcenodeid : GetUncheckedUniqueSourceNodeId,
    pub getuncheckednodeidsandedgetypeidfromedgeid : GetUncheckedNodeIdsAndEdgeTypeIdFromEdgeId,
    pub getnodeidsandedgetypeidfromedgeid : GetNodeIdsAndEdgeTypeIdFromEdgeId,
    pub getuncheckednodeidsandedgetypeidandedgeweightfromedgeid : GetUncheckedNodeIdsAndEdgeTypeIdAndEdgeWeightFromEdgeId,
    pub getnodeidsandedgetypeidandedgeweightfromedgeid : GetNodeIdsAndEdgeTypeIdAndEdgeWeightFromEdgeId,
    pub gettopkcentralnodeids : GetTopKCentralNodeIds,
    pub getuncheckedunweightednodedegreefromnodeid : GetUncheckedUnweightedNodeDegreeFromNodeId,
    pub getuncheckedweightednodedegreefromnodeid : GetUncheckedWeightedNodeDegreeFromNodeId,
    pub getunweightednodedegreefromnodeid : GetUnweightedNodeDegreeFromNodeId,
    pub getweightednodedegreefromnodeid : GetWeightedNodeDegreeFromNodeId,
    pub gettopkcentralnodenames : GetTopKCentralNodeNames,
    pub getuncheckednodetypeidfromnodeid : GetUncheckedNodeTypeIdFromNodeId,
    pub getnodetypeidfromnodeid : GetNodeTypeIdFromNodeId,
    pub getuncheckededgetypeidfromedgeid : GetUncheckedEdgeTypeIdFromEdgeId,
    pub getedgetypeidfromedgeid : GetEdgeTypeIdFromEdgeId,
    pub getuncheckednodetypenamesfromnodeid : GetUncheckedNodeTypeNamesFromNodeId,
    pub getnodetypenamesfromnodeid : GetNodeTypeNamesFromNodeId,
    pub getedgetypenamefromedgeid : GetEdgeTypeNameFromEdgeId,
    pub getedgetypenamefromedgetypeid : GetEdgeTypeNameFromEdgeTypeId,
    pub getedgeweightfromedgeid : GetEdgeWeightFromEdgeId,
    pub getedgeweightfromnodeids : GetEdgeWeightFromNodeIds,
    pub getedgeweightfromnodeidsandedgetypeid : GetEdgeWeightFromNodeIdsAndEdgeTypeId,
    pub getuncheckednodenamefromnodeid : GetUncheckedNodeNameFromNodeId,
    pub getnodenamefromnodeid : GetNodeNameFromNodeId,
    pub getedgenodenamesfromedgenodeids : GetEdgeNodeNamesFromEdgeNodeIds,
    pub getedgecountfromedgetypeid : GetEdgeCountFromEdgeTypeId,
    pub getnodecountfromnodetypeid : GetNodeCountFromNodeTypeId,
    pub getdestinationnodeidfromedgeid : GetDestinationNodeIdFromEdgeId,
    pub getneighbournodeidsfromnodeid : GetNeighbourNodeIdsFromNodeId,
    pub getminmaxedgeidsfromnodeids : GetMinmaxEdgeIdsFromNodeIds,
    pub getedgeidfromnodeidsandedgetypeid : GetEdgeIdFromNodeIdsAndEdgeTypeId,
    pub getuncheckedminmaxedgeidsfromsourcenodeid : GetUncheckedMinmaxEdgeIdsFromSourceNodeId,
    pub getminmaxedgeidsfromsourcenodeid : GetMinmaxEdgeIdsFromSourceNodeId,
    pub getnodetypenamefromnodetypeid : GetNodeTypeNameFromNodeTypeId,
    pub getuncheckednodetypenamesfromnodetypeids : GetUncheckedNodeTypeNamesFromNodeTypeIds,
    pub filterfromids : FilterFromIds,
    pub dropunknownnodetypes : DropUnknownNodeTypes,
    pub dropunknownedgetypes : DropUnknownEdgeTypes,
    pub dropsingletonnodes : DropSingletonNodes,
    pub dropsingletonnodeswithselfloops : DropSingletonNodesWithSelfloops,
    pub dropselfloops : DropSelfloops,
    pub dropparalleledges : DropParallelEdges,
    pub validatenodeid : ValidateNodeId,
    pub validatenodeids : ValidateNodeIds,
    pub validateedgeid : ValidateEdgeId,
    pub validateedgeids : ValidateEdgeIds,
    pub validatenodetypeid : ValidateNodeTypeId,
    pub validatenodetypeids : ValidateNodeTypeIds,
    pub validateedgetypeid : ValidateEdgeTypeId,
    pub validateedgetypeids : ValidateEdgeTypeIds,















    pub getuncheckedunweightedclosenesscentralityfromnodeid : GetUncheckedUnweightedClosenessCentralityFromNodeId,
    pub getuncheckedweightedclosenesscentralityfromnodeid : GetUncheckedWeightedClosenessCentralityFromNodeId,
    pub pariterunweightedclosenesscentrality : ParIterUnweightedClosenessCentrality,
    pub pariterweightedclosenesscentrality : ParIterWeightedClosenessCentrality,
    pub getunweightedclosenesscentrality : GetUnweightedClosenessCentrality,
    pub getweightedclosenesscentrality : GetWeightedClosenessCentrality,
    pub getuncheckedunweightedharmoniccentralityfromnodeid : GetUncheckedUnweightedHarmonicCentralityFromNodeId,
    pub getuncheckedweightedharmoniccentralityfromnodeid : GetUncheckedWeightedHarmonicCentralityFromNodeId,
    pub pariterunweightedharmoniccentrality : ParIterUnweightedHarmonicCentrality,
    pub pariterweightedharmoniccentrality : ParIterWeightedHarmonicCentrality,
    pub getunweightedharmoniccentrality : GetUnweightedHarmonicCentrality,
    pub getweightedharmoniccentrality : GetWeightedHarmonicCentrality,
    pub getstresscentrality : GetStressCentrality,
    pub getbetweennesscentrality : GetBetweennessCentrality,
    pub getunweightedeigenvectorcentrality : GetUnweightedEigenvectorCentrality,
    pub getweightedeigenvectorcentrality : GetWeightedEigenvectorCentrality,
    pub removeinplacenodetypeids : RemoveInplaceNodeTypeIds,

    pub removeinplaceedgetypeids : RemoveInplaceEdgeTypeIds,

    pub removenodetypeid : RemoveNodeTypeId,

    pub removeedgetypeid : RemoveEdgeTypeId,




    pub removeedgetypes : RemoveEdgeTypes,


    pub encodeedge : EncodeEdge,
    pub decodeedge : DecodeEdge,




    pub getnodereportfromnodeid : GetNodeReportFromNodeId,

    pub textualreport : TextualReport,
    pub getconnectedcomponentsnumber : GetConnectedComponentsNumber,
































    pub getsourcenodeids : GetSourceNodeIds,
    pub getsourcenames : GetSourceNames,
    pub getdestinationnodeids : GetDestinationNodeIds,
    pub getdestinationnames : GetDestinationNames,



















    pub getedgenodeids : GetEdgeNodeIds,
    pub getedgenodenames : GetEdgeNodeNames,





















    pub getnodeconnectedcomponentids : GetNodeConnectedComponentIds,















    pub getdenseweightedadjacencymatrix : GetDenseWeightedAdjacencyMatrix,



























    pub itersourcenodeids : IterSourceNodeIds,


    pub paritersourcenodeids : ParIterSourceNodeIds,
    pub iterdestinationnodeids : IterDestinationNodeIds,
    pub pariterdestinationnodeids : ParIterDestinationNodeIds,







    pub iteredgenodeids : IterEdgeNodeIds,
    pub iteredges : IterEdges,
    pub pariteredgenodeids : ParIterEdgeNodeIds,

    pub pariteredges : ParIterEdges,


    pub iteredgenodeidsandedgetypeid : IterEdgeNodeIdsAndEdgeTypeId,


    pub iteredgenodenamesandedgetypename : IterEdgeNodeNamesAndEdgeTypeName,
    pub pariteredgenodenamesandedgetypename : ParIterEdgeNodeNamesAndEdgeTypeName,
    pub pariteredgenodeidsandedgetypeid : ParIterEdgeNodeIdsAndEdgeTypeId,
    pub pariteredgenodenamesandedgetypenameandedgeweight : ParIterEdgeNodeNamesAndEdgeTypeNameAndEdgeWeight,
    pub iteredgenodenamesandedgetypenameandedgeweight : IterEdgeNodeNamesAndEdgeTypeNameAndEdgeWeight,
    pub pariteredgenodeidsandedgetypeidandedgeweight : ParIterEdgeNodeIdsAndEdgeTypeIdAndEdgeWeight,
    pub iteredgenodeidsandedgetypeidandedgeweight : IterEdgeNodeIdsAndEdgeTypeIdAndEdgeWeight,
    pub iteruniqueedgenodeids : IterUniqueEdgeNodeIds,



    pub iteredgenodeidswithunknownedgetypes : IterEdgeNodeIdsWithUnknownEdgeTypes,
    pub iteredgenodeidswithknownedgetypes : IterEdgeNodeIdsWithKnownEdgeTypes,




    pub pariteredgenodeidswithunknownedgetypes : ParIterEdgeNodeIdsWithUnknownEdgeTypes,
    pub pariteredgenodeidswithknownedgetypes : ParIterEdgeNodeIdsWithKnownEdgeTypes,


    pub getunweightedlaplaciantransformedgraph : GetUnweightedLaplacianTransformedGraph,
    pub getunweightedrandomwalknormalizedlaplaciantransformedgraph : GetUnweightedRandomWalkNormalizedLaplacianTransformedGraph,
    pub getunweightedsymmetricnormalizedlaplaciantransformedgraph : GetUnweightedSymmetricNormalizedLaplacianTransformedGraph,
    pub getunweightedsymmetricnormalizedtransformedgraph : GetUnweightedSymmetricNormalizedTransformedGraph,
    pub getweightedlaplaciantransformedgraph : GetWeightedLaplacianTransformedGraph,
    pub getweightedsymmetricnormalizedlaplaciantransformedgraph : GetWeightedSymmetricNormalizedLaplacianTransformedGraph,
    pub getweightedsymmetricnormalizedtransformedgraph : GetWeightedSymmetricNormalizedTransformedGraph,
    pub getweightedrandomwalknormalizedlaplaciantransformedgraph : GetWeightedRandomWalkNormalizedLaplacianTransformedGraph,













    pub isconnected : IsConnected,













    pub nodelabelholdout : NodeLabelHoldout,
    pub edgelabelholdout : EdgeLabelHoldout,
    pub randomsubgraph : RandomSubgraph,
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
        true,
        data.from_vec.verbose,
    )?;

    let mut rng = Rng::new(data.seed);
    let mut trace = Vec::new();
    for _ in 0..10 {
        let data_for_current_test = data_copy_for_tests.clone();
        let data_for_panic_handler = data_copy_for_tests.clone();
        match rng.next() % 396 {

    0 => {
        trace.push(format!("spanning_arborescence_kruskal({:?})", &data.spanningarborescencekruskal.verbose));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.spanning_arborescence_kruskal(data.spanningarborescencekruskal.verbose.clone());
    }
    

    1 => {
        trace.push(format!("spanning_arborescence({:?})", &data.spanningarborescence.verbose));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.spanning_arborescence(data.spanningarborescence.verbose.clone());
    }
    

    2 => {
        trace.push(format!("connected_components({:?})", &data.connectedcomponents.verbose));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.connected_components(data.connectedcomponents.verbose.clone());
    }
    

    3 => {
        trace.push(format!("get_node_label_prediction_tuple_from_node_ids({:?}, {:?}, {:?}, {:?}, {:?})", &data.getnodelabelpredictiontuplefromnodeids.node_ids, &data.getnodelabelpredictiontuplefromnodeids.random_state, &data.getnodelabelpredictiontuplefromnodeids.include_central_node, &data.getnodelabelpredictiontuplefromnodeids.offset, &data.getnodelabelpredictiontuplefromnodeids.max_neighbours));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_node_label_prediction_tuple_from_node_ids(data.getnodelabelpredictiontuplefromnodeids.node_ids.clone(), data.getnodelabelpredictiontuplefromnodeids.random_state.clone(), data.getnodelabelpredictiontuplefromnodeids.include_central_node.clone(), data.getnodelabelpredictiontuplefromnodeids.offset.clone(), data.getnodelabelpredictiontuplefromnodeids.max_neighbours.clone());
    }
    

    4 => {
        trace.push(format!("par_iter_unchecked_edge_prediction_metrics({:?}, {:?})", &data.pariteruncheckededgepredictionmetrics.source_node_ids, &data.pariteruncheckededgepredictionmetrics.destination_node_ids));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        unsafe{
            let _ = graph.par_iter_unchecked_edge_prediction_metrics(data.pariteruncheckededgepredictionmetrics.source_node_ids.clone(), data.pariteruncheckededgepredictionmetrics.destination_node_ids.clone()).collect::<Vec<_>>();
            }
    }
    

    5 => {
        trace.push(format!("get_okapi_bm25_node_feature_propagation({:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?})", &data.getokapibm25nodefeaturepropagation.features, &data.getokapibm25nodefeaturepropagation.iterations, &data.getokapibm25nodefeaturepropagation.maximal_distance, &data.getokapibm25nodefeaturepropagation.k1, &data.getokapibm25nodefeaturepropagation.b, &data.getokapibm25nodefeaturepropagation.include_central_node, &data.getokapibm25nodefeaturepropagation.verbose));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_okapi_bm25_node_feature_propagation(data.getokapibm25nodefeaturepropagation.features.clone(), data.getokapibm25nodefeaturepropagation.iterations.clone(), data.getokapibm25nodefeaturepropagation.maximal_distance.clone(), data.getokapibm25nodefeaturepropagation.k1.clone(), data.getokapibm25nodefeaturepropagation.b.clone(), data.getokapibm25nodefeaturepropagation.include_central_node.clone(), data.getokapibm25nodefeaturepropagation.verbose.clone());
    }
    

    6 => {
        trace.push(format!("get_okapi_bm25_node_label_propagation({:?}, {:?}, {:?}, {:?}, {:?})", &data.getokapibm25nodelabelpropagation.iterations, &data.getokapibm25nodelabelpropagation.maximal_distance, &data.getokapibm25nodelabelpropagation.k1, &data.getokapibm25nodelabelpropagation.b, &data.getokapibm25nodelabelpropagation.verbose));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_okapi_bm25_node_label_propagation(data.getokapibm25nodelabelpropagation.iterations.clone(), data.getokapibm25nodelabelpropagation.maximal_distance.clone(), data.getokapibm25nodelabelpropagation.k1.clone(), data.getokapibm25nodelabelpropagation.b.clone(), data.getokapibm25nodelabelpropagation.verbose.clone());
    }
    

    7 => {
        trace.push(format!("to_dot({:?})", &data.todot.use_node_names));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.to_dot(data.todot.use_node_names.clone());
    }
    

    8 => {
        trace.push(format!("is_unchecked_singleton_from_node_id({:?})", &data.isuncheckedsingletonfromnodeid.node_id));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        unsafe{let _ = graph.is_unchecked_singleton_from_node_id(data.isuncheckedsingletonfromnodeid.node_id.clone());}
    }
    

    9 => {
        trace.push(format!("is_unchecked_disconnected_from_node_id({:?})", &data.isuncheckeddisconnectedfromnodeid.node_id));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        unsafe{let _ = graph.is_unchecked_disconnected_from_node_id(data.isuncheckeddisconnectedfromnodeid.node_id.clone());}
    }
    

    10 => {
        trace.push(format!("is_unchecked_connected_from_node_id({:?})", &data.isuncheckedconnectedfromnodeid.node_id));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        unsafe{let _ = graph.is_unchecked_connected_from_node_id(data.isuncheckedconnectedfromnodeid.node_id.clone());}
    }
    

    11 => {
        trace.push(format!("is_singleton_from_node_id({:?})", &data.issingletonfromnodeid.node_id));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.is_singleton_from_node_id(data.issingletonfromnodeid.node_id.clone());
    }
    

    12 => {
        trace.push(format!("is_singleton_with_selfloops_from_node_id({:?})", &data.issingletonwithselfloopsfromnodeid.node_id));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.is_singleton_with_selfloops_from_node_id(data.issingletonwithselfloopsfromnodeid.node_id.clone());
    }
    

    13 => {
        trace.push(format!("has_node_type_id({:?})", &data.hasnodetypeid.node_type_id));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.has_node_type_id(data.hasnodetypeid.node_type_id.clone());
    }
    

    14 => {
        trace.push(format!("has_edge_type_id({:?})", &data.hasedgetypeid.edge_type_id));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.has_edge_type_id(data.hasedgetypeid.edge_type_id.clone());
    }
    

    15 => {
        trace.push(format!("has_edge_from_node_ids({:?}, {:?})", &data.hasedgefromnodeids.src, &data.hasedgefromnodeids.dst));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.has_edge_from_node_ids(data.hasedgefromnodeids.src.clone(), data.hasedgefromnodeids.dst.clone());
    }
    

    16 => {
        trace.push(format!("has_selfloop_from_node_id({:?})", &data.hasselfloopfromnodeid.node_id));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.has_selfloop_from_node_id(data.hasselfloopfromnodeid.node_id.clone());
    }
    

    17 => {
        trace.push(format!("has_edge_from_node_ids_and_edge_type_id({:?}, {:?}, {:?})", &data.hasedgefromnodeidsandedgetypeid.src, &data.hasedgefromnodeidsandedgetypeid.dst, &data.hasedgefromnodeidsandedgetypeid.edge_type));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.has_edge_from_node_ids_and_edge_type_id(data.hasedgefromnodeidsandedgetypeid.src.clone(), data.hasedgefromnodeidsandedgetypeid.dst.clone(), data.hasedgefromnodeidsandedgetypeid.edge_type.clone());
    }
    

    18 => {
        trace.push(format!("is_unchecked_trap_node_from_node_id({:?})", &data.isuncheckedtrapnodefromnodeid.node_id));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        unsafe{let _ = graph.is_unchecked_trap_node_from_node_id(data.isuncheckedtrapnodefromnodeid.node_id.clone());}
    }
    

    19 => {
        trace.push(format!("is_trap_node_from_node_id({:?})", &data.istrapnodefromnodeid.node_id));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.is_trap_node_from_node_id(data.istrapnodefromnodeid.node_id.clone());
    }
    

    20 => {
        trace.push(format!("get_unchecked_min_preferential_attachment()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        unsafe{let _ = graph.get_unchecked_min_preferential_attachment();}
    }
    

    21 => {
        trace.push(format!("get_unchecked_max_preferential_attachment()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        unsafe{let _ = graph.get_unchecked_max_preferential_attachment();}
    }
    

    22 => {
        trace.push(format!("get_unchecked_preferential_attachment({:?}, {:?}, {:?})", &data.getuncheckedpreferentialattachment.one, &data.getuncheckedpreferentialattachment.two, &data.getuncheckedpreferentialattachment.normalize));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        unsafe{let _ = graph.get_unchecked_preferential_attachment(data.getuncheckedpreferentialattachment.one.clone(), data.getuncheckedpreferentialattachment.two.clone(), data.getuncheckedpreferentialattachment.normalize.clone());}
    }
    

    23 => {
        trace.push(format!("get_preferential_attachment({:?}, {:?}, {:?})", &data.getpreferentialattachment.one, &data.getpreferentialattachment.two, &data.getpreferentialattachment.normalize));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_preferential_attachment(data.getpreferentialattachment.one.clone(), data.getpreferentialattachment.two.clone(), data.getpreferentialattachment.normalize.clone());
    }
    

    24 => {
        trace.push(format!("get_unchecked_jaccard_coefficient({:?}, {:?})", &data.getuncheckedjaccardcoefficient.one, &data.getuncheckedjaccardcoefficient.two));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        unsafe{let _ = graph.get_unchecked_jaccard_coefficient(data.getuncheckedjaccardcoefficient.one.clone(), data.getuncheckedjaccardcoefficient.two.clone());}
    }
    

    25 => {
        trace.push(format!("get_jaccard_coefficient({:?}, {:?})", &data.getjaccardcoefficient.one, &data.getjaccardcoefficient.two));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_jaccard_coefficient(data.getjaccardcoefficient.one.clone(), data.getjaccardcoefficient.two.clone());
    }
    

    26 => {
        trace.push(format!("get_unchecked_adamic_adar_index({:?}, {:?})", &data.getuncheckedadamicadarindex.one, &data.getuncheckedadamicadarindex.two));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        unsafe{let _ = graph.get_unchecked_adamic_adar_index(data.getuncheckedadamicadarindex.one.clone(), data.getuncheckedadamicadarindex.two.clone());}
    }
    

    27 => {
        trace.push(format!("get_adamic_adar_index({:?}, {:?})", &data.getadamicadarindex.one, &data.getadamicadarindex.two));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_adamic_adar_index(data.getadamicadarindex.one.clone(), data.getadamicadarindex.two.clone());
    }
    

    28 => {
        trace.push(format!("get_unchecked_resource_allocation_index({:?}, {:?})", &data.getuncheckedresourceallocationindex.one, &data.getuncheckedresourceallocationindex.two));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        unsafe{let _ = graph.get_unchecked_resource_allocation_index(data.getuncheckedresourceallocationindex.one.clone(), data.getuncheckedresourceallocationindex.two.clone());}
    }
    

    29 => {
        trace.push(format!("get_resource_allocation_index({:?}, {:?})", &data.getresourceallocationindex.one, &data.getresourceallocationindex.two));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_resource_allocation_index(data.getresourceallocationindex.one.clone(), data.getresourceallocationindex.two.clone());
    }
    

    30 => {
        trace.push(format!("enable({:?}, {:?}, {:?}, {:?})", &data.enable.vector_sources, &data.enable.vector_destinations, &data.enable.vector_cumulative_node_degrees, &data.enable.cache_size));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.enable(data.enable.vector_sources.clone(), data.enable.vector_destinations.clone(), data.enable.vector_cumulative_node_degrees.clone(), data.enable.cache_size.clone());
    }
    

    31 => {
        trace.push(format!("disable_all()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        graph.disable_all()
    }
    

    32 => {
        trace.push(format!("get_unweighted_number_of_triangles({:?})", &data.getunweightednumberoftriangles.normalize));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_unweighted_number_of_triangles(data.getunweightednumberoftriangles.normalize.clone());
    }
    

    33 => {
        trace.push(format!("get_unweighted_triads_number()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_unweighted_triads_number();
    }
    

    34 => {
        trace.push(format!("get_weighted_triads_number()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_weighted_triads_number();
    }
    

    35 => {
        trace.push(format!("get_unweighted_transitivity()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_unweighted_transitivity();
    }
    

    36 => {
        trace.push(format!("get_unweighted_number_of_triangles_per_node({:?})", &data.getunweightednumberoftrianglespernode.normalize));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_unweighted_number_of_triangles_per_node(data.getunweightednumberoftrianglespernode.normalize.clone());
    }
    

    37 => {
        trace.push(format!("iter_clustering_coefficient_per_node()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.iter_clustering_coefficient_per_node().collect::<Vec<_>>();
            
    }
    

    38 => {
        trace.push(format!("get_clustering_coefficient_per_node()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_clustering_coefficient_per_node();
    }
    

    39 => {
        trace.push(format!("get_clustering_coefficient()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_clustering_coefficient();
    }
    

    40 => {
        trace.push(format!("get_average_clustering_coefficient()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_average_clustering_coefficient();
    }
    

    41 => {
        trace.push(format!("get_unchecked_breath_first_search_from_node_ids({:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?})", &data.getuncheckedbreathfirstsearchfromnodeids.src_node_id, &data.getuncheckedbreathfirstsearchfromnodeids.maybe_dst_node_id, &data.getuncheckedbreathfirstsearchfromnodeids.maybe_dst_node_ids, &data.getuncheckedbreathfirstsearchfromnodeids.compute_distances, &data.getuncheckedbreathfirstsearchfromnodeids.compute_predecessors, &data.getuncheckedbreathfirstsearchfromnodeids.compute_visited, &data.getuncheckedbreathfirstsearchfromnodeids.maximal_depth));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        unsafe{let _ = graph.get_unchecked_breath_first_search_from_node_ids(data.getuncheckedbreathfirstsearchfromnodeids.src_node_id.clone(), data.getuncheckedbreathfirstsearchfromnodeids.maybe_dst_node_id.clone(), data.getuncheckedbreathfirstsearchfromnodeids.maybe_dst_node_ids.clone(), data.getuncheckedbreathfirstsearchfromnodeids.compute_distances.clone(), data.getuncheckedbreathfirstsearchfromnodeids.compute_predecessors.clone(), data.getuncheckedbreathfirstsearchfromnodeids.compute_visited.clone(), data.getuncheckedbreathfirstsearchfromnodeids.maximal_depth.clone());}
    }
    

    42 => {
        trace.push(format!("get_unchecked_unweighted_minimum_path_node_ids_from_node_ids({:?}, {:?}, {:?})", &data.getuncheckedunweightedminimumpathnodeidsfromnodeids.src_node_id, &data.getuncheckedunweightedminimumpathnodeidsfromnodeids.dst_node_id, &data.getuncheckedunweightedminimumpathnodeidsfromnodeids.maximal_depth));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        unsafe{let _ = graph.get_unchecked_unweighted_minimum_path_node_ids_from_node_ids(data.getuncheckedunweightedminimumpathnodeidsfromnodeids.src_node_id.clone(), data.getuncheckedunweightedminimumpathnodeidsfromnodeids.dst_node_id.clone(), data.getuncheckedunweightedminimumpathnodeidsfromnodeids.maximal_depth.clone());}
    }
    

    43 => {
        trace.push(format!("get_unchecked_unweighted_minimum_path_node_names_from_node_ids({:?}, {:?}, {:?})", &data.getuncheckedunweightedminimumpathnodenamesfromnodeids.src_node_id, &data.getuncheckedunweightedminimumpathnodenamesfromnodeids.dst_node_id, &data.getuncheckedunweightedminimumpathnodenamesfromnodeids.maximal_depth));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        unsafe{let _ = graph.get_unchecked_unweighted_minimum_path_node_names_from_node_ids(data.getuncheckedunweightedminimumpathnodenamesfromnodeids.src_node_id.clone(), data.getuncheckedunweightedminimumpathnodenamesfromnodeids.dst_node_id.clone(), data.getuncheckedunweightedminimumpathnodenamesfromnodeids.maximal_depth.clone());}
    }
    

    44 => {
        trace.push(format!("get_unweighted_minimum_path_node_ids_from_node_ids({:?}, {:?}, {:?})", &data.getunweightedminimumpathnodeidsfromnodeids.src_node_id, &data.getunweightedminimumpathnodeidsfromnodeids.dst_node_id, &data.getunweightedminimumpathnodeidsfromnodeids.maximal_depth));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_unweighted_minimum_path_node_ids_from_node_ids(data.getunweightedminimumpathnodeidsfromnodeids.src_node_id.clone(), data.getunweightedminimumpathnodeidsfromnodeids.dst_node_id.clone(), data.getunweightedminimumpathnodeidsfromnodeids.maximal_depth.clone());
    }
    

    45 => {
        trace.push(format!("get_unchecked_unweighted_k_shortest_path_node_ids_from_node_ids({:?}, {:?}, {:?})", &data.getuncheckedunweightedkshortestpathnodeidsfromnodeids.src_node_id, &data.getuncheckedunweightedkshortestpathnodeidsfromnodeids.dst_node_id, &data.getuncheckedunweightedkshortestpathnodeidsfromnodeids.k));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        unsafe{let _ = graph.get_unchecked_unweighted_k_shortest_path_node_ids_from_node_ids(data.getuncheckedunweightedkshortestpathnodeidsfromnodeids.src_node_id.clone(), data.getuncheckedunweightedkshortestpathnodeidsfromnodeids.dst_node_id.clone(), data.getuncheckedunweightedkshortestpathnodeidsfromnodeids.k.clone());}
    }
    

    46 => {
        trace.push(format!("get_unweighted_k_shortest_path_node_ids_from_node_ids({:?}, {:?}, {:?})", &data.getunweightedkshortestpathnodeidsfromnodeids.src_node_id, &data.getunweightedkshortestpathnodeidsfromnodeids.dst_node_id, &data.getunweightedkshortestpathnodeidsfromnodeids.k));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_unweighted_k_shortest_path_node_ids_from_node_ids(data.getunweightedkshortestpathnodeidsfromnodeids.src_node_id.clone(), data.getunweightedkshortestpathnodeidsfromnodeids.dst_node_id.clone(), data.getunweightedkshortestpathnodeidsfromnodeids.k.clone());
    }
    

    47 => {
        trace.push(format!("get_unchecked_unweighted_eccentricity_from_node_id({:?})", &data.getuncheckedunweightedeccentricityfromnodeid.node_id));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        unsafe{let _ = graph.get_unchecked_unweighted_eccentricity_from_node_id(data.getuncheckedunweightedeccentricityfromnodeid.node_id.clone());}
    }
    

    48 => {
        trace.push(format!("get_unchecked_weighted_eccentricity_from_node_id({:?}, {:?})", &data.getuncheckedweightedeccentricityfromnodeid.node_id, &data.getuncheckedweightedeccentricityfromnodeid.use_edge_weights_as_probabilities));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        unsafe{let _ = graph.get_unchecked_weighted_eccentricity_from_node_id(data.getuncheckedweightedeccentricityfromnodeid.node_id.clone(), data.getuncheckedweightedeccentricityfromnodeid.use_edge_weights_as_probabilities.clone());}
    }
    

    49 => {
        trace.push(format!("get_unweighted_eccentricity_from_node_id({:?})", &data.getunweightedeccentricityfromnodeid.node_id));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_unweighted_eccentricity_from_node_id(data.getunweightedeccentricityfromnodeid.node_id.clone());
    }
    

    50 => {
        trace.push(format!("get_weighted_eccentricity_from_node_id({:?}, {:?})", &data.getweightedeccentricityfromnodeid.node_id, &data.getweightedeccentricityfromnodeid.use_edge_weights_as_probabilities));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_weighted_eccentricity_from_node_id(data.getweightedeccentricityfromnodeid.node_id.clone(), data.getweightedeccentricityfromnodeid.use_edge_weights_as_probabilities.clone());
    }
    

    51 => {
        trace.push(format!("get_unchecked_dijkstra_from_node_ids({:?}, {:?}, {:?}, {:?}, {:?}, {:?})", &data.getuncheckeddijkstrafromnodeids.src_node_id, &data.getuncheckeddijkstrafromnodeids.maybe_dst_node_id, &data.getuncheckeddijkstrafromnodeids.maybe_dst_node_ids, &data.getuncheckeddijkstrafromnodeids.compute_predecessors, &data.getuncheckeddijkstrafromnodeids.maximal_depth, &data.getuncheckeddijkstrafromnodeids.use_edge_weights_as_probabilities));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        unsafe{let _ = graph.get_unchecked_dijkstra_from_node_ids(data.getuncheckeddijkstrafromnodeids.src_node_id.clone(), data.getuncheckeddijkstrafromnodeids.maybe_dst_node_id.clone(), data.getuncheckeddijkstrafromnodeids.maybe_dst_node_ids.clone(), data.getuncheckeddijkstrafromnodeids.compute_predecessors.clone(), data.getuncheckeddijkstrafromnodeids.maximal_depth.clone(), data.getuncheckeddijkstrafromnodeids.use_edge_weights_as_probabilities.clone());}
    }
    

    52 => {
        trace.push(format!("get_unchecked_weighted_minimum_path_node_ids_from_node_ids({:?}, {:?}, {:?}, {:?})", &data.getuncheckedweightedminimumpathnodeidsfromnodeids.src_node_id, &data.getuncheckedweightedminimumpathnodeidsfromnodeids.dst_node_id, &data.getuncheckedweightedminimumpathnodeidsfromnodeids.use_edge_weights_as_probabilities, &data.getuncheckedweightedminimumpathnodeidsfromnodeids.maximal_depth));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        unsafe{let _ = graph.get_unchecked_weighted_minimum_path_node_ids_from_node_ids(data.getuncheckedweightedminimumpathnodeidsfromnodeids.src_node_id.clone(), data.getuncheckedweightedminimumpathnodeidsfromnodeids.dst_node_id.clone(), data.getuncheckedweightedminimumpathnodeidsfromnodeids.use_edge_weights_as_probabilities.clone(), data.getuncheckedweightedminimumpathnodeidsfromnodeids.maximal_depth.clone());}
    }
    

    53 => {
        trace.push(format!("get_unchecked_weighted_minimum_path_node_names_from_node_ids({:?}, {:?}, {:?}, {:?})", &data.getuncheckedweightedminimumpathnodenamesfromnodeids.src_node_id, &data.getuncheckedweightedminimumpathnodenamesfromnodeids.dst_node_id, &data.getuncheckedweightedminimumpathnodenamesfromnodeids.use_edge_weights_as_probabilities, &data.getuncheckedweightedminimumpathnodenamesfromnodeids.maximal_depth));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        unsafe{let _ = graph.get_unchecked_weighted_minimum_path_node_names_from_node_ids(data.getuncheckedweightedminimumpathnodenamesfromnodeids.src_node_id.clone(), data.getuncheckedweightedminimumpathnodenamesfromnodeids.dst_node_id.clone(), data.getuncheckedweightedminimumpathnodenamesfromnodeids.use_edge_weights_as_probabilities.clone(), data.getuncheckedweightedminimumpathnodenamesfromnodeids.maximal_depth.clone());}
    }
    

    54 => {
        trace.push(format!("get_weighted_minimum_path_node_ids_from_node_ids({:?}, {:?}, {:?}, {:?})", &data.getweightedminimumpathnodeidsfromnodeids.src_node_id, &data.getweightedminimumpathnodeidsfromnodeids.dst_node_id, &data.getweightedminimumpathnodeidsfromnodeids.use_edge_weights_as_probabilities, &data.getweightedminimumpathnodeidsfromnodeids.maximal_depth));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_weighted_minimum_path_node_ids_from_node_ids(data.getweightedminimumpathnodeidsfromnodeids.src_node_id.clone(), data.getweightedminimumpathnodeidsfromnodeids.dst_node_id.clone(), data.getweightedminimumpathnodeidsfromnodeids.use_edge_weights_as_probabilities.clone(), data.getweightedminimumpathnodeidsfromnodeids.maximal_depth.clone());
    }
    

    55 => {
        trace.push(format!("get_breath_first_search_from_node_ids({:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?})", &data.getbreathfirstsearchfromnodeids.src_node_id, &data.getbreathfirstsearchfromnodeids.maybe_dst_node_id, &data.getbreathfirstsearchfromnodeids.maybe_dst_node_ids, &data.getbreathfirstsearchfromnodeids.compute_distances, &data.getbreathfirstsearchfromnodeids.compute_predecessors, &data.getbreathfirstsearchfromnodeids.compute_visited, &data.getbreathfirstsearchfromnodeids.maximal_depth));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_breath_first_search_from_node_ids(data.getbreathfirstsearchfromnodeids.src_node_id.clone(), data.getbreathfirstsearchfromnodeids.maybe_dst_node_id.clone(), data.getbreathfirstsearchfromnodeids.maybe_dst_node_ids.clone(), data.getbreathfirstsearchfromnodeids.compute_distances.clone(), data.getbreathfirstsearchfromnodeids.compute_predecessors.clone(), data.getbreathfirstsearchfromnodeids.compute_visited.clone(), data.getbreathfirstsearchfromnodeids.maximal_depth.clone());
    }
    

    56 => {
        trace.push(format!("get_dijkstra_from_node_ids({:?}, {:?}, {:?}, {:?}, {:?}, {:?})", &data.getdijkstrafromnodeids.src_node_id, &data.getdijkstrafromnodeids.maybe_dst_node_id, &data.getdijkstrafromnodeids.maybe_dst_node_ids, &data.getdijkstrafromnodeids.compute_predecessors, &data.getdijkstrafromnodeids.maximal_depth, &data.getdijkstrafromnodeids.use_edge_weights_as_probabilities));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_dijkstra_from_node_ids(data.getdijkstrafromnodeids.src_node_id.clone(), data.getdijkstrafromnodeids.maybe_dst_node_id.clone(), data.getdijkstrafromnodeids.maybe_dst_node_ids.clone(), data.getdijkstrafromnodeids.compute_predecessors.clone(), data.getdijkstrafromnodeids.maximal_depth.clone(), data.getdijkstrafromnodeids.use_edge_weights_as_probabilities.clone());
    }
    

    57 => {
        trace.push(format!("get_unweighted_diameter({:?}, {:?})", &data.getunweighteddiameter.ignore_infinity, &data.getunweighteddiameter.verbose));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_unweighted_diameter(data.getunweighteddiameter.ignore_infinity.clone(), data.getunweighteddiameter.verbose.clone());
    }
    

    58 => {
        trace.push(format!("get_weighted_diameter({:?}, {:?}, {:?})", &data.getweighteddiameter.ignore_infinity, &data.getweighteddiameter.use_edge_weights_as_probabilities, &data.getweighteddiameter.verbose));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_weighted_diameter(data.getweighteddiameter.ignore_infinity.clone(), data.getweighteddiameter.use_edge_weights_as_probabilities.clone(), data.getweighteddiameter.verbose.clone());
    }
    

    59 => {
        trace.push(format!("iter_unchecked_edge_ids_from_source_node_id({:?})", &data.iteruncheckededgeidsfromsourcenodeid.src));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        unsafe{let _ = graph.iter_unchecked_edge_ids_from_source_node_id(data.iteruncheckededgeidsfromsourcenodeid.src.clone());}
    }
    

    60 => {
        trace.push(format!("iter_unchecked_edge_weights_from_source_node_id({:?})", &data.iteruncheckededgeweightsfromsourcenodeid.source_node_id));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        unsafe{
            let _ = graph.iter_unchecked_edge_weights_from_source_node_id(data.iteruncheckededgeweightsfromsourcenodeid.source_node_id.clone()).collect::<Vec<_>>();
            }
    }
    

    61 => {
        trace.push(format!("par_iter_unchecked_edge_ids_from_source_node_id({:?})", &data.pariteruncheckededgeidsfromsourcenodeid.src));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        unsafe{
            let _ = graph.par_iter_unchecked_edge_ids_from_source_node_id(data.pariteruncheckededgeidsfromsourcenodeid.src.clone()).collect::<Vec<_>>();
            }
    }
    

    62 => {
        trace.push(format!("iter_unchecked_edge_ids_from_node_ids({:?}, {:?})", &data.iteruncheckededgeidsfromnodeids.src, &data.iteruncheckededgeidsfromnodeids.dst));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        unsafe{
            let _ = graph.iter_unchecked_edge_ids_from_node_ids(data.iteruncheckededgeidsfromnodeids.src.clone(), data.iteruncheckededgeidsfromnodeids.dst.clone()).collect::<Vec<_>>();
            }
    }
    

    63 => {
        trace.push(format!("iter_unchecked_neighbour_node_ids_from_source_node_id({:?})", &data.iteruncheckedneighbournodeidsfromsourcenodeid.src));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        unsafe{
            let _ = graph.iter_unchecked_neighbour_node_ids_from_source_node_id(data.iteruncheckedneighbournodeidsfromsourcenodeid.src.clone()).collect::<Vec<_>>();
            }
    }
    

    64 => {
        trace.push(format!("iter_unchecked_neighbour_node_ids_intersection_from_source_node_ids({:?}, {:?})", &data.iteruncheckedneighbournodeidsintersectionfromsourcenodeids.first_src_node_id, &data.iteruncheckedneighbournodeidsintersectionfromsourcenodeids.second_src_node_id));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        unsafe{
            let _ = graph.iter_unchecked_neighbour_node_ids_intersection_from_source_node_ids(data.iteruncheckedneighbournodeidsintersectionfromsourcenodeids.first_src_node_id.clone(), data.iteruncheckedneighbournodeidsintersectionfromsourcenodeids.second_src_node_id.clone()).collect::<Vec<_>>();
            }
    }
    

    65 => {
        trace.push(format!("iter_unchecked_neighbour_node_ids_union_from_source_node_ids({:?}, {:?})", &data.iteruncheckedneighbournodeidsunionfromsourcenodeids.first_src_node_id, &data.iteruncheckedneighbournodeidsunionfromsourcenodeids.second_src_node_id));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        unsafe{
            let _ = graph.iter_unchecked_neighbour_node_ids_union_from_source_node_ids(data.iteruncheckedneighbournodeidsunionfromsourcenodeids.first_src_node_id.clone(), data.iteruncheckedneighbournodeidsunionfromsourcenodeids.second_src_node_id.clone()).collect::<Vec<_>>();
            }
    }
    

    66 => {
        trace.push(format!("iter_unchecked_neighbour_node_ids_difference_from_source_node_ids({:?}, {:?})", &data.iteruncheckedneighbournodeidsdifferencefromsourcenodeids.first_src_node_id, &data.iteruncheckedneighbournodeidsdifferencefromsourcenodeids.second_src_node_id));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        unsafe{
            let _ = graph.iter_unchecked_neighbour_node_ids_difference_from_source_node_ids(data.iteruncheckedneighbournodeidsdifferencefromsourcenodeids.first_src_node_id.clone(), data.iteruncheckedneighbournodeidsdifferencefromsourcenodeids.second_src_node_id.clone()).collect::<Vec<_>>();
            }
    }
    

    67 => {
        trace.push(format!("iter_unchecked_neighbour_node_names_from_source_node_id({:?})", &data.iteruncheckedneighbournodenamesfromsourcenodeid.src));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        unsafe{
            let _ = graph.iter_unchecked_neighbour_node_names_from_source_node_id(data.iteruncheckedneighbournodenamesfromsourcenodeid.src.clone()).collect::<Vec<_>>();
            }
    }
    

    68 => {
        trace.push(format!("iter_edge_ids_from_node_ids({:?}, {:?})", &data.iteredgeidsfromnodeids.src, &data.iteredgeidsfromnodeids.dst));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.iter_edge_ids_from_node_ids(data.iteredgeidsfromnodeids.src.clone(), data.iteredgeidsfromnodeids.dst.clone()).map(|x| x.collect::<Vec<_>>());
            
    }
    

    69 => {
        trace.push(format!("iter_edge_node_ids_and_edge_type_id_from_edge_type_id({:?}, {:?})", &data.iteredgenodeidsandedgetypeidfromedgetypeid.edge_type_id, &data.iteredgenodeidsandedgetypeidfromedgetypeid.directed));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.iter_edge_node_ids_and_edge_type_id_from_edge_type_id(data.iteredgenodeidsandedgetypeidfromedgetypeid.edge_type_id.clone(), data.iteredgenodeidsandedgetypeidfromedgetypeid.directed.clone()).map(|x| x.collect::<Vec<_>>());
            
    }
    

    70 => {
        trace.push(format!("iter_node_ids_and_node_type_ids_from_node_type_id({:?})", &data.iternodeidsandnodetypeidsfromnodetypeid.node_type_id));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.iter_node_ids_and_node_type_ids_from_node_type_id(data.iternodeidsandnodetypeidsfromnodetypeid.node_type_id.clone()).map(|x| x.collect::<Vec<_>>());
            
    }
    

    71 => {
        trace.push(format!("iter_node_names_and_node_type_names_from_node_type_id({:?})", &data.iternodenamesandnodetypenamesfromnodetypeid.node_type_id));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.iter_node_names_and_node_type_names_from_node_type_id(data.iternodenamesandnodetypenamesfromnodetypeid.node_type_id.clone());
    }
    

    72 => {
        trace.push(format!("iter_edge_node_names_and_edge_type_name_from_edge_type_id({:?}, {:?})", &data.iteredgenodenamesandedgetypenamefromedgetypeid.edge_type_id, &data.iteredgenodenamesandedgetypenamefromedgetypeid.directed));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.iter_edge_node_names_and_edge_type_name_from_edge_type_id(data.iteredgenodenamesandedgetypenamefromedgetypeid.edge_type_id.clone(), data.iteredgenodenamesandedgetypenamefromedgetypeid.directed.clone());
    }
    

    73 => {
        trace.push(format!("get_transitive_closure({:?}, {:?})", &data.gettransitiveclosure.iterations, &data.gettransitiveclosure.verbose));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        graph = graph.get_transitive_closure(data.gettransitiveclosure.iterations.clone(), data.gettransitiveclosure.verbose.clone());
    }
    

    74 => {
        trace.push(format!("get_unweighted_all_shortest_paths({:?}, {:?})", &data.getunweightedallshortestpaths.iterations, &data.getunweightedallshortestpaths.verbose));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        graph = graph.get_unweighted_all_shortest_paths(data.getunweightedallshortestpaths.iterations.clone(), data.getunweightedallshortestpaths.verbose.clone());
    }
    

    75 => {
        trace.push(format!("get_weighted_all_shortest_paths({:?}, {:?}, {:?})", &data.getweightedallshortestpaths.iterations, &data.getweightedallshortestpaths.use_edge_weights_as_probabilities, &data.getweightedallshortestpaths.verbose));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
        if let Ok(res) = graph.get_weighted_all_shortest_paths(data.getweightedallshortestpaths.iterations.clone(), data.getweightedallshortestpaths.use_edge_weights_as_probabilities.clone(), data.getweightedallshortestpaths.verbose.clone()) {
            graph = res;
        }
        
    }
    

    76 => {
        trace.push(format!("strongly_connected_components()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.strongly_connected_components();
    }
    

    77 => {
        trace.push(format!("get_unchecked_edge_weight_from_edge_id({:?})", &data.getuncheckededgeweightfromedgeid.edge_id));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        unsafe{let _ = graph.get_unchecked_edge_weight_from_edge_id(data.getuncheckededgeweightfromedgeid.edge_id.clone());}
    }
    

    78 => {
        trace.push(format!("get_unchecked_edge_weight_from_node_ids({:?}, {:?})", &data.getuncheckededgeweightfromnodeids.src, &data.getuncheckededgeweightfromnodeids.dst));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        unsafe{let _ = graph.get_unchecked_edge_weight_from_node_ids(data.getuncheckededgeweightfromnodeids.src.clone(), data.getuncheckededgeweightfromnodeids.dst.clone());}
    }
    

    79 => {
        trace.push(format!("get_unchecked_edge_type_name_from_edge_type_id({:?})", &data.getuncheckededgetypenamefromedgetypeid.edge_type_id));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        unsafe{let _ = graph.get_unchecked_edge_type_name_from_edge_type_id(data.getuncheckededgetypenamefromedgetypeid.edge_type_id.clone());}
    }
    

    80 => {
        trace.push(format!("get_unchecked_edge_count_from_edge_type_id({:?})", &data.getuncheckededgecountfromedgetypeid.edge_type));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        unsafe{let _ = graph.get_unchecked_edge_count_from_edge_type_id(data.getuncheckededgecountfromedgetypeid.edge_type.clone());}
    }
    

    81 => {
        trace.push(format!("get_unchecked_edge_id_from_node_ids_and_edge_type_id({:?}, {:?}, {:?})", &data.getuncheckededgeidfromnodeidsandedgetypeid.src, &data.getuncheckededgeidfromnodeidsandedgetypeid.dst, &data.getuncheckededgeidfromnodeidsandedgetypeid.edge_type));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        unsafe{let _ = graph.get_unchecked_edge_id_from_node_ids_and_edge_type_id(data.getuncheckededgeidfromnodeidsandedgetypeid.src.clone(), data.getuncheckededgeidfromnodeidsandedgetypeid.dst.clone(), data.getuncheckededgeidfromnodeidsandedgetypeid.edge_type.clone());}
    }
    

    82 => {
        trace.push(format!("get_unchecked_minmax_edge_ids_from_node_ids({:?}, {:?})", &data.getuncheckedminmaxedgeidsfromnodeids.src, &data.getuncheckedminmaxedgeidsfromnodeids.dst));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        unsafe{let _ = graph.get_unchecked_minmax_edge_ids_from_node_ids(data.getuncheckedminmaxedgeidsfromnodeids.src.clone(), data.getuncheckedminmaxedgeidsfromnodeids.dst.clone());}
    }
    

    83 => {
        trace.push(format!("get_unchecked_node_ids_from_edge_id({:?})", &data.getuncheckednodeidsfromedgeid.edge_id));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        unsafe{let _ = graph.get_unchecked_node_ids_from_edge_id(data.getuncheckednodeidsfromedgeid.edge_id.clone());}
    }
    

    84 => {
        trace.push(format!("get_node_ids_from_edge_id({:?})", &data.getnodeidsfromedgeid.edge_id));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_node_ids_from_edge_id(data.getnodeidsfromedgeid.edge_id.clone());
    }
    

    85 => {
        trace.push(format!("get_unchecked_edge_id_from_node_ids({:?}, {:?})", &data.getuncheckededgeidfromnodeids.src, &data.getuncheckededgeidfromnodeids.dst));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        unsafe{let _ = graph.get_unchecked_edge_id_from_node_ids(data.getuncheckededgeidfromnodeids.src.clone(), data.getuncheckededgeidfromnodeids.dst.clone());}
    }
    

    86 => {
        trace.push(format!("get_edge_id_from_node_ids({:?}, {:?})", &data.getedgeidfromnodeids.src, &data.getedgeidfromnodeids.dst));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_edge_id_from_node_ids(data.getedgeidfromnodeids.src.clone(), data.getedgeidfromnodeids.dst.clone());
    }
    

    87 => {
        trace.push(format!("get_unchecked_unique_source_node_id({:?})", &data.getuncheckeduniquesourcenodeid.source_id));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        unsafe{let _ = graph.get_unchecked_unique_source_node_id(data.getuncheckeduniquesourcenodeid.source_id.clone());}
    }
    

    88 => {
        trace.push(format!("get_unchecked_node_ids_and_edge_type_id_from_edge_id({:?})", &data.getuncheckednodeidsandedgetypeidfromedgeid.edge_id));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        unsafe{let _ = graph.get_unchecked_node_ids_and_edge_type_id_from_edge_id(data.getuncheckednodeidsandedgetypeidfromedgeid.edge_id.clone());}
    }
    

    89 => {
        trace.push(format!("get_node_ids_and_edge_type_id_from_edge_id({:?})", &data.getnodeidsandedgetypeidfromedgeid.edge_id));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_node_ids_and_edge_type_id_from_edge_id(data.getnodeidsandedgetypeidfromedgeid.edge_id.clone());
    }
    

    90 => {
        trace.push(format!("get_unchecked_node_ids_and_edge_type_id_and_edge_weight_from_edge_id({:?})", &data.getuncheckednodeidsandedgetypeidandedgeweightfromedgeid.edge_id));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        unsafe{let _ = graph.get_unchecked_node_ids_and_edge_type_id_and_edge_weight_from_edge_id(data.getuncheckednodeidsandedgetypeidandedgeweightfromedgeid.edge_id.clone());}
    }
    

    91 => {
        trace.push(format!("get_node_ids_and_edge_type_id_and_edge_weight_from_edge_id({:?})", &data.getnodeidsandedgetypeidandedgeweightfromedgeid.edge_id));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_node_ids_and_edge_type_id_and_edge_weight_from_edge_id(data.getnodeidsandedgetypeidandedgeweightfromedgeid.edge_id.clone());
    }
    

    92 => {
        trace.push(format!("get_top_k_central_node_ids({:?})", &data.gettopkcentralnodeids.k));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_top_k_central_node_ids(data.gettopkcentralnodeids.k.clone());
    }
    

    93 => {
        trace.push(format!("get_unchecked_unweighted_node_degree_from_node_id({:?})", &data.getuncheckedunweightednodedegreefromnodeid.node_id));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        unsafe{let _ = graph.get_unchecked_unweighted_node_degree_from_node_id(data.getuncheckedunweightednodedegreefromnodeid.node_id.clone());}
    }
    

    94 => {
        trace.push(format!("get_unchecked_weighted_node_degree_from_node_id({:?})", &data.getuncheckedweightednodedegreefromnodeid.node_id));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        unsafe{let _ = graph.get_unchecked_weighted_node_degree_from_node_id(data.getuncheckedweightednodedegreefromnodeid.node_id.clone());}
    }
    

    95 => {
        trace.push(format!("get_unweighted_node_degree_from_node_id({:?})", &data.getunweightednodedegreefromnodeid.node_id));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_unweighted_node_degree_from_node_id(data.getunweightednodedegreefromnodeid.node_id.clone());
    }
    

    96 => {
        trace.push(format!("get_weighted_node_degree_from_node_id({:?})", &data.getweightednodedegreefromnodeid.node_id));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_weighted_node_degree_from_node_id(data.getweightednodedegreefromnodeid.node_id.clone());
    }
    

    97 => {
        trace.push(format!("get_top_k_central_node_names({:?})", &data.gettopkcentralnodenames.k));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_top_k_central_node_names(data.gettopkcentralnodenames.k.clone());
    }
    

    98 => {
        trace.push(format!("get_unchecked_node_type_id_from_node_id({:?})", &data.getuncheckednodetypeidfromnodeid.node_id));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        unsafe{let _ = graph.get_unchecked_node_type_id_from_node_id(data.getuncheckednodetypeidfromnodeid.node_id.clone());}
    }
    

    99 => {
        trace.push(format!("get_node_type_id_from_node_id({:?})", &data.getnodetypeidfromnodeid.node_id));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_node_type_id_from_node_id(data.getnodetypeidfromnodeid.node_id.clone());
    }
    

    100 => {
        trace.push(format!("get_unchecked_edge_type_id_from_edge_id({:?})", &data.getuncheckededgetypeidfromedgeid.edge_id));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        unsafe{let _ = graph.get_unchecked_edge_type_id_from_edge_id(data.getuncheckededgetypeidfromedgeid.edge_id.clone());}
    }
    

    101 => {
        trace.push(format!("get_edge_type_id_from_edge_id({:?})", &data.getedgetypeidfromedgeid.edge_id));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_edge_type_id_from_edge_id(data.getedgetypeidfromedgeid.edge_id.clone());
    }
    

    102 => {
        trace.push(format!("get_unchecked_node_type_names_from_node_id({:?})", &data.getuncheckednodetypenamesfromnodeid.node_id));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        unsafe{let _ = graph.get_unchecked_node_type_names_from_node_id(data.getuncheckednodetypenamesfromnodeid.node_id.clone());}
    }
    

    103 => {
        trace.push(format!("get_node_type_names_from_node_id({:?})", &data.getnodetypenamesfromnodeid.node_id));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_node_type_names_from_node_id(data.getnodetypenamesfromnodeid.node_id.clone());
    }
    

    104 => {
        trace.push(format!("get_edge_type_name_from_edge_id({:?})", &data.getedgetypenamefromedgeid.edge_id));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_edge_type_name_from_edge_id(data.getedgetypenamefromedgeid.edge_id.clone());
    }
    

    105 => {
        trace.push(format!("get_edge_type_name_from_edge_type_id({:?})", &data.getedgetypenamefromedgetypeid.edge_type_id));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_edge_type_name_from_edge_type_id(data.getedgetypenamefromedgetypeid.edge_type_id.clone());
    }
    

    106 => {
        trace.push(format!("get_edge_weight_from_edge_id({:?})", &data.getedgeweightfromedgeid.edge_id));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_edge_weight_from_edge_id(data.getedgeweightfromedgeid.edge_id.clone());
    }
    

    107 => {
        trace.push(format!("get_edge_weight_from_node_ids({:?}, {:?})", &data.getedgeweightfromnodeids.src, &data.getedgeweightfromnodeids.dst));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_edge_weight_from_node_ids(data.getedgeweightfromnodeids.src.clone(), data.getedgeweightfromnodeids.dst.clone());
    }
    

    108 => {
        trace.push(format!("get_edge_weight_from_node_ids_and_edge_type_id({:?}, {:?}, {:?})", &data.getedgeweightfromnodeidsandedgetypeid.src, &data.getedgeweightfromnodeidsandedgetypeid.dst, &data.getedgeweightfromnodeidsandedgetypeid.edge_type));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_edge_weight_from_node_ids_and_edge_type_id(data.getedgeweightfromnodeidsandedgetypeid.src.clone(), data.getedgeweightfromnodeidsandedgetypeid.dst.clone(), data.getedgeweightfromnodeidsandedgetypeid.edge_type.clone());
    }
    

    109 => {
        trace.push(format!("get_unchecked_node_name_from_node_id({:?})", &data.getuncheckednodenamefromnodeid.node_id));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        unsafe{let _ = graph.get_unchecked_node_name_from_node_id(data.getuncheckednodenamefromnodeid.node_id.clone());}
    }
    

    110 => {
        trace.push(format!("get_node_name_from_node_id({:?})", &data.getnodenamefromnodeid.node_id));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_node_name_from_node_id(data.getnodenamefromnodeid.node_id.clone());
    }
    

    111 => {
        trace.push(format!("get_edge_node_names_from_edge_node_ids({:?})", &data.getedgenodenamesfromedgenodeids.edge_node_ids));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_edge_node_names_from_edge_node_ids(data.getedgenodenamesfromedgenodeids.edge_node_ids.clone());
    }
    

    112 => {
        trace.push(format!("get_edge_count_from_edge_type_id({:?})", &data.getedgecountfromedgetypeid.edge_type_id));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_edge_count_from_edge_type_id(data.getedgecountfromedgetypeid.edge_type_id.clone());
    }
    

    113 => {
        trace.push(format!("get_node_count_from_node_type_id({:?})", &data.getnodecountfromnodetypeid.node_type_id));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_node_count_from_node_type_id(data.getnodecountfromnodetypeid.node_type_id.clone());
    }
    

    114 => {
        trace.push(format!("get_destination_node_id_from_edge_id({:?})", &data.getdestinationnodeidfromedgeid.edge_id));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_destination_node_id_from_edge_id(data.getdestinationnodeidfromedgeid.edge_id.clone());
    }
    

    115 => {
        trace.push(format!("get_neighbour_node_ids_from_node_id({:?})", &data.getneighbournodeidsfromnodeid.node_id));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_neighbour_node_ids_from_node_id(data.getneighbournodeidsfromnodeid.node_id.clone());
    }
    

    116 => {
        trace.push(format!("get_minmax_edge_ids_from_node_ids({:?}, {:?})", &data.getminmaxedgeidsfromnodeids.src, &data.getminmaxedgeidsfromnodeids.dst));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_minmax_edge_ids_from_node_ids(data.getminmaxedgeidsfromnodeids.src.clone(), data.getminmaxedgeidsfromnodeids.dst.clone());
    }
    

    117 => {
        trace.push(format!("get_edge_id_from_node_ids_and_edge_type_id({:?}, {:?}, {:?})", &data.getedgeidfromnodeidsandedgetypeid.src, &data.getedgeidfromnodeidsandedgetypeid.dst, &data.getedgeidfromnodeidsandedgetypeid.edge_type));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_edge_id_from_node_ids_and_edge_type_id(data.getedgeidfromnodeidsandedgetypeid.src.clone(), data.getedgeidfromnodeidsandedgetypeid.dst.clone(), data.getedgeidfromnodeidsandedgetypeid.edge_type.clone());
    }
    

    118 => {
        trace.push(format!("get_unchecked_minmax_edge_ids_from_source_node_id({:?})", &data.getuncheckedminmaxedgeidsfromsourcenodeid.src));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        unsafe{let _ = graph.get_unchecked_minmax_edge_ids_from_source_node_id(data.getuncheckedminmaxedgeidsfromsourcenodeid.src.clone());}
    }
    

    119 => {
        trace.push(format!("get_minmax_edge_ids_from_source_node_id({:?})", &data.getminmaxedgeidsfromsourcenodeid.src));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_minmax_edge_ids_from_source_node_id(data.getminmaxedgeidsfromsourcenodeid.src.clone());
    }
    

    120 => {
        trace.push(format!("get_node_type_name_from_node_type_id({:?})", &data.getnodetypenamefromnodetypeid.node_type_id));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_node_type_name_from_node_type_id(data.getnodetypenamefromnodetypeid.node_type_id.clone());
    }
    

    121 => {
        trace.push(format!("get_unchecked_node_type_names_from_node_type_ids({:?})", &data.getuncheckednodetypenamesfromnodetypeids.node_type_ids));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        unsafe{let _ = graph.get_unchecked_node_type_names_from_node_type_ids(data.getuncheckednodetypenamesfromnodetypeids.node_type_ids.clone());}
    }
    

    122 => {
        trace.push(format!("filter_from_ids({:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?})", &data.filterfromids.node_ids_to_keep, &data.filterfromids.node_ids_to_filter, &data.filterfromids.node_type_ids_to_keep, &data.filterfromids.node_type_ids_to_filter, &data.filterfromids.node_type_id_to_keep, &data.filterfromids.node_type_id_to_filter, &data.filterfromids.edge_ids_to_keep, &data.filterfromids.edge_ids_to_filter, &data.filterfromids.edge_node_ids_to_keep, &data.filterfromids.edge_node_ids_to_filter, &data.filterfromids.edge_type_ids_to_keep, &data.filterfromids.edge_type_ids_to_filter, &data.filterfromids.min_edge_weight, &data.filterfromids.max_edge_weight, &data.filterfromids.filter_singleton_nodes, &data.filterfromids.filter_singleton_nodes_with_selfloop, &data.filterfromids.filter_selfloops, &data.filterfromids.filter_parallel_edges, &data.filterfromids.verbose));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        graph = graph.filter_from_ids(data.filterfromids.node_ids_to_keep.clone(), data.filterfromids.node_ids_to_filter.clone(), data.filterfromids.node_type_ids_to_keep.clone(), data.filterfromids.node_type_ids_to_filter.clone(), data.filterfromids.node_type_id_to_keep.clone(), data.filterfromids.node_type_id_to_filter.clone(), data.filterfromids.edge_ids_to_keep.clone(), data.filterfromids.edge_ids_to_filter.clone(), data.filterfromids.edge_node_ids_to_keep.clone(), data.filterfromids.edge_node_ids_to_filter.clone(), data.filterfromids.edge_type_ids_to_keep.clone(), data.filterfromids.edge_type_ids_to_filter.clone(), data.filterfromids.min_edge_weight.clone(), data.filterfromids.max_edge_weight.clone(), data.filterfromids.filter_singleton_nodes.clone(), data.filterfromids.filter_singleton_nodes_with_selfloop.clone(), data.filterfromids.filter_selfloops.clone(), data.filterfromids.filter_parallel_edges.clone(), data.filterfromids.verbose.clone());
    }
    

    123 => {
        trace.push(format!("drop_unknown_node_types({:?})", &data.dropunknownnodetypes.verbose));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        graph = graph.drop_unknown_node_types(data.dropunknownnodetypes.verbose.clone());
    }
    

    124 => {
        trace.push(format!("drop_unknown_edge_types({:?})", &data.dropunknownedgetypes.verbose));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        graph = graph.drop_unknown_edge_types(data.dropunknownedgetypes.verbose.clone());
    }
    

    125 => {
        trace.push(format!("drop_singleton_nodes({:?})", &data.dropsingletonnodes.verbose));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        graph = graph.drop_singleton_nodes(data.dropsingletonnodes.verbose.clone());
    }
    

    126 => {
        trace.push(format!("drop_singleton_nodes_with_selfloops({:?})", &data.dropsingletonnodeswithselfloops.verbose));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        graph = graph.drop_singleton_nodes_with_selfloops(data.dropsingletonnodeswithselfloops.verbose.clone());
    }
    

    127 => {
        trace.push(format!("drop_selfloops({:?})", &data.dropselfloops.verbose));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        graph = graph.drop_selfloops(data.dropselfloops.verbose.clone());
    }
    

    128 => {
        trace.push(format!("drop_parallel_edges({:?})", &data.dropparalleledges.verbose));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        graph = graph.drop_parallel_edges(data.dropparalleledges.verbose.clone());
    }
    

    129 => {
        trace.push(format!("validate_node_id({:?})", &data.validatenodeid.node_id));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.validate_node_id(data.validatenodeid.node_id.clone());
    }
    

    130 => {
        trace.push(format!("validate_node_ids({:?})", &data.validatenodeids.node_ids));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.validate_node_ids(data.validatenodeids.node_ids.clone());
    }
    

    131 => {
        trace.push(format!("validate_edge_id({:?})", &data.validateedgeid.edge_id));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.validate_edge_id(data.validateedgeid.edge_id.clone());
    }
    

    132 => {
        trace.push(format!("validate_edge_ids({:?})", &data.validateedgeids.edge_ids));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.validate_edge_ids(data.validateedgeids.edge_ids.clone());
    }
    

    133 => {
        trace.push(format!("validate_node_type_id({:?})", &data.validatenodetypeid.node_type_id));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.validate_node_type_id(data.validatenodetypeid.node_type_id.clone());
    }
    

    134 => {
        trace.push(format!("validate_node_type_ids({:?})", &data.validatenodetypeids.node_type_ids));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.validate_node_type_ids(data.validatenodetypeids.node_type_ids.clone());
    }
    

    135 => {
        trace.push(format!("validate_edge_type_id({:?})", &data.validateedgetypeid.edge_type_id));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.validate_edge_type_id(data.validateedgetypeid.edge_type_id.clone());
    }
    

    136 => {
        trace.push(format!("validate_edge_type_ids({:?})", &data.validateedgetypeids.edge_type_ids));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.validate_edge_type_ids(data.validateedgetypeids.edge_type_ids.clone());
    }
    

    137 => {
        trace.push(format!("must_have_node_types()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.must_have_node_types();
    }
    

    138 => {
        trace.push(format!("must_have_edge_types()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.must_have_edge_types();
    }
    

    139 => {
        trace.push(format!("must_be_undirected()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.must_be_undirected();
    }
    

    140 => {
        trace.push(format!("must_be_multigraph()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.must_be_multigraph();
    }
    

    141 => {
        trace.push(format!("must_not_be_multigraph()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.must_not_be_multigraph();
    }
    

    142 => {
        trace.push(format!("must_have_edge_weights()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.must_have_edge_weights();
    }
    

    143 => {
        trace.push(format!("must_have_edge_weights_representing_probabilities()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.must_have_edge_weights_representing_probabilities();
    }
    

    144 => {
        trace.push(format!("must_have_positive_edge_weights()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.must_have_positive_edge_weights();
    }
    

    145 => {
        trace.push(format!("must_not_contain_weighted_singleton_nodes()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.must_not_contain_weighted_singleton_nodes();
    }
    

    146 => {
        trace.push(format!("must_have_edges()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.must_have_edges();
    }
    

    147 => {
        trace.push(format!("must_have_nodes()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.must_have_nodes();
    }
    

    148 => {
        trace.push(format!("iter_unweighted_degree_centrality()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.iter_unweighted_degree_centrality().map(|x| x.collect::<Vec<_>>());
            
    }
    

    149 => {
        trace.push(format!("par_iter_weighted_degree_centrality()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.par_iter_weighted_degree_centrality().map(|x| x.collect::<Vec<_>>());
            
    }
    

    150 => {
        trace.push(format!("get_unweighted_degree_centrality()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_unweighted_degree_centrality();
    }
    

    151 => {
        trace.push(format!("get_weighted_degree_centrality()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_weighted_degree_centrality();
    }
    

    152 => {
        trace.push(format!("get_unchecked_unweighted_closeness_centrality_from_node_id({:?})", &data.getuncheckedunweightedclosenesscentralityfromnodeid.node_id));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        unsafe{let _ = graph.get_unchecked_unweighted_closeness_centrality_from_node_id(data.getuncheckedunweightedclosenesscentralityfromnodeid.node_id.clone());}
    }
    

    153 => {
        trace.push(format!("get_unchecked_weighted_closeness_centrality_from_node_id({:?}, {:?})", &data.getuncheckedweightedclosenesscentralityfromnodeid.node_id, &data.getuncheckedweightedclosenesscentralityfromnodeid.use_edge_weights_as_probabilities));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        unsafe{let _ = graph.get_unchecked_weighted_closeness_centrality_from_node_id(data.getuncheckedweightedclosenesscentralityfromnodeid.node_id.clone(), data.getuncheckedweightedclosenesscentralityfromnodeid.use_edge_weights_as_probabilities.clone());}
    }
    

    154 => {
        trace.push(format!("par_iter_unweighted_closeness_centrality({:?})", &data.pariterunweightedclosenesscentrality.verbose));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.par_iter_unweighted_closeness_centrality(data.pariterunweightedclosenesscentrality.verbose.clone()).collect::<Vec<_>>();
            
    }
    

    155 => {
        trace.push(format!("par_iter_weighted_closeness_centrality({:?}, {:?})", &data.pariterweightedclosenesscentrality.use_edge_weights_as_probabilities, &data.pariterweightedclosenesscentrality.verbose));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.par_iter_weighted_closeness_centrality(data.pariterweightedclosenesscentrality.use_edge_weights_as_probabilities.clone(), data.pariterweightedclosenesscentrality.verbose.clone()).map(|x| x.collect::<Vec<_>>());
            
    }
    

    156 => {
        trace.push(format!("get_unweighted_closeness_centrality({:?})", &data.getunweightedclosenesscentrality.verbose));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_unweighted_closeness_centrality(data.getunweightedclosenesscentrality.verbose.clone());
    }
    

    157 => {
        trace.push(format!("get_weighted_closeness_centrality({:?}, {:?})", &data.getweightedclosenesscentrality.use_edge_weights_as_probabilities, &data.getweightedclosenesscentrality.verbose));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_weighted_closeness_centrality(data.getweightedclosenesscentrality.use_edge_weights_as_probabilities.clone(), data.getweightedclosenesscentrality.verbose.clone());
    }
    

    158 => {
        trace.push(format!("get_unchecked_unweighted_harmonic_centrality_from_node_id({:?})", &data.getuncheckedunweightedharmoniccentralityfromnodeid.node_id));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        unsafe{let _ = graph.get_unchecked_unweighted_harmonic_centrality_from_node_id(data.getuncheckedunweightedharmoniccentralityfromnodeid.node_id.clone());}
    }
    

    159 => {
        trace.push(format!("get_unchecked_weighted_harmonic_centrality_from_node_id({:?}, {:?})", &data.getuncheckedweightedharmoniccentralityfromnodeid.node_id, &data.getuncheckedweightedharmoniccentralityfromnodeid.use_edge_weights_as_probabilities));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        unsafe{let _ = graph.get_unchecked_weighted_harmonic_centrality_from_node_id(data.getuncheckedweightedharmoniccentralityfromnodeid.node_id.clone(), data.getuncheckedweightedharmoniccentralityfromnodeid.use_edge_weights_as_probabilities.clone());}
    }
    

    160 => {
        trace.push(format!("par_iter_unweighted_harmonic_centrality({:?})", &data.pariterunweightedharmoniccentrality.verbose));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.par_iter_unweighted_harmonic_centrality(data.pariterunweightedharmoniccentrality.verbose.clone()).collect::<Vec<_>>();
            
    }
    

    161 => {
        trace.push(format!("par_iter_weighted_harmonic_centrality({:?}, {:?})", &data.pariterweightedharmoniccentrality.use_edge_weights_as_probabilities, &data.pariterweightedharmoniccentrality.verbose));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.par_iter_weighted_harmonic_centrality(data.pariterweightedharmoniccentrality.use_edge_weights_as_probabilities.clone(), data.pariterweightedharmoniccentrality.verbose.clone()).map(|x| x.collect::<Vec<_>>());
            
    }
    

    162 => {
        trace.push(format!("get_unweighted_harmonic_centrality({:?})", &data.getunweightedharmoniccentrality.verbose));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_unweighted_harmonic_centrality(data.getunweightedharmoniccentrality.verbose.clone());
    }
    

    163 => {
        trace.push(format!("get_weighted_harmonic_centrality({:?}, {:?})", &data.getweightedharmoniccentrality.use_edge_weights_as_probabilities, &data.getweightedharmoniccentrality.verbose));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_weighted_harmonic_centrality(data.getweightedharmoniccentrality.use_edge_weights_as_probabilities.clone(), data.getweightedharmoniccentrality.verbose.clone());
    }
    

    164 => {
        trace.push(format!("get_stress_centrality({:?}, {:?})", &data.getstresscentrality.normalize, &data.getstresscentrality.verbose));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_stress_centrality(data.getstresscentrality.normalize.clone(), data.getstresscentrality.verbose.clone());
    }
    

    165 => {
        trace.push(format!("get_betweenness_centrality({:?}, {:?})", &data.getbetweennesscentrality.normalize, &data.getbetweennesscentrality.verbose));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_betweenness_centrality(data.getbetweennesscentrality.normalize.clone(), data.getbetweennesscentrality.verbose.clone());
    }
    

    166 => {
        trace.push(format!("get_unweighted_eigenvector_centrality({:?}, {:?})", &data.getunweightedeigenvectorcentrality.maximum_iterations_number, &data.getunweightedeigenvectorcentrality.tollerance));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_unweighted_eigenvector_centrality(data.getunweightedeigenvectorcentrality.maximum_iterations_number.clone(), data.getunweightedeigenvectorcentrality.tollerance.clone());
    }
    

    167 => {
        trace.push(format!("get_weighted_eigenvector_centrality({:?}, {:?})", &data.getweightedeigenvectorcentrality.maximum_iterations_number, &data.getweightedeigenvectorcentrality.tollerance));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_weighted_eigenvector_centrality(data.getweightedeigenvectorcentrality.maximum_iterations_number.clone(), data.getweightedeigenvectorcentrality.tollerance.clone());
    }
    

    168 => {
        trace.push(format!("remove_inplace_node_type_ids({:?})", &data.removeinplacenodetypeids.node_type_ids_to_remove));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.remove_inplace_node_type_ids(data.removeinplacenodetypeids.node_type_ids_to_remove.clone());
    }
    

    169 => {
        trace.push(format!("remove_inplace_singleton_node_types()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.remove_inplace_singleton_node_types();
    }
    

    170 => {
        trace.push(format!("remove_inplace_edge_type_ids({:?})", &data.removeinplaceedgetypeids.edge_type_ids_to_remove));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.remove_inplace_edge_type_ids(data.removeinplaceedgetypeids.edge_type_ids_to_remove.clone());
    }
    

    171 => {
        trace.push(format!("remove_inplace_singleton_edge_types()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.remove_inplace_singleton_edge_types();
    }
    

    172 => {
        trace.push(format!("remove_node_type_id({:?})", &data.removenodetypeid.node_type_id));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
        if let Ok(res) = graph.remove_node_type_id(data.removenodetypeid.node_type_id.clone()) {
            graph = res;
        }
        
    }
    

    173 => {
        trace.push(format!("remove_singleton_node_types()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
        if let Ok(res) = graph.remove_singleton_node_types() {
            graph = res;
        }
        
    }
    

    174 => {
        trace.push(format!("remove_edge_type_id({:?})", &data.removeedgetypeid.edge_type_id));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
        if let Ok(res) = graph.remove_edge_type_id(data.removeedgetypeid.edge_type_id.clone()) {
            graph = res;
        }
        
    }
    

    175 => {
        trace.push(format!("remove_singleton_edge_types()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
        if let Ok(res) = graph.remove_singleton_edge_types() {
            graph = res;
        }
        
    }
    

    176 => {
        trace.push(format!("remove_inplace_node_types()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.remove_inplace_node_types();
    }
    

    177 => {
        trace.push(format!("remove_node_types()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
        if let Ok(res) = graph.remove_node_types() {
            graph = res;
        }
        
    }
    

    178 => {
        trace.push(format!("remove_inplace_edge_types()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.remove_inplace_edge_types();
    }
    

    179 => {
        trace.push(format!("remove_edge_types({:?})", &data.removeedgetypes.verbose));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
        if let Ok(res) = graph.remove_edge_types(data.removeedgetypes.verbose.clone()) {
            graph = res;
        }
        
    }
    

    180 => {
        trace.push(format!("remove_inplace_edge_weights()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.remove_inplace_edge_weights();
    }
    

    181 => {
        trace.push(format!("remove_edge_weights()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
        if let Ok(res) = graph.remove_edge_weights() {
            graph = res;
        }
        
    }
    

    182 => {
        trace.push(format!("encode_edge({:?}, {:?})", &data.encodeedge.src, &data.encodeedge.dst));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.encode_edge(data.encodeedge.src.clone(), data.encodeedge.dst.clone());
    }
    

    183 => {
        trace.push(format!("decode_edge({:?})", &data.decodeedge.edge));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.decode_edge(data.decodeedge.edge.clone());
    }
    

    184 => {
        trace.push(format!("get_max_encodable_edge_number()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_max_encodable_edge_number();
    }
    

    185 => {
        trace.push(format!("approximated_vertex_cover_bitvec()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.approximated_vertex_cover_bitvec();
    }
    

    186 => {
        trace.push(format!("approximated_vertex_cover_set()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.approximated_vertex_cover_set();
    }
    

    187 => {
        trace.push(format!("report()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.report();
    }
    

    188 => {
        trace.push(format!("get_node_report_from_node_id({:?})", &data.getnodereportfromnodeid.node_id));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_node_report_from_node_id(data.getnodereportfromnodeid.node_id.clone());
    }
    

    189 => {
        trace.push(format!("get_peculiarities_report_markdown()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_peculiarities_report_markdown();
    }
    

    190 => {
        trace.push(format!("textual_report({:?})", &data.textualreport.verbose));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.textual_report(data.textualreport.verbose.clone());
    }
    

    191 => {
        trace.push(format!("get_connected_components_number({:?})", &data.getconnectedcomponentsnumber.verbose));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_connected_components_number(data.getconnectedcomponentsnumber.verbose.clone());
    }
    

    192 => {
        trace.push(format!("get_singleton_nodes_number()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_singleton_nodes_number();
    }
    

    193 => {
        trace.push(format!("get_weighted_singleton_nodes_number()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_weighted_singleton_nodes_number();
    }
    

    194 => {
        trace.push(format!("get_disconnected_nodes_number()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_disconnected_nodes_number();
    }
    

    195 => {
        trace.push(format!("get_singleton_node_ids()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_singleton_node_ids();
    }
    

    196 => {
        trace.push(format!("get_singleton_node_names()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_singleton_node_names();
    }
    

    197 => {
        trace.push(format!("get_singleton_nodes_with_selfloops_number()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_singleton_nodes_with_selfloops_number();
    }
    

    198 => {
        trace.push(format!("get_singleton_with_selfloops_node_ids()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_singleton_with_selfloops_node_ids();
    }
    

    199 => {
        trace.push(format!("get_singleton_with_selfloops_node_names()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_singleton_with_selfloops_node_names();
    }
    

    200 => {
        trace.push(format!("get_connected_nodes_number()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_connected_nodes_number();
    }
    

    201 => {
        trace.push(format!("get_density()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_density();
    }
    

    202 => {
        trace.push(format!("get_trap_nodes_rate()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_trap_nodes_rate();
    }
    

    203 => {
        trace.push(format!("get_unweighted_node_degrees_mean()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_unweighted_node_degrees_mean();
    }
    

    204 => {
        trace.push(format!("get_undirected_edges_number()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_undirected_edges_number();
    }
    

    205 => {
        trace.push(format!("get_unique_undirected_edges_number()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_unique_undirected_edges_number();
    }
    

    206 => {
        trace.push(format!("get_edges_number()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_edges_number();
    }
    

    207 => {
        trace.push(format!("get_unique_edges_number()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_unique_edges_number();
    }
    

    208 => {
        trace.push(format!("get_unweighted_node_degrees_median()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_unweighted_node_degrees_median();
    }
    

    209 => {
        trace.push(format!("get_weighted_node_degrees_median()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_weighted_node_degrees_median();
    }
    

    210 => {
        trace.push(format!("get_unchecked_unweighted_max_node_degree()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        unsafe{let _ = graph.get_unchecked_unweighted_max_node_degree();}
    }
    

    211 => {
        trace.push(format!("get_weighted_max_node_degree()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_weighted_max_node_degree();
    }
    

    212 => {
        trace.push(format!("get_unweighted_max_node_degree()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_unweighted_max_node_degree();
    }
    

    213 => {
        trace.push(format!("get_unchecked_argmax_node_degree()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        unsafe{let _ = graph.get_unchecked_argmax_node_degree();}
    }
    

    214 => {
        trace.push(format!("get_argmax_node_degree()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_argmax_node_degree();
    }
    

    215 => {
        trace.push(format!("get_unchecked_unweighted_min_node_degree()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        unsafe{let _ = graph.get_unchecked_unweighted_min_node_degree();}
    }
    

    216 => {
        trace.push(format!("get_weighted_min_node_degree()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_weighted_min_node_degree();
    }
    

    217 => {
        trace.push(format!("get_min_node_degree()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_min_node_degree();
    }
    

    218 => {
        trace.push(format!("get_unweighted_node_degrees_mode()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_unweighted_node_degrees_mode();
    }
    

    219 => {
        trace.push(format!("get_selfloop_nodes_number()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_selfloop_nodes_number();
    }
    

    220 => {
        trace.push(format!("get_unique_selfloop_number()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_unique_selfloop_number();
    }
    

    221 => {
        trace.push(format!("get_selfloop_nodes_rate()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_selfloop_nodes_rate();
    }
    

    222 => {
        trace.push(format!("get_name()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_name();
    }
    

    223 => {
        trace.push(format!("get_trap_nodes_number()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_trap_nodes_number();
    }
    

    224 => {
        trace.push(format!("get_source_node_ids({:?})", &data.getsourcenodeids.directed));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_source_node_ids(data.getsourcenodeids.directed.clone());
    }
    

    225 => {
        trace.push(format!("get_source_names({:?})", &data.getsourcenames.directed));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_source_names(data.getsourcenames.directed.clone());
    }
    

    226 => {
        trace.push(format!("get_destination_node_ids({:?})", &data.getdestinationnodeids.directed));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_destination_node_ids(data.getdestinationnodeids.directed.clone());
    }
    

    227 => {
        trace.push(format!("get_destination_names({:?})", &data.getdestinationnames.directed));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_destination_names(data.getdestinationnames.directed.clone());
    }
    

    228 => {
        trace.push(format!("get_node_names()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_node_names();
    }
    

    229 => {
        trace.push(format!("get_node_ids()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_node_ids();
    }
    

    230 => {
        trace.push(format!("get_edge_type_ids()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_edge_type_ids();
    }
    

    231 => {
        trace.push(format!("get_unique_edge_type_ids()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_unique_edge_type_ids();
    }
    

    232 => {
        trace.push(format!("get_edge_type_names()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_edge_type_names();
    }
    

    233 => {
        trace.push(format!("get_unique_edge_type_names()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_unique_edge_type_names();
    }
    

    234 => {
        trace.push(format!("get_edge_weights()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_edge_weights();
    }
    

    235 => {
        trace.push(format!("get_min_edge_weight()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_min_edge_weight();
    }
    

    236 => {
        trace.push(format!("get_max_edge_weight()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_max_edge_weight();
    }
    

    237 => {
        trace.push(format!("get_node_type_ids()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_node_type_ids();
    }
    

    238 => {
        trace.push(format!("get_one_hot_encoded_node_types()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_one_hot_encoded_node_types();
    }
    

    239 => {
        trace.push(format!("get_one_hot_encoded_known_node_types()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_one_hot_encoded_known_node_types();
    }
    

    240 => {
        trace.push(format!("get_one_hot_encoded_edge_types()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_one_hot_encoded_edge_types();
    }
    

    241 => {
        trace.push(format!("get_one_hot_encoded_known_edge_types()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_one_hot_encoded_known_edge_types();
    }
    

    242 => {
        trace.push(format!("get_node_type_names()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_node_type_names();
    }
    

    243 => {
        trace.push(format!("get_unique_node_type_ids()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_unique_node_type_ids();
    }
    

    244 => {
        trace.push(format!("get_unique_node_type_names()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_unique_node_type_names();
    }
    

    245 => {
        trace.push(format!("get_unique_directed_edges_number()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_unique_directed_edges_number();
    }
    

    246 => {
        trace.push(format!("get_nodes_mapping()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_nodes_mapping();
    }
    

    247 => {
        trace.push(format!("get_edge_node_ids({:?})", &data.getedgenodeids.directed));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_edge_node_ids(data.getedgenodeids.directed.clone());
    }
    

    248 => {
        trace.push(format!("get_edge_node_names({:?})", &data.getedgenodenames.directed));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_edge_node_names(data.getedgenodenames.directed.clone());
    }
    

    249 => {
        trace.push(format!("get_unknown_node_types_number()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_unknown_node_types_number();
    }
    

    250 => {
        trace.push(format!("get_known_node_types_number()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_known_node_types_number();
    }
    

    251 => {
        trace.push(format!("get_unknown_node_types_rate()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_unknown_node_types_rate();
    }
    

    252 => {
        trace.push(format!("get_known_node_types_rate()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_known_node_types_rate();
    }
    

    253 => {
        trace.push(format!("get_minimum_node_types_number()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_minimum_node_types_number();
    }
    

    254 => {
        trace.push(format!("get_singleton_node_types_number()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_singleton_node_types_number();
    }
    

    255 => {
        trace.push(format!("get_singleton_node_type_ids()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_singleton_node_type_ids();
    }
    

    256 => {
        trace.push(format!("get_singleton_node_type_names()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_singleton_node_type_names();
    }
    

    257 => {
        trace.push(format!("get_unknown_edge_types_number()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_unknown_edge_types_number();
    }
    

    258 => {
        trace.push(format!("get_edge_ids_with_unknown_edge_types()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_edge_ids_with_unknown_edge_types();
    }
    

    259 => {
        trace.push(format!("get_edge_ids_with_known_edge_types()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_edge_ids_with_known_edge_types();
    }
    

    260 => {
        trace.push(format!("get_node_ids_with_unknown_node_types()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_node_ids_with_unknown_node_types();
    }
    

    261 => {
        trace.push(format!("get_node_ids_with_known_node_types()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_node_ids_with_known_node_types();
    }
    

    262 => {
        trace.push(format!("get_known_edge_types_number()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_known_edge_types_number();
    }
    

    263 => {
        trace.push(format!("get_unknown_edge_types_rate()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_unknown_edge_types_rate();
    }
    

    264 => {
        trace.push(format!("get_known_edge_types_rate()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_known_edge_types_rate();
    }
    

    265 => {
        trace.push(format!("get_minimum_edge_types_number()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_minimum_edge_types_number();
    }
    

    266 => {
        trace.push(format!("get_singleton_edge_types_number()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_singleton_edge_types_number();
    }
    

    267 => {
        trace.push(format!("get_singleton_edge_type_ids()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_singleton_edge_type_ids();
    }
    

    268 => {
        trace.push(format!("get_singleton_edge_type_names()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_singleton_edge_type_names();
    }
    

    269 => {
        trace.push(format!("get_nodes_number()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_nodes_number();
    }
    

    270 => {
        trace.push(format!("get_node_connected_component_ids({:?})", &data.getnodeconnectedcomponentids.verbose));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_node_connected_component_ids(data.getnodeconnectedcomponentids.verbose.clone());
    }
    

    271 => {
        trace.push(format!("get_directed_edges_number()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_directed_edges_number();
    }
    

    272 => {
        trace.push(format!("get_edge_types_number()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_edge_types_number();
    }
    

    273 => {
        trace.push(format!("get_node_types_number()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_node_types_number();
    }
    

    274 => {
        trace.push(format!("get_unweighted_node_degrees()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_unweighted_node_degrees();
    }
    

    275 => {
        trace.push(format!("get_weighted_node_degrees()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_weighted_node_degrees();
    }
    

    276 => {
        trace.push(format!("get_not_singletons_node_ids()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_not_singletons_node_ids();
    }
    

    277 => {
        trace.push(format!("get_dense_nodes_mapping()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_dense_nodes_mapping();
    }
    

    278 => {
        trace.push(format!("get_multigraph_edges_number()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_multigraph_edges_number();
    }
    

    279 => {
        trace.push(format!("get_cumulative_node_degrees()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_cumulative_node_degrees();
    }
    

    280 => {
        trace.push(format!("get_unique_source_nodes_number()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_unique_source_nodes_number();
    }
    

    281 => {
        trace.push(format!("get_edge_type_id_counts_hashmap()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_edge_type_id_counts_hashmap();
    }
    

    282 => {
        trace.push(format!("get_edge_type_names_counts_hashmap()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_edge_type_names_counts_hashmap();
    }
    

    283 => {
        trace.push(format!("get_node_type_id_counts_hashmap()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_node_type_id_counts_hashmap();
    }
    

    284 => {
        trace.push(format!("get_node_type_names_counts_hashmap()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_node_type_names_counts_hashmap();
    }
    

    285 => {
        trace.push(format!("get_dense_binary_adjacency_matrix()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_dense_binary_adjacency_matrix();
    }
    

    286 => {
        trace.push(format!("get_dense_weighted_adjacency_matrix({:?})", &data.getdenseweightedadjacencymatrix.weight));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_dense_weighted_adjacency_matrix(data.getdenseweightedadjacencymatrix.weight.clone());
    }
    

    287 => {
        trace.push(format!("iter_node_ids()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.iter_node_ids().collect::<Vec<_>>();
            
    }
    

    288 => {
        trace.push(format!("par_iter_node_ids()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.par_iter_node_ids().collect::<Vec<_>>();
            
    }
    

    289 => {
        trace.push(format!("iter_node_names()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.iter_node_names().collect::<Vec<_>>();
            
    }
    

    290 => {
        trace.push(format!("par_iter_node_names()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.par_iter_node_names().collect::<Vec<_>>();
            
    }
    

    291 => {
        trace.push(format!("iter_unique_node_type_ids()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.iter_unique_node_type_ids().map(|x| x.collect::<Vec<_>>());
            
    }
    

    292 => {
        trace.push(format!("iter_node_type_counts()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.iter_node_type_counts().map(|x| x.collect::<Vec<_>>());
            
    }
    

    293 => {
        trace.push(format!("iter_unique_node_type_ids_and_counts()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.iter_unique_node_type_ids_and_counts().map(|x| x.collect::<Vec<_>>());
            
    }
    

    294 => {
        trace.push(format!("iter_unique_node_type_names()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.iter_unique_node_type_names().map(|x| x.collect::<Vec<_>>());
            
    }
    

    295 => {
        trace.push(format!("iter_unique_node_type_names_and_counts()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.iter_unique_node_type_names_and_counts().map(|x| x.collect::<Vec<_>>());
            
    }
    

    296 => {
        trace.push(format!("iter_unique_edge_type_ids()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.iter_unique_edge_type_ids().map(|x| x.collect::<Vec<_>>());
            
    }
    

    297 => {
        trace.push(format!("iter_edge_type_counts()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.iter_edge_type_counts().map(|x| x.collect::<Vec<_>>());
            
    }
    

    298 => {
        trace.push(format!("iter_unique_edge_type_ids_and_counts()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.iter_unique_edge_type_ids_and_counts().map(|x| x.collect::<Vec<_>>());
            
    }
    

    299 => {
        trace.push(format!("iter_unique_edge_type_names_and_counts()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.iter_unique_edge_type_names_and_counts().map(|x| x.collect::<Vec<_>>());
            
    }
    

    300 => {
        trace.push(format!("iter_unique_edge_type_names()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.iter_unique_edge_type_names().map(|x| x.collect::<Vec<_>>());
            
    }
    

    301 => {
        trace.push(format!("iter_unweighted_node_degrees()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.iter_unweighted_node_degrees().collect::<Vec<_>>();
            
    }
    

    302 => {
        trace.push(format!("par_iter_unweighted_node_degrees()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.par_iter_unweighted_node_degrees().collect::<Vec<_>>();
            
    }
    

    303 => {
        trace.push(format!("iter_weighted_node_degrees()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.iter_weighted_node_degrees().map(|x| x.collect::<Vec<_>>());
            
    }
    

    304 => {
        trace.push(format!("par_iter_weighted_node_degrees()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.par_iter_weighted_node_degrees().map(|x| x.collect::<Vec<_>>());
            
    }
    

    305 => {
        trace.push(format!("iter_connected_node_ids()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.iter_connected_node_ids().collect::<Vec<_>>();
            
    }
    

    306 => {
        trace.push(format!("iter_singleton_node_ids()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.iter_singleton_node_ids().collect::<Vec<_>>();
            
    }
    

    307 => {
        trace.push(format!("iter_singleton_node_names()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.iter_singleton_node_names().collect::<Vec<_>>();
            
    }
    

    308 => {
        trace.push(format!("iter_singleton_with_selfloops_node_ids()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.iter_singleton_with_selfloops_node_ids().collect::<Vec<_>>();
            
    }
    

    309 => {
        trace.push(format!("iter_singleton_with_selfloops_node_names()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.iter_singleton_with_selfloops_node_names().collect::<Vec<_>>();
            
    }
    

    310 => {
        trace.push(format!("iter_singleton_node_type_ids()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.iter_singleton_node_type_ids().map(|x| x.collect::<Vec<_>>());
            
    }
    

    311 => {
        trace.push(format!("iter_singleton_edge_type_ids()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.iter_singleton_edge_type_ids().map(|x| x.collect::<Vec<_>>());
            
    }
    

    312 => {
        trace.push(format!("iter_singleton_node_type_names()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.iter_singleton_node_type_names().map(|x| x.collect::<Vec<_>>());
            
    }
    

    313 => {
        trace.push(format!("iter_singleton_edge_type_names()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.iter_singleton_edge_type_names().map(|x| x.collect::<Vec<_>>());
            
    }
    

    314 => {
        trace.push(format!("iter_source_node_ids({:?})", &data.itersourcenodeids.directed));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.iter_source_node_ids(data.itersourcenodeids.directed.clone()).collect::<Vec<_>>();
            
    }
    

    315 => {
        trace.push(format!("iter_edge_weights()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.iter_edge_weights().map(|x| x.collect::<Vec<_>>());
            
    }
    

    316 => {
        trace.push(format!("par_iter_edge_weights()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.par_iter_edge_weights().map(|x| x.collect::<Vec<_>>());
            
    }
    

    317 => {
        trace.push(format!("par_iter_source_node_ids({:?})", &data.paritersourcenodeids.directed));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.par_iter_source_node_ids(data.paritersourcenodeids.directed.clone()).collect::<Vec<_>>();
            
    }
    

    318 => {
        trace.push(format!("iter_destination_node_ids({:?})", &data.iterdestinationnodeids.directed));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.iter_destination_node_ids(data.iterdestinationnodeids.directed.clone()).collect::<Vec<_>>();
            
    }
    

    319 => {
        trace.push(format!("par_iter_destination_node_ids({:?})", &data.pariterdestinationnodeids.directed));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.par_iter_destination_node_ids(data.pariterdestinationnodeids.directed.clone()).collect::<Vec<_>>();
            
    }
    

    320 => {
        trace.push(format!("iter_node_ids_and_node_type_ids()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.iter_node_ids_and_node_type_ids().collect::<Vec<_>>();
            
    }
    

    321 => {
        trace.push(format!("iter_unchecked_node_type_ids()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        unsafe{
            let _ = graph.iter_unchecked_node_type_ids().collect::<Vec<_>>();
            }
    }
    

    322 => {
        trace.push(format!("iter_one_hot_encoded_node_type_ids()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.iter_one_hot_encoded_node_type_ids().map(|x| x.collect::<Vec<_>>());
            
    }
    

    323 => {
        trace.push(format!("iter_one_hot_encoded_known_node_type_ids()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.iter_one_hot_encoded_known_node_type_ids().map(|x| x.collect::<Vec<_>>());
            
    }
    

    324 => {
        trace.push(format!("par_iter_unchecked_node_ids_and_node_type_ids()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        unsafe{
            let _ = graph.par_iter_unchecked_node_ids_and_node_type_ids().collect::<Vec<_>>();
            }
    }
    

    325 => {
        trace.push(format!("iter_node_names_and_node_type_names()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.iter_node_names_and_node_type_names().collect::<Vec<_>>();
            
    }
    

    326 => {
        trace.push(format!("par_iter_node_names_and_node_type_names()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.par_iter_node_names_and_node_type_names().collect::<Vec<_>>();
            
    }
    

    327 => {
        trace.push(format!("iter_edge_node_ids({:?})", &data.iteredgenodeids.directed));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.iter_edge_node_ids(data.iteredgenodeids.directed.clone()).collect::<Vec<_>>();
            
    }
    

    328 => {
        trace.push(format!("iter_edges({:?})", &data.iteredges.directed));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.iter_edges(data.iteredges.directed.clone()).collect::<Vec<_>>();
            
    }
    

    329 => {
        trace.push(format!("par_iter_edge_node_ids({:?})", &data.pariteredgenodeids.directed));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.par_iter_edge_node_ids(data.pariteredgenodeids.directed.clone()).collect::<Vec<_>>();
            
    }
    

    330 => {
        trace.push(format!("par_iter_directed_edge_ids()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.par_iter_directed_edge_ids().collect::<Vec<_>>();
            
    }
    

    331 => {
        trace.push(format!("par_iter_edges({:?})", &data.pariteredges.directed));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.par_iter_edges(data.pariteredges.directed.clone()).collect::<Vec<_>>();
            
    }
    

    332 => {
        trace.push(format!("iter_edge_node_ids_and_edge_weight()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.iter_edge_node_ids_and_edge_weight().map(|x| x.collect::<Vec<_>>());
            
    }
    

    333 => {
        trace.push(format!("par_iter_edge_node_ids_and_edge_weight()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.par_iter_edge_node_ids_and_edge_weight().map(|x| x.collect::<Vec<_>>());
            
    }
    

    334 => {
        trace.push(format!("iter_edge_node_ids_and_edge_type_id({:?})", &data.iteredgenodeidsandedgetypeid.directed));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.iter_edge_node_ids_and_edge_type_id(data.iteredgenodeidsandedgetypeid.directed.clone()).collect::<Vec<_>>();
            
    }
    

    335 => {
        trace.push(format!("iter_one_hot_encoded_edge_type_ids()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.iter_one_hot_encoded_edge_type_ids().map(|x| x.collect::<Vec<_>>());
            
    }
    

    336 => {
        trace.push(format!("iter_one_hot_encoded_known_edge_type_ids()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.iter_one_hot_encoded_known_edge_type_ids().map(|x| x.collect::<Vec<_>>());
            
    }
    

    337 => {
        trace.push(format!("iter_edge_node_names_and_edge_type_name({:?})", &data.iteredgenodenamesandedgetypename.directed));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.iter_edge_node_names_and_edge_type_name(data.iteredgenodenamesandedgetypename.directed.clone());
    }
    

    338 => {
        trace.push(format!("par_iter_edge_node_names_and_edge_type_name({:?})", &data.pariteredgenodenamesandedgetypename.directed));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.par_iter_edge_node_names_and_edge_type_name(data.pariteredgenodenamesandedgetypename.directed.clone());
    }
    

    339 => {
        trace.push(format!("par_iter_edge_node_ids_and_edge_type_id({:?})", &data.pariteredgenodeidsandedgetypeid.directed));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.par_iter_edge_node_ids_and_edge_type_id(data.pariteredgenodeidsandedgetypeid.directed.clone()).collect::<Vec<_>>();
            
    }
    

    340 => {
        trace.push(format!("par_iter_edge_node_names_and_edge_type_name_and_edge_weight({:?})", &data.pariteredgenodenamesandedgetypenameandedgeweight.directed));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.par_iter_edge_node_names_and_edge_type_name_and_edge_weight(data.pariteredgenodenamesandedgetypenameandedgeweight.directed.clone());
    }
    

    341 => {
        trace.push(format!("iter_edge_node_names_and_edge_type_name_and_edge_weight({:?})", &data.iteredgenodenamesandedgetypenameandedgeweight.directed));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.iter_edge_node_names_and_edge_type_name_and_edge_weight(data.iteredgenodenamesandedgetypenameandedgeweight.directed.clone());
    }
    

    342 => {
        trace.push(format!("par_iter_edge_node_ids_and_edge_type_id_and_edge_weight({:?})", &data.pariteredgenodeidsandedgetypeidandedgeweight.directed));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.par_iter_edge_node_ids_and_edge_type_id_and_edge_weight(data.pariteredgenodeidsandedgetypeidandedgeweight.directed.clone()).collect::<Vec<_>>();
            
    }
    

    343 => {
        trace.push(format!("iter_edge_node_ids_and_edge_type_id_and_edge_weight({:?})", &data.iteredgenodeidsandedgetypeidandedgeweight.directed));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.iter_edge_node_ids_and_edge_type_id_and_edge_weight(data.iteredgenodeidsandedgetypeidandedgeweight.directed.clone()).collect::<Vec<_>>();
            
    }
    

    344 => {
        trace.push(format!("iter_unique_edge_node_ids({:?})", &data.iteruniqueedgenodeids.directed));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.iter_unique_edge_node_ids(data.iteruniqueedgenodeids.directed.clone()).collect::<Vec<_>>();
            
    }
    

    345 => {
        trace.push(format!("iter_unique_source_node_ids()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.iter_unique_source_node_ids().collect::<Vec<_>>();
            
    }
    

    346 => {
        trace.push(format!("iter_edge_ids_with_unknown_edge_types()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.iter_edge_ids_with_unknown_edge_types().map(|x| x.collect::<Vec<_>>());
            
    }
    

    347 => {
        trace.push(format!("iter_edge_ids_with_known_edge_types()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.iter_edge_ids_with_known_edge_types().map(|x| x.collect::<Vec<_>>());
            
    }
    

    348 => {
        trace.push(format!("iter_edge_node_ids_with_unknown_edge_types({:?})", &data.iteredgenodeidswithunknownedgetypes.directed));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.iter_edge_node_ids_with_unknown_edge_types(data.iteredgenodeidswithunknownedgetypes.directed.clone()).map(|x| x.collect::<Vec<_>>());
            
    }
    

    349 => {
        trace.push(format!("iter_edge_node_ids_with_known_edge_types({:?})", &data.iteredgenodeidswithknownedgetypes.directed));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.iter_edge_node_ids_with_known_edge_types(data.iteredgenodeidswithknownedgetypes.directed.clone()).map(|x| x.collect::<Vec<_>>());
            
    }
    

    350 => {
        trace.push(format!("iter_node_ids_with_unknown_node_types()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.iter_node_ids_with_unknown_node_types().map(|x| x.collect::<Vec<_>>());
            
    }
    

    351 => {
        trace.push(format!("iter_node_ids_with_known_node_types()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.iter_node_ids_with_known_node_types().map(|x| x.collect::<Vec<_>>());
            
    }
    

    352 => {
        trace.push(format!("par_iter_edge_ids_with_unknown_edge_types()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.par_iter_edge_ids_with_unknown_edge_types().map(|x| x.collect::<Vec<_>>());
            
    }
    

    353 => {
        trace.push(format!("par_iter_edge_ids_with_known_edge_types()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.par_iter_edge_ids_with_known_edge_types().map(|x| x.collect::<Vec<_>>());
            
    }
    

    354 => {
        trace.push(format!("par_iter_edge_node_ids_with_unknown_edge_types({:?})", &data.pariteredgenodeidswithunknownedgetypes.directed));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.par_iter_edge_node_ids_with_unknown_edge_types(data.pariteredgenodeidswithunknownedgetypes.directed.clone()).map(|x| x.collect::<Vec<_>>());
            
    }
    

    355 => {
        trace.push(format!("par_iter_edge_node_ids_with_known_edge_types({:?})", &data.pariteredgenodeidswithknownedgetypes.directed));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.par_iter_edge_node_ids_with_known_edge_types(data.pariteredgenodeidswithknownedgetypes.directed.clone()).map(|x| x.collect::<Vec<_>>());
            
    }
    

    356 => {
        trace.push(format!("par_iter_node_ids_with_unknown_node_types()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.par_iter_node_ids_with_unknown_node_types().map(|x| x.collect::<Vec<_>>());
            
    }
    

    357 => {
        trace.push(format!("par_iter_node_ids_with_known_node_types()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.par_iter_node_ids_with_known_node_types().map(|x| x.collect::<Vec<_>>());
            
    }
    

    358 => {
        trace.push(format!("get_unweighted_laplacian_transformed_graph({:?})", &data.getunweightedlaplaciantransformedgraph.verbose));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        graph = graph.get_unweighted_laplacian_transformed_graph(data.getunweightedlaplaciantransformedgraph.verbose.clone());
    }
    

    359 => {
        trace.push(format!("get_unweighted_random_walk_normalized_laplacian_transformed_graph({:?})", &data.getunweightedrandomwalknormalizedlaplaciantransformedgraph.verbose));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        graph = graph.get_unweighted_random_walk_normalized_laplacian_transformed_graph(data.getunweightedrandomwalknormalizedlaplaciantransformedgraph.verbose.clone());
    }
    

    360 => {
        trace.push(format!("get_unweighted_symmetric_normalized_laplacian_transformed_graph({:?})", &data.getunweightedsymmetricnormalizedlaplaciantransformedgraph.verbose));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
        if let Ok(res) = graph.get_unweighted_symmetric_normalized_laplacian_transformed_graph(data.getunweightedsymmetricnormalizedlaplaciantransformedgraph.verbose.clone()) {
            graph = res;
        }
        
    }
    

    361 => {
        trace.push(format!("get_unweighted_symmetric_normalized_transformed_graph({:?})", &data.getunweightedsymmetricnormalizedtransformedgraph.verbose));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
        if let Ok(res) = graph.get_unweighted_symmetric_normalized_transformed_graph(data.getunweightedsymmetricnormalizedtransformedgraph.verbose.clone()) {
            graph = res;
        }
        
    }
    

    362 => {
        trace.push(format!("get_weighted_laplacian_transformed_graph({:?})", &data.getweightedlaplaciantransformedgraph.verbose));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
        if let Ok(res) = graph.get_weighted_laplacian_transformed_graph(data.getweightedlaplaciantransformedgraph.verbose.clone()) {
            graph = res;
        }
        
    }
    

    363 => {
        trace.push(format!("get_weighted_symmetric_normalized_laplacian_transformed_graph({:?})", &data.getweightedsymmetricnormalizedlaplaciantransformedgraph.verbose));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
        if let Ok(res) = graph.get_weighted_symmetric_normalized_laplacian_transformed_graph(data.getweightedsymmetricnormalizedlaplaciantransformedgraph.verbose.clone()) {
            graph = res;
        }
        
    }
    

    364 => {
        trace.push(format!("get_weighted_symmetric_normalized_transformed_graph({:?})", &data.getweightedsymmetricnormalizedtransformedgraph.verbose));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
        if let Ok(res) = graph.get_weighted_symmetric_normalized_transformed_graph(data.getweightedsymmetricnormalizedtransformedgraph.verbose.clone()) {
            graph = res;
        }
        
    }
    

    365 => {
        trace.push(format!("get_weighted_random_walk_normalized_laplacian_transformed_graph({:?})", &data.getweightedrandomwalknormalizedlaplaciantransformedgraph.verbose));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
        if let Ok(res) = graph.get_weighted_random_walk_normalized_laplacian_transformed_graph(data.getweightedrandomwalknormalizedlaplaciantransformedgraph.verbose.clone()) {
            graph = res;
        }
        
    }
    

    366 => {
        trace.push(format!("has_nodes()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.has_nodes();
    }
    

    367 => {
        trace.push(format!("has_edges()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.has_edges();
    }
    

    368 => {
        trace.push(format!("has_trap_nodes()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.has_trap_nodes();
    }
    

    369 => {
        trace.push(format!("is_directed()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.is_directed();
    }
    

    370 => {
        trace.push(format!("has_edge_weights()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.has_edge_weights();
    }
    

    371 => {
        trace.push(format!("has_edge_weights_representing_probabilities()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.has_edge_weights_representing_probabilities();
    }
    

    372 => {
        trace.push(format!("has_weighted_singleton_nodes()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.has_weighted_singleton_nodes();
    }
    

    373 => {
        trace.push(format!("has_negative_edge_weights()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.has_negative_edge_weights();
    }
    

    374 => {
        trace.push(format!("has_edge_types()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.has_edge_types();
    }
    

    375 => {
        trace.push(format!("has_selfloops()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.has_selfloops();
    }
    

    376 => {
        trace.push(format!("has_disconnected_nodes()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.has_disconnected_nodes();
    }
    

    377 => {
        trace.push(format!("has_singleton_nodes()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.has_singleton_nodes();
    }
    

    378 => {
        trace.push(format!("has_singleton_nodes_with_selfloops()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.has_singleton_nodes_with_selfloops();
    }
    

    379 => {
        trace.push(format!("is_connected({:?})", &data.isconnected.verbose));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.is_connected(data.isconnected.verbose.clone());
    }
    

    380 => {
        trace.push(format!("has_node_types()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.has_node_types();
    }
    

    381 => {
        trace.push(format!("has_multilabel_node_types()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.has_multilabel_node_types();
    }
    

    382 => {
        trace.push(format!("has_unknown_node_types()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.has_unknown_node_types();
    }
    

    383 => {
        trace.push(format!("has_unknown_edge_types()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.has_unknown_edge_types();
    }
    

    384 => {
        trace.push(format!("has_homogeneous_node_types()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.has_homogeneous_node_types();
    }
    

    385 => {
        trace.push(format!("has_homogeneous_edge_types()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.has_homogeneous_edge_types();
    }
    

    386 => {
        trace.push(format!("has_singleton_node_types()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.has_singleton_node_types();
    }
    

    387 => {
        trace.push(format!("has_node_oddities()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.has_node_oddities();
    }
    

    388 => {
        trace.push(format!("has_node_types_oddities()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.has_node_types_oddities();
    }
    

    389 => {
        trace.push(format!("has_singleton_edge_types()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.has_singleton_edge_types();
    }
    

    390 => {
        trace.push(format!("has_edge_types_oddities()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.has_edge_types_oddities();
    }
    

    391 => {
        trace.push(format!("is_multigraph()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.is_multigraph();
    }
    

    392 => {
        trace.push(format!("compute_hash()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.compute_hash();
    }
    

    393 => {
        trace.push(format!("node_label_holdout({:?}, {:?}, {:?})", &data.nodelabelholdout.train_size, &data.nodelabelholdout.use_stratification, &data.nodelabelholdout.random_state));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
        if let Ok((res1, res2)) = graph.node_label_holdout(data.nodelabelholdout.train_size.clone(), data.nodelabelholdout.use_stratification.clone(), data.nodelabelholdout.random_state.clone()) {
            if rng.next() % 2 == 0 {
                graph = res1;
            } else {
                graph = res2;
            }
        }
        
    }
    

    394 => {
        trace.push(format!("edge_label_holdout({:?}, {:?}, {:?})", &data.edgelabelholdout.train_size, &data.edgelabelholdout.use_stratification, &data.edgelabelholdout.random_state));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
        if let Ok((res1, res2)) = graph.edge_label_holdout(data.edgelabelholdout.train_size.clone(), data.edgelabelholdout.use_stratification.clone(), data.edgelabelholdout.random_state.clone()) {
            if rng.next() % 2 == 0 {
                graph = res1;
            } else {
                graph = res2;
            }
        }
        
    }
    

    395 => {
        trace.push(format!("random_subgraph({:?}, {:?}, {:?})", &data.randomsubgraph.nodes_number, &data.randomsubgraph.random_state, &data.randomsubgraph.verbose));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
        if let Ok(res) = graph.random_subgraph(data.randomsubgraph.nodes_number.clone(), data.randomsubgraph.random_state.clone(), data.randomsubgraph.verbose.clone()) {
            graph = res;
        }
        
    }
    
            _ => unreachable!()
        }
    }
    
    let _ = graph::test_utilities::default_test_suite(&mut graph, None);

    Ok(())
}
