
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
pub struct IsTrapNodeFromNodeId {
    pub node_id : NodeT,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct SortByIncreasingOutboundNodeDegree {
    pub verbose : Option<bool>,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct SortByDecreasingOutboundNodeDegree {
    pub verbose : Option<bool>,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct GetDenseWeightedAdjacencyMatrix {
    pub weight : Option<WeightT>,
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
pub struct EncodeEdge {
    pub src : NodeT,
    pub dst : NodeT,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct DecodeEdge {
    pub edge : u64,
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
pub struct IterEdgeNodeNamesWithUnknownEdgeTypes {
    pub directed : bool,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct IterEdgeNodeNamesWithKnownEdgeTypes {
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
pub struct ParIterEdgeNodeNamesWithUnknownEdgeTypes {
    pub directed : bool,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct ParIterEdgeNodeNamesWithKnownEdgeTypes {
    pub directed : bool,
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
pub struct GetUnweightedNumberOfTriangles {
    pub normalize : Option<bool>,
    pub low_centrality : Option<usize>,
    pub verbose : Option<bool>,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct GetUnweightedTransitivity {
    pub low_centrality : Option<usize>,
    pub verbose : Option<bool>,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct GetUnweightedNumberOfTrianglesPerNode {
    pub normalize : Option<bool>,
    pub low_centrality : Option<usize>,
    pub verbose : Option<bool>,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct IterClusteringCoefficientPerNode {
    pub low_centrality : Option<usize>,
    pub verbose : Option<bool>,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct GetClusteringCoefficientPerNode {
    pub low_centrality : Option<usize>,
    pub verbose : Option<bool>,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct GetClusteringCoefficient {
    pub low_centrality : Option<usize>,
    pub verbose : Option<bool>,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct GetAverageClusteringCoefficient {
    pub low_centrality : Option<usize>,
    pub verbose : Option<bool>,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct RemapFromNodeIds {
    pub node_ids : Vec<NodeT>,
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
pub struct GetUnweightedMinimumPathNodeIdsFromNodeIds {
    pub src_node_id : NodeT,
    pub dst_node_id : NodeT,
    pub maximal_depth : Option<NodeT>,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct GetUnweightedKShortestPathNodeIdsFromNodeIds {
    pub src_node_id : NodeT,
    pub dst_node_id : NodeT,
    pub k : u8,
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
pub struct GetEdgeNodeIdsWithUnknownEdgeTypes {
    pub directed : bool,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct GetEdgeNodeIdsWithKnownEdgeTypes {
    pub directed : bool,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct GetEdgeNodeNamesWithUnknownEdgeTypes {
    pub directed : bool,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct GetEdgeNodeNamesWithKnownEdgeTypes {
    pub directed : bool,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct GetNodeConnectedComponentIds {
    pub verbose : Option<bool>,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct ToUpperTriangular {
    pub verbose : Option<bool>,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct ToLowerTriangular {
    pub verbose : Option<bool>,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct ToMainDiagonal {
    pub verbose : Option<bool>,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct ToAntiDiagonal {
    pub verbose : Option<bool>,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct ToBidiagonal {
    pub verbose : Option<bool>,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct ToArrowhead {
    pub verbose : Option<bool>,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct ToTransposed {
    pub verbose : Option<bool>,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct ToComplementary {
    pub verbose : Option<bool>,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct GetNodeReportFromNodeId {
    pub node_id : NodeT,
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
pub struct DropDisconnectedNodes {
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
pub struct Enable {
    pub vector_sources : Option<bool>,
    pub vector_destinations : Option<bool>,
    pub vector_cumulative_node_degrees : Option<bool>,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct GetNodeLabelPredictionTupleFromNodeIds {
    pub node_ids : Vec<NodeT>,
    pub random_state : u64,
    pub include_central_node : Option<bool>,
    pub return_edge_weights : Option<bool>,
    pub offset : NodeT,
    pub max_neighbours : Option<NodeT>,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct ParIterEdgePredictionMetrics {
    pub normalize : Option<bool>,
    pub verbose : Option<bool>,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct GetOkapiBm25NodeFeaturePropagation {
    pub features : Vec<Vec<f64>>,
    pub iterations : Option<u8>,
    pub maximal_distance : Option<usize>,
    pub k1 : Option<f64>,
    pub b : Option<f64>,
    pub include_central_node : Option<bool>,
    pub verbose : Option<bool>,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct GetOkapiBm25NodeLabelPropagation {
    pub iterations : Option<u8>,
    pub maximal_distance : Option<usize>,
    pub k1 : Option<f64>,
    pub b : Option<f64>,
    pub verbose : Option<bool>,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct IsConnected {
    pub verbose : Option<bool>,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct GetTransitiveClosure {
    pub iterations : Option<u8>,
    pub verbose : Option<bool>,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct GetUnweightedAllShortestPaths {
    pub iterations : Option<u8>,
    pub verbose : Option<bool>,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct GetWeightedAllShortestPaths {
    pub iterations : Option<u8>,
    pub use_edge_weights_as_probabilities : Option<bool>,
    pub verbose : Option<bool>,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct GetSourceNodeIdFromEdgeId {
    pub edge_id : EdgeT,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct GetDestinationNodeIdFromEdgeId {
    pub edge_id : EdgeT,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct GetSourceNodeNameFromEdgeId {
    pub edge_id : EdgeT,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct GetDestinationNodeNameFromEdgeId {
    pub edge_id : EdgeT,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct GetNodeNamesFromEdgeId {
    pub edge_id : EdgeT,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct GetNodeIdsFromEdgeId {
    pub edge_id : EdgeT,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct GetEdgeIdFromNodeIds {
    pub src : NodeT,
    pub dst : NodeT,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct GetNodeIdsAndEdgeTypeIdFromEdgeId {
    pub edge_id : EdgeT,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct GetNodeIdsAndEdgeTypeIdAndEdgeWeightFromEdgeId {
    pub edge_id : EdgeT,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct GetUnweightedTopKCentralNodeIds {
    pub k : NodeT,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct GetWeightedTopKCentralNodeIds {
    pub k : NodeT,
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
pub struct GetNodeTypeIdFromNodeId {
    pub node_id : NodeT,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct GetEdgeTypeIdFromEdgeId {
    pub edge_id : EdgeT,
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
pub struct GetMinmaxEdgeIdsFromSourceNodeId {
    pub src : NodeT,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct GetNodeTypeNameFromNodeTypeId {
    pub node_type_id : NodeTypeT,
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
    pub maximum_iterations_number : Option<u8>,
    pub tollerance : Option<f64>,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct GetWeightedEigenvectorCentrality {
    pub maximum_iterations_number : Option<u8>,
    pub tollerance : Option<f64>,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct ToDot {
    pub use_node_names : Option<bool>,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct GetUnweightedPreferentialAttachmentFromNodeIds {
    pub source_node_id : NodeT,
    pub destination_node_id : NodeT,
    pub normalize : bool,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct GetWeightedPreferentialAttachmentFromNodeIds {
    pub source_node_id : NodeT,
    pub destination_node_id : NodeT,
    pub normalize : bool,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct GetJaccardCoefficientFromNodeIds {
    pub source_node_id : NodeT,
    pub destination_node_id : NodeT,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct GetAdamicAdarIndexFromNodeIds {
    pub source_node_id : NodeT,
    pub destination_node_id : NodeT,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct GetUnweightedResourceAllocationIndexFromNodeIds {
    pub source_node_id : NodeT,
    pub destination_node_id : NodeT,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct GetWeightedResourceAllocationIndexFromNodeIds {
    pub source_node_id : NodeT,
    pub destination_node_id : NodeT,
}



#[derive(Arbitrary, Debug, Clone)]
pub struct MetaParams {
    pub seed: u64,
    pub getunweightedlaplaciantransformedgraph : GetUnweightedLaplacianTransformedGraph,
    pub getunweightedrandomwalknormalizedlaplaciantransformedgraph : GetUnweightedRandomWalkNormalizedLaplacianTransformedGraph,
    pub getunweightedsymmetricnormalizedlaplaciantransformedgraph : GetUnweightedSymmetricNormalizedLaplacianTransformedGraph,
    pub getunweightedsymmetricnormalizedtransformedgraph : GetUnweightedSymmetricNormalizedTransformedGraph,
    pub getweightedlaplaciantransformedgraph : GetWeightedLaplacianTransformedGraph,
    pub getweightedsymmetricnormalizedlaplaciantransformedgraph : GetWeightedSymmetricNormalizedLaplacianTransformedGraph,
    pub getweightedsymmetricnormalizedtransformedgraph : GetWeightedSymmetricNormalizedTransformedGraph,
    pub getweightedrandomwalknormalizedlaplaciantransformedgraph : GetWeightedRandomWalkNormalizedLaplacianTransformedGraph,
    pub issingletonfromnodeid : IsSingletonFromNodeId,
    pub issingletonwithselfloopsfromnodeid : IsSingletonWithSelfloopsFromNodeId,
    pub hasnodetypeid : HasNodeTypeId,
    pub hasedgetypeid : HasEdgeTypeId,
    pub hasedgefromnodeids : HasEdgeFromNodeIds,
    pub hasselfloopfromnodeid : HasSelfloopFromNodeId,
    pub hasedgefromnodeidsandedgetypeid : HasEdgeFromNodeIdsAndEdgeTypeId,
    pub istrapnodefromnodeid : IsTrapNodeFromNodeId,

    pub sortbyincreasingoutboundnodedegree : SortByIncreasingOutboundNodeDegree,
    pub sortbydecreasingoutboundnodedegree : SortByDecreasingOutboundNodeDegree,

    pub getdenseweightedadjacencymatrix : GetDenseWeightedAdjacencyMatrix,
    pub iteredgeidsfromnodeids : IterEdgeIdsFromNodeIds,
    pub iteredgenodeidsandedgetypeidfromedgetypeid : IterEdgeNodeIdsAndEdgeTypeIdFromEdgeTypeId,
    pub iternodeidsandnodetypeidsfromnodetypeid : IterNodeIdsAndNodeTypeIdsFromNodeTypeId,
    pub iternodenamesandnodetypenamesfromnodetypeid : IterNodeNamesAndNodeTypeNamesFromNodeTypeId,
    pub iteredgenodenamesandedgetypenamefromedgetypeid : IterEdgeNodeNamesAndEdgeTypeNameFromEdgeTypeId,
    pub encodeedge : EncodeEdge,
    pub decodeedge : DecodeEdge,

    pub validatenodeid : ValidateNodeId,
    pub validatenodeids : ValidateNodeIds,
    pub validateedgeid : ValidateEdgeId,
    pub validateedgeids : ValidateEdgeIds,


    pub validatenodetypeid : ValidateNodeTypeId,
    pub validatenodetypeids : ValidateNodeTypeIds,
    pub validateedgetypeid : ValidateEdgeTypeId,
    pub validateedgetypeids : ValidateEdgeTypeIds,






































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


    pub iteredgenodenameswithunknownedgetypes : IterEdgeNodeNamesWithUnknownEdgeTypes,
    pub iteredgenodenameswithknownedgetypes : IterEdgeNodeNamesWithKnownEdgeTypes,




    pub pariteredgenodeidswithunknownedgetypes : ParIterEdgeNodeIdsWithUnknownEdgeTypes,
    pub pariteredgenodeidswithknownedgetypes : ParIterEdgeNodeIdsWithKnownEdgeTypes,


    pub pariteredgenodenameswithunknownedgetypes : ParIterEdgeNodeNamesWithUnknownEdgeTypes,
    pub pariteredgenodenameswithknownedgetypes : ParIterEdgeNodeNamesWithKnownEdgeTypes,


    pub removeinplacenodetypeids : RemoveInplaceNodeTypeIds,

    pub removeinplaceedgetypeids : RemoveInplaceEdgeTypeIds,

    pub removenodetypeid : RemoveNodeTypeId,

    pub removeedgetypeid : RemoveEdgeTypeId,




    pub removeedgetypes : RemoveEdgeTypes,


    pub getunweightednumberoftriangles : GetUnweightedNumberOfTriangles,


    pub getunweightedtransitivity : GetUnweightedTransitivity,
    pub getunweightednumberoftrianglespernode : GetUnweightedNumberOfTrianglesPerNode,
    pub iterclusteringcoefficientpernode : IterClusteringCoefficientPerNode,
    pub getclusteringcoefficientpernode : GetClusteringCoefficientPerNode,
    pub getclusteringcoefficient : GetClusteringCoefficient,
    pub getaverageclusteringcoefficient : GetAverageClusteringCoefficient,
    pub remapfromnodeids : RemapFromNodeIds,
    pub nodelabelholdout : NodeLabelHoldout,
    pub edgelabelholdout : EdgeLabelHoldout,
    pub randomsubgraph : RandomSubgraph,
    pub getunweightedminimumpathnodeidsfromnodeids : GetUnweightedMinimumPathNodeIdsFromNodeIds,
    pub getunweightedkshortestpathnodeidsfromnodeids : GetUnweightedKShortestPathNodeIdsFromNodeIds,
    pub getunweightedeccentricityfromnodeid : GetUnweightedEccentricityFromNodeId,
    pub getweightedeccentricityfromnodeid : GetWeightedEccentricityFromNodeId,
    pub getweightedminimumpathnodeidsfromnodeids : GetWeightedMinimumPathNodeIdsFromNodeIds,
    pub getbreathfirstsearchfromnodeids : GetBreathFirstSearchFromNodeIds,
    pub getdijkstrafromnodeids : GetDijkstraFromNodeIds,
    pub getunweighteddiameter : GetUnweightedDiameter,
    pub getweighteddiameter : GetWeightedDiameter,
    pub getconnectedcomponentsnumber : GetConnectedComponentsNumber,































    pub getsourcenodeids : GetSourceNodeIds,
    pub getsourcenames : GetSourceNames,
    pub getdestinationnodeids : GetDestinationNodeIds,
    pub getdestinationnames : GetDestinationNames,




















    pub getedgenodeids : GetEdgeNodeIds,
    pub getedgenodenames : GetEdgeNodeNames,













    pub getedgenodeidswithunknownedgetypes : GetEdgeNodeIdsWithUnknownEdgeTypes,
    pub getedgenodeidswithknownedgetypes : GetEdgeNodeIdsWithKnownEdgeTypes,
    pub getedgenodenameswithunknownedgetypes : GetEdgeNodeNamesWithUnknownEdgeTypes,
    pub getedgenodenameswithknownedgetypes : GetEdgeNodeNamesWithKnownEdgeTypes,
















    pub getnodeconnectedcomponentids : GetNodeConnectedComponentIds,
















    pub touppertriangular : ToUpperTriangular,
    pub tolowertriangular : ToLowerTriangular,
    pub tomaindiagonal : ToMainDiagonal,
    pub toantidiagonal : ToAntiDiagonal,
    pub tobidiagonal : ToBidiagonal,
    pub toarrowhead : ToArrowhead,
    pub totransposed : ToTransposed,
    pub tocomplementary : ToComplementary,

    pub getnodereportfromnodeid : GetNodeReportFromNodeId,

    pub filterfromids : FilterFromIds,
    pub dropunknownnodetypes : DropUnknownNodeTypes,
    pub dropunknownedgetypes : DropUnknownEdgeTypes,
    pub dropsingletonnodes : DropSingletonNodes,
    pub dropsingletonnodeswithselfloops : DropSingletonNodesWithSelfloops,
    pub dropdisconnectednodes : DropDisconnectedNodes,
    pub dropselfloops : DropSelfloops,
    pub dropparalleledges : DropParallelEdges,
    pub spanningarborescencekruskal : SpanningArborescenceKruskal,
    pub spanningarborescence : SpanningArborescence,
    pub connectedcomponents : ConnectedComponents,
    pub enable : Enable,



    pub getnodelabelpredictiontuplefromnodeids : GetNodeLabelPredictionTupleFromNodeIds,
    pub pariteredgepredictionmetrics : ParIterEdgePredictionMetrics,
    pub getokapibm25nodefeaturepropagation : GetOkapiBm25NodeFeaturePropagation,
    pub getokapibm25nodelabelpropagation : GetOkapiBm25NodeLabelPropagation,















    pub isconnected : IsConnected,
















    pub gettransitiveclosure : GetTransitiveClosure,
    pub getunweightedallshortestpaths : GetUnweightedAllShortestPaths,
    pub getweightedallshortestpaths : GetWeightedAllShortestPaths,
    pub getsourcenodeidfromedgeid : GetSourceNodeIdFromEdgeId,
    pub getdestinationnodeidfromedgeid : GetDestinationNodeIdFromEdgeId,
    pub getsourcenodenamefromedgeid : GetSourceNodeNameFromEdgeId,
    pub getdestinationnodenamefromedgeid : GetDestinationNodeNameFromEdgeId,
    pub getnodenamesfromedgeid : GetNodeNamesFromEdgeId,
    pub getnodeidsfromedgeid : GetNodeIdsFromEdgeId,
    pub getedgeidfromnodeids : GetEdgeIdFromNodeIds,
    pub getnodeidsandedgetypeidfromedgeid : GetNodeIdsAndEdgeTypeIdFromEdgeId,
    pub getnodeidsandedgetypeidandedgeweightfromedgeid : GetNodeIdsAndEdgeTypeIdAndEdgeWeightFromEdgeId,
    pub getunweightedtopkcentralnodeids : GetUnweightedTopKCentralNodeIds,
    pub getweightedtopkcentralnodeids : GetWeightedTopKCentralNodeIds,
    pub getunweightednodedegreefromnodeid : GetUnweightedNodeDegreeFromNodeId,
    pub getweightednodedegreefromnodeid : GetWeightedNodeDegreeFromNodeId,
    pub gettopkcentralnodenames : GetTopKCentralNodeNames,
    pub getnodetypeidfromnodeid : GetNodeTypeIdFromNodeId,
    pub getedgetypeidfromedgeid : GetEdgeTypeIdFromEdgeId,
    pub getnodetypenamesfromnodeid : GetNodeTypeNamesFromNodeId,
    pub getedgetypenamefromedgeid : GetEdgeTypeNameFromEdgeId,
    pub getedgetypenamefromedgetypeid : GetEdgeTypeNameFromEdgeTypeId,
    pub getedgeweightfromedgeid : GetEdgeWeightFromEdgeId,
    pub getedgeweightfromnodeids : GetEdgeWeightFromNodeIds,
    pub getedgeweightfromnodeidsandedgetypeid : GetEdgeWeightFromNodeIdsAndEdgeTypeId,
    pub getnodenamefromnodeid : GetNodeNameFromNodeId,
    pub getedgenodenamesfromedgenodeids : GetEdgeNodeNamesFromEdgeNodeIds,
    pub getedgecountfromedgetypeid : GetEdgeCountFromEdgeTypeId,
    pub getnodecountfromnodetypeid : GetNodeCountFromNodeTypeId,
    pub getneighbournodeidsfromnodeid : GetNeighbourNodeIdsFromNodeId,
    pub getminmaxedgeidsfromnodeids : GetMinmaxEdgeIdsFromNodeIds,
    pub getedgeidfromnodeidsandedgetypeid : GetEdgeIdFromNodeIdsAndEdgeTypeId,
    pub getminmaxedgeidsfromsourcenodeid : GetMinmaxEdgeIdsFromSourceNodeId,
    pub getnodetypenamefromnodetypeid : GetNodeTypeNameFromNodeTypeId,




    pub pariterunweightedclosenesscentrality : ParIterUnweightedClosenessCentrality,
    pub pariterweightedclosenesscentrality : ParIterWeightedClosenessCentrality,
    pub getunweightedclosenesscentrality : GetUnweightedClosenessCentrality,
    pub getweightedclosenesscentrality : GetWeightedClosenessCentrality,
    pub pariterunweightedharmoniccentrality : ParIterUnweightedHarmonicCentrality,
    pub pariterweightedharmoniccentrality : ParIterWeightedHarmonicCentrality,
    pub getunweightedharmoniccentrality : GetUnweightedHarmonicCentrality,
    pub getweightedharmoniccentrality : GetWeightedHarmonicCentrality,
    pub getstresscentrality : GetStressCentrality,
    pub getbetweennesscentrality : GetBetweennessCentrality,
    pub getunweightedeigenvectorcentrality : GetUnweightedEigenvectorCentrality,
    pub getweightedeigenvectorcentrality : GetWeightedEigenvectorCentrality,
    pub todot : ToDot,

    pub getunweightedpreferentialattachmentfromnodeids : GetUnweightedPreferentialAttachmentFromNodeIds,
    pub getweightedpreferentialattachmentfromnodeids : GetWeightedPreferentialAttachmentFromNodeIds,
    pub getjaccardcoefficientfromnodeids : GetJaccardCoefficientFromNodeIds,
    pub getadamicadarindexfromnodeids : GetAdamicAdarIndexFromNodeIds,
    pub getunweightedresourceallocationindexfromnodeids : GetUnweightedResourceAllocationIndexFromNodeIds,
    pub getweightedresourceallocationindexfromnodeids : GetWeightedResourceAllocationIndexFromNodeIds,
    pub from_vec: FromVecHarnessParams,
}


pub fn meta_test_harness_with_panic_handling(data: MetaParams) -> Result<(), String> {
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
        match rng.next() % 391 {

    0 => {
        trace.push(format!("get_unweighted_laplacian_transformed_graph(verbose: {:?})", &data.getunweightedlaplaciantransformedgraph.verbose));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        graph = graph.get_unweighted_laplacian_transformed_graph(data.getunweightedlaplaciantransformedgraph.verbose.clone());
    }
    

    1 => {
        trace.push(format!("get_unweighted_random_walk_normalized_laplacian_transformed_graph(verbose: {:?})", &data.getunweightedrandomwalknormalizedlaplaciantransformedgraph.verbose));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        graph = graph.get_unweighted_random_walk_normalized_laplacian_transformed_graph(data.getunweightedrandomwalknormalizedlaplaciantransformedgraph.verbose.clone());
    }
    

    2 => {
        trace.push(format!("get_unweighted_symmetric_normalized_laplacian_transformed_graph(verbose: {:?})", &data.getunweightedsymmetricnormalizedlaplaciantransformedgraph.verbose));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
        if let Ok(res) = graph.get_unweighted_symmetric_normalized_laplacian_transformed_graph(data.getunweightedsymmetricnormalizedlaplaciantransformedgraph.verbose.clone()) {
            graph = res;
        }
        
    }
    

    3 => {
        trace.push(format!("get_unweighted_symmetric_normalized_transformed_graph(verbose: {:?})", &data.getunweightedsymmetricnormalizedtransformedgraph.verbose));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
        if let Ok(res) = graph.get_unweighted_symmetric_normalized_transformed_graph(data.getunweightedsymmetricnormalizedtransformedgraph.verbose.clone()) {
            graph = res;
        }
        
    }
    

    4 => {
        trace.push(format!("get_weighted_laplacian_transformed_graph(verbose: {:?})", &data.getweightedlaplaciantransformedgraph.verbose));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
        if let Ok(res) = graph.get_weighted_laplacian_transformed_graph(data.getweightedlaplaciantransformedgraph.verbose.clone()) {
            graph = res;
        }
        
    }
    

    5 => {
        trace.push(format!("get_weighted_symmetric_normalized_laplacian_transformed_graph(verbose: {:?})", &data.getweightedsymmetricnormalizedlaplaciantransformedgraph.verbose));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
        if let Ok(res) = graph.get_weighted_symmetric_normalized_laplacian_transformed_graph(data.getweightedsymmetricnormalizedlaplaciantransformedgraph.verbose.clone()) {
            graph = res;
        }
        
    }
    

    6 => {
        trace.push(format!("get_weighted_symmetric_normalized_transformed_graph(verbose: {:?})", &data.getweightedsymmetricnormalizedtransformedgraph.verbose));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
        if let Ok(res) = graph.get_weighted_symmetric_normalized_transformed_graph(data.getweightedsymmetricnormalizedtransformedgraph.verbose.clone()) {
            graph = res;
        }
        
    }
    

    7 => {
        trace.push(format!("get_weighted_random_walk_normalized_laplacian_transformed_graph(verbose: {:?})", &data.getweightedrandomwalknormalizedlaplaciantransformedgraph.verbose));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
        if let Ok(res) = graph.get_weighted_random_walk_normalized_laplacian_transformed_graph(data.getweightedrandomwalknormalizedlaplaciantransformedgraph.verbose.clone()) {
            graph = res;
        }
        
    }
    

    8 => {
        trace.push(format!("is_singleton_from_node_id(node_id: {:?})", &data.issingletonfromnodeid.node_id));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.is_singleton_from_node_id(data.issingletonfromnodeid.node_id.clone());
    }
    

    9 => {
        trace.push(format!("is_singleton_with_selfloops_from_node_id(node_id: {:?})", &data.issingletonwithselfloopsfromnodeid.node_id));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.is_singleton_with_selfloops_from_node_id(data.issingletonwithselfloopsfromnodeid.node_id.clone());
    }
    

    10 => {
        trace.push(format!("has_node_type_id(node_type_id: {:?})", &data.hasnodetypeid.node_type_id));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.has_node_type_id(data.hasnodetypeid.node_type_id.clone());
    }
    

    11 => {
        trace.push(format!("has_edge_type_id(edge_type_id: {:?})", &data.hasedgetypeid.edge_type_id));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.has_edge_type_id(data.hasedgetypeid.edge_type_id.clone());
    }
    

    12 => {
        trace.push(format!("has_edge_from_node_ids(src: {:?}, dst: {:?})", &data.hasedgefromnodeids.src, &data.hasedgefromnodeids.dst));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.has_edge_from_node_ids(data.hasedgefromnodeids.src.clone(), data.hasedgefromnodeids.dst.clone());
    }
    

    13 => {
        trace.push(format!("has_selfloop_from_node_id(node_id: {:?})", &data.hasselfloopfromnodeid.node_id));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.has_selfloop_from_node_id(data.hasselfloopfromnodeid.node_id.clone());
    }
    

    14 => {
        trace.push(format!("has_edge_from_node_ids_and_edge_type_id(src: {:?}, dst: {:?}, edge_type: {:?})", &data.hasedgefromnodeidsandedgetypeid.src, &data.hasedgefromnodeidsandedgetypeid.dst, &data.hasedgefromnodeidsandedgetypeid.edge_type));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.has_edge_from_node_ids_and_edge_type_id(data.hasedgefromnodeidsandedgetypeid.src.clone(), data.hasedgefromnodeidsandedgetypeid.dst.clone(), data.hasedgefromnodeidsandedgetypeid.edge_type.clone());
    }
    

    15 => {
        trace.push(format!("is_trap_node_from_node_id(node_id: {:?})", &data.istrapnodefromnodeid.node_id));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.is_trap_node_from_node_id(data.istrapnodefromnodeid.node_id.clone());
    }
    

    16 => {
        trace.push(format!("strongly_connected_components()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.strongly_connected_components();
    }
    

    17 => {
        trace.push(format!("sort_by_increasing_outbound_node_degree(verbose: {:?})", &data.sortbyincreasingoutboundnodedegree.verbose));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        graph = graph.sort_by_increasing_outbound_node_degree(data.sortbyincreasingoutboundnodedegree.verbose.clone());
    }
    

    18 => {
        trace.push(format!("sort_by_decreasing_outbound_node_degree(verbose: {:?})", &data.sortbydecreasingoutboundnodedegree.verbose));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        graph = graph.sort_by_decreasing_outbound_node_degree(data.sortbydecreasingoutboundnodedegree.verbose.clone());
    }
    

    19 => {
        trace.push(format!("get_dense_binary_adjacency_matrix()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_dense_binary_adjacency_matrix();
    }
    

    20 => {
        trace.push(format!("get_dense_weighted_adjacency_matrix(weight: {:?})", &data.getdenseweightedadjacencymatrix.weight));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_dense_weighted_adjacency_matrix(data.getdenseweightedadjacencymatrix.weight.clone());
    }
    

    21 => {
        trace.push(format!("iter_edge_ids_from_node_ids(src: {:?}, dst: {:?})", &data.iteredgeidsfromnodeids.src, &data.iteredgeidsfromnodeids.dst));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.iter_edge_ids_from_node_ids(data.iteredgeidsfromnodeids.src.clone(), data.iteredgeidsfromnodeids.dst.clone()).map(|x| x.collect::<Vec<_>>());
            
    }
    

    22 => {
        trace.push(format!("iter_edge_node_ids_and_edge_type_id_from_edge_type_id(edge_type_id: {:?}, directed: {:?})", &data.iteredgenodeidsandedgetypeidfromedgetypeid.edge_type_id, &data.iteredgenodeidsandedgetypeidfromedgetypeid.directed));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.iter_edge_node_ids_and_edge_type_id_from_edge_type_id(data.iteredgenodeidsandedgetypeidfromedgetypeid.edge_type_id.clone(), data.iteredgenodeidsandedgetypeidfromedgetypeid.directed.clone()).map(|x| x.collect::<Vec<_>>());
            
    }
    

    23 => {
        trace.push(format!("iter_node_ids_and_node_type_ids_from_node_type_id(node_type_id: {:?})", &data.iternodeidsandnodetypeidsfromnodetypeid.node_type_id));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.iter_node_ids_and_node_type_ids_from_node_type_id(data.iternodeidsandnodetypeidsfromnodetypeid.node_type_id.clone()).map(|x| x.collect::<Vec<_>>());
            
    }
    

    24 => {
        trace.push(format!("iter_node_names_and_node_type_names_from_node_type_id(node_type_id: {:?})", &data.iternodenamesandnodetypenamesfromnodetypeid.node_type_id));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.iter_node_names_and_node_type_names_from_node_type_id(data.iternodenamesandnodetypenamesfromnodetypeid.node_type_id.clone());
    }
    

    25 => {
        trace.push(format!("iter_edge_node_names_and_edge_type_name_from_edge_type_id(edge_type_id: {:?}, directed: {:?})", &data.iteredgenodenamesandedgetypenamefromedgetypeid.edge_type_id, &data.iteredgenodenamesandedgetypenamefromedgetypeid.directed));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.iter_edge_node_names_and_edge_type_name_from_edge_type_id(data.iteredgenodenamesandedgetypenamefromedgetypeid.edge_type_id.clone(), data.iteredgenodenamesandedgetypenamefromedgetypeid.directed.clone());
    }
    

    26 => {
        trace.push(format!("encode_edge(src: {:?}, dst: {:?})", &data.encodeedge.src, &data.encodeedge.dst));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.encode_edge(data.encodeedge.src.clone(), data.encodeedge.dst.clone());
    }
    

    27 => {
        trace.push(format!("decode_edge(edge: {:?})", &data.decodeedge.edge));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.decode_edge(data.decodeedge.edge.clone());
    }
    

    28 => {
        trace.push(format!("get_max_encodable_edge_number()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_max_encodable_edge_number();
    }
    

    29 => {
        trace.push(format!("validate_node_id(node_id: {:?})", &data.validatenodeid.node_id));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.validate_node_id(data.validatenodeid.node_id.clone());
    }
    

    30 => {
        trace.push(format!("validate_node_ids(node_ids: {:?})", &data.validatenodeids.node_ids));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.validate_node_ids(data.validatenodeids.node_ids.clone());
    }
    

    31 => {
        trace.push(format!("validate_edge_id(edge_id: {:?})", &data.validateedgeid.edge_id));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.validate_edge_id(data.validateedgeid.edge_id.clone());
    }
    

    32 => {
        trace.push(format!("validate_edge_ids(edge_ids: {:?})", &data.validateedgeids.edge_ids));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.validate_edge_ids(data.validateedgeids.edge_ids.clone());
    }
    

    33 => {
        trace.push(format!("must_not_contain_unknown_node_types()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.must_not_contain_unknown_node_types();
    }
    

    34 => {
        trace.push(format!("must_not_contain_unknown_edge_types()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.must_not_contain_unknown_edge_types();
    }
    

    35 => {
        trace.push(format!("validate_node_type_id(node_type_id: {:?})", &data.validatenodetypeid.node_type_id));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.validate_node_type_id(data.validatenodetypeid.node_type_id.clone());
    }
    

    36 => {
        trace.push(format!("validate_node_type_ids(node_type_ids: {:?})", &data.validatenodetypeids.node_type_ids));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.validate_node_type_ids(data.validatenodetypeids.node_type_ids.clone());
    }
    

    37 => {
        trace.push(format!("validate_edge_type_id(edge_type_id: {:?})", &data.validateedgetypeid.edge_type_id));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.validate_edge_type_id(data.validateedgetypeid.edge_type_id.clone());
    }
    

    38 => {
        trace.push(format!("validate_edge_type_ids(edge_type_ids: {:?})", &data.validateedgetypeids.edge_type_ids));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.validate_edge_type_ids(data.validateedgetypeids.edge_type_ids.clone());
    }
    

    39 => {
        trace.push(format!("must_have_node_types()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.must_have_node_types();
    }
    

    40 => {
        trace.push(format!("must_have_edge_types()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.must_have_edge_types();
    }
    

    41 => {
        trace.push(format!("must_be_undirected()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.must_be_undirected();
    }
    

    42 => {
        trace.push(format!("must_be_multigraph()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.must_be_multigraph();
    }
    

    43 => {
        trace.push(format!("must_not_be_multigraph()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.must_not_be_multigraph();
    }
    

    44 => {
        trace.push(format!("must_have_edge_weights()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.must_have_edge_weights();
    }
    

    45 => {
        trace.push(format!("must_have_edge_weights_representing_probabilities()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.must_have_edge_weights_representing_probabilities();
    }
    

    46 => {
        trace.push(format!("must_have_positive_edge_weights()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.must_have_positive_edge_weights();
    }
    

    47 => {
        trace.push(format!("must_not_contain_weighted_singleton_nodes()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.must_not_contain_weighted_singleton_nodes();
    }
    

    48 => {
        trace.push(format!("must_have_edges()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.must_have_edges();
    }
    

    49 => {
        trace.push(format!("must_have_nodes()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.must_have_nodes();
    }
    

    50 => {
        trace.push(format!("iter_node_ids()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.iter_node_ids().collect::<Vec<_>>();
            
    }
    

    51 => {
        trace.push(format!("par_iter_node_ids()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.par_iter_node_ids().collect::<Vec<_>>();
            
    }
    

    52 => {
        trace.push(format!("iter_node_names()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.iter_node_names().collect::<Vec<_>>();
            
    }
    

    53 => {
        trace.push(format!("par_iter_node_names()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.par_iter_node_names().collect::<Vec<_>>();
            
    }
    

    54 => {
        trace.push(format!("iter_unique_node_type_ids()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.iter_unique_node_type_ids().map(|x| x.collect::<Vec<_>>());
            
    }
    

    55 => {
        trace.push(format!("iter_node_type_counts()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.iter_node_type_counts().map(|x| x.collect::<Vec<_>>());
            
    }
    

    56 => {
        trace.push(format!("iter_unique_node_type_ids_and_counts()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.iter_unique_node_type_ids_and_counts().map(|x| x.collect::<Vec<_>>());
            
    }
    

    57 => {
        trace.push(format!("iter_unique_node_type_names()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.iter_unique_node_type_names().map(|x| x.collect::<Vec<_>>());
            
    }
    

    58 => {
        trace.push(format!("iter_unique_node_type_names_and_counts()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.iter_unique_node_type_names_and_counts().map(|x| x.collect::<Vec<_>>());
            
    }
    

    59 => {
        trace.push(format!("iter_unique_edge_type_ids()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.iter_unique_edge_type_ids().map(|x| x.collect::<Vec<_>>());
            
    }
    

    60 => {
        trace.push(format!("iter_edge_type_counts()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.iter_edge_type_counts().map(|x| x.collect::<Vec<_>>());
            
    }
    

    61 => {
        trace.push(format!("iter_unique_edge_type_ids_and_counts()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.iter_unique_edge_type_ids_and_counts().map(|x| x.collect::<Vec<_>>());
            
    }
    

    62 => {
        trace.push(format!("iter_unique_edge_type_names_and_counts()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.iter_unique_edge_type_names_and_counts().map(|x| x.collect::<Vec<_>>());
            
    }
    

    63 => {
        trace.push(format!("iter_unique_edge_type_names()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.iter_unique_edge_type_names().map(|x| x.collect::<Vec<_>>());
            
    }
    

    64 => {
        trace.push(format!("iter_unweighted_node_degrees()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.iter_unweighted_node_degrees().collect::<Vec<_>>();
            
    }
    

    65 => {
        trace.push(format!("par_iter_unweighted_node_degrees()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.par_iter_unweighted_node_degrees().collect::<Vec<_>>();
            
    }
    

    66 => {
        trace.push(format!("iter_weighted_node_degrees()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.iter_weighted_node_degrees().map(|x| x.collect::<Vec<_>>());
            
    }
    

    67 => {
        trace.push(format!("par_iter_weighted_node_degrees()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.par_iter_weighted_node_degrees().map(|x| x.collect::<Vec<_>>());
            
    }
    

    68 => {
        trace.push(format!("iter_connected_node_ids()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.iter_connected_node_ids().collect::<Vec<_>>();
            
    }
    

    69 => {
        trace.push(format!("iter_singleton_node_ids()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.iter_singleton_node_ids().collect::<Vec<_>>();
            
    }
    

    70 => {
        trace.push(format!("iter_singleton_node_names()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.iter_singleton_node_names().collect::<Vec<_>>();
            
    }
    

    71 => {
        trace.push(format!("iter_singleton_nodes_with_selfloops_node_ids()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.iter_singleton_nodes_with_selfloops_node_ids().collect::<Vec<_>>();
            
    }
    

    72 => {
        trace.push(format!("iter_singleton_nodes_with_selfloops_node_names()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.iter_singleton_nodes_with_selfloops_node_names().collect::<Vec<_>>();
            
    }
    

    73 => {
        trace.push(format!("iter_singleton_node_type_ids()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.iter_singleton_node_type_ids().map(|x| x.collect::<Vec<_>>());
            
    }
    

    74 => {
        trace.push(format!("iter_singleton_edge_type_ids()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.iter_singleton_edge_type_ids().map(|x| x.collect::<Vec<_>>());
            
    }
    

    75 => {
        trace.push(format!("iter_singleton_node_type_names()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.iter_singleton_node_type_names().map(|x| x.collect::<Vec<_>>());
            
    }
    

    76 => {
        trace.push(format!("iter_singleton_edge_type_names()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.iter_singleton_edge_type_names().map(|x| x.collect::<Vec<_>>());
            
    }
    

    77 => {
        trace.push(format!("iter_source_node_ids(directed: {:?})", &data.itersourcenodeids.directed));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.iter_source_node_ids(data.itersourcenodeids.directed.clone()).collect::<Vec<_>>();
            
    }
    

    78 => {
        trace.push(format!("iter_edge_weights()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.iter_edge_weights().map(|x| x.collect::<Vec<_>>());
            
    }
    

    79 => {
        trace.push(format!("par_iter_edge_weights()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.par_iter_edge_weights().map(|x| x.collect::<Vec<_>>());
            
    }
    

    80 => {
        trace.push(format!("par_iter_source_node_ids(directed: {:?})", &data.paritersourcenodeids.directed));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.par_iter_source_node_ids(data.paritersourcenodeids.directed.clone()).collect::<Vec<_>>();
            
    }
    

    81 => {
        trace.push(format!("iter_destination_node_ids(directed: {:?})", &data.iterdestinationnodeids.directed));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.iter_destination_node_ids(data.iterdestinationnodeids.directed.clone()).collect::<Vec<_>>();
            
    }
    

    82 => {
        trace.push(format!("par_iter_destination_node_ids(directed: {:?})", &data.pariterdestinationnodeids.directed));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.par_iter_destination_node_ids(data.pariterdestinationnodeids.directed.clone()).collect::<Vec<_>>();
            
    }
    

    83 => {
        trace.push(format!("iter_node_ids_and_node_type_ids()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.iter_node_ids_and_node_type_ids().collect::<Vec<_>>();
            
    }
    

    84 => {
        trace.push(format!("iter_one_hot_encoded_node_type_ids()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.iter_one_hot_encoded_node_type_ids().map(|x| x.collect::<Vec<_>>());
            
    }
    

    85 => {
        trace.push(format!("iter_one_hot_encoded_known_node_type_ids()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.iter_one_hot_encoded_known_node_type_ids().map(|x| x.collect::<Vec<_>>());
            
    }
    

    86 => {
        trace.push(format!("iter_node_names_and_node_type_names()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.iter_node_names_and_node_type_names().collect::<Vec<_>>();
            
    }
    

    87 => {
        trace.push(format!("par_iter_node_names_and_node_type_names()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.par_iter_node_names_and_node_type_names().collect::<Vec<_>>();
            
    }
    

    88 => {
        trace.push(format!("iter_edge_node_ids(directed: {:?})", &data.iteredgenodeids.directed));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.iter_edge_node_ids(data.iteredgenodeids.directed.clone()).collect::<Vec<_>>();
            
    }
    

    89 => {
        trace.push(format!("iter_edges(directed: {:?})", &data.iteredges.directed));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.iter_edges(data.iteredges.directed.clone()).collect::<Vec<_>>();
            
    }
    

    90 => {
        trace.push(format!("par_iter_edge_node_ids(directed: {:?})", &data.pariteredgenodeids.directed));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.par_iter_edge_node_ids(data.pariteredgenodeids.directed.clone()).collect::<Vec<_>>();
            
    }
    

    91 => {
        trace.push(format!("par_iter_directed_edge_ids()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.par_iter_directed_edge_ids().collect::<Vec<_>>();
            
    }
    

    92 => {
        trace.push(format!("par_iter_edges(directed: {:?})", &data.pariteredges.directed));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.par_iter_edges(data.pariteredges.directed.clone()).collect::<Vec<_>>();
            
    }
    

    93 => {
        trace.push(format!("iter_edge_node_ids_and_edge_weight()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.iter_edge_node_ids_and_edge_weight().map(|x| x.collect::<Vec<_>>());
            
    }
    

    94 => {
        trace.push(format!("par_iter_edge_node_ids_and_edge_weight()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.par_iter_edge_node_ids_and_edge_weight().map(|x| x.collect::<Vec<_>>());
            
    }
    

    95 => {
        trace.push(format!("iter_edge_node_ids_and_edge_type_id(directed: {:?})", &data.iteredgenodeidsandedgetypeid.directed));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.iter_edge_node_ids_and_edge_type_id(data.iteredgenodeidsandedgetypeid.directed.clone()).collect::<Vec<_>>();
            
    }
    

    96 => {
        trace.push(format!("iter_one_hot_encoded_edge_type_ids()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.iter_one_hot_encoded_edge_type_ids().map(|x| x.collect::<Vec<_>>());
            
    }
    

    97 => {
        trace.push(format!("iter_one_hot_encoded_known_edge_type_ids()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.iter_one_hot_encoded_known_edge_type_ids().map(|x| x.collect::<Vec<_>>());
            
    }
    

    98 => {
        trace.push(format!("iter_edge_node_names_and_edge_type_name(directed: {:?})", &data.iteredgenodenamesandedgetypename.directed));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.iter_edge_node_names_and_edge_type_name(data.iteredgenodenamesandedgetypename.directed.clone());
    }
    

    99 => {
        trace.push(format!("par_iter_edge_node_names_and_edge_type_name(directed: {:?})", &data.pariteredgenodenamesandedgetypename.directed));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.par_iter_edge_node_names_and_edge_type_name(data.pariteredgenodenamesandedgetypename.directed.clone());
    }
    

    100 => {
        trace.push(format!("par_iter_edge_node_ids_and_edge_type_id(directed: {:?})", &data.pariteredgenodeidsandedgetypeid.directed));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.par_iter_edge_node_ids_and_edge_type_id(data.pariteredgenodeidsandedgetypeid.directed.clone()).collect::<Vec<_>>();
            
    }
    

    101 => {
        trace.push(format!("par_iter_edge_node_names_and_edge_type_name_and_edge_weight(directed: {:?})", &data.pariteredgenodenamesandedgetypenameandedgeweight.directed));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.par_iter_edge_node_names_and_edge_type_name_and_edge_weight(data.pariteredgenodenamesandedgetypenameandedgeweight.directed.clone());
    }
    

    102 => {
        trace.push(format!("iter_edge_node_names_and_edge_type_name_and_edge_weight(directed: {:?})", &data.iteredgenodenamesandedgetypenameandedgeweight.directed));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.iter_edge_node_names_and_edge_type_name_and_edge_weight(data.iteredgenodenamesandedgetypenameandedgeweight.directed.clone());
    }
    

    103 => {
        trace.push(format!("par_iter_edge_node_ids_and_edge_type_id_and_edge_weight(directed: {:?})", &data.pariteredgenodeidsandedgetypeidandedgeweight.directed));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.par_iter_edge_node_ids_and_edge_type_id_and_edge_weight(data.pariteredgenodeidsandedgetypeidandedgeweight.directed.clone()).collect::<Vec<_>>();
            
    }
    

    104 => {
        trace.push(format!("iter_edge_node_ids_and_edge_type_id_and_edge_weight(directed: {:?})", &data.iteredgenodeidsandedgetypeidandedgeweight.directed));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.iter_edge_node_ids_and_edge_type_id_and_edge_weight(data.iteredgenodeidsandedgetypeidandedgeweight.directed.clone()).collect::<Vec<_>>();
            
    }
    

    105 => {
        trace.push(format!("iter_unique_edge_node_ids(directed: {:?})", &data.iteruniqueedgenodeids.directed));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.iter_unique_edge_node_ids(data.iteruniqueedgenodeids.directed.clone()).collect::<Vec<_>>();
            
    }
    

    106 => {
        trace.push(format!("iter_unique_source_node_ids()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.iter_unique_source_node_ids().collect::<Vec<_>>();
            
    }
    

    107 => {
        trace.push(format!("iter_edge_ids_with_unknown_edge_types()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.iter_edge_ids_with_unknown_edge_types().map(|x| x.collect::<Vec<_>>());
            
    }
    

    108 => {
        trace.push(format!("iter_edge_ids_with_known_edge_types()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.iter_edge_ids_with_known_edge_types().map(|x| x.collect::<Vec<_>>());
            
    }
    

    109 => {
        trace.push(format!("iter_edge_node_ids_with_unknown_edge_types(directed: {:?})", &data.iteredgenodeidswithunknownedgetypes.directed));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.iter_edge_node_ids_with_unknown_edge_types(data.iteredgenodeidswithunknownedgetypes.directed.clone()).map(|x| x.collect::<Vec<_>>());
            
    }
    

    110 => {
        trace.push(format!("iter_edge_node_ids_with_known_edge_types(directed: {:?})", &data.iteredgenodeidswithknownedgetypes.directed));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.iter_edge_node_ids_with_known_edge_types(data.iteredgenodeidswithknownedgetypes.directed.clone()).map(|x| x.collect::<Vec<_>>());
            
    }
    

    111 => {
        trace.push(format!("iter_node_ids_with_unknown_node_types()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.iter_node_ids_with_unknown_node_types().map(|x| x.collect::<Vec<_>>());
            
    }
    

    112 => {
        trace.push(format!("iter_node_ids_with_known_node_types()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.iter_node_ids_with_known_node_types().map(|x| x.collect::<Vec<_>>());
            
    }
    

    113 => {
        trace.push(format!("iter_edge_node_names_with_unknown_edge_types(directed: {:?})", &data.iteredgenodenameswithunknownedgetypes.directed));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.iter_edge_node_names_with_unknown_edge_types(data.iteredgenodenameswithunknownedgetypes.directed.clone()).map(|x| x.collect::<Vec<_>>());
            
    }
    

    114 => {
        trace.push(format!("iter_edge_node_names_with_known_edge_types(directed: {:?})", &data.iteredgenodenameswithknownedgetypes.directed));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.iter_edge_node_names_with_known_edge_types(data.iteredgenodenameswithknownedgetypes.directed.clone()).map(|x| x.collect::<Vec<_>>());
            
    }
    

    115 => {
        trace.push(format!("iter_node_names_with_unknown_node_types()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.iter_node_names_with_unknown_node_types().map(|x| x.collect::<Vec<_>>());
            
    }
    

    116 => {
        trace.push(format!("iter_node_names_with_known_node_types()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.iter_node_names_with_known_node_types().map(|x| x.collect::<Vec<_>>());
            
    }
    

    117 => {
        trace.push(format!("par_iter_edge_ids_with_unknown_edge_types()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.par_iter_edge_ids_with_unknown_edge_types().map(|x| x.collect::<Vec<_>>());
            
    }
    

    118 => {
        trace.push(format!("par_iter_edge_ids_with_known_edge_types()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.par_iter_edge_ids_with_known_edge_types().map(|x| x.collect::<Vec<_>>());
            
    }
    

    119 => {
        trace.push(format!("par_iter_edge_node_ids_with_unknown_edge_types(directed: {:?})", &data.pariteredgenodeidswithunknownedgetypes.directed));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.par_iter_edge_node_ids_with_unknown_edge_types(data.pariteredgenodeidswithunknownedgetypes.directed.clone()).map(|x| x.collect::<Vec<_>>());
            
    }
    

    120 => {
        trace.push(format!("par_iter_edge_node_ids_with_known_edge_types(directed: {:?})", &data.pariteredgenodeidswithknownedgetypes.directed));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.par_iter_edge_node_ids_with_known_edge_types(data.pariteredgenodeidswithknownedgetypes.directed.clone()).map(|x| x.collect::<Vec<_>>());
            
    }
    

    121 => {
        trace.push(format!("par_iter_node_ids_with_unknown_node_types()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.par_iter_node_ids_with_unknown_node_types().map(|x| x.collect::<Vec<_>>());
            
    }
    

    122 => {
        trace.push(format!("par_iter_node_ids_with_known_node_types()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.par_iter_node_ids_with_known_node_types().map(|x| x.collect::<Vec<_>>());
            
    }
    

    123 => {
        trace.push(format!("par_iter_edge_node_names_with_unknown_edge_types(directed: {:?})", &data.pariteredgenodenameswithunknownedgetypes.directed));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.par_iter_edge_node_names_with_unknown_edge_types(data.pariteredgenodenameswithunknownedgetypes.directed.clone()).map(|x| x.collect::<Vec<_>>());
            
    }
    

    124 => {
        trace.push(format!("par_iter_edge_node_names_with_known_edge_types(directed: {:?})", &data.pariteredgenodenameswithknownedgetypes.directed));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.par_iter_edge_node_names_with_known_edge_types(data.pariteredgenodenameswithknownedgetypes.directed.clone()).map(|x| x.collect::<Vec<_>>());
            
    }
    

    125 => {
        trace.push(format!("par_iter_node_names_with_unknown_node_types()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.par_iter_node_names_with_unknown_node_types().map(|x| x.collect::<Vec<_>>());
            
    }
    

    126 => {
        trace.push(format!("par_iter_node_names_with_known_node_types()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.par_iter_node_names_with_known_node_types().map(|x| x.collect::<Vec<_>>());
            
    }
    

    127 => {
        trace.push(format!("remove_inplace_node_type_ids(node_type_ids_to_remove: {:?})", &data.removeinplacenodetypeids.node_type_ids_to_remove));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.remove_inplace_node_type_ids(data.removeinplacenodetypeids.node_type_ids_to_remove.clone());
    }
    

    128 => {
        trace.push(format!("remove_inplace_singleton_node_types()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.remove_inplace_singleton_node_types();
    }
    

    129 => {
        trace.push(format!("remove_inplace_edge_type_ids(edge_type_ids_to_remove: {:?})", &data.removeinplaceedgetypeids.edge_type_ids_to_remove));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.remove_inplace_edge_type_ids(data.removeinplaceedgetypeids.edge_type_ids_to_remove.clone());
    }
    

    130 => {
        trace.push(format!("remove_inplace_singleton_edge_types()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.remove_inplace_singleton_edge_types();
    }
    

    131 => {
        trace.push(format!("remove_node_type_id(node_type_id: {:?})", &data.removenodetypeid.node_type_id));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
        if let Ok(res) = graph.remove_node_type_id(data.removenodetypeid.node_type_id.clone()) {
            graph = res;
        }
        
    }
    

    132 => {
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
    

    133 => {
        trace.push(format!("remove_edge_type_id(edge_type_id: {:?})", &data.removeedgetypeid.edge_type_id));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
        if let Ok(res) = graph.remove_edge_type_id(data.removeedgetypeid.edge_type_id.clone()) {
            graph = res;
        }
        
    }
    

    134 => {
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
    

    135 => {
        trace.push(format!("remove_inplace_node_types()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.remove_inplace_node_types();
    }
    

    136 => {
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
    

    137 => {
        trace.push(format!("remove_inplace_edge_types()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.remove_inplace_edge_types();
    }
    

    138 => {
        trace.push(format!("remove_edge_types(verbose: {:?})", &data.removeedgetypes.verbose));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
        if let Ok(res) = graph.remove_edge_types(data.removeedgetypes.verbose.clone()) {
            graph = res;
        }
        
    }
    

    139 => {
        trace.push(format!("remove_inplace_edge_weights()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.remove_inplace_edge_weights();
    }
    

    140 => {
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
    

    141 => {
        trace.push(format!("get_unweighted_number_of_triangles(normalize: {:?}, low_centrality: {:?}, verbose: {:?})", &data.getunweightednumberoftriangles.normalize, &data.getunweightednumberoftriangles.low_centrality, &data.getunweightednumberoftriangles.verbose));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_unweighted_number_of_triangles(data.getunweightednumberoftriangles.normalize.clone(), data.getunweightednumberoftriangles.low_centrality.clone(), data.getunweightednumberoftriangles.verbose.clone());
    }
    

    142 => {
        trace.push(format!("get_unweighted_triads_number()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_unweighted_triads_number();
    }
    

    143 => {
        trace.push(format!("get_weighted_triads_number()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_weighted_triads_number();
    }
    

    144 => {
        trace.push(format!("get_unweighted_transitivity(low_centrality: {:?}, verbose: {:?})", &data.getunweightedtransitivity.low_centrality, &data.getunweightedtransitivity.verbose));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_unweighted_transitivity(data.getunweightedtransitivity.low_centrality.clone(), data.getunweightedtransitivity.verbose.clone());
    }
    

    145 => {
        trace.push(format!("get_unweighted_number_of_triangles_per_node(normalize: {:?}, low_centrality: {:?}, verbose: {:?})", &data.getunweightednumberoftrianglespernode.normalize, &data.getunweightednumberoftrianglespernode.low_centrality, &data.getunweightednumberoftrianglespernode.verbose));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_unweighted_number_of_triangles_per_node(data.getunweightednumberoftrianglespernode.normalize.clone(), data.getunweightednumberoftrianglespernode.low_centrality.clone(), data.getunweightednumberoftrianglespernode.verbose.clone());
    }
    

    146 => {
        trace.push(format!("iter_clustering_coefficient_per_node(low_centrality: {:?}, verbose: {:?})", &data.iterclusteringcoefficientpernode.low_centrality, &data.iterclusteringcoefficientpernode.verbose));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.iter_clustering_coefficient_per_node(data.iterclusteringcoefficientpernode.low_centrality.clone(), data.iterclusteringcoefficientpernode.verbose.clone()).collect::<Vec<_>>();
            
    }
    

    147 => {
        trace.push(format!("get_clustering_coefficient_per_node(low_centrality: {:?}, verbose: {:?})", &data.getclusteringcoefficientpernode.low_centrality, &data.getclusteringcoefficientpernode.verbose));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_clustering_coefficient_per_node(data.getclusteringcoefficientpernode.low_centrality.clone(), data.getclusteringcoefficientpernode.verbose.clone());
    }
    

    148 => {
        trace.push(format!("get_clustering_coefficient(low_centrality: {:?}, verbose: {:?})", &data.getclusteringcoefficient.low_centrality, &data.getclusteringcoefficient.verbose));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_clustering_coefficient(data.getclusteringcoefficient.low_centrality.clone(), data.getclusteringcoefficient.verbose.clone());
    }
    

    149 => {
        trace.push(format!("get_average_clustering_coefficient(low_centrality: {:?}, verbose: {:?})", &data.getaverageclusteringcoefficient.low_centrality, &data.getaverageclusteringcoefficient.verbose));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_average_clustering_coefficient(data.getaverageclusteringcoefficient.low_centrality.clone(), data.getaverageclusteringcoefficient.verbose.clone());
    }
    

    150 => {
        trace.push(format!("remap_from_node_ids(node_ids: {:?}, verbose: {:?})", &data.remapfromnodeids.node_ids, &data.remapfromnodeids.verbose));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
        if let Ok(res) = graph.remap_from_node_ids(data.remapfromnodeids.node_ids.clone(), data.remapfromnodeids.verbose.clone()) {
            graph = res;
        }
        
    }
    

    151 => {
        trace.push(format!("node_label_holdout(train_size: {:?}, use_stratification: {:?}, random_state: {:?})", &data.nodelabelholdout.train_size, &data.nodelabelholdout.use_stratification, &data.nodelabelholdout.random_state));
    
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
    

    152 => {
        trace.push(format!("edge_label_holdout(train_size: {:?}, use_stratification: {:?}, random_state: {:?})", &data.edgelabelholdout.train_size, &data.edgelabelholdout.use_stratification, &data.edgelabelholdout.random_state));
    
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
    

    153 => {
        trace.push(format!("random_subgraph(nodes_number: {:?}, random_state: {:?}, verbose: {:?})", &data.randomsubgraph.nodes_number, &data.randomsubgraph.random_state, &data.randomsubgraph.verbose));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
        if let Ok(res) = graph.random_subgraph(data.randomsubgraph.nodes_number.clone(), data.randomsubgraph.random_state.clone(), data.randomsubgraph.verbose.clone()) {
            graph = res;
        }
        
    }
    

    154 => {
        trace.push(format!("get_unweighted_minimum_path_node_ids_from_node_ids(src_node_id: {:?}, dst_node_id: {:?}, maximal_depth: {:?})", &data.getunweightedminimumpathnodeidsfromnodeids.src_node_id, &data.getunweightedminimumpathnodeidsfromnodeids.dst_node_id, &data.getunweightedminimumpathnodeidsfromnodeids.maximal_depth));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_unweighted_minimum_path_node_ids_from_node_ids(data.getunweightedminimumpathnodeidsfromnodeids.src_node_id.clone(), data.getunweightedminimumpathnodeidsfromnodeids.dst_node_id.clone(), data.getunweightedminimumpathnodeidsfromnodeids.maximal_depth.clone());
    }
    

    155 => {
        trace.push(format!("get_unweighted_k_shortest_path_node_ids_from_node_ids(src_node_id: {:?}, dst_node_id: {:?}, k: {:?})", &data.getunweightedkshortestpathnodeidsfromnodeids.src_node_id, &data.getunweightedkshortestpathnodeidsfromnodeids.dst_node_id, &data.getunweightedkshortestpathnodeidsfromnodeids.k as usize));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_unweighted_k_shortest_path_node_ids_from_node_ids(data.getunweightedkshortestpathnodeidsfromnodeids.src_node_id.clone(), data.getunweightedkshortestpathnodeidsfromnodeids.dst_node_id.clone(), data.getunweightedkshortestpathnodeidsfromnodeids.k as usize.clone());
    }
    

    156 => {
        trace.push(format!("get_unweighted_eccentricity_from_node_id(node_id: {:?})", &data.getunweightedeccentricityfromnodeid.node_id));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_unweighted_eccentricity_from_node_id(data.getunweightedeccentricityfromnodeid.node_id.clone());
    }
    

    157 => {
        trace.push(format!("get_weighted_eccentricity_from_node_id(node_id: {:?}, use_edge_weights_as_probabilities: {:?})", &data.getweightedeccentricityfromnodeid.node_id, &data.getweightedeccentricityfromnodeid.use_edge_weights_as_probabilities));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_weighted_eccentricity_from_node_id(data.getweightedeccentricityfromnodeid.node_id.clone(), data.getweightedeccentricityfromnodeid.use_edge_weights_as_probabilities.clone());
    }
    

    158 => {
        trace.push(format!("get_weighted_minimum_path_node_ids_from_node_ids(src_node_id: {:?}, dst_node_id: {:?}, use_edge_weights_as_probabilities: {:?}, maximal_depth: {:?})", &data.getweightedminimumpathnodeidsfromnodeids.src_node_id, &data.getweightedminimumpathnodeidsfromnodeids.dst_node_id, &data.getweightedminimumpathnodeidsfromnodeids.use_edge_weights_as_probabilities, &data.getweightedminimumpathnodeidsfromnodeids.maximal_depth));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_weighted_minimum_path_node_ids_from_node_ids(data.getweightedminimumpathnodeidsfromnodeids.src_node_id.clone(), data.getweightedminimumpathnodeidsfromnodeids.dst_node_id.clone(), data.getweightedminimumpathnodeidsfromnodeids.use_edge_weights_as_probabilities.clone(), data.getweightedminimumpathnodeidsfromnodeids.maximal_depth.clone());
    }
    

    159 => {
        trace.push(format!("get_breath_first_search_from_node_ids(src_node_id: {:?}, maybe_dst_node_id: {:?}, maybe_dst_node_ids: {:?}, compute_distances: {:?}, compute_predecessors: {:?}, compute_visited: {:?}, maximal_depth: {:?})", &data.getbreathfirstsearchfromnodeids.src_node_id, &data.getbreathfirstsearchfromnodeids.maybe_dst_node_id, &data.getbreathfirstsearchfromnodeids.maybe_dst_node_ids, &data.getbreathfirstsearchfromnodeids.compute_distances, &data.getbreathfirstsearchfromnodeids.compute_predecessors, &data.getbreathfirstsearchfromnodeids.compute_visited, &data.getbreathfirstsearchfromnodeids.maximal_depth));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_breath_first_search_from_node_ids(data.getbreathfirstsearchfromnodeids.src_node_id.clone(), data.getbreathfirstsearchfromnodeids.maybe_dst_node_id.clone(), data.getbreathfirstsearchfromnodeids.maybe_dst_node_ids.clone(), data.getbreathfirstsearchfromnodeids.compute_distances.clone(), data.getbreathfirstsearchfromnodeids.compute_predecessors.clone(), data.getbreathfirstsearchfromnodeids.compute_visited.clone(), data.getbreathfirstsearchfromnodeids.maximal_depth.clone());
    }
    

    160 => {
        trace.push(format!("get_dijkstra_from_node_ids(src_node_id: {:?}, maybe_dst_node_id: {:?}, maybe_dst_node_ids: {:?}, compute_predecessors: {:?}, maximal_depth: {:?}, use_edge_weights_as_probabilities: {:?})", &data.getdijkstrafromnodeids.src_node_id, &data.getdijkstrafromnodeids.maybe_dst_node_id, &data.getdijkstrafromnodeids.maybe_dst_node_ids, &data.getdijkstrafromnodeids.compute_predecessors, &data.getdijkstrafromnodeids.maximal_depth, &data.getdijkstrafromnodeids.use_edge_weights_as_probabilities));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_dijkstra_from_node_ids(data.getdijkstrafromnodeids.src_node_id.clone(), data.getdijkstrafromnodeids.maybe_dst_node_id.clone(), data.getdijkstrafromnodeids.maybe_dst_node_ids.clone(), data.getdijkstrafromnodeids.compute_predecessors.clone(), data.getdijkstrafromnodeids.maximal_depth.clone(), data.getdijkstrafromnodeids.use_edge_weights_as_probabilities.clone());
    }
    

    161 => {
        trace.push(format!("get_unweighted_diameter(ignore_infinity: {:?}, verbose: {:?})", &data.getunweighteddiameter.ignore_infinity, &data.getunweighteddiameter.verbose));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_unweighted_diameter(data.getunweighteddiameter.ignore_infinity.clone(), data.getunweighteddiameter.verbose.clone());
    }
    

    162 => {
        trace.push(format!("get_weighted_diameter(ignore_infinity: {:?}, use_edge_weights_as_probabilities: {:?}, verbose: {:?})", &data.getweighteddiameter.ignore_infinity, &data.getweighteddiameter.use_edge_weights_as_probabilities, &data.getweighteddiameter.verbose));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_weighted_diameter(data.getweighteddiameter.ignore_infinity.clone(), data.getweighteddiameter.use_edge_weights_as_probabilities.clone(), data.getweighteddiameter.verbose.clone());
    }
    

    163 => {
        trace.push(format!("get_connected_components_number(verbose: {:?})", &data.getconnectedcomponentsnumber.verbose));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_connected_components_number(data.getconnectedcomponentsnumber.verbose.clone());
    }
    

    164 => {
        trace.push(format!("get_singleton_nodes_number()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_singleton_nodes_number();
    }
    

    165 => {
        trace.push(format!("get_weighted_singleton_nodes_number()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_weighted_singleton_nodes_number();
    }
    

    166 => {
        trace.push(format!("get_disconnected_nodes_number()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_disconnected_nodes_number();
    }
    

    167 => {
        trace.push(format!("get_singleton_node_ids()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_singleton_node_ids();
    }
    

    168 => {
        trace.push(format!("get_singleton_node_names()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_singleton_node_names();
    }
    

    169 => {
        trace.push(format!("get_singleton_nodes_with_selfloops_number()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_singleton_nodes_with_selfloops_number();
    }
    

    170 => {
        trace.push(format!("get_singleton_with_selfloops_node_ids()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_singleton_with_selfloops_node_ids();
    }
    

    171 => {
        trace.push(format!("get_singleton_with_selfloops_node_names()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_singleton_with_selfloops_node_names();
    }
    

    172 => {
        trace.push(format!("get_connected_nodes_number()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_connected_nodes_number();
    }
    

    173 => {
        trace.push(format!("get_density()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_density();
    }
    

    174 => {
        trace.push(format!("get_trap_nodes_rate()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_trap_nodes_rate();
    }
    

    175 => {
        trace.push(format!("get_unweighted_node_degrees_mean()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_unweighted_node_degrees_mean();
    }
    

    176 => {
        trace.push(format!("get_weighted_node_degrees_mean()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_weighted_node_degrees_mean();
    }
    

    177 => {
        trace.push(format!("get_undirected_edges_number()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_undirected_edges_number();
    }
    

    178 => {
        trace.push(format!("get_unique_undirected_edges_number()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_unique_undirected_edges_number();
    }
    

    179 => {
        trace.push(format!("get_edges_number()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_edges_number();
    }
    

    180 => {
        trace.push(format!("get_unique_edges_number()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_unique_edges_number();
    }
    

    181 => {
        trace.push(format!("get_unweighted_node_degrees_median()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_unweighted_node_degrees_median();
    }
    

    182 => {
        trace.push(format!("get_weighted_node_degrees_median()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_weighted_node_degrees_median();
    }
    

    183 => {
        trace.push(format!("get_weighted_maximum_node_degree()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_weighted_maximum_node_degree();
    }
    

    184 => {
        trace.push(format!("get_weighted_minimum_node_degree()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_weighted_minimum_node_degree();
    }
    

    185 => {
        trace.push(format!("get_unweighted_maximum_node_degree()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_unweighted_maximum_node_degree();
    }
    

    186 => {
        trace.push(format!("get_argmax_node_degree()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_argmax_node_degree();
    }
    

    187 => {
        trace.push(format!("get_weighted_mininum_node_degree()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_weighted_mininum_node_degree();
    }
    

    188 => {
        trace.push(format!("get_unweighted_minimum_node_degree()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_unweighted_minimum_node_degree();
    }
    

    189 => {
        trace.push(format!("get_unweighted_node_degrees_mode()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_unweighted_node_degrees_mode();
    }
    

    190 => {
        trace.push(format!("get_selfloop_number()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_selfloop_number();
    }
    

    191 => {
        trace.push(format!("get_unique_selfloop_number()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_unique_selfloop_number();
    }
    

    192 => {
        trace.push(format!("get_selfloop_nodes_rate()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_selfloop_nodes_rate();
    }
    

    193 => {
        trace.push(format!("get_name()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_name();
    }
    

    194 => {
        trace.push(format!("get_trap_nodes_number()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_trap_nodes_number();
    }
    

    195 => {
        trace.push(format!("get_source_node_ids(directed: {:?})", &data.getsourcenodeids.directed));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_source_node_ids(data.getsourcenodeids.directed.clone());
    }
    

    196 => {
        trace.push(format!("get_source_names(directed: {:?})", &data.getsourcenames.directed));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_source_names(data.getsourcenames.directed.clone());
    }
    

    197 => {
        trace.push(format!("get_destination_node_ids(directed: {:?})", &data.getdestinationnodeids.directed));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_destination_node_ids(data.getdestinationnodeids.directed.clone());
    }
    

    198 => {
        trace.push(format!("get_destination_names(directed: {:?})", &data.getdestinationnames.directed));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_destination_names(data.getdestinationnames.directed.clone());
    }
    

    199 => {
        trace.push(format!("get_node_names()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_node_names();
    }
    

    200 => {
        trace.push(format!("get_node_ids()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_node_ids();
    }
    

    201 => {
        trace.push(format!("get_edge_type_ids()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_edge_type_ids();
    }
    

    202 => {
        trace.push(format!("get_unique_edge_type_ids()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_unique_edge_type_ids();
    }
    

    203 => {
        trace.push(format!("get_edge_type_names()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_edge_type_names();
    }
    

    204 => {
        trace.push(format!("get_unique_edge_type_names()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_unique_edge_type_names();
    }
    

    205 => {
        trace.push(format!("get_edge_weights()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_edge_weights();
    }
    

    206 => {
        trace.push(format!("get_total_edge_weights()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_total_edge_weights();
    }
    

    207 => {
        trace.push(format!("get_mininum_edge_weight()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_mininum_edge_weight();
    }
    

    208 => {
        trace.push(format!("get_maximum_edge_weight()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_maximum_edge_weight();
    }
    

    209 => {
        trace.push(format!("get_node_type_ids()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_node_type_ids();
    }
    

    210 => {
        trace.push(format!("get_one_hot_encoded_node_types()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_one_hot_encoded_node_types();
    }
    

    211 => {
        trace.push(format!("get_one_hot_encoded_known_node_types()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_one_hot_encoded_known_node_types();
    }
    

    212 => {
        trace.push(format!("get_one_hot_encoded_edge_types()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_one_hot_encoded_edge_types();
    }
    

    213 => {
        trace.push(format!("get_one_hot_encoded_known_edge_types()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_one_hot_encoded_known_edge_types();
    }
    

    214 => {
        trace.push(format!("get_node_type_names()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_node_type_names();
    }
    

    215 => {
        trace.push(format!("get_unique_node_type_ids()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_unique_node_type_ids();
    }
    

    216 => {
        trace.push(format!("get_unique_node_type_names()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_unique_node_type_names();
    }
    

    217 => {
        trace.push(format!("get_unique_directed_edges_number()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_unique_directed_edges_number();
    }
    

    218 => {
        trace.push(format!("get_nodes_mapping()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_nodes_mapping();
    }
    

    219 => {
        trace.push(format!("get_edge_node_ids(directed: {:?})", &data.getedgenodeids.directed));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_edge_node_ids(data.getedgenodeids.directed.clone());
    }
    

    220 => {
        trace.push(format!("get_edge_node_names(directed: {:?})", &data.getedgenodenames.directed));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_edge_node_names(data.getedgenodenames.directed.clone());
    }
    

    221 => {
        trace.push(format!("get_unknown_node_types_number()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_unknown_node_types_number();
    }
    

    222 => {
        trace.push(format!("get_known_node_types_number()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_known_node_types_number();
    }
    

    223 => {
        trace.push(format!("get_unknown_node_types_rate()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_unknown_node_types_rate();
    }
    

    224 => {
        trace.push(format!("get_known_node_types_rate()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_known_node_types_rate();
    }
    

    225 => {
        trace.push(format!("get_minimum_node_types_number()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_minimum_node_types_number();
    }
    

    226 => {
        trace.push(format!("get_maximum_node_types_number()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_maximum_node_types_number();
    }
    

    227 => {
        trace.push(format!("get_maximum_multilabel_count()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_maximum_multilabel_count();
    }
    

    228 => {
        trace.push(format!("get_singleton_node_types_number()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_singleton_node_types_number();
    }
    

    229 => {
        trace.push(format!("get_singleton_node_type_ids()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_singleton_node_type_ids();
    }
    

    230 => {
        trace.push(format!("get_singleton_node_type_names()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_singleton_node_type_names();
    }
    

    231 => {
        trace.push(format!("get_unknown_edge_types_number()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_unknown_edge_types_number();
    }
    

    232 => {
        trace.push(format!("get_edge_ids_with_unknown_edge_types()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_edge_ids_with_unknown_edge_types();
    }
    

    233 => {
        trace.push(format!("get_edge_ids_with_known_edge_types()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_edge_ids_with_known_edge_types();
    }
    

    234 => {
        trace.push(format!("get_edge_node_ids_with_unknown_edge_types(directed: {:?})", &data.getedgenodeidswithunknownedgetypes.directed));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_edge_node_ids_with_unknown_edge_types(data.getedgenodeidswithunknownedgetypes.directed.clone());
    }
    

    235 => {
        trace.push(format!("get_edge_node_ids_with_known_edge_types(directed: {:?})", &data.getedgenodeidswithknownedgetypes.directed));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_edge_node_ids_with_known_edge_types(data.getedgenodeidswithknownedgetypes.directed.clone());
    }
    

    236 => {
        trace.push(format!("get_edge_node_names_with_unknown_edge_types(directed: {:?})", &data.getedgenodenameswithunknownedgetypes.directed));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_edge_node_names_with_unknown_edge_types(data.getedgenodenameswithunknownedgetypes.directed.clone());
    }
    

    237 => {
        trace.push(format!("get_edge_node_names_with_known_edge_types(directed: {:?})", &data.getedgenodenameswithknownedgetypes.directed));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_edge_node_names_with_known_edge_types(data.getedgenodenameswithknownedgetypes.directed.clone());
    }
    

    238 => {
        trace.push(format!("get_edge_ids_with_unknown_edge_types_mask()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_edge_ids_with_unknown_edge_types_mask();
    }
    

    239 => {
        trace.push(format!("get_edge_ids_with_known_edge_types_mask()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_edge_ids_with_known_edge_types_mask();
    }
    

    240 => {
        trace.push(format!("get_node_ids_with_unknown_node_types()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_node_ids_with_unknown_node_types();
    }
    

    241 => {
        trace.push(format!("get_node_ids_with_known_node_types()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_node_ids_with_known_node_types();
    }
    

    242 => {
        trace.push(format!("get_node_names_with_unknown_node_types()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_node_names_with_unknown_node_types();
    }
    

    243 => {
        trace.push(format!("get_node_names_with_known_node_types()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_node_names_with_known_node_types();
    }
    

    244 => {
        trace.push(format!("get_node_ids_with_unknown_node_types_mask()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_node_ids_with_unknown_node_types_mask();
    }
    

    245 => {
        trace.push(format!("get_node_ids_with_known_node_types_mask()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_node_ids_with_known_node_types_mask();
    }
    

    246 => {
        trace.push(format!("get_known_edge_types_number()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_known_edge_types_number();
    }
    

    247 => {
        trace.push(format!("get_unknown_edge_types_rate()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_unknown_edge_types_rate();
    }
    

    248 => {
        trace.push(format!("get_known_edge_types_rate()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_known_edge_types_rate();
    }
    

    249 => {
        trace.push(format!("get_minimum_edge_types_number()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_minimum_edge_types_number();
    }
    

    250 => {
        trace.push(format!("get_singleton_edge_types_number()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_singleton_edge_types_number();
    }
    

    251 => {
        trace.push(format!("get_singleton_edge_type_ids()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_singleton_edge_type_ids();
    }
    

    252 => {
        trace.push(format!("get_singleton_edge_type_names()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_singleton_edge_type_names();
    }
    

    253 => {
        trace.push(format!("get_nodes_number()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_nodes_number();
    }
    

    254 => {
        trace.push(format!("get_node_connected_component_ids(verbose: {:?})", &data.getnodeconnectedcomponentids.verbose));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_node_connected_component_ids(data.getnodeconnectedcomponentids.verbose.clone());
    }
    

    255 => {
        trace.push(format!("get_directed_edges_number()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_directed_edges_number();
    }
    

    256 => {
        trace.push(format!("get_edge_types_number()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_edge_types_number();
    }
    

    257 => {
        trace.push(format!("get_node_types_number()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_node_types_number();
    }
    

    258 => {
        trace.push(format!("get_unweighted_node_degrees()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_unweighted_node_degrees();
    }
    

    259 => {
        trace.push(format!("get_weighted_node_degrees()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_weighted_node_degrees();
    }
    

    260 => {
        trace.push(format!("get_not_singletons_node_ids()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_not_singletons_node_ids();
    }
    

    261 => {
        trace.push(format!("get_dense_nodes_mapping()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_dense_nodes_mapping();
    }
    

    262 => {
        trace.push(format!("get_parallel_edges_number()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_parallel_edges_number();
    }
    

    263 => {
        trace.push(format!("get_cumulative_node_degrees()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_cumulative_node_degrees();
    }
    

    264 => {
        trace.push(format!("get_unique_source_nodes_number()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_unique_source_nodes_number();
    }
    

    265 => {
        trace.push(format!("get_edge_type_id_counts_hashmap()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_edge_type_id_counts_hashmap();
    }
    

    266 => {
        trace.push(format!("get_edge_type_names_counts_hashmap()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_edge_type_names_counts_hashmap();
    }
    

    267 => {
        trace.push(format!("get_node_type_id_counts_hashmap()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_node_type_id_counts_hashmap();
    }
    

    268 => {
        trace.push(format!("get_node_type_names_counts_hashmap()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_node_type_names_counts_hashmap();
    }
    

    269 => {
        trace.push(format!("to_directed_inplace()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.to_directed_inplace();
    }
    

    270 => {
        trace.push(format!("to_directed()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        graph = graph.to_directed();
    }
    

    271 => {
        trace.push(format!("to_upper_triangular(verbose: {:?})", &data.touppertriangular.verbose));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        graph = graph.to_upper_triangular(data.touppertriangular.verbose.clone());
    }
    

    272 => {
        trace.push(format!("to_lower_triangular(verbose: {:?})", &data.tolowertriangular.verbose));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        graph = graph.to_lower_triangular(data.tolowertriangular.verbose.clone());
    }
    

    273 => {
        trace.push(format!("to_main_diagonal(verbose: {:?})", &data.tomaindiagonal.verbose));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        graph = graph.to_main_diagonal(data.tomaindiagonal.verbose.clone());
    }
    

    274 => {
        trace.push(format!("to_anti_diagonal(verbose: {:?})", &data.toantidiagonal.verbose));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        graph = graph.to_anti_diagonal(data.toantidiagonal.verbose.clone());
    }
    

    275 => {
        trace.push(format!("to_bidiagonal(verbose: {:?})", &data.tobidiagonal.verbose));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        graph = graph.to_bidiagonal(data.tobidiagonal.verbose.clone());
    }
    

    276 => {
        trace.push(format!("to_arrowhead(verbose: {:?})", &data.toarrowhead.verbose));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        graph = graph.to_arrowhead(data.toarrowhead.verbose.clone());
    }
    

    277 => {
        trace.push(format!("to_transposed(verbose: {:?})", &data.totransposed.verbose));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        graph = graph.to_transposed(data.totransposed.verbose.clone());
    }
    

    278 => {
        trace.push(format!("to_complementary(verbose: {:?})", &data.tocomplementary.verbose));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        graph = graph.to_complementary(data.tocomplementary.verbose.clone());
    }
    

    279 => {
        trace.push(format!("report()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.report();
    }
    

    280 => {
        trace.push(format!("get_node_report_from_node_id(node_id: {:?})", &data.getnodereportfromnodeid.node_id));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_node_report_from_node_id(data.getnodereportfromnodeid.node_id.clone());
    }
    

    281 => {
        trace.push(format!("textual_report()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.textual_report();
    }
    

    282 => {
        trace.push(format!("filter_from_ids(node_ids_to_keep: {:?}, node_ids_to_filter: {:?}, node_type_ids_to_keep: {:?}, node_type_ids_to_filter: {:?}, node_type_id_to_keep: {:?}, node_type_id_to_filter: {:?}, edge_ids_to_keep: {:?}, edge_ids_to_filter: {:?}, edge_node_ids_to_keep: {:?}, edge_node_ids_to_filter: {:?}, edge_type_ids_to_keep: {:?}, edge_type_ids_to_filter: {:?}, min_edge_weight: {:?}, max_edge_weight: {:?}, filter_singleton_nodes: {:?}, filter_singleton_nodes_with_selfloop: {:?}, filter_selfloops: {:?}, filter_parallel_edges: {:?}, verbose: {:?})", &data.filterfromids.node_ids_to_keep, &data.filterfromids.node_ids_to_filter, &data.filterfromids.node_type_ids_to_keep, &data.filterfromids.node_type_ids_to_filter, &data.filterfromids.node_type_id_to_keep, &data.filterfromids.node_type_id_to_filter, &data.filterfromids.edge_ids_to_keep, &data.filterfromids.edge_ids_to_filter, &data.filterfromids.edge_node_ids_to_keep, &data.filterfromids.edge_node_ids_to_filter, &data.filterfromids.edge_type_ids_to_keep, &data.filterfromids.edge_type_ids_to_filter, &data.filterfromids.min_edge_weight, &data.filterfromids.max_edge_weight, &data.filterfromids.filter_singleton_nodes, &data.filterfromids.filter_singleton_nodes_with_selfloop, &data.filterfromids.filter_selfloops, &data.filterfromids.filter_parallel_edges, &data.filterfromids.verbose));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
        if let Ok(res) = graph.filter_from_ids(data.filterfromids.node_ids_to_keep.clone(), data.filterfromids.node_ids_to_filter.clone(), data.filterfromids.node_type_ids_to_keep.clone(), data.filterfromids.node_type_ids_to_filter.clone(), data.filterfromids.node_type_id_to_keep.clone(), data.filterfromids.node_type_id_to_filter.clone(), data.filterfromids.edge_ids_to_keep.clone(), data.filterfromids.edge_ids_to_filter.clone(), data.filterfromids.edge_node_ids_to_keep.clone(), data.filterfromids.edge_node_ids_to_filter.clone(), data.filterfromids.edge_type_ids_to_keep.clone(), data.filterfromids.edge_type_ids_to_filter.clone(), data.filterfromids.min_edge_weight.clone(), data.filterfromids.max_edge_weight.clone(), data.filterfromids.filter_singleton_nodes.clone(), data.filterfromids.filter_singleton_nodes_with_selfloop.clone(), data.filterfromids.filter_selfloops.clone(), data.filterfromids.filter_parallel_edges.clone(), data.filterfromids.verbose.clone()) {
            graph = res;
        }
        
    }
    

    283 => {
        trace.push(format!("drop_unknown_node_types(verbose: {:?})", &data.dropunknownnodetypes.verbose));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        graph = graph.drop_unknown_node_types(data.dropunknownnodetypes.verbose.clone());
    }
    

    284 => {
        trace.push(format!("drop_unknown_edge_types(verbose: {:?})", &data.dropunknownedgetypes.verbose));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        graph = graph.drop_unknown_edge_types(data.dropunknownedgetypes.verbose.clone());
    }
    

    285 => {
        trace.push(format!("drop_singleton_nodes(verbose: {:?})", &data.dropsingletonnodes.verbose));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        graph = graph.drop_singleton_nodes(data.dropsingletonnodes.verbose.clone());
    }
    

    286 => {
        trace.push(format!("drop_singleton_nodes_with_selfloops(verbose: {:?})", &data.dropsingletonnodeswithselfloops.verbose));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        graph = graph.drop_singleton_nodes_with_selfloops(data.dropsingletonnodeswithselfloops.verbose.clone());
    }
    

    287 => {
        trace.push(format!("drop_disconnected_nodes(verbose: {:?})", &data.dropdisconnectednodes.verbose));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        graph = graph.drop_disconnected_nodes(data.dropdisconnectednodes.verbose.clone());
    }
    

    288 => {
        trace.push(format!("drop_selfloops(verbose: {:?})", &data.dropselfloops.verbose));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        graph = graph.drop_selfloops(data.dropselfloops.verbose.clone());
    }
    

    289 => {
        trace.push(format!("drop_parallel_edges(verbose: {:?})", &data.dropparalleledges.verbose));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        graph = graph.drop_parallel_edges(data.dropparalleledges.verbose.clone());
    }
    

    290 => {
        trace.push(format!("spanning_arborescence_kruskal(verbose: {:?})", &data.spanningarborescencekruskal.verbose));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.spanning_arborescence_kruskal(data.spanningarborescencekruskal.verbose.clone());
    }
    

    291 => {
        trace.push(format!("spanning_arborescence(verbose: {:?})", &data.spanningarborescence.verbose));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.spanning_arborescence(data.spanningarborescence.verbose.clone());
    }
    

    292 => {
        trace.push(format!("connected_components(verbose: {:?})", &data.connectedcomponents.verbose));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.connected_components(data.connectedcomponents.verbose.clone());
    }
    

    293 => {
        trace.push(format!("enable(vector_sources: {:?}, vector_destinations: {:?}, vector_cumulative_node_degrees: {:?})", &data.enable.vector_sources, &data.enable.vector_destinations, &data.enable.vector_cumulative_node_degrees));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.enable(data.enable.vector_sources.clone(), data.enable.vector_destinations.clone(), data.enable.vector_cumulative_node_degrees.clone());
    }
    

    294 => {
        trace.push(format!("disable_all()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        graph.disable_all()
    }
    

    295 => {
        trace.push(format!("par_iter_approximated_vertex_cover()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.par_iter_approximated_vertex_cover().collect::<Vec<_>>();
            
    }
    

    296 => {
        trace.push(format!("approximated_vertex_cover_set()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.approximated_vertex_cover_set();
    }
    

    297 => {
        trace.push(format!("get_node_label_prediction_tuple_from_node_ids(node_ids: {:?}, random_state: {:?}, include_central_node: {:?}, return_edge_weights: {:?}, offset: {:?}, max_neighbours: {:?})", &data.getnodelabelpredictiontuplefromnodeids.node_ids, &data.getnodelabelpredictiontuplefromnodeids.random_state, &data.getnodelabelpredictiontuplefromnodeids.include_central_node, &data.getnodelabelpredictiontuplefromnodeids.return_edge_weights, &data.getnodelabelpredictiontuplefromnodeids.offset, &data.getnodelabelpredictiontuplefromnodeids.max_neighbours));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_node_label_prediction_tuple_from_node_ids(data.getnodelabelpredictiontuplefromnodeids.node_ids.clone(), data.getnodelabelpredictiontuplefromnodeids.random_state.clone(), data.getnodelabelpredictiontuplefromnodeids.include_central_node.clone(), data.getnodelabelpredictiontuplefromnodeids.return_edge_weights.clone(), data.getnodelabelpredictiontuplefromnodeids.offset.clone(), data.getnodelabelpredictiontuplefromnodeids.max_neighbours.clone());
    }
    

    298 => {
        trace.push(format!("par_iter_edge_prediction_metrics(normalize: {:?}, verbose: {:?})", &data.pariteredgepredictionmetrics.normalize, &data.pariteredgepredictionmetrics.verbose));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.par_iter_edge_prediction_metrics(data.pariteredgepredictionmetrics.normalize.clone(), data.pariteredgepredictionmetrics.verbose.clone()).collect::<Vec<_>>();
            
    }
    

    299 => {
        trace.push(format!("get_okapi_bm25_node_feature_propagation(features: {:?}, iterations: {:?}, maximal_distance: {:?}, k1: {:?}, b: {:?}, include_central_node: {:?}, verbose: {:?})", &data.getokapibm25nodefeaturepropagation.features, &data.getokapibm25nodefeaturepropagation.iterations.map(|x| x as usize), &data.getokapibm25nodefeaturepropagation.maximal_distance, &data.getokapibm25nodefeaturepropagation.k1, &data.getokapibm25nodefeaturepropagation.b, &data.getokapibm25nodefeaturepropagation.include_central_node, &data.getokapibm25nodefeaturepropagation.verbose));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_okapi_bm25_node_feature_propagation(data.getokapibm25nodefeaturepropagation.features.clone(), data.getokapibm25nodefeaturepropagation.iterations.map(|x| x as usize).clone(), data.getokapibm25nodefeaturepropagation.maximal_distance.clone(), data.getokapibm25nodefeaturepropagation.k1.clone(), data.getokapibm25nodefeaturepropagation.b.clone(), data.getokapibm25nodefeaturepropagation.include_central_node.clone(), data.getokapibm25nodefeaturepropagation.verbose.clone());
    }
    

    300 => {
        trace.push(format!("get_okapi_bm25_node_label_propagation(iterations: {:?}, maximal_distance: {:?}, k1: {:?}, b: {:?}, verbose: {:?})", &data.getokapibm25nodelabelpropagation.iterations.map(|x| x as usize), &data.getokapibm25nodelabelpropagation.maximal_distance, &data.getokapibm25nodelabelpropagation.k1, &data.getokapibm25nodelabelpropagation.b, &data.getokapibm25nodelabelpropagation.verbose));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_okapi_bm25_node_label_propagation(data.getokapibm25nodelabelpropagation.iterations.map(|x| x as usize).clone(), data.getokapibm25nodelabelpropagation.maximal_distance.clone(), data.getokapibm25nodelabelpropagation.k1.clone(), data.getokapibm25nodelabelpropagation.b.clone(), data.getokapibm25nodelabelpropagation.verbose.clone());
    }
    

    301 => {
        trace.push(format!("has_default_graph_name()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.has_default_graph_name();
    }
    

    302 => {
        trace.push(format!("has_nodes()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.has_nodes();
    }
    

    303 => {
        trace.push(format!("has_edges()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.has_edges();
    }
    

    304 => {
        trace.push(format!("has_trap_nodes()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.has_trap_nodes();
    }
    

    305 => {
        trace.push(format!("is_directed()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.is_directed();
    }
    

    306 => {
        trace.push(format!("has_edge_weights()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.has_edge_weights();
    }
    

    307 => {
        trace.push(format!("has_edge_weights_representing_probabilities()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.has_edge_weights_representing_probabilities();
    }
    

    308 => {
        trace.push(format!("has_weighted_singleton_nodes()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.has_weighted_singleton_nodes();
    }
    

    309 => {
        trace.push(format!("has_constant_edge_weights()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.has_constant_edge_weights();
    }
    

    310 => {
        trace.push(format!("has_negative_edge_weights()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.has_negative_edge_weights();
    }
    

    311 => {
        trace.push(format!("has_edge_types()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.has_edge_types();
    }
    

    312 => {
        trace.push(format!("has_selfloops()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.has_selfloops();
    }
    

    313 => {
        trace.push(format!("has_disconnected_nodes()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.has_disconnected_nodes();
    }
    

    314 => {
        trace.push(format!("has_singleton_nodes()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.has_singleton_nodes();
    }
    

    315 => {
        trace.push(format!("has_singleton_nodes_with_selfloops()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.has_singleton_nodes_with_selfloops();
    }
    

    316 => {
        trace.push(format!("is_connected(verbose: {:?})", &data.isconnected.verbose));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.is_connected(data.isconnected.verbose.clone());
    }
    

    317 => {
        trace.push(format!("has_node_types()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.has_node_types();
    }
    

    318 => {
        trace.push(format!("has_multilabel_node_types()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.has_multilabel_node_types();
    }
    

    319 => {
        trace.push(format!("has_unknown_node_types()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.has_unknown_node_types();
    }
    

    320 => {
        trace.push(format!("has_known_node_types()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.has_known_node_types();
    }
    

    321 => {
        trace.push(format!("has_unknown_edge_types()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.has_unknown_edge_types();
    }
    

    322 => {
        trace.push(format!("has_known_edge_types()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.has_known_edge_types();
    }
    

    323 => {
        trace.push(format!("has_homogeneous_node_types()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.has_homogeneous_node_types();
    }
    

    324 => {
        trace.push(format!("has_homogeneous_edge_types()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.has_homogeneous_edge_types();
    }
    

    325 => {
        trace.push(format!("has_singleton_node_types()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.has_singleton_node_types();
    }
    

    326 => {
        trace.push(format!("has_node_oddities()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.has_node_oddities();
    }
    

    327 => {
        trace.push(format!("has_node_types_oddities()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.has_node_types_oddities();
    }
    

    328 => {
        trace.push(format!("has_singleton_edge_types()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.has_singleton_edge_types();
    }
    

    329 => {
        trace.push(format!("has_edge_types_oddities()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.has_edge_types_oddities();
    }
    

    330 => {
        trace.push(format!("is_multigraph()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.is_multigraph();
    }
    

    331 => {
        trace.push(format!("has_nodes_sorted_by_decreasing_outbound_node_degree()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.has_nodes_sorted_by_decreasing_outbound_node_degree();
    }
    

    332 => {
        trace.push(format!("has_nodes_sorted_by_increasing_outbound_node_degree()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.has_nodes_sorted_by_increasing_outbound_node_degree();
    }
    

    333 => {
        trace.push(format!("get_transitive_closure(iterations: {:?}, verbose: {:?})", &data.gettransitiveclosure.iterations.map(|x| x as NodeT), &data.gettransitiveclosure.verbose));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        graph = graph.get_transitive_closure(data.gettransitiveclosure.iterations.map(|x| x as NodeT).clone(), data.gettransitiveclosure.verbose.clone());
    }
    

    334 => {
        trace.push(format!("get_unweighted_all_shortest_paths(iterations: {:?}, verbose: {:?})", &data.getunweightedallshortestpaths.iterations.map(|x| x as NodeT), &data.getunweightedallshortestpaths.verbose));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        graph = graph.get_unweighted_all_shortest_paths(data.getunweightedallshortestpaths.iterations.map(|x| x as NodeT).clone(), data.getunweightedallshortestpaths.verbose.clone());
    }
    

    335 => {
        trace.push(format!("get_weighted_all_shortest_paths(iterations: {:?}, use_edge_weights_as_probabilities: {:?}, verbose: {:?})", &data.getweightedallshortestpaths.iterations.map(|x| x as NodeT), &data.getweightedallshortestpaths.use_edge_weights_as_probabilities, &data.getweightedallshortestpaths.verbose));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
        if let Ok(res) = graph.get_weighted_all_shortest_paths(data.getweightedallshortestpaths.iterations.map(|x| x as NodeT).clone(), data.getweightedallshortestpaths.use_edge_weights_as_probabilities.clone(), data.getweightedallshortestpaths.verbose.clone()) {
            graph = res;
        }
        
    }
    

    336 => {
        trace.push(format!("get_source_node_id_from_edge_id(edge_id: {:?})", &data.getsourcenodeidfromedgeid.edge_id));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_source_node_id_from_edge_id(data.getsourcenodeidfromedgeid.edge_id.clone());
    }
    

    337 => {
        trace.push(format!("get_destination_node_id_from_edge_id(edge_id: {:?})", &data.getdestinationnodeidfromedgeid.edge_id));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_destination_node_id_from_edge_id(data.getdestinationnodeidfromedgeid.edge_id.clone());
    }
    

    338 => {
        trace.push(format!("get_source_node_name_from_edge_id(edge_id: {:?})", &data.getsourcenodenamefromedgeid.edge_id));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_source_node_name_from_edge_id(data.getsourcenodenamefromedgeid.edge_id.clone());
    }
    

    339 => {
        trace.push(format!("get_destination_node_name_from_edge_id(edge_id: {:?})", &data.getdestinationnodenamefromedgeid.edge_id));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_destination_node_name_from_edge_id(data.getdestinationnodenamefromedgeid.edge_id.clone());
    }
    

    340 => {
        trace.push(format!("get_node_names_from_edge_id(edge_id: {:?})", &data.getnodenamesfromedgeid.edge_id));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_node_names_from_edge_id(data.getnodenamesfromedgeid.edge_id.clone());
    }
    

    341 => {
        trace.push(format!("get_node_ids_from_edge_id(edge_id: {:?})", &data.getnodeidsfromedgeid.edge_id));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_node_ids_from_edge_id(data.getnodeidsfromedgeid.edge_id.clone());
    }
    

    342 => {
        trace.push(format!("get_edge_id_from_node_ids(src: {:?}, dst: {:?})", &data.getedgeidfromnodeids.src, &data.getedgeidfromnodeids.dst));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_edge_id_from_node_ids(data.getedgeidfromnodeids.src.clone(), data.getedgeidfromnodeids.dst.clone());
    }
    

    343 => {
        trace.push(format!("get_node_ids_and_edge_type_id_from_edge_id(edge_id: {:?})", &data.getnodeidsandedgetypeidfromedgeid.edge_id));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_node_ids_and_edge_type_id_from_edge_id(data.getnodeidsandedgetypeidfromedgeid.edge_id.clone());
    }
    

    344 => {
        trace.push(format!("get_node_ids_and_edge_type_id_and_edge_weight_from_edge_id(edge_id: {:?})", &data.getnodeidsandedgetypeidandedgeweightfromedgeid.edge_id));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_node_ids_and_edge_type_id_and_edge_weight_from_edge_id(data.getnodeidsandedgetypeidandedgeweightfromedgeid.edge_id.clone());
    }
    

    345 => {
        trace.push(format!("get_unweighted_top_k_central_node_ids(k: {:?})", &data.getunweightedtopkcentralnodeids.k));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_unweighted_top_k_central_node_ids(data.getunweightedtopkcentralnodeids.k.clone());
    }
    

    346 => {
        trace.push(format!("get_weighted_top_k_central_node_ids(k: {:?})", &data.getweightedtopkcentralnodeids.k));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_weighted_top_k_central_node_ids(data.getweightedtopkcentralnodeids.k.clone());
    }
    

    347 => {
        trace.push(format!("get_unweighted_node_degree_from_node_id(node_id: {:?})", &data.getunweightednodedegreefromnodeid.node_id));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_unweighted_node_degree_from_node_id(data.getunweightednodedegreefromnodeid.node_id.clone());
    }
    

    348 => {
        trace.push(format!("get_weighted_node_degree_from_node_id(node_id: {:?})", &data.getweightednodedegreefromnodeid.node_id));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_weighted_node_degree_from_node_id(data.getweightednodedegreefromnodeid.node_id.clone());
    }
    

    349 => {
        trace.push(format!("get_top_k_central_node_names(k: {:?})", &data.gettopkcentralnodenames.k));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_top_k_central_node_names(data.gettopkcentralnodenames.k.clone());
    }
    

    350 => {
        trace.push(format!("get_node_type_id_from_node_id(node_id: {:?})", &data.getnodetypeidfromnodeid.node_id));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_node_type_id_from_node_id(data.getnodetypeidfromnodeid.node_id.clone());
    }
    

    351 => {
        trace.push(format!("get_edge_type_id_from_edge_id(edge_id: {:?})", &data.getedgetypeidfromedgeid.edge_id));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_edge_type_id_from_edge_id(data.getedgetypeidfromedgeid.edge_id.clone());
    }
    

    352 => {
        trace.push(format!("get_node_type_names_from_node_id(node_id: {:?})", &data.getnodetypenamesfromnodeid.node_id));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_node_type_names_from_node_id(data.getnodetypenamesfromnodeid.node_id.clone());
    }
    

    353 => {
        trace.push(format!("get_edge_type_name_from_edge_id(edge_id: {:?})", &data.getedgetypenamefromedgeid.edge_id));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_edge_type_name_from_edge_id(data.getedgetypenamefromedgeid.edge_id.clone());
    }
    

    354 => {
        trace.push(format!("get_edge_type_name_from_edge_type_id(edge_type_id: {:?})", &data.getedgetypenamefromedgetypeid.edge_type_id));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_edge_type_name_from_edge_type_id(data.getedgetypenamefromedgetypeid.edge_type_id.clone());
    }
    

    355 => {
        trace.push(format!("get_edge_weight_from_edge_id(edge_id: {:?})", &data.getedgeweightfromedgeid.edge_id));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_edge_weight_from_edge_id(data.getedgeweightfromedgeid.edge_id.clone());
    }
    

    356 => {
        trace.push(format!("get_edge_weight_from_node_ids(src: {:?}, dst: {:?})", &data.getedgeweightfromnodeids.src, &data.getedgeweightfromnodeids.dst));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_edge_weight_from_node_ids(data.getedgeweightfromnodeids.src.clone(), data.getedgeweightfromnodeids.dst.clone());
    }
    

    357 => {
        trace.push(format!("get_edge_weight_from_node_ids_and_edge_type_id(src: {:?}, dst: {:?}, edge_type: {:?})", &data.getedgeweightfromnodeidsandedgetypeid.src, &data.getedgeweightfromnodeidsandedgetypeid.dst, &data.getedgeweightfromnodeidsandedgetypeid.edge_type));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_edge_weight_from_node_ids_and_edge_type_id(data.getedgeweightfromnodeidsandedgetypeid.src.clone(), data.getedgeweightfromnodeidsandedgetypeid.dst.clone(), data.getedgeweightfromnodeidsandedgetypeid.edge_type.clone());
    }
    

    358 => {
        trace.push(format!("get_node_name_from_node_id(node_id: {:?})", &data.getnodenamefromnodeid.node_id));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_node_name_from_node_id(data.getnodenamefromnodeid.node_id.clone());
    }
    

    359 => {
        trace.push(format!("get_edge_node_names_from_edge_node_ids(edge_node_ids: {:?})", &data.getedgenodenamesfromedgenodeids.edge_node_ids));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_edge_node_names_from_edge_node_ids(data.getedgenodenamesfromedgenodeids.edge_node_ids.clone());
    }
    

    360 => {
        trace.push(format!("get_edge_count_from_edge_type_id(edge_type_id: {:?})", &data.getedgecountfromedgetypeid.edge_type_id));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_edge_count_from_edge_type_id(data.getedgecountfromedgetypeid.edge_type_id.clone());
    }
    

    361 => {
        trace.push(format!("get_node_count_from_node_type_id(node_type_id: {:?})", &data.getnodecountfromnodetypeid.node_type_id));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_node_count_from_node_type_id(data.getnodecountfromnodetypeid.node_type_id.clone());
    }
    

    362 => {
        trace.push(format!("get_neighbour_node_ids_from_node_id(node_id: {:?})", &data.getneighbournodeidsfromnodeid.node_id));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_neighbour_node_ids_from_node_id(data.getneighbournodeidsfromnodeid.node_id.clone());
    }
    

    363 => {
        trace.push(format!("get_minmax_edge_ids_from_node_ids(src: {:?}, dst: {:?})", &data.getminmaxedgeidsfromnodeids.src, &data.getminmaxedgeidsfromnodeids.dst));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_minmax_edge_ids_from_node_ids(data.getminmaxedgeidsfromnodeids.src.clone(), data.getminmaxedgeidsfromnodeids.dst.clone());
    }
    

    364 => {
        trace.push(format!("get_edge_id_from_node_ids_and_edge_type_id(src: {:?}, dst: {:?}, edge_type: {:?})", &data.getedgeidfromnodeidsandedgetypeid.src, &data.getedgeidfromnodeidsandedgetypeid.dst, &data.getedgeidfromnodeidsandedgetypeid.edge_type));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_edge_id_from_node_ids_and_edge_type_id(data.getedgeidfromnodeidsandedgetypeid.src.clone(), data.getedgeidfromnodeidsandedgetypeid.dst.clone(), data.getedgeidfromnodeidsandedgetypeid.edge_type.clone());
    }
    

    365 => {
        trace.push(format!("get_minmax_edge_ids_from_source_node_id(src: {:?})", &data.getminmaxedgeidsfromsourcenodeid.src));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_minmax_edge_ids_from_source_node_id(data.getminmaxedgeidsfromsourcenodeid.src.clone());
    }
    

    366 => {
        trace.push(format!("get_node_type_name_from_node_type_id(node_type_id: {:?})", &data.getnodetypenamefromnodetypeid.node_type_id));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_node_type_name_from_node_type_id(data.getnodetypenamefromnodetypeid.node_type_id.clone());
    }
    

    367 => {
        trace.push(format!("iter_unweighted_degree_centrality()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.iter_unweighted_degree_centrality().map(|x| x.collect::<Vec<_>>());
            
    }
    

    368 => {
        trace.push(format!("par_iter_weighted_degree_centrality()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.par_iter_weighted_degree_centrality().map(|x| x.collect::<Vec<_>>());
            
    }
    

    369 => {
        trace.push(format!("get_unweighted_degree_centrality()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_unweighted_degree_centrality();
    }
    

    370 => {
        trace.push(format!("get_weighted_degree_centrality()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_weighted_degree_centrality();
    }
    

    371 => {
        trace.push(format!("par_iter_unweighted_closeness_centrality(verbose: {:?})", &data.pariterunweightedclosenesscentrality.verbose));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.par_iter_unweighted_closeness_centrality(data.pariterunweightedclosenesscentrality.verbose.clone()).collect::<Vec<_>>();
            
    }
    

    372 => {
        trace.push(format!("par_iter_weighted_closeness_centrality(use_edge_weights_as_probabilities: {:?}, verbose: {:?})", &data.pariterweightedclosenesscentrality.use_edge_weights_as_probabilities, &data.pariterweightedclosenesscentrality.verbose));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.par_iter_weighted_closeness_centrality(data.pariterweightedclosenesscentrality.use_edge_weights_as_probabilities.clone(), data.pariterweightedclosenesscentrality.verbose.clone()).map(|x| x.collect::<Vec<_>>());
            
    }
    

    373 => {
        trace.push(format!("get_unweighted_closeness_centrality(verbose: {:?})", &data.getunweightedclosenesscentrality.verbose));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_unweighted_closeness_centrality(data.getunweightedclosenesscentrality.verbose.clone());
    }
    

    374 => {
        trace.push(format!("get_weighted_closeness_centrality(use_edge_weights_as_probabilities: {:?}, verbose: {:?})", &data.getweightedclosenesscentrality.use_edge_weights_as_probabilities, &data.getweightedclosenesscentrality.verbose));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_weighted_closeness_centrality(data.getweightedclosenesscentrality.use_edge_weights_as_probabilities.clone(), data.getweightedclosenesscentrality.verbose.clone());
    }
    

    375 => {
        trace.push(format!("par_iter_unweighted_harmonic_centrality(verbose: {:?})", &data.pariterunweightedharmoniccentrality.verbose));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.par_iter_unweighted_harmonic_centrality(data.pariterunweightedharmoniccentrality.verbose.clone()).collect::<Vec<_>>();
            
    }
    

    376 => {
        trace.push(format!("par_iter_weighted_harmonic_centrality(use_edge_weights_as_probabilities: {:?}, verbose: {:?})", &data.pariterweightedharmoniccentrality.use_edge_weights_as_probabilities, &data.pariterweightedharmoniccentrality.verbose));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.par_iter_weighted_harmonic_centrality(data.pariterweightedharmoniccentrality.use_edge_weights_as_probabilities.clone(), data.pariterweightedharmoniccentrality.verbose.clone()).map(|x| x.collect::<Vec<_>>());
            
    }
    

    377 => {
        trace.push(format!("get_unweighted_harmonic_centrality(verbose: {:?})", &data.getunweightedharmoniccentrality.verbose));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_unweighted_harmonic_centrality(data.getunweightedharmoniccentrality.verbose.clone());
    }
    

    378 => {
        trace.push(format!("get_weighted_harmonic_centrality(use_edge_weights_as_probabilities: {:?}, verbose: {:?})", &data.getweightedharmoniccentrality.use_edge_weights_as_probabilities, &data.getweightedharmoniccentrality.verbose));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_weighted_harmonic_centrality(data.getweightedharmoniccentrality.use_edge_weights_as_probabilities.clone(), data.getweightedharmoniccentrality.verbose.clone());
    }
    

    379 => {
        trace.push(format!("get_stress_centrality(normalize: {:?}, verbose: {:?})", &data.getstresscentrality.normalize, &data.getstresscentrality.verbose));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_stress_centrality(data.getstresscentrality.normalize.clone(), data.getstresscentrality.verbose.clone());
    }
    

    380 => {
        trace.push(format!("get_betweenness_centrality(normalize: {:?}, verbose: {:?})", &data.getbetweennesscentrality.normalize, &data.getbetweennesscentrality.verbose));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_betweenness_centrality(data.getbetweennesscentrality.normalize.clone(), data.getbetweennesscentrality.verbose.clone());
    }
    

    381 => {
        trace.push(format!("get_unweighted_eigenvector_centrality(maximum_iterations_number: {:?}, tollerance: {:?})", &data.getunweightedeigenvectorcentrality.maximum_iterations_number.map(|x| x as usize), &data.getunweightedeigenvectorcentrality.tollerance));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_unweighted_eigenvector_centrality(data.getunweightedeigenvectorcentrality.maximum_iterations_number.map(|x| x as usize).clone(), data.getunweightedeigenvectorcentrality.tollerance.clone());
    }
    

    382 => {
        trace.push(format!("get_weighted_eigenvector_centrality(maximum_iterations_number: {:?}, tollerance: {:?})", &data.getweightedeigenvectorcentrality.maximum_iterations_number.map(|x| x as usize), &data.getweightedeigenvectorcentrality.tollerance));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_weighted_eigenvector_centrality(data.getweightedeigenvectorcentrality.maximum_iterations_number.map(|x| x as usize).clone(), data.getweightedeigenvectorcentrality.tollerance.clone());
    }
    

    383 => {
        trace.push(format!("to_dot(use_node_names: {:?})", &data.todot.use_node_names));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.to_dot(data.todot.use_node_names.clone());
    }
    

    384 => {
        trace.push(format!("compute_hash()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.compute_hash();
    }
    

    385 => {
        trace.push(format!("get_unweighted_preferential_attachment_from_node_ids(source_node_id: {:?}, destination_node_id: {:?}, normalize: {:?})", &data.getunweightedpreferentialattachmentfromnodeids.source_node_id, &data.getunweightedpreferentialattachmentfromnodeids.destination_node_id, &data.getunweightedpreferentialattachmentfromnodeids.normalize));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_unweighted_preferential_attachment_from_node_ids(data.getunweightedpreferentialattachmentfromnodeids.source_node_id.clone(), data.getunweightedpreferentialattachmentfromnodeids.destination_node_id.clone(), data.getunweightedpreferentialattachmentfromnodeids.normalize.clone());
    }
    

    386 => {
        trace.push(format!("get_weighted_preferential_attachment_from_node_ids(source_node_id: {:?}, destination_node_id: {:?}, normalize: {:?})", &data.getweightedpreferentialattachmentfromnodeids.source_node_id, &data.getweightedpreferentialattachmentfromnodeids.destination_node_id, &data.getweightedpreferentialattachmentfromnodeids.normalize));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_weighted_preferential_attachment_from_node_ids(data.getweightedpreferentialattachmentfromnodeids.source_node_id.clone(), data.getweightedpreferentialattachmentfromnodeids.destination_node_id.clone(), data.getweightedpreferentialattachmentfromnodeids.normalize.clone());
    }
    

    387 => {
        trace.push(format!("get_jaccard_coefficient_from_node_ids(source_node_id: {:?}, destination_node_id: {:?})", &data.getjaccardcoefficientfromnodeids.source_node_id, &data.getjaccardcoefficientfromnodeids.destination_node_id));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_jaccard_coefficient_from_node_ids(data.getjaccardcoefficientfromnodeids.source_node_id.clone(), data.getjaccardcoefficientfromnodeids.destination_node_id.clone());
    }
    

    388 => {
        trace.push(format!("get_adamic_adar_index_from_node_ids(source_node_id: {:?}, destination_node_id: {:?})", &data.getadamicadarindexfromnodeids.source_node_id, &data.getadamicadarindexfromnodeids.destination_node_id));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_adamic_adar_index_from_node_ids(data.getadamicadarindexfromnodeids.source_node_id.clone(), data.getadamicadarindexfromnodeids.destination_node_id.clone());
    }
    

    389 => {
        trace.push(format!("get_unweighted_resource_allocation_index_from_node_ids(source_node_id: {:?}, destination_node_id: {:?})", &data.getunweightedresourceallocationindexfromnodeids.source_node_id, &data.getunweightedresourceallocationindexfromnodeids.destination_node_id));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_unweighted_resource_allocation_index_from_node_ids(data.getunweightedresourceallocationindexfromnodeids.source_node_id.clone(), data.getunweightedresourceallocationindexfromnodeids.destination_node_id.clone());
    }
    

    390 => {
        trace.push(format!("get_weighted_resource_allocation_index_from_node_ids(source_node_id: {:?}, destination_node_id: {:?})", &data.getweightedresourceallocationindexfromnodeids.source_node_id, &data.getweightedresourceallocationindexfromnodeids.destination_node_id));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_weighted_resource_allocation_index_from_node_ids(data.getweightedresourceallocationindexfromnodeids.source_node_id.clone(), data.getweightedresourceallocationindexfromnodeids.destination_node_id.clone());
    }
    
            _ => unreachable!()
        }
    }
    
    let _ = graph::test_utilities::default_test_suite(&mut graph, None);

    Ok(())
}

pub fn meta_test_harness(data: MetaParams) -> Result<(), String> {

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
    for _ in 0..10 {
        match rng.next() % 391 {

    0 => {
        graph = graph.get_unweighted_laplacian_transformed_graph(data.getunweightedlaplaciantransformedgraph.verbose.clone());
    }
    

    1 => {
        graph = graph.get_unweighted_random_walk_normalized_laplacian_transformed_graph(data.getunweightedrandomwalknormalizedlaplaciantransformedgraph.verbose.clone());
    }
    

    2 => {
        
        if let Ok(res) = graph.get_unweighted_symmetric_normalized_laplacian_transformed_graph(data.getunweightedsymmetricnormalizedlaplaciantransformedgraph.verbose.clone()) {
            graph = res;
        }
        
    }
    

    3 => {
        
        if let Ok(res) = graph.get_unweighted_symmetric_normalized_transformed_graph(data.getunweightedsymmetricnormalizedtransformedgraph.verbose.clone()) {
            graph = res;
        }
        
    }
    

    4 => {
        
        if let Ok(res) = graph.get_weighted_laplacian_transformed_graph(data.getweightedlaplaciantransformedgraph.verbose.clone()) {
            graph = res;
        }
        
    }
    

    5 => {
        
        if let Ok(res) = graph.get_weighted_symmetric_normalized_laplacian_transformed_graph(data.getweightedsymmetricnormalizedlaplaciantransformedgraph.verbose.clone()) {
            graph = res;
        }
        
    }
    

    6 => {
        
        if let Ok(res) = graph.get_weighted_symmetric_normalized_transformed_graph(data.getweightedsymmetricnormalizedtransformedgraph.verbose.clone()) {
            graph = res;
        }
        
    }
    

    7 => {
        
        if let Ok(res) = graph.get_weighted_random_walk_normalized_laplacian_transformed_graph(data.getweightedrandomwalknormalizedlaplaciantransformedgraph.verbose.clone()) {
            graph = res;
        }
        
    }
    

    8 => {
        let _ = graph.is_singleton_from_node_id(data.issingletonfromnodeid.node_id.clone());
    }
    

    9 => {
        let _ = graph.is_singleton_with_selfloops_from_node_id(data.issingletonwithselfloopsfromnodeid.node_id.clone());
    }
    

    10 => {
        let _ = graph.has_node_type_id(data.hasnodetypeid.node_type_id.clone());
    }
    

    11 => {
        let _ = graph.has_edge_type_id(data.hasedgetypeid.edge_type_id.clone());
    }
    

    12 => {
        let _ = graph.has_edge_from_node_ids(data.hasedgefromnodeids.src.clone(), data.hasedgefromnodeids.dst.clone());
    }
    

    13 => {
        let _ = graph.has_selfloop_from_node_id(data.hasselfloopfromnodeid.node_id.clone());
    }
    

    14 => {
        let _ = graph.has_edge_from_node_ids_and_edge_type_id(data.hasedgefromnodeidsandedgetypeid.src.clone(), data.hasedgefromnodeidsandedgetypeid.dst.clone(), data.hasedgefromnodeidsandedgetypeid.edge_type.clone());
    }
    

    15 => {
        let _ = graph.is_trap_node_from_node_id(data.istrapnodefromnodeid.node_id.clone());
    }
    

    16 => {
        let _ = graph.strongly_connected_components();
    }
    

    17 => {
        graph = graph.sort_by_increasing_outbound_node_degree(data.sortbyincreasingoutboundnodedegree.verbose.clone());
    }
    

    18 => {
        graph = graph.sort_by_decreasing_outbound_node_degree(data.sortbydecreasingoutboundnodedegree.verbose.clone());
    }
    

    19 => {
        let _ = graph.get_dense_binary_adjacency_matrix();
    }
    

    20 => {
        let _ = graph.get_dense_weighted_adjacency_matrix(data.getdenseweightedadjacencymatrix.weight.clone());
    }
    

    21 => {
        
            let _ = graph.iter_edge_ids_from_node_ids(data.iteredgeidsfromnodeids.src.clone(), data.iteredgeidsfromnodeids.dst.clone()).map(|x| x.collect::<Vec<_>>());
            
    }
    

    22 => {
        
            let _ = graph.iter_edge_node_ids_and_edge_type_id_from_edge_type_id(data.iteredgenodeidsandedgetypeidfromedgetypeid.edge_type_id.clone(), data.iteredgenodeidsandedgetypeidfromedgetypeid.directed.clone()).map(|x| x.collect::<Vec<_>>());
            
    }
    

    23 => {
        
            let _ = graph.iter_node_ids_and_node_type_ids_from_node_type_id(data.iternodeidsandnodetypeidsfromnodetypeid.node_type_id.clone()).map(|x| x.collect::<Vec<_>>());
            
    }
    

    24 => {
        let _ = graph.iter_node_names_and_node_type_names_from_node_type_id(data.iternodenamesandnodetypenamesfromnodetypeid.node_type_id.clone());
    }
    

    25 => {
        let _ = graph.iter_edge_node_names_and_edge_type_name_from_edge_type_id(data.iteredgenodenamesandedgetypenamefromedgetypeid.edge_type_id.clone(), data.iteredgenodenamesandedgetypenamefromedgetypeid.directed.clone());
    }
    

    26 => {
        let _ = graph.encode_edge(data.encodeedge.src.clone(), data.encodeedge.dst.clone());
    }
    

    27 => {
        let _ = graph.decode_edge(data.decodeedge.edge.clone());
    }
    

    28 => {
        let _ = graph.get_max_encodable_edge_number();
    }
    

    29 => {
        let _ = graph.validate_node_id(data.validatenodeid.node_id.clone());
    }
    

    30 => {
        let _ = graph.validate_node_ids(data.validatenodeids.node_ids.clone());
    }
    

    31 => {
        let _ = graph.validate_edge_id(data.validateedgeid.edge_id.clone());
    }
    

    32 => {
        let _ = graph.validate_edge_ids(data.validateedgeids.edge_ids.clone());
    }
    

    33 => {
        let _ = graph.must_not_contain_unknown_node_types();
    }
    

    34 => {
        let _ = graph.must_not_contain_unknown_edge_types();
    }
    

    35 => {
        let _ = graph.validate_node_type_id(data.validatenodetypeid.node_type_id.clone());
    }
    

    36 => {
        let _ = graph.validate_node_type_ids(data.validatenodetypeids.node_type_ids.clone());
    }
    

    37 => {
        let _ = graph.validate_edge_type_id(data.validateedgetypeid.edge_type_id.clone());
    }
    

    38 => {
        let _ = graph.validate_edge_type_ids(data.validateedgetypeids.edge_type_ids.clone());
    }
    

    39 => {
        let _ = graph.must_have_node_types();
    }
    

    40 => {
        let _ = graph.must_have_edge_types();
    }
    

    41 => {
        let _ = graph.must_be_undirected();
    }
    

    42 => {
        let _ = graph.must_be_multigraph();
    }
    

    43 => {
        let _ = graph.must_not_be_multigraph();
    }
    

    44 => {
        let _ = graph.must_have_edge_weights();
    }
    

    45 => {
        let _ = graph.must_have_edge_weights_representing_probabilities();
    }
    

    46 => {
        let _ = graph.must_have_positive_edge_weights();
    }
    

    47 => {
        let _ = graph.must_not_contain_weighted_singleton_nodes();
    }
    

    48 => {
        let _ = graph.must_have_edges();
    }
    

    49 => {
        let _ = graph.must_have_nodes();
    }
    

    50 => {
        
            let _ = graph.iter_node_ids().collect::<Vec<_>>();
            
    }
    

    51 => {
        
            let _ = graph.par_iter_node_ids().collect::<Vec<_>>();
            
    }
    

    52 => {
        
            let _ = graph.iter_node_names().collect::<Vec<_>>();
            
    }
    

    53 => {
        
            let _ = graph.par_iter_node_names().collect::<Vec<_>>();
            
    }
    

    54 => {
        
            let _ = graph.iter_unique_node_type_ids().map(|x| x.collect::<Vec<_>>());
            
    }
    

    55 => {
        
            let _ = graph.iter_node_type_counts().map(|x| x.collect::<Vec<_>>());
            
    }
    

    56 => {
        
            let _ = graph.iter_unique_node_type_ids_and_counts().map(|x| x.collect::<Vec<_>>());
            
    }
    

    57 => {
        
            let _ = graph.iter_unique_node_type_names().map(|x| x.collect::<Vec<_>>());
            
    }
    

    58 => {
        
            let _ = graph.iter_unique_node_type_names_and_counts().map(|x| x.collect::<Vec<_>>());
            
    }
    

    59 => {
        
            let _ = graph.iter_unique_edge_type_ids().map(|x| x.collect::<Vec<_>>());
            
    }
    

    60 => {
        
            let _ = graph.iter_edge_type_counts().map(|x| x.collect::<Vec<_>>());
            
    }
    

    61 => {
        
            let _ = graph.iter_unique_edge_type_ids_and_counts().map(|x| x.collect::<Vec<_>>());
            
    }
    

    62 => {
        
            let _ = graph.iter_unique_edge_type_names_and_counts().map(|x| x.collect::<Vec<_>>());
            
    }
    

    63 => {
        
            let _ = graph.iter_unique_edge_type_names().map(|x| x.collect::<Vec<_>>());
            
    }
    

    64 => {
        
            let _ = graph.iter_unweighted_node_degrees().collect::<Vec<_>>();
            
    }
    

    65 => {
        
            let _ = graph.par_iter_unweighted_node_degrees().collect::<Vec<_>>();
            
    }
    

    66 => {
        
            let _ = graph.iter_weighted_node_degrees().map(|x| x.collect::<Vec<_>>());
            
    }
    

    67 => {
        
            let _ = graph.par_iter_weighted_node_degrees().map(|x| x.collect::<Vec<_>>());
            
    }
    

    68 => {
        
            let _ = graph.iter_connected_node_ids().collect::<Vec<_>>();
            
    }
    

    69 => {
        
            let _ = graph.iter_singleton_node_ids().collect::<Vec<_>>();
            
    }
    

    70 => {
        
            let _ = graph.iter_singleton_node_names().collect::<Vec<_>>();
            
    }
    

    71 => {
        
            let _ = graph.iter_singleton_nodes_with_selfloops_node_ids().collect::<Vec<_>>();
            
    }
    

    72 => {
        
            let _ = graph.iter_singleton_nodes_with_selfloops_node_names().collect::<Vec<_>>();
            
    }
    

    73 => {
        
            let _ = graph.iter_singleton_node_type_ids().map(|x| x.collect::<Vec<_>>());
            
    }
    

    74 => {
        
            let _ = graph.iter_singleton_edge_type_ids().map(|x| x.collect::<Vec<_>>());
            
    }
    

    75 => {
        
            let _ = graph.iter_singleton_node_type_names().map(|x| x.collect::<Vec<_>>());
            
    }
    

    76 => {
        
            let _ = graph.iter_singleton_edge_type_names().map(|x| x.collect::<Vec<_>>());
            
    }
    

    77 => {
        
            let _ = graph.iter_source_node_ids(data.itersourcenodeids.directed.clone()).collect::<Vec<_>>();
            
    }
    

    78 => {
        
            let _ = graph.iter_edge_weights().map(|x| x.collect::<Vec<_>>());
            
    }
    

    79 => {
        
            let _ = graph.par_iter_edge_weights().map(|x| x.collect::<Vec<_>>());
            
    }
    

    80 => {
        
            let _ = graph.par_iter_source_node_ids(data.paritersourcenodeids.directed.clone()).collect::<Vec<_>>();
            
    }
    

    81 => {
        
            let _ = graph.iter_destination_node_ids(data.iterdestinationnodeids.directed.clone()).collect::<Vec<_>>();
            
    }
    

    82 => {
        
            let _ = graph.par_iter_destination_node_ids(data.pariterdestinationnodeids.directed.clone()).collect::<Vec<_>>();
            
    }
    

    83 => {
        
            let _ = graph.iter_node_ids_and_node_type_ids().collect::<Vec<_>>();
            
    }
    

    84 => {
        
            let _ = graph.iter_one_hot_encoded_node_type_ids().map(|x| x.collect::<Vec<_>>());
            
    }
    

    85 => {
        
            let _ = graph.iter_one_hot_encoded_known_node_type_ids().map(|x| x.collect::<Vec<_>>());
            
    }
    

    86 => {
        
            let _ = graph.iter_node_names_and_node_type_names().collect::<Vec<_>>();
            
    }
    

    87 => {
        
            let _ = graph.par_iter_node_names_and_node_type_names().collect::<Vec<_>>();
            
    }
    

    88 => {
        
            let _ = graph.iter_edge_node_ids(data.iteredgenodeids.directed.clone()).collect::<Vec<_>>();
            
    }
    

    89 => {
        
            let _ = graph.iter_edges(data.iteredges.directed.clone()).collect::<Vec<_>>();
            
    }
    

    90 => {
        
            let _ = graph.par_iter_edge_node_ids(data.pariteredgenodeids.directed.clone()).collect::<Vec<_>>();
            
    }
    

    91 => {
        
            let _ = graph.par_iter_directed_edge_ids().collect::<Vec<_>>();
            
    }
    

    92 => {
        
            let _ = graph.par_iter_edges(data.pariteredges.directed.clone()).collect::<Vec<_>>();
            
    }
    

    93 => {
        
            let _ = graph.iter_edge_node_ids_and_edge_weight().map(|x| x.collect::<Vec<_>>());
            
    }
    

    94 => {
        
            let _ = graph.par_iter_edge_node_ids_and_edge_weight().map(|x| x.collect::<Vec<_>>());
            
    }
    

    95 => {
        
            let _ = graph.iter_edge_node_ids_and_edge_type_id(data.iteredgenodeidsandedgetypeid.directed.clone()).collect::<Vec<_>>();
            
    }
    

    96 => {
        
            let _ = graph.iter_one_hot_encoded_edge_type_ids().map(|x| x.collect::<Vec<_>>());
            
    }
    

    97 => {
        
            let _ = graph.iter_one_hot_encoded_known_edge_type_ids().map(|x| x.collect::<Vec<_>>());
            
    }
    

    98 => {
        let _ = graph.iter_edge_node_names_and_edge_type_name(data.iteredgenodenamesandedgetypename.directed.clone());
    }
    

    99 => {
        let _ = graph.par_iter_edge_node_names_and_edge_type_name(data.pariteredgenodenamesandedgetypename.directed.clone());
    }
    

    100 => {
        
            let _ = graph.par_iter_edge_node_ids_and_edge_type_id(data.pariteredgenodeidsandedgetypeid.directed.clone()).collect::<Vec<_>>();
            
    }
    

    101 => {
        let _ = graph.par_iter_edge_node_names_and_edge_type_name_and_edge_weight(data.pariteredgenodenamesandedgetypenameandedgeweight.directed.clone());
    }
    

    102 => {
        let _ = graph.iter_edge_node_names_and_edge_type_name_and_edge_weight(data.iteredgenodenamesandedgetypenameandedgeweight.directed.clone());
    }
    

    103 => {
        
            let _ = graph.par_iter_edge_node_ids_and_edge_type_id_and_edge_weight(data.pariteredgenodeidsandedgetypeidandedgeweight.directed.clone()).collect::<Vec<_>>();
            
    }
    

    104 => {
        
            let _ = graph.iter_edge_node_ids_and_edge_type_id_and_edge_weight(data.iteredgenodeidsandedgetypeidandedgeweight.directed.clone()).collect::<Vec<_>>();
            
    }
    

    105 => {
        
            let _ = graph.iter_unique_edge_node_ids(data.iteruniqueedgenodeids.directed.clone()).collect::<Vec<_>>();
            
    }
    

    106 => {
        
            let _ = graph.iter_unique_source_node_ids().collect::<Vec<_>>();
            
    }
    

    107 => {
        
            let _ = graph.iter_edge_ids_with_unknown_edge_types().map(|x| x.collect::<Vec<_>>());
            
    }
    

    108 => {
        
            let _ = graph.iter_edge_ids_with_known_edge_types().map(|x| x.collect::<Vec<_>>());
            
    }
    

    109 => {
        
            let _ = graph.iter_edge_node_ids_with_unknown_edge_types(data.iteredgenodeidswithunknownedgetypes.directed.clone()).map(|x| x.collect::<Vec<_>>());
            
    }
    

    110 => {
        
            let _ = graph.iter_edge_node_ids_with_known_edge_types(data.iteredgenodeidswithknownedgetypes.directed.clone()).map(|x| x.collect::<Vec<_>>());
            
    }
    

    111 => {
        
            let _ = graph.iter_node_ids_with_unknown_node_types().map(|x| x.collect::<Vec<_>>());
            
    }
    

    112 => {
        
            let _ = graph.iter_node_ids_with_known_node_types().map(|x| x.collect::<Vec<_>>());
            
    }
    

    113 => {
        
            let _ = graph.iter_edge_node_names_with_unknown_edge_types(data.iteredgenodenameswithunknownedgetypes.directed.clone()).map(|x| x.collect::<Vec<_>>());
            
    }
    

    114 => {
        
            let _ = graph.iter_edge_node_names_with_known_edge_types(data.iteredgenodenameswithknownedgetypes.directed.clone()).map(|x| x.collect::<Vec<_>>());
            
    }
    

    115 => {
        
            let _ = graph.iter_node_names_with_unknown_node_types().map(|x| x.collect::<Vec<_>>());
            
    }
    

    116 => {
        
            let _ = graph.iter_node_names_with_known_node_types().map(|x| x.collect::<Vec<_>>());
            
    }
    

    117 => {
        
            let _ = graph.par_iter_edge_ids_with_unknown_edge_types().map(|x| x.collect::<Vec<_>>());
            
    }
    

    118 => {
        
            let _ = graph.par_iter_edge_ids_with_known_edge_types().map(|x| x.collect::<Vec<_>>());
            
    }
    

    119 => {
        
            let _ = graph.par_iter_edge_node_ids_with_unknown_edge_types(data.pariteredgenodeidswithunknownedgetypes.directed.clone()).map(|x| x.collect::<Vec<_>>());
            
    }
    

    120 => {
        
            let _ = graph.par_iter_edge_node_ids_with_known_edge_types(data.pariteredgenodeidswithknownedgetypes.directed.clone()).map(|x| x.collect::<Vec<_>>());
            
    }
    

    121 => {
        
            let _ = graph.par_iter_node_ids_with_unknown_node_types().map(|x| x.collect::<Vec<_>>());
            
    }
    

    122 => {
        
            let _ = graph.par_iter_node_ids_with_known_node_types().map(|x| x.collect::<Vec<_>>());
            
    }
    

    123 => {
        
            let _ = graph.par_iter_edge_node_names_with_unknown_edge_types(data.pariteredgenodenameswithunknownedgetypes.directed.clone()).map(|x| x.collect::<Vec<_>>());
            
    }
    

    124 => {
        
            let _ = graph.par_iter_edge_node_names_with_known_edge_types(data.pariteredgenodenameswithknownedgetypes.directed.clone()).map(|x| x.collect::<Vec<_>>());
            
    }
    

    125 => {
        
            let _ = graph.par_iter_node_names_with_unknown_node_types().map(|x| x.collect::<Vec<_>>());
            
    }
    

    126 => {
        
            let _ = graph.par_iter_node_names_with_known_node_types().map(|x| x.collect::<Vec<_>>());
            
    }
    

    127 => {
        let _ = graph.remove_inplace_node_type_ids(data.removeinplacenodetypeids.node_type_ids_to_remove.clone());
    }
    

    128 => {
        let _ = graph.remove_inplace_singleton_node_types();
    }
    

    129 => {
        let _ = graph.remove_inplace_edge_type_ids(data.removeinplaceedgetypeids.edge_type_ids_to_remove.clone());
    }
    

    130 => {
        let _ = graph.remove_inplace_singleton_edge_types();
    }
    

    131 => {
        
        if let Ok(res) = graph.remove_node_type_id(data.removenodetypeid.node_type_id.clone()) {
            graph = res;
        }
        
    }
    

    132 => {
        
        if let Ok(res) = graph.remove_singleton_node_types() {
            graph = res;
        }
        
    }
    

    133 => {
        
        if let Ok(res) = graph.remove_edge_type_id(data.removeedgetypeid.edge_type_id.clone()) {
            graph = res;
        }
        
    }
    

    134 => {
        
        if let Ok(res) = graph.remove_singleton_edge_types() {
            graph = res;
        }
        
    }
    

    135 => {
        let _ = graph.remove_inplace_node_types();
    }
    

    136 => {
        
        if let Ok(res) = graph.remove_node_types() {
            graph = res;
        }
        
    }
    

    137 => {
        let _ = graph.remove_inplace_edge_types();
    }
    

    138 => {
        
        if let Ok(res) = graph.remove_edge_types(data.removeedgetypes.verbose.clone()) {
            graph = res;
        }
        
    }
    

    139 => {
        let _ = graph.remove_inplace_edge_weights();
    }
    

    140 => {
        
        if let Ok(res) = graph.remove_edge_weights() {
            graph = res;
        }
        
    }
    

    141 => {
        let _ = graph.get_unweighted_number_of_triangles(data.getunweightednumberoftriangles.normalize.clone(), data.getunweightednumberoftriangles.low_centrality.clone(), data.getunweightednumberoftriangles.verbose.clone());
    }
    

    142 => {
        let _ = graph.get_unweighted_triads_number();
    }
    

    143 => {
        let _ = graph.get_weighted_triads_number();
    }
    

    144 => {
        let _ = graph.get_unweighted_transitivity(data.getunweightedtransitivity.low_centrality.clone(), data.getunweightedtransitivity.verbose.clone());
    }
    

    145 => {
        let _ = graph.get_unweighted_number_of_triangles_per_node(data.getunweightednumberoftrianglespernode.normalize.clone(), data.getunweightednumberoftrianglespernode.low_centrality.clone(), data.getunweightednumberoftrianglespernode.verbose.clone());
    }
    

    146 => {
        
            let _ = graph.iter_clustering_coefficient_per_node(data.iterclusteringcoefficientpernode.low_centrality.clone(), data.iterclusteringcoefficientpernode.verbose.clone()).collect::<Vec<_>>();
            
    }
    

    147 => {
        let _ = graph.get_clustering_coefficient_per_node(data.getclusteringcoefficientpernode.low_centrality.clone(), data.getclusteringcoefficientpernode.verbose.clone());
    }
    

    148 => {
        let _ = graph.get_clustering_coefficient(data.getclusteringcoefficient.low_centrality.clone(), data.getclusteringcoefficient.verbose.clone());
    }
    

    149 => {
        let _ = graph.get_average_clustering_coefficient(data.getaverageclusteringcoefficient.low_centrality.clone(), data.getaverageclusteringcoefficient.verbose.clone());
    }
    

    150 => {
        
        if let Ok(res) = graph.remap_from_node_ids(data.remapfromnodeids.node_ids.clone(), data.remapfromnodeids.verbose.clone()) {
            graph = res;
        }
        
    }
    

    151 => {
        
        if let Ok((res1, res2)) = graph.node_label_holdout(data.nodelabelholdout.train_size.clone(), data.nodelabelholdout.use_stratification.clone(), data.nodelabelholdout.random_state.clone()) {
            if rng.next() % 2 == 0 {
                graph = res1;
            } else {
                graph = res2;
            }
        }
        
    }
    

    152 => {
        
        if let Ok((res1, res2)) = graph.edge_label_holdout(data.edgelabelholdout.train_size.clone(), data.edgelabelholdout.use_stratification.clone(), data.edgelabelholdout.random_state.clone()) {
            if rng.next() % 2 == 0 {
                graph = res1;
            } else {
                graph = res2;
            }
        }
        
    }
    

    153 => {
        
        if let Ok(res) = graph.random_subgraph(data.randomsubgraph.nodes_number.clone(), data.randomsubgraph.random_state.clone(), data.randomsubgraph.verbose.clone()) {
            graph = res;
        }
        
    }
    

    154 => {
        let _ = graph.get_unweighted_minimum_path_node_ids_from_node_ids(data.getunweightedminimumpathnodeidsfromnodeids.src_node_id.clone(), data.getunweightedminimumpathnodeidsfromnodeids.dst_node_id.clone(), data.getunweightedminimumpathnodeidsfromnodeids.maximal_depth.clone());
    }
    

    155 => {
        let _ = graph.get_unweighted_k_shortest_path_node_ids_from_node_ids(data.getunweightedkshortestpathnodeidsfromnodeids.src_node_id.clone(), data.getunweightedkshortestpathnodeidsfromnodeids.dst_node_id.clone(), data.getunweightedkshortestpathnodeidsfromnodeids.k as usize.clone());
    }
    

    156 => {
        let _ = graph.get_unweighted_eccentricity_from_node_id(data.getunweightedeccentricityfromnodeid.node_id.clone());
    }
    

    157 => {
        let _ = graph.get_weighted_eccentricity_from_node_id(data.getweightedeccentricityfromnodeid.node_id.clone(), data.getweightedeccentricityfromnodeid.use_edge_weights_as_probabilities.clone());
    }
    

    158 => {
        let _ = graph.get_weighted_minimum_path_node_ids_from_node_ids(data.getweightedminimumpathnodeidsfromnodeids.src_node_id.clone(), data.getweightedminimumpathnodeidsfromnodeids.dst_node_id.clone(), data.getweightedminimumpathnodeidsfromnodeids.use_edge_weights_as_probabilities.clone(), data.getweightedminimumpathnodeidsfromnodeids.maximal_depth.clone());
    }
    

    159 => {
        let _ = graph.get_breath_first_search_from_node_ids(data.getbreathfirstsearchfromnodeids.src_node_id.clone(), data.getbreathfirstsearchfromnodeids.maybe_dst_node_id.clone(), data.getbreathfirstsearchfromnodeids.maybe_dst_node_ids.clone(), data.getbreathfirstsearchfromnodeids.compute_distances.clone(), data.getbreathfirstsearchfromnodeids.compute_predecessors.clone(), data.getbreathfirstsearchfromnodeids.compute_visited.clone(), data.getbreathfirstsearchfromnodeids.maximal_depth.clone());
    }
    

    160 => {
        let _ = graph.get_dijkstra_from_node_ids(data.getdijkstrafromnodeids.src_node_id.clone(), data.getdijkstrafromnodeids.maybe_dst_node_id.clone(), data.getdijkstrafromnodeids.maybe_dst_node_ids.clone(), data.getdijkstrafromnodeids.compute_predecessors.clone(), data.getdijkstrafromnodeids.maximal_depth.clone(), data.getdijkstrafromnodeids.use_edge_weights_as_probabilities.clone());
    }
    

    161 => {
        let _ = graph.get_unweighted_diameter(data.getunweighteddiameter.ignore_infinity.clone(), data.getunweighteddiameter.verbose.clone());
    }
    

    162 => {
        let _ = graph.get_weighted_diameter(data.getweighteddiameter.ignore_infinity.clone(), data.getweighteddiameter.use_edge_weights_as_probabilities.clone(), data.getweighteddiameter.verbose.clone());
    }
    

    163 => {
        let _ = graph.get_connected_components_number(data.getconnectedcomponentsnumber.verbose.clone());
    }
    

    164 => {
        let _ = graph.get_singleton_nodes_number();
    }
    

    165 => {
        let _ = graph.get_weighted_singleton_nodes_number();
    }
    

    166 => {
        let _ = graph.get_disconnected_nodes_number();
    }
    

    167 => {
        let _ = graph.get_singleton_node_ids();
    }
    

    168 => {
        let _ = graph.get_singleton_node_names();
    }
    

    169 => {
        let _ = graph.get_singleton_nodes_with_selfloops_number();
    }
    

    170 => {
        let _ = graph.get_singleton_with_selfloops_node_ids();
    }
    

    171 => {
        let _ = graph.get_singleton_with_selfloops_node_names();
    }
    

    172 => {
        let _ = graph.get_connected_nodes_number();
    }
    

    173 => {
        let _ = graph.get_density();
    }
    

    174 => {
        let _ = graph.get_trap_nodes_rate();
    }
    

    175 => {
        let _ = graph.get_unweighted_node_degrees_mean();
    }
    

    176 => {
        let _ = graph.get_weighted_node_degrees_mean();
    }
    

    177 => {
        let _ = graph.get_undirected_edges_number();
    }
    

    178 => {
        let _ = graph.get_unique_undirected_edges_number();
    }
    

    179 => {
        let _ = graph.get_edges_number();
    }
    

    180 => {
        let _ = graph.get_unique_edges_number();
    }
    

    181 => {
        let _ = graph.get_unweighted_node_degrees_median();
    }
    

    182 => {
        let _ = graph.get_weighted_node_degrees_median();
    }
    

    183 => {
        let _ = graph.get_weighted_maximum_node_degree();
    }
    

    184 => {
        let _ = graph.get_weighted_minimum_node_degree();
    }
    

    185 => {
        let _ = graph.get_unweighted_maximum_node_degree();
    }
    

    186 => {
        let _ = graph.get_argmax_node_degree();
    }
    

    187 => {
        let _ = graph.get_weighted_mininum_node_degree();
    }
    

    188 => {
        let _ = graph.get_unweighted_minimum_node_degree();
    }
    

    189 => {
        let _ = graph.get_unweighted_node_degrees_mode();
    }
    

    190 => {
        let _ = graph.get_selfloop_number();
    }
    

    191 => {
        let _ = graph.get_unique_selfloop_number();
    }
    

    192 => {
        let _ = graph.get_selfloop_nodes_rate();
    }
    

    193 => {
        let _ = graph.get_name();
    }
    

    194 => {
        let _ = graph.get_trap_nodes_number();
    }
    

    195 => {
        let _ = graph.get_source_node_ids(data.getsourcenodeids.directed.clone());
    }
    

    196 => {
        let _ = graph.get_source_names(data.getsourcenames.directed.clone());
    }
    

    197 => {
        let _ = graph.get_destination_node_ids(data.getdestinationnodeids.directed.clone());
    }
    

    198 => {
        let _ = graph.get_destination_names(data.getdestinationnames.directed.clone());
    }
    

    199 => {
        let _ = graph.get_node_names();
    }
    

    200 => {
        let _ = graph.get_node_ids();
    }
    

    201 => {
        let _ = graph.get_edge_type_ids();
    }
    

    202 => {
        let _ = graph.get_unique_edge_type_ids();
    }
    

    203 => {
        let _ = graph.get_edge_type_names();
    }
    

    204 => {
        let _ = graph.get_unique_edge_type_names();
    }
    

    205 => {
        let _ = graph.get_edge_weights();
    }
    

    206 => {
        let _ = graph.get_total_edge_weights();
    }
    

    207 => {
        let _ = graph.get_mininum_edge_weight();
    }
    

    208 => {
        let _ = graph.get_maximum_edge_weight();
    }
    

    209 => {
        let _ = graph.get_node_type_ids();
    }
    

    210 => {
        let _ = graph.get_one_hot_encoded_node_types();
    }
    

    211 => {
        let _ = graph.get_one_hot_encoded_known_node_types();
    }
    

    212 => {
        let _ = graph.get_one_hot_encoded_edge_types();
    }
    

    213 => {
        let _ = graph.get_one_hot_encoded_known_edge_types();
    }
    

    214 => {
        let _ = graph.get_node_type_names();
    }
    

    215 => {
        let _ = graph.get_unique_node_type_ids();
    }
    

    216 => {
        let _ = graph.get_unique_node_type_names();
    }
    

    217 => {
        let _ = graph.get_unique_directed_edges_number();
    }
    

    218 => {
        let _ = graph.get_nodes_mapping();
    }
    

    219 => {
        let _ = graph.get_edge_node_ids(data.getedgenodeids.directed.clone());
    }
    

    220 => {
        let _ = graph.get_edge_node_names(data.getedgenodenames.directed.clone());
    }
    

    221 => {
        let _ = graph.get_unknown_node_types_number();
    }
    

    222 => {
        let _ = graph.get_known_node_types_number();
    }
    

    223 => {
        let _ = graph.get_unknown_node_types_rate();
    }
    

    224 => {
        let _ = graph.get_known_node_types_rate();
    }
    

    225 => {
        let _ = graph.get_minimum_node_types_number();
    }
    

    226 => {
        let _ = graph.get_maximum_node_types_number();
    }
    

    227 => {
        let _ = graph.get_maximum_multilabel_count();
    }
    

    228 => {
        let _ = graph.get_singleton_node_types_number();
    }
    

    229 => {
        let _ = graph.get_singleton_node_type_ids();
    }
    

    230 => {
        let _ = graph.get_singleton_node_type_names();
    }
    

    231 => {
        let _ = graph.get_unknown_edge_types_number();
    }
    

    232 => {
        let _ = graph.get_edge_ids_with_unknown_edge_types();
    }
    

    233 => {
        let _ = graph.get_edge_ids_with_known_edge_types();
    }
    

    234 => {
        let _ = graph.get_edge_node_ids_with_unknown_edge_types(data.getedgenodeidswithunknownedgetypes.directed.clone());
    }
    

    235 => {
        let _ = graph.get_edge_node_ids_with_known_edge_types(data.getedgenodeidswithknownedgetypes.directed.clone());
    }
    

    236 => {
        let _ = graph.get_edge_node_names_with_unknown_edge_types(data.getedgenodenameswithunknownedgetypes.directed.clone());
    }
    

    237 => {
        let _ = graph.get_edge_node_names_with_known_edge_types(data.getedgenodenameswithknownedgetypes.directed.clone());
    }
    

    238 => {
        let _ = graph.get_edge_ids_with_unknown_edge_types_mask();
    }
    

    239 => {
        let _ = graph.get_edge_ids_with_known_edge_types_mask();
    }
    

    240 => {
        let _ = graph.get_node_ids_with_unknown_node_types();
    }
    

    241 => {
        let _ = graph.get_node_ids_with_known_node_types();
    }
    

    242 => {
        let _ = graph.get_node_names_with_unknown_node_types();
    }
    

    243 => {
        let _ = graph.get_node_names_with_known_node_types();
    }
    

    244 => {
        let _ = graph.get_node_ids_with_unknown_node_types_mask();
    }
    

    245 => {
        let _ = graph.get_node_ids_with_known_node_types_mask();
    }
    

    246 => {
        let _ = graph.get_known_edge_types_number();
    }
    

    247 => {
        let _ = graph.get_unknown_edge_types_rate();
    }
    

    248 => {
        let _ = graph.get_known_edge_types_rate();
    }
    

    249 => {
        let _ = graph.get_minimum_edge_types_number();
    }
    

    250 => {
        let _ = graph.get_singleton_edge_types_number();
    }
    

    251 => {
        let _ = graph.get_singleton_edge_type_ids();
    }
    

    252 => {
        let _ = graph.get_singleton_edge_type_names();
    }
    

    253 => {
        let _ = graph.get_nodes_number();
    }
    

    254 => {
        let _ = graph.get_node_connected_component_ids(data.getnodeconnectedcomponentids.verbose.clone());
    }
    

    255 => {
        let _ = graph.get_directed_edges_number();
    }
    

    256 => {
        let _ = graph.get_edge_types_number();
    }
    

    257 => {
        let _ = graph.get_node_types_number();
    }
    

    258 => {
        let _ = graph.get_unweighted_node_degrees();
    }
    

    259 => {
        let _ = graph.get_weighted_node_degrees();
    }
    

    260 => {
        let _ = graph.get_not_singletons_node_ids();
    }
    

    261 => {
        let _ = graph.get_dense_nodes_mapping();
    }
    

    262 => {
        let _ = graph.get_parallel_edges_number();
    }
    

    263 => {
        let _ = graph.get_cumulative_node_degrees();
    }
    

    264 => {
        let _ = graph.get_unique_source_nodes_number();
    }
    

    265 => {
        let _ = graph.get_edge_type_id_counts_hashmap();
    }
    

    266 => {
        let _ = graph.get_edge_type_names_counts_hashmap();
    }
    

    267 => {
        let _ = graph.get_node_type_id_counts_hashmap();
    }
    

    268 => {
        let _ = graph.get_node_type_names_counts_hashmap();
    }
    

    269 => {
        let _ = graph.to_directed_inplace();
    }
    

    270 => {
        graph = graph.to_directed();
    }
    

    271 => {
        graph = graph.to_upper_triangular(data.touppertriangular.verbose.clone());
    }
    

    272 => {
        graph = graph.to_lower_triangular(data.tolowertriangular.verbose.clone());
    }
    

    273 => {
        graph = graph.to_main_diagonal(data.tomaindiagonal.verbose.clone());
    }
    

    274 => {
        graph = graph.to_anti_diagonal(data.toantidiagonal.verbose.clone());
    }
    

    275 => {
        graph = graph.to_bidiagonal(data.tobidiagonal.verbose.clone());
    }
    

    276 => {
        graph = graph.to_arrowhead(data.toarrowhead.verbose.clone());
    }
    

    277 => {
        graph = graph.to_transposed(data.totransposed.verbose.clone());
    }
    

    278 => {
        graph = graph.to_complementary(data.tocomplementary.verbose.clone());
    }
    

    279 => {
        let _ = graph.report();
    }
    

    280 => {
        let _ = graph.get_node_report_from_node_id(data.getnodereportfromnodeid.node_id.clone());
    }
    

    281 => {
        let _ = graph.textual_report();
    }
    

    282 => {
        
        if let Ok(res) = graph.filter_from_ids(data.filterfromids.node_ids_to_keep.clone(), data.filterfromids.node_ids_to_filter.clone(), data.filterfromids.node_type_ids_to_keep.clone(), data.filterfromids.node_type_ids_to_filter.clone(), data.filterfromids.node_type_id_to_keep.clone(), data.filterfromids.node_type_id_to_filter.clone(), data.filterfromids.edge_ids_to_keep.clone(), data.filterfromids.edge_ids_to_filter.clone(), data.filterfromids.edge_node_ids_to_keep.clone(), data.filterfromids.edge_node_ids_to_filter.clone(), data.filterfromids.edge_type_ids_to_keep.clone(), data.filterfromids.edge_type_ids_to_filter.clone(), data.filterfromids.min_edge_weight.clone(), data.filterfromids.max_edge_weight.clone(), data.filterfromids.filter_singleton_nodes.clone(), data.filterfromids.filter_singleton_nodes_with_selfloop.clone(), data.filterfromids.filter_selfloops.clone(), data.filterfromids.filter_parallel_edges.clone(), data.filterfromids.verbose.clone()) {
            graph = res;
        }
        
    }
    

    283 => {
        graph = graph.drop_unknown_node_types(data.dropunknownnodetypes.verbose.clone());
    }
    

    284 => {
        graph = graph.drop_unknown_edge_types(data.dropunknownedgetypes.verbose.clone());
    }
    

    285 => {
        graph = graph.drop_singleton_nodes(data.dropsingletonnodes.verbose.clone());
    }
    

    286 => {
        graph = graph.drop_singleton_nodes_with_selfloops(data.dropsingletonnodeswithselfloops.verbose.clone());
    }
    

    287 => {
        graph = graph.drop_disconnected_nodes(data.dropdisconnectednodes.verbose.clone());
    }
    

    288 => {
        graph = graph.drop_selfloops(data.dropselfloops.verbose.clone());
    }
    

    289 => {
        graph = graph.drop_parallel_edges(data.dropparalleledges.verbose.clone());
    }
    

    290 => {
        let _ = graph.spanning_arborescence_kruskal(data.spanningarborescencekruskal.verbose.clone());
    }
    

    291 => {
        let _ = graph.spanning_arborescence(data.spanningarborescence.verbose.clone());
    }
    

    292 => {
        let _ = graph.connected_components(data.connectedcomponents.verbose.clone());
    }
    

    293 => {
        let _ = graph.enable(data.enable.vector_sources.clone(), data.enable.vector_destinations.clone(), data.enable.vector_cumulative_node_degrees.clone());
    }
    

    294 => {
        graph.disable_all()
    }
    

    295 => {
        
            let _ = graph.par_iter_approximated_vertex_cover().collect::<Vec<_>>();
            
    }
    

    296 => {
        let _ = graph.approximated_vertex_cover_set();
    }
    

    297 => {
        let _ = graph.get_node_label_prediction_tuple_from_node_ids(data.getnodelabelpredictiontuplefromnodeids.node_ids.clone(), data.getnodelabelpredictiontuplefromnodeids.random_state.clone(), data.getnodelabelpredictiontuplefromnodeids.include_central_node.clone(), data.getnodelabelpredictiontuplefromnodeids.return_edge_weights.clone(), data.getnodelabelpredictiontuplefromnodeids.offset.clone(), data.getnodelabelpredictiontuplefromnodeids.max_neighbours.clone());
    }
    

    298 => {
        
            let _ = graph.par_iter_edge_prediction_metrics(data.pariteredgepredictionmetrics.normalize.clone(), data.pariteredgepredictionmetrics.verbose.clone()).collect::<Vec<_>>();
            
    }
    

    299 => {
        let _ = graph.get_okapi_bm25_node_feature_propagation(data.getokapibm25nodefeaturepropagation.features.clone(), data.getokapibm25nodefeaturepropagation.iterations.map(|x| x as usize).clone(), data.getokapibm25nodefeaturepropagation.maximal_distance.clone(), data.getokapibm25nodefeaturepropagation.k1.clone(), data.getokapibm25nodefeaturepropagation.b.clone(), data.getokapibm25nodefeaturepropagation.include_central_node.clone(), data.getokapibm25nodefeaturepropagation.verbose.clone());
    }
    

    300 => {
        let _ = graph.get_okapi_bm25_node_label_propagation(data.getokapibm25nodelabelpropagation.iterations.map(|x| x as usize).clone(), data.getokapibm25nodelabelpropagation.maximal_distance.clone(), data.getokapibm25nodelabelpropagation.k1.clone(), data.getokapibm25nodelabelpropagation.b.clone(), data.getokapibm25nodelabelpropagation.verbose.clone());
    }
    

    301 => {
        let _ = graph.has_default_graph_name();
    }
    

    302 => {
        let _ = graph.has_nodes();
    }
    

    303 => {
        let _ = graph.has_edges();
    }
    

    304 => {
        let _ = graph.has_trap_nodes();
    }
    

    305 => {
        let _ = graph.is_directed();
    }
    

    306 => {
        let _ = graph.has_edge_weights();
    }
    

    307 => {
        let _ = graph.has_edge_weights_representing_probabilities();
    }
    

    308 => {
        let _ = graph.has_weighted_singleton_nodes();
    }
    

    309 => {
        let _ = graph.has_constant_edge_weights();
    }
    

    310 => {
        let _ = graph.has_negative_edge_weights();
    }
    

    311 => {
        let _ = graph.has_edge_types();
    }
    

    312 => {
        let _ = graph.has_selfloops();
    }
    

    313 => {
        let _ = graph.has_disconnected_nodes();
    }
    

    314 => {
        let _ = graph.has_singleton_nodes();
    }
    

    315 => {
        let _ = graph.has_singleton_nodes_with_selfloops();
    }
    

    316 => {
        let _ = graph.is_connected(data.isconnected.verbose.clone());
    }
    

    317 => {
        let _ = graph.has_node_types();
    }
    

    318 => {
        let _ = graph.has_multilabel_node_types();
    }
    

    319 => {
        let _ = graph.has_unknown_node_types();
    }
    

    320 => {
        let _ = graph.has_known_node_types();
    }
    

    321 => {
        let _ = graph.has_unknown_edge_types();
    }
    

    322 => {
        let _ = graph.has_known_edge_types();
    }
    

    323 => {
        let _ = graph.has_homogeneous_node_types();
    }
    

    324 => {
        let _ = graph.has_homogeneous_edge_types();
    }
    

    325 => {
        let _ = graph.has_singleton_node_types();
    }
    

    326 => {
        let _ = graph.has_node_oddities();
    }
    

    327 => {
        let _ = graph.has_node_types_oddities();
    }
    

    328 => {
        let _ = graph.has_singleton_edge_types();
    }
    

    329 => {
        let _ = graph.has_edge_types_oddities();
    }
    

    330 => {
        let _ = graph.is_multigraph();
    }
    

    331 => {
        let _ = graph.has_nodes_sorted_by_decreasing_outbound_node_degree();
    }
    

    332 => {
        let _ = graph.has_nodes_sorted_by_increasing_outbound_node_degree();
    }
    

    333 => {
        graph = graph.get_transitive_closure(data.gettransitiveclosure.iterations.map(|x| x as NodeT).clone(), data.gettransitiveclosure.verbose.clone());
    }
    

    334 => {
        graph = graph.get_unweighted_all_shortest_paths(data.getunweightedallshortestpaths.iterations.map(|x| x as NodeT).clone(), data.getunweightedallshortestpaths.verbose.clone());
    }
    

    335 => {
        
        if let Ok(res) = graph.get_weighted_all_shortest_paths(data.getweightedallshortestpaths.iterations.map(|x| x as NodeT).clone(), data.getweightedallshortestpaths.use_edge_weights_as_probabilities.clone(), data.getweightedallshortestpaths.verbose.clone()) {
            graph = res;
        }
        
    }
    

    336 => {
        let _ = graph.get_source_node_id_from_edge_id(data.getsourcenodeidfromedgeid.edge_id.clone());
    }
    

    337 => {
        let _ = graph.get_destination_node_id_from_edge_id(data.getdestinationnodeidfromedgeid.edge_id.clone());
    }
    

    338 => {
        let _ = graph.get_source_node_name_from_edge_id(data.getsourcenodenamefromedgeid.edge_id.clone());
    }
    

    339 => {
        let _ = graph.get_destination_node_name_from_edge_id(data.getdestinationnodenamefromedgeid.edge_id.clone());
    }
    

    340 => {
        let _ = graph.get_node_names_from_edge_id(data.getnodenamesfromedgeid.edge_id.clone());
    }
    

    341 => {
        let _ = graph.get_node_ids_from_edge_id(data.getnodeidsfromedgeid.edge_id.clone());
    }
    

    342 => {
        let _ = graph.get_edge_id_from_node_ids(data.getedgeidfromnodeids.src.clone(), data.getedgeidfromnodeids.dst.clone());
    }
    

    343 => {
        let _ = graph.get_node_ids_and_edge_type_id_from_edge_id(data.getnodeidsandedgetypeidfromedgeid.edge_id.clone());
    }
    

    344 => {
        let _ = graph.get_node_ids_and_edge_type_id_and_edge_weight_from_edge_id(data.getnodeidsandedgetypeidandedgeweightfromedgeid.edge_id.clone());
    }
    

    345 => {
        let _ = graph.get_unweighted_top_k_central_node_ids(data.getunweightedtopkcentralnodeids.k.clone());
    }
    

    346 => {
        let _ = graph.get_weighted_top_k_central_node_ids(data.getweightedtopkcentralnodeids.k.clone());
    }
    

    347 => {
        let _ = graph.get_unweighted_node_degree_from_node_id(data.getunweightednodedegreefromnodeid.node_id.clone());
    }
    

    348 => {
        let _ = graph.get_weighted_node_degree_from_node_id(data.getweightednodedegreefromnodeid.node_id.clone());
    }
    

    349 => {
        let _ = graph.get_top_k_central_node_names(data.gettopkcentralnodenames.k.clone());
    }
    

    350 => {
        let _ = graph.get_node_type_id_from_node_id(data.getnodetypeidfromnodeid.node_id.clone());
    }
    

    351 => {
        let _ = graph.get_edge_type_id_from_edge_id(data.getedgetypeidfromedgeid.edge_id.clone());
    }
    

    352 => {
        let _ = graph.get_node_type_names_from_node_id(data.getnodetypenamesfromnodeid.node_id.clone());
    }
    

    353 => {
        let _ = graph.get_edge_type_name_from_edge_id(data.getedgetypenamefromedgeid.edge_id.clone());
    }
    

    354 => {
        let _ = graph.get_edge_type_name_from_edge_type_id(data.getedgetypenamefromedgetypeid.edge_type_id.clone());
    }
    

    355 => {
        let _ = graph.get_edge_weight_from_edge_id(data.getedgeweightfromedgeid.edge_id.clone());
    }
    

    356 => {
        let _ = graph.get_edge_weight_from_node_ids(data.getedgeweightfromnodeids.src.clone(), data.getedgeweightfromnodeids.dst.clone());
    }
    

    357 => {
        let _ = graph.get_edge_weight_from_node_ids_and_edge_type_id(data.getedgeweightfromnodeidsandedgetypeid.src.clone(), data.getedgeweightfromnodeidsandedgetypeid.dst.clone(), data.getedgeweightfromnodeidsandedgetypeid.edge_type.clone());
    }
    

    358 => {
        let _ = graph.get_node_name_from_node_id(data.getnodenamefromnodeid.node_id.clone());
    }
    

    359 => {
        let _ = graph.get_edge_node_names_from_edge_node_ids(data.getedgenodenamesfromedgenodeids.edge_node_ids.clone());
    }
    

    360 => {
        let _ = graph.get_edge_count_from_edge_type_id(data.getedgecountfromedgetypeid.edge_type_id.clone());
    }
    

    361 => {
        let _ = graph.get_node_count_from_node_type_id(data.getnodecountfromnodetypeid.node_type_id.clone());
    }
    

    362 => {
        let _ = graph.get_neighbour_node_ids_from_node_id(data.getneighbournodeidsfromnodeid.node_id.clone());
    }
    

    363 => {
        let _ = graph.get_minmax_edge_ids_from_node_ids(data.getminmaxedgeidsfromnodeids.src.clone(), data.getminmaxedgeidsfromnodeids.dst.clone());
    }
    

    364 => {
        let _ = graph.get_edge_id_from_node_ids_and_edge_type_id(data.getedgeidfromnodeidsandedgetypeid.src.clone(), data.getedgeidfromnodeidsandedgetypeid.dst.clone(), data.getedgeidfromnodeidsandedgetypeid.edge_type.clone());
    }
    

    365 => {
        let _ = graph.get_minmax_edge_ids_from_source_node_id(data.getminmaxedgeidsfromsourcenodeid.src.clone());
    }
    

    366 => {
        let _ = graph.get_node_type_name_from_node_type_id(data.getnodetypenamefromnodetypeid.node_type_id.clone());
    }
    

    367 => {
        
            let _ = graph.iter_unweighted_degree_centrality().map(|x| x.collect::<Vec<_>>());
            
    }
    

    368 => {
        
            let _ = graph.par_iter_weighted_degree_centrality().map(|x| x.collect::<Vec<_>>());
            
    }
    

    369 => {
        let _ = graph.get_unweighted_degree_centrality();
    }
    

    370 => {
        let _ = graph.get_weighted_degree_centrality();
    }
    

    371 => {
        
            let _ = graph.par_iter_unweighted_closeness_centrality(data.pariterunweightedclosenesscentrality.verbose.clone()).collect::<Vec<_>>();
            
    }
    

    372 => {
        
            let _ = graph.par_iter_weighted_closeness_centrality(data.pariterweightedclosenesscentrality.use_edge_weights_as_probabilities.clone(), data.pariterweightedclosenesscentrality.verbose.clone()).map(|x| x.collect::<Vec<_>>());
            
    }
    

    373 => {
        let _ = graph.get_unweighted_closeness_centrality(data.getunweightedclosenesscentrality.verbose.clone());
    }
    

    374 => {
        let _ = graph.get_weighted_closeness_centrality(data.getweightedclosenesscentrality.use_edge_weights_as_probabilities.clone(), data.getweightedclosenesscentrality.verbose.clone());
    }
    

    375 => {
        
            let _ = graph.par_iter_unweighted_harmonic_centrality(data.pariterunweightedharmoniccentrality.verbose.clone()).collect::<Vec<_>>();
            
    }
    

    376 => {
        
            let _ = graph.par_iter_weighted_harmonic_centrality(data.pariterweightedharmoniccentrality.use_edge_weights_as_probabilities.clone(), data.pariterweightedharmoniccentrality.verbose.clone()).map(|x| x.collect::<Vec<_>>());
            
    }
    

    377 => {
        let _ = graph.get_unweighted_harmonic_centrality(data.getunweightedharmoniccentrality.verbose.clone());
    }
    

    378 => {
        let _ = graph.get_weighted_harmonic_centrality(data.getweightedharmoniccentrality.use_edge_weights_as_probabilities.clone(), data.getweightedharmoniccentrality.verbose.clone());
    }
    

    379 => {
        let _ = graph.get_stress_centrality(data.getstresscentrality.normalize.clone(), data.getstresscentrality.verbose.clone());
    }
    

    380 => {
        let _ = graph.get_betweenness_centrality(data.getbetweennesscentrality.normalize.clone(), data.getbetweennesscentrality.verbose.clone());
    }
    

    381 => {
        let _ = graph.get_unweighted_eigenvector_centrality(data.getunweightedeigenvectorcentrality.maximum_iterations_number.map(|x| x as usize).clone(), data.getunweightedeigenvectorcentrality.tollerance.clone());
    }
    

    382 => {
        let _ = graph.get_weighted_eigenvector_centrality(data.getweightedeigenvectorcentrality.maximum_iterations_number.map(|x| x as usize).clone(), data.getweightedeigenvectorcentrality.tollerance.clone());
    }
    

    383 => {
        let _ = graph.to_dot(data.todot.use_node_names.clone());
    }
    

    384 => {
        let _ = graph.compute_hash();
    }
    

    385 => {
        let _ = graph.get_unweighted_preferential_attachment_from_node_ids(data.getunweightedpreferentialattachmentfromnodeids.source_node_id.clone(), data.getunweightedpreferentialattachmentfromnodeids.destination_node_id.clone(), data.getunweightedpreferentialattachmentfromnodeids.normalize.clone());
    }
    

    386 => {
        let _ = graph.get_weighted_preferential_attachment_from_node_ids(data.getweightedpreferentialattachmentfromnodeids.source_node_id.clone(), data.getweightedpreferentialattachmentfromnodeids.destination_node_id.clone(), data.getweightedpreferentialattachmentfromnodeids.normalize.clone());
    }
    

    387 => {
        let _ = graph.get_jaccard_coefficient_from_node_ids(data.getjaccardcoefficientfromnodeids.source_node_id.clone(), data.getjaccardcoefficientfromnodeids.destination_node_id.clone());
    }
    

    388 => {
        let _ = graph.get_adamic_adar_index_from_node_ids(data.getadamicadarindexfromnodeids.source_node_id.clone(), data.getadamicadarindexfromnodeids.destination_node_id.clone());
    }
    

    389 => {
        let _ = graph.get_unweighted_resource_allocation_index_from_node_ids(data.getunweightedresourceallocationindexfromnodeids.source_node_id.clone(), data.getunweightedresourceallocationindexfromnodeids.destination_node_id.clone());
    }
    

    390 => {
        let _ = graph.get_weighted_resource_allocation_index_from_node_ids(data.getweightedresourceallocationindexfromnodeids.source_node_id.clone(), data.getweightedresourceallocationindexfromnodeids.destination_node_id.clone());
    }
    
            _ => unreachable!()
        }
    }
    
    let _ = graph::test_utilities::default_test_suite(&mut graph, None);

    Ok(())
}

pub fn meta_test_trace(data: MetaParams) -> Result<(), String> {
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
    println!("{}\n\n", graph.textual_report());

    let mut rng = Rng::new(data.seed);
    for _ in 0..10 {
        match rng.next() % 391 {

    0 => {
        println!("get_unweighted_laplacian_transformed_graph(verbose: {:?})", &data.getunweightedlaplaciantransformedgraph.verbose);
    }
    

    1 => {
        println!("get_unweighted_random_walk_normalized_laplacian_transformed_graph(verbose: {:?})", &data.getunweightedrandomwalknormalizedlaplaciantransformedgraph.verbose);
    }
    

    2 => {
        println!("get_unweighted_symmetric_normalized_laplacian_transformed_graph(verbose: {:?})", &data.getunweightedsymmetricnormalizedlaplaciantransformedgraph.verbose);
    }
    

    3 => {
        println!("get_unweighted_symmetric_normalized_transformed_graph(verbose: {:?})", &data.getunweightedsymmetricnormalizedtransformedgraph.verbose);
    }
    

    4 => {
        println!("get_weighted_laplacian_transformed_graph(verbose: {:?})", &data.getweightedlaplaciantransformedgraph.verbose);
    }
    

    5 => {
        println!("get_weighted_symmetric_normalized_laplacian_transformed_graph(verbose: {:?})", &data.getweightedsymmetricnormalizedlaplaciantransformedgraph.verbose);
    }
    

    6 => {
        println!("get_weighted_symmetric_normalized_transformed_graph(verbose: {:?})", &data.getweightedsymmetricnormalizedtransformedgraph.verbose);
    }
    

    7 => {
        println!("get_weighted_random_walk_normalized_laplacian_transformed_graph(verbose: {:?})", &data.getweightedrandomwalknormalizedlaplaciantransformedgraph.verbose);
    }
    

    8 => {
        println!("is_singleton_from_node_id(node_id: {:?})", &data.issingletonfromnodeid.node_id);
    }
    

    9 => {
        println!("is_singleton_with_selfloops_from_node_id(node_id: {:?})", &data.issingletonwithselfloopsfromnodeid.node_id);
    }
    

    10 => {
        println!("has_node_type_id(node_type_id: {:?})", &data.hasnodetypeid.node_type_id);
    }
    

    11 => {
        println!("has_edge_type_id(edge_type_id: {:?})", &data.hasedgetypeid.edge_type_id);
    }
    

    12 => {
        println!("has_edge_from_node_ids(src: {:?}, dst: {:?})", &data.hasedgefromnodeids.src, &data.hasedgefromnodeids.dst);
    }
    

    13 => {
        println!("has_selfloop_from_node_id(node_id: {:?})", &data.hasselfloopfromnodeid.node_id);
    }
    

    14 => {
        println!("has_edge_from_node_ids_and_edge_type_id(src: {:?}, dst: {:?}, edge_type: {:?})", &data.hasedgefromnodeidsandedgetypeid.src, &data.hasedgefromnodeidsandedgetypeid.dst, &data.hasedgefromnodeidsandedgetypeid.edge_type);
    }
    

    15 => {
        println!("is_trap_node_from_node_id(node_id: {:?})", &data.istrapnodefromnodeid.node_id);
    }
    

    16 => {
        println!("strongly_connected_components()", );
    }
    

    17 => {
        println!("sort_by_increasing_outbound_node_degree(verbose: {:?})", &data.sortbyincreasingoutboundnodedegree.verbose);
    }
    

    18 => {
        println!("sort_by_decreasing_outbound_node_degree(verbose: {:?})", &data.sortbydecreasingoutboundnodedegree.verbose);
    }
    

    19 => {
        println!("get_dense_binary_adjacency_matrix()", );
    }
    

    20 => {
        println!("get_dense_weighted_adjacency_matrix(weight: {:?})", &data.getdenseweightedadjacencymatrix.weight);
    }
    

    21 => {
        println!("iter_edge_ids_from_node_ids(src: {:?}, dst: {:?})", &data.iteredgeidsfromnodeids.src, &data.iteredgeidsfromnodeids.dst);
    }
    

    22 => {
        println!("iter_edge_node_ids_and_edge_type_id_from_edge_type_id(edge_type_id: {:?}, directed: {:?})", &data.iteredgenodeidsandedgetypeidfromedgetypeid.edge_type_id, &data.iteredgenodeidsandedgetypeidfromedgetypeid.directed);
    }
    

    23 => {
        println!("iter_node_ids_and_node_type_ids_from_node_type_id(node_type_id: {:?})", &data.iternodeidsandnodetypeidsfromnodetypeid.node_type_id);
    }
    

    24 => {
        println!("iter_node_names_and_node_type_names_from_node_type_id(node_type_id: {:?})", &data.iternodenamesandnodetypenamesfromnodetypeid.node_type_id);
    }
    

    25 => {
        println!("iter_edge_node_names_and_edge_type_name_from_edge_type_id(edge_type_id: {:?}, directed: {:?})", &data.iteredgenodenamesandedgetypenamefromedgetypeid.edge_type_id, &data.iteredgenodenamesandedgetypenamefromedgetypeid.directed);
    }
    

    26 => {
        println!("encode_edge(src: {:?}, dst: {:?})", &data.encodeedge.src, &data.encodeedge.dst);
    }
    

    27 => {
        println!("decode_edge(edge: {:?})", &data.decodeedge.edge);
    }
    

    28 => {
        println!("get_max_encodable_edge_number()", );
    }
    

    29 => {
        println!("validate_node_id(node_id: {:?})", &data.validatenodeid.node_id);
    }
    

    30 => {
        println!("validate_node_ids(node_ids: {:?})", &data.validatenodeids.node_ids);
    }
    

    31 => {
        println!("validate_edge_id(edge_id: {:?})", &data.validateedgeid.edge_id);
    }
    

    32 => {
        println!("validate_edge_ids(edge_ids: {:?})", &data.validateedgeids.edge_ids);
    }
    

    33 => {
        println!("must_not_contain_unknown_node_types()", );
    }
    

    34 => {
        println!("must_not_contain_unknown_edge_types()", );
    }
    

    35 => {
        println!("validate_node_type_id(node_type_id: {:?})", &data.validatenodetypeid.node_type_id);
    }
    

    36 => {
        println!("validate_node_type_ids(node_type_ids: {:?})", &data.validatenodetypeids.node_type_ids);
    }
    

    37 => {
        println!("validate_edge_type_id(edge_type_id: {:?})", &data.validateedgetypeid.edge_type_id);
    }
    

    38 => {
        println!("validate_edge_type_ids(edge_type_ids: {:?})", &data.validateedgetypeids.edge_type_ids);
    }
    

    39 => {
        println!("must_have_node_types()", );
    }
    

    40 => {
        println!("must_have_edge_types()", );
    }
    

    41 => {
        println!("must_be_undirected()", );
    }
    

    42 => {
        println!("must_be_multigraph()", );
    }
    

    43 => {
        println!("must_not_be_multigraph()", );
    }
    

    44 => {
        println!("must_have_edge_weights()", );
    }
    

    45 => {
        println!("must_have_edge_weights_representing_probabilities()", );
    }
    

    46 => {
        println!("must_have_positive_edge_weights()", );
    }
    

    47 => {
        println!("must_not_contain_weighted_singleton_nodes()", );
    }
    

    48 => {
        println!("must_have_edges()", );
    }
    

    49 => {
        println!("must_have_nodes()", );
    }
    

    50 => {
        println!("iter_node_ids()", );
    }
    

    51 => {
        println!("par_iter_node_ids()", );
    }
    

    52 => {
        println!("iter_node_names()", );
    }
    

    53 => {
        println!("par_iter_node_names()", );
    }
    

    54 => {
        println!("iter_unique_node_type_ids()", );
    }
    

    55 => {
        println!("iter_node_type_counts()", );
    }
    

    56 => {
        println!("iter_unique_node_type_ids_and_counts()", );
    }
    

    57 => {
        println!("iter_unique_node_type_names()", );
    }
    

    58 => {
        println!("iter_unique_node_type_names_and_counts()", );
    }
    

    59 => {
        println!("iter_unique_edge_type_ids()", );
    }
    

    60 => {
        println!("iter_edge_type_counts()", );
    }
    

    61 => {
        println!("iter_unique_edge_type_ids_and_counts()", );
    }
    

    62 => {
        println!("iter_unique_edge_type_names_and_counts()", );
    }
    

    63 => {
        println!("iter_unique_edge_type_names()", );
    }
    

    64 => {
        println!("iter_unweighted_node_degrees()", );
    }
    

    65 => {
        println!("par_iter_unweighted_node_degrees()", );
    }
    

    66 => {
        println!("iter_weighted_node_degrees()", );
    }
    

    67 => {
        println!("par_iter_weighted_node_degrees()", );
    }
    

    68 => {
        println!("iter_connected_node_ids()", );
    }
    

    69 => {
        println!("iter_singleton_node_ids()", );
    }
    

    70 => {
        println!("iter_singleton_node_names()", );
    }
    

    71 => {
        println!("iter_singleton_nodes_with_selfloops_node_ids()", );
    }
    

    72 => {
        println!("iter_singleton_nodes_with_selfloops_node_names()", );
    }
    

    73 => {
        println!("iter_singleton_node_type_ids()", );
    }
    

    74 => {
        println!("iter_singleton_edge_type_ids()", );
    }
    

    75 => {
        println!("iter_singleton_node_type_names()", );
    }
    

    76 => {
        println!("iter_singleton_edge_type_names()", );
    }
    

    77 => {
        println!("iter_source_node_ids(directed: {:?})", &data.itersourcenodeids.directed);
    }
    

    78 => {
        println!("iter_edge_weights()", );
    }
    

    79 => {
        println!("par_iter_edge_weights()", );
    }
    

    80 => {
        println!("par_iter_source_node_ids(directed: {:?})", &data.paritersourcenodeids.directed);
    }
    

    81 => {
        println!("iter_destination_node_ids(directed: {:?})", &data.iterdestinationnodeids.directed);
    }
    

    82 => {
        println!("par_iter_destination_node_ids(directed: {:?})", &data.pariterdestinationnodeids.directed);
    }
    

    83 => {
        println!("iter_node_ids_and_node_type_ids()", );
    }
    

    84 => {
        println!("iter_one_hot_encoded_node_type_ids()", );
    }
    

    85 => {
        println!("iter_one_hot_encoded_known_node_type_ids()", );
    }
    

    86 => {
        println!("iter_node_names_and_node_type_names()", );
    }
    

    87 => {
        println!("par_iter_node_names_and_node_type_names()", );
    }
    

    88 => {
        println!("iter_edge_node_ids(directed: {:?})", &data.iteredgenodeids.directed);
    }
    

    89 => {
        println!("iter_edges(directed: {:?})", &data.iteredges.directed);
    }
    

    90 => {
        println!("par_iter_edge_node_ids(directed: {:?})", &data.pariteredgenodeids.directed);
    }
    

    91 => {
        println!("par_iter_directed_edge_ids()", );
    }
    

    92 => {
        println!("par_iter_edges(directed: {:?})", &data.pariteredges.directed);
    }
    

    93 => {
        println!("iter_edge_node_ids_and_edge_weight()", );
    }
    

    94 => {
        println!("par_iter_edge_node_ids_and_edge_weight()", );
    }
    

    95 => {
        println!("iter_edge_node_ids_and_edge_type_id(directed: {:?})", &data.iteredgenodeidsandedgetypeid.directed);
    }
    

    96 => {
        println!("iter_one_hot_encoded_edge_type_ids()", );
    }
    

    97 => {
        println!("iter_one_hot_encoded_known_edge_type_ids()", );
    }
    

    98 => {
        println!("iter_edge_node_names_and_edge_type_name(directed: {:?})", &data.iteredgenodenamesandedgetypename.directed);
    }
    

    99 => {
        println!("par_iter_edge_node_names_and_edge_type_name(directed: {:?})", &data.pariteredgenodenamesandedgetypename.directed);
    }
    

    100 => {
        println!("par_iter_edge_node_ids_and_edge_type_id(directed: {:?})", &data.pariteredgenodeidsandedgetypeid.directed);
    }
    

    101 => {
        println!("par_iter_edge_node_names_and_edge_type_name_and_edge_weight(directed: {:?})", &data.pariteredgenodenamesandedgetypenameandedgeweight.directed);
    }
    

    102 => {
        println!("iter_edge_node_names_and_edge_type_name_and_edge_weight(directed: {:?})", &data.iteredgenodenamesandedgetypenameandedgeweight.directed);
    }
    

    103 => {
        println!("par_iter_edge_node_ids_and_edge_type_id_and_edge_weight(directed: {:?})", &data.pariteredgenodeidsandedgetypeidandedgeweight.directed);
    }
    

    104 => {
        println!("iter_edge_node_ids_and_edge_type_id_and_edge_weight(directed: {:?})", &data.iteredgenodeidsandedgetypeidandedgeweight.directed);
    }
    

    105 => {
        println!("iter_unique_edge_node_ids(directed: {:?})", &data.iteruniqueedgenodeids.directed);
    }
    

    106 => {
        println!("iter_unique_source_node_ids()", );
    }
    

    107 => {
        println!("iter_edge_ids_with_unknown_edge_types()", );
    }
    

    108 => {
        println!("iter_edge_ids_with_known_edge_types()", );
    }
    

    109 => {
        println!("iter_edge_node_ids_with_unknown_edge_types(directed: {:?})", &data.iteredgenodeidswithunknownedgetypes.directed);
    }
    

    110 => {
        println!("iter_edge_node_ids_with_known_edge_types(directed: {:?})", &data.iteredgenodeidswithknownedgetypes.directed);
    }
    

    111 => {
        println!("iter_node_ids_with_unknown_node_types()", );
    }
    

    112 => {
        println!("iter_node_ids_with_known_node_types()", );
    }
    

    113 => {
        println!("iter_edge_node_names_with_unknown_edge_types(directed: {:?})", &data.iteredgenodenameswithunknownedgetypes.directed);
    }
    

    114 => {
        println!("iter_edge_node_names_with_known_edge_types(directed: {:?})", &data.iteredgenodenameswithknownedgetypes.directed);
    }
    

    115 => {
        println!("iter_node_names_with_unknown_node_types()", );
    }
    

    116 => {
        println!("iter_node_names_with_known_node_types()", );
    }
    

    117 => {
        println!("par_iter_edge_ids_with_unknown_edge_types()", );
    }
    

    118 => {
        println!("par_iter_edge_ids_with_known_edge_types()", );
    }
    

    119 => {
        println!("par_iter_edge_node_ids_with_unknown_edge_types(directed: {:?})", &data.pariteredgenodeidswithunknownedgetypes.directed);
    }
    

    120 => {
        println!("par_iter_edge_node_ids_with_known_edge_types(directed: {:?})", &data.pariteredgenodeidswithknownedgetypes.directed);
    }
    

    121 => {
        println!("par_iter_node_ids_with_unknown_node_types()", );
    }
    

    122 => {
        println!("par_iter_node_ids_with_known_node_types()", );
    }
    

    123 => {
        println!("par_iter_edge_node_names_with_unknown_edge_types(directed: {:?})", &data.pariteredgenodenameswithunknownedgetypes.directed);
    }
    

    124 => {
        println!("par_iter_edge_node_names_with_known_edge_types(directed: {:?})", &data.pariteredgenodenameswithknownedgetypes.directed);
    }
    

    125 => {
        println!("par_iter_node_names_with_unknown_node_types()", );
    }
    

    126 => {
        println!("par_iter_node_names_with_known_node_types()", );
    }
    

    127 => {
        println!("remove_inplace_node_type_ids(node_type_ids_to_remove: {:?})", &data.removeinplacenodetypeids.node_type_ids_to_remove);
    }
    

    128 => {
        println!("remove_inplace_singleton_node_types()", );
    }
    

    129 => {
        println!("remove_inplace_edge_type_ids(edge_type_ids_to_remove: {:?})", &data.removeinplaceedgetypeids.edge_type_ids_to_remove);
    }
    

    130 => {
        println!("remove_inplace_singleton_edge_types()", );
    }
    

    131 => {
        println!("remove_node_type_id(node_type_id: {:?})", &data.removenodetypeid.node_type_id);
    }
    

    132 => {
        println!("remove_singleton_node_types()", );
    }
    

    133 => {
        println!("remove_edge_type_id(edge_type_id: {:?})", &data.removeedgetypeid.edge_type_id);
    }
    

    134 => {
        println!("remove_singleton_edge_types()", );
    }
    

    135 => {
        println!("remove_inplace_node_types()", );
    }
    

    136 => {
        println!("remove_node_types()", );
    }
    

    137 => {
        println!("remove_inplace_edge_types()", );
    }
    

    138 => {
        println!("remove_edge_types(verbose: {:?})", &data.removeedgetypes.verbose);
    }
    

    139 => {
        println!("remove_inplace_edge_weights()", );
    }
    

    140 => {
        println!("remove_edge_weights()", );
    }
    

    141 => {
        println!("get_unweighted_number_of_triangles(normalize: {:?}, low_centrality: {:?}, verbose: {:?})", &data.getunweightednumberoftriangles.normalize, &data.getunweightednumberoftriangles.low_centrality, &data.getunweightednumberoftriangles.verbose);
    }
    

    142 => {
        println!("get_unweighted_triads_number()", );
    }
    

    143 => {
        println!("get_weighted_triads_number()", );
    }
    

    144 => {
        println!("get_unweighted_transitivity(low_centrality: {:?}, verbose: {:?})", &data.getunweightedtransitivity.low_centrality, &data.getunweightedtransitivity.verbose);
    }
    

    145 => {
        println!("get_unweighted_number_of_triangles_per_node(normalize: {:?}, low_centrality: {:?}, verbose: {:?})", &data.getunweightednumberoftrianglespernode.normalize, &data.getunweightednumberoftrianglespernode.low_centrality, &data.getunweightednumberoftrianglespernode.verbose);
    }
    

    146 => {
        println!("iter_clustering_coefficient_per_node(low_centrality: {:?}, verbose: {:?})", &data.iterclusteringcoefficientpernode.low_centrality, &data.iterclusteringcoefficientpernode.verbose);
    }
    

    147 => {
        println!("get_clustering_coefficient_per_node(low_centrality: {:?}, verbose: {:?})", &data.getclusteringcoefficientpernode.low_centrality, &data.getclusteringcoefficientpernode.verbose);
    }
    

    148 => {
        println!("get_clustering_coefficient(low_centrality: {:?}, verbose: {:?})", &data.getclusteringcoefficient.low_centrality, &data.getclusteringcoefficient.verbose);
    }
    

    149 => {
        println!("get_average_clustering_coefficient(low_centrality: {:?}, verbose: {:?})", &data.getaverageclusteringcoefficient.low_centrality, &data.getaverageclusteringcoefficient.verbose);
    }
    

    150 => {
        println!("remap_from_node_ids(node_ids: {:?}, verbose: {:?})", &data.remapfromnodeids.node_ids, &data.remapfromnodeids.verbose);
    }
    

    151 => {
        println!("node_label_holdout(train_size: {:?}, use_stratification: {:?}, random_state: {:?})", &data.nodelabelholdout.train_size, &data.nodelabelholdout.use_stratification, &data.nodelabelholdout.random_state);
    }
    

    152 => {
        println!("edge_label_holdout(train_size: {:?}, use_stratification: {:?}, random_state: {:?})", &data.edgelabelholdout.train_size, &data.edgelabelholdout.use_stratification, &data.edgelabelholdout.random_state);
    }
    

    153 => {
        println!("random_subgraph(nodes_number: {:?}, random_state: {:?}, verbose: {:?})", &data.randomsubgraph.nodes_number, &data.randomsubgraph.random_state, &data.randomsubgraph.verbose);
    }
    

    154 => {
        println!("get_unweighted_minimum_path_node_ids_from_node_ids(src_node_id: {:?}, dst_node_id: {:?}, maximal_depth: {:?})", &data.getunweightedminimumpathnodeidsfromnodeids.src_node_id, &data.getunweightedminimumpathnodeidsfromnodeids.dst_node_id, &data.getunweightedminimumpathnodeidsfromnodeids.maximal_depth);
    }
    

    155 => {
        println!("get_unweighted_k_shortest_path_node_ids_from_node_ids(src_node_id: {:?}, dst_node_id: {:?}, k: {:?})", &data.getunweightedkshortestpathnodeidsfromnodeids.src_node_id, &data.getunweightedkshortestpathnodeidsfromnodeids.dst_node_id, &data.getunweightedkshortestpathnodeidsfromnodeids.k as usize);
    }
    

    156 => {
        println!("get_unweighted_eccentricity_from_node_id(node_id: {:?})", &data.getunweightedeccentricityfromnodeid.node_id);
    }
    

    157 => {
        println!("get_weighted_eccentricity_from_node_id(node_id: {:?}, use_edge_weights_as_probabilities: {:?})", &data.getweightedeccentricityfromnodeid.node_id, &data.getweightedeccentricityfromnodeid.use_edge_weights_as_probabilities);
    }
    

    158 => {
        println!("get_weighted_minimum_path_node_ids_from_node_ids(src_node_id: {:?}, dst_node_id: {:?}, use_edge_weights_as_probabilities: {:?}, maximal_depth: {:?})", &data.getweightedminimumpathnodeidsfromnodeids.src_node_id, &data.getweightedminimumpathnodeidsfromnodeids.dst_node_id, &data.getweightedminimumpathnodeidsfromnodeids.use_edge_weights_as_probabilities, &data.getweightedminimumpathnodeidsfromnodeids.maximal_depth);
    }
    

    159 => {
        println!("get_breath_first_search_from_node_ids(src_node_id: {:?}, maybe_dst_node_id: {:?}, maybe_dst_node_ids: {:?}, compute_distances: {:?}, compute_predecessors: {:?}, compute_visited: {:?}, maximal_depth: {:?})", &data.getbreathfirstsearchfromnodeids.src_node_id, &data.getbreathfirstsearchfromnodeids.maybe_dst_node_id, &data.getbreathfirstsearchfromnodeids.maybe_dst_node_ids, &data.getbreathfirstsearchfromnodeids.compute_distances, &data.getbreathfirstsearchfromnodeids.compute_predecessors, &data.getbreathfirstsearchfromnodeids.compute_visited, &data.getbreathfirstsearchfromnodeids.maximal_depth);
    }
    

    160 => {
        println!("get_dijkstra_from_node_ids(src_node_id: {:?}, maybe_dst_node_id: {:?}, maybe_dst_node_ids: {:?}, compute_predecessors: {:?}, maximal_depth: {:?}, use_edge_weights_as_probabilities: {:?})", &data.getdijkstrafromnodeids.src_node_id, &data.getdijkstrafromnodeids.maybe_dst_node_id, &data.getdijkstrafromnodeids.maybe_dst_node_ids, &data.getdijkstrafromnodeids.compute_predecessors, &data.getdijkstrafromnodeids.maximal_depth, &data.getdijkstrafromnodeids.use_edge_weights_as_probabilities);
    }
    

    161 => {
        println!("get_unweighted_diameter(ignore_infinity: {:?}, verbose: {:?})", &data.getunweighteddiameter.ignore_infinity, &data.getunweighteddiameter.verbose);
    }
    

    162 => {
        println!("get_weighted_diameter(ignore_infinity: {:?}, use_edge_weights_as_probabilities: {:?}, verbose: {:?})", &data.getweighteddiameter.ignore_infinity, &data.getweighteddiameter.use_edge_weights_as_probabilities, &data.getweighteddiameter.verbose);
    }
    

    163 => {
        println!("get_connected_components_number(verbose: {:?})", &data.getconnectedcomponentsnumber.verbose);
    }
    

    164 => {
        println!("get_singleton_nodes_number()", );
    }
    

    165 => {
        println!("get_weighted_singleton_nodes_number()", );
    }
    

    166 => {
        println!("get_disconnected_nodes_number()", );
    }
    

    167 => {
        println!("get_singleton_node_ids()", );
    }
    

    168 => {
        println!("get_singleton_node_names()", );
    }
    

    169 => {
        println!("get_singleton_nodes_with_selfloops_number()", );
    }
    

    170 => {
        println!("get_singleton_with_selfloops_node_ids()", );
    }
    

    171 => {
        println!("get_singleton_with_selfloops_node_names()", );
    }
    

    172 => {
        println!("get_connected_nodes_number()", );
    }
    

    173 => {
        println!("get_density()", );
    }
    

    174 => {
        println!("get_trap_nodes_rate()", );
    }
    

    175 => {
        println!("get_unweighted_node_degrees_mean()", );
    }
    

    176 => {
        println!("get_weighted_node_degrees_mean()", );
    }
    

    177 => {
        println!("get_undirected_edges_number()", );
    }
    

    178 => {
        println!("get_unique_undirected_edges_number()", );
    }
    

    179 => {
        println!("get_edges_number()", );
    }
    

    180 => {
        println!("get_unique_edges_number()", );
    }
    

    181 => {
        println!("get_unweighted_node_degrees_median()", );
    }
    

    182 => {
        println!("get_weighted_node_degrees_median()", );
    }
    

    183 => {
        println!("get_weighted_maximum_node_degree()", );
    }
    

    184 => {
        println!("get_weighted_minimum_node_degree()", );
    }
    

    185 => {
        println!("get_unweighted_maximum_node_degree()", );
    }
    

    186 => {
        println!("get_argmax_node_degree()", );
    }
    

    187 => {
        println!("get_weighted_mininum_node_degree()", );
    }
    

    188 => {
        println!("get_unweighted_minimum_node_degree()", );
    }
    

    189 => {
        println!("get_unweighted_node_degrees_mode()", );
    }
    

    190 => {
        println!("get_selfloop_number()", );
    }
    

    191 => {
        println!("get_unique_selfloop_number()", );
    }
    

    192 => {
        println!("get_selfloop_nodes_rate()", );
    }
    

    193 => {
        println!("get_name()", );
    }
    

    194 => {
        println!("get_trap_nodes_number()", );
    }
    

    195 => {
        println!("get_source_node_ids(directed: {:?})", &data.getsourcenodeids.directed);
    }
    

    196 => {
        println!("get_source_names(directed: {:?})", &data.getsourcenames.directed);
    }
    

    197 => {
        println!("get_destination_node_ids(directed: {:?})", &data.getdestinationnodeids.directed);
    }
    

    198 => {
        println!("get_destination_names(directed: {:?})", &data.getdestinationnames.directed);
    }
    

    199 => {
        println!("get_node_names()", );
    }
    

    200 => {
        println!("get_node_ids()", );
    }
    

    201 => {
        println!("get_edge_type_ids()", );
    }
    

    202 => {
        println!("get_unique_edge_type_ids()", );
    }
    

    203 => {
        println!("get_edge_type_names()", );
    }
    

    204 => {
        println!("get_unique_edge_type_names()", );
    }
    

    205 => {
        println!("get_edge_weights()", );
    }
    

    206 => {
        println!("get_total_edge_weights()", );
    }
    

    207 => {
        println!("get_mininum_edge_weight()", );
    }
    

    208 => {
        println!("get_maximum_edge_weight()", );
    }
    

    209 => {
        println!("get_node_type_ids()", );
    }
    

    210 => {
        println!("get_one_hot_encoded_node_types()", );
    }
    

    211 => {
        println!("get_one_hot_encoded_known_node_types()", );
    }
    

    212 => {
        println!("get_one_hot_encoded_edge_types()", );
    }
    

    213 => {
        println!("get_one_hot_encoded_known_edge_types()", );
    }
    

    214 => {
        println!("get_node_type_names()", );
    }
    

    215 => {
        println!("get_unique_node_type_ids()", );
    }
    

    216 => {
        println!("get_unique_node_type_names()", );
    }
    

    217 => {
        println!("get_unique_directed_edges_number()", );
    }
    

    218 => {
        println!("get_nodes_mapping()", );
    }
    

    219 => {
        println!("get_edge_node_ids(directed: {:?})", &data.getedgenodeids.directed);
    }
    

    220 => {
        println!("get_edge_node_names(directed: {:?})", &data.getedgenodenames.directed);
    }
    

    221 => {
        println!("get_unknown_node_types_number()", );
    }
    

    222 => {
        println!("get_known_node_types_number()", );
    }
    

    223 => {
        println!("get_unknown_node_types_rate()", );
    }
    

    224 => {
        println!("get_known_node_types_rate()", );
    }
    

    225 => {
        println!("get_minimum_node_types_number()", );
    }
    

    226 => {
        println!("get_maximum_node_types_number()", );
    }
    

    227 => {
        println!("get_maximum_multilabel_count()", );
    }
    

    228 => {
        println!("get_singleton_node_types_number()", );
    }
    

    229 => {
        println!("get_singleton_node_type_ids()", );
    }
    

    230 => {
        println!("get_singleton_node_type_names()", );
    }
    

    231 => {
        println!("get_unknown_edge_types_number()", );
    }
    

    232 => {
        println!("get_edge_ids_with_unknown_edge_types()", );
    }
    

    233 => {
        println!("get_edge_ids_with_known_edge_types()", );
    }
    

    234 => {
        println!("get_edge_node_ids_with_unknown_edge_types(directed: {:?})", &data.getedgenodeidswithunknownedgetypes.directed);
    }
    

    235 => {
        println!("get_edge_node_ids_with_known_edge_types(directed: {:?})", &data.getedgenodeidswithknownedgetypes.directed);
    }
    

    236 => {
        println!("get_edge_node_names_with_unknown_edge_types(directed: {:?})", &data.getedgenodenameswithunknownedgetypes.directed);
    }
    

    237 => {
        println!("get_edge_node_names_with_known_edge_types(directed: {:?})", &data.getedgenodenameswithknownedgetypes.directed);
    }
    

    238 => {
        println!("get_edge_ids_with_unknown_edge_types_mask()", );
    }
    

    239 => {
        println!("get_edge_ids_with_known_edge_types_mask()", );
    }
    

    240 => {
        println!("get_node_ids_with_unknown_node_types()", );
    }
    

    241 => {
        println!("get_node_ids_with_known_node_types()", );
    }
    

    242 => {
        println!("get_node_names_with_unknown_node_types()", );
    }
    

    243 => {
        println!("get_node_names_with_known_node_types()", );
    }
    

    244 => {
        println!("get_node_ids_with_unknown_node_types_mask()", );
    }
    

    245 => {
        println!("get_node_ids_with_known_node_types_mask()", );
    }
    

    246 => {
        println!("get_known_edge_types_number()", );
    }
    

    247 => {
        println!("get_unknown_edge_types_rate()", );
    }
    

    248 => {
        println!("get_known_edge_types_rate()", );
    }
    

    249 => {
        println!("get_minimum_edge_types_number()", );
    }
    

    250 => {
        println!("get_singleton_edge_types_number()", );
    }
    

    251 => {
        println!("get_singleton_edge_type_ids()", );
    }
    

    252 => {
        println!("get_singleton_edge_type_names()", );
    }
    

    253 => {
        println!("get_nodes_number()", );
    }
    

    254 => {
        println!("get_node_connected_component_ids(verbose: {:?})", &data.getnodeconnectedcomponentids.verbose);
    }
    

    255 => {
        println!("get_directed_edges_number()", );
    }
    

    256 => {
        println!("get_edge_types_number()", );
    }
    

    257 => {
        println!("get_node_types_number()", );
    }
    

    258 => {
        println!("get_unweighted_node_degrees()", );
    }
    

    259 => {
        println!("get_weighted_node_degrees()", );
    }
    

    260 => {
        println!("get_not_singletons_node_ids()", );
    }
    

    261 => {
        println!("get_dense_nodes_mapping()", );
    }
    

    262 => {
        println!("get_parallel_edges_number()", );
    }
    

    263 => {
        println!("get_cumulative_node_degrees()", );
    }
    

    264 => {
        println!("get_unique_source_nodes_number()", );
    }
    

    265 => {
        println!("get_edge_type_id_counts_hashmap()", );
    }
    

    266 => {
        println!("get_edge_type_names_counts_hashmap()", );
    }
    

    267 => {
        println!("get_node_type_id_counts_hashmap()", );
    }
    

    268 => {
        println!("get_node_type_names_counts_hashmap()", );
    }
    

    269 => {
        println!("to_directed_inplace()", );
    }
    

    270 => {
        println!("to_directed()", );
    }
    

    271 => {
        println!("to_upper_triangular(verbose: {:?})", &data.touppertriangular.verbose);
    }
    

    272 => {
        println!("to_lower_triangular(verbose: {:?})", &data.tolowertriangular.verbose);
    }
    

    273 => {
        println!("to_main_diagonal(verbose: {:?})", &data.tomaindiagonal.verbose);
    }
    

    274 => {
        println!("to_anti_diagonal(verbose: {:?})", &data.toantidiagonal.verbose);
    }
    

    275 => {
        println!("to_bidiagonal(verbose: {:?})", &data.tobidiagonal.verbose);
    }
    

    276 => {
        println!("to_arrowhead(verbose: {:?})", &data.toarrowhead.verbose);
    }
    

    277 => {
        println!("to_transposed(verbose: {:?})", &data.totransposed.verbose);
    }
    

    278 => {
        println!("to_complementary(verbose: {:?})", &data.tocomplementary.verbose);
    }
    

    279 => {
        println!("report()", );
    }
    

    280 => {
        println!("get_node_report_from_node_id(node_id: {:?})", &data.getnodereportfromnodeid.node_id);
    }
    

    281 => {
        println!("textual_report()", );
    }
    

    282 => {
        println!("filter_from_ids(node_ids_to_keep: {:?}, node_ids_to_filter: {:?}, node_type_ids_to_keep: {:?}, node_type_ids_to_filter: {:?}, node_type_id_to_keep: {:?}, node_type_id_to_filter: {:?}, edge_ids_to_keep: {:?}, edge_ids_to_filter: {:?}, edge_node_ids_to_keep: {:?}, edge_node_ids_to_filter: {:?}, edge_type_ids_to_keep: {:?}, edge_type_ids_to_filter: {:?}, min_edge_weight: {:?}, max_edge_weight: {:?}, filter_singleton_nodes: {:?}, filter_singleton_nodes_with_selfloop: {:?}, filter_selfloops: {:?}, filter_parallel_edges: {:?}, verbose: {:?})", &data.filterfromids.node_ids_to_keep, &data.filterfromids.node_ids_to_filter, &data.filterfromids.node_type_ids_to_keep, &data.filterfromids.node_type_ids_to_filter, &data.filterfromids.node_type_id_to_keep, &data.filterfromids.node_type_id_to_filter, &data.filterfromids.edge_ids_to_keep, &data.filterfromids.edge_ids_to_filter, &data.filterfromids.edge_node_ids_to_keep, &data.filterfromids.edge_node_ids_to_filter, &data.filterfromids.edge_type_ids_to_keep, &data.filterfromids.edge_type_ids_to_filter, &data.filterfromids.min_edge_weight, &data.filterfromids.max_edge_weight, &data.filterfromids.filter_singleton_nodes, &data.filterfromids.filter_singleton_nodes_with_selfloop, &data.filterfromids.filter_selfloops, &data.filterfromids.filter_parallel_edges, &data.filterfromids.verbose);
    }
    

    283 => {
        println!("drop_unknown_node_types(verbose: {:?})", &data.dropunknownnodetypes.verbose);
    }
    

    284 => {
        println!("drop_unknown_edge_types(verbose: {:?})", &data.dropunknownedgetypes.verbose);
    }
    

    285 => {
        println!("drop_singleton_nodes(verbose: {:?})", &data.dropsingletonnodes.verbose);
    }
    

    286 => {
        println!("drop_singleton_nodes_with_selfloops(verbose: {:?})", &data.dropsingletonnodeswithselfloops.verbose);
    }
    

    287 => {
        println!("drop_disconnected_nodes(verbose: {:?})", &data.dropdisconnectednodes.verbose);
    }
    

    288 => {
        println!("drop_selfloops(verbose: {:?})", &data.dropselfloops.verbose);
    }
    

    289 => {
        println!("drop_parallel_edges(verbose: {:?})", &data.dropparalleledges.verbose);
    }
    

    290 => {
        println!("spanning_arborescence_kruskal(verbose: {:?})", &data.spanningarborescencekruskal.verbose);
    }
    

    291 => {
        println!("spanning_arborescence(verbose: {:?})", &data.spanningarborescence.verbose);
    }
    

    292 => {
        println!("connected_components(verbose: {:?})", &data.connectedcomponents.verbose);
    }
    

    293 => {
        println!("enable(vector_sources: {:?}, vector_destinations: {:?}, vector_cumulative_node_degrees: {:?})", &data.enable.vector_sources, &data.enable.vector_destinations, &data.enable.vector_cumulative_node_degrees);
    }
    

    294 => {
        println!("disable_all()", );
    }
    

    295 => {
        println!("par_iter_approximated_vertex_cover()", );
    }
    

    296 => {
        println!("approximated_vertex_cover_set()", );
    }
    

    297 => {
        println!("get_node_label_prediction_tuple_from_node_ids(node_ids: {:?}, random_state: {:?}, include_central_node: {:?}, return_edge_weights: {:?}, offset: {:?}, max_neighbours: {:?})", &data.getnodelabelpredictiontuplefromnodeids.node_ids, &data.getnodelabelpredictiontuplefromnodeids.random_state, &data.getnodelabelpredictiontuplefromnodeids.include_central_node, &data.getnodelabelpredictiontuplefromnodeids.return_edge_weights, &data.getnodelabelpredictiontuplefromnodeids.offset, &data.getnodelabelpredictiontuplefromnodeids.max_neighbours);
    }
    

    298 => {
        println!("par_iter_edge_prediction_metrics(normalize: {:?}, verbose: {:?})", &data.pariteredgepredictionmetrics.normalize, &data.pariteredgepredictionmetrics.verbose);
    }
    

    299 => {
        println!("get_okapi_bm25_node_feature_propagation(features: {:?}, iterations: {:?}, maximal_distance: {:?}, k1: {:?}, b: {:?}, include_central_node: {:?}, verbose: {:?})", &data.getokapibm25nodefeaturepropagation.features, &data.getokapibm25nodefeaturepropagation.iterations.map(|x| x as usize), &data.getokapibm25nodefeaturepropagation.maximal_distance, &data.getokapibm25nodefeaturepropagation.k1, &data.getokapibm25nodefeaturepropagation.b, &data.getokapibm25nodefeaturepropagation.include_central_node, &data.getokapibm25nodefeaturepropagation.verbose);
    }
    

    300 => {
        println!("get_okapi_bm25_node_label_propagation(iterations: {:?}, maximal_distance: {:?}, k1: {:?}, b: {:?}, verbose: {:?})", &data.getokapibm25nodelabelpropagation.iterations.map(|x| x as usize), &data.getokapibm25nodelabelpropagation.maximal_distance, &data.getokapibm25nodelabelpropagation.k1, &data.getokapibm25nodelabelpropagation.b, &data.getokapibm25nodelabelpropagation.verbose);
    }
    

    301 => {
        println!("has_default_graph_name()", );
    }
    

    302 => {
        println!("has_nodes()", );
    }
    

    303 => {
        println!("has_edges()", );
    }
    

    304 => {
        println!("has_trap_nodes()", );
    }
    

    305 => {
        println!("is_directed()", );
    }
    

    306 => {
        println!("has_edge_weights()", );
    }
    

    307 => {
        println!("has_edge_weights_representing_probabilities()", );
    }
    

    308 => {
        println!("has_weighted_singleton_nodes()", );
    }
    

    309 => {
        println!("has_constant_edge_weights()", );
    }
    

    310 => {
        println!("has_negative_edge_weights()", );
    }
    

    311 => {
        println!("has_edge_types()", );
    }
    

    312 => {
        println!("has_selfloops()", );
    }
    

    313 => {
        println!("has_disconnected_nodes()", );
    }
    

    314 => {
        println!("has_singleton_nodes()", );
    }
    

    315 => {
        println!("has_singleton_nodes_with_selfloops()", );
    }
    

    316 => {
        println!("is_connected(verbose: {:?})", &data.isconnected.verbose);
    }
    

    317 => {
        println!("has_node_types()", );
    }
    

    318 => {
        println!("has_multilabel_node_types()", );
    }
    

    319 => {
        println!("has_unknown_node_types()", );
    }
    

    320 => {
        println!("has_known_node_types()", );
    }
    

    321 => {
        println!("has_unknown_edge_types()", );
    }
    

    322 => {
        println!("has_known_edge_types()", );
    }
    

    323 => {
        println!("has_homogeneous_node_types()", );
    }
    

    324 => {
        println!("has_homogeneous_edge_types()", );
    }
    

    325 => {
        println!("has_singleton_node_types()", );
    }
    

    326 => {
        println!("has_node_oddities()", );
    }
    

    327 => {
        println!("has_node_types_oddities()", );
    }
    

    328 => {
        println!("has_singleton_edge_types()", );
    }
    

    329 => {
        println!("has_edge_types_oddities()", );
    }
    

    330 => {
        println!("is_multigraph()", );
    }
    

    331 => {
        println!("has_nodes_sorted_by_decreasing_outbound_node_degree()", );
    }
    

    332 => {
        println!("has_nodes_sorted_by_increasing_outbound_node_degree()", );
    }
    

    333 => {
        println!("get_transitive_closure(iterations: {:?}, verbose: {:?})", &data.gettransitiveclosure.iterations.map(|x| x as NodeT), &data.gettransitiveclosure.verbose);
    }
    

    334 => {
        println!("get_unweighted_all_shortest_paths(iterations: {:?}, verbose: {:?})", &data.getunweightedallshortestpaths.iterations.map(|x| x as NodeT), &data.getunweightedallshortestpaths.verbose);
    }
    

    335 => {
        println!("get_weighted_all_shortest_paths(iterations: {:?}, use_edge_weights_as_probabilities: {:?}, verbose: {:?})", &data.getweightedallshortestpaths.iterations.map(|x| x as NodeT), &data.getweightedallshortestpaths.use_edge_weights_as_probabilities, &data.getweightedallshortestpaths.verbose);
    }
    

    336 => {
        println!("get_source_node_id_from_edge_id(edge_id: {:?})", &data.getsourcenodeidfromedgeid.edge_id);
    }
    

    337 => {
        println!("get_destination_node_id_from_edge_id(edge_id: {:?})", &data.getdestinationnodeidfromedgeid.edge_id);
    }
    

    338 => {
        println!("get_source_node_name_from_edge_id(edge_id: {:?})", &data.getsourcenodenamefromedgeid.edge_id);
    }
    

    339 => {
        println!("get_destination_node_name_from_edge_id(edge_id: {:?})", &data.getdestinationnodenamefromedgeid.edge_id);
    }
    

    340 => {
        println!("get_node_names_from_edge_id(edge_id: {:?})", &data.getnodenamesfromedgeid.edge_id);
    }
    

    341 => {
        println!("get_node_ids_from_edge_id(edge_id: {:?})", &data.getnodeidsfromedgeid.edge_id);
    }
    

    342 => {
        println!("get_edge_id_from_node_ids(src: {:?}, dst: {:?})", &data.getedgeidfromnodeids.src, &data.getedgeidfromnodeids.dst);
    }
    

    343 => {
        println!("get_node_ids_and_edge_type_id_from_edge_id(edge_id: {:?})", &data.getnodeidsandedgetypeidfromedgeid.edge_id);
    }
    

    344 => {
        println!("get_node_ids_and_edge_type_id_and_edge_weight_from_edge_id(edge_id: {:?})", &data.getnodeidsandedgetypeidandedgeweightfromedgeid.edge_id);
    }
    

    345 => {
        println!("get_unweighted_top_k_central_node_ids(k: {:?})", &data.getunweightedtopkcentralnodeids.k);
    }
    

    346 => {
        println!("get_weighted_top_k_central_node_ids(k: {:?})", &data.getweightedtopkcentralnodeids.k);
    }
    

    347 => {
        println!("get_unweighted_node_degree_from_node_id(node_id: {:?})", &data.getunweightednodedegreefromnodeid.node_id);
    }
    

    348 => {
        println!("get_weighted_node_degree_from_node_id(node_id: {:?})", &data.getweightednodedegreefromnodeid.node_id);
    }
    

    349 => {
        println!("get_top_k_central_node_names(k: {:?})", &data.gettopkcentralnodenames.k);
    }
    

    350 => {
        println!("get_node_type_id_from_node_id(node_id: {:?})", &data.getnodetypeidfromnodeid.node_id);
    }
    

    351 => {
        println!("get_edge_type_id_from_edge_id(edge_id: {:?})", &data.getedgetypeidfromedgeid.edge_id);
    }
    

    352 => {
        println!("get_node_type_names_from_node_id(node_id: {:?})", &data.getnodetypenamesfromnodeid.node_id);
    }
    

    353 => {
        println!("get_edge_type_name_from_edge_id(edge_id: {:?})", &data.getedgetypenamefromedgeid.edge_id);
    }
    

    354 => {
        println!("get_edge_type_name_from_edge_type_id(edge_type_id: {:?})", &data.getedgetypenamefromedgetypeid.edge_type_id);
    }
    

    355 => {
        println!("get_edge_weight_from_edge_id(edge_id: {:?})", &data.getedgeweightfromedgeid.edge_id);
    }
    

    356 => {
        println!("get_edge_weight_from_node_ids(src: {:?}, dst: {:?})", &data.getedgeweightfromnodeids.src, &data.getedgeweightfromnodeids.dst);
    }
    

    357 => {
        println!("get_edge_weight_from_node_ids_and_edge_type_id(src: {:?}, dst: {:?}, edge_type: {:?})", &data.getedgeweightfromnodeidsandedgetypeid.src, &data.getedgeweightfromnodeidsandedgetypeid.dst, &data.getedgeweightfromnodeidsandedgetypeid.edge_type);
    }
    

    358 => {
        println!("get_node_name_from_node_id(node_id: {:?})", &data.getnodenamefromnodeid.node_id);
    }
    

    359 => {
        println!("get_edge_node_names_from_edge_node_ids(edge_node_ids: {:?})", &data.getedgenodenamesfromedgenodeids.edge_node_ids);
    }
    

    360 => {
        println!("get_edge_count_from_edge_type_id(edge_type_id: {:?})", &data.getedgecountfromedgetypeid.edge_type_id);
    }
    

    361 => {
        println!("get_node_count_from_node_type_id(node_type_id: {:?})", &data.getnodecountfromnodetypeid.node_type_id);
    }
    

    362 => {
        println!("get_neighbour_node_ids_from_node_id(node_id: {:?})", &data.getneighbournodeidsfromnodeid.node_id);
    }
    

    363 => {
        println!("get_minmax_edge_ids_from_node_ids(src: {:?}, dst: {:?})", &data.getminmaxedgeidsfromnodeids.src, &data.getminmaxedgeidsfromnodeids.dst);
    }
    

    364 => {
        println!("get_edge_id_from_node_ids_and_edge_type_id(src: {:?}, dst: {:?}, edge_type: {:?})", &data.getedgeidfromnodeidsandedgetypeid.src, &data.getedgeidfromnodeidsandedgetypeid.dst, &data.getedgeidfromnodeidsandedgetypeid.edge_type);
    }
    

    365 => {
        println!("get_minmax_edge_ids_from_source_node_id(src: {:?})", &data.getminmaxedgeidsfromsourcenodeid.src);
    }
    

    366 => {
        println!("get_node_type_name_from_node_type_id(node_type_id: {:?})", &data.getnodetypenamefromnodetypeid.node_type_id);
    }
    

    367 => {
        println!("iter_unweighted_degree_centrality()", );
    }
    

    368 => {
        println!("par_iter_weighted_degree_centrality()", );
    }
    

    369 => {
        println!("get_unweighted_degree_centrality()", );
    }
    

    370 => {
        println!("get_weighted_degree_centrality()", );
    }
    

    371 => {
        println!("par_iter_unweighted_closeness_centrality(verbose: {:?})", &data.pariterunweightedclosenesscentrality.verbose);
    }
    

    372 => {
        println!("par_iter_weighted_closeness_centrality(use_edge_weights_as_probabilities: {:?}, verbose: {:?})", &data.pariterweightedclosenesscentrality.use_edge_weights_as_probabilities, &data.pariterweightedclosenesscentrality.verbose);
    }
    

    373 => {
        println!("get_unweighted_closeness_centrality(verbose: {:?})", &data.getunweightedclosenesscentrality.verbose);
    }
    

    374 => {
        println!("get_weighted_closeness_centrality(use_edge_weights_as_probabilities: {:?}, verbose: {:?})", &data.getweightedclosenesscentrality.use_edge_weights_as_probabilities, &data.getweightedclosenesscentrality.verbose);
    }
    

    375 => {
        println!("par_iter_unweighted_harmonic_centrality(verbose: {:?})", &data.pariterunweightedharmoniccentrality.verbose);
    }
    

    376 => {
        println!("par_iter_weighted_harmonic_centrality(use_edge_weights_as_probabilities: {:?}, verbose: {:?})", &data.pariterweightedharmoniccentrality.use_edge_weights_as_probabilities, &data.pariterweightedharmoniccentrality.verbose);
    }
    

    377 => {
        println!("get_unweighted_harmonic_centrality(verbose: {:?})", &data.getunweightedharmoniccentrality.verbose);
    }
    

    378 => {
        println!("get_weighted_harmonic_centrality(use_edge_weights_as_probabilities: {:?}, verbose: {:?})", &data.getweightedharmoniccentrality.use_edge_weights_as_probabilities, &data.getweightedharmoniccentrality.verbose);
    }
    

    379 => {
        println!("get_stress_centrality(normalize: {:?}, verbose: {:?})", &data.getstresscentrality.normalize, &data.getstresscentrality.verbose);
    }
    

    380 => {
        println!("get_betweenness_centrality(normalize: {:?}, verbose: {:?})", &data.getbetweennesscentrality.normalize, &data.getbetweennesscentrality.verbose);
    }
    

    381 => {
        println!("get_unweighted_eigenvector_centrality(maximum_iterations_number: {:?}, tollerance: {:?})", &data.getunweightedeigenvectorcentrality.maximum_iterations_number.map(|x| x as usize), &data.getunweightedeigenvectorcentrality.tollerance);
    }
    

    382 => {
        println!("get_weighted_eigenvector_centrality(maximum_iterations_number: {:?}, tollerance: {:?})", &data.getweightedeigenvectorcentrality.maximum_iterations_number.map(|x| x as usize), &data.getweightedeigenvectorcentrality.tollerance);
    }
    

    383 => {
        println!("to_dot(use_node_names: {:?})", &data.todot.use_node_names);
    }
    

    384 => {
        println!("compute_hash()", );
    }
    

    385 => {
        println!("get_unweighted_preferential_attachment_from_node_ids(source_node_id: {:?}, destination_node_id: {:?}, normalize: {:?})", &data.getunweightedpreferentialattachmentfromnodeids.source_node_id, &data.getunweightedpreferentialattachmentfromnodeids.destination_node_id, &data.getunweightedpreferentialattachmentfromnodeids.normalize);
    }
    

    386 => {
        println!("get_weighted_preferential_attachment_from_node_ids(source_node_id: {:?}, destination_node_id: {:?}, normalize: {:?})", &data.getweightedpreferentialattachmentfromnodeids.source_node_id, &data.getweightedpreferentialattachmentfromnodeids.destination_node_id, &data.getweightedpreferentialattachmentfromnodeids.normalize);
    }
    

    387 => {
        println!("get_jaccard_coefficient_from_node_ids(source_node_id: {:?}, destination_node_id: {:?})", &data.getjaccardcoefficientfromnodeids.source_node_id, &data.getjaccardcoefficientfromnodeids.destination_node_id);
    }
    

    388 => {
        println!("get_adamic_adar_index_from_node_ids(source_node_id: {:?}, destination_node_id: {:?})", &data.getadamicadarindexfromnodeids.source_node_id, &data.getadamicadarindexfromnodeids.destination_node_id);
    }
    

    389 => {
        println!("get_unweighted_resource_allocation_index_from_node_ids(source_node_id: {:?}, destination_node_id: {:?})", &data.getunweightedresourceallocationindexfromnodeids.source_node_id, &data.getunweightedresourceallocationindexfromnodeids.destination_node_id);
    }
    

    390 => {
        println!("get_weighted_resource_allocation_index_from_node_ids(source_node_id: {:?}, destination_node_id: {:?})", &data.getweightedresourceallocationindexfromnodeids.source_node_id, &data.getweightedresourceallocationindexfromnodeids.destination_node_id);
    }
    
            _ => unreachable!()
        }
    }

    Ok(())
}
