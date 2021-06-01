
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
pub struct RandomSpanningArborescenceKruskal {
    pub random_state : Option<EdgeT>,
    pub undesired_edge_types : Option<HashSet<Option<EdgeTypeT>>>,
    pub verbose : Option<bool>,
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
pub struct Overlaps {
    pub other : &Graph,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct Contains {
    pub other : &Graph,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct Node2vec {
    pub walk_parameters : &'a WalksParameters,
    pub quantity : NodeT,
    pub window_size : usize,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct CooccurenceMatrix {
    pub walks_parameters : &'a WalksParameters,
    pub window_size : usize,
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
pub struct LinkPredictionIds {
    pub idx : u64,
    pub batch_size : usize,
    pub negative_samples : f64,
    pub avoid_false_negatives : bool,
    pub maximal_sampling_attempts : usize,
    pub graph_to_avoid : &'a Option<&Graph>,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct LinkPredictionDegrees {
    pub idx : u64,
    pub batch_size : usize,
    pub normalize : bool,
    pub negative_samples : f64,
    pub avoid_false_negatives : bool,
    pub maximal_sampling_attempts : usize,
    pub graph_to_avoid : &'a Option<&Graph>,
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
pub struct AreNodesRemappable {
    pub other : &Graph,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct Remap {
    pub other : &Graph,
    pub verbose : Option<bool>,
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
pub struct IsUncheckedSingletonFromNodeName {
    pub node_name : &str,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct IsSingletonFromNodeName {
    pub node_name : &str,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct HasNodeName {
    pub node_name : &str,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct HasNodeTypeId {
    pub node_type_id : NodeTypeT,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct HasNodeTypeName {
    pub node_type_name : &str,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct HasEdgeTypeId {
    pub edge_type_id : EdgeTypeT,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct HasEdgeTypeName {
    pub edge_type_name : &str,
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
pub struct HasNodeNameAndNodeTypeName {
    pub node_name : &str,
    pub node_type_name : Option<Vec<String>>,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct HasEdgeFromNodeNames {
    pub src_name : &str,
    pub dst_name : &str,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct HasEdgeFromNodeNamesAndEdgeTypeName {
    pub src_name : &str,
    pub dst_name : &str,
    pub edge_type_name : Option<&str>,
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
pub struct GetUnweightedMinimumPathNodeIdsFromNodeNames {
    pub src_node_name : &str,
    pub dst_node_name : &str,
    pub maximal_depth : Option<NodeT>,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct GetUnweightedMinimumPathNodeNamesFromNodeNames {
    pub src_node_name : &str,
    pub dst_node_name : &str,
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
pub struct GetUnweightedKShortestPathNodeIdsFromNodeNames {
    pub src_node_name : &str,
    pub dst_node_name : &str,
    pub k : usize,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct GetUnweightedKShortestPathNodeNamesFromNodeNames {
    pub src_node_name : &str,
    pub dst_node_name : &str,
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
pub struct GetUnweightedEccentricityFromNodeName {
    pub node_name : &str,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct GetWeightedEccentricityFromNodeName {
    pub node_name : &str,
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
pub struct GetWeightedMinimumPathNodeIdsFromNodeNames {
    pub src_node_name : &str,
    pub dst_node_name : &str,
    pub use_edge_weights_as_probabilities : Option<bool>,
    pub maximal_depth : Option<NodeT>,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct GetWeightedMinimumPathNodeNamesFromNodeNames {
    pub src_node_name : &str,
    pub dst_node_name : &str,
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
pub struct GetBreathFirstSearchFromNodeNames {
    pub src_node_name : &str,
    pub maybe_dst_node_name : Option<&str>,
    pub maybe_dst_node_names : Option<Vec<&str>>,
    pub compute_distances : Option<bool>,
    pub compute_predecessors : Option<bool>,
    pub compute_visited : Option<bool>,
    pub maximal_depth : Option<NodeT>,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct GetDijkstraFromNodeNames {
    pub src_node_name : &str,
    pub maybe_dst_node_name : Option<&str>,
    pub maybe_dst_node_names : Option<Vec<&str>>,
    pub compute_predecessors : Option<bool>,
    pub maximal_depth : Option<NodeT>,
    pub use_edge_weights_as_probabilities : Option<bool>,
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
pub struct GetUncheckedNodeIdFromNodeName {
    pub node_name : &str,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct GetUncheckedEdgeTypeIdFromEdgeTypeName {
    pub edge_type_name : &str,
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
pub struct GetNodeDegreeFromNodeName {
    pub node_name : &str,
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
pub struct GetNodeTypeNamesFromNodeName {
    pub node_name : &str,
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
pub struct GetEdgeWeightFromNodeNamesAndEdgeTypeName {
    pub src : &str,
    pub dst : &str,
    pub edge_type : Option<&str>,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct GetEdgeWeightFromNodeNames {
    pub src_name : &str,
    pub dst_name : &str,
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
pub struct GetNodeIdFromNodeName {
    pub node_name : &str,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct GetNodeIdsFromNodeNames {
    pub node_names : Vec<&str>,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct GetEdgeNodeIdsFromEdgeNodeNames {
    pub edge_node_names : Vec<(&str, &str)>,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct GetEdgeNodeNamesFromEdgeNodeIds {
    pub edge_node_ids : Vec<(NodeT, NodeT)>,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct GetNodeTypeIdFromNodeName {
    pub node_name : &str,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct GetNodeTypeNameFromNodeName {
    pub node_name : &str,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct GetEdgeCountFromEdgeTypeId {
    pub edge_type_id : Option<EdgeTypeT>,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct GetEdgeTypeIdFromEdgeTypeName {
    pub edge_type_name : Option<&str>,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct GetEdgeCountFromEdgeTypeName {
    pub edge_type_name : Option<&str>,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct GetNodeTypeIdFromNodeTypeName {
    pub node_type_name : &str,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct GetNodeCountFromNodeTypeId {
    pub node_type_id : Option<NodeTypeT>,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct GetNodeCountFromNodeTypeName {
    pub node_type_name : Option<&str>,
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
pub struct GetNeighbourNodeIdsFromNodeName {
    pub node_name : &str,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct GetNeighbourNodeNamesFromNodeName {
    pub node_name : &str,
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
pub struct GetEdgeIdFromNodeNames {
    pub src_name : &str,
    pub dst_name : &str,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct GetEdgeIdFromNodeNamesAndEdgeTypeName {
    pub src_name : &str,
    pub dst_name : &str,
    pub edge_type_name : Option<&str>,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct GetEdgeTypeIdsFromEdgeTypeNames {
    pub edge_type_names : Vec<Option<String>>,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct GetNodeTypeIdsFromNodeTypeNames {
    pub node_type_names : Vec<Option<String>>,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct GetMultipleNodeTypeIdsFromNodeTypeNames {
    pub node_type_names : Vec<Option<Vec<&str>>>,
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
pub struct FilterFromNames {
    pub node_names_to_keep : Option<Vec<&str>>,
    pub node_names_to_filter : Option<Vec<&str>>,
    pub node_type_names_to_keep : Option<Vec<Option<Vec<&str>>>>,
    pub node_type_names_to_filter : Option<Vec<Option<Vec<&str>>>>,
    pub node_type_name_to_keep : Option<Vec<Option<String>>>,
    pub node_type_name_to_filter : Option<Vec<Option<String>>>,
    pub edge_node_names_to_keep : Option<Vec<(&str, &str)>>,
    pub edge_node_names_to_filter : Option<Vec<(&str, &str)>>,
    pub edge_type_names_to_keep : Option<Vec<Option<String>>>,
    pub edge_type_names_to_filter : Option<Vec<Option<String>>>,
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
pub struct RemoveComponents {
    pub node_names : Option<Vec<String>>,
    pub node_types : Option<Vec<Option<String>>>,
    pub edge_types : Option<Vec<Option<String>>>,
    pub minimum_component_size : Option<NodeT>,
    pub top_k_components : Option<NodeT>,
    pub verbose : Option<bool>,
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
pub struct SetName {
    pub name : String,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct SetInplaceAllEdgeTypes {
    pub edge_type : S,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct SetAllEdgeTypes {
    pub edge_type : S,
    pub verbose : Option<bool>,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct SetInplaceAllNodeTypes {
    pub node_type : S,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct SetAllNodeTypes {
    pub node_type : S,
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
pub struct RemoveInplaceNodeTypeName {
    pub node_type_name : &str,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct RemoveNodeTypeId {
    pub node_type_id : NodeTypeT,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct RemoveNodeTypeName {
    pub node_type_name : &str,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct RemoveInplaceEdgeTypeName {
    pub edge_type_name : &str,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct RemoveEdgeTypeId {
    pub edge_type_id : EdgeTypeT,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct RemoveEdgeTypeName {
    pub edge_type_name : &str,
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
pub struct GetBipartiteEdges {
    pub removed_existing_edges : Option<bool>,
    pub first_nodes_set : Option<HashSet<String>>,
    pub second_nodes_set : Option<HashSet<String>>,
    pub first_node_types_set : Option<HashSet<String>>,
    pub second_node_types_set : Option<HashSet<String>>,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct GetBipartiteEdgeNames {
    pub removed_existing_edges : Option<bool>,
    pub first_nodes_set : Option<HashSet<String>>,
    pub second_nodes_set : Option<HashSet<String>>,
    pub first_node_types_set : Option<HashSet<String>>,
    pub second_node_types_set : Option<HashSet<String>>,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct GetStarEdges {
    pub central_node : String,
    pub removed_existing_edges : Option<bool>,
    pub star_points_nodes_set : Option<HashSet<String>>,
    pub star_points_node_types_set : Option<HashSet<String>>,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct GetStarEdgeNames {
    pub central_node : String,
    pub removed_existing_edges : Option<bool>,
    pub star_points_nodes_set : Option<HashSet<String>>,
    pub star_points_node_types_set : Option<HashSet<String>>,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct GetCliqueEdges {
    pub directed : Option<bool>,
    pub allow_selfloops : Option<bool>,
    pub removed_existing_edges : Option<bool>,
    pub allow_node_type_set : Option<HashSet<String>>,
    pub allow_node_set : Option<HashSet<String>>,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct GetCliqueEdgeNames {
    pub directed : Option<bool>,
    pub allow_selfloops : Option<bool>,
    pub removed_existing_edges : Option<bool>,
    pub allow_node_type_set : Option<HashSet<String>>,
    pub allow_node_set : Option<HashSet<String>>,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct Replace {
    pub node_name_mapping : Option<HashMap<String, String>>,
    pub node_type_name_mapping : Option<HashMap<String, String>>,
    pub node_type_names_mapping : Option<HashMap<Option<Vec<String>>, Option<Vec<String>>>>,
    pub edge_type_name_mapping : Option<HashMap<Option<String>, Option<String>>>,
    pub verbose : Option<bool>,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct ReplaceUnknownNodeTypesWithNodeTypeName {
    pub node_type_names : Vec<String>,
    pub verbose : Option<bool>,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct ReplaceUnknownEdgeTypesWithEdgeTypeName {
    pub edge_type_name : String,
    pub verbose : Option<bool>,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct OverlapTextualReport {
    pub other : &Graph,
    pub verbose : Option<bool>,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct GetNodeReportFromNodeId {
    pub node_id : NodeT,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct GetNodeReportFromNodeName {
    pub node_name : &str,
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
pub struct GenerateNewEdgesFromNodeFeatures {
    pub features : Vec<Vec<f64>>,
    pub neighbours_number : Option<NodeT>,
    pub max_degree : Option<NodeT>,
    pub distance_name : Option<&str>,
    pub verbose : Option<bool>,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct AddSelfloops {
    pub edge_type_name : Option<&str>,
    pub weight : Option<WeightT>,
    pub verbose : Option<bool>,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct SampleNegatives {
    pub negatives_number : EdgeT,
    pub random_state : Option<EdgeT>,
    pub seed_graph : Option<&Graph>,
    pub only_from_same_component : Option<bool>,
    pub verbose : Option<bool>,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct ConnectedHoldout {
    pub train_size : f64,
    pub random_state : Option<EdgeT>,
    pub edge_types : Option<Vec<Option<String>>>,
    pub include_all_edge_types : Option<bool>,
    pub verbose : Option<bool>,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct RandomHoldout {
    pub train_size : f64,
    pub random_state : Option<EdgeT>,
    pub include_all_edge_types : Option<bool>,
    pub edge_types : Option<Vec<Option<String>>>,
    pub min_number_overlaps : Option<EdgeT>,
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
pub struct Kfold {
    pub k : EdgeT,
    pub k_index : u64,
    pub edge_types : Option<Vec<Option<String>>>,
    pub random_state : Option<EdgeT>,
    pub verbose : Option<bool>,
}



#[derive(Arbitrary, Debug, Clone)]
pub struct MetaParams {
    pub seed: u64,
    pub randomspanningarborescencekruskal : RandomSpanningArborescenceKruskal,
    pub spanningarborescencekruskal : SpanningArborescenceKruskal,
    pub spanningarborescence : SpanningArborescence,
    pub connectedcomponents : ConnectedComponents,
    pub overlaps : Overlaps,
    pub contains : Contains,
    pub node2vec : Node2vec,
    pub cooccurencematrix : CooccurenceMatrix,
    pub getnodelabelpredictiontuplefromnodeids : GetNodeLabelPredictionTupleFromNodeIds,
    pub linkpredictionids : LinkPredictionIds,
    pub linkpredictiondegrees : LinkPredictionDegrees,
    pub pariteruncheckededgepredictionmetrics : ParIterUncheckedEdgePredictionMetrics,
    pub getokapibm25nodefeaturepropagation : GetOkapiBm25NodeFeaturePropagation,
    pub getokapibm25nodelabelpropagation : GetOkapiBm25NodeLabelPropagation,
    pub todot : ToDot,
    pub arenodesremappable : AreNodesRemappable,
    pub remap : Remap,
    pub isuncheckedsingletonfromnodeid : IsUncheckedSingletonFromNodeId,
    pub isuncheckeddisconnectedfromnodeid : IsUncheckedDisconnectedFromNodeId,
    pub isuncheckedconnectedfromnodeid : IsUncheckedConnectedFromNodeId,
    pub issingletonfromnodeid : IsSingletonFromNodeId,
    pub issingletonwithselfloopsfromnodeid : IsSingletonWithSelfloopsFromNodeId,
    pub isuncheckedsingletonfromnodename : IsUncheckedSingletonFromNodeName,
    pub issingletonfromnodename : IsSingletonFromNodeName,
    pub hasnodename : HasNodeName,
    pub hasnodetypeid : HasNodeTypeId,
    pub hasnodetypename : HasNodeTypeName,
    pub hasedgetypeid : HasEdgeTypeId,
    pub hasedgetypename : HasEdgeTypeName,
    pub hasedgefromnodeids : HasEdgeFromNodeIds,
    pub hasselfloopfromnodeid : HasSelfloopFromNodeId,
    pub hasedgefromnodeidsandedgetypeid : HasEdgeFromNodeIdsAndEdgeTypeId,
    pub isuncheckedtrapnodefromnodeid : IsUncheckedTrapNodeFromNodeId,
    pub istrapnodefromnodeid : IsTrapNodeFromNodeId,
    pub hasnodenameandnodetypename : HasNodeNameAndNodeTypeName,
    pub hasedgefromnodenames : HasEdgeFromNodeNames,
    pub hasedgefromnodenamesandedgetypename : HasEdgeFromNodeNamesAndEdgeTypeName,
    pub getuncheckedminpreferentialattachment : GetUncheckedMinPreferentialAttachment,
    pub getuncheckedmaxpreferentialattachment : GetUncheckedMaxPreferentialAttachment,
    pub getuncheckedpreferentialattachment : GetUncheckedPreferentialAttachment,
    pub getpreferentialattachment : GetPreferentialAttachment,
    pub getuncheckedjaccardcoefficient : GetUncheckedJaccardCoefficient,
    pub getjaccardcoefficient : GetJaccardCoefficient,
    pub getuncheckedadamicadarindex : GetUncheckedAdamicAdarIndex,
    pub getadamicadarindex : GetAdamicAdarIndex,
    pub getuncheckedresourceallocationindex : GetUncheckedResourceAllocationIndex,
    pub getresourceallocationindex : GetResourceAllocationIndex,
    pub enable : Enable,
    pub disableall : DisableAll,
    pub getunweightednumberoftriangles : GetUnweightedNumberOfTriangles,
    pub getunweightedtriadsnumber : GetUnweightedTriadsNumber,
    pub getweightedtriadsnumber : GetWeightedTriadsNumber,
    pub getunweightedtransitivity : GetUnweightedTransitivity,
    pub getunweightednumberoftrianglespernode : GetUnweightedNumberOfTrianglesPerNode,
    pub iterclusteringcoefficientpernode : IterClusteringCoefficientPerNode,
    pub getclusteringcoefficientpernode : GetClusteringCoefficientPerNode,
    pub getclusteringcoefficient : GetClusteringCoefficient,
    pub getaverageclusteringcoefficient : GetAverageClusteringCoefficient,
    pub getuncheckedbreathfirstsearchfromnodeids : GetUncheckedBreathFirstSearchFromNodeIds,
    pub getuncheckedunweightedminimumpathnodeidsfromnodeids : GetUncheckedUnweightedMinimumPathNodeIdsFromNodeIds,
    pub getuncheckedunweightedminimumpathnodenamesfromnodeids : GetUncheckedUnweightedMinimumPathNodeNamesFromNodeIds,
    pub getunweightedminimumpathnodeidsfromnodeids : GetUnweightedMinimumPathNodeIdsFromNodeIds,
    pub getunweightedminimumpathnodeidsfromnodenames : GetUnweightedMinimumPathNodeIdsFromNodeNames,
    pub getunweightedminimumpathnodenamesfromnodenames : GetUnweightedMinimumPathNodeNamesFromNodeNames,
    pub getuncheckedunweightedkshortestpathnodeidsfromnodeids : GetUncheckedUnweightedKShortestPathNodeIdsFromNodeIds,
    pub getunweightedkshortestpathnodeidsfromnodeids : GetUnweightedKShortestPathNodeIdsFromNodeIds,
    pub getunweightedkshortestpathnodeidsfromnodenames : GetUnweightedKShortestPathNodeIdsFromNodeNames,
    pub getunweightedkshortestpathnodenamesfromnodenames : GetUnweightedKShortestPathNodeNamesFromNodeNames,
    pub getuncheckedunweightedeccentricityfromnodeid : GetUncheckedUnweightedEccentricityFromNodeId,
    pub getuncheckedweightedeccentricityfromnodeid : GetUncheckedWeightedEccentricityFromNodeId,
    pub getunweightedeccentricityfromnodeid : GetUnweightedEccentricityFromNodeId,
    pub getweightedeccentricityfromnodeid : GetWeightedEccentricityFromNodeId,
    pub getunweightedeccentricityfromnodename : GetUnweightedEccentricityFromNodeName,
    pub getweightedeccentricityfromnodename : GetWeightedEccentricityFromNodeName,
    pub getuncheckeddijkstrafromnodeids : GetUncheckedDijkstraFromNodeIds,
    pub getuncheckedweightedminimumpathnodeidsfromnodeids : GetUncheckedWeightedMinimumPathNodeIdsFromNodeIds,
    pub getuncheckedweightedminimumpathnodenamesfromnodeids : GetUncheckedWeightedMinimumPathNodeNamesFromNodeIds,
    pub getweightedminimumpathnodeidsfromnodeids : GetWeightedMinimumPathNodeIdsFromNodeIds,
    pub getweightedminimumpathnodeidsfromnodenames : GetWeightedMinimumPathNodeIdsFromNodeNames,
    pub getweightedminimumpathnodenamesfromnodenames : GetWeightedMinimumPathNodeNamesFromNodeNames,
    pub getbreathfirstsearchfromnodeids : GetBreathFirstSearchFromNodeIds,
    pub getdijkstrafromnodeids : GetDijkstraFromNodeIds,
    pub getunweighteddiameter : GetUnweightedDiameter,
    pub getweighteddiameter : GetWeightedDiameter,
    pub getbreathfirstsearchfromnodenames : GetBreathFirstSearchFromNodeNames,
    pub getdijkstrafromnodenames : GetDijkstraFromNodeNames,
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
    pub stronglyconnectedcomponents : StronglyConnectedComponents,
    pub getuncheckededgeweightfromedgeid : GetUncheckedEdgeWeightFromEdgeId,
    pub getuncheckededgeweightfromnodeids : GetUncheckedEdgeWeightFromNodeIds,
    pub getuncheckednodeidfromnodename : GetUncheckedNodeIdFromNodeName,
    pub getuncheckededgetypeidfromedgetypename : GetUncheckedEdgeTypeIdFromEdgeTypeName,
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
    pub getnodedegreefromnodename : GetNodeDegreeFromNodeName,
    pub gettopkcentralnodenames : GetTopKCentralNodeNames,
    pub getuncheckednodetypeidfromnodeid : GetUncheckedNodeTypeIdFromNodeId,
    pub getnodetypeidfromnodeid : GetNodeTypeIdFromNodeId,
    pub getuncheckededgetypeidfromedgeid : GetUncheckedEdgeTypeIdFromEdgeId,
    pub getedgetypeidfromedgeid : GetEdgeTypeIdFromEdgeId,
    pub getuncheckednodetypenamesfromnodeid : GetUncheckedNodeTypeNamesFromNodeId,
    pub getnodetypenamesfromnodeid : GetNodeTypeNamesFromNodeId,
    pub getnodetypenamesfromnodename : GetNodeTypeNamesFromNodeName,
    pub getedgetypenamefromedgeid : GetEdgeTypeNameFromEdgeId,
    pub getedgetypenamefromedgetypeid : GetEdgeTypeNameFromEdgeTypeId,
    pub getedgeweightfromedgeid : GetEdgeWeightFromEdgeId,
    pub getedgeweightfromnodeids : GetEdgeWeightFromNodeIds,
    pub getedgeweightfromnodeidsandedgetypeid : GetEdgeWeightFromNodeIdsAndEdgeTypeId,
    pub getedgeweightfromnodenamesandedgetypename : GetEdgeWeightFromNodeNamesAndEdgeTypeName,
    pub getedgeweightfromnodenames : GetEdgeWeightFromNodeNames,
    pub getuncheckednodenamefromnodeid : GetUncheckedNodeNameFromNodeId,
    pub getnodenamefromnodeid : GetNodeNameFromNodeId,
    pub getnodeidfromnodename : GetNodeIdFromNodeName,
    pub getnodeidsfromnodenames : GetNodeIdsFromNodeNames,
    pub getedgenodeidsfromedgenodenames : GetEdgeNodeIdsFromEdgeNodeNames,
    pub getedgenodenamesfromedgenodeids : GetEdgeNodeNamesFromEdgeNodeIds,
    pub getnodetypeidfromnodename : GetNodeTypeIdFromNodeName,
    pub getnodetypenamefromnodename : GetNodeTypeNameFromNodeName,
    pub getedgecountfromedgetypeid : GetEdgeCountFromEdgeTypeId,
    pub getedgetypeidfromedgetypename : GetEdgeTypeIdFromEdgeTypeName,
    pub getedgecountfromedgetypename : GetEdgeCountFromEdgeTypeName,
    pub getnodetypeidfromnodetypename : GetNodeTypeIdFromNodeTypeName,
    pub getnodecountfromnodetypeid : GetNodeCountFromNodeTypeId,
    pub getnodecountfromnodetypename : GetNodeCountFromNodeTypeName,
    pub getdestinationnodeidfromedgeid : GetDestinationNodeIdFromEdgeId,
    pub getneighbournodeidsfromnodeid : GetNeighbourNodeIdsFromNodeId,
    pub getneighbournodeidsfromnodename : GetNeighbourNodeIdsFromNodeName,
    pub getneighbournodenamesfromnodename : GetNeighbourNodeNamesFromNodeName,
    pub getminmaxedgeidsfromnodeids : GetMinmaxEdgeIdsFromNodeIds,
    pub getedgeidfromnodeidsandedgetypeid : GetEdgeIdFromNodeIdsAndEdgeTypeId,
    pub getedgeidfromnodenames : GetEdgeIdFromNodeNames,
    pub getedgeidfromnodenamesandedgetypename : GetEdgeIdFromNodeNamesAndEdgeTypeName,
    pub getedgetypeidsfromedgetypenames : GetEdgeTypeIdsFromEdgeTypeNames,
    pub getnodetypeidsfromnodetypenames : GetNodeTypeIdsFromNodeTypeNames,
    pub getmultiplenodetypeidsfromnodetypenames : GetMultipleNodeTypeIdsFromNodeTypeNames,
    pub getuncheckedminmaxedgeidsfromsourcenodeid : GetUncheckedMinmaxEdgeIdsFromSourceNodeId,
    pub getminmaxedgeidsfromsourcenodeid : GetMinmaxEdgeIdsFromSourceNodeId,
    pub getnodetypenamefromnodetypeid : GetNodeTypeNameFromNodeTypeId,
    pub getuncheckednodetypenamesfromnodetypeids : GetUncheckedNodeTypeNamesFromNodeTypeIds,
    pub filterfromids : FilterFromIds,
    pub filterfromnames : FilterFromNames,
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
    pub musthavenodetypes : MustHaveNodeTypes,
    pub musthaveedgetypes : MustHaveEdgeTypes,
    pub mustbeundirected : MustBeUndirected,
    pub mustbemultigraph : MustBeMultigraph,
    pub mustnotbemultigraph : MustNotBeMultigraph,
    pub musthaveedgeweights : MustHaveEdgeWeights,
    pub musthaveedgeweightsrepresentingprobabilities : MustHaveEdgeWeightsRepresentingProbabilities,
    pub musthavepositiveedgeweights : MustHavePositiveEdgeWeights,
    pub mustnotcontainweightedsingletonnodes : MustNotContainWeightedSingletonNodes,
    pub musthaveedges : MustHaveEdges,
    pub musthavenodes : MustHaveNodes,
    pub removecomponents : RemoveComponents,
    pub iterunweighteddegreecentrality : IterUnweightedDegreeCentrality,
    pub pariterweighteddegreecentrality : ParIterWeightedDegreeCentrality,
    pub getunweighteddegreecentrality : GetUnweightedDegreeCentrality,
    pub getweighteddegreecentrality : GetWeightedDegreeCentrality,
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
    pub setname : SetName,
    pub setinplacealledgetypes : SetInplaceAllEdgeTypes,
    pub setalledgetypes : SetAllEdgeTypes,
    pub setinplaceallnodetypes : SetInplaceAllNodeTypes,
    pub setallnodetypes : SetAllNodeTypes,
    pub removeinplacenodetypeids : RemoveInplaceNodeTypeIds,
    pub removeinplacesingletonnodetypes : RemoveInplaceSingletonNodeTypes,
    pub removeinplaceedgetypeids : RemoveInplaceEdgeTypeIds,
    pub removeinplacesingletonedgetypes : RemoveInplaceSingletonEdgeTypes,
    pub removeinplacenodetypename : RemoveInplaceNodeTypeName,
    pub removenodetypeid : RemoveNodeTypeId,
    pub removesingletonnodetypes : RemoveSingletonNodeTypes,
    pub removenodetypename : RemoveNodeTypeName,
    pub removeinplaceedgetypename : RemoveInplaceEdgeTypeName,
    pub removeedgetypeid : RemoveEdgeTypeId,
    pub removesingletonedgetypes : RemoveSingletonEdgeTypes,
    pub removeedgetypename : RemoveEdgeTypeName,
    pub removeinplacenodetypes : RemoveInplaceNodeTypes,
    pub removenodetypes : RemoveNodeTypes,
    pub removeinplaceedgetypes : RemoveInplaceEdgeTypes,
    pub removeedgetypes : RemoveEdgeTypes,
    pub removeinplaceedgeweights : RemoveInplaceEdgeWeights,
    pub removeedgeweights : RemoveEdgeWeights,
    pub encodeedge : EncodeEdge,
    pub decodeedge : DecodeEdge,
    pub getmaxencodableedgenumber : GetMaxEncodableEdgeNumber,
    pub getbipartiteedges : GetBipartiteEdges,
    pub getbipartiteedgenames : GetBipartiteEdgeNames,
    pub getstaredges : GetStarEdges,
    pub getstaredgenames : GetStarEdgeNames,
    pub getcliqueedges : GetCliqueEdges,
    pub getcliqueedgenames : GetCliqueEdgeNames,
    pub replace : Replace,
    pub replaceunknownnodetypeswithnodetypename : ReplaceUnknownNodeTypesWithNodeTypeName,
    pub replaceunknownedgetypeswithedgetypename : ReplaceUnknownEdgeTypesWithEdgeTypeName,
    pub approximatedvertexcoverbitvec : ApproximatedVertexCoverBitvec,
    pub approximatedvertexcoverset : ApproximatedVertexCoverSet,
    pub report : Report,
    pub overlaptextualreport : OverlapTextualReport,
    pub getnodereportfromnodeid : GetNodeReportFromNodeId,
    pub getnodereportfromnodename : GetNodeReportFromNodeName,
    pub getpeculiaritiesreportmarkdown : GetPeculiaritiesReportMarkdown,
    pub textualreport : TextualReport,
    pub getconnectedcomponentsnumber : GetConnectedComponentsNumber,
    pub getsingletonnodesnumber : GetSingletonNodesNumber,
    pub getweightedsingletonnodesnumber : GetWeightedSingletonNodesNumber,
    pub getdisconnectednodesnumber : GetDisconnectedNodesNumber,
    pub getsingletonnodeids : GetSingletonNodeIds,
    pub getsingletonnodenames : GetSingletonNodeNames,
    pub getsingletonnodeswithselfloopsnumber : GetSingletonNodesWithSelfloopsNumber,
    pub getsingletonwithselfloopsnodeids : GetSingletonWithSelfloopsNodeIds,
    pub getsingletonwithselfloopsnodenames : GetSingletonWithSelfloopsNodeNames,
    pub getconnectednodesnumber : GetConnectedNodesNumber,
    pub getdensity : GetDensity,
    pub gettrapnodesrate : GetTrapNodesRate,
    pub getunweightednodedegreesmean : GetUnweightedNodeDegreesMean,
    pub getundirectededgesnumber : GetUndirectedEdgesNumber,
    pub getuniqueundirectededgesnumber : GetUniqueUndirectedEdgesNumber,
    pub getedgesnumber : GetEdgesNumber,
    pub getuniqueedgesnumber : GetUniqueEdgesNumber,
    pub getunweightednodedegreesmedian : GetUnweightedNodeDegreesMedian,
    pub getweightednodedegreesmedian : GetWeightedNodeDegreesMedian,
    pub getuncheckedunweightedmaxnodedegree : GetUncheckedUnweightedMaxNodeDegree,
    pub getweightedmaxnodedegree : GetWeightedMaxNodeDegree,
    pub getunweightedmaxnodedegree : GetUnweightedMaxNodeDegree,
    pub getuncheckedargmaxnodedegree : GetUncheckedArgmaxNodeDegree,
    pub getargmaxnodedegree : GetArgmaxNodeDegree,
    pub getuncheckedunweightedminnodedegree : GetUncheckedUnweightedMinNodeDegree,
    pub getweightedminnodedegree : GetWeightedMinNodeDegree,
    pub getminnodedegree : GetMinNodeDegree,
    pub getunweightednodedegreesmode : GetUnweightedNodeDegreesMode,
    pub getselfloopnodesnumber : GetSelfloopNodesNumber,
    pub getuniqueselfloopnumber : GetUniqueSelfloopNumber,
    pub getselfloopnodesrate : GetSelfloopNodesRate,
    pub getname : GetName,
    pub gettrapnodesnumber : GetTrapNodesNumber,
    pub getsourcenodeids : GetSourceNodeIds,
    pub getsourcenames : GetSourceNames,
    pub getdestinationnodeids : GetDestinationNodeIds,
    pub getdestinationnames : GetDestinationNames,
    pub getnodenames : GetNodeNames,
    pub getnodeids : GetNodeIds,
    pub getedgetypeids : GetEdgeTypeIds,
    pub getuniqueedgetypeids : GetUniqueEdgeTypeIds,
    pub getedgetypenames : GetEdgeTypeNames,
    pub getuniqueedgetypenames : GetUniqueEdgeTypeNames,
    pub getedgeweights : GetEdgeWeights,
    pub getminedgeweight : GetMinEdgeWeight,
    pub getmaxedgeweight : GetMaxEdgeWeight,
    pub getnodetypeids : GetNodeTypeIds,
    pub getonehotencodednodetypes : GetOneHotEncodedNodeTypes,
    pub getonehotencodedknownnodetypes : GetOneHotEncodedKnownNodeTypes,
    pub getonehotencodededgetypes : GetOneHotEncodedEdgeTypes,
    pub getonehotencodedknownedgetypes : GetOneHotEncodedKnownEdgeTypes,
    pub getnodetypenames : GetNodeTypeNames,
    pub getuniquenodetypeids : GetUniqueNodeTypeIds,
    pub getuniquenodetypenames : GetUniqueNodeTypeNames,
    pub getuniquedirectededgesnumber : GetUniqueDirectedEdgesNumber,
    pub getnodesmapping : GetNodesMapping,
    pub getedgenodeids : GetEdgeNodeIds,
    pub getedgenodenames : GetEdgeNodeNames,
    pub getunknownnodetypesnumber : GetUnknownNodeTypesNumber,
    pub getknownnodetypesnumber : GetKnownNodeTypesNumber,
    pub getunknownnodetypesrate : GetUnknownNodeTypesRate,
    pub getknownnodetypesrate : GetKnownNodeTypesRate,
    pub getminimumnodetypesnumber : GetMinimumNodeTypesNumber,
    pub getsingletonnodetypesnumber : GetSingletonNodeTypesNumber,
    pub getsingletonnodetypeids : GetSingletonNodeTypeIds,
    pub getsingletonnodetypenames : GetSingletonNodeTypeNames,
    pub getunknownedgetypesnumber : GetUnknownEdgeTypesNumber,
    pub getedgeidswithunknownedgetypes : GetEdgeIdsWithUnknownEdgeTypes,
    pub getedgeidswithknownedgetypes : GetEdgeIdsWithKnownEdgeTypes,
    pub getnodeidswithunknownnodetypes : GetNodeIdsWithUnknownNodeTypes,
    pub getnodeidswithknownnodetypes : GetNodeIdsWithKnownNodeTypes,
    pub getknownedgetypesnumber : GetKnownEdgeTypesNumber,
    pub getunknownedgetypesrate : GetUnknownEdgeTypesRate,
    pub getknownedgetypesrate : GetKnownEdgeTypesRate,
    pub getminimumedgetypesnumber : GetMinimumEdgeTypesNumber,
    pub getsingletonedgetypesnumber : GetSingletonEdgeTypesNumber,
    pub getsingletonedgetypeids : GetSingletonEdgeTypeIds,
    pub getsingletonedgetypenames : GetSingletonEdgeTypeNames,
    pub getnodesnumber : GetNodesNumber,
    pub getnodeconnectedcomponentids : GetNodeConnectedComponentIds,
    pub getdirectededgesnumber : GetDirectedEdgesNumber,
    pub getedgetypesnumber : GetEdgeTypesNumber,
    pub getnodetypesnumber : GetNodeTypesNumber,
    pub getunweightednodedegrees : GetUnweightedNodeDegrees,
    pub getweightednodedegrees : GetWeightedNodeDegrees,
    pub getnotsingletonsnodeids : GetNotSingletonsNodeIds,
    pub getdensenodesmapping : GetDenseNodesMapping,
    pub getmultigraphedgesnumber : GetMultigraphEdgesNumber,
    pub getcumulativenodedegrees : GetCumulativeNodeDegrees,
    pub getuniquesourcenodesnumber : GetUniqueSourceNodesNumber,
    pub getedgetypeidcountshashmap : GetEdgeTypeIdCountsHashmap,
    pub getedgetypenamescountshashmap : GetEdgeTypeNamesCountsHashmap,
    pub getnodetypeidcountshashmap : GetNodeTypeIdCountsHashmap,
    pub getnodetypenamescountshashmap : GetNodeTypeNamesCountsHashmap,
    pub getdensebinaryadjacencymatrix : GetDenseBinaryAdjacencyMatrix,
    pub getdenseweightedadjacencymatrix : GetDenseWeightedAdjacencyMatrix,
    pub iternodeids : IterNodeIds,
    pub pariternodeids : ParIterNodeIds,
    pub iternodenames : IterNodeNames,
    pub pariternodenames : ParIterNodeNames,
    pub iteruniquenodetypeids : IterUniqueNodeTypeIds,
    pub iternodetypecounts : IterNodeTypeCounts,
    pub iteruniquenodetypeidsandcounts : IterUniqueNodeTypeIdsAndCounts,
    pub iteruniquenodetypenames : IterUniqueNodeTypeNames,
    pub iteruniquenodetypenamesandcounts : IterUniqueNodeTypeNamesAndCounts,
    pub iteruniqueedgetypeids : IterUniqueEdgeTypeIds,
    pub iteredgetypecounts : IterEdgeTypeCounts,
    pub iteruniqueedgetypeidsandcounts : IterUniqueEdgeTypeIdsAndCounts,
    pub iteruniqueedgetypenamesandcounts : IterUniqueEdgeTypeNamesAndCounts,
    pub iteruniqueedgetypenames : IterUniqueEdgeTypeNames,
    pub iterunweightednodedegrees : IterUnweightedNodeDegrees,
    pub pariterunweightednodedegrees : ParIterUnweightedNodeDegrees,
    pub iterweightednodedegrees : IterWeightedNodeDegrees,
    pub pariterweightednodedegrees : ParIterWeightedNodeDegrees,
    pub iterconnectednodeids : IterConnectedNodeIds,
    pub itersingletonnodeids : IterSingletonNodeIds,
    pub itersingletonnodenames : IterSingletonNodeNames,
    pub itersingletonwithselfloopsnodeids : IterSingletonWithSelfloopsNodeIds,
    pub itersingletonwithselfloopsnodenames : IterSingletonWithSelfloopsNodeNames,
    pub itersingletonnodetypeids : IterSingletonNodeTypeIds,
    pub itersingletonedgetypeids : IterSingletonEdgeTypeIds,
    pub itersingletonnodetypenames : IterSingletonNodeTypeNames,
    pub itersingletonedgetypenames : IterSingletonEdgeTypeNames,
    pub itersourcenodeids : IterSourceNodeIds,
    pub iteredgeweights : IterEdgeWeights,
    pub pariteredgeweights : ParIterEdgeWeights,
    pub paritersourcenodeids : ParIterSourceNodeIds,
    pub iterdestinationnodeids : IterDestinationNodeIds,
    pub pariterdestinationnodeids : ParIterDestinationNodeIds,
    pub iternodeidsandnodetypeids : IterNodeIdsAndNodeTypeIds,
    pub iteruncheckednodetypeids : IterUncheckedNodeTypeIds,
    pub iteronehotencodednodetypeids : IterOneHotEncodedNodeTypeIds,
    pub iteronehotencodedknownnodetypeids : IterOneHotEncodedKnownNodeTypeIds,
    pub pariteruncheckednodeidsandnodetypeids : ParIterUncheckedNodeIdsAndNodeTypeIds,
    pub iternodenamesandnodetypenames : IterNodeNamesAndNodeTypeNames,
    pub pariternodenamesandnodetypenames : ParIterNodeNamesAndNodeTypeNames,
    pub iteredgenodeids : IterEdgeNodeIds,
    pub iteredges : IterEdges,
    pub pariteredgenodeids : ParIterEdgeNodeIds,
    pub pariterdirectededgeids : ParIterDirectedEdgeIds,
    pub pariteredges : ParIterEdges,
    pub iteredgenodeidsandedgeweight : IterEdgeNodeIdsAndEdgeWeight,
    pub pariteredgenodeidsandedgeweight : ParIterEdgeNodeIdsAndEdgeWeight,
    pub iteredgenodeidsandedgetypeid : IterEdgeNodeIdsAndEdgeTypeId,
    pub iteronehotencodededgetypeids : IterOneHotEncodedEdgeTypeIds,
    pub iteronehotencodedknownedgetypeids : IterOneHotEncodedKnownEdgeTypeIds,
    pub iteredgenodenamesandedgetypename : IterEdgeNodeNamesAndEdgeTypeName,
    pub pariteredgenodenamesandedgetypename : ParIterEdgeNodeNamesAndEdgeTypeName,
    pub pariteredgenodeidsandedgetypeid : ParIterEdgeNodeIdsAndEdgeTypeId,
    pub pariteredgenodenamesandedgetypenameandedgeweight : ParIterEdgeNodeNamesAndEdgeTypeNameAndEdgeWeight,
    pub iteredgenodenamesandedgetypenameandedgeweight : IterEdgeNodeNamesAndEdgeTypeNameAndEdgeWeight,
    pub pariteredgenodeidsandedgetypeidandedgeweight : ParIterEdgeNodeIdsAndEdgeTypeIdAndEdgeWeight,
    pub iteredgenodeidsandedgetypeidandedgeweight : IterEdgeNodeIdsAndEdgeTypeIdAndEdgeWeight,
    pub iteruniqueedgenodeids : IterUniqueEdgeNodeIds,
    pub iteruniquesourcenodeids : IterUniqueSourceNodeIds,
    pub iteredgeidswithunknownedgetypes : IterEdgeIdsWithUnknownEdgeTypes,
    pub iteredgeidswithknownedgetypes : IterEdgeIdsWithKnownEdgeTypes,
    pub iteredgenodeidswithunknownedgetypes : IterEdgeNodeIdsWithUnknownEdgeTypes,
    pub iteredgenodeidswithknownedgetypes : IterEdgeNodeIdsWithKnownEdgeTypes,
    pub iternodeidswithunknownnodetypes : IterNodeIdsWithUnknownNodeTypes,
    pub iternodeidswithknownnodetypes : IterNodeIdsWithKnownNodeTypes,
    pub pariteredgeidswithunknownedgetypes : ParIterEdgeIdsWithUnknownEdgeTypes,
    pub pariteredgeidswithknownedgetypes : ParIterEdgeIdsWithKnownEdgeTypes,
    pub pariteredgenodeidswithunknownedgetypes : ParIterEdgeNodeIdsWithUnknownEdgeTypes,
    pub pariteredgenodeidswithknownedgetypes : ParIterEdgeNodeIdsWithKnownEdgeTypes,
    pub pariternodeidswithunknownnodetypes : ParIterNodeIdsWithUnknownNodeTypes,
    pub pariternodeidswithknownnodetypes : ParIterNodeIdsWithKnownNodeTypes,
    pub getunweightedlaplaciantransformedgraph : GetUnweightedLaplacianTransformedGraph,
    pub getunweightedrandomwalknormalizedlaplaciantransformedgraph : GetUnweightedRandomWalkNormalizedLaplacianTransformedGraph,
    pub getunweightedsymmetricnormalizedlaplaciantransformedgraph : GetUnweightedSymmetricNormalizedLaplacianTransformedGraph,
    pub getunweightedsymmetricnormalizedtransformedgraph : GetUnweightedSymmetricNormalizedTransformedGraph,
    pub getweightedlaplaciantransformedgraph : GetWeightedLaplacianTransformedGraph,
    pub getweightedsymmetricnormalizedlaplaciantransformedgraph : GetWeightedSymmetricNormalizedLaplacianTransformedGraph,
    pub getweightedsymmetricnormalizedtransformedgraph : GetWeightedSymmetricNormalizedTransformedGraph,
    pub getweightedrandomwalknormalizedlaplaciantransformedgraph : GetWeightedRandomWalkNormalizedLaplacianTransformedGraph,
    pub hasnodes : HasNodes,
    pub hasedges : HasEdges,
    pub hastrapnodes : HasTrapNodes,
    pub isdirected : IsDirected,
    pub hasedgeweights : HasEdgeWeights,
    pub hasedgeweightsrepresentingprobabilities : HasEdgeWeightsRepresentingProbabilities,
    pub hasweightedsingletonnodes : HasWeightedSingletonNodes,
    pub hasnegativeedgeweights : HasNegativeEdgeWeights,
    pub hasedgetypes : HasEdgeTypes,
    pub hasselfloops : HasSelfloops,
    pub hasdisconnectednodes : HasDisconnectedNodes,
    pub hassingletonnodes : HasSingletonNodes,
    pub hassingletonnodeswithselfloops : HasSingletonNodesWithSelfloops,
    pub isconnected : IsConnected,
    pub hasnodetypes : HasNodeTypes,
    pub hasmultilabelnodetypes : HasMultilabelNodeTypes,
    pub hasunknownnodetypes : HasUnknownNodeTypes,
    pub hasunknownedgetypes : HasUnknownEdgeTypes,
    pub hashomogeneousnodetypes : HasHomogeneousNodeTypes,
    pub hashomogeneousedgetypes : HasHomogeneousEdgeTypes,
    pub hassingletonnodetypes : HasSingletonNodeTypes,
    pub hasnodeoddities : HasNodeOddities,
    pub hasnodetypesoddities : HasNodeTypesOddities,
    pub hassingletonedgetypes : HasSingletonEdgeTypes,
    pub hasedgetypesoddities : HasEdgeTypesOddities,
    pub ismultigraph : IsMultigraph,
    pub computehash : ComputeHash,
    pub generatenewedgesfromnodefeatures : GenerateNewEdgesFromNodeFeatures,
    pub addselfloops : AddSelfloops,
    pub samplenegatives : SampleNegatives,
    pub connectedholdout : ConnectedHoldout,
    pub randomholdout : RandomHoldout,
    pub nodelabelholdout : NodeLabelHoldout,
    pub edgelabelholdout : EdgeLabelHoldout,
    pub randomsubgraph : RandomSubgraph,
    pub kfold : Kfold,
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
        match rng.next() % 473 {

    0 => {
        trace.push(format!("random_spanning_arborescence_kruskal({:?}, {:?}, {:?})", data.randomspanningarborescencekruskal.random_state, data.randomspanningarborescencekruskal.undesired_edge_types, data.randomspanningarborescencekruskal.verbose));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        graph.random_spanning_arborescence_kruskal(data.randomspanningarborescencekruskal.random_state, data.randomspanningarborescencekruskal.undesired_edge_types, data.randomspanningarborescencekruskal.verbose)
    }
    

    1 => {
        trace.push(format!("spanning_arborescence_kruskal({:?})", data.spanningarborescencekruskal.verbose));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        graph.spanning_arborescence_kruskal(data.spanningarborescencekruskal.verbose)
    }
    

    2 => {
        trace.push(format!("spanning_arborescence({:?})", data.spanningarborescence.verbose));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.spanning_arborescence(data.spanningarborescence.verbose);
    }
    

    3 => {
        trace.push(format!("connected_components({:?})", data.connectedcomponents.verbose));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.connected_components(data.connectedcomponents.verbose);
    }
    

    4 => {
        trace.push(format!("overlaps({:?})", data.overlaps.other));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.overlaps(data.overlaps.other);
    }
    

    5 => {
        trace.push(format!("contains({:?})", data.contains.other));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.contains(data.contains.other);
    }
    

    6 => {
        trace.push(format!("node2vec({:?}, {:?}, {:?})", data.node2vec.walk_parameters, data.node2vec.quantity, data.node2vec.window_size));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.node2vec(data.node2vec.walk_parameters, data.node2vec.quantity, data.node2vec.window_size).map(|x| x.collect::<Vec<_>>());
            
    }
    

    7 => {
        trace.push(format!("cooccurence_matrix({:?}, {:?}, {:?})", data.cooccurencematrix.walks_parameters, data.cooccurencematrix.window_size, data.cooccurencematrix.verbose));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.cooccurence_matrix(data.cooccurencematrix.walks_parameters, data.cooccurencematrix.window_size, data.cooccurencematrix.verbose);
    }
    

    8 => {
        trace.push(format!("get_node_label_prediction_tuple_from_node_ids({:?}, {:?}, {:?}, {:?}, {:?})", data.getnodelabelpredictiontuplefromnodeids.node_ids, data.getnodelabelpredictiontuplefromnodeids.random_state, data.getnodelabelpredictiontuplefromnodeids.include_central_node, data.getnodelabelpredictiontuplefromnodeids.offset, data.getnodelabelpredictiontuplefromnodeids.max_neighbours));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        graph.get_node_label_prediction_tuple_from_node_ids(data.getnodelabelpredictiontuplefromnodeids.node_ids, data.getnodelabelpredictiontuplefromnodeids.random_state, data.getnodelabelpredictiontuplefromnodeids.include_central_node, data.getnodelabelpredictiontuplefromnodeids.offset, data.getnodelabelpredictiontuplefromnodeids.max_neighbours)
    }
    

    9 => {
        trace.push(format!("link_prediction_ids({:?}, {:?}, {:?}, {:?}, {:?}, {:?})", data.linkpredictionids.idx, data.linkpredictionids.batch_size, data.linkpredictionids.negative_samples, data.linkpredictionids.avoid_false_negatives, data.linkpredictionids.maximal_sampling_attempts, data.linkpredictionids.graph_to_avoid));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.link_prediction_ids(data.linkpredictionids.idx, data.linkpredictionids.batch_size, data.linkpredictionids.negative_samples, data.linkpredictionids.avoid_false_negatives, data.linkpredictionids.maximal_sampling_attempts, data.linkpredictionids.graph_to_avoid).map(|x| x.collect::<Vec<_>>());
            
    }
    

    10 => {
        trace.push(format!("link_prediction_degrees({:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?})", data.linkpredictiondegrees.idx, data.linkpredictiondegrees.batch_size, data.linkpredictiondegrees.normalize, data.linkpredictiondegrees.negative_samples, data.linkpredictiondegrees.avoid_false_negatives, data.linkpredictiondegrees.maximal_sampling_attempts, data.linkpredictiondegrees.graph_to_avoid));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.link_prediction_degrees(data.linkpredictiondegrees.idx, data.linkpredictiondegrees.batch_size, data.linkpredictiondegrees.normalize, data.linkpredictiondegrees.negative_samples, data.linkpredictiondegrees.avoid_false_negatives, data.linkpredictiondegrees.maximal_sampling_attempts, data.linkpredictiondegrees.graph_to_avoid).map(|x| x.collect::<Vec<_>>());
            
    }
    

    11 => {
        trace.push(format!("par_iter_unchecked_edge_prediction_metrics({:?}, {:?})", data.pariteruncheckededgepredictionmetrics.source_node_ids, data.pariteruncheckededgepredictionmetrics.destination_node_ids));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.par_iter_unchecked_edge_prediction_metrics(data.pariteruncheckededgepredictionmetrics.source_node_ids, data.pariteruncheckededgepredictionmetrics.destination_node_ids).collect::<Vec<_>>();
            
    }
    

    12 => {
        trace.push(format!("get_okapi_bm25_node_feature_propagation({:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?})", data.getokapibm25nodefeaturepropagation.features, data.getokapibm25nodefeaturepropagation.iterations, data.getokapibm25nodefeaturepropagation.maximal_distance, data.getokapibm25nodefeaturepropagation.k1, data.getokapibm25nodefeaturepropagation.b, data.getokapibm25nodefeaturepropagation.include_central_node, data.getokapibm25nodefeaturepropagation.verbose));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_okapi_bm25_node_feature_propagation(data.getokapibm25nodefeaturepropagation.features, data.getokapibm25nodefeaturepropagation.iterations, data.getokapibm25nodefeaturepropagation.maximal_distance, data.getokapibm25nodefeaturepropagation.k1, data.getokapibm25nodefeaturepropagation.b, data.getokapibm25nodefeaturepropagation.include_central_node, data.getokapibm25nodefeaturepropagation.verbose);
    }
    

    13 => {
        trace.push(format!("get_okapi_bm25_node_label_propagation({:?}, {:?}, {:?}, {:?}, {:?})", data.getokapibm25nodelabelpropagation.iterations, data.getokapibm25nodelabelpropagation.maximal_distance, data.getokapibm25nodelabelpropagation.k1, data.getokapibm25nodelabelpropagation.b, data.getokapibm25nodelabelpropagation.verbose));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_okapi_bm25_node_label_propagation(data.getokapibm25nodelabelpropagation.iterations, data.getokapibm25nodelabelpropagation.maximal_distance, data.getokapibm25nodelabelpropagation.k1, data.getokapibm25nodelabelpropagation.b, data.getokapibm25nodelabelpropagation.verbose);
    }
    

    14 => {
        trace.push(format!("to_dot({:?})", data.todot.use_node_names));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        graph.to_dot(data.todot.use_node_names)
    }
    

    15 => {
        trace.push(format!("are_nodes_remappable({:?})", data.arenodesremappable.other));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        graph.are_nodes_remappable(data.arenodesremappable.other)
    }
    

    16 => {
        trace.push(format!("remap({:?}, {:?})", data.remap.other, data.remap.verbose));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
        if let Ok(res) = graph.remap(data.remap.other, data.remap.verbose) {
            graph = res;
        }
        
    }
    

    17 => {
        trace.push(format!("is_unchecked_singleton_from_node_id({:?})", data.isuncheckedsingletonfromnodeid.node_id));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        graph.is_unchecked_singleton_from_node_id(data.isuncheckedsingletonfromnodeid.node_id)
    }
    

    18 => {
        trace.push(format!("is_unchecked_disconnected_from_node_id({:?})", data.isuncheckeddisconnectedfromnodeid.node_id));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        graph.is_unchecked_disconnected_from_node_id(data.isuncheckeddisconnectedfromnodeid.node_id)
    }
    

    19 => {
        trace.push(format!("is_unchecked_connected_from_node_id({:?})", data.isuncheckedconnectedfromnodeid.node_id));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        graph.is_unchecked_connected_from_node_id(data.isuncheckedconnectedfromnodeid.node_id)
    }
    

    20 => {
        trace.push(format!("is_singleton_from_node_id({:?})", data.issingletonfromnodeid.node_id));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.is_singleton_from_node_id(data.issingletonfromnodeid.node_id);
    }
    

    21 => {
        trace.push(format!("is_singleton_with_selfloops_from_node_id({:?})", data.issingletonwithselfloopsfromnodeid.node_id));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        graph.is_singleton_with_selfloops_from_node_id(data.issingletonwithselfloopsfromnodeid.node_id)
    }
    

    22 => {
        trace.push(format!("is_unchecked_singleton_from_node_name({:?})", data.isuncheckedsingletonfromnodename.node_name));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        graph.is_unchecked_singleton_from_node_name(data.isuncheckedsingletonfromnodename.node_name)
    }
    

    23 => {
        trace.push(format!("is_singleton_from_node_name({:?})", data.issingletonfromnodename.node_name));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.is_singleton_from_node_name(data.issingletonfromnodename.node_name);
    }
    

    24 => {
        trace.push(format!("has_node_name({:?})", data.hasnodename.node_name));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        graph.has_node_name(data.hasnodename.node_name)
    }
    

    25 => {
        trace.push(format!("has_node_type_id({:?})", data.hasnodetypeid.node_type_id));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        graph.has_node_type_id(data.hasnodetypeid.node_type_id)
    }
    

    26 => {
        trace.push(format!("has_node_type_name({:?})", data.hasnodetypename.node_type_name));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        graph.has_node_type_name(data.hasnodetypename.node_type_name)
    }
    

    27 => {
        trace.push(format!("has_edge_type_id({:?})", data.hasedgetypeid.edge_type_id));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        graph.has_edge_type_id(data.hasedgetypeid.edge_type_id)
    }
    

    28 => {
        trace.push(format!("has_edge_type_name({:?})", data.hasedgetypename.edge_type_name));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        graph.has_edge_type_name(data.hasedgetypename.edge_type_name)
    }
    

    29 => {
        trace.push(format!("has_edge_from_node_ids({:?}, {:?})", data.hasedgefromnodeids.src, data.hasedgefromnodeids.dst));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        graph.has_edge_from_node_ids(data.hasedgefromnodeids.src, data.hasedgefromnodeids.dst)
    }
    

    30 => {
        trace.push(format!("has_selfloop_from_node_id({:?})", data.hasselfloopfromnodeid.node_id));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        graph.has_selfloop_from_node_id(data.hasselfloopfromnodeid.node_id)
    }
    

    31 => {
        trace.push(format!("has_edge_from_node_ids_and_edge_type_id({:?}, {:?}, {:?})", data.hasedgefromnodeidsandedgetypeid.src, data.hasedgefromnodeidsandedgetypeid.dst, data.hasedgefromnodeidsandedgetypeid.edge_type));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        graph.has_edge_from_node_ids_and_edge_type_id(data.hasedgefromnodeidsandedgetypeid.src, data.hasedgefromnodeidsandedgetypeid.dst, data.hasedgefromnodeidsandedgetypeid.edge_type)
    }
    

    32 => {
        trace.push(format!("is_unchecked_trap_node_from_node_id({:?})", data.isuncheckedtrapnodefromnodeid.node_id));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        graph.is_unchecked_trap_node_from_node_id(data.isuncheckedtrapnodefromnodeid.node_id)
    }
    

    33 => {
        trace.push(format!("is_trap_node_from_node_id({:?})", data.istrapnodefromnodeid.node_id));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.is_trap_node_from_node_id(data.istrapnodefromnodeid.node_id);
    }
    

    34 => {
        trace.push(format!("has_node_name_and_node_type_name({:?}, {:?})", data.hasnodenameandnodetypename.node_name, data.hasnodenameandnodetypename.node_type_name));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        graph.has_node_name_and_node_type_name(data.hasnodenameandnodetypename.node_name, data.hasnodenameandnodetypename.node_type_name)
    }
    

    35 => {
        trace.push(format!("has_edge_from_node_names({:?}, {:?})", data.hasedgefromnodenames.src_name, data.hasedgefromnodenames.dst_name));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        graph.has_edge_from_node_names(data.hasedgefromnodenames.src_name, data.hasedgefromnodenames.dst_name)
    }
    

    36 => {
        trace.push(format!("has_edge_from_node_names_and_edge_type_name({:?}, {:?}, {:?})", data.hasedgefromnodenamesandedgetypename.src_name, data.hasedgefromnodenamesandedgetypename.dst_name, data.hasedgefromnodenamesandedgetypename.edge_type_name));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        graph.has_edge_from_node_names_and_edge_type_name(data.hasedgefromnodenamesandedgetypename.src_name, data.hasedgefromnodenamesandedgetypename.dst_name, data.hasedgefromnodenamesandedgetypename.edge_type_name)
    }
    

    37 => {
        trace.push(format!("get_unchecked_min_preferential_attachment()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        graph.get_unchecked_min_preferential_attachment()
    }
    

    38 => {
        trace.push(format!("get_unchecked_max_preferential_attachment()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        graph.get_unchecked_max_preferential_attachment()
    }
    

    39 => {
        trace.push(format!("get_unchecked_preferential_attachment({:?}, {:?}, {:?})", data.getuncheckedpreferentialattachment.one, data.getuncheckedpreferentialattachment.two, data.getuncheckedpreferentialattachment.normalize));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        graph.get_unchecked_preferential_attachment(data.getuncheckedpreferentialattachment.one, data.getuncheckedpreferentialattachment.two, data.getuncheckedpreferentialattachment.normalize)
    }
    

    40 => {
        trace.push(format!("get_preferential_attachment({:?}, {:?}, {:?})", data.getpreferentialattachment.one, data.getpreferentialattachment.two, data.getpreferentialattachment.normalize));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_preferential_attachment(data.getpreferentialattachment.one, data.getpreferentialattachment.two, data.getpreferentialattachment.normalize);
    }
    

    41 => {
        trace.push(format!("get_unchecked_jaccard_coefficient({:?}, {:?})", data.getuncheckedjaccardcoefficient.one, data.getuncheckedjaccardcoefficient.two));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        graph.get_unchecked_jaccard_coefficient(data.getuncheckedjaccardcoefficient.one, data.getuncheckedjaccardcoefficient.two)
    }
    

    42 => {
        trace.push(format!("get_jaccard_coefficient({:?}, {:?})", data.getjaccardcoefficient.one, data.getjaccardcoefficient.two));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_jaccard_coefficient(data.getjaccardcoefficient.one, data.getjaccardcoefficient.two);
    }
    

    43 => {
        trace.push(format!("get_unchecked_adamic_adar_index({:?}, {:?})", data.getuncheckedadamicadarindex.one, data.getuncheckedadamicadarindex.two));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        graph.get_unchecked_adamic_adar_index(data.getuncheckedadamicadarindex.one, data.getuncheckedadamicadarindex.two)
    }
    

    44 => {
        trace.push(format!("get_adamic_adar_index({:?}, {:?})", data.getadamicadarindex.one, data.getadamicadarindex.two));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_adamic_adar_index(data.getadamicadarindex.one, data.getadamicadarindex.two);
    }
    

    45 => {
        trace.push(format!("get_unchecked_resource_allocation_index({:?}, {:?})", data.getuncheckedresourceallocationindex.one, data.getuncheckedresourceallocationindex.two));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        graph.get_unchecked_resource_allocation_index(data.getuncheckedresourceallocationindex.one, data.getuncheckedresourceallocationindex.two)
    }
    

    46 => {
        trace.push(format!("get_resource_allocation_index({:?}, {:?})", data.getresourceallocationindex.one, data.getresourceallocationindex.two));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_resource_allocation_index(data.getresourceallocationindex.one, data.getresourceallocationindex.two);
    }
    

    47 => {
        trace.push(format!("enable({:?}, {:?}, {:?}, {:?})", data.enable.vector_sources, data.enable.vector_destinations, data.enable.vector_cumulative_node_degrees, data.enable.cache_size));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.enable(data.enable.vector_sources, data.enable.vector_destinations, data.enable.vector_cumulative_node_degrees, data.enable.cache_size);
    }
    

    48 => {
        trace.push(format!("disable_all()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        graph.disable_all()
    }
    

    49 => {
        trace.push(format!("get_unweighted_number_of_triangles({:?})", data.getunweightednumberoftriangles.normalize));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        graph.get_unweighted_number_of_triangles(data.getunweightednumberoftriangles.normalize)
    }
    

    50 => {
        trace.push(format!("get_unweighted_triads_number()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        graph.get_unweighted_triads_number()
    }
    

    51 => {
        trace.push(format!("get_weighted_triads_number()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_weighted_triads_number();
    }
    

    52 => {
        trace.push(format!("get_unweighted_transitivity()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        graph.get_unweighted_transitivity()
    }
    

    53 => {
        trace.push(format!("get_unweighted_number_of_triangles_per_node({:?})", data.getunweightednumberoftrianglespernode.normalize));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        graph.get_unweighted_number_of_triangles_per_node(data.getunweightednumberoftrianglespernode.normalize)
    }
    

    54 => {
        trace.push(format!("iter_clustering_coefficient_per_node()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.iter_clustering_coefficient_per_node().collect::<Vec<_>>();
            
    }
    

    55 => {
        trace.push(format!("get_clustering_coefficient_per_node()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        graph.get_clustering_coefficient_per_node()
    }
    

    56 => {
        trace.push(format!("get_clustering_coefficient()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        graph.get_clustering_coefficient()
    }
    

    57 => {
        trace.push(format!("get_average_clustering_coefficient()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        graph.get_average_clustering_coefficient()
    }
    

    58 => {
        trace.push(format!("get_unchecked_breath_first_search_from_node_ids({:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?})", data.getuncheckedbreathfirstsearchfromnodeids.src_node_id, data.getuncheckedbreathfirstsearchfromnodeids.maybe_dst_node_id, data.getuncheckedbreathfirstsearchfromnodeids.maybe_dst_node_ids, data.getuncheckedbreathfirstsearchfromnodeids.compute_distances, data.getuncheckedbreathfirstsearchfromnodeids.compute_predecessors, data.getuncheckedbreathfirstsearchfromnodeids.compute_visited, data.getuncheckedbreathfirstsearchfromnodeids.maximal_depth));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        graph.get_unchecked_breath_first_search_from_node_ids(data.getuncheckedbreathfirstsearchfromnodeids.src_node_id, data.getuncheckedbreathfirstsearchfromnodeids.maybe_dst_node_id, data.getuncheckedbreathfirstsearchfromnodeids.maybe_dst_node_ids, data.getuncheckedbreathfirstsearchfromnodeids.compute_distances, data.getuncheckedbreathfirstsearchfromnodeids.compute_predecessors, data.getuncheckedbreathfirstsearchfromnodeids.compute_visited, data.getuncheckedbreathfirstsearchfromnodeids.maximal_depth)
    }
    

    59 => {
        trace.push(format!("get_unchecked_unweighted_minimum_path_node_ids_from_node_ids({:?}, {:?}, {:?})", data.getuncheckedunweightedminimumpathnodeidsfromnodeids.src_node_id, data.getuncheckedunweightedminimumpathnodeidsfromnodeids.dst_node_id, data.getuncheckedunweightedminimumpathnodeidsfromnodeids.maximal_depth));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_unchecked_unweighted_minimum_path_node_ids_from_node_ids(data.getuncheckedunweightedminimumpathnodeidsfromnodeids.src_node_id, data.getuncheckedunweightedminimumpathnodeidsfromnodeids.dst_node_id, data.getuncheckedunweightedminimumpathnodeidsfromnodeids.maximal_depth);
    }
    

    60 => {
        trace.push(format!("get_unchecked_unweighted_minimum_path_node_names_from_node_ids({:?}, {:?}, {:?})", data.getuncheckedunweightedminimumpathnodenamesfromnodeids.src_node_id, data.getuncheckedunweightedminimumpathnodenamesfromnodeids.dst_node_id, data.getuncheckedunweightedminimumpathnodenamesfromnodeids.maximal_depth));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_unchecked_unweighted_minimum_path_node_names_from_node_ids(data.getuncheckedunweightedminimumpathnodenamesfromnodeids.src_node_id, data.getuncheckedunweightedminimumpathnodenamesfromnodeids.dst_node_id, data.getuncheckedunweightedminimumpathnodenamesfromnodeids.maximal_depth);
    }
    

    61 => {
        trace.push(format!("get_unweighted_minimum_path_node_ids_from_node_ids({:?}, {:?}, {:?})", data.getunweightedminimumpathnodeidsfromnodeids.src_node_id, data.getunweightedminimumpathnodeidsfromnodeids.dst_node_id, data.getunweightedminimumpathnodeidsfromnodeids.maximal_depth));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_unweighted_minimum_path_node_ids_from_node_ids(data.getunweightedminimumpathnodeidsfromnodeids.src_node_id, data.getunweightedminimumpathnodeidsfromnodeids.dst_node_id, data.getunweightedminimumpathnodeidsfromnodeids.maximal_depth);
    }
    

    62 => {
        trace.push(format!("get_unweighted_minimum_path_node_ids_from_node_names({:?}, {:?}, {:?})", data.getunweightedminimumpathnodeidsfromnodenames.src_node_name, data.getunweightedminimumpathnodeidsfromnodenames.dst_node_name, data.getunweightedminimumpathnodeidsfromnodenames.maximal_depth));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_unweighted_minimum_path_node_ids_from_node_names(data.getunweightedminimumpathnodeidsfromnodenames.src_node_name, data.getunweightedminimumpathnodeidsfromnodenames.dst_node_name, data.getunweightedminimumpathnodeidsfromnodenames.maximal_depth);
    }
    

    63 => {
        trace.push(format!("get_unweighted_minimum_path_node_names_from_node_names({:?}, {:?}, {:?})", data.getunweightedminimumpathnodenamesfromnodenames.src_node_name, data.getunweightedminimumpathnodenamesfromnodenames.dst_node_name, data.getunweightedminimumpathnodenamesfromnodenames.maximal_depth));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_unweighted_minimum_path_node_names_from_node_names(data.getunweightedminimumpathnodenamesfromnodenames.src_node_name, data.getunweightedminimumpathnodenamesfromnodenames.dst_node_name, data.getunweightedminimumpathnodenamesfromnodenames.maximal_depth);
    }
    

    64 => {
        trace.push(format!("get_unchecked_unweighted_k_shortest_path_node_ids_from_node_ids({:?}, {:?}, {:?})", data.getuncheckedunweightedkshortestpathnodeidsfromnodeids.src_node_id, data.getuncheckedunweightedkshortestpathnodeidsfromnodeids.dst_node_id, data.getuncheckedunweightedkshortestpathnodeidsfromnodeids.k));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        graph.get_unchecked_unweighted_k_shortest_path_node_ids_from_node_ids(data.getuncheckedunweightedkshortestpathnodeidsfromnodeids.src_node_id, data.getuncheckedunweightedkshortestpathnodeidsfromnodeids.dst_node_id, data.getuncheckedunweightedkshortestpathnodeidsfromnodeids.k)
    }
    

    65 => {
        trace.push(format!("get_unweighted_k_shortest_path_node_ids_from_node_ids({:?}, {:?}, {:?})", data.getunweightedkshortestpathnodeidsfromnodeids.src_node_id, data.getunweightedkshortestpathnodeidsfromnodeids.dst_node_id, data.getunweightedkshortestpathnodeidsfromnodeids.k));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_unweighted_k_shortest_path_node_ids_from_node_ids(data.getunweightedkshortestpathnodeidsfromnodeids.src_node_id, data.getunweightedkshortestpathnodeidsfromnodeids.dst_node_id, data.getunweightedkshortestpathnodeidsfromnodeids.k);
    }
    

    66 => {
        trace.push(format!("get_unweighted_k_shortest_path_node_ids_from_node_names({:?}, {:?}, {:?})", data.getunweightedkshortestpathnodeidsfromnodenames.src_node_name, data.getunweightedkshortestpathnodeidsfromnodenames.dst_node_name, data.getunweightedkshortestpathnodeidsfromnodenames.k));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_unweighted_k_shortest_path_node_ids_from_node_names(data.getunweightedkshortestpathnodeidsfromnodenames.src_node_name, data.getunweightedkshortestpathnodeidsfromnodenames.dst_node_name, data.getunweightedkshortestpathnodeidsfromnodenames.k);
    }
    

    67 => {
        trace.push(format!("get_unweighted_k_shortest_path_node_names_from_node_names({:?}, {:?}, {:?})", data.getunweightedkshortestpathnodenamesfromnodenames.src_node_name, data.getunweightedkshortestpathnodenamesfromnodenames.dst_node_name, data.getunweightedkshortestpathnodenamesfromnodenames.k));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_unweighted_k_shortest_path_node_names_from_node_names(data.getunweightedkshortestpathnodenamesfromnodenames.src_node_name, data.getunweightedkshortestpathnodenamesfromnodenames.dst_node_name, data.getunweightedkshortestpathnodenamesfromnodenames.k);
    }
    

    68 => {
        trace.push(format!("get_unchecked_unweighted_eccentricity_from_node_id({:?})", data.getuncheckedunweightedeccentricityfromnodeid.node_id));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        graph.get_unchecked_unweighted_eccentricity_from_node_id(data.getuncheckedunweightedeccentricityfromnodeid.node_id)
    }
    

    69 => {
        trace.push(format!("get_unchecked_weighted_eccentricity_from_node_id({:?}, {:?})", data.getuncheckedweightedeccentricityfromnodeid.node_id, data.getuncheckedweightedeccentricityfromnodeid.use_edge_weights_as_probabilities));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        graph.get_unchecked_weighted_eccentricity_from_node_id(data.getuncheckedweightedeccentricityfromnodeid.node_id, data.getuncheckedweightedeccentricityfromnodeid.use_edge_weights_as_probabilities)
    }
    

    70 => {
        trace.push(format!("get_unweighted_eccentricity_from_node_id({:?})", data.getunweightedeccentricityfromnodeid.node_id));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_unweighted_eccentricity_from_node_id(data.getunweightedeccentricityfromnodeid.node_id);
    }
    

    71 => {
        trace.push(format!("get_weighted_eccentricity_from_node_id({:?}, {:?})", data.getweightedeccentricityfromnodeid.node_id, data.getweightedeccentricityfromnodeid.use_edge_weights_as_probabilities));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_weighted_eccentricity_from_node_id(data.getweightedeccentricityfromnodeid.node_id, data.getweightedeccentricityfromnodeid.use_edge_weights_as_probabilities);
    }
    

    72 => {
        trace.push(format!("get_unweighted_eccentricity_from_node_name({:?})", data.getunweightedeccentricityfromnodename.node_name));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_unweighted_eccentricity_from_node_name(data.getunweightedeccentricityfromnodename.node_name);
    }
    

    73 => {
        trace.push(format!("get_weighted_eccentricity_from_node_name({:?}, {:?})", data.getweightedeccentricityfromnodename.node_name, data.getweightedeccentricityfromnodename.use_edge_weights_as_probabilities));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_weighted_eccentricity_from_node_name(data.getweightedeccentricityfromnodename.node_name, data.getweightedeccentricityfromnodename.use_edge_weights_as_probabilities);
    }
    

    74 => {
        trace.push(format!("get_unchecked_dijkstra_from_node_ids({:?}, {:?}, {:?}, {:?}, {:?}, {:?})", data.getuncheckeddijkstrafromnodeids.src_node_id, data.getuncheckeddijkstrafromnodeids.maybe_dst_node_id, data.getuncheckeddijkstrafromnodeids.maybe_dst_node_ids, data.getuncheckeddijkstrafromnodeids.compute_predecessors, data.getuncheckeddijkstrafromnodeids.maximal_depth, data.getuncheckeddijkstrafromnodeids.use_edge_weights_as_probabilities));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        graph.get_unchecked_dijkstra_from_node_ids(data.getuncheckeddijkstrafromnodeids.src_node_id, data.getuncheckeddijkstrafromnodeids.maybe_dst_node_id, data.getuncheckeddijkstrafromnodeids.maybe_dst_node_ids, data.getuncheckeddijkstrafromnodeids.compute_predecessors, data.getuncheckeddijkstrafromnodeids.maximal_depth, data.getuncheckeddijkstrafromnodeids.use_edge_weights_as_probabilities)
    }
    

    75 => {
        trace.push(format!("get_unchecked_weighted_minimum_path_node_ids_from_node_ids({:?}, {:?}, {:?}, {:?})", data.getuncheckedweightedminimumpathnodeidsfromnodeids.src_node_id, data.getuncheckedweightedminimumpathnodeidsfromnodeids.dst_node_id, data.getuncheckedweightedminimumpathnodeidsfromnodeids.use_edge_weights_as_probabilities, data.getuncheckedweightedminimumpathnodeidsfromnodeids.maximal_depth));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        graph.get_unchecked_weighted_minimum_path_node_ids_from_node_ids(data.getuncheckedweightedminimumpathnodeidsfromnodeids.src_node_id, data.getuncheckedweightedminimumpathnodeidsfromnodeids.dst_node_id, data.getuncheckedweightedminimumpathnodeidsfromnodeids.use_edge_weights_as_probabilities, data.getuncheckedweightedminimumpathnodeidsfromnodeids.maximal_depth)
    }
    

    76 => {
        trace.push(format!("get_unchecked_weighted_minimum_path_node_names_from_node_ids({:?}, {:?}, {:?}, {:?})", data.getuncheckedweightedminimumpathnodenamesfromnodeids.src_node_id, data.getuncheckedweightedminimumpathnodenamesfromnodeids.dst_node_id, data.getuncheckedweightedminimumpathnodenamesfromnodeids.use_edge_weights_as_probabilities, data.getuncheckedweightedminimumpathnodenamesfromnodeids.maximal_depth));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        graph.get_unchecked_weighted_minimum_path_node_names_from_node_ids(data.getuncheckedweightedminimumpathnodenamesfromnodeids.src_node_id, data.getuncheckedweightedminimumpathnodenamesfromnodeids.dst_node_id, data.getuncheckedweightedminimumpathnodenamesfromnodeids.use_edge_weights_as_probabilities, data.getuncheckedweightedminimumpathnodenamesfromnodeids.maximal_depth)
    }
    

    77 => {
        trace.push(format!("get_weighted_minimum_path_node_ids_from_node_ids({:?}, {:?}, {:?}, {:?})", data.getweightedminimumpathnodeidsfromnodeids.src_node_id, data.getweightedminimumpathnodeidsfromnodeids.dst_node_id, data.getweightedminimumpathnodeidsfromnodeids.use_edge_weights_as_probabilities, data.getweightedminimumpathnodeidsfromnodeids.maximal_depth));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_weighted_minimum_path_node_ids_from_node_ids(data.getweightedminimumpathnodeidsfromnodeids.src_node_id, data.getweightedminimumpathnodeidsfromnodeids.dst_node_id, data.getweightedminimumpathnodeidsfromnodeids.use_edge_weights_as_probabilities, data.getweightedminimumpathnodeidsfromnodeids.maximal_depth);
    }
    

    78 => {
        trace.push(format!("get_weighted_minimum_path_node_ids_from_node_names({:?}, {:?}, {:?}, {:?})", data.getweightedminimumpathnodeidsfromnodenames.src_node_name, data.getweightedminimumpathnodeidsfromnodenames.dst_node_name, data.getweightedminimumpathnodeidsfromnodenames.use_edge_weights_as_probabilities, data.getweightedminimumpathnodeidsfromnodenames.maximal_depth));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_weighted_minimum_path_node_ids_from_node_names(data.getweightedminimumpathnodeidsfromnodenames.src_node_name, data.getweightedminimumpathnodeidsfromnodenames.dst_node_name, data.getweightedminimumpathnodeidsfromnodenames.use_edge_weights_as_probabilities, data.getweightedminimumpathnodeidsfromnodenames.maximal_depth);
    }
    

    79 => {
        trace.push(format!("get_weighted_minimum_path_node_names_from_node_names({:?}, {:?}, {:?}, {:?})", data.getweightedminimumpathnodenamesfromnodenames.src_node_name, data.getweightedminimumpathnodenamesfromnodenames.dst_node_name, data.getweightedminimumpathnodenamesfromnodenames.use_edge_weights_as_probabilities, data.getweightedminimumpathnodenamesfromnodenames.maximal_depth));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_weighted_minimum_path_node_names_from_node_names(data.getweightedminimumpathnodenamesfromnodenames.src_node_name, data.getweightedminimumpathnodenamesfromnodenames.dst_node_name, data.getweightedminimumpathnodenamesfromnodenames.use_edge_weights_as_probabilities, data.getweightedminimumpathnodenamesfromnodenames.maximal_depth);
    }
    

    80 => {
        trace.push(format!("get_breath_first_search_from_node_ids({:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?})", data.getbreathfirstsearchfromnodeids.src_node_id, data.getbreathfirstsearchfromnodeids.maybe_dst_node_id, data.getbreathfirstsearchfromnodeids.maybe_dst_node_ids, data.getbreathfirstsearchfromnodeids.compute_distances, data.getbreathfirstsearchfromnodeids.compute_predecessors, data.getbreathfirstsearchfromnodeids.compute_visited, data.getbreathfirstsearchfromnodeids.maximal_depth));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_breath_first_search_from_node_ids(data.getbreathfirstsearchfromnodeids.src_node_id, data.getbreathfirstsearchfromnodeids.maybe_dst_node_id, data.getbreathfirstsearchfromnodeids.maybe_dst_node_ids, data.getbreathfirstsearchfromnodeids.compute_distances, data.getbreathfirstsearchfromnodeids.compute_predecessors, data.getbreathfirstsearchfromnodeids.compute_visited, data.getbreathfirstsearchfromnodeids.maximal_depth);
    }
    

    81 => {
        trace.push(format!("get_dijkstra_from_node_ids({:?}, {:?}, {:?}, {:?}, {:?}, {:?})", data.getdijkstrafromnodeids.src_node_id, data.getdijkstrafromnodeids.maybe_dst_node_id, data.getdijkstrafromnodeids.maybe_dst_node_ids, data.getdijkstrafromnodeids.compute_predecessors, data.getdijkstrafromnodeids.maximal_depth, data.getdijkstrafromnodeids.use_edge_weights_as_probabilities));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_dijkstra_from_node_ids(data.getdijkstrafromnodeids.src_node_id, data.getdijkstrafromnodeids.maybe_dst_node_id, data.getdijkstrafromnodeids.maybe_dst_node_ids, data.getdijkstrafromnodeids.compute_predecessors, data.getdijkstrafromnodeids.maximal_depth, data.getdijkstrafromnodeids.use_edge_weights_as_probabilities);
    }
    

    82 => {
        trace.push(format!("get_unweighted_diameter({:?}, {:?})", data.getunweighteddiameter.ignore_infinity, data.getunweighteddiameter.verbose));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_unweighted_diameter(data.getunweighteddiameter.ignore_infinity, data.getunweighteddiameter.verbose);
    }
    

    83 => {
        trace.push(format!("get_weighted_diameter({:?}, {:?}, {:?})", data.getweighteddiameter.ignore_infinity, data.getweighteddiameter.use_edge_weights_as_probabilities, data.getweighteddiameter.verbose));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_weighted_diameter(data.getweighteddiameter.ignore_infinity, data.getweighteddiameter.use_edge_weights_as_probabilities, data.getweighteddiameter.verbose);
    }
    

    84 => {
        trace.push(format!("get_breath_first_search_from_node_names({:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?})", data.getbreathfirstsearchfromnodenames.src_node_name, data.getbreathfirstsearchfromnodenames.maybe_dst_node_name, data.getbreathfirstsearchfromnodenames.maybe_dst_node_names, data.getbreathfirstsearchfromnodenames.compute_distances, data.getbreathfirstsearchfromnodenames.compute_predecessors, data.getbreathfirstsearchfromnodenames.compute_visited, data.getbreathfirstsearchfromnodenames.maximal_depth));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_breath_first_search_from_node_names(data.getbreathfirstsearchfromnodenames.src_node_name, data.getbreathfirstsearchfromnodenames.maybe_dst_node_name, data.getbreathfirstsearchfromnodenames.maybe_dst_node_names, data.getbreathfirstsearchfromnodenames.compute_distances, data.getbreathfirstsearchfromnodenames.compute_predecessors, data.getbreathfirstsearchfromnodenames.compute_visited, data.getbreathfirstsearchfromnodenames.maximal_depth);
    }
    

    85 => {
        trace.push(format!("get_dijkstra_from_node_names({:?}, {:?}, {:?}, {:?}, {:?}, {:?})", data.getdijkstrafromnodenames.src_node_name, data.getdijkstrafromnodenames.maybe_dst_node_name, data.getdijkstrafromnodenames.maybe_dst_node_names, data.getdijkstrafromnodenames.compute_predecessors, data.getdijkstrafromnodenames.maximal_depth, data.getdijkstrafromnodenames.use_edge_weights_as_probabilities));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_dijkstra_from_node_names(data.getdijkstrafromnodenames.src_node_name, data.getdijkstrafromnodenames.maybe_dst_node_name, data.getdijkstrafromnodenames.maybe_dst_node_names, data.getdijkstrafromnodenames.compute_predecessors, data.getdijkstrafromnodenames.maximal_depth, data.getdijkstrafromnodenames.use_edge_weights_as_probabilities);
    }
    

    86 => {
        trace.push(format!("iter_unchecked_edge_ids_from_source_node_id({:?})", data.iteruncheckededgeidsfromsourcenodeid.src));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        graph.iter_unchecked_edge_ids_from_source_node_id(data.iteruncheckededgeidsfromsourcenodeid.src)
    }
    

    87 => {
        trace.push(format!("iter_unchecked_edge_weights_from_source_node_id({:?})", data.iteruncheckededgeweightsfromsourcenodeid.source_node_id));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.iter_unchecked_edge_weights_from_source_node_id(data.iteruncheckededgeweightsfromsourcenodeid.source_node_id).collect::<Vec<_>>();
            
    }
    

    88 => {
        trace.push(format!("par_iter_unchecked_edge_ids_from_source_node_id({:?})", data.pariteruncheckededgeidsfromsourcenodeid.src));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.par_iter_unchecked_edge_ids_from_source_node_id(data.pariteruncheckededgeidsfromsourcenodeid.src).collect::<Vec<_>>();
            
    }
    

    89 => {
        trace.push(format!("iter_unchecked_edge_ids_from_node_ids({:?}, {:?})", data.iteruncheckededgeidsfromnodeids.src, data.iteruncheckededgeidsfromnodeids.dst));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.iter_unchecked_edge_ids_from_node_ids(data.iteruncheckededgeidsfromnodeids.src, data.iteruncheckededgeidsfromnodeids.dst).collect::<Vec<_>>();
            
    }
    

    90 => {
        trace.push(format!("iter_unchecked_neighbour_node_ids_from_source_node_id({:?})", data.iteruncheckedneighbournodeidsfromsourcenodeid.src));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.iter_unchecked_neighbour_node_ids_from_source_node_id(data.iteruncheckedneighbournodeidsfromsourcenodeid.src).collect::<Vec<_>>();
            
    }
    

    91 => {
        trace.push(format!("iter_unchecked_neighbour_node_ids_intersection_from_source_node_ids({:?}, {:?})", data.iteruncheckedneighbournodeidsintersectionfromsourcenodeids.first_src_node_id, data.iteruncheckedneighbournodeidsintersectionfromsourcenodeids.second_src_node_id));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.iter_unchecked_neighbour_node_ids_intersection_from_source_node_ids(data.iteruncheckedneighbournodeidsintersectionfromsourcenodeids.first_src_node_id, data.iteruncheckedneighbournodeidsintersectionfromsourcenodeids.second_src_node_id).collect::<Vec<_>>();
            
    }
    

    92 => {
        trace.push(format!("iter_unchecked_neighbour_node_ids_union_from_source_node_ids({:?}, {:?})", data.iteruncheckedneighbournodeidsunionfromsourcenodeids.first_src_node_id, data.iteruncheckedneighbournodeidsunionfromsourcenodeids.second_src_node_id));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.iter_unchecked_neighbour_node_ids_union_from_source_node_ids(data.iteruncheckedneighbournodeidsunionfromsourcenodeids.first_src_node_id, data.iteruncheckedneighbournodeidsunionfromsourcenodeids.second_src_node_id).collect::<Vec<_>>();
            
    }
    

    93 => {
        trace.push(format!("iter_unchecked_neighbour_node_ids_difference_from_source_node_ids({:?}, {:?})", data.iteruncheckedneighbournodeidsdifferencefromsourcenodeids.first_src_node_id, data.iteruncheckedneighbournodeidsdifferencefromsourcenodeids.second_src_node_id));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.iter_unchecked_neighbour_node_ids_difference_from_source_node_ids(data.iteruncheckedneighbournodeidsdifferencefromsourcenodeids.first_src_node_id, data.iteruncheckedneighbournodeidsdifferencefromsourcenodeids.second_src_node_id).collect::<Vec<_>>();
            
    }
    

    94 => {
        trace.push(format!("iter_unchecked_neighbour_node_names_from_source_node_id({:?})", data.iteruncheckedneighbournodenamesfromsourcenodeid.src));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.iter_unchecked_neighbour_node_names_from_source_node_id(data.iteruncheckedneighbournodenamesfromsourcenodeid.src).collect::<Vec<_>>();
            
    }
    

    95 => {
        trace.push(format!("iter_edge_ids_from_node_ids({:?}, {:?})", data.iteredgeidsfromnodeids.src, data.iteredgeidsfromnodeids.dst));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.iter_edge_ids_from_node_ids(data.iteredgeidsfromnodeids.src, data.iteredgeidsfromnodeids.dst).map(|x| x.collect::<Vec<_>>());
            
    }
    

    96 => {
        trace.push(format!("iter_edge_node_ids_and_edge_type_id_from_edge_type_id({:?}, {:?})", data.iteredgenodeidsandedgetypeidfromedgetypeid.edge_type_id, data.iteredgenodeidsandedgetypeidfromedgetypeid.directed));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.iter_edge_node_ids_and_edge_type_id_from_edge_type_id(data.iteredgenodeidsandedgetypeidfromedgetypeid.edge_type_id, data.iteredgenodeidsandedgetypeidfromedgetypeid.directed).map(|x| x.collect::<Vec<_>>());
            
    }
    

    97 => {
        trace.push(format!("iter_node_ids_and_node_type_ids_from_node_type_id({:?})", data.iternodeidsandnodetypeidsfromnodetypeid.node_type_id));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.iter_node_ids_and_node_type_ids_from_node_type_id(data.iternodeidsandnodetypeidsfromnodetypeid.node_type_id).map(|x| x.collect::<Vec<_>>());
            
    }
    

    98 => {
        trace.push(format!("iter_node_names_and_node_type_names_from_node_type_id({:?})", data.iternodenamesandnodetypenamesfromnodetypeid.node_type_id));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        graph.iter_node_names_and_node_type_names_from_node_type_id(data.iternodenamesandnodetypenamesfromnodetypeid.node_type_id)
    }
    

    99 => {
        trace.push(format!("iter_edge_node_names_and_edge_type_name_from_edge_type_id({:?}, {:?})", data.iteredgenodenamesandedgetypenamefromedgetypeid.edge_type_id, data.iteredgenodenamesandedgetypenamefromedgetypeid.directed));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        graph.iter_edge_node_names_and_edge_type_name_from_edge_type_id(data.iteredgenodenamesandedgetypenamefromedgetypeid.edge_type_id, data.iteredgenodenamesandedgetypenamefromedgetypeid.directed)
    }
    

    100 => {
        trace.push(format!("get_transitive_closure({:?}, {:?})", data.gettransitiveclosure.iterations, data.gettransitiveclosure.verbose));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        graph = graph.get_transitive_closure(data.gettransitiveclosure.iterations, data.gettransitiveclosure.verbose);
    }
    

    101 => {
        trace.push(format!("get_unweighted_all_shortest_paths({:?}, {:?})", data.getunweightedallshortestpaths.iterations, data.getunweightedallshortestpaths.verbose));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        graph = graph.get_unweighted_all_shortest_paths(data.getunweightedallshortestpaths.iterations, data.getunweightedallshortestpaths.verbose);
    }
    

    102 => {
        trace.push(format!("get_weighted_all_shortest_paths({:?}, {:?}, {:?})", data.getweightedallshortestpaths.iterations, data.getweightedallshortestpaths.use_edge_weights_as_probabilities, data.getweightedallshortestpaths.verbose));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
        if let Ok(res) = graph.get_weighted_all_shortest_paths(data.getweightedallshortestpaths.iterations, data.getweightedallshortestpaths.use_edge_weights_as_probabilities, data.getweightedallshortestpaths.verbose) {
            graph = res;
        }
        
    }
    

    103 => {
        trace.push(format!("strongly_connected_components()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        graph.strongly_connected_components()
    }
    

    104 => {
        trace.push(format!("get_unchecked_edge_weight_from_edge_id({:?})", data.getuncheckededgeweightfromedgeid.edge_id));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        graph.get_unchecked_edge_weight_from_edge_id(data.getuncheckededgeweightfromedgeid.edge_id)
    }
    

    105 => {
        trace.push(format!("get_unchecked_edge_weight_from_node_ids({:?}, {:?})", data.getuncheckededgeweightfromnodeids.src, data.getuncheckededgeweightfromnodeids.dst));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        graph.get_unchecked_edge_weight_from_node_ids(data.getuncheckededgeweightfromnodeids.src, data.getuncheckededgeweightfromnodeids.dst)
    }
    

    106 => {
        trace.push(format!("get_unchecked_node_id_from_node_name({:?})", data.getuncheckednodeidfromnodename.node_name));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        graph.get_unchecked_node_id_from_node_name(data.getuncheckednodeidfromnodename.node_name)
    }
    

    107 => {
        trace.push(format!("get_unchecked_edge_type_id_from_edge_type_name({:?})", data.getuncheckededgetypeidfromedgetypename.edge_type_name));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        graph.get_unchecked_edge_type_id_from_edge_type_name(data.getuncheckededgetypeidfromedgetypename.edge_type_name)
    }
    

    108 => {
        trace.push(format!("get_unchecked_edge_type_name_from_edge_type_id({:?})", data.getuncheckededgetypenamefromedgetypeid.edge_type_id));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        graph.get_unchecked_edge_type_name_from_edge_type_id(data.getuncheckededgetypenamefromedgetypeid.edge_type_id)
    }
    

    109 => {
        trace.push(format!("get_unchecked_edge_count_from_edge_type_id({:?})", data.getuncheckededgecountfromedgetypeid.edge_type));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        graph.get_unchecked_edge_count_from_edge_type_id(data.getuncheckededgecountfromedgetypeid.edge_type)
    }
    

    110 => {
        trace.push(format!("get_unchecked_edge_id_from_node_ids_and_edge_type_id({:?}, {:?}, {:?})", data.getuncheckededgeidfromnodeidsandedgetypeid.src, data.getuncheckededgeidfromnodeidsandedgetypeid.dst, data.getuncheckededgeidfromnodeidsandedgetypeid.edge_type));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        graph.get_unchecked_edge_id_from_node_ids_and_edge_type_id(data.getuncheckededgeidfromnodeidsandedgetypeid.src, data.getuncheckededgeidfromnodeidsandedgetypeid.dst, data.getuncheckededgeidfromnodeidsandedgetypeid.edge_type)
    }
    

    111 => {
        trace.push(format!("get_unchecked_minmax_edge_ids_from_node_ids({:?}, {:?})", data.getuncheckedminmaxedgeidsfromnodeids.src, data.getuncheckedminmaxedgeidsfromnodeids.dst));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        graph.get_unchecked_minmax_edge_ids_from_node_ids(data.getuncheckedminmaxedgeidsfromnodeids.src, data.getuncheckedminmaxedgeidsfromnodeids.dst)
    }
    

    112 => {
        trace.push(format!("get_unchecked_node_ids_from_edge_id({:?})", data.getuncheckednodeidsfromedgeid.edge_id));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        graph.get_unchecked_node_ids_from_edge_id(data.getuncheckednodeidsfromedgeid.edge_id)
    }
    

    113 => {
        trace.push(format!("get_node_ids_from_edge_id({:?})", data.getnodeidsfromedgeid.edge_id));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_node_ids_from_edge_id(data.getnodeidsfromedgeid.edge_id);
    }
    

    114 => {
        trace.push(format!("get_unchecked_edge_id_from_node_ids({:?}, {:?})", data.getuncheckededgeidfromnodeids.src, data.getuncheckededgeidfromnodeids.dst));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        graph.get_unchecked_edge_id_from_node_ids(data.getuncheckededgeidfromnodeids.src, data.getuncheckededgeidfromnodeids.dst)
    }
    

    115 => {
        trace.push(format!("get_edge_id_from_node_ids({:?}, {:?})", data.getedgeidfromnodeids.src, data.getedgeidfromnodeids.dst));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_edge_id_from_node_ids(data.getedgeidfromnodeids.src, data.getedgeidfromnodeids.dst);
    }
    

    116 => {
        trace.push(format!("get_unchecked_unique_source_node_id({:?})", data.getuncheckeduniquesourcenodeid.source_id));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        graph.get_unchecked_unique_source_node_id(data.getuncheckeduniquesourcenodeid.source_id)
    }
    

    117 => {
        trace.push(format!("get_unchecked_node_ids_and_edge_type_id_from_edge_id({:?})", data.getuncheckednodeidsandedgetypeidfromedgeid.edge_id));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        graph.get_unchecked_node_ids_and_edge_type_id_from_edge_id(data.getuncheckednodeidsandedgetypeidfromedgeid.edge_id)
    }
    

    118 => {
        trace.push(format!("get_node_ids_and_edge_type_id_from_edge_id({:?})", data.getnodeidsandedgetypeidfromedgeid.edge_id));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_node_ids_and_edge_type_id_from_edge_id(data.getnodeidsandedgetypeidfromedgeid.edge_id);
    }
    

    119 => {
        trace.push(format!("get_unchecked_node_ids_and_edge_type_id_and_edge_weight_from_edge_id({:?})", data.getuncheckednodeidsandedgetypeidandedgeweightfromedgeid.edge_id));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        graph.get_unchecked_node_ids_and_edge_type_id_and_edge_weight_from_edge_id(data.getuncheckednodeidsandedgetypeidandedgeweightfromedgeid.edge_id)
    }
    

    120 => {
        trace.push(format!("get_node_ids_and_edge_type_id_and_edge_weight_from_edge_id({:?})", data.getnodeidsandedgetypeidandedgeweightfromedgeid.edge_id));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_node_ids_and_edge_type_id_and_edge_weight_from_edge_id(data.getnodeidsandedgetypeidandedgeweightfromedgeid.edge_id);
    }
    

    121 => {
        trace.push(format!("get_top_k_central_node_ids({:?})", data.gettopkcentralnodeids.k));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        graph.get_top_k_central_node_ids(data.gettopkcentralnodeids.k)
    }
    

    122 => {
        trace.push(format!("get_unchecked_unweighted_node_degree_from_node_id({:?})", data.getuncheckedunweightednodedegreefromnodeid.node_id));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        graph.get_unchecked_unweighted_node_degree_from_node_id(data.getuncheckedunweightednodedegreefromnodeid.node_id)
    }
    

    123 => {
        trace.push(format!("get_unchecked_weighted_node_degree_from_node_id({:?})", data.getuncheckedweightednodedegreefromnodeid.node_id));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        graph.get_unchecked_weighted_node_degree_from_node_id(data.getuncheckedweightednodedegreefromnodeid.node_id)
    }
    

    124 => {
        trace.push(format!("get_unweighted_node_degree_from_node_id({:?})", data.getunweightednodedegreefromnodeid.node_id));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_unweighted_node_degree_from_node_id(data.getunweightednodedegreefromnodeid.node_id);
    }
    

    125 => {
        trace.push(format!("get_weighted_node_degree_from_node_id({:?})", data.getweightednodedegreefromnodeid.node_id));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_weighted_node_degree_from_node_id(data.getweightednodedegreefromnodeid.node_id);
    }
    

    126 => {
        trace.push(format!("get_node_degree_from_node_name({:?})", data.getnodedegreefromnodename.node_name));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_node_degree_from_node_name(data.getnodedegreefromnodename.node_name);
    }
    

    127 => {
        trace.push(format!("get_top_k_central_node_names({:?})", data.gettopkcentralnodenames.k));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        graph.get_top_k_central_node_names(data.gettopkcentralnodenames.k)
    }
    

    128 => {
        trace.push(format!("get_unchecked_node_type_id_from_node_id({:?})", data.getuncheckednodetypeidfromnodeid.node_id));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        graph.get_unchecked_node_type_id_from_node_id(data.getuncheckednodetypeidfromnodeid.node_id)
    }
    

    129 => {
        trace.push(format!("get_node_type_id_from_node_id({:?})", data.getnodetypeidfromnodeid.node_id));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_node_type_id_from_node_id(data.getnodetypeidfromnodeid.node_id);
    }
    

    130 => {
        trace.push(format!("get_unchecked_edge_type_id_from_edge_id({:?})", data.getuncheckededgetypeidfromedgeid.edge_id));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        graph.get_unchecked_edge_type_id_from_edge_id(data.getuncheckededgetypeidfromedgeid.edge_id)
    }
    

    131 => {
        trace.push(format!("get_edge_type_id_from_edge_id({:?})", data.getedgetypeidfromedgeid.edge_id));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_edge_type_id_from_edge_id(data.getedgetypeidfromedgeid.edge_id);
    }
    

    132 => {
        trace.push(format!("get_unchecked_node_type_names_from_node_id({:?})", data.getuncheckednodetypenamesfromnodeid.node_id));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        graph.get_unchecked_node_type_names_from_node_id(data.getuncheckednodetypenamesfromnodeid.node_id)
    }
    

    133 => {
        trace.push(format!("get_node_type_names_from_node_id({:?})", data.getnodetypenamesfromnodeid.node_id));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_node_type_names_from_node_id(data.getnodetypenamesfromnodeid.node_id);
    }
    

    134 => {
        trace.push(format!("get_node_type_names_from_node_name({:?})", data.getnodetypenamesfromnodename.node_name));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_node_type_names_from_node_name(data.getnodetypenamesfromnodename.node_name);
    }
    

    135 => {
        trace.push(format!("get_edge_type_name_from_edge_id({:?})", data.getedgetypenamefromedgeid.edge_id));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_edge_type_name_from_edge_id(data.getedgetypenamefromedgeid.edge_id);
    }
    

    136 => {
        trace.push(format!("get_edge_type_name_from_edge_type_id({:?})", data.getedgetypenamefromedgetypeid.edge_type_id));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_edge_type_name_from_edge_type_id(data.getedgetypenamefromedgetypeid.edge_type_id);
    }
    

    137 => {
        trace.push(format!("get_edge_weight_from_edge_id({:?})", data.getedgeweightfromedgeid.edge_id));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_edge_weight_from_edge_id(data.getedgeweightfromedgeid.edge_id);
    }
    

    138 => {
        trace.push(format!("get_edge_weight_from_node_ids({:?}, {:?})", data.getedgeweightfromnodeids.src, data.getedgeweightfromnodeids.dst));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_edge_weight_from_node_ids(data.getedgeweightfromnodeids.src, data.getedgeweightfromnodeids.dst);
    }
    

    139 => {
        trace.push(format!("get_edge_weight_from_node_ids_and_edge_type_id({:?}, {:?}, {:?})", data.getedgeweightfromnodeidsandedgetypeid.src, data.getedgeweightfromnodeidsandedgetypeid.dst, data.getedgeweightfromnodeidsandedgetypeid.edge_type));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_edge_weight_from_node_ids_and_edge_type_id(data.getedgeweightfromnodeidsandedgetypeid.src, data.getedgeweightfromnodeidsandedgetypeid.dst, data.getedgeweightfromnodeidsandedgetypeid.edge_type);
    }
    

    140 => {
        trace.push(format!("get_edge_weight_from_node_names_and_edge_type_name({:?}, {:?}, {:?})", data.getedgeweightfromnodenamesandedgetypename.src, data.getedgeweightfromnodenamesandedgetypename.dst, data.getedgeweightfromnodenamesandedgetypename.edge_type));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_edge_weight_from_node_names_and_edge_type_name(data.getedgeweightfromnodenamesandedgetypename.src, data.getedgeweightfromnodenamesandedgetypename.dst, data.getedgeweightfromnodenamesandedgetypename.edge_type);
    }
    

    141 => {
        trace.push(format!("get_edge_weight_from_node_names({:?}, {:?})", data.getedgeweightfromnodenames.src_name, data.getedgeweightfromnodenames.dst_name));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_edge_weight_from_node_names(data.getedgeweightfromnodenames.src_name, data.getedgeweightfromnodenames.dst_name);
    }
    

    142 => {
        trace.push(format!("get_unchecked_node_name_from_node_id({:?})", data.getuncheckednodenamefromnodeid.node_id));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        graph.get_unchecked_node_name_from_node_id(data.getuncheckednodenamefromnodeid.node_id)
    }
    

    143 => {
        trace.push(format!("get_node_name_from_node_id({:?})", data.getnodenamefromnodeid.node_id));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_node_name_from_node_id(data.getnodenamefromnodeid.node_id);
    }
    

    144 => {
        trace.push(format!("get_node_id_from_node_name({:?})", data.getnodeidfromnodename.node_name));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_node_id_from_node_name(data.getnodeidfromnodename.node_name);
    }
    

    145 => {
        trace.push(format!("get_node_ids_from_node_names({:?})", data.getnodeidsfromnodenames.node_names));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_node_ids_from_node_names(data.getnodeidsfromnodenames.node_names);
    }
    

    146 => {
        trace.push(format!("get_edge_node_ids_from_edge_node_names({:?})", data.getedgenodeidsfromedgenodenames.edge_node_names));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_edge_node_ids_from_edge_node_names(data.getedgenodeidsfromedgenodenames.edge_node_names);
    }
    

    147 => {
        trace.push(format!("get_edge_node_names_from_edge_node_ids({:?})", data.getedgenodenamesfromedgenodeids.edge_node_ids));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_edge_node_names_from_edge_node_ids(data.getedgenodenamesfromedgenodeids.edge_node_ids);
    }
    

    148 => {
        trace.push(format!("get_node_type_id_from_node_name({:?})", data.getnodetypeidfromnodename.node_name));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_node_type_id_from_node_name(data.getnodetypeidfromnodename.node_name);
    }
    

    149 => {
        trace.push(format!("get_node_type_name_from_node_name({:?})", data.getnodetypenamefromnodename.node_name));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_node_type_name_from_node_name(data.getnodetypenamefromnodename.node_name);
    }
    

    150 => {
        trace.push(format!("get_edge_count_from_edge_type_id({:?})", data.getedgecountfromedgetypeid.edge_type_id));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_edge_count_from_edge_type_id(data.getedgecountfromedgetypeid.edge_type_id);
    }
    

    151 => {
        trace.push(format!("get_edge_type_id_from_edge_type_name({:?})", data.getedgetypeidfromedgetypename.edge_type_name));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_edge_type_id_from_edge_type_name(data.getedgetypeidfromedgetypename.edge_type_name);
    }
    

    152 => {
        trace.push(format!("get_edge_count_from_edge_type_name({:?})", data.getedgecountfromedgetypename.edge_type_name));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_edge_count_from_edge_type_name(data.getedgecountfromedgetypename.edge_type_name);
    }
    

    153 => {
        trace.push(format!("get_node_type_id_from_node_type_name({:?})", data.getnodetypeidfromnodetypename.node_type_name));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_node_type_id_from_node_type_name(data.getnodetypeidfromnodetypename.node_type_name);
    }
    

    154 => {
        trace.push(format!("get_node_count_from_node_type_id({:?})", data.getnodecountfromnodetypeid.node_type_id));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_node_count_from_node_type_id(data.getnodecountfromnodetypeid.node_type_id);
    }
    

    155 => {
        trace.push(format!("get_node_count_from_node_type_name({:?})", data.getnodecountfromnodetypename.node_type_name));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_node_count_from_node_type_name(data.getnodecountfromnodetypename.node_type_name);
    }
    

    156 => {
        trace.push(format!("get_destination_node_id_from_edge_id({:?})", data.getdestinationnodeidfromedgeid.edge_id));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_destination_node_id_from_edge_id(data.getdestinationnodeidfromedgeid.edge_id);
    }
    

    157 => {
        trace.push(format!("get_neighbour_node_ids_from_node_id({:?})", data.getneighbournodeidsfromnodeid.node_id));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_neighbour_node_ids_from_node_id(data.getneighbournodeidsfromnodeid.node_id);
    }
    

    158 => {
        trace.push(format!("get_neighbour_node_ids_from_node_name({:?})", data.getneighbournodeidsfromnodename.node_name));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_neighbour_node_ids_from_node_name(data.getneighbournodeidsfromnodename.node_name);
    }
    

    159 => {
        trace.push(format!("get_neighbour_node_names_from_node_name({:?})", data.getneighbournodenamesfromnodename.node_name));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_neighbour_node_names_from_node_name(data.getneighbournodenamesfromnodename.node_name);
    }
    

    160 => {
        trace.push(format!("get_minmax_edge_ids_from_node_ids({:?}, {:?})", data.getminmaxedgeidsfromnodeids.src, data.getminmaxedgeidsfromnodeids.dst));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_minmax_edge_ids_from_node_ids(data.getminmaxedgeidsfromnodeids.src, data.getminmaxedgeidsfromnodeids.dst);
    }
    

    161 => {
        trace.push(format!("get_edge_id_from_node_ids_and_edge_type_id({:?}, {:?}, {:?})", data.getedgeidfromnodeidsandedgetypeid.src, data.getedgeidfromnodeidsandedgetypeid.dst, data.getedgeidfromnodeidsandedgetypeid.edge_type));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_edge_id_from_node_ids_and_edge_type_id(data.getedgeidfromnodeidsandedgetypeid.src, data.getedgeidfromnodeidsandedgetypeid.dst, data.getedgeidfromnodeidsandedgetypeid.edge_type);
    }
    

    162 => {
        trace.push(format!("get_edge_id_from_node_names({:?}, {:?})", data.getedgeidfromnodenames.src_name, data.getedgeidfromnodenames.dst_name));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_edge_id_from_node_names(data.getedgeidfromnodenames.src_name, data.getedgeidfromnodenames.dst_name);
    }
    

    163 => {
        trace.push(format!("get_edge_id_from_node_names_and_edge_type_name({:?}, {:?}, {:?})", data.getedgeidfromnodenamesandedgetypename.src_name, data.getedgeidfromnodenamesandedgetypename.dst_name, data.getedgeidfromnodenamesandedgetypename.edge_type_name));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_edge_id_from_node_names_and_edge_type_name(data.getedgeidfromnodenamesandedgetypename.src_name, data.getedgeidfromnodenamesandedgetypename.dst_name, data.getedgeidfromnodenamesandedgetypename.edge_type_name);
    }
    

    164 => {
        trace.push(format!("get_edge_type_ids_from_edge_type_names({:?})", data.getedgetypeidsfromedgetypenames.edge_type_names));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_edge_type_ids_from_edge_type_names(data.getedgetypeidsfromedgetypenames.edge_type_names);
    }
    

    165 => {
        trace.push(format!("get_node_type_ids_from_node_type_names({:?})", data.getnodetypeidsfromnodetypenames.node_type_names));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_node_type_ids_from_node_type_names(data.getnodetypeidsfromnodetypenames.node_type_names);
    }
    

    166 => {
        trace.push(format!("get_multiple_node_type_ids_from_node_type_names({:?})", data.getmultiplenodetypeidsfromnodetypenames.node_type_names));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_multiple_node_type_ids_from_node_type_names(data.getmultiplenodetypeidsfromnodetypenames.node_type_names);
    }
    

    167 => {
        trace.push(format!("get_unchecked_minmax_edge_ids_from_source_node_id({:?})", data.getuncheckedminmaxedgeidsfromsourcenodeid.src));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        graph.get_unchecked_minmax_edge_ids_from_source_node_id(data.getuncheckedminmaxedgeidsfromsourcenodeid.src)
    }
    

    168 => {
        trace.push(format!("get_minmax_edge_ids_from_source_node_id({:?})", data.getminmaxedgeidsfromsourcenodeid.src));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_minmax_edge_ids_from_source_node_id(data.getminmaxedgeidsfromsourcenodeid.src);
    }
    

    169 => {
        trace.push(format!("get_node_type_name_from_node_type_id({:?})", data.getnodetypenamefromnodetypeid.node_type_id));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_node_type_name_from_node_type_id(data.getnodetypenamefromnodetypeid.node_type_id);
    }
    

    170 => {
        trace.push(format!("get_unchecked_node_type_names_from_node_type_ids({:?})", data.getuncheckednodetypenamesfromnodetypeids.node_type_ids));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        graph.get_unchecked_node_type_names_from_node_type_ids(data.getuncheckednodetypenamesfromnodetypeids.node_type_ids)
    }
    

    171 => {
        trace.push(format!("filter_from_ids({:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?})", data.filterfromids.node_ids_to_keep, data.filterfromids.node_ids_to_filter, data.filterfromids.node_type_ids_to_keep, data.filterfromids.node_type_ids_to_filter, data.filterfromids.node_type_id_to_keep, data.filterfromids.node_type_id_to_filter, data.filterfromids.edge_ids_to_keep, data.filterfromids.edge_ids_to_filter, data.filterfromids.edge_node_ids_to_keep, data.filterfromids.edge_node_ids_to_filter, data.filterfromids.edge_type_ids_to_keep, data.filterfromids.edge_type_ids_to_filter, data.filterfromids.min_edge_weight, data.filterfromids.max_edge_weight, data.filterfromids.filter_singleton_nodes, data.filterfromids.filter_singleton_nodes_with_selfloop, data.filterfromids.filter_selfloops, data.filterfromids.filter_parallel_edges, data.filterfromids.verbose));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        graph = graph.filter_from_ids(data.filterfromids.node_ids_to_keep, data.filterfromids.node_ids_to_filter, data.filterfromids.node_type_ids_to_keep, data.filterfromids.node_type_ids_to_filter, data.filterfromids.node_type_id_to_keep, data.filterfromids.node_type_id_to_filter, data.filterfromids.edge_ids_to_keep, data.filterfromids.edge_ids_to_filter, data.filterfromids.edge_node_ids_to_keep, data.filterfromids.edge_node_ids_to_filter, data.filterfromids.edge_type_ids_to_keep, data.filterfromids.edge_type_ids_to_filter, data.filterfromids.min_edge_weight, data.filterfromids.max_edge_weight, data.filterfromids.filter_singleton_nodes, data.filterfromids.filter_singleton_nodes_with_selfloop, data.filterfromids.filter_selfloops, data.filterfromids.filter_parallel_edges, data.filterfromids.verbose);
    }
    

    172 => {
        trace.push(format!("filter_from_names({:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?})", data.filterfromnames.node_names_to_keep, data.filterfromnames.node_names_to_filter, data.filterfromnames.node_type_names_to_keep, data.filterfromnames.node_type_names_to_filter, data.filterfromnames.node_type_name_to_keep, data.filterfromnames.node_type_name_to_filter, data.filterfromnames.edge_node_names_to_keep, data.filterfromnames.edge_node_names_to_filter, data.filterfromnames.edge_type_names_to_keep, data.filterfromnames.edge_type_names_to_filter, data.filterfromnames.min_edge_weight, data.filterfromnames.max_edge_weight, data.filterfromnames.filter_singleton_nodes, data.filterfromnames.filter_singleton_nodes_with_selfloop, data.filterfromnames.filter_selfloops, data.filterfromnames.filter_parallel_edges, data.filterfromnames.verbose));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
        if let Ok(res) = graph.filter_from_names(data.filterfromnames.node_names_to_keep, data.filterfromnames.node_names_to_filter, data.filterfromnames.node_type_names_to_keep, data.filterfromnames.node_type_names_to_filter, data.filterfromnames.node_type_name_to_keep, data.filterfromnames.node_type_name_to_filter, data.filterfromnames.edge_node_names_to_keep, data.filterfromnames.edge_node_names_to_filter, data.filterfromnames.edge_type_names_to_keep, data.filterfromnames.edge_type_names_to_filter, data.filterfromnames.min_edge_weight, data.filterfromnames.max_edge_weight, data.filterfromnames.filter_singleton_nodes, data.filterfromnames.filter_singleton_nodes_with_selfloop, data.filterfromnames.filter_selfloops, data.filterfromnames.filter_parallel_edges, data.filterfromnames.verbose) {
            graph = res;
        }
        
    }
    

    173 => {
        trace.push(format!("drop_unknown_node_types({:?})", data.dropunknownnodetypes.verbose));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        graph = graph.drop_unknown_node_types(data.dropunknownnodetypes.verbose);
    }
    

    174 => {
        trace.push(format!("drop_unknown_edge_types({:?})", data.dropunknownedgetypes.verbose));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        graph = graph.drop_unknown_edge_types(data.dropunknownedgetypes.verbose);
    }
    

    175 => {
        trace.push(format!("drop_singleton_nodes({:?})", data.dropsingletonnodes.verbose));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        graph = graph.drop_singleton_nodes(data.dropsingletonnodes.verbose);
    }
    

    176 => {
        trace.push(format!("drop_singleton_nodes_with_selfloops({:?})", data.dropsingletonnodeswithselfloops.verbose));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        graph = graph.drop_singleton_nodes_with_selfloops(data.dropsingletonnodeswithselfloops.verbose);
    }
    

    177 => {
        trace.push(format!("drop_selfloops({:?})", data.dropselfloops.verbose));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        graph = graph.drop_selfloops(data.dropselfloops.verbose);
    }
    

    178 => {
        trace.push(format!("drop_parallel_edges({:?})", data.dropparalleledges.verbose));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        graph = graph.drop_parallel_edges(data.dropparalleledges.verbose);
    }
    

    179 => {
        trace.push(format!("validate_node_id({:?})", data.validatenodeid.node_id));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.validate_node_id(data.validatenodeid.node_id);
    }
    

    180 => {
        trace.push(format!("validate_node_ids({:?})", data.validatenodeids.node_ids));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.validate_node_ids(data.validatenodeids.node_ids);
    }
    

    181 => {
        trace.push(format!("validate_edge_id({:?})", data.validateedgeid.edge_id));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.validate_edge_id(data.validateedgeid.edge_id);
    }
    

    182 => {
        trace.push(format!("validate_edge_ids({:?})", data.validateedgeids.edge_ids));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.validate_edge_ids(data.validateedgeids.edge_ids);
    }
    

    183 => {
        trace.push(format!("validate_node_type_id({:?})", data.validatenodetypeid.node_type_id));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.validate_node_type_id(data.validatenodetypeid.node_type_id);
    }
    

    184 => {
        trace.push(format!("validate_node_type_ids({:?})", data.validatenodetypeids.node_type_ids));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.validate_node_type_ids(data.validatenodetypeids.node_type_ids);
    }
    

    185 => {
        trace.push(format!("validate_edge_type_id({:?})", data.validateedgetypeid.edge_type_id));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.validate_edge_type_id(data.validateedgetypeid.edge_type_id);
    }
    

    186 => {
        trace.push(format!("validate_edge_type_ids({:?})", data.validateedgetypeids.edge_type_ids));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.validate_edge_type_ids(data.validateedgetypeids.edge_type_ids);
    }
    

    187 => {
        trace.push(format!("must_have_node_types()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.must_have_node_types();
    }
    

    188 => {
        trace.push(format!("must_have_edge_types()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.must_have_edge_types();
    }
    

    189 => {
        trace.push(format!("must_be_undirected()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.must_be_undirected();
    }
    

    190 => {
        trace.push(format!("must_be_multigraph()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.must_be_multigraph();
    }
    

    191 => {
        trace.push(format!("must_not_be_multigraph()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.must_not_be_multigraph();
    }
    

    192 => {
        trace.push(format!("must_have_edge_weights()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.must_have_edge_weights();
    }
    

    193 => {
        trace.push(format!("must_have_edge_weights_representing_probabilities()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.must_have_edge_weights_representing_probabilities();
    }
    

    194 => {
        trace.push(format!("must_have_positive_edge_weights()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.must_have_positive_edge_weights();
    }
    

    195 => {
        trace.push(format!("must_not_contain_weighted_singleton_nodes()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.must_not_contain_weighted_singleton_nodes();
    }
    

    196 => {
        trace.push(format!("must_have_edges()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.must_have_edges();
    }
    

    197 => {
        trace.push(format!("must_have_nodes()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.must_have_nodes();
    }
    

    198 => {
        trace.push(format!("remove_components({:?}, {:?}, {:?}, {:?}, {:?}, {:?})", data.removecomponents.node_names, data.removecomponents.node_types, data.removecomponents.edge_types, data.removecomponents.minimum_component_size, data.removecomponents.top_k_components, data.removecomponents.verbose));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
        if let Ok(res) = graph.remove_components(data.removecomponents.node_names, data.removecomponents.node_types, data.removecomponents.edge_types, data.removecomponents.minimum_component_size, data.removecomponents.top_k_components, data.removecomponents.verbose) {
            graph = res;
        }
        
    }
    

    199 => {
        trace.push(format!("iter_unweighted_degree_centrality()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.iter_unweighted_degree_centrality().map(|x| x.collect::<Vec<_>>());
            
    }
    

    200 => {
        trace.push(format!("par_iter_weighted_degree_centrality()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.par_iter_weighted_degree_centrality().map(|x| x.collect::<Vec<_>>());
            
    }
    

    201 => {
        trace.push(format!("get_unweighted_degree_centrality()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_unweighted_degree_centrality();
    }
    

    202 => {
        trace.push(format!("get_weighted_degree_centrality()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_weighted_degree_centrality();
    }
    

    203 => {
        trace.push(format!("get_unchecked_unweighted_closeness_centrality_from_node_id({:?})", data.getuncheckedunweightedclosenesscentralityfromnodeid.node_id));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        graph.get_unchecked_unweighted_closeness_centrality_from_node_id(data.getuncheckedunweightedclosenesscentralityfromnodeid.node_id)
    }
    

    204 => {
        trace.push(format!("get_unchecked_weighted_closeness_centrality_from_node_id({:?}, {:?})", data.getuncheckedweightedclosenesscentralityfromnodeid.node_id, data.getuncheckedweightedclosenesscentralityfromnodeid.use_edge_weights_as_probabilities));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        graph.get_unchecked_weighted_closeness_centrality_from_node_id(data.getuncheckedweightedclosenesscentralityfromnodeid.node_id, data.getuncheckedweightedclosenesscentralityfromnodeid.use_edge_weights_as_probabilities)
    }
    

    205 => {
        trace.push(format!("par_iter_unweighted_closeness_centrality({:?})", data.pariterunweightedclosenesscentrality.verbose));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.par_iter_unweighted_closeness_centrality(data.pariterunweightedclosenesscentrality.verbose).collect::<Vec<_>>();
            
    }
    

    206 => {
        trace.push(format!("par_iter_weighted_closeness_centrality({:?}, {:?})", data.pariterweightedclosenesscentrality.use_edge_weights_as_probabilities, data.pariterweightedclosenesscentrality.verbose));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.par_iter_weighted_closeness_centrality(data.pariterweightedclosenesscentrality.use_edge_weights_as_probabilities, data.pariterweightedclosenesscentrality.verbose).map(|x| x.collect::<Vec<_>>());
            
    }
    

    207 => {
        trace.push(format!("get_unweighted_closeness_centrality({:?})", data.getunweightedclosenesscentrality.verbose));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        graph.get_unweighted_closeness_centrality(data.getunweightedclosenesscentrality.verbose)
    }
    

    208 => {
        trace.push(format!("get_weighted_closeness_centrality({:?}, {:?})", data.getweightedclosenesscentrality.use_edge_weights_as_probabilities, data.getweightedclosenesscentrality.verbose));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_weighted_closeness_centrality(data.getweightedclosenesscentrality.use_edge_weights_as_probabilities, data.getweightedclosenesscentrality.verbose);
    }
    

    209 => {
        trace.push(format!("get_unchecked_unweighted_harmonic_centrality_from_node_id({:?})", data.getuncheckedunweightedharmoniccentralityfromnodeid.node_id));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        graph.get_unchecked_unweighted_harmonic_centrality_from_node_id(data.getuncheckedunweightedharmoniccentralityfromnodeid.node_id)
    }
    

    210 => {
        trace.push(format!("get_unchecked_weighted_harmonic_centrality_from_node_id({:?}, {:?})", data.getuncheckedweightedharmoniccentralityfromnodeid.node_id, data.getuncheckedweightedharmoniccentralityfromnodeid.use_edge_weights_as_probabilities));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        graph.get_unchecked_weighted_harmonic_centrality_from_node_id(data.getuncheckedweightedharmoniccentralityfromnodeid.node_id, data.getuncheckedweightedharmoniccentralityfromnodeid.use_edge_weights_as_probabilities)
    }
    

    211 => {
        trace.push(format!("par_iter_unweighted_harmonic_centrality({:?})", data.pariterunweightedharmoniccentrality.verbose));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.par_iter_unweighted_harmonic_centrality(data.pariterunweightedharmoniccentrality.verbose).collect::<Vec<_>>();
            
    }
    

    212 => {
        trace.push(format!("par_iter_weighted_harmonic_centrality({:?}, {:?})", data.pariterweightedharmoniccentrality.use_edge_weights_as_probabilities, data.pariterweightedharmoniccentrality.verbose));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.par_iter_weighted_harmonic_centrality(data.pariterweightedharmoniccentrality.use_edge_weights_as_probabilities, data.pariterweightedharmoniccentrality.verbose).map(|x| x.collect::<Vec<_>>());
            
    }
    

    213 => {
        trace.push(format!("get_unweighted_harmonic_centrality({:?})", data.getunweightedharmoniccentrality.verbose));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        graph.get_unweighted_harmonic_centrality(data.getunweightedharmoniccentrality.verbose)
    }
    

    214 => {
        trace.push(format!("get_weighted_harmonic_centrality({:?}, {:?})", data.getweightedharmoniccentrality.use_edge_weights_as_probabilities, data.getweightedharmoniccentrality.verbose));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_weighted_harmonic_centrality(data.getweightedharmoniccentrality.use_edge_weights_as_probabilities, data.getweightedharmoniccentrality.verbose);
    }
    

    215 => {
        trace.push(format!("get_stress_centrality({:?}, {:?})", data.getstresscentrality.normalize, data.getstresscentrality.verbose));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        graph.get_stress_centrality(data.getstresscentrality.normalize, data.getstresscentrality.verbose)
    }
    

    216 => {
        trace.push(format!("get_betweenness_centrality({:?}, {:?})", data.getbetweennesscentrality.normalize, data.getbetweennesscentrality.verbose));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        graph.get_betweenness_centrality(data.getbetweennesscentrality.normalize, data.getbetweennesscentrality.verbose)
    }
    

    217 => {
        trace.push(format!("get_unweighted_eigenvector_centrality({:?}, {:?})", data.getunweightedeigenvectorcentrality.maximum_iterations_number, data.getunweightedeigenvectorcentrality.tollerance));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_unweighted_eigenvector_centrality(data.getunweightedeigenvectorcentrality.maximum_iterations_number, data.getunweightedeigenvectorcentrality.tollerance);
    }
    

    218 => {
        trace.push(format!("get_weighted_eigenvector_centrality({:?}, {:?})", data.getweightedeigenvectorcentrality.maximum_iterations_number, data.getweightedeigenvectorcentrality.tollerance));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_weighted_eigenvector_centrality(data.getweightedeigenvectorcentrality.maximum_iterations_number, data.getweightedeigenvectorcentrality.tollerance);
    }
    

    219 => {
        trace.push(format!("set_name({:?})", data.setname.name));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        graph.set_name(data.setname.name)
    }
    

    220 => {
        trace.push(format!("set_inplace_all_edge_types({:?})", data.setinplacealledgetypes.edge_type));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.set_inplace_all_edge_types(data.setinplacealledgetypes.edge_type);
    }
    

    221 => {
        trace.push(format!("set_all_edge_types({:?}, {:?})", data.setalledgetypes.edge_type, data.setalledgetypes.verbose));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
        if let Ok(res) = graph.set_all_edge_types(data.setalledgetypes.edge_type, data.setalledgetypes.verbose) {
            graph = res;
        }
        
    }
    

    222 => {
        trace.push(format!("set_inplace_all_node_types({:?})", data.setinplaceallnodetypes.node_type));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.set_inplace_all_node_types(data.setinplaceallnodetypes.node_type);
    }
    

    223 => {
        trace.push(format!("set_all_node_types({:?})", data.setallnodetypes.node_type));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
        if let Ok(res) = graph.set_all_node_types(data.setallnodetypes.node_type) {
            graph = res;
        }
        
    }
    

    224 => {
        trace.push(format!("remove_inplace_node_type_ids({:?})", data.removeinplacenodetypeids.node_type_ids_to_remove));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.remove_inplace_node_type_ids(data.removeinplacenodetypeids.node_type_ids_to_remove);
    }
    

    225 => {
        trace.push(format!("remove_inplace_singleton_node_types()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.remove_inplace_singleton_node_types();
    }
    

    226 => {
        trace.push(format!("remove_inplace_edge_type_ids({:?})", data.removeinplaceedgetypeids.edge_type_ids_to_remove));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.remove_inplace_edge_type_ids(data.removeinplaceedgetypeids.edge_type_ids_to_remove);
    }
    

    227 => {
        trace.push(format!("remove_inplace_singleton_edge_types()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.remove_inplace_singleton_edge_types();
    }
    

    228 => {
        trace.push(format!("remove_inplace_node_type_name({:?})", data.removeinplacenodetypename.node_type_name));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.remove_inplace_node_type_name(data.removeinplacenodetypename.node_type_name);
    }
    

    229 => {
        trace.push(format!("remove_node_type_id({:?})", data.removenodetypeid.node_type_id));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
        if let Ok(res) = graph.remove_node_type_id(data.removenodetypeid.node_type_id) {
            graph = res;
        }
        
    }
    

    230 => {
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
    

    231 => {
        trace.push(format!("remove_node_type_name({:?})", data.removenodetypename.node_type_name));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
        if let Ok(res) = graph.remove_node_type_name(data.removenodetypename.node_type_name) {
            graph = res;
        }
        
    }
    

    232 => {
        trace.push(format!("remove_inplace_edge_type_name({:?})", data.removeinplaceedgetypename.edge_type_name));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.remove_inplace_edge_type_name(data.removeinplaceedgetypename.edge_type_name);
    }
    

    233 => {
        trace.push(format!("remove_edge_type_id({:?})", data.removeedgetypeid.edge_type_id));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
        if let Ok(res) = graph.remove_edge_type_id(data.removeedgetypeid.edge_type_id) {
            graph = res;
        }
        
    }
    

    234 => {
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
    

    235 => {
        trace.push(format!("remove_edge_type_name({:?})", data.removeedgetypename.edge_type_name));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
        if let Ok(res) = graph.remove_edge_type_name(data.removeedgetypename.edge_type_name) {
            graph = res;
        }
        
    }
    

    236 => {
        trace.push(format!("remove_inplace_node_types()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.remove_inplace_node_types();
    }
    

    237 => {
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
    

    238 => {
        trace.push(format!("remove_inplace_edge_types()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.remove_inplace_edge_types();
    }
    

    239 => {
        trace.push(format!("remove_edge_types({:?})", data.removeedgetypes.verbose));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
        if let Ok(res) = graph.remove_edge_types(data.removeedgetypes.verbose) {
            graph = res;
        }
        
    }
    

    240 => {
        trace.push(format!("remove_inplace_edge_weights()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.remove_inplace_edge_weights();
    }
    

    241 => {
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
    

    242 => {
        trace.push(format!("encode_edge({:?}, {:?})", data.encodeedge.src, data.encodeedge.dst));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        graph.encode_edge(data.encodeedge.src, data.encodeedge.dst)
    }
    

    243 => {
        trace.push(format!("decode_edge({:?})", data.decodeedge.edge));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        graph.decode_edge(data.decodeedge.edge)
    }
    

    244 => {
        trace.push(format!("get_max_encodable_edge_number()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        graph.get_max_encodable_edge_number()
    }
    

    245 => {
        trace.push(format!("get_bipartite_edges({:?}, {:?}, {:?}, {:?}, {:?})", data.getbipartiteedges.removed_existing_edges, data.getbipartiteedges.first_nodes_set, data.getbipartiteedges.second_nodes_set, data.getbipartiteedges.first_node_types_set, data.getbipartiteedges.second_node_types_set));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_bipartite_edges(data.getbipartiteedges.removed_existing_edges, data.getbipartiteedges.first_nodes_set, data.getbipartiteedges.second_nodes_set, data.getbipartiteedges.first_node_types_set, data.getbipartiteedges.second_node_types_set);
    }
    

    246 => {
        trace.push(format!("get_bipartite_edge_names({:?}, {:?}, {:?}, {:?}, {:?})", data.getbipartiteedgenames.removed_existing_edges, data.getbipartiteedgenames.first_nodes_set, data.getbipartiteedgenames.second_nodes_set, data.getbipartiteedgenames.first_node_types_set, data.getbipartiteedgenames.second_node_types_set));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_bipartite_edge_names(data.getbipartiteedgenames.removed_existing_edges, data.getbipartiteedgenames.first_nodes_set, data.getbipartiteedgenames.second_nodes_set, data.getbipartiteedgenames.first_node_types_set, data.getbipartiteedgenames.second_node_types_set);
    }
    

    247 => {
        trace.push(format!("get_star_edges({:?}, {:?}, {:?}, {:?})", data.getstaredges.central_node, data.getstaredges.removed_existing_edges, data.getstaredges.star_points_nodes_set, data.getstaredges.star_points_node_types_set));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_star_edges(data.getstaredges.central_node, data.getstaredges.removed_existing_edges, data.getstaredges.star_points_nodes_set, data.getstaredges.star_points_node_types_set);
    }
    

    248 => {
        trace.push(format!("get_star_edge_names({:?}, {:?}, {:?}, {:?})", data.getstaredgenames.central_node, data.getstaredgenames.removed_existing_edges, data.getstaredgenames.star_points_nodes_set, data.getstaredgenames.star_points_node_types_set));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_star_edge_names(data.getstaredgenames.central_node, data.getstaredgenames.removed_existing_edges, data.getstaredgenames.star_points_nodes_set, data.getstaredgenames.star_points_node_types_set);
    }
    

    249 => {
        trace.push(format!("get_clique_edges({:?}, {:?}, {:?}, {:?}, {:?})", data.getcliqueedges.directed, data.getcliqueedges.allow_selfloops, data.getcliqueedges.removed_existing_edges, data.getcliqueedges.allow_node_type_set, data.getcliqueedges.allow_node_set));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        graph.get_clique_edges(data.getcliqueedges.directed, data.getcliqueedges.allow_selfloops, data.getcliqueedges.removed_existing_edges, data.getcliqueedges.allow_node_type_set, data.getcliqueedges.allow_node_set)
    }
    

    250 => {
        trace.push(format!("get_clique_edge_names({:?}, {:?}, {:?}, {:?}, {:?})", data.getcliqueedgenames.directed, data.getcliqueedgenames.allow_selfloops, data.getcliqueedgenames.removed_existing_edges, data.getcliqueedgenames.allow_node_type_set, data.getcliqueedgenames.allow_node_set));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        graph.get_clique_edge_names(data.getcliqueedgenames.directed, data.getcliqueedgenames.allow_selfloops, data.getcliqueedgenames.removed_existing_edges, data.getcliqueedgenames.allow_node_type_set, data.getcliqueedgenames.allow_node_set)
    }
    

    251 => {
        trace.push(format!("replace({:?}, {:?}, {:?}, {:?}, {:?})", data.replace.node_name_mapping, data.replace.node_type_name_mapping, data.replace.node_type_names_mapping, data.replace.edge_type_name_mapping, data.replace.verbose));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
        if let Ok(res) = graph.replace(data.replace.node_name_mapping, data.replace.node_type_name_mapping, data.replace.node_type_names_mapping, data.replace.edge_type_name_mapping, data.replace.verbose) {
            graph = res;
        }
        
    }
    

    252 => {
        trace.push(format!("replace_unknown_node_types_with_node_type_name({:?}, {:?})", data.replaceunknownnodetypeswithnodetypename.node_type_names, data.replaceunknownnodetypeswithnodetypename.verbose));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
        if let Ok(res) = graph.replace_unknown_node_types_with_node_type_name(data.replaceunknownnodetypeswithnodetypename.node_type_names, data.replaceunknownnodetypeswithnodetypename.verbose) {
            graph = res;
        }
        
    }
    

    253 => {
        trace.push(format!("replace_unknown_edge_types_with_edge_type_name({:?}, {:?})", data.replaceunknownedgetypeswithedgetypename.edge_type_name, data.replaceunknownedgetypeswithedgetypename.verbose));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
        if let Ok(res) = graph.replace_unknown_edge_types_with_edge_type_name(data.replaceunknownedgetypeswithedgetypename.edge_type_name, data.replaceunknownedgetypeswithedgetypename.verbose) {
            graph = res;
        }
        
    }
    

    254 => {
        trace.push(format!("approximated_vertex_cover_bitvec()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        graph.approximated_vertex_cover_bitvec()
    }
    

    255 => {
        trace.push(format!("approximated_vertex_cover_set()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        graph.approximated_vertex_cover_set()
    }
    

    256 => {
        trace.push(format!("report()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        graph.report()
    }
    

    257 => {
        trace.push(format!("overlap_textual_report({:?}, {:?})", data.overlaptextualreport.other, data.overlaptextualreport.verbose));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.overlap_textual_report(data.overlaptextualreport.other, data.overlaptextualreport.verbose);
    }
    

    258 => {
        trace.push(format!("get_node_report_from_node_id({:?})", data.getnodereportfromnodeid.node_id));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_node_report_from_node_id(data.getnodereportfromnodeid.node_id);
    }
    

    259 => {
        trace.push(format!("get_node_report_from_node_name({:?})", data.getnodereportfromnodename.node_name));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_node_report_from_node_name(data.getnodereportfromnodename.node_name);
    }
    

    260 => {
        trace.push(format!("get_peculiarities_report_markdown()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        graph.get_peculiarities_report_markdown()
    }
    

    261 => {
        trace.push(format!("textual_report({:?})", data.textualreport.verbose));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.textual_report(data.textualreport.verbose);
    }
    

    262 => {
        trace.push(format!("get_connected_components_number({:?})", data.getconnectedcomponentsnumber.verbose));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        graph.get_connected_components_number(data.getconnectedcomponentsnumber.verbose)
    }
    

    263 => {
        trace.push(format!("get_singleton_nodes_number()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        graph.get_singleton_nodes_number()
    }
    

    264 => {
        trace.push(format!("get_weighted_singleton_nodes_number()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_weighted_singleton_nodes_number();
    }
    

    265 => {
        trace.push(format!("get_disconnected_nodes_number()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        graph.get_disconnected_nodes_number()
    }
    

    266 => {
        trace.push(format!("get_singleton_node_ids()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        graph.get_singleton_node_ids()
    }
    

    267 => {
        trace.push(format!("get_singleton_node_names()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        graph.get_singleton_node_names()
    }
    

    268 => {
        trace.push(format!("get_singleton_nodes_with_selfloops_number()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        graph.get_singleton_nodes_with_selfloops_number()
    }
    

    269 => {
        trace.push(format!("get_singleton_with_selfloops_node_ids()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        graph.get_singleton_with_selfloops_node_ids()
    }
    

    270 => {
        trace.push(format!("get_singleton_with_selfloops_node_names()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        graph.get_singleton_with_selfloops_node_names()
    }
    

    271 => {
        trace.push(format!("get_connected_nodes_number()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        graph.get_connected_nodes_number()
    }
    

    272 => {
        trace.push(format!("get_density()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_density();
    }
    

    273 => {
        trace.push(format!("get_trap_nodes_rate()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        graph.get_trap_nodes_rate()
    }
    

    274 => {
        trace.push(format!("get_unweighted_node_degrees_mean()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_unweighted_node_degrees_mean();
    }
    

    275 => {
        trace.push(format!("get_undirected_edges_number()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        graph.get_undirected_edges_number()
    }
    

    276 => {
        trace.push(format!("get_unique_undirected_edges_number()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        graph.get_unique_undirected_edges_number()
    }
    

    277 => {
        trace.push(format!("get_edges_number()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        graph.get_edges_number()
    }
    

    278 => {
        trace.push(format!("get_unique_edges_number()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        graph.get_unique_edges_number()
    }
    

    279 => {
        trace.push(format!("get_unweighted_node_degrees_median()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_unweighted_node_degrees_median();
    }
    

    280 => {
        trace.push(format!("get_weighted_node_degrees_median()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_weighted_node_degrees_median();
    }
    

    281 => {
        trace.push(format!("get_unchecked_unweighted_max_node_degree()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        graph.get_unchecked_unweighted_max_node_degree()
    }
    

    282 => {
        trace.push(format!("get_weighted_max_node_degree()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_weighted_max_node_degree();
    }
    

    283 => {
        trace.push(format!("get_unweighted_max_node_degree()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_unweighted_max_node_degree();
    }
    

    284 => {
        trace.push(format!("get_unchecked_argmax_node_degree()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        graph.get_unchecked_argmax_node_degree()
    }
    

    285 => {
        trace.push(format!("get_argmax_node_degree()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_argmax_node_degree();
    }
    

    286 => {
        trace.push(format!("get_unchecked_unweighted_min_node_degree()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        graph.get_unchecked_unweighted_min_node_degree()
    }
    

    287 => {
        trace.push(format!("get_weighted_min_node_degree()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_weighted_min_node_degree();
    }
    

    288 => {
        trace.push(format!("get_min_node_degree()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_min_node_degree();
    }
    

    289 => {
        trace.push(format!("get_unweighted_node_degrees_mode()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_unweighted_node_degrees_mode();
    }
    

    290 => {
        trace.push(format!("get_selfloop_nodes_number()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        graph.get_selfloop_nodes_number()
    }
    

    291 => {
        trace.push(format!("get_unique_selfloop_number()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        graph.get_unique_selfloop_number()
    }
    

    292 => {
        trace.push(format!("get_selfloop_nodes_rate()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_selfloop_nodes_rate();
    }
    

    293 => {
        trace.push(format!("get_name()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        graph.get_name()
    }
    

    294 => {
        trace.push(format!("get_trap_nodes_number()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        graph.get_trap_nodes_number()
    }
    

    295 => {
        trace.push(format!("get_source_node_ids({:?})", data.getsourcenodeids.directed));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        graph.get_source_node_ids(data.getsourcenodeids.directed)
    }
    

    296 => {
        trace.push(format!("get_source_names({:?})", data.getsourcenames.directed));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        graph.get_source_names(data.getsourcenames.directed)
    }
    

    297 => {
        trace.push(format!("get_destination_node_ids({:?})", data.getdestinationnodeids.directed));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        graph.get_destination_node_ids(data.getdestinationnodeids.directed)
    }
    

    298 => {
        trace.push(format!("get_destination_names({:?})", data.getdestinationnames.directed));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        graph.get_destination_names(data.getdestinationnames.directed)
    }
    

    299 => {
        trace.push(format!("get_node_names()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        graph.get_node_names()
    }
    

    300 => {
        trace.push(format!("get_node_ids()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        graph.get_node_ids()
    }
    

    301 => {
        trace.push(format!("get_edge_type_ids()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_edge_type_ids();
    }
    

    302 => {
        trace.push(format!("get_unique_edge_type_ids()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_unique_edge_type_ids();
    }
    

    303 => {
        trace.push(format!("get_edge_type_names()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_edge_type_names();
    }
    

    304 => {
        trace.push(format!("get_unique_edge_type_names()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_unique_edge_type_names();
    }
    

    305 => {
        trace.push(format!("get_edge_weights()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_edge_weights();
    }
    

    306 => {
        trace.push(format!("get_min_edge_weight()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_min_edge_weight();
    }
    

    307 => {
        trace.push(format!("get_max_edge_weight()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_max_edge_weight();
    }
    

    308 => {
        trace.push(format!("get_node_type_ids()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_node_type_ids();
    }
    

    309 => {
        trace.push(format!("get_one_hot_encoded_node_types()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_one_hot_encoded_node_types();
    }
    

    310 => {
        trace.push(format!("get_one_hot_encoded_known_node_types()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_one_hot_encoded_known_node_types();
    }
    

    311 => {
        trace.push(format!("get_one_hot_encoded_edge_types()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_one_hot_encoded_edge_types();
    }
    

    312 => {
        trace.push(format!("get_one_hot_encoded_known_edge_types()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_one_hot_encoded_known_edge_types();
    }
    

    313 => {
        trace.push(format!("get_node_type_names()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_node_type_names();
    }
    

    314 => {
        trace.push(format!("get_unique_node_type_ids()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_unique_node_type_ids();
    }
    

    315 => {
        trace.push(format!("get_unique_node_type_names()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_unique_node_type_names();
    }
    

    316 => {
        trace.push(format!("get_unique_directed_edges_number()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        graph.get_unique_directed_edges_number()
    }
    

    317 => {
        trace.push(format!("get_nodes_mapping()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        graph.get_nodes_mapping()
    }
    

    318 => {
        trace.push(format!("get_edge_node_ids({:?})", data.getedgenodeids.directed));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        graph.get_edge_node_ids(data.getedgenodeids.directed)
    }
    

    319 => {
        trace.push(format!("get_edge_node_names({:?})", data.getedgenodenames.directed));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        graph.get_edge_node_names(data.getedgenodenames.directed)
    }
    

    320 => {
        trace.push(format!("get_unknown_node_types_number()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_unknown_node_types_number();
    }
    

    321 => {
        trace.push(format!("get_known_node_types_number()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_known_node_types_number();
    }
    

    322 => {
        trace.push(format!("get_unknown_node_types_rate()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_unknown_node_types_rate();
    }
    

    323 => {
        trace.push(format!("get_known_node_types_rate()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_known_node_types_rate();
    }
    

    324 => {
        trace.push(format!("get_minimum_node_types_number()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_minimum_node_types_number();
    }
    

    325 => {
        trace.push(format!("get_singleton_node_types_number()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_singleton_node_types_number();
    }
    

    326 => {
        trace.push(format!("get_singleton_node_type_ids()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_singleton_node_type_ids();
    }
    

    327 => {
        trace.push(format!("get_singleton_node_type_names()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_singleton_node_type_names();
    }
    

    328 => {
        trace.push(format!("get_unknown_edge_types_number()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_unknown_edge_types_number();
    }
    

    329 => {
        trace.push(format!("get_edge_ids_with_unknown_edge_types()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_edge_ids_with_unknown_edge_types();
    }
    

    330 => {
        trace.push(format!("get_edge_ids_with_known_edge_types()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_edge_ids_with_known_edge_types();
    }
    

    331 => {
        trace.push(format!("get_node_ids_with_unknown_node_types()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_node_ids_with_unknown_node_types();
    }
    

    332 => {
        trace.push(format!("get_node_ids_with_known_node_types()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_node_ids_with_known_node_types();
    }
    

    333 => {
        trace.push(format!("get_known_edge_types_number()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_known_edge_types_number();
    }
    

    334 => {
        trace.push(format!("get_unknown_edge_types_rate()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_unknown_edge_types_rate();
    }
    

    335 => {
        trace.push(format!("get_known_edge_types_rate()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_known_edge_types_rate();
    }
    

    336 => {
        trace.push(format!("get_minimum_edge_types_number()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_minimum_edge_types_number();
    }
    

    337 => {
        trace.push(format!("get_singleton_edge_types_number()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_singleton_edge_types_number();
    }
    

    338 => {
        trace.push(format!("get_singleton_edge_type_ids()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_singleton_edge_type_ids();
    }
    

    339 => {
        trace.push(format!("get_singleton_edge_type_names()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_singleton_edge_type_names();
    }
    

    340 => {
        trace.push(format!("get_nodes_number()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        graph.get_nodes_number()
    }
    

    341 => {
        trace.push(format!("get_node_connected_component_ids({:?})", data.getnodeconnectedcomponentids.verbose));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        graph.get_node_connected_component_ids(data.getnodeconnectedcomponentids.verbose)
    }
    

    342 => {
        trace.push(format!("get_directed_edges_number()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        graph.get_directed_edges_number()
    }
    

    343 => {
        trace.push(format!("get_edge_types_number()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_edge_types_number();
    }
    

    344 => {
        trace.push(format!("get_node_types_number()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_node_types_number();
    }
    

    345 => {
        trace.push(format!("get_unweighted_node_degrees()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        graph.get_unweighted_node_degrees()
    }
    

    346 => {
        trace.push(format!("get_weighted_node_degrees()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_weighted_node_degrees();
    }
    

    347 => {
        trace.push(format!("get_not_singletons_node_ids()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        graph.get_not_singletons_node_ids()
    }
    

    348 => {
        trace.push(format!("get_dense_nodes_mapping()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        graph.get_dense_nodes_mapping()
    }
    

    349 => {
        trace.push(format!("get_multigraph_edges_number()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        graph.get_multigraph_edges_number()
    }
    

    350 => {
        trace.push(format!("get_cumulative_node_degrees()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        graph.get_cumulative_node_degrees()
    }
    

    351 => {
        trace.push(format!("get_unique_source_nodes_number()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        graph.get_unique_source_nodes_number()
    }
    

    352 => {
        trace.push(format!("get_edge_type_id_counts_hashmap()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_edge_type_id_counts_hashmap();
    }
    

    353 => {
        trace.push(format!("get_edge_type_names_counts_hashmap()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_edge_type_names_counts_hashmap();
    }
    

    354 => {
        trace.push(format!("get_node_type_id_counts_hashmap()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_node_type_id_counts_hashmap();
    }
    

    355 => {
        trace.push(format!("get_node_type_names_counts_hashmap()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_node_type_names_counts_hashmap();
    }
    

    356 => {
        trace.push(format!("get_dense_binary_adjacency_matrix()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        graph.get_dense_binary_adjacency_matrix()
    }
    

    357 => {
        trace.push(format!("get_dense_weighted_adjacency_matrix({:?})", data.getdenseweightedadjacencymatrix.weight));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_dense_weighted_adjacency_matrix(data.getdenseweightedadjacencymatrix.weight);
    }
    

    358 => {
        trace.push(format!("iter_node_ids()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.iter_node_ids().collect::<Vec<_>>();
            
    }
    

    359 => {
        trace.push(format!("par_iter_node_ids()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.par_iter_node_ids().collect::<Vec<_>>();
            
    }
    

    360 => {
        trace.push(format!("iter_node_names()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.iter_node_names().collect::<Vec<_>>();
            
    }
    

    361 => {
        trace.push(format!("par_iter_node_names()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.par_iter_node_names().collect::<Vec<_>>();
            
    }
    

    362 => {
        trace.push(format!("iter_unique_node_type_ids()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.iter_unique_node_type_ids().map(|x| x.collect::<Vec<_>>());
            
    }
    

    363 => {
        trace.push(format!("iter_node_type_counts()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.iter_node_type_counts().map(|x| x.collect::<Vec<_>>());
            
    }
    

    364 => {
        trace.push(format!("iter_unique_node_type_ids_and_counts()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.iter_unique_node_type_ids_and_counts().map(|x| x.collect::<Vec<_>>());
            
    }
    

    365 => {
        trace.push(format!("iter_unique_node_type_names()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.iter_unique_node_type_names().map(|x| x.collect::<Vec<_>>());
            
    }
    

    366 => {
        trace.push(format!("iter_unique_node_type_names_and_counts()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.iter_unique_node_type_names_and_counts().map(|x| x.collect::<Vec<_>>());
            
    }
    

    367 => {
        trace.push(format!("iter_unique_edge_type_ids()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.iter_unique_edge_type_ids().map(|x| x.collect::<Vec<_>>());
            
    }
    

    368 => {
        trace.push(format!("iter_edge_type_counts()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.iter_edge_type_counts().map(|x| x.collect::<Vec<_>>());
            
    }
    

    369 => {
        trace.push(format!("iter_unique_edge_type_ids_and_counts()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.iter_unique_edge_type_ids_and_counts().map(|x| x.collect::<Vec<_>>());
            
    }
    

    370 => {
        trace.push(format!("iter_unique_edge_type_names_and_counts()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.iter_unique_edge_type_names_and_counts().map(|x| x.collect::<Vec<_>>());
            
    }
    

    371 => {
        trace.push(format!("iter_unique_edge_type_names()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.iter_unique_edge_type_names().map(|x| x.collect::<Vec<_>>());
            
    }
    

    372 => {
        trace.push(format!("iter_unweighted_node_degrees()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.iter_unweighted_node_degrees().collect::<Vec<_>>();
            
    }
    

    373 => {
        trace.push(format!("par_iter_unweighted_node_degrees()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.par_iter_unweighted_node_degrees().collect::<Vec<_>>();
            
    }
    

    374 => {
        trace.push(format!("iter_weighted_node_degrees()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.iter_weighted_node_degrees().map(|x| x.collect::<Vec<_>>());
            
    }
    

    375 => {
        trace.push(format!("par_iter_weighted_node_degrees()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.par_iter_weighted_node_degrees().map(|x| x.collect::<Vec<_>>());
            
    }
    

    376 => {
        trace.push(format!("iter_connected_node_ids()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.iter_connected_node_ids().collect::<Vec<_>>();
            
    }
    

    377 => {
        trace.push(format!("iter_singleton_node_ids()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.iter_singleton_node_ids().collect::<Vec<_>>();
            
    }
    

    378 => {
        trace.push(format!("iter_singleton_node_names()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.iter_singleton_node_names().collect::<Vec<_>>();
            
    }
    

    379 => {
        trace.push(format!("iter_singleton_with_selfloops_node_ids()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.iter_singleton_with_selfloops_node_ids().collect::<Vec<_>>();
            
    }
    

    380 => {
        trace.push(format!("iter_singleton_with_selfloops_node_names()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.iter_singleton_with_selfloops_node_names().collect::<Vec<_>>();
            
    }
    

    381 => {
        trace.push(format!("iter_singleton_node_type_ids()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.iter_singleton_node_type_ids().map(|x| x.collect::<Vec<_>>());
            
    }
    

    382 => {
        trace.push(format!("iter_singleton_edge_type_ids()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.iter_singleton_edge_type_ids().map(|x| x.collect::<Vec<_>>());
            
    }
    

    383 => {
        trace.push(format!("iter_singleton_node_type_names()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.iter_singleton_node_type_names().map(|x| x.collect::<Vec<_>>());
            
    }
    

    384 => {
        trace.push(format!("iter_singleton_edge_type_names()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.iter_singleton_edge_type_names().map(|x| x.collect::<Vec<_>>());
            
    }
    

    385 => {
        trace.push(format!("iter_source_node_ids({:?})", data.itersourcenodeids.directed));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.iter_source_node_ids(data.itersourcenodeids.directed).collect::<Vec<_>>();
            
    }
    

    386 => {
        trace.push(format!("iter_edge_weights()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.iter_edge_weights().map(|x| x.collect::<Vec<_>>());
            
    }
    

    387 => {
        trace.push(format!("par_iter_edge_weights()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.par_iter_edge_weights().map(|x| x.collect::<Vec<_>>());
            
    }
    

    388 => {
        trace.push(format!("par_iter_source_node_ids({:?})", data.paritersourcenodeids.directed));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.par_iter_source_node_ids(data.paritersourcenodeids.directed).collect::<Vec<_>>();
            
    }
    

    389 => {
        trace.push(format!("iter_destination_node_ids({:?})", data.iterdestinationnodeids.directed));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.iter_destination_node_ids(data.iterdestinationnodeids.directed).collect::<Vec<_>>();
            
    }
    

    390 => {
        trace.push(format!("par_iter_destination_node_ids({:?})", data.pariterdestinationnodeids.directed));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.par_iter_destination_node_ids(data.pariterdestinationnodeids.directed).collect::<Vec<_>>();
            
    }
    

    391 => {
        trace.push(format!("iter_node_ids_and_node_type_ids()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.iter_node_ids_and_node_type_ids().collect::<Vec<_>>();
            
    }
    

    392 => {
        trace.push(format!("iter_unchecked_node_type_ids()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.iter_unchecked_node_type_ids().collect::<Vec<_>>();
            
    }
    

    393 => {
        trace.push(format!("iter_one_hot_encoded_node_type_ids()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.iter_one_hot_encoded_node_type_ids().map(|x| x.collect::<Vec<_>>());
            
    }
    

    394 => {
        trace.push(format!("iter_one_hot_encoded_known_node_type_ids()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.iter_one_hot_encoded_known_node_type_ids().map(|x| x.collect::<Vec<_>>());
            
    }
    

    395 => {
        trace.push(format!("par_iter_unchecked_node_ids_and_node_type_ids()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.par_iter_unchecked_node_ids_and_node_type_ids().collect::<Vec<_>>();
            
    }
    

    396 => {
        trace.push(format!("iter_node_names_and_node_type_names()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.iter_node_names_and_node_type_names().collect::<Vec<_>>();
            
    }
    

    397 => {
        trace.push(format!("par_iter_node_names_and_node_type_names()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.par_iter_node_names_and_node_type_names().collect::<Vec<_>>();
            
    }
    

    398 => {
        trace.push(format!("iter_edge_node_ids({:?})", data.iteredgenodeids.directed));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.iter_edge_node_ids(data.iteredgenodeids.directed).collect::<Vec<_>>();
            
    }
    

    399 => {
        trace.push(format!("iter_edges({:?})", data.iteredges.directed));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.iter_edges(data.iteredges.directed).collect::<Vec<_>>();
            
    }
    

    400 => {
        trace.push(format!("par_iter_edge_node_ids({:?})", data.pariteredgenodeids.directed));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.par_iter_edge_node_ids(data.pariteredgenodeids.directed).collect::<Vec<_>>();
            
    }
    

    401 => {
        trace.push(format!("par_iter_directed_edge_ids()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.par_iter_directed_edge_ids().collect::<Vec<_>>();
            
    }
    

    402 => {
        trace.push(format!("par_iter_edges({:?})", data.pariteredges.directed));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.par_iter_edges(data.pariteredges.directed).collect::<Vec<_>>();
            
    }
    

    403 => {
        trace.push(format!("iter_edge_node_ids_and_edge_weight()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.iter_edge_node_ids_and_edge_weight().map(|x| x.collect::<Vec<_>>());
            
    }
    

    404 => {
        trace.push(format!("par_iter_edge_node_ids_and_edge_weight()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.par_iter_edge_node_ids_and_edge_weight().map(|x| x.collect::<Vec<_>>());
            
    }
    

    405 => {
        trace.push(format!("iter_edge_node_ids_and_edge_type_id({:?})", data.iteredgenodeidsandedgetypeid.directed));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.iter_edge_node_ids_and_edge_type_id(data.iteredgenodeidsandedgetypeid.directed).collect::<Vec<_>>();
            
    }
    

    406 => {
        trace.push(format!("iter_one_hot_encoded_edge_type_ids()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.iter_one_hot_encoded_edge_type_ids().map(|x| x.collect::<Vec<_>>());
            
    }
    

    407 => {
        trace.push(format!("iter_one_hot_encoded_known_edge_type_ids()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.iter_one_hot_encoded_known_edge_type_ids().map(|x| x.collect::<Vec<_>>());
            
    }
    

    408 => {
        trace.push(format!("iter_edge_node_names_and_edge_type_name({:?})", data.iteredgenodenamesandedgetypename.directed));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        graph.iter_edge_node_names_and_edge_type_name(data.iteredgenodenamesandedgetypename.directed)
    }
    

    409 => {
        trace.push(format!("par_iter_edge_node_names_and_edge_type_name({:?})", data.pariteredgenodenamesandedgetypename.directed));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        graph.par_iter_edge_node_names_and_edge_type_name(data.pariteredgenodenamesandedgetypename.directed)
    }
    

    410 => {
        trace.push(format!("par_iter_edge_node_ids_and_edge_type_id({:?})", data.pariteredgenodeidsandedgetypeid.directed));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.par_iter_edge_node_ids_and_edge_type_id(data.pariteredgenodeidsandedgetypeid.directed).collect::<Vec<_>>();
            
    }
    

    411 => {
        trace.push(format!("par_iter_edge_node_names_and_edge_type_name_and_edge_weight({:?})", data.pariteredgenodenamesandedgetypenameandedgeweight.directed));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        graph.par_iter_edge_node_names_and_edge_type_name_and_edge_weight(data.pariteredgenodenamesandedgetypenameandedgeweight.directed)
    }
    

    412 => {
        trace.push(format!("iter_edge_node_names_and_edge_type_name_and_edge_weight({:?})", data.iteredgenodenamesandedgetypenameandedgeweight.directed));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        graph.iter_edge_node_names_and_edge_type_name_and_edge_weight(data.iteredgenodenamesandedgetypenameandedgeweight.directed)
    }
    

    413 => {
        trace.push(format!("par_iter_edge_node_ids_and_edge_type_id_and_edge_weight({:?})", data.pariteredgenodeidsandedgetypeidandedgeweight.directed));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.par_iter_edge_node_ids_and_edge_type_id_and_edge_weight(data.pariteredgenodeidsandedgetypeidandedgeweight.directed).collect::<Vec<_>>();
            
    }
    

    414 => {
        trace.push(format!("iter_edge_node_ids_and_edge_type_id_and_edge_weight({:?})", data.iteredgenodeidsandedgetypeidandedgeweight.directed));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.iter_edge_node_ids_and_edge_type_id_and_edge_weight(data.iteredgenodeidsandedgetypeidandedgeweight.directed).collect::<Vec<_>>();
            
    }
    

    415 => {
        trace.push(format!("iter_unique_edge_node_ids({:?})", data.iteruniqueedgenodeids.directed));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.iter_unique_edge_node_ids(data.iteruniqueedgenodeids.directed).collect::<Vec<_>>();
            
    }
    

    416 => {
        trace.push(format!("iter_unique_source_node_ids()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.iter_unique_source_node_ids().collect::<Vec<_>>();
            
    }
    

    417 => {
        trace.push(format!("iter_edge_ids_with_unknown_edge_types()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.iter_edge_ids_with_unknown_edge_types().map(|x| x.collect::<Vec<_>>());
            
    }
    

    418 => {
        trace.push(format!("iter_edge_ids_with_known_edge_types()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.iter_edge_ids_with_known_edge_types().map(|x| x.collect::<Vec<_>>());
            
    }
    

    419 => {
        trace.push(format!("iter_edge_node_ids_with_unknown_edge_types({:?})", data.iteredgenodeidswithunknownedgetypes.directed));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.iter_edge_node_ids_with_unknown_edge_types(data.iteredgenodeidswithunknownedgetypes.directed).map(|x| x.collect::<Vec<_>>());
            
    }
    

    420 => {
        trace.push(format!("iter_edge_node_ids_with_known_edge_types({:?})", data.iteredgenodeidswithknownedgetypes.directed));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.iter_edge_node_ids_with_known_edge_types(data.iteredgenodeidswithknownedgetypes.directed).map(|x| x.collect::<Vec<_>>());
            
    }
    

    421 => {
        trace.push(format!("iter_node_ids_with_unknown_node_types()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.iter_node_ids_with_unknown_node_types().map(|x| x.collect::<Vec<_>>());
            
    }
    

    422 => {
        trace.push(format!("iter_node_ids_with_known_node_types()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.iter_node_ids_with_known_node_types().map(|x| x.collect::<Vec<_>>());
            
    }
    

    423 => {
        trace.push(format!("par_iter_edge_ids_with_unknown_edge_types()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.par_iter_edge_ids_with_unknown_edge_types().map(|x| x.collect::<Vec<_>>());
            
    }
    

    424 => {
        trace.push(format!("par_iter_edge_ids_with_known_edge_types()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.par_iter_edge_ids_with_known_edge_types().map(|x| x.collect::<Vec<_>>());
            
    }
    

    425 => {
        trace.push(format!("par_iter_edge_node_ids_with_unknown_edge_types({:?})", data.pariteredgenodeidswithunknownedgetypes.directed));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.par_iter_edge_node_ids_with_unknown_edge_types(data.pariteredgenodeidswithunknownedgetypes.directed).map(|x| x.collect::<Vec<_>>());
            
    }
    

    426 => {
        trace.push(format!("par_iter_edge_node_ids_with_known_edge_types({:?})", data.pariteredgenodeidswithknownedgetypes.directed));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.par_iter_edge_node_ids_with_known_edge_types(data.pariteredgenodeidswithknownedgetypes.directed).map(|x| x.collect::<Vec<_>>());
            
    }
    

    427 => {
        trace.push(format!("par_iter_node_ids_with_unknown_node_types()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.par_iter_node_ids_with_unknown_node_types().map(|x| x.collect::<Vec<_>>());
            
    }
    

    428 => {
        trace.push(format!("par_iter_node_ids_with_known_node_types()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.par_iter_node_ids_with_known_node_types().map(|x| x.collect::<Vec<_>>());
            
    }
    

    429 => {
        trace.push(format!("get_unweighted_laplacian_transformed_graph({:?})", data.getunweightedlaplaciantransformedgraph.verbose));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        graph = graph.get_unweighted_laplacian_transformed_graph(data.getunweightedlaplaciantransformedgraph.verbose);
    }
    

    430 => {
        trace.push(format!("get_unweighted_random_walk_normalized_laplacian_transformed_graph({:?})", data.getunweightedrandomwalknormalizedlaplaciantransformedgraph.verbose));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        graph = graph.get_unweighted_random_walk_normalized_laplacian_transformed_graph(data.getunweightedrandomwalknormalizedlaplaciantransformedgraph.verbose);
    }
    

    431 => {
        trace.push(format!("get_unweighted_symmetric_normalized_laplacian_transformed_graph({:?})", data.getunweightedsymmetricnormalizedlaplaciantransformedgraph.verbose));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
        if let Ok(res) = graph.get_unweighted_symmetric_normalized_laplacian_transformed_graph(data.getunweightedsymmetricnormalizedlaplaciantransformedgraph.verbose) {
            graph = res;
        }
        
    }
    

    432 => {
        trace.push(format!("get_unweighted_symmetric_normalized_transformed_graph({:?})", data.getunweightedsymmetricnormalizedtransformedgraph.verbose));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
        if let Ok(res) = graph.get_unweighted_symmetric_normalized_transformed_graph(data.getunweightedsymmetricnormalizedtransformedgraph.verbose) {
            graph = res;
        }
        
    }
    

    433 => {
        trace.push(format!("get_weighted_laplacian_transformed_graph({:?})", data.getweightedlaplaciantransformedgraph.verbose));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
        if let Ok(res) = graph.get_weighted_laplacian_transformed_graph(data.getweightedlaplaciantransformedgraph.verbose) {
            graph = res;
        }
        
    }
    

    434 => {
        trace.push(format!("get_weighted_symmetric_normalized_laplacian_transformed_graph({:?})", data.getweightedsymmetricnormalizedlaplaciantransformedgraph.verbose));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
        if let Ok(res) = graph.get_weighted_symmetric_normalized_laplacian_transformed_graph(data.getweightedsymmetricnormalizedlaplaciantransformedgraph.verbose) {
            graph = res;
        }
        
    }
    

    435 => {
        trace.push(format!("get_weighted_symmetric_normalized_transformed_graph({:?})", data.getweightedsymmetricnormalizedtransformedgraph.verbose));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
        if let Ok(res) = graph.get_weighted_symmetric_normalized_transformed_graph(data.getweightedsymmetricnormalizedtransformedgraph.verbose) {
            graph = res;
        }
        
    }
    

    436 => {
        trace.push(format!("get_weighted_random_walk_normalized_laplacian_transformed_graph({:?})", data.getweightedrandomwalknormalizedlaplaciantransformedgraph.verbose));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
        if let Ok(res) = graph.get_weighted_random_walk_normalized_laplacian_transformed_graph(data.getweightedrandomwalknormalizedlaplaciantransformedgraph.verbose) {
            graph = res;
        }
        
    }
    

    437 => {
        trace.push(format!("has_nodes()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        graph.has_nodes()
    }
    

    438 => {
        trace.push(format!("has_edges()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        graph.has_edges()
    }
    

    439 => {
        trace.push(format!("has_trap_nodes()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        graph.has_trap_nodes()
    }
    

    440 => {
        trace.push(format!("is_directed()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        graph.is_directed()
    }
    

    441 => {
        trace.push(format!("has_edge_weights()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        graph.has_edge_weights()
    }
    

    442 => {
        trace.push(format!("has_edge_weights_representing_probabilities()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.has_edge_weights_representing_probabilities();
    }
    

    443 => {
        trace.push(format!("has_weighted_singleton_nodes()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.has_weighted_singleton_nodes();
    }
    

    444 => {
        trace.push(format!("has_negative_edge_weights()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.has_negative_edge_weights();
    }
    

    445 => {
        trace.push(format!("has_edge_types()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        graph.has_edge_types()
    }
    

    446 => {
        trace.push(format!("has_selfloops()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        graph.has_selfloops()
    }
    

    447 => {
        trace.push(format!("has_disconnected_nodes()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        graph.has_disconnected_nodes()
    }
    

    448 => {
        trace.push(format!("has_singleton_nodes()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        graph.has_singleton_nodes()
    }
    

    449 => {
        trace.push(format!("has_singleton_nodes_with_selfloops()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        graph.has_singleton_nodes_with_selfloops()
    }
    

    450 => {
        trace.push(format!("is_connected({:?})", data.isconnected.verbose));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        graph.is_connected(data.isconnected.verbose)
    }
    

    451 => {
        trace.push(format!("has_node_types()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        graph.has_node_types()
    }
    

    452 => {
        trace.push(format!("has_multilabel_node_types()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.has_multilabel_node_types();
    }
    

    453 => {
        trace.push(format!("has_unknown_node_types()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.has_unknown_node_types();
    }
    

    454 => {
        trace.push(format!("has_unknown_edge_types()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.has_unknown_edge_types();
    }
    

    455 => {
        trace.push(format!("has_homogeneous_node_types()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.has_homogeneous_node_types();
    }
    

    456 => {
        trace.push(format!("has_homogeneous_edge_types()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.has_homogeneous_edge_types();
    }
    

    457 => {
        trace.push(format!("has_singleton_node_types()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.has_singleton_node_types();
    }
    

    458 => {
        trace.push(format!("has_node_oddities()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        graph.has_node_oddities()
    }
    

    459 => {
        trace.push(format!("has_node_types_oddities()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.has_node_types_oddities();
    }
    

    460 => {
        trace.push(format!("has_singleton_edge_types()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.has_singleton_edge_types();
    }
    

    461 => {
        trace.push(format!("has_edge_types_oddities()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.has_edge_types_oddities();
    }
    

    462 => {
        trace.push(format!("is_multigraph()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        graph.is_multigraph()
    }
    

    463 => {
        trace.push(format!("compute_hash()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        graph.compute_hash()
    }
    

    464 => {
        trace.push(format!("generate_new_edges_from_node_features({:?}, {:?}, {:?}, {:?}, {:?})", data.generatenewedgesfromnodefeatures.features, data.generatenewedgesfromnodefeatures.neighbours_number, data.generatenewedgesfromnodefeatures.max_degree, data.generatenewedgesfromnodefeatures.distance_name, data.generatenewedgesfromnodefeatures.verbose));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
        if let Ok(res) = graph.generate_new_edges_from_node_features(data.generatenewedgesfromnodefeatures.features, data.generatenewedgesfromnodefeatures.neighbours_number, data.generatenewedgesfromnodefeatures.max_degree, data.generatenewedgesfromnodefeatures.distance_name, data.generatenewedgesfromnodefeatures.verbose) {
            graph = res;
        }
        
    }
    

    465 => {
        trace.push(format!("add_selfloops({:?}, {:?}, {:?})", data.addselfloops.edge_type_name, data.addselfloops.weight, data.addselfloops.verbose));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
        if let Ok(res) = graph.add_selfloops(data.addselfloops.edge_type_name, data.addselfloops.weight, data.addselfloops.verbose) {
            graph = res;
        }
        
    }
    

    466 => {
        trace.push(format!("sample_negatives({:?}, {:?}, {:?}, {:?}, {:?})", data.samplenegatives.negatives_number, data.samplenegatives.random_state, data.samplenegatives.seed_graph, data.samplenegatives.only_from_same_component, data.samplenegatives.verbose));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
        if let Ok(res) = graph.sample_negatives(data.samplenegatives.negatives_number, data.samplenegatives.random_state, data.samplenegatives.seed_graph, data.samplenegatives.only_from_same_component, data.samplenegatives.verbose) {
            graph = res;
        }
        
    }
    

    467 => {
        trace.push(format!("connected_holdout({:?}, {:?}, {:?}, {:?}, {:?})", data.connectedholdout.train_size, data.connectedholdout.random_state, data.connectedholdout.edge_types, data.connectedholdout.include_all_edge_types, data.connectedholdout.verbose));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
        if let Ok((res1, res2)) = graph.connected_holdout(data.connectedholdout.train_size, data.connectedholdout.random_state, data.connectedholdout.edge_types, data.connectedholdout.include_all_edge_types, data.connectedholdout.verbose) {
            if rng.next() % 2 == 0 {
                graph = res1;
            } else {
                graph = res2;
            }
        }
        
    }
    

    468 => {
        trace.push(format!("random_holdout({:?}, {:?}, {:?}, {:?}, {:?}, {:?})", data.randomholdout.train_size, data.randomholdout.random_state, data.randomholdout.include_all_edge_types, data.randomholdout.edge_types, data.randomholdout.min_number_overlaps, data.randomholdout.verbose));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
        if let Ok((res1, res2)) = graph.random_holdout(data.randomholdout.train_size, data.randomholdout.random_state, data.randomholdout.include_all_edge_types, data.randomholdout.edge_types, data.randomholdout.min_number_overlaps, data.randomholdout.verbose) {
            if rng.next() % 2 == 0 {
                graph = res1;
            } else {
                graph = res2;
            }
        }
        
    }
    

    469 => {
        trace.push(format!("node_label_holdout({:?}, {:?}, {:?})", data.nodelabelholdout.train_size, data.nodelabelholdout.use_stratification, data.nodelabelholdout.random_state));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
        if let Ok((res1, res2)) = graph.node_label_holdout(data.nodelabelholdout.train_size, data.nodelabelholdout.use_stratification, data.nodelabelholdout.random_state) {
            if rng.next() % 2 == 0 {
                graph = res1;
            } else {
                graph = res2;
            }
        }
        
    }
    

    470 => {
        trace.push(format!("edge_label_holdout({:?}, {:?}, {:?})", data.edgelabelholdout.train_size, data.edgelabelholdout.use_stratification, data.edgelabelholdout.random_state));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
        if let Ok((res1, res2)) = graph.edge_label_holdout(data.edgelabelholdout.train_size, data.edgelabelholdout.use_stratification, data.edgelabelholdout.random_state) {
            if rng.next() % 2 == 0 {
                graph = res1;
            } else {
                graph = res2;
            }
        }
        
    }
    

    471 => {
        trace.push(format!("random_subgraph({:?}, {:?}, {:?})", data.randomsubgraph.nodes_number, data.randomsubgraph.random_state, data.randomsubgraph.verbose));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
        if let Ok(res) = graph.random_subgraph(data.randomsubgraph.nodes_number, data.randomsubgraph.random_state, data.randomsubgraph.verbose) {
            graph = res;
        }
        
    }
    

    472 => {
        trace.push(format!("kfold({:?}, {:?}, {:?}, {:?}, {:?})", data.kfold.k, data.kfold.k_index, data.kfold.edge_types, data.kfold.random_state, data.kfold.verbose));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
        if let Ok((res1, res2)) = graph.kfold(data.kfold.k, data.kfold.k_index, data.kfold.edge_types, data.kfold.random_state, data.kfold.verbose) {
            if rng.next() % 2 == 0 {
                graph = res1;
            } else {
                graph = res2;
            }
        }
        
    }
    
            _ => unreachable!()
        }
    }
    
    let _ = graph::test_utilities::default_test_suite(&mut graph, None);

    Ok(())
}
