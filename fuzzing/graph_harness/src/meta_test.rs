
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
pub struct GetNodeLabelPredictionMiniBatch {
    pub idx : u64,
    pub batch_size : Option<NodeT>,
    pub include_central_node : Option<bool>,
    pub return_edge_weights : Option<bool>,
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
pub struct SortByIncreasingOutboundNodeDegree {
    pub verbose : Option<bool>,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct SortByDecreasingOutboundNodeDegree {
    pub verbose : Option<bool>,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct ToDot {
    pub use_node_names : Option<bool>,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct RemapFromNodeIds {
    pub node_ids : Vec<NodeT>,
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
pub struct GetPreferentialAttachmentFromNodeIds {
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
pub struct GetResourceAllocationIndexFromNodeIds {
    pub source_node_id : NodeT,
    pub destination_node_id : NodeT,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct GetWeightedResourceAllocationIndexFromNodeIds {
    pub source_node_id : NodeT,
    pub destination_node_id : NodeT,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct Enable {
    pub vector_sources : Option<bool>,
    pub vector_destinations : Option<bool>,
    pub vector_cumulative_node_degrees : Option<bool>,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct GetNumberOfTriangles {
    pub normalize : Option<bool>,
    pub low_centrality : Option<usize>,
    pub verbose : Option<bool>,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct GetTransitivity {
    pub low_centrality : Option<usize>,
    pub verbose : Option<bool>,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct GetNumberOfTrianglesPerNode {
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
pub struct GetMinimumPathNodeIdsFromNodeIds {
    pub src_node_id : NodeT,
    pub dst_node_id : NodeT,
    pub maximal_depth : Option<NodeT>,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct GetKShortestPathNodeIdsFromNodeIds {
    pub src_node_id : NodeT,
    pub dst_node_id : NodeT,
    pub k : u8,
    pub max_path_length : Option<NodeT>,
    pub verbose : Option<bool>,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct GetEccentricityFromNodeId {
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
    pub dst_node_id : Option<NodeT>,
    pub compute_predecessors : Option<bool>,
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
pub struct GetDiameterNaive {
    pub ignore_infinity : Option<bool>,
    pub verbose : Option<bool>,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct GetDiameter {
    pub ignore_infinity : Option<bool>,
    pub verbose : Option<bool>,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct GetWeightedDiameterNaive {
    pub ignore_infinity : Option<bool>,
    pub use_edge_weights_as_probabilities : Option<bool>,
    pub verbose : Option<bool>,
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
    pub iterations : Option<u8>,
    pub verbose : Option<bool>,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct GetAllShortestPaths {
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
pub struct GetTopKCentralNodeIds {
    pub k : NodeT,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct GetWeightedTopKCentralNodeIds {
    pub k : NodeT,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct GetNodeDegreeFromNodeId {
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
pub struct ParIterClosenessCentrality {
    pub verbose : Option<bool>,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct ParIterWeightedClosenessCentrality {
    pub use_edge_weights_as_probabilities : Option<bool>,
    pub verbose : Option<bool>,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct GetClosenessCentrality {
    pub verbose : Option<bool>,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct GetWeightedClosenessCentrality {
    pub use_edge_weights_as_probabilities : Option<bool>,
    pub verbose : Option<bool>,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct ParIterHarmonicCentrality {
    pub verbose : Option<bool>,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct ParIterWeightedHarmonicCentrality {
    pub use_edge_weights_as_probabilities : Option<bool>,
    pub verbose : Option<bool>,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct GetHarmonicCentrality {
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
pub struct GetEigenvectorCentrality {
    pub maximum_iterations_number : Option<u8>,
    pub tollerance : Option<f64>,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct GetWeightedEigenvectorCentrality {
    pub maximum_iterations_number : Option<u8>,
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
pub struct GetLaplacianTransformedGraph {
    pub verbose : Option<bool>,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct GetRandomWalkNormalizedLaplacianTransformedGraph {
    pub verbose : Option<bool>,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct GetSymmetricNormalizedLaplacianTransformedGraph {
    pub verbose : Option<bool>,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct GetSymmetricNormalizedTransformedGraph {
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
pub struct GetNodeLabelRandomHoldout {
    pub train_size : f64,
    pub use_stratification : Option<bool>,
    pub random_state : Option<EdgeT>,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct GetNodeLabelKfold {
    pub k : usize,
    pub k_index : usize,
    pub use_stratification : Option<bool>,
    pub random_state : Option<EdgeT>,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct GetEdgeLabelRandomHoldout {
    pub train_size : f64,
    pub use_stratification : Option<bool>,
    pub random_state : Option<EdgeT>,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct GetEdgeLabelKfold {
    pub k : usize,
    pub k_index : usize,
    pub use_stratification : Option<bool>,
    pub random_state : Option<EdgeT>,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct GetRandomSubgraph {
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
    pub getnodelabelpredictionminibatch : GetNodeLabelPredictionMiniBatch,
    pub pariteredgepredictionmetrics : ParIterEdgePredictionMetrics,
    pub getokapibm25nodefeaturepropagation : GetOkapiBm25NodeFeaturePropagation,
    pub getokapibm25nodelabelpropagation : GetOkapiBm25NodeLabelPropagation,
    pub sortbyincreasingoutboundnodedegree : SortByIncreasingOutboundNodeDegree,
    pub sortbydecreasingoutboundnodedegree : SortByDecreasingOutboundNodeDegree,
    pub todot : ToDot,
    pub remapfromnodeids : RemapFromNodeIds,
    pub issingletonfromnodeid : IsSingletonFromNodeId,
    pub issingletonwithselfloopsfromnodeid : IsSingletonWithSelfloopsFromNodeId,
    pub hasnodetypeid : HasNodeTypeId,
    pub hasedgetypeid : HasEdgeTypeId,
    pub hasedgefromnodeids : HasEdgeFromNodeIds,
    pub hasselfloopfromnodeid : HasSelfloopFromNodeId,
    pub hasedgefromnodeidsandedgetypeid : HasEdgeFromNodeIdsAndEdgeTypeId,
    pub istrapnodefromnodeid : IsTrapNodeFromNodeId,
    pub getpreferentialattachmentfromnodeids : GetPreferentialAttachmentFromNodeIds,
    pub getweightedpreferentialattachmentfromnodeids : GetWeightedPreferentialAttachmentFromNodeIds,
    pub getjaccardcoefficientfromnodeids : GetJaccardCoefficientFromNodeIds,
    pub getadamicadarindexfromnodeids : GetAdamicAdarIndexFromNodeIds,
    pub getresourceallocationindexfromnodeids : GetResourceAllocationIndexFromNodeIds,
    pub getweightedresourceallocationindexfromnodeids : GetWeightedResourceAllocationIndexFromNodeIds,
    pub enable : Enable,

    pub getnumberoftriangles : GetNumberOfTriangles,


    pub gettransitivity : GetTransitivity,
    pub getnumberoftrianglespernode : GetNumberOfTrianglesPerNode,
    pub iterclusteringcoefficientpernode : IterClusteringCoefficientPerNode,
    pub getclusteringcoefficientpernode : GetClusteringCoefficientPerNode,
    pub getclusteringcoefficient : GetClusteringCoefficient,
    pub getaverageclusteringcoefficient : GetAverageClusteringCoefficient,
    pub getminimumpathnodeidsfromnodeids : GetMinimumPathNodeIdsFromNodeIds,
    pub getkshortestpathnodeidsfromnodeids : GetKShortestPathNodeIdsFromNodeIds,
    pub geteccentricityfromnodeid : GetEccentricityFromNodeId,
    pub getweightedeccentricityfromnodeid : GetWeightedEccentricityFromNodeId,
    pub getweightedminimumpathnodeidsfromnodeids : GetWeightedMinimumPathNodeIdsFromNodeIds,
    pub getbreathfirstsearchfromnodeids : GetBreathFirstSearchFromNodeIds,
    pub getdijkstrafromnodeids : GetDijkstraFromNodeIds,
    pub getdiameternaive : GetDiameterNaive,
    pub getdiameter : GetDiameter,
    pub getweighteddiameternaive : GetWeightedDiameterNaive,
    pub iteredgeidsfromnodeids : IterEdgeIdsFromNodeIds,
    pub iteredgenodeidsandedgetypeidfromedgetypeid : IterEdgeNodeIdsAndEdgeTypeIdFromEdgeTypeId,
    pub iternodeidsandnodetypeidsfromnodetypeid : IterNodeIdsAndNodeTypeIdsFromNodeTypeId,
    pub iternodenamesandnodetypenamesfromnodetypeid : IterNodeNamesAndNodeTypeNamesFromNodeTypeId,
    pub iteredgenodenamesandedgetypenamefromedgetypeid : IterEdgeNodeNamesAndEdgeTypeNameFromEdgeTypeId,
    pub gettransitiveclosure : GetTransitiveClosure,
    pub getallshortestpaths : GetAllShortestPaths,
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
    pub gettopkcentralnodeids : GetTopKCentralNodeIds,
    pub getweightedtopkcentralnodeids : GetWeightedTopKCentralNodeIds,
    pub getnodedegreefromnodeid : GetNodeDegreeFromNodeId,
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
    pub filterfromids : FilterFromIds,
    pub dropunknownnodetypes : DropUnknownNodeTypes,
    pub dropunknownedgetypes : DropUnknownEdgeTypes,
    pub dropsingletonnodes : DropSingletonNodes,
    pub dropsingletonnodeswithselfloops : DropSingletonNodesWithSelfloops,
    pub dropdisconnectednodes : DropDisconnectedNodes,
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



















    pub pariterclosenesscentrality : ParIterClosenessCentrality,
    pub pariterweightedclosenesscentrality : ParIterWeightedClosenessCentrality,
    pub getclosenesscentrality : GetClosenessCentrality,
    pub getweightedclosenesscentrality : GetWeightedClosenessCentrality,
    pub pariterharmoniccentrality : ParIterHarmonicCentrality,
    pub pariterweightedharmoniccentrality : ParIterWeightedHarmonicCentrality,
    pub getharmoniccentrality : GetHarmonicCentrality,
    pub getweightedharmoniccentrality : GetWeightedHarmonicCentrality,
    pub getstresscentrality : GetStressCentrality,
    pub getbetweennesscentrality : GetBetweennessCentrality,
    pub geteigenvectorcentrality : GetEigenvectorCentrality,
    pub getweightedeigenvectorcentrality : GetWeightedEigenvectorCentrality,
    pub removeinplacenodetypeids : RemoveInplaceNodeTypeIds,

    pub removeinplaceedgetypeids : RemoveInplaceEdgeTypeIds,

    pub removenodetypeid : RemoveNodeTypeId,

    pub removeedgetypeid : RemoveEdgeTypeId,




    pub removeedgetypes : RemoveEdgeTypes,


    pub encodeedge : EncodeEdge,
    pub decodeedge : DecodeEdge,




    pub getnodereportfromnodeid : GetNodeReportFromNodeId,

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


    pub iteredgenodenameswithunknownedgetypes : IterEdgeNodeNamesWithUnknownEdgeTypes,
    pub iteredgenodenameswithknownedgetypes : IterEdgeNodeNamesWithKnownEdgeTypes,




    pub pariteredgenodeidswithunknownedgetypes : ParIterEdgeNodeIdsWithUnknownEdgeTypes,
    pub pariteredgenodeidswithknownedgetypes : ParIterEdgeNodeIdsWithKnownEdgeTypes,


    pub pariteredgenodenameswithunknownedgetypes : ParIterEdgeNodeNamesWithUnknownEdgeTypes,
    pub pariteredgenodenameswithknownedgetypes : ParIterEdgeNodeNamesWithKnownEdgeTypes,


    pub getlaplaciantransformedgraph : GetLaplacianTransformedGraph,
    pub getrandomwalknormalizedlaplaciantransformedgraph : GetRandomWalkNormalizedLaplacianTransformedGraph,
    pub getsymmetricnormalizedlaplaciantransformedgraph : GetSymmetricNormalizedLaplacianTransformedGraph,
    pub getsymmetricnormalizedtransformedgraph : GetSymmetricNormalizedTransformedGraph,
    pub getweightedlaplaciantransformedgraph : GetWeightedLaplacianTransformedGraph,
    pub getweightedsymmetricnormalizedlaplaciantransformedgraph : GetWeightedSymmetricNormalizedLaplacianTransformedGraph,
    pub getweightedsymmetricnormalizedtransformedgraph : GetWeightedSymmetricNormalizedTransformedGraph,
    pub getweightedrandomwalknormalizedlaplaciantransformedgraph : GetWeightedRandomWalkNormalizedLaplacianTransformedGraph,















    pub isconnected : IsConnected,



















    pub touppertriangular : ToUpperTriangular,
    pub tolowertriangular : ToLowerTriangular,
    pub tomaindiagonal : ToMainDiagonal,
    pub toantidiagonal : ToAntiDiagonal,
    pub tobidiagonal : ToBidiagonal,
    pub toarrowhead : ToArrowhead,
    pub totransposed : ToTransposed,
    pub tocomplementary : ToComplementary,
    pub getnodelabelrandomholdout : GetNodeLabelRandomHoldout,
    pub getnodelabelkfold : GetNodeLabelKfold,
    pub getedgelabelrandomholdout : GetEdgeLabelRandomHoldout,
    pub getedgelabelkfold : GetEdgeLabelKfold,
    pub getrandomsubgraph : GetRandomSubgraph,
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
        match rng.next() % 400 {

    0 => {
        trace.push(format!("spanning_arborescence_kruskal(verbose: {:?})", &data.spanningarborescencekruskal.verbose));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.spanning_arborescence_kruskal(data.spanningarborescencekruskal.verbose.clone());
    }
    

    1 => {
        trace.push(format!("spanning_arborescence(verbose: {:?})", &data.spanningarborescence.verbose));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.spanning_arborescence(data.spanningarborescence.verbose.clone());
    }
    

    2 => {
        trace.push(format!("connected_components(verbose: {:?})", &data.connectedcomponents.verbose));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.connected_components(data.connectedcomponents.verbose.clone());
    }
    

    3 => {
        trace.push(format!("get_node_label_prediction_mini_batch(idx: {:?}, batch_size: {:?}, include_central_node: {:?}, return_edge_weights: {:?}, max_neighbours: {:?})", &data.getnodelabelpredictionminibatch.idx, &data.getnodelabelpredictionminibatch.batch_size, &data.getnodelabelpredictionminibatch.include_central_node, &data.getnodelabelpredictionminibatch.return_edge_weights, &data.getnodelabelpredictionminibatch.max_neighbours));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_node_label_prediction_mini_batch(data.getnodelabelpredictionminibatch.idx.clone(), data.getnodelabelpredictionminibatch.batch_size.clone(), data.getnodelabelpredictionminibatch.include_central_node.clone(), data.getnodelabelpredictionminibatch.return_edge_weights.clone(), data.getnodelabelpredictionminibatch.max_neighbours.clone());
    }
    

    4 => {
        trace.push(format!("par_iter_edge_prediction_metrics(normalize: {:?}, verbose: {:?})", &data.pariteredgepredictionmetrics.normalize, &data.pariteredgepredictionmetrics.verbose));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.par_iter_edge_prediction_metrics(data.pariteredgepredictionmetrics.normalize.clone(), data.pariteredgepredictionmetrics.verbose.clone()).collect::<Vec<_>>();
            
    }
    

    5 => {
        trace.push(format!("get_okapi_bm25_node_feature_propagation(features: {:?}, iterations: {:?}, maximal_distance: {:?}, k1: {:?}, b: {:?}, include_central_node: {:?}, verbose: {:?})", &data.getokapibm25nodefeaturepropagation.features, &data.getokapibm25nodefeaturepropagation.iterations.map(|x| x as usize), &data.getokapibm25nodefeaturepropagation.maximal_distance, &data.getokapibm25nodefeaturepropagation.k1, &data.getokapibm25nodefeaturepropagation.b, &data.getokapibm25nodefeaturepropagation.include_central_node, &data.getokapibm25nodefeaturepropagation.verbose));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_okapi_bm25_node_feature_propagation(data.getokapibm25nodefeaturepropagation.features.clone(), data.getokapibm25nodefeaturepropagation.iterations.map(|x| x as usize).clone(), data.getokapibm25nodefeaturepropagation.maximal_distance.clone(), data.getokapibm25nodefeaturepropagation.k1.clone(), data.getokapibm25nodefeaturepropagation.b.clone(), data.getokapibm25nodefeaturepropagation.include_central_node.clone(), data.getokapibm25nodefeaturepropagation.verbose.clone());
    }
    

    6 => {
        trace.push(format!("get_okapi_bm25_node_label_propagation(iterations: {:?}, maximal_distance: {:?}, k1: {:?}, b: {:?}, verbose: {:?})", &data.getokapibm25nodelabelpropagation.iterations.map(|x| x as usize), &data.getokapibm25nodelabelpropagation.maximal_distance, &data.getokapibm25nodelabelpropagation.k1, &data.getokapibm25nodelabelpropagation.b, &data.getokapibm25nodelabelpropagation.verbose));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_okapi_bm25_node_label_propagation(data.getokapibm25nodelabelpropagation.iterations.map(|x| x as usize).clone(), data.getokapibm25nodelabelpropagation.maximal_distance.clone(), data.getokapibm25nodelabelpropagation.k1.clone(), data.getokapibm25nodelabelpropagation.b.clone(), data.getokapibm25nodelabelpropagation.verbose.clone());
    }
    

    7 => {
        trace.push(format!("sort_by_increasing_outbound_node_degree(verbose: {:?})", &data.sortbyincreasingoutboundnodedegree.verbose));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        graph = graph.sort_by_increasing_outbound_node_degree(data.sortbyincreasingoutboundnodedegree.verbose.clone());
    }
    

    8 => {
        trace.push(format!("sort_by_decreasing_outbound_node_degree(verbose: {:?})", &data.sortbydecreasingoutboundnodedegree.verbose));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        graph = graph.sort_by_decreasing_outbound_node_degree(data.sortbydecreasingoutboundnodedegree.verbose.clone());
    }
    

    9 => {
        trace.push(format!("to_dot(use_node_names: {:?})", &data.todot.use_node_names));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.to_dot(data.todot.use_node_names.clone());
    }
    

    10 => {
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
    

    11 => {
        trace.push(format!("is_singleton_from_node_id(node_id: {:?})", &data.issingletonfromnodeid.node_id));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.is_singleton_from_node_id(data.issingletonfromnodeid.node_id.clone());
    }
    

    12 => {
        trace.push(format!("is_singleton_with_selfloops_from_node_id(node_id: {:?})", &data.issingletonwithselfloopsfromnodeid.node_id));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.is_singleton_with_selfloops_from_node_id(data.issingletonwithselfloopsfromnodeid.node_id.clone());
    }
    

    13 => {
        trace.push(format!("has_node_type_id(node_type_id: {:?})", &data.hasnodetypeid.node_type_id));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.has_node_type_id(data.hasnodetypeid.node_type_id.clone());
    }
    

    14 => {
        trace.push(format!("has_edge_type_id(edge_type_id: {:?})", &data.hasedgetypeid.edge_type_id));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.has_edge_type_id(data.hasedgetypeid.edge_type_id.clone());
    }
    

    15 => {
        trace.push(format!("has_edge_from_node_ids(src: {:?}, dst: {:?})", &data.hasedgefromnodeids.src, &data.hasedgefromnodeids.dst));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.has_edge_from_node_ids(data.hasedgefromnodeids.src.clone(), data.hasedgefromnodeids.dst.clone());
    }
    

    16 => {
        trace.push(format!("has_selfloop_from_node_id(node_id: {:?})", &data.hasselfloopfromnodeid.node_id));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.has_selfloop_from_node_id(data.hasselfloopfromnodeid.node_id.clone());
    }
    

    17 => {
        trace.push(format!("has_edge_from_node_ids_and_edge_type_id(src: {:?}, dst: {:?}, edge_type: {:?})", &data.hasedgefromnodeidsandedgetypeid.src, &data.hasedgefromnodeidsandedgetypeid.dst, &data.hasedgefromnodeidsandedgetypeid.edge_type));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.has_edge_from_node_ids_and_edge_type_id(data.hasedgefromnodeidsandedgetypeid.src.clone(), data.hasedgefromnodeidsandedgetypeid.dst.clone(), data.hasedgefromnodeidsandedgetypeid.edge_type.clone());
    }
    

    18 => {
        trace.push(format!("is_trap_node_from_node_id(node_id: {:?})", &data.istrapnodefromnodeid.node_id));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.is_trap_node_from_node_id(data.istrapnodefromnodeid.node_id.clone());
    }
    

    19 => {
        trace.push(format!("get_preferential_attachment_from_node_ids(source_node_id: {:?}, destination_node_id: {:?}, normalize: {:?})", &data.getpreferentialattachmentfromnodeids.source_node_id, &data.getpreferentialattachmentfromnodeids.destination_node_id, &data.getpreferentialattachmentfromnodeids.normalize));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_preferential_attachment_from_node_ids(data.getpreferentialattachmentfromnodeids.source_node_id.clone(), data.getpreferentialattachmentfromnodeids.destination_node_id.clone(), data.getpreferentialattachmentfromnodeids.normalize.clone());
    }
    

    20 => {
        trace.push(format!("get_weighted_preferential_attachment_from_node_ids(source_node_id: {:?}, destination_node_id: {:?}, normalize: {:?})", &data.getweightedpreferentialattachmentfromnodeids.source_node_id, &data.getweightedpreferentialattachmentfromnodeids.destination_node_id, &data.getweightedpreferentialattachmentfromnodeids.normalize));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_weighted_preferential_attachment_from_node_ids(data.getweightedpreferentialattachmentfromnodeids.source_node_id.clone(), data.getweightedpreferentialattachmentfromnodeids.destination_node_id.clone(), data.getweightedpreferentialattachmentfromnodeids.normalize.clone());
    }
    

    21 => {
        trace.push(format!("get_jaccard_coefficient_from_node_ids(source_node_id: {:?}, destination_node_id: {:?})", &data.getjaccardcoefficientfromnodeids.source_node_id, &data.getjaccardcoefficientfromnodeids.destination_node_id));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_jaccard_coefficient_from_node_ids(data.getjaccardcoefficientfromnodeids.source_node_id.clone(), data.getjaccardcoefficientfromnodeids.destination_node_id.clone());
    }
    

    22 => {
        trace.push(format!("get_adamic_adar_index_from_node_ids(source_node_id: {:?}, destination_node_id: {:?})", &data.getadamicadarindexfromnodeids.source_node_id, &data.getadamicadarindexfromnodeids.destination_node_id));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_adamic_adar_index_from_node_ids(data.getadamicadarindexfromnodeids.source_node_id.clone(), data.getadamicadarindexfromnodeids.destination_node_id.clone());
    }
    

    23 => {
        trace.push(format!("get_resource_allocation_index_from_node_ids(source_node_id: {:?}, destination_node_id: {:?})", &data.getresourceallocationindexfromnodeids.source_node_id, &data.getresourceallocationindexfromnodeids.destination_node_id));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_resource_allocation_index_from_node_ids(data.getresourceallocationindexfromnodeids.source_node_id.clone(), data.getresourceallocationindexfromnodeids.destination_node_id.clone());
    }
    

    24 => {
        trace.push(format!("get_weighted_resource_allocation_index_from_node_ids(source_node_id: {:?}, destination_node_id: {:?})", &data.getweightedresourceallocationindexfromnodeids.source_node_id, &data.getweightedresourceallocationindexfromnodeids.destination_node_id));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_weighted_resource_allocation_index_from_node_ids(data.getweightedresourceallocationindexfromnodeids.source_node_id.clone(), data.getweightedresourceallocationindexfromnodeids.destination_node_id.clone());
    }
    

    25 => {
        trace.push(format!("enable(vector_sources: {:?}, vector_destinations: {:?}, vector_cumulative_node_degrees: {:?})", &data.enable.vector_sources, &data.enable.vector_destinations, &data.enable.vector_cumulative_node_degrees));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.enable(data.enable.vector_sources.clone(), data.enable.vector_destinations.clone(), data.enable.vector_cumulative_node_degrees.clone());
    }
    

    26 => {
        trace.push(format!("disable_all()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        graph.disable_all()
    }
    

    27 => {
        trace.push(format!("get_number_of_triangles(normalize: {:?}, low_centrality: {:?}, verbose: {:?})", &data.getnumberoftriangles.normalize, &data.getnumberoftriangles.low_centrality, &data.getnumberoftriangles.verbose));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_number_of_triangles(data.getnumberoftriangles.normalize.clone(), data.getnumberoftriangles.low_centrality.clone(), data.getnumberoftriangles.verbose.clone());
    }
    

    28 => {
        trace.push(format!("get_triads_number()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_triads_number();
    }
    

    29 => {
        trace.push(format!("get_weighted_triads_number()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_weighted_triads_number();
    }
    

    30 => {
        trace.push(format!("get_transitivity(low_centrality: {:?}, verbose: {:?})", &data.gettransitivity.low_centrality, &data.gettransitivity.verbose));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_transitivity(data.gettransitivity.low_centrality.clone(), data.gettransitivity.verbose.clone());
    }
    

    31 => {
        trace.push(format!("get_number_of_triangles_per_node(normalize: {:?}, low_centrality: {:?}, verbose: {:?})", &data.getnumberoftrianglespernode.normalize, &data.getnumberoftrianglespernode.low_centrality, &data.getnumberoftrianglespernode.verbose));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_number_of_triangles_per_node(data.getnumberoftrianglespernode.normalize.clone(), data.getnumberoftrianglespernode.low_centrality.clone(), data.getnumberoftrianglespernode.verbose.clone());
    }
    

    32 => {
        trace.push(format!("iter_clustering_coefficient_per_node(low_centrality: {:?}, verbose: {:?})", &data.iterclusteringcoefficientpernode.low_centrality, &data.iterclusteringcoefficientpernode.verbose));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.iter_clustering_coefficient_per_node(data.iterclusteringcoefficientpernode.low_centrality.clone(), data.iterclusteringcoefficientpernode.verbose.clone()).collect::<Vec<_>>();
            
    }
    

    33 => {
        trace.push(format!("get_clustering_coefficient_per_node(low_centrality: {:?}, verbose: {:?})", &data.getclusteringcoefficientpernode.low_centrality, &data.getclusteringcoefficientpernode.verbose));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_clustering_coefficient_per_node(data.getclusteringcoefficientpernode.low_centrality.clone(), data.getclusteringcoefficientpernode.verbose.clone());
    }
    

    34 => {
        trace.push(format!("get_clustering_coefficient(low_centrality: {:?}, verbose: {:?})", &data.getclusteringcoefficient.low_centrality, &data.getclusteringcoefficient.verbose));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_clustering_coefficient(data.getclusteringcoefficient.low_centrality.clone(), data.getclusteringcoefficient.verbose.clone());
    }
    

    35 => {
        trace.push(format!("get_average_clustering_coefficient(low_centrality: {:?}, verbose: {:?})", &data.getaverageclusteringcoefficient.low_centrality, &data.getaverageclusteringcoefficient.verbose));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_average_clustering_coefficient(data.getaverageclusteringcoefficient.low_centrality.clone(), data.getaverageclusteringcoefficient.verbose.clone());
    }
    

    36 => {
        trace.push(format!("get_minimum_path_node_ids_from_node_ids(src_node_id: {:?}, dst_node_id: {:?}, maximal_depth: {:?})", &data.getminimumpathnodeidsfromnodeids.src_node_id, &data.getminimumpathnodeidsfromnodeids.dst_node_id, &data.getminimumpathnodeidsfromnodeids.maximal_depth));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_minimum_path_node_ids_from_node_ids(data.getminimumpathnodeidsfromnodeids.src_node_id.clone(), data.getminimumpathnodeidsfromnodeids.dst_node_id.clone(), data.getminimumpathnodeidsfromnodeids.maximal_depth.clone());
    }
    

    37 => {
        trace.push(format!("get_k_shortest_path_node_ids_from_node_ids(src_node_id: {:?}, dst_node_id: {:?}, k: {:?}, max_path_length: {:?}, verbose: {:?})", &data.getkshortestpathnodeidsfromnodeids.src_node_id, &data.getkshortestpathnodeidsfromnodeids.dst_node_id, &(data.getkshortestpathnodeidsfromnodeids.k as usize), &data.getkshortestpathnodeidsfromnodeids.max_path_length, &data.getkshortestpathnodeidsfromnodeids.verbose));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_k_shortest_path_node_ids_from_node_ids(data.getkshortestpathnodeidsfromnodeids.src_node_id.clone(), data.getkshortestpathnodeidsfromnodeids.dst_node_id.clone(), (data.getkshortestpathnodeidsfromnodeids.k as usize).clone(), data.getkshortestpathnodeidsfromnodeids.max_path_length.clone(), data.getkshortestpathnodeidsfromnodeids.verbose.clone());
    }
    

    38 => {
        trace.push(format!("get_eccentricity_from_node_id(node_id: {:?})", &data.geteccentricityfromnodeid.node_id));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_eccentricity_from_node_id(data.geteccentricityfromnodeid.node_id.clone());
    }
    

    39 => {
        trace.push(format!("get_weighted_eccentricity_from_node_id(node_id: {:?}, use_edge_weights_as_probabilities: {:?})", &data.getweightedeccentricityfromnodeid.node_id, &data.getweightedeccentricityfromnodeid.use_edge_weights_as_probabilities));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_weighted_eccentricity_from_node_id(data.getweightedeccentricityfromnodeid.node_id.clone(), data.getweightedeccentricityfromnodeid.use_edge_weights_as_probabilities.clone());
    }
    

    40 => {
        trace.push(format!("get_weighted_minimum_path_node_ids_from_node_ids(src_node_id: {:?}, dst_node_id: {:?}, use_edge_weights_as_probabilities: {:?}, maximal_depth: {:?})", &data.getweightedminimumpathnodeidsfromnodeids.src_node_id, &data.getweightedminimumpathnodeidsfromnodeids.dst_node_id, &data.getweightedminimumpathnodeidsfromnodeids.use_edge_weights_as_probabilities, &data.getweightedminimumpathnodeidsfromnodeids.maximal_depth));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_weighted_minimum_path_node_ids_from_node_ids(data.getweightedminimumpathnodeidsfromnodeids.src_node_id.clone(), data.getweightedminimumpathnodeidsfromnodeids.dst_node_id.clone(), data.getweightedminimumpathnodeidsfromnodeids.use_edge_weights_as_probabilities.clone(), data.getweightedminimumpathnodeidsfromnodeids.maximal_depth.clone());
    }
    

    41 => {
        trace.push(format!("get_breath_first_search_from_node_ids(src_node_id: {:?}, dst_node_id: {:?}, compute_predecessors: {:?}, maximal_depth: {:?})", &data.getbreathfirstsearchfromnodeids.src_node_id, &data.getbreathfirstsearchfromnodeids.dst_node_id, &data.getbreathfirstsearchfromnodeids.compute_predecessors, &data.getbreathfirstsearchfromnodeids.maximal_depth));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_breath_first_search_from_node_ids(data.getbreathfirstsearchfromnodeids.src_node_id.clone(), data.getbreathfirstsearchfromnodeids.dst_node_id.clone(), data.getbreathfirstsearchfromnodeids.compute_predecessors.clone(), data.getbreathfirstsearchfromnodeids.maximal_depth.clone());
    }
    

    42 => {
        trace.push(format!("get_dijkstra_from_node_ids(src_node_id: {:?}, maybe_dst_node_id: {:?}, maybe_dst_node_ids: {:?}, compute_predecessors: {:?}, maximal_depth: {:?}, use_edge_weights_as_probabilities: {:?})", &data.getdijkstrafromnodeids.src_node_id, &data.getdijkstrafromnodeids.maybe_dst_node_id, &data.getdijkstrafromnodeids.maybe_dst_node_ids, &data.getdijkstrafromnodeids.compute_predecessors, &data.getdijkstrafromnodeids.maximal_depth, &data.getdijkstrafromnodeids.use_edge_weights_as_probabilities));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_dijkstra_from_node_ids(data.getdijkstrafromnodeids.src_node_id.clone(), data.getdijkstrafromnodeids.maybe_dst_node_id.clone(), data.getdijkstrafromnodeids.maybe_dst_node_ids.clone(), data.getdijkstrafromnodeids.compute_predecessors.clone(), data.getdijkstrafromnodeids.maximal_depth.clone(), data.getdijkstrafromnodeids.use_edge_weights_as_probabilities.clone());
    }
    

    43 => {
        trace.push(format!("get_diameter_naive(ignore_infinity: {:?}, verbose: {:?})", &data.getdiameternaive.ignore_infinity, &data.getdiameternaive.verbose));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_diameter_naive(data.getdiameternaive.ignore_infinity.clone(), data.getdiameternaive.verbose.clone());
    }
    

    44 => {
        trace.push(format!("get_diameter(ignore_infinity: {:?}, verbose: {:?})", &data.getdiameter.ignore_infinity, &data.getdiameter.verbose));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_diameter(data.getdiameter.ignore_infinity.clone(), data.getdiameter.verbose.clone());
    }
    

    45 => {
        trace.push(format!("get_weighted_diameter_naive(ignore_infinity: {:?}, use_edge_weights_as_probabilities: {:?}, verbose: {:?})", &data.getweighteddiameternaive.ignore_infinity, &data.getweighteddiameternaive.use_edge_weights_as_probabilities, &data.getweighteddiameternaive.verbose));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_weighted_diameter_naive(data.getweighteddiameternaive.ignore_infinity.clone(), data.getweighteddiameternaive.use_edge_weights_as_probabilities.clone(), data.getweighteddiameternaive.verbose.clone());
    }
    

    46 => {
        trace.push(format!("iter_edge_ids_from_node_ids(src: {:?}, dst: {:?})", &data.iteredgeidsfromnodeids.src, &data.iteredgeidsfromnodeids.dst));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.iter_edge_ids_from_node_ids(data.iteredgeidsfromnodeids.src.clone(), data.iteredgeidsfromnodeids.dst.clone()).map(|x| x.collect::<Vec<_>>());
            
    }
    

    47 => {
        trace.push(format!("iter_edge_node_ids_and_edge_type_id_from_edge_type_id(edge_type_id: {:?}, directed: {:?})", &data.iteredgenodeidsandedgetypeidfromedgetypeid.edge_type_id, &data.iteredgenodeidsandedgetypeidfromedgetypeid.directed));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.iter_edge_node_ids_and_edge_type_id_from_edge_type_id(data.iteredgenodeidsandedgetypeidfromedgetypeid.edge_type_id.clone(), data.iteredgenodeidsandedgetypeidfromedgetypeid.directed.clone()).map(|x| x.collect::<Vec<_>>());
            
    }
    

    48 => {
        trace.push(format!("iter_node_ids_and_node_type_ids_from_node_type_id(node_type_id: {:?})", &data.iternodeidsandnodetypeidsfromnodetypeid.node_type_id));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.iter_node_ids_and_node_type_ids_from_node_type_id(data.iternodeidsandnodetypeidsfromnodetypeid.node_type_id.clone()).map(|x| x.collect::<Vec<_>>());
            
    }
    

    49 => {
        trace.push(format!("iter_node_names_and_node_type_names_from_node_type_id(node_type_id: {:?})", &data.iternodenamesandnodetypenamesfromnodetypeid.node_type_id));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.iter_node_names_and_node_type_names_from_node_type_id(data.iternodenamesandnodetypenamesfromnodetypeid.node_type_id.clone());
    }
    

    50 => {
        trace.push(format!("iter_edge_node_names_and_edge_type_name_from_edge_type_id(edge_type_id: {:?}, directed: {:?})", &data.iteredgenodenamesandedgetypenamefromedgetypeid.edge_type_id, &data.iteredgenodenamesandedgetypenamefromedgetypeid.directed));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.iter_edge_node_names_and_edge_type_name_from_edge_type_id(data.iteredgenodenamesandedgetypenamefromedgetypeid.edge_type_id.clone(), data.iteredgenodenamesandedgetypenamefromedgetypeid.directed.clone());
    }
    

    51 => {
        trace.push(format!("get_transitive_closure(iterations: {:?}, verbose: {:?})", &data.gettransitiveclosure.iterations.map(|x| x as NodeT), &data.gettransitiveclosure.verbose));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        graph = graph.get_transitive_closure(data.gettransitiveclosure.iterations.map(|x| x as NodeT).clone(), data.gettransitiveclosure.verbose.clone());
    }
    

    52 => {
        trace.push(format!("get_all_shortest_paths(iterations: {:?}, verbose: {:?})", &data.getallshortestpaths.iterations.map(|x| x as NodeT), &data.getallshortestpaths.verbose));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        graph = graph.get_all_shortest_paths(data.getallshortestpaths.iterations.map(|x| x as NodeT).clone(), data.getallshortestpaths.verbose.clone());
    }
    

    53 => {
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
    

    54 => {
        trace.push(format!("strongly_connected_components()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.strongly_connected_components();
    }
    

    55 => {
        trace.push(format!("get_source_node_id_from_edge_id(edge_id: {:?})", &data.getsourcenodeidfromedgeid.edge_id));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_source_node_id_from_edge_id(data.getsourcenodeidfromedgeid.edge_id.clone());
    }
    

    56 => {
        trace.push(format!("get_destination_node_id_from_edge_id(edge_id: {:?})", &data.getdestinationnodeidfromedgeid.edge_id));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_destination_node_id_from_edge_id(data.getdestinationnodeidfromedgeid.edge_id.clone());
    }
    

    57 => {
        trace.push(format!("get_source_node_name_from_edge_id(edge_id: {:?})", &data.getsourcenodenamefromedgeid.edge_id));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_source_node_name_from_edge_id(data.getsourcenodenamefromedgeid.edge_id.clone());
    }
    

    58 => {
        trace.push(format!("get_destination_node_name_from_edge_id(edge_id: {:?})", &data.getdestinationnodenamefromedgeid.edge_id));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_destination_node_name_from_edge_id(data.getdestinationnodenamefromedgeid.edge_id.clone());
    }
    

    59 => {
        trace.push(format!("get_node_names_from_edge_id(edge_id: {:?})", &data.getnodenamesfromedgeid.edge_id));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_node_names_from_edge_id(data.getnodenamesfromedgeid.edge_id.clone());
    }
    

    60 => {
        trace.push(format!("get_node_ids_from_edge_id(edge_id: {:?})", &data.getnodeidsfromedgeid.edge_id));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_node_ids_from_edge_id(data.getnodeidsfromedgeid.edge_id.clone());
    }
    

    61 => {
        trace.push(format!("get_edge_id_from_node_ids(src: {:?}, dst: {:?})", &data.getedgeidfromnodeids.src, &data.getedgeidfromnodeids.dst));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_edge_id_from_node_ids(data.getedgeidfromnodeids.src.clone(), data.getedgeidfromnodeids.dst.clone());
    }
    

    62 => {
        trace.push(format!("get_node_ids_and_edge_type_id_from_edge_id(edge_id: {:?})", &data.getnodeidsandedgetypeidfromedgeid.edge_id));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_node_ids_and_edge_type_id_from_edge_id(data.getnodeidsandedgetypeidfromedgeid.edge_id.clone());
    }
    

    63 => {
        trace.push(format!("get_node_ids_and_edge_type_id_and_edge_weight_from_edge_id(edge_id: {:?})", &data.getnodeidsandedgetypeidandedgeweightfromedgeid.edge_id));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_node_ids_and_edge_type_id_and_edge_weight_from_edge_id(data.getnodeidsandedgetypeidandedgeweightfromedgeid.edge_id.clone());
    }
    

    64 => {
        trace.push(format!("get_top_k_central_node_ids(k: {:?})", &data.gettopkcentralnodeids.k));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_top_k_central_node_ids(data.gettopkcentralnodeids.k.clone());
    }
    

    65 => {
        trace.push(format!("get_weighted_top_k_central_node_ids(k: {:?})", &data.getweightedtopkcentralnodeids.k));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_weighted_top_k_central_node_ids(data.getweightedtopkcentralnodeids.k.clone());
    }
    

    66 => {
        trace.push(format!("get_node_degree_from_node_id(node_id: {:?})", &data.getnodedegreefromnodeid.node_id));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_node_degree_from_node_id(data.getnodedegreefromnodeid.node_id.clone());
    }
    

    67 => {
        trace.push(format!("get_weighted_node_degree_from_node_id(node_id: {:?})", &data.getweightednodedegreefromnodeid.node_id));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_weighted_node_degree_from_node_id(data.getweightednodedegreefromnodeid.node_id.clone());
    }
    

    68 => {
        trace.push(format!("get_top_k_central_node_names(k: {:?})", &data.gettopkcentralnodenames.k));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_top_k_central_node_names(data.gettopkcentralnodenames.k.clone());
    }
    

    69 => {
        trace.push(format!("get_node_type_id_from_node_id(node_id: {:?})", &data.getnodetypeidfromnodeid.node_id));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_node_type_id_from_node_id(data.getnodetypeidfromnodeid.node_id.clone());
    }
    

    70 => {
        trace.push(format!("get_edge_type_id_from_edge_id(edge_id: {:?})", &data.getedgetypeidfromedgeid.edge_id));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_edge_type_id_from_edge_id(data.getedgetypeidfromedgeid.edge_id.clone());
    }
    

    71 => {
        trace.push(format!("get_node_type_names_from_node_id(node_id: {:?})", &data.getnodetypenamesfromnodeid.node_id));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_node_type_names_from_node_id(data.getnodetypenamesfromnodeid.node_id.clone());
    }
    

    72 => {
        trace.push(format!("get_edge_type_name_from_edge_id(edge_id: {:?})", &data.getedgetypenamefromedgeid.edge_id));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_edge_type_name_from_edge_id(data.getedgetypenamefromedgeid.edge_id.clone());
    }
    

    73 => {
        trace.push(format!("get_edge_type_name_from_edge_type_id(edge_type_id: {:?})", &data.getedgetypenamefromedgetypeid.edge_type_id));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_edge_type_name_from_edge_type_id(data.getedgetypenamefromedgetypeid.edge_type_id.clone());
    }
    

    74 => {
        trace.push(format!("get_edge_weight_from_edge_id(edge_id: {:?})", &data.getedgeweightfromedgeid.edge_id));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_edge_weight_from_edge_id(data.getedgeweightfromedgeid.edge_id.clone());
    }
    

    75 => {
        trace.push(format!("get_edge_weight_from_node_ids(src: {:?}, dst: {:?})", &data.getedgeweightfromnodeids.src, &data.getedgeweightfromnodeids.dst));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_edge_weight_from_node_ids(data.getedgeweightfromnodeids.src.clone(), data.getedgeweightfromnodeids.dst.clone());
    }
    

    76 => {
        trace.push(format!("get_edge_weight_from_node_ids_and_edge_type_id(src: {:?}, dst: {:?}, edge_type: {:?})", &data.getedgeweightfromnodeidsandedgetypeid.src, &data.getedgeweightfromnodeidsandedgetypeid.dst, &data.getedgeweightfromnodeidsandedgetypeid.edge_type));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_edge_weight_from_node_ids_and_edge_type_id(data.getedgeweightfromnodeidsandedgetypeid.src.clone(), data.getedgeweightfromnodeidsandedgetypeid.dst.clone(), data.getedgeweightfromnodeidsandedgetypeid.edge_type.clone());
    }
    

    77 => {
        trace.push(format!("get_node_name_from_node_id(node_id: {:?})", &data.getnodenamefromnodeid.node_id));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_node_name_from_node_id(data.getnodenamefromnodeid.node_id.clone());
    }
    

    78 => {
        trace.push(format!("get_edge_node_names_from_edge_node_ids(edge_node_ids: {:?})", &data.getedgenodenamesfromedgenodeids.edge_node_ids));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_edge_node_names_from_edge_node_ids(data.getedgenodenamesfromedgenodeids.edge_node_ids.clone());
    }
    

    79 => {
        trace.push(format!("get_edge_count_from_edge_type_id(edge_type_id: {:?})", &data.getedgecountfromedgetypeid.edge_type_id));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_edge_count_from_edge_type_id(data.getedgecountfromedgetypeid.edge_type_id.clone());
    }
    

    80 => {
        trace.push(format!("get_node_count_from_node_type_id(node_type_id: {:?})", &data.getnodecountfromnodetypeid.node_type_id));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_node_count_from_node_type_id(data.getnodecountfromnodetypeid.node_type_id.clone());
    }
    

    81 => {
        trace.push(format!("get_neighbour_node_ids_from_node_id(node_id: {:?})", &data.getneighbournodeidsfromnodeid.node_id));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_neighbour_node_ids_from_node_id(data.getneighbournodeidsfromnodeid.node_id.clone());
    }
    

    82 => {
        trace.push(format!("get_minmax_edge_ids_from_node_ids(src: {:?}, dst: {:?})", &data.getminmaxedgeidsfromnodeids.src, &data.getminmaxedgeidsfromnodeids.dst));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_minmax_edge_ids_from_node_ids(data.getminmaxedgeidsfromnodeids.src.clone(), data.getminmaxedgeidsfromnodeids.dst.clone());
    }
    

    83 => {
        trace.push(format!("get_edge_id_from_node_ids_and_edge_type_id(src: {:?}, dst: {:?}, edge_type: {:?})", &data.getedgeidfromnodeidsandedgetypeid.src, &data.getedgeidfromnodeidsandedgetypeid.dst, &data.getedgeidfromnodeidsandedgetypeid.edge_type));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_edge_id_from_node_ids_and_edge_type_id(data.getedgeidfromnodeidsandedgetypeid.src.clone(), data.getedgeidfromnodeidsandedgetypeid.dst.clone(), data.getedgeidfromnodeidsandedgetypeid.edge_type.clone());
    }
    

    84 => {
        trace.push(format!("get_minmax_edge_ids_from_source_node_id(src: {:?})", &data.getminmaxedgeidsfromsourcenodeid.src));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_minmax_edge_ids_from_source_node_id(data.getminmaxedgeidsfromsourcenodeid.src.clone());
    }
    

    85 => {
        trace.push(format!("get_node_type_name_from_node_type_id(node_type_id: {:?})", &data.getnodetypenamefromnodetypeid.node_type_id));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_node_type_name_from_node_type_id(data.getnodetypenamefromnodetypeid.node_type_id.clone());
    }
    

    86 => {
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
    

    87 => {
        trace.push(format!("drop_unknown_node_types(verbose: {:?})", &data.dropunknownnodetypes.verbose));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        graph = graph.drop_unknown_node_types(data.dropunknownnodetypes.verbose.clone());
    }
    

    88 => {
        trace.push(format!("drop_unknown_edge_types(verbose: {:?})", &data.dropunknownedgetypes.verbose));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        graph = graph.drop_unknown_edge_types(data.dropunknownedgetypes.verbose.clone());
    }
    

    89 => {
        trace.push(format!("drop_singleton_nodes(verbose: {:?})", &data.dropsingletonnodes.verbose));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        graph = graph.drop_singleton_nodes(data.dropsingletonnodes.verbose.clone());
    }
    

    90 => {
        trace.push(format!("drop_singleton_nodes_with_selfloops(verbose: {:?})", &data.dropsingletonnodeswithselfloops.verbose));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        graph = graph.drop_singleton_nodes_with_selfloops(data.dropsingletonnodeswithselfloops.verbose.clone());
    }
    

    91 => {
        trace.push(format!("drop_disconnected_nodes(verbose: {:?})", &data.dropdisconnectednodes.verbose));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        graph = graph.drop_disconnected_nodes(data.dropdisconnectednodes.verbose.clone());
    }
    

    92 => {
        trace.push(format!("drop_selfloops(verbose: {:?})", &data.dropselfloops.verbose));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        graph = graph.drop_selfloops(data.dropselfloops.verbose.clone());
    }
    

    93 => {
        trace.push(format!("drop_parallel_edges(verbose: {:?})", &data.dropparalleledges.verbose));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        graph = graph.drop_parallel_edges(data.dropparalleledges.verbose.clone());
    }
    

    94 => {
        trace.push(format!("validate_node_id(node_id: {:?})", &data.validatenodeid.node_id));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.validate_node_id(data.validatenodeid.node_id.clone());
    }
    

    95 => {
        trace.push(format!("validate_node_ids(node_ids: {:?})", &data.validatenodeids.node_ids));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.validate_node_ids(data.validatenodeids.node_ids.clone());
    }
    

    96 => {
        trace.push(format!("validate_edge_id(edge_id: {:?})", &data.validateedgeid.edge_id));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.validate_edge_id(data.validateedgeid.edge_id.clone());
    }
    

    97 => {
        trace.push(format!("validate_edge_ids(edge_ids: {:?})", &data.validateedgeids.edge_ids));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.validate_edge_ids(data.validateedgeids.edge_ids.clone());
    }
    

    98 => {
        trace.push(format!("must_not_contain_unknown_node_types()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.must_not_contain_unknown_node_types();
    }
    

    99 => {
        trace.push(format!("must_not_contain_unknown_edge_types()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.must_not_contain_unknown_edge_types();
    }
    

    100 => {
        trace.push(format!("validate_node_type_id(node_type_id: {:?})", &data.validatenodetypeid.node_type_id));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.validate_node_type_id(data.validatenodetypeid.node_type_id.clone());
    }
    

    101 => {
        trace.push(format!("validate_node_type_ids(node_type_ids: {:?})", &data.validatenodetypeids.node_type_ids));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.validate_node_type_ids(data.validatenodetypeids.node_type_ids.clone());
    }
    

    102 => {
        trace.push(format!("validate_edge_type_id(edge_type_id: {:?})", &data.validateedgetypeid.edge_type_id));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.validate_edge_type_id(data.validateedgetypeid.edge_type_id.clone());
    }
    

    103 => {
        trace.push(format!("validate_edge_type_ids(edge_type_ids: {:?})", &data.validateedgetypeids.edge_type_ids));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.validate_edge_type_ids(data.validateedgetypeids.edge_type_ids.clone());
    }
    

    104 => {
        trace.push(format!("must_have_node_types()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.must_have_node_types();
    }
    

    105 => {
        trace.push(format!("must_have_edge_types()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.must_have_edge_types();
    }
    

    106 => {
        trace.push(format!("must_be_undirected()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.must_be_undirected();
    }
    

    107 => {
        trace.push(format!("must_be_multigraph()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.must_be_multigraph();
    }
    

    108 => {
        trace.push(format!("must_not_be_multigraph()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.must_not_be_multigraph();
    }
    

    109 => {
        trace.push(format!("must_have_edge_weights()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.must_have_edge_weights();
    }
    

    110 => {
        trace.push(format!("must_have_known_node_types()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.must_have_known_node_types();
    }
    

    111 => {
        trace.push(format!("must_have_unknown_node_types()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.must_have_unknown_node_types();
    }
    

    112 => {
        trace.push(format!("must_have_known_edge_types()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.must_have_known_edge_types();
    }
    

    113 => {
        trace.push(format!("must_have_unknown_edge_types()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.must_have_unknown_edge_types();
    }
    

    114 => {
        trace.push(format!("must_have_edge_weights_representing_probabilities()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.must_have_edge_weights_representing_probabilities();
    }
    

    115 => {
        trace.push(format!("must_have_positive_edge_weights()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.must_have_positive_edge_weights();
    }
    

    116 => {
        trace.push(format!("must_not_contain_weighted_singleton_nodes()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.must_not_contain_weighted_singleton_nodes();
    }
    

    117 => {
        trace.push(format!("must_have_edges()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.must_have_edges();
    }
    

    118 => {
        trace.push(format!("must_have_nodes()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.must_have_nodes();
    }
    

    119 => {
        trace.push(format!("iter_degree_centrality()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.iter_degree_centrality().map(|x| x.collect::<Vec<_>>());
            
    }
    

    120 => {
        trace.push(format!("par_iter_weighted_degree_centrality()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.par_iter_weighted_degree_centrality().map(|x| x.collect::<Vec<_>>());
            
    }
    

    121 => {
        trace.push(format!("get_degree_centrality()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_degree_centrality();
    }
    

    122 => {
        trace.push(format!("get_weighted_degree_centrality()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_weighted_degree_centrality();
    }
    

    123 => {
        trace.push(format!("par_iter_closeness_centrality(verbose: {:?})", &data.pariterclosenesscentrality.verbose));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.par_iter_closeness_centrality(data.pariterclosenesscentrality.verbose.clone()).collect::<Vec<_>>();
            
    }
    

    124 => {
        trace.push(format!("par_iter_weighted_closeness_centrality(use_edge_weights_as_probabilities: {:?}, verbose: {:?})", &data.pariterweightedclosenesscentrality.use_edge_weights_as_probabilities, &data.pariterweightedclosenesscentrality.verbose));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.par_iter_weighted_closeness_centrality(data.pariterweightedclosenesscentrality.use_edge_weights_as_probabilities.clone(), data.pariterweightedclosenesscentrality.verbose.clone()).map(|x| x.collect::<Vec<_>>());
            
    }
    

    125 => {
        trace.push(format!("get_closeness_centrality(verbose: {:?})", &data.getclosenesscentrality.verbose));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_closeness_centrality(data.getclosenesscentrality.verbose.clone());
    }
    

    126 => {
        trace.push(format!("get_weighted_closeness_centrality(use_edge_weights_as_probabilities: {:?}, verbose: {:?})", &data.getweightedclosenesscentrality.use_edge_weights_as_probabilities, &data.getweightedclosenesscentrality.verbose));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_weighted_closeness_centrality(data.getweightedclosenesscentrality.use_edge_weights_as_probabilities.clone(), data.getweightedclosenesscentrality.verbose.clone());
    }
    

    127 => {
        trace.push(format!("par_iter_harmonic_centrality(verbose: {:?})", &data.pariterharmoniccentrality.verbose));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.par_iter_harmonic_centrality(data.pariterharmoniccentrality.verbose.clone()).collect::<Vec<_>>();
            
    }
    

    128 => {
        trace.push(format!("par_iter_weighted_harmonic_centrality(use_edge_weights_as_probabilities: {:?}, verbose: {:?})", &data.pariterweightedharmoniccentrality.use_edge_weights_as_probabilities, &data.pariterweightedharmoniccentrality.verbose));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.par_iter_weighted_harmonic_centrality(data.pariterweightedharmoniccentrality.use_edge_weights_as_probabilities.clone(), data.pariterweightedharmoniccentrality.verbose.clone()).map(|x| x.collect::<Vec<_>>());
            
    }
    

    129 => {
        trace.push(format!("get_harmonic_centrality(verbose: {:?})", &data.getharmoniccentrality.verbose));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_harmonic_centrality(data.getharmoniccentrality.verbose.clone());
    }
    

    130 => {
        trace.push(format!("get_weighted_harmonic_centrality(use_edge_weights_as_probabilities: {:?}, verbose: {:?})", &data.getweightedharmoniccentrality.use_edge_weights_as_probabilities, &data.getweightedharmoniccentrality.verbose));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_weighted_harmonic_centrality(data.getweightedharmoniccentrality.use_edge_weights_as_probabilities.clone(), data.getweightedharmoniccentrality.verbose.clone());
    }
    

    131 => {
        trace.push(format!("get_stress_centrality(normalize: {:?}, verbose: {:?})", &data.getstresscentrality.normalize, &data.getstresscentrality.verbose));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_stress_centrality(data.getstresscentrality.normalize.clone(), data.getstresscentrality.verbose.clone());
    }
    

    132 => {
        trace.push(format!("get_betweenness_centrality(normalize: {:?}, verbose: {:?})", &data.getbetweennesscentrality.normalize, &data.getbetweennesscentrality.verbose));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_betweenness_centrality(data.getbetweennesscentrality.normalize.clone(), data.getbetweennesscentrality.verbose.clone());
    }
    

    133 => {
        trace.push(format!("get_eigenvector_centrality(maximum_iterations_number: {:?}, tollerance: {:?})", &data.geteigenvectorcentrality.maximum_iterations_number.map(|x| x as usize), &data.geteigenvectorcentrality.tollerance));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_eigenvector_centrality(data.geteigenvectorcentrality.maximum_iterations_number.map(|x| x as usize).clone(), data.geteigenvectorcentrality.tollerance.clone());
    }
    

    134 => {
        trace.push(format!("get_weighted_eigenvector_centrality(maximum_iterations_number: {:?}, tollerance: {:?})", &data.getweightedeigenvectorcentrality.maximum_iterations_number.map(|x| x as usize), &data.getweightedeigenvectorcentrality.tollerance));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_weighted_eigenvector_centrality(data.getweightedeigenvectorcentrality.maximum_iterations_number.map(|x| x as usize).clone(), data.getweightedeigenvectorcentrality.tollerance.clone());
    }
    

    135 => {
        trace.push(format!("remove_inplace_node_type_ids(node_type_ids_to_remove: {:?})", &data.removeinplacenodetypeids.node_type_ids_to_remove));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.remove_inplace_node_type_ids(data.removeinplacenodetypeids.node_type_ids_to_remove.clone());
    }
    

    136 => {
        trace.push(format!("remove_inplace_singleton_node_types()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.remove_inplace_singleton_node_types();
    }
    

    137 => {
        trace.push(format!("remove_inplace_edge_type_ids(edge_type_ids_to_remove: {:?})", &data.removeinplaceedgetypeids.edge_type_ids_to_remove));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.remove_inplace_edge_type_ids(data.removeinplaceedgetypeids.edge_type_ids_to_remove.clone());
    }
    

    138 => {
        trace.push(format!("remove_inplace_singleton_edge_types()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.remove_inplace_singleton_edge_types();
    }
    

    139 => {
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
    

    140 => {
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
    

    141 => {
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
    

    142 => {
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
    

    143 => {
        trace.push(format!("remove_inplace_node_types()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.remove_inplace_node_types();
    }
    

    144 => {
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
    

    145 => {
        trace.push(format!("remove_inplace_edge_types()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.remove_inplace_edge_types();
    }
    

    146 => {
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
    

    147 => {
        trace.push(format!("remove_inplace_edge_weights()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.remove_inplace_edge_weights();
    }
    

    148 => {
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
    

    149 => {
        trace.push(format!("encode_edge(src: {:?}, dst: {:?})", &data.encodeedge.src, &data.encodeedge.dst));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.encode_edge(data.encodeedge.src.clone(), data.encodeedge.dst.clone());
    }
    

    150 => {
        trace.push(format!("decode_edge(edge: {:?})", &data.decodeedge.edge));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.decode_edge(data.decodeedge.edge.clone());
    }
    

    151 => {
        trace.push(format!("get_max_encodable_edge_number()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_max_encodable_edge_number();
    }
    

    152 => {
        trace.push(format!("par_iter_approximated_vertex_cover()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.par_iter_approximated_vertex_cover().collect::<Vec<_>>();
            
    }
    

    153 => {
        trace.push(format!("approximated_vertex_cover_set()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.approximated_vertex_cover_set();
    }
    

    154 => {
        trace.push(format!("report()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.report();
    }
    

    155 => {
        trace.push(format!("get_node_report_from_node_id(node_id: {:?})", &data.getnodereportfromnodeid.node_id));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_node_report_from_node_id(data.getnodereportfromnodeid.node_id.clone());
    }
    

    156 => {
        trace.push(format!("textual_report()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.textual_report();
    }
    

    157 => {
        trace.push(format!("get_connected_components_number(verbose: {:?})", &data.getconnectedcomponentsnumber.verbose));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_connected_components_number(data.getconnectedcomponentsnumber.verbose.clone());
    }
    

    158 => {
        trace.push(format!("get_singleton_nodes_number()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_singleton_nodes_number();
    }
    

    159 => {
        trace.push(format!("get_weighted_singleton_nodes_number()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_weighted_singleton_nodes_number();
    }
    

    160 => {
        trace.push(format!("get_disconnected_nodes_number()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_disconnected_nodes_number();
    }
    

    161 => {
        trace.push(format!("get_singleton_node_ids()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_singleton_node_ids();
    }
    

    162 => {
        trace.push(format!("get_singleton_node_names()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_singleton_node_names();
    }
    

    163 => {
        trace.push(format!("get_singleton_nodes_with_selfloops_number()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_singleton_nodes_with_selfloops_number();
    }
    

    164 => {
        trace.push(format!("get_singleton_with_selfloops_node_ids()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_singleton_with_selfloops_node_ids();
    }
    

    165 => {
        trace.push(format!("get_singleton_with_selfloops_node_names()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_singleton_with_selfloops_node_names();
    }
    

    166 => {
        trace.push(format!("get_connected_nodes_number()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_connected_nodes_number();
    }
    

    167 => {
        trace.push(format!("get_density()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_density();
    }
    

    168 => {
        trace.push(format!("get_trap_nodes_rate()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_trap_nodes_rate();
    }
    

    169 => {
        trace.push(format!("get_node_degrees_mean()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_node_degrees_mean();
    }
    

    170 => {
        trace.push(format!("get_weighted_node_degrees_mean()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_weighted_node_degrees_mean();
    }
    

    171 => {
        trace.push(format!("get_undirected_edges_number()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_undirected_edges_number();
    }
    

    172 => {
        trace.push(format!("get_unique_undirected_edges_number()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_unique_undirected_edges_number();
    }
    

    173 => {
        trace.push(format!("get_edges_number()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_edges_number();
    }
    

    174 => {
        trace.push(format!("get_unique_edges_number()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_unique_edges_number();
    }
    

    175 => {
        trace.push(format!("get_node_degrees_median()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_node_degrees_median();
    }
    

    176 => {
        trace.push(format!("get_weighted_node_degrees_median()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_weighted_node_degrees_median();
    }
    

    177 => {
        trace.push(format!("get_weighted_maximum_node_degree()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_weighted_maximum_node_degree();
    }
    

    178 => {
        trace.push(format!("get_weighted_minimum_node_degree()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_weighted_minimum_node_degree();
    }
    

    179 => {
        trace.push(format!("get_maximum_node_degree()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_maximum_node_degree();
    }
    

    180 => {
        trace.push(format!("get_most_central_node_id()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_most_central_node_id();
    }
    

    181 => {
        trace.push(format!("get_weighted_mininum_node_degree()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_weighted_mininum_node_degree();
    }
    

    182 => {
        trace.push(format!("get_minimum_node_degree()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_minimum_node_degree();
    }
    

    183 => {
        trace.push(format!("get_node_degrees_mode()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_node_degrees_mode();
    }
    

    184 => {
        trace.push(format!("get_selfloop_number()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_selfloop_number();
    }
    

    185 => {
        trace.push(format!("get_unique_selfloop_number()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_unique_selfloop_number();
    }
    

    186 => {
        trace.push(format!("get_selfloop_nodes_rate()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_selfloop_nodes_rate();
    }
    

    187 => {
        trace.push(format!("get_name()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_name();
    }
    

    188 => {
        trace.push(format!("get_trap_nodes_number()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_trap_nodes_number();
    }
    

    189 => {
        trace.push(format!("get_source_node_ids(directed: {:?})", &data.getsourcenodeids.directed));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_source_node_ids(data.getsourcenodeids.directed.clone());
    }
    

    190 => {
        trace.push(format!("get_source_names(directed: {:?})", &data.getsourcenames.directed));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_source_names(data.getsourcenames.directed.clone());
    }
    

    191 => {
        trace.push(format!("get_destination_node_ids(directed: {:?})", &data.getdestinationnodeids.directed));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_destination_node_ids(data.getdestinationnodeids.directed.clone());
    }
    

    192 => {
        trace.push(format!("get_destination_names(directed: {:?})", &data.getdestinationnames.directed));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_destination_names(data.getdestinationnames.directed.clone());
    }
    

    193 => {
        trace.push(format!("get_node_names()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_node_names();
    }
    

    194 => {
        trace.push(format!("get_node_ids()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_node_ids();
    }
    

    195 => {
        trace.push(format!("get_edge_type_ids()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_edge_type_ids();
    }
    

    196 => {
        trace.push(format!("get_unique_edge_type_ids()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_unique_edge_type_ids();
    }
    

    197 => {
        trace.push(format!("get_edge_type_names()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_edge_type_names();
    }
    

    198 => {
        trace.push(format!("get_unique_edge_type_names()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_unique_edge_type_names();
    }
    

    199 => {
        trace.push(format!("get_edge_weights()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_edge_weights();
    }
    

    200 => {
        trace.push(format!("get_total_edge_weights()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_total_edge_weights();
    }
    

    201 => {
        trace.push(format!("get_mininum_edge_weight()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_mininum_edge_weight();
    }
    

    202 => {
        trace.push(format!("get_maximum_edge_weight()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_maximum_edge_weight();
    }
    

    203 => {
        trace.push(format!("get_node_type_ids()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_node_type_ids();
    }
    

    204 => {
        trace.push(format!("get_one_hot_encoded_node_types()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_one_hot_encoded_node_types();
    }
    

    205 => {
        trace.push(format!("get_known_node_types_mask()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_known_node_types_mask();
    }
    

    206 => {
        trace.push(format!("get_unknown_node_types_mask()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_unknown_node_types_mask();
    }
    

    207 => {
        trace.push(format!("get_one_hot_encoded_known_node_types()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_one_hot_encoded_known_node_types();
    }
    

    208 => {
        trace.push(format!("get_one_hot_encoded_edge_types()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_one_hot_encoded_edge_types();
    }
    

    209 => {
        trace.push(format!("get_one_hot_encoded_known_edge_types()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_one_hot_encoded_known_edge_types();
    }
    

    210 => {
        trace.push(format!("get_node_type_names()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_node_type_names();
    }
    

    211 => {
        trace.push(format!("get_unique_node_type_ids()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_unique_node_type_ids();
    }
    

    212 => {
        trace.push(format!("get_unique_node_type_names()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_unique_node_type_names();
    }
    

    213 => {
        trace.push(format!("get_unique_directed_edges_number()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_unique_directed_edges_number();
    }
    

    214 => {
        trace.push(format!("get_nodes_mapping()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_nodes_mapping();
    }
    

    215 => {
        trace.push(format!("get_edge_node_ids(directed: {:?})", &data.getedgenodeids.directed));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_edge_node_ids(data.getedgenodeids.directed.clone());
    }
    

    216 => {
        trace.push(format!("get_edge_node_names(directed: {:?})", &data.getedgenodenames.directed));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_edge_node_names(data.getedgenodenames.directed.clone());
    }
    

    217 => {
        trace.push(format!("get_unknown_node_types_number()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_unknown_node_types_number();
    }
    

    218 => {
        trace.push(format!("get_known_node_types_number()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_known_node_types_number();
    }
    

    219 => {
        trace.push(format!("get_unknown_node_types_rate()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_unknown_node_types_rate();
    }
    

    220 => {
        trace.push(format!("get_known_node_types_rate()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_known_node_types_rate();
    }
    

    221 => {
        trace.push(format!("get_minimum_node_types_number()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_minimum_node_types_number();
    }
    

    222 => {
        trace.push(format!("get_maximum_node_types_number()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_maximum_node_types_number();
    }
    

    223 => {
        trace.push(format!("get_maximum_multilabel_count()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_maximum_multilabel_count();
    }
    

    224 => {
        trace.push(format!("get_singleton_node_types_number()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_singleton_node_types_number();
    }
    

    225 => {
        trace.push(format!("get_singleton_node_type_ids()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_singleton_node_type_ids();
    }
    

    226 => {
        trace.push(format!("get_singleton_node_type_names()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_singleton_node_type_names();
    }
    

    227 => {
        trace.push(format!("get_unknown_edge_types_number()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_unknown_edge_types_number();
    }
    

    228 => {
        trace.push(format!("get_edge_ids_with_unknown_edge_types()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_edge_ids_with_unknown_edge_types();
    }
    

    229 => {
        trace.push(format!("get_edge_ids_with_known_edge_types()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_edge_ids_with_known_edge_types();
    }
    

    230 => {
        trace.push(format!("get_edge_node_ids_with_unknown_edge_types(directed: {:?})", &data.getedgenodeidswithunknownedgetypes.directed));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_edge_node_ids_with_unknown_edge_types(data.getedgenodeidswithunknownedgetypes.directed.clone());
    }
    

    231 => {
        trace.push(format!("get_edge_node_ids_with_known_edge_types(directed: {:?})", &data.getedgenodeidswithknownedgetypes.directed));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_edge_node_ids_with_known_edge_types(data.getedgenodeidswithknownedgetypes.directed.clone());
    }
    

    232 => {
        trace.push(format!("get_edge_node_names_with_unknown_edge_types(directed: {:?})", &data.getedgenodenameswithunknownedgetypes.directed));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_edge_node_names_with_unknown_edge_types(data.getedgenodenameswithunknownedgetypes.directed.clone());
    }
    

    233 => {
        trace.push(format!("get_edge_node_names_with_known_edge_types(directed: {:?})", &data.getedgenodenameswithknownedgetypes.directed));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_edge_node_names_with_known_edge_types(data.getedgenodenameswithknownedgetypes.directed.clone());
    }
    

    234 => {
        trace.push(format!("get_edge_ids_with_unknown_edge_types_mask()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_edge_ids_with_unknown_edge_types_mask();
    }
    

    235 => {
        trace.push(format!("get_edge_ids_with_known_edge_types_mask()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_edge_ids_with_known_edge_types_mask();
    }
    

    236 => {
        trace.push(format!("get_node_ids_with_unknown_node_types()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_node_ids_with_unknown_node_types();
    }
    

    237 => {
        trace.push(format!("get_node_ids_with_known_node_types()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_node_ids_with_known_node_types();
    }
    

    238 => {
        trace.push(format!("get_node_names_with_unknown_node_types()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_node_names_with_unknown_node_types();
    }
    

    239 => {
        trace.push(format!("get_node_names_with_known_node_types()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_node_names_with_known_node_types();
    }
    

    240 => {
        trace.push(format!("get_node_ids_with_unknown_node_types_mask()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_node_ids_with_unknown_node_types_mask();
    }
    

    241 => {
        trace.push(format!("get_node_ids_with_known_node_types_mask()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_node_ids_with_known_node_types_mask();
    }
    

    242 => {
        trace.push(format!("get_known_edge_types_number()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_known_edge_types_number();
    }
    

    243 => {
        trace.push(format!("get_unknown_edge_types_rate()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_unknown_edge_types_rate();
    }
    

    244 => {
        trace.push(format!("get_known_edge_types_rate()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_known_edge_types_rate();
    }
    

    245 => {
        trace.push(format!("get_minimum_edge_types_number()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_minimum_edge_types_number();
    }
    

    246 => {
        trace.push(format!("get_singleton_edge_types_number()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_singleton_edge_types_number();
    }
    

    247 => {
        trace.push(format!("get_singleton_edge_type_ids()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_singleton_edge_type_ids();
    }
    

    248 => {
        trace.push(format!("get_singleton_edge_type_names()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_singleton_edge_type_names();
    }
    

    249 => {
        trace.push(format!("get_nodes_number()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_nodes_number();
    }
    

    250 => {
        trace.push(format!("get_node_connected_component_ids(verbose: {:?})", &data.getnodeconnectedcomponentids.verbose));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_node_connected_component_ids(data.getnodeconnectedcomponentids.verbose.clone());
    }
    

    251 => {
        trace.push(format!("get_directed_edges_number()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_directed_edges_number();
    }
    

    252 => {
        trace.push(format!("get_edge_types_number()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_edge_types_number();
    }
    

    253 => {
        trace.push(format!("get_node_types_number()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_node_types_number();
    }
    

    254 => {
        trace.push(format!("get_node_degrees()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_node_degrees();
    }
    

    255 => {
        trace.push(format!("get_weighted_node_degrees()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_weighted_node_degrees();
    }
    

    256 => {
        trace.push(format!("get_not_singletons_node_ids()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_not_singletons_node_ids();
    }
    

    257 => {
        trace.push(format!("get_dense_nodes_mapping()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_dense_nodes_mapping();
    }
    

    258 => {
        trace.push(format!("get_parallel_edges_number()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_parallel_edges_number();
    }
    

    259 => {
        trace.push(format!("get_cumulative_node_degrees()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_cumulative_node_degrees();
    }
    

    260 => {
        trace.push(format!("get_unique_source_nodes_number()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_unique_source_nodes_number();
    }
    

    261 => {
        trace.push(format!("get_edge_type_id_counts_hashmap()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_edge_type_id_counts_hashmap();
    }
    

    262 => {
        trace.push(format!("get_edge_type_names_counts_hashmap()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_edge_type_names_counts_hashmap();
    }
    

    263 => {
        trace.push(format!("get_node_type_id_counts_hashmap()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_node_type_id_counts_hashmap();
    }
    

    264 => {
        trace.push(format!("get_node_type_names_counts_hashmap()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_node_type_names_counts_hashmap();
    }
    

    265 => {
        trace.push(format!("get_dense_binary_adjacency_matrix()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_dense_binary_adjacency_matrix();
    }
    

    266 => {
        trace.push(format!("get_dense_weighted_adjacency_matrix(weight: {:?})", &data.getdenseweightedadjacencymatrix.weight));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.get_dense_weighted_adjacency_matrix(data.getdenseweightedadjacencymatrix.weight.clone());
    }
    

    267 => {
        trace.push(format!("iter_node_ids()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.iter_node_ids().collect::<Vec<_>>();
            
    }
    

    268 => {
        trace.push(format!("par_iter_node_ids()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.par_iter_node_ids().collect::<Vec<_>>();
            
    }
    

    269 => {
        trace.push(format!("iter_node_names()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.iter_node_names().collect::<Vec<_>>();
            
    }
    

    270 => {
        trace.push(format!("par_iter_node_names()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.par_iter_node_names().collect::<Vec<_>>();
            
    }
    

    271 => {
        trace.push(format!("iter_unique_node_type_ids()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.iter_unique_node_type_ids().map(|x| x.collect::<Vec<_>>());
            
    }
    

    272 => {
        trace.push(format!("iter_node_type_counts()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.iter_node_type_counts().map(|x| x.collect::<Vec<_>>());
            
    }
    

    273 => {
        trace.push(format!("iter_unique_node_type_ids_and_counts()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.iter_unique_node_type_ids_and_counts().map(|x| x.collect::<Vec<_>>());
            
    }
    

    274 => {
        trace.push(format!("iter_unique_node_type_names()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.iter_unique_node_type_names().map(|x| x.collect::<Vec<_>>());
            
    }
    

    275 => {
        trace.push(format!("iter_unique_node_type_names_and_counts()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.iter_unique_node_type_names_and_counts().map(|x| x.collect::<Vec<_>>());
            
    }
    

    276 => {
        trace.push(format!("iter_unique_edge_type_ids()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.iter_unique_edge_type_ids().map(|x| x.collect::<Vec<_>>());
            
    }
    

    277 => {
        trace.push(format!("iter_edge_type_counts()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.iter_edge_type_counts().map(|x| x.collect::<Vec<_>>());
            
    }
    

    278 => {
        trace.push(format!("iter_unique_edge_type_ids_and_counts()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.iter_unique_edge_type_ids_and_counts().map(|x| x.collect::<Vec<_>>());
            
    }
    

    279 => {
        trace.push(format!("iter_unique_edge_type_names_and_counts()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.iter_unique_edge_type_names_and_counts().map(|x| x.collect::<Vec<_>>());
            
    }
    

    280 => {
        trace.push(format!("iter_unique_edge_type_names()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.iter_unique_edge_type_names().map(|x| x.collect::<Vec<_>>());
            
    }
    

    281 => {
        trace.push(format!("iter_node_degrees()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.iter_node_degrees().collect::<Vec<_>>();
            
    }
    

    282 => {
        trace.push(format!("par_iter_node_degrees()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.par_iter_node_degrees().collect::<Vec<_>>();
            
    }
    

    283 => {
        trace.push(format!("iter_weighted_node_degrees()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.iter_weighted_node_degrees().map(|x| x.collect::<Vec<_>>());
            
    }
    

    284 => {
        trace.push(format!("par_iter_weighted_node_degrees()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.par_iter_weighted_node_degrees().map(|x| x.collect::<Vec<_>>());
            
    }
    

    285 => {
        trace.push(format!("iter_connected_node_ids()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.iter_connected_node_ids().collect::<Vec<_>>();
            
    }
    

    286 => {
        trace.push(format!("iter_singleton_node_ids()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.iter_singleton_node_ids().collect::<Vec<_>>();
            
    }
    

    287 => {
        trace.push(format!("iter_singleton_node_names()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.iter_singleton_node_names().collect::<Vec<_>>();
            
    }
    

    288 => {
        trace.push(format!("iter_singleton_nodes_with_selfloops_node_ids()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.iter_singleton_nodes_with_selfloops_node_ids().collect::<Vec<_>>();
            
    }
    

    289 => {
        trace.push(format!("iter_singleton_nodes_with_selfloops_node_names()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.iter_singleton_nodes_with_selfloops_node_names().collect::<Vec<_>>();
            
    }
    

    290 => {
        trace.push(format!("iter_singleton_node_type_ids()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.iter_singleton_node_type_ids().map(|x| x.collect::<Vec<_>>());
            
    }
    

    291 => {
        trace.push(format!("iter_singleton_edge_type_ids()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.iter_singleton_edge_type_ids().map(|x| x.collect::<Vec<_>>());
            
    }
    

    292 => {
        trace.push(format!("iter_singleton_node_type_names()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.iter_singleton_node_type_names().map(|x| x.collect::<Vec<_>>());
            
    }
    

    293 => {
        trace.push(format!("iter_singleton_edge_type_names()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.iter_singleton_edge_type_names().map(|x| x.collect::<Vec<_>>());
            
    }
    

    294 => {
        trace.push(format!("iter_source_node_ids(directed: {:?})", &data.itersourcenodeids.directed));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.iter_source_node_ids(data.itersourcenodeids.directed.clone()).collect::<Vec<_>>();
            
    }
    

    295 => {
        trace.push(format!("iter_edge_weights()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.iter_edge_weights().map(|x| x.collect::<Vec<_>>());
            
    }
    

    296 => {
        trace.push(format!("par_iter_edge_weights()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.par_iter_edge_weights().map(|x| x.collect::<Vec<_>>());
            
    }
    

    297 => {
        trace.push(format!("par_iter_source_node_ids(directed: {:?})", &data.paritersourcenodeids.directed));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.par_iter_source_node_ids(data.paritersourcenodeids.directed.clone()).collect::<Vec<_>>();
            
    }
    

    298 => {
        trace.push(format!("iter_destination_node_ids(directed: {:?})", &data.iterdestinationnodeids.directed));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.iter_destination_node_ids(data.iterdestinationnodeids.directed.clone()).collect::<Vec<_>>();
            
    }
    

    299 => {
        trace.push(format!("par_iter_destination_node_ids(directed: {:?})", &data.pariterdestinationnodeids.directed));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.par_iter_destination_node_ids(data.pariterdestinationnodeids.directed.clone()).collect::<Vec<_>>();
            
    }
    

    300 => {
        trace.push(format!("iter_node_ids_and_node_type_ids()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.iter_node_ids_and_node_type_ids().collect::<Vec<_>>();
            
    }
    

    301 => {
        trace.push(format!("iter_one_hot_encoded_node_type_ids()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.iter_one_hot_encoded_node_type_ids().map(|x| x.collect::<Vec<_>>());
            
    }
    

    302 => {
        trace.push(format!("iter_one_hot_encoded_known_node_type_ids()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.iter_one_hot_encoded_known_node_type_ids().map(|x| x.collect::<Vec<_>>());
            
    }
    

    303 => {
        trace.push(format!("iter_node_names_and_node_type_names()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.iter_node_names_and_node_type_names().collect::<Vec<_>>();
            
    }
    

    304 => {
        trace.push(format!("par_iter_node_names_and_node_type_names()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.par_iter_node_names_and_node_type_names().collect::<Vec<_>>();
            
    }
    

    305 => {
        trace.push(format!("iter_edge_node_ids(directed: {:?})", &data.iteredgenodeids.directed));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.iter_edge_node_ids(data.iteredgenodeids.directed.clone()).collect::<Vec<_>>();
            
    }
    

    306 => {
        trace.push(format!("iter_edges(directed: {:?})", &data.iteredges.directed));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.iter_edges(data.iteredges.directed.clone()).collect::<Vec<_>>();
            
    }
    

    307 => {
        trace.push(format!("par_iter_edge_node_ids(directed: {:?})", &data.pariteredgenodeids.directed));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.par_iter_edge_node_ids(data.pariteredgenodeids.directed.clone()).collect::<Vec<_>>();
            
    }
    

    308 => {
        trace.push(format!("par_iter_directed_edge_ids()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.par_iter_directed_edge_ids().collect::<Vec<_>>();
            
    }
    

    309 => {
        trace.push(format!("par_iter_edges(directed: {:?})", &data.pariteredges.directed));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.par_iter_edges(data.pariteredges.directed.clone()).collect::<Vec<_>>();
            
    }
    

    310 => {
        trace.push(format!("iter_edge_node_ids_and_edge_weight()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.iter_edge_node_ids_and_edge_weight().map(|x| x.collect::<Vec<_>>());
            
    }
    

    311 => {
        trace.push(format!("par_iter_edge_node_ids_and_edge_weight()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.par_iter_edge_node_ids_and_edge_weight().map(|x| x.collect::<Vec<_>>());
            
    }
    

    312 => {
        trace.push(format!("iter_edge_node_ids_and_edge_type_id(directed: {:?})", &data.iteredgenodeidsandedgetypeid.directed));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.iter_edge_node_ids_and_edge_type_id(data.iteredgenodeidsandedgetypeid.directed.clone()).collect::<Vec<_>>();
            
    }
    

    313 => {
        trace.push(format!("iter_one_hot_encoded_edge_type_ids()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.iter_one_hot_encoded_edge_type_ids().map(|x| x.collect::<Vec<_>>());
            
    }
    

    314 => {
        trace.push(format!("iter_one_hot_encoded_known_edge_type_ids()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.iter_one_hot_encoded_known_edge_type_ids().map(|x| x.collect::<Vec<_>>());
            
    }
    

    315 => {
        trace.push(format!("iter_edge_node_names_and_edge_type_name(directed: {:?})", &data.iteredgenodenamesandedgetypename.directed));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.iter_edge_node_names_and_edge_type_name(data.iteredgenodenamesandedgetypename.directed.clone());
    }
    

    316 => {
        trace.push(format!("par_iter_edge_node_names_and_edge_type_name(directed: {:?})", &data.pariteredgenodenamesandedgetypename.directed));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.par_iter_edge_node_names_and_edge_type_name(data.pariteredgenodenamesandedgetypename.directed.clone());
    }
    

    317 => {
        trace.push(format!("par_iter_edge_node_ids_and_edge_type_id(directed: {:?})", &data.pariteredgenodeidsandedgetypeid.directed));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.par_iter_edge_node_ids_and_edge_type_id(data.pariteredgenodeidsandedgetypeid.directed.clone()).collect::<Vec<_>>();
            
    }
    

    318 => {
        trace.push(format!("par_iter_edge_node_names_and_edge_type_name_and_edge_weight(directed: {:?})", &data.pariteredgenodenamesandedgetypenameandedgeweight.directed));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.par_iter_edge_node_names_and_edge_type_name_and_edge_weight(data.pariteredgenodenamesandedgetypenameandedgeweight.directed.clone());
    }
    

    319 => {
        trace.push(format!("iter_edge_node_names_and_edge_type_name_and_edge_weight(directed: {:?})", &data.iteredgenodenamesandedgetypenameandedgeweight.directed));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.iter_edge_node_names_and_edge_type_name_and_edge_weight(data.iteredgenodenamesandedgetypenameandedgeweight.directed.clone());
    }
    

    320 => {
        trace.push(format!("par_iter_edge_node_ids_and_edge_type_id_and_edge_weight(directed: {:?})", &data.pariteredgenodeidsandedgetypeidandedgeweight.directed));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.par_iter_edge_node_ids_and_edge_type_id_and_edge_weight(data.pariteredgenodeidsandedgetypeidandedgeweight.directed.clone()).collect::<Vec<_>>();
            
    }
    

    321 => {
        trace.push(format!("iter_edge_node_ids_and_edge_type_id_and_edge_weight(directed: {:?})", &data.iteredgenodeidsandedgetypeidandedgeweight.directed));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.iter_edge_node_ids_and_edge_type_id_and_edge_weight(data.iteredgenodeidsandedgetypeidandedgeweight.directed.clone()).collect::<Vec<_>>();
            
    }
    

    322 => {
        trace.push(format!("iter_unique_edge_node_ids(directed: {:?})", &data.iteruniqueedgenodeids.directed));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.iter_unique_edge_node_ids(data.iteruniqueedgenodeids.directed.clone()).collect::<Vec<_>>();
            
    }
    

    323 => {
        trace.push(format!("iter_unique_source_node_ids()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.iter_unique_source_node_ids().collect::<Vec<_>>();
            
    }
    

    324 => {
        trace.push(format!("iter_edge_ids_with_unknown_edge_types()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.iter_edge_ids_with_unknown_edge_types().map(|x| x.collect::<Vec<_>>());
            
    }
    

    325 => {
        trace.push(format!("iter_edge_ids_with_known_edge_types()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.iter_edge_ids_with_known_edge_types().map(|x| x.collect::<Vec<_>>());
            
    }
    

    326 => {
        trace.push(format!("iter_edge_node_ids_with_unknown_edge_types(directed: {:?})", &data.iteredgenodeidswithunknownedgetypes.directed));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.iter_edge_node_ids_with_unknown_edge_types(data.iteredgenodeidswithunknownedgetypes.directed.clone()).map(|x| x.collect::<Vec<_>>());
            
    }
    

    327 => {
        trace.push(format!("iter_edge_node_ids_with_known_edge_types(directed: {:?})", &data.iteredgenodeidswithknownedgetypes.directed));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.iter_edge_node_ids_with_known_edge_types(data.iteredgenodeidswithknownedgetypes.directed.clone()).map(|x| x.collect::<Vec<_>>());
            
    }
    

    328 => {
        trace.push(format!("iter_node_ids_with_unknown_node_types()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.iter_node_ids_with_unknown_node_types().map(|x| x.collect::<Vec<_>>());
            
    }
    

    329 => {
        trace.push(format!("iter_node_ids_with_known_node_types()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.iter_node_ids_with_known_node_types().map(|x| x.collect::<Vec<_>>());
            
    }
    

    330 => {
        trace.push(format!("iter_edge_node_names_with_unknown_edge_types(directed: {:?})", &data.iteredgenodenameswithunknownedgetypes.directed));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.iter_edge_node_names_with_unknown_edge_types(data.iteredgenodenameswithunknownedgetypes.directed.clone()).map(|x| x.collect::<Vec<_>>());
            
    }
    

    331 => {
        trace.push(format!("iter_edge_node_names_with_known_edge_types(directed: {:?})", &data.iteredgenodenameswithknownedgetypes.directed));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.iter_edge_node_names_with_known_edge_types(data.iteredgenodenameswithknownedgetypes.directed.clone()).map(|x| x.collect::<Vec<_>>());
            
    }
    

    332 => {
        trace.push(format!("iter_node_names_with_unknown_node_types()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.iter_node_names_with_unknown_node_types().map(|x| x.collect::<Vec<_>>());
            
    }
    

    333 => {
        trace.push(format!("iter_node_names_with_known_node_types()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.iter_node_names_with_known_node_types().map(|x| x.collect::<Vec<_>>());
            
    }
    

    334 => {
        trace.push(format!("par_iter_edge_ids_with_unknown_edge_types()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.par_iter_edge_ids_with_unknown_edge_types().map(|x| x.collect::<Vec<_>>());
            
    }
    

    335 => {
        trace.push(format!("par_iter_edge_ids_with_known_edge_types()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.par_iter_edge_ids_with_known_edge_types().map(|x| x.collect::<Vec<_>>());
            
    }
    

    336 => {
        trace.push(format!("par_iter_edge_node_ids_with_unknown_edge_types(directed: {:?})", &data.pariteredgenodeidswithunknownedgetypes.directed));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.par_iter_edge_node_ids_with_unknown_edge_types(data.pariteredgenodeidswithunknownedgetypes.directed.clone()).map(|x| x.collect::<Vec<_>>());
            
    }
    

    337 => {
        trace.push(format!("par_iter_edge_node_ids_with_known_edge_types(directed: {:?})", &data.pariteredgenodeidswithknownedgetypes.directed));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.par_iter_edge_node_ids_with_known_edge_types(data.pariteredgenodeidswithknownedgetypes.directed.clone()).map(|x| x.collect::<Vec<_>>());
            
    }
    

    338 => {
        trace.push(format!("par_iter_node_ids_with_unknown_node_types()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.par_iter_node_ids_with_unknown_node_types().map(|x| x.collect::<Vec<_>>());
            
    }
    

    339 => {
        trace.push(format!("par_iter_node_ids_with_known_node_types()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.par_iter_node_ids_with_known_node_types().map(|x| x.collect::<Vec<_>>());
            
    }
    

    340 => {
        trace.push(format!("par_iter_edge_node_names_with_unknown_edge_types(directed: {:?})", &data.pariteredgenodenameswithunknownedgetypes.directed));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.par_iter_edge_node_names_with_unknown_edge_types(data.pariteredgenodenameswithunknownedgetypes.directed.clone()).map(|x| x.collect::<Vec<_>>());
            
    }
    

    341 => {
        trace.push(format!("par_iter_edge_node_names_with_known_edge_types(directed: {:?})", &data.pariteredgenodenameswithknownedgetypes.directed));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.par_iter_edge_node_names_with_known_edge_types(data.pariteredgenodenameswithknownedgetypes.directed.clone()).map(|x| x.collect::<Vec<_>>());
            
    }
    

    342 => {
        trace.push(format!("par_iter_node_names_with_unknown_node_types()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.par_iter_node_names_with_unknown_node_types().map(|x| x.collect::<Vec<_>>());
            
    }
    

    343 => {
        trace.push(format!("par_iter_node_names_with_known_node_types()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
            let _ = graph.par_iter_node_names_with_known_node_types().map(|x| x.collect::<Vec<_>>());
            
    }
    

    344 => {
        trace.push(format!("get_laplacian_transformed_graph(verbose: {:?})", &data.getlaplaciantransformedgraph.verbose));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        graph = graph.get_laplacian_transformed_graph(data.getlaplaciantransformedgraph.verbose.clone());
    }
    

    345 => {
        trace.push(format!("get_random_walk_normalized_laplacian_transformed_graph(verbose: {:?})", &data.getrandomwalknormalizedlaplaciantransformedgraph.verbose));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        graph = graph.get_random_walk_normalized_laplacian_transformed_graph(data.getrandomwalknormalizedlaplaciantransformedgraph.verbose.clone());
    }
    

    346 => {
        trace.push(format!("get_symmetric_normalized_laplacian_transformed_graph(verbose: {:?})", &data.getsymmetricnormalizedlaplaciantransformedgraph.verbose));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
        if let Ok(res) = graph.get_symmetric_normalized_laplacian_transformed_graph(data.getsymmetricnormalizedlaplaciantransformedgraph.verbose.clone()) {
            graph = res;
        }
        
    }
    

    347 => {
        trace.push(format!("get_symmetric_normalized_transformed_graph(verbose: {:?})", &data.getsymmetricnormalizedtransformedgraph.verbose));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
        if let Ok(res) = graph.get_symmetric_normalized_transformed_graph(data.getsymmetricnormalizedtransformedgraph.verbose.clone()) {
            graph = res;
        }
        
    }
    

    348 => {
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
    

    349 => {
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
    

    350 => {
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
    

    351 => {
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
    

    352 => {
        trace.push(format!("has_default_graph_name()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.has_default_graph_name();
    }
    

    353 => {
        trace.push(format!("has_nodes()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.has_nodes();
    }
    

    354 => {
        trace.push(format!("has_edges()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.has_edges();
    }
    

    355 => {
        trace.push(format!("has_trap_nodes()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.has_trap_nodes();
    }
    

    356 => {
        trace.push(format!("is_directed()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.is_directed();
    }
    

    357 => {
        trace.push(format!("has_edge_weights()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.has_edge_weights();
    }
    

    358 => {
        trace.push(format!("has_edge_weights_representing_probabilities()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.has_edge_weights_representing_probabilities();
    }
    

    359 => {
        trace.push(format!("has_weighted_singleton_nodes()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.has_weighted_singleton_nodes();
    }
    

    360 => {
        trace.push(format!("has_constant_edge_weights()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.has_constant_edge_weights();
    }
    

    361 => {
        trace.push(format!("has_negative_edge_weights()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.has_negative_edge_weights();
    }
    

    362 => {
        trace.push(format!("has_edge_types()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.has_edge_types();
    }
    

    363 => {
        trace.push(format!("has_selfloops()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.has_selfloops();
    }
    

    364 => {
        trace.push(format!("has_disconnected_nodes()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.has_disconnected_nodes();
    }
    

    365 => {
        trace.push(format!("has_singleton_nodes()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.has_singleton_nodes();
    }
    

    366 => {
        trace.push(format!("has_singleton_nodes_with_selfloops()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.has_singleton_nodes_with_selfloops();
    }
    

    367 => {
        trace.push(format!("is_connected(verbose: {:?})", &data.isconnected.verbose));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.is_connected(data.isconnected.verbose.clone());
    }
    

    368 => {
        trace.push(format!("has_node_types()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.has_node_types();
    }
    

    369 => {
        trace.push(format!("has_multilabel_node_types()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.has_multilabel_node_types();
    }
    

    370 => {
        trace.push(format!("has_unknown_node_types()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.has_unknown_node_types();
    }
    

    371 => {
        trace.push(format!("has_known_node_types()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.has_known_node_types();
    }
    

    372 => {
        trace.push(format!("has_unknown_edge_types()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.has_unknown_edge_types();
    }
    

    373 => {
        trace.push(format!("has_known_edge_types()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.has_known_edge_types();
    }
    

    374 => {
        trace.push(format!("has_homogeneous_node_types()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.has_homogeneous_node_types();
    }
    

    375 => {
        trace.push(format!("has_homogeneous_edge_types()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.has_homogeneous_edge_types();
    }
    

    376 => {
        trace.push(format!("has_singleton_node_types()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.has_singleton_node_types();
    }
    

    377 => {
        trace.push(format!("has_node_oddities()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.has_node_oddities();
    }
    

    378 => {
        trace.push(format!("has_node_types_oddities()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.has_node_types_oddities();
    }
    

    379 => {
        trace.push(format!("has_singleton_edge_types()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.has_singleton_edge_types();
    }
    

    380 => {
        trace.push(format!("has_edge_types_oddities()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.has_edge_types_oddities();
    }
    

    381 => {
        trace.push(format!("is_multigraph()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.is_multigraph();
    }
    

    382 => {
        trace.push(format!("has_nodes_sorted_by_decreasing_outbound_node_degree()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.has_nodes_sorted_by_decreasing_outbound_node_degree();
    }
    

    383 => {
        trace.push(format!("has_nodes_sorted_by_increasing_outbound_node_degree()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.has_nodes_sorted_by_increasing_outbound_node_degree();
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
        trace.push(format!("to_directed_inplace()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        let _ = graph.to_directed_inplace();
    }
    

    386 => {
        trace.push(format!("to_directed()", ));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        graph = graph.to_directed();
    }
    

    387 => {
        trace.push(format!("to_upper_triangular(verbose: {:?})", &data.touppertriangular.verbose));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        graph = graph.to_upper_triangular(data.touppertriangular.verbose.clone());
    }
    

    388 => {
        trace.push(format!("to_lower_triangular(verbose: {:?})", &data.tolowertriangular.verbose));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        graph = graph.to_lower_triangular(data.tolowertriangular.verbose.clone());
    }
    

    389 => {
        trace.push(format!("to_main_diagonal(verbose: {:?})", &data.tomaindiagonal.verbose));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        graph = graph.to_main_diagonal(data.tomaindiagonal.verbose.clone());
    }
    

    390 => {
        trace.push(format!("to_anti_diagonal(verbose: {:?})", &data.toantidiagonal.verbose));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        graph = graph.to_anti_diagonal(data.toantidiagonal.verbose.clone());
    }
    

    391 => {
        trace.push(format!("to_bidiagonal(verbose: {:?})", &data.tobidiagonal.verbose));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        graph = graph.to_bidiagonal(data.tobidiagonal.verbose.clone());
    }
    

    392 => {
        trace.push(format!("to_arrowhead(verbose: {:?})", &data.toarrowhead.verbose));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        graph = graph.to_arrowhead(data.toarrowhead.verbose.clone());
    }
    

    393 => {
        trace.push(format!("to_transposed(verbose: {:?})", &data.totransposed.verbose));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        graph = graph.to_transposed(data.totransposed.verbose.clone());
    }
    

    394 => {
        trace.push(format!("to_complementary(verbose: {:?})", &data.tocomplementary.verbose));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        graph = graph.to_complementary(data.tocomplementary.verbose.clone());
    }
    

    395 => {
        trace.push(format!("get_node_label_random_holdout(train_size: {:?}, use_stratification: {:?}, random_state: {:?})", &data.getnodelabelrandomholdout.train_size, &data.getnodelabelrandomholdout.use_stratification, &data.getnodelabelrandomholdout.random_state));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
        if let Ok((res1, res2)) = graph.get_node_label_random_holdout(data.getnodelabelrandomholdout.train_size.clone(), data.getnodelabelrandomholdout.use_stratification.clone(), data.getnodelabelrandomholdout.random_state.clone()) {
            if rng.next() % 2 == 0 {
                graph = res1;
            } else {
                graph = res2;
            }
        }
        
    }
    

    396 => {
        trace.push(format!("get_node_label_kfold(k: {:?}, k_index: {:?}, use_stratification: {:?}, random_state: {:?})", &data.getnodelabelkfold.k, &data.getnodelabelkfold.k_index, &data.getnodelabelkfold.use_stratification, &data.getnodelabelkfold.random_state));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
        if let Ok((res1, res2)) = graph.get_node_label_kfold(data.getnodelabelkfold.k.clone(), data.getnodelabelkfold.k_index.clone(), data.getnodelabelkfold.use_stratification.clone(), data.getnodelabelkfold.random_state.clone()) {
            if rng.next() % 2 == 0 {
                graph = res1;
            } else {
                graph = res2;
            }
        }
        
    }
    

    397 => {
        trace.push(format!("get_edge_label_random_holdout(train_size: {:?}, use_stratification: {:?}, random_state: {:?})", &data.getedgelabelrandomholdout.train_size, &data.getedgelabelrandomholdout.use_stratification, &data.getedgelabelrandomholdout.random_state));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
        if let Ok((res1, res2)) = graph.get_edge_label_random_holdout(data.getedgelabelrandomholdout.train_size.clone(), data.getedgelabelrandomholdout.use_stratification.clone(), data.getedgelabelrandomholdout.random_state.clone()) {
            if rng.next() % 2 == 0 {
                graph = res1;
            } else {
                graph = res2;
            }
        }
        
    }
    

    398 => {
        trace.push(format!("get_edge_label_kfold(k: {:?}, k_index: {:?}, use_stratification: {:?}, random_state: {:?})", &data.getedgelabelkfold.k, &data.getedgelabelkfold.k_index, &data.getedgelabelkfold.use_stratification, &data.getedgelabelkfold.random_state));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
        if let Ok((res1, res2)) = graph.get_edge_label_kfold(data.getedgelabelkfold.k.clone(), data.getedgelabelkfold.k_index.clone(), data.getedgelabelkfold.use_stratification.clone(), data.getedgelabelkfold.random_state.clone()) {
            if rng.next() % 2 == 0 {
                graph = res1;
            } else {
                graph = res2;
            }
        }
        
    }
    

    399 => {
        trace.push(format!("get_random_subgraph(nodes_number: {:?}, random_state: {:?}, verbose: {:?})", &data.getrandomsubgraph.nodes_number, &data.getrandomsubgraph.random_state, &data.getrandomsubgraph.verbose));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }));
        
        if let Ok(res) = graph.get_random_subgraph(data.getrandomsubgraph.nodes_number.clone(), data.getrandomsubgraph.random_state.clone(), data.getrandomsubgraph.verbose.clone()) {
            graph = res;
        }
        
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
        match rng.next() % 400 {

    0 => {
        let _ = graph.spanning_arborescence_kruskal(data.spanningarborescencekruskal.verbose.clone());
    }
    

    1 => {
        let _ = graph.spanning_arborescence(data.spanningarborescence.verbose.clone());
    }
    

    2 => {
        let _ = graph.connected_components(data.connectedcomponents.verbose.clone());
    }
    

    3 => {
        let _ = graph.get_node_label_prediction_mini_batch(data.getnodelabelpredictionminibatch.idx.clone(), data.getnodelabelpredictionminibatch.batch_size.clone(), data.getnodelabelpredictionminibatch.include_central_node.clone(), data.getnodelabelpredictionminibatch.return_edge_weights.clone(), data.getnodelabelpredictionminibatch.max_neighbours.clone());
    }
    

    4 => {
        
            let _ = graph.par_iter_edge_prediction_metrics(data.pariteredgepredictionmetrics.normalize.clone(), data.pariteredgepredictionmetrics.verbose.clone()).collect::<Vec<_>>();
            
    }
    

    5 => {
        let _ = graph.get_okapi_bm25_node_feature_propagation(data.getokapibm25nodefeaturepropagation.features.clone(), data.getokapibm25nodefeaturepropagation.iterations.map(|x| x as usize).clone(), data.getokapibm25nodefeaturepropagation.maximal_distance.clone(), data.getokapibm25nodefeaturepropagation.k1.clone(), data.getokapibm25nodefeaturepropagation.b.clone(), data.getokapibm25nodefeaturepropagation.include_central_node.clone(), data.getokapibm25nodefeaturepropagation.verbose.clone());
    }
    

    6 => {
        let _ = graph.get_okapi_bm25_node_label_propagation(data.getokapibm25nodelabelpropagation.iterations.map(|x| x as usize).clone(), data.getokapibm25nodelabelpropagation.maximal_distance.clone(), data.getokapibm25nodelabelpropagation.k1.clone(), data.getokapibm25nodelabelpropagation.b.clone(), data.getokapibm25nodelabelpropagation.verbose.clone());
    }
    

    7 => {
        graph = graph.sort_by_increasing_outbound_node_degree(data.sortbyincreasingoutboundnodedegree.verbose.clone());
    }
    

    8 => {
        graph = graph.sort_by_decreasing_outbound_node_degree(data.sortbydecreasingoutboundnodedegree.verbose.clone());
    }
    

    9 => {
        let _ = graph.to_dot(data.todot.use_node_names.clone());
    }
    

    10 => {
        
        if let Ok(res) = graph.remap_from_node_ids(data.remapfromnodeids.node_ids.clone(), data.remapfromnodeids.verbose.clone()) {
            graph = res;
        }
        
    }
    

    11 => {
        let _ = graph.is_singleton_from_node_id(data.issingletonfromnodeid.node_id.clone());
    }
    

    12 => {
        let _ = graph.is_singleton_with_selfloops_from_node_id(data.issingletonwithselfloopsfromnodeid.node_id.clone());
    }
    

    13 => {
        let _ = graph.has_node_type_id(data.hasnodetypeid.node_type_id.clone());
    }
    

    14 => {
        let _ = graph.has_edge_type_id(data.hasedgetypeid.edge_type_id.clone());
    }
    

    15 => {
        let _ = graph.has_edge_from_node_ids(data.hasedgefromnodeids.src.clone(), data.hasedgefromnodeids.dst.clone());
    }
    

    16 => {
        let _ = graph.has_selfloop_from_node_id(data.hasselfloopfromnodeid.node_id.clone());
    }
    

    17 => {
        let _ = graph.has_edge_from_node_ids_and_edge_type_id(data.hasedgefromnodeidsandedgetypeid.src.clone(), data.hasedgefromnodeidsandedgetypeid.dst.clone(), data.hasedgefromnodeidsandedgetypeid.edge_type.clone());
    }
    

    18 => {
        let _ = graph.is_trap_node_from_node_id(data.istrapnodefromnodeid.node_id.clone());
    }
    

    19 => {
        let _ = graph.get_preferential_attachment_from_node_ids(data.getpreferentialattachmentfromnodeids.source_node_id.clone(), data.getpreferentialattachmentfromnodeids.destination_node_id.clone(), data.getpreferentialattachmentfromnodeids.normalize.clone());
    }
    

    20 => {
        let _ = graph.get_weighted_preferential_attachment_from_node_ids(data.getweightedpreferentialattachmentfromnodeids.source_node_id.clone(), data.getweightedpreferentialattachmentfromnodeids.destination_node_id.clone(), data.getweightedpreferentialattachmentfromnodeids.normalize.clone());
    }
    

    21 => {
        let _ = graph.get_jaccard_coefficient_from_node_ids(data.getjaccardcoefficientfromnodeids.source_node_id.clone(), data.getjaccardcoefficientfromnodeids.destination_node_id.clone());
    }
    

    22 => {
        let _ = graph.get_adamic_adar_index_from_node_ids(data.getadamicadarindexfromnodeids.source_node_id.clone(), data.getadamicadarindexfromnodeids.destination_node_id.clone());
    }
    

    23 => {
        let _ = graph.get_resource_allocation_index_from_node_ids(data.getresourceallocationindexfromnodeids.source_node_id.clone(), data.getresourceallocationindexfromnodeids.destination_node_id.clone());
    }
    

    24 => {
        let _ = graph.get_weighted_resource_allocation_index_from_node_ids(data.getweightedresourceallocationindexfromnodeids.source_node_id.clone(), data.getweightedresourceallocationindexfromnodeids.destination_node_id.clone());
    }
    

    25 => {
        let _ = graph.enable(data.enable.vector_sources.clone(), data.enable.vector_destinations.clone(), data.enable.vector_cumulative_node_degrees.clone());
    }
    

    26 => {
        graph.disable_all()
    }
    

    27 => {
        let _ = graph.get_number_of_triangles(data.getnumberoftriangles.normalize.clone(), data.getnumberoftriangles.low_centrality.clone(), data.getnumberoftriangles.verbose.clone());
    }
    

    28 => {
        let _ = graph.get_triads_number();
    }
    

    29 => {
        let _ = graph.get_weighted_triads_number();
    }
    

    30 => {
        let _ = graph.get_transitivity(data.gettransitivity.low_centrality.clone(), data.gettransitivity.verbose.clone());
    }
    

    31 => {
        let _ = graph.get_number_of_triangles_per_node(data.getnumberoftrianglespernode.normalize.clone(), data.getnumberoftrianglespernode.low_centrality.clone(), data.getnumberoftrianglespernode.verbose.clone());
    }
    

    32 => {
        
            let _ = graph.iter_clustering_coefficient_per_node(data.iterclusteringcoefficientpernode.low_centrality.clone(), data.iterclusteringcoefficientpernode.verbose.clone()).collect::<Vec<_>>();
            
    }
    

    33 => {
        let _ = graph.get_clustering_coefficient_per_node(data.getclusteringcoefficientpernode.low_centrality.clone(), data.getclusteringcoefficientpernode.verbose.clone());
    }
    

    34 => {
        let _ = graph.get_clustering_coefficient(data.getclusteringcoefficient.low_centrality.clone(), data.getclusteringcoefficient.verbose.clone());
    }
    

    35 => {
        let _ = graph.get_average_clustering_coefficient(data.getaverageclusteringcoefficient.low_centrality.clone(), data.getaverageclusteringcoefficient.verbose.clone());
    }
    

    36 => {
        let _ = graph.get_minimum_path_node_ids_from_node_ids(data.getminimumpathnodeidsfromnodeids.src_node_id.clone(), data.getminimumpathnodeidsfromnodeids.dst_node_id.clone(), data.getminimumpathnodeidsfromnodeids.maximal_depth.clone());
    }
    

    37 => {
        let _ = graph.get_k_shortest_path_node_ids_from_node_ids(data.getkshortestpathnodeidsfromnodeids.src_node_id.clone(), data.getkshortestpathnodeidsfromnodeids.dst_node_id.clone(), (data.getkshortestpathnodeidsfromnodeids.k as usize).clone(), data.getkshortestpathnodeidsfromnodeids.max_path_length.clone(), data.getkshortestpathnodeidsfromnodeids.verbose.clone());
    }
    

    38 => {
        let _ = graph.get_eccentricity_from_node_id(data.geteccentricityfromnodeid.node_id.clone());
    }
    

    39 => {
        let _ = graph.get_weighted_eccentricity_from_node_id(data.getweightedeccentricityfromnodeid.node_id.clone(), data.getweightedeccentricityfromnodeid.use_edge_weights_as_probabilities.clone());
    }
    

    40 => {
        let _ = graph.get_weighted_minimum_path_node_ids_from_node_ids(data.getweightedminimumpathnodeidsfromnodeids.src_node_id.clone(), data.getweightedminimumpathnodeidsfromnodeids.dst_node_id.clone(), data.getweightedminimumpathnodeidsfromnodeids.use_edge_weights_as_probabilities.clone(), data.getweightedminimumpathnodeidsfromnodeids.maximal_depth.clone());
    }
    

    41 => {
        let _ = graph.get_breath_first_search_from_node_ids(data.getbreathfirstsearchfromnodeids.src_node_id.clone(), data.getbreathfirstsearchfromnodeids.dst_node_id.clone(), data.getbreathfirstsearchfromnodeids.compute_predecessors.clone(), data.getbreathfirstsearchfromnodeids.maximal_depth.clone());
    }
    

    42 => {
        let _ = graph.get_dijkstra_from_node_ids(data.getdijkstrafromnodeids.src_node_id.clone(), data.getdijkstrafromnodeids.maybe_dst_node_id.clone(), data.getdijkstrafromnodeids.maybe_dst_node_ids.clone(), data.getdijkstrafromnodeids.compute_predecessors.clone(), data.getdijkstrafromnodeids.maximal_depth.clone(), data.getdijkstrafromnodeids.use_edge_weights_as_probabilities.clone());
    }
    

    43 => {
        let _ = graph.get_diameter_naive(data.getdiameternaive.ignore_infinity.clone(), data.getdiameternaive.verbose.clone());
    }
    

    44 => {
        let _ = graph.get_diameter(data.getdiameter.ignore_infinity.clone(), data.getdiameter.verbose.clone());
    }
    

    45 => {
        let _ = graph.get_weighted_diameter_naive(data.getweighteddiameternaive.ignore_infinity.clone(), data.getweighteddiameternaive.use_edge_weights_as_probabilities.clone(), data.getweighteddiameternaive.verbose.clone());
    }
    

    46 => {
        
            let _ = graph.iter_edge_ids_from_node_ids(data.iteredgeidsfromnodeids.src.clone(), data.iteredgeidsfromnodeids.dst.clone()).map(|x| x.collect::<Vec<_>>());
            
    }
    

    47 => {
        
            let _ = graph.iter_edge_node_ids_and_edge_type_id_from_edge_type_id(data.iteredgenodeidsandedgetypeidfromedgetypeid.edge_type_id.clone(), data.iteredgenodeidsandedgetypeidfromedgetypeid.directed.clone()).map(|x| x.collect::<Vec<_>>());
            
    }
    

    48 => {
        
            let _ = graph.iter_node_ids_and_node_type_ids_from_node_type_id(data.iternodeidsandnodetypeidsfromnodetypeid.node_type_id.clone()).map(|x| x.collect::<Vec<_>>());
            
    }
    

    49 => {
        let _ = graph.iter_node_names_and_node_type_names_from_node_type_id(data.iternodenamesandnodetypenamesfromnodetypeid.node_type_id.clone());
    }
    

    50 => {
        let _ = graph.iter_edge_node_names_and_edge_type_name_from_edge_type_id(data.iteredgenodenamesandedgetypenamefromedgetypeid.edge_type_id.clone(), data.iteredgenodenamesandedgetypenamefromedgetypeid.directed.clone());
    }
    

    51 => {
        graph = graph.get_transitive_closure(data.gettransitiveclosure.iterations.map(|x| x as NodeT).clone(), data.gettransitiveclosure.verbose.clone());
    }
    

    52 => {
        graph = graph.get_all_shortest_paths(data.getallshortestpaths.iterations.map(|x| x as NodeT).clone(), data.getallshortestpaths.verbose.clone());
    }
    

    53 => {
        
        if let Ok(res) = graph.get_weighted_all_shortest_paths(data.getweightedallshortestpaths.iterations.map(|x| x as NodeT).clone(), data.getweightedallshortestpaths.use_edge_weights_as_probabilities.clone(), data.getweightedallshortestpaths.verbose.clone()) {
            graph = res;
        }
        
    }
    

    54 => {
        let _ = graph.strongly_connected_components();
    }
    

    55 => {
        let _ = graph.get_source_node_id_from_edge_id(data.getsourcenodeidfromedgeid.edge_id.clone());
    }
    

    56 => {
        let _ = graph.get_destination_node_id_from_edge_id(data.getdestinationnodeidfromedgeid.edge_id.clone());
    }
    

    57 => {
        let _ = graph.get_source_node_name_from_edge_id(data.getsourcenodenamefromedgeid.edge_id.clone());
    }
    

    58 => {
        let _ = graph.get_destination_node_name_from_edge_id(data.getdestinationnodenamefromedgeid.edge_id.clone());
    }
    

    59 => {
        let _ = graph.get_node_names_from_edge_id(data.getnodenamesfromedgeid.edge_id.clone());
    }
    

    60 => {
        let _ = graph.get_node_ids_from_edge_id(data.getnodeidsfromedgeid.edge_id.clone());
    }
    

    61 => {
        let _ = graph.get_edge_id_from_node_ids(data.getedgeidfromnodeids.src.clone(), data.getedgeidfromnodeids.dst.clone());
    }
    

    62 => {
        let _ = graph.get_node_ids_and_edge_type_id_from_edge_id(data.getnodeidsandedgetypeidfromedgeid.edge_id.clone());
    }
    

    63 => {
        let _ = graph.get_node_ids_and_edge_type_id_and_edge_weight_from_edge_id(data.getnodeidsandedgetypeidandedgeweightfromedgeid.edge_id.clone());
    }
    

    64 => {
        let _ = graph.get_top_k_central_node_ids(data.gettopkcentralnodeids.k.clone());
    }
    

    65 => {
        let _ = graph.get_weighted_top_k_central_node_ids(data.getweightedtopkcentralnodeids.k.clone());
    }
    

    66 => {
        let _ = graph.get_node_degree_from_node_id(data.getnodedegreefromnodeid.node_id.clone());
    }
    

    67 => {
        let _ = graph.get_weighted_node_degree_from_node_id(data.getweightednodedegreefromnodeid.node_id.clone());
    }
    

    68 => {
        let _ = graph.get_top_k_central_node_names(data.gettopkcentralnodenames.k.clone());
    }
    

    69 => {
        let _ = graph.get_node_type_id_from_node_id(data.getnodetypeidfromnodeid.node_id.clone());
    }
    

    70 => {
        let _ = graph.get_edge_type_id_from_edge_id(data.getedgetypeidfromedgeid.edge_id.clone());
    }
    

    71 => {
        let _ = graph.get_node_type_names_from_node_id(data.getnodetypenamesfromnodeid.node_id.clone());
    }
    

    72 => {
        let _ = graph.get_edge_type_name_from_edge_id(data.getedgetypenamefromedgeid.edge_id.clone());
    }
    

    73 => {
        let _ = graph.get_edge_type_name_from_edge_type_id(data.getedgetypenamefromedgetypeid.edge_type_id.clone());
    }
    

    74 => {
        let _ = graph.get_edge_weight_from_edge_id(data.getedgeweightfromedgeid.edge_id.clone());
    }
    

    75 => {
        let _ = graph.get_edge_weight_from_node_ids(data.getedgeweightfromnodeids.src.clone(), data.getedgeweightfromnodeids.dst.clone());
    }
    

    76 => {
        let _ = graph.get_edge_weight_from_node_ids_and_edge_type_id(data.getedgeweightfromnodeidsandedgetypeid.src.clone(), data.getedgeweightfromnodeidsandedgetypeid.dst.clone(), data.getedgeweightfromnodeidsandedgetypeid.edge_type.clone());
    }
    

    77 => {
        let _ = graph.get_node_name_from_node_id(data.getnodenamefromnodeid.node_id.clone());
    }
    

    78 => {
        let _ = graph.get_edge_node_names_from_edge_node_ids(data.getedgenodenamesfromedgenodeids.edge_node_ids.clone());
    }
    

    79 => {
        let _ = graph.get_edge_count_from_edge_type_id(data.getedgecountfromedgetypeid.edge_type_id.clone());
    }
    

    80 => {
        let _ = graph.get_node_count_from_node_type_id(data.getnodecountfromnodetypeid.node_type_id.clone());
    }
    

    81 => {
        let _ = graph.get_neighbour_node_ids_from_node_id(data.getneighbournodeidsfromnodeid.node_id.clone());
    }
    

    82 => {
        let _ = graph.get_minmax_edge_ids_from_node_ids(data.getminmaxedgeidsfromnodeids.src.clone(), data.getminmaxedgeidsfromnodeids.dst.clone());
    }
    

    83 => {
        let _ = graph.get_edge_id_from_node_ids_and_edge_type_id(data.getedgeidfromnodeidsandedgetypeid.src.clone(), data.getedgeidfromnodeidsandedgetypeid.dst.clone(), data.getedgeidfromnodeidsandedgetypeid.edge_type.clone());
    }
    

    84 => {
        let _ = graph.get_minmax_edge_ids_from_source_node_id(data.getminmaxedgeidsfromsourcenodeid.src.clone());
    }
    

    85 => {
        let _ = graph.get_node_type_name_from_node_type_id(data.getnodetypenamefromnodetypeid.node_type_id.clone());
    }
    

    86 => {
        
        if let Ok(res) = graph.filter_from_ids(data.filterfromids.node_ids_to_keep.clone(), data.filterfromids.node_ids_to_filter.clone(), data.filterfromids.node_type_ids_to_keep.clone(), data.filterfromids.node_type_ids_to_filter.clone(), data.filterfromids.node_type_id_to_keep.clone(), data.filterfromids.node_type_id_to_filter.clone(), data.filterfromids.edge_ids_to_keep.clone(), data.filterfromids.edge_ids_to_filter.clone(), data.filterfromids.edge_node_ids_to_keep.clone(), data.filterfromids.edge_node_ids_to_filter.clone(), data.filterfromids.edge_type_ids_to_keep.clone(), data.filterfromids.edge_type_ids_to_filter.clone(), data.filterfromids.min_edge_weight.clone(), data.filterfromids.max_edge_weight.clone(), data.filterfromids.filter_singleton_nodes.clone(), data.filterfromids.filter_singleton_nodes_with_selfloop.clone(), data.filterfromids.filter_selfloops.clone(), data.filterfromids.filter_parallel_edges.clone(), data.filterfromids.verbose.clone()) {
            graph = res;
        }
        
    }
    

    87 => {
        graph = graph.drop_unknown_node_types(data.dropunknownnodetypes.verbose.clone());
    }
    

    88 => {
        graph = graph.drop_unknown_edge_types(data.dropunknownedgetypes.verbose.clone());
    }
    

    89 => {
        graph = graph.drop_singleton_nodes(data.dropsingletonnodes.verbose.clone());
    }
    

    90 => {
        graph = graph.drop_singleton_nodes_with_selfloops(data.dropsingletonnodeswithselfloops.verbose.clone());
    }
    

    91 => {
        graph = graph.drop_disconnected_nodes(data.dropdisconnectednodes.verbose.clone());
    }
    

    92 => {
        graph = graph.drop_selfloops(data.dropselfloops.verbose.clone());
    }
    

    93 => {
        graph = graph.drop_parallel_edges(data.dropparalleledges.verbose.clone());
    }
    

    94 => {
        let _ = graph.validate_node_id(data.validatenodeid.node_id.clone());
    }
    

    95 => {
        let _ = graph.validate_node_ids(data.validatenodeids.node_ids.clone());
    }
    

    96 => {
        let _ = graph.validate_edge_id(data.validateedgeid.edge_id.clone());
    }
    

    97 => {
        let _ = graph.validate_edge_ids(data.validateedgeids.edge_ids.clone());
    }
    

    98 => {
        let _ = graph.must_not_contain_unknown_node_types();
    }
    

    99 => {
        let _ = graph.must_not_contain_unknown_edge_types();
    }
    

    100 => {
        let _ = graph.validate_node_type_id(data.validatenodetypeid.node_type_id.clone());
    }
    

    101 => {
        let _ = graph.validate_node_type_ids(data.validatenodetypeids.node_type_ids.clone());
    }
    

    102 => {
        let _ = graph.validate_edge_type_id(data.validateedgetypeid.edge_type_id.clone());
    }
    

    103 => {
        let _ = graph.validate_edge_type_ids(data.validateedgetypeids.edge_type_ids.clone());
    }
    

    104 => {
        let _ = graph.must_have_node_types();
    }
    

    105 => {
        let _ = graph.must_have_edge_types();
    }
    

    106 => {
        let _ = graph.must_be_undirected();
    }
    

    107 => {
        let _ = graph.must_be_multigraph();
    }
    

    108 => {
        let _ = graph.must_not_be_multigraph();
    }
    

    109 => {
        let _ = graph.must_have_edge_weights();
    }
    

    110 => {
        let _ = graph.must_have_known_node_types();
    }
    

    111 => {
        let _ = graph.must_have_unknown_node_types();
    }
    

    112 => {
        let _ = graph.must_have_known_edge_types();
    }
    

    113 => {
        let _ = graph.must_have_unknown_edge_types();
    }
    

    114 => {
        let _ = graph.must_have_edge_weights_representing_probabilities();
    }
    

    115 => {
        let _ = graph.must_have_positive_edge_weights();
    }
    

    116 => {
        let _ = graph.must_not_contain_weighted_singleton_nodes();
    }
    

    117 => {
        let _ = graph.must_have_edges();
    }
    

    118 => {
        let _ = graph.must_have_nodes();
    }
    

    119 => {
        
            let _ = graph.iter_degree_centrality().map(|x| x.collect::<Vec<_>>());
            
    }
    

    120 => {
        
            let _ = graph.par_iter_weighted_degree_centrality().map(|x| x.collect::<Vec<_>>());
            
    }
    

    121 => {
        let _ = graph.get_degree_centrality();
    }
    

    122 => {
        let _ = graph.get_weighted_degree_centrality();
    }
    

    123 => {
        
            let _ = graph.par_iter_closeness_centrality(data.pariterclosenesscentrality.verbose.clone()).collect::<Vec<_>>();
            
    }
    

    124 => {
        
            let _ = graph.par_iter_weighted_closeness_centrality(data.pariterweightedclosenesscentrality.use_edge_weights_as_probabilities.clone(), data.pariterweightedclosenesscentrality.verbose.clone()).map(|x| x.collect::<Vec<_>>());
            
    }
    

    125 => {
        let _ = graph.get_closeness_centrality(data.getclosenesscentrality.verbose.clone());
    }
    

    126 => {
        let _ = graph.get_weighted_closeness_centrality(data.getweightedclosenesscentrality.use_edge_weights_as_probabilities.clone(), data.getweightedclosenesscentrality.verbose.clone());
    }
    

    127 => {
        
            let _ = graph.par_iter_harmonic_centrality(data.pariterharmoniccentrality.verbose.clone()).collect::<Vec<_>>();
            
    }
    

    128 => {
        
            let _ = graph.par_iter_weighted_harmonic_centrality(data.pariterweightedharmoniccentrality.use_edge_weights_as_probabilities.clone(), data.pariterweightedharmoniccentrality.verbose.clone()).map(|x| x.collect::<Vec<_>>());
            
    }
    

    129 => {
        let _ = graph.get_harmonic_centrality(data.getharmoniccentrality.verbose.clone());
    }
    

    130 => {
        let _ = graph.get_weighted_harmonic_centrality(data.getweightedharmoniccentrality.use_edge_weights_as_probabilities.clone(), data.getweightedharmoniccentrality.verbose.clone());
    }
    

    131 => {
        let _ = graph.get_stress_centrality(data.getstresscentrality.normalize.clone(), data.getstresscentrality.verbose.clone());
    }
    

    132 => {
        let _ = graph.get_betweenness_centrality(data.getbetweennesscentrality.normalize.clone(), data.getbetweennesscentrality.verbose.clone());
    }
    

    133 => {
        let _ = graph.get_eigenvector_centrality(data.geteigenvectorcentrality.maximum_iterations_number.map(|x| x as usize).clone(), data.geteigenvectorcentrality.tollerance.clone());
    }
    

    134 => {
        let _ = graph.get_weighted_eigenvector_centrality(data.getweightedeigenvectorcentrality.maximum_iterations_number.map(|x| x as usize).clone(), data.getweightedeigenvectorcentrality.tollerance.clone());
    }
    

    135 => {
        let _ = graph.remove_inplace_node_type_ids(data.removeinplacenodetypeids.node_type_ids_to_remove.clone());
    }
    

    136 => {
        let _ = graph.remove_inplace_singleton_node_types();
    }
    

    137 => {
        let _ = graph.remove_inplace_edge_type_ids(data.removeinplaceedgetypeids.edge_type_ids_to_remove.clone());
    }
    

    138 => {
        let _ = graph.remove_inplace_singleton_edge_types();
    }
    

    139 => {
        
        if let Ok(res) = graph.remove_node_type_id(data.removenodetypeid.node_type_id.clone()) {
            graph = res;
        }
        
    }
    

    140 => {
        
        if let Ok(res) = graph.remove_singleton_node_types() {
            graph = res;
        }
        
    }
    

    141 => {
        
        if let Ok(res) = graph.remove_edge_type_id(data.removeedgetypeid.edge_type_id.clone()) {
            graph = res;
        }
        
    }
    

    142 => {
        
        if let Ok(res) = graph.remove_singleton_edge_types() {
            graph = res;
        }
        
    }
    

    143 => {
        let _ = graph.remove_inplace_node_types();
    }
    

    144 => {
        
        if let Ok(res) = graph.remove_node_types() {
            graph = res;
        }
        
    }
    

    145 => {
        let _ = graph.remove_inplace_edge_types();
    }
    

    146 => {
        
        if let Ok(res) = graph.remove_edge_types(data.removeedgetypes.verbose.clone()) {
            graph = res;
        }
        
    }
    

    147 => {
        let _ = graph.remove_inplace_edge_weights();
    }
    

    148 => {
        
        if let Ok(res) = graph.remove_edge_weights() {
            graph = res;
        }
        
    }
    

    149 => {
        let _ = graph.encode_edge(data.encodeedge.src.clone(), data.encodeedge.dst.clone());
    }
    

    150 => {
        let _ = graph.decode_edge(data.decodeedge.edge.clone());
    }
    

    151 => {
        let _ = graph.get_max_encodable_edge_number();
    }
    

    152 => {
        
            let _ = graph.par_iter_approximated_vertex_cover().collect::<Vec<_>>();
            
    }
    

    153 => {
        let _ = graph.approximated_vertex_cover_set();
    }
    

    154 => {
        let _ = graph.report();
    }
    

    155 => {
        let _ = graph.get_node_report_from_node_id(data.getnodereportfromnodeid.node_id.clone());
    }
    

    156 => {
        let _ = graph.textual_report();
    }
    

    157 => {
        let _ = graph.get_connected_components_number(data.getconnectedcomponentsnumber.verbose.clone());
    }
    

    158 => {
        let _ = graph.get_singleton_nodes_number();
    }
    

    159 => {
        let _ = graph.get_weighted_singleton_nodes_number();
    }
    

    160 => {
        let _ = graph.get_disconnected_nodes_number();
    }
    

    161 => {
        let _ = graph.get_singleton_node_ids();
    }
    

    162 => {
        let _ = graph.get_singleton_node_names();
    }
    

    163 => {
        let _ = graph.get_singleton_nodes_with_selfloops_number();
    }
    

    164 => {
        let _ = graph.get_singleton_with_selfloops_node_ids();
    }
    

    165 => {
        let _ = graph.get_singleton_with_selfloops_node_names();
    }
    

    166 => {
        let _ = graph.get_connected_nodes_number();
    }
    

    167 => {
        let _ = graph.get_density();
    }
    

    168 => {
        let _ = graph.get_trap_nodes_rate();
    }
    

    169 => {
        let _ = graph.get_node_degrees_mean();
    }
    

    170 => {
        let _ = graph.get_weighted_node_degrees_mean();
    }
    

    171 => {
        let _ = graph.get_undirected_edges_number();
    }
    

    172 => {
        let _ = graph.get_unique_undirected_edges_number();
    }
    

    173 => {
        let _ = graph.get_edges_number();
    }
    

    174 => {
        let _ = graph.get_unique_edges_number();
    }
    

    175 => {
        let _ = graph.get_node_degrees_median();
    }
    

    176 => {
        let _ = graph.get_weighted_node_degrees_median();
    }
    

    177 => {
        let _ = graph.get_weighted_maximum_node_degree();
    }
    

    178 => {
        let _ = graph.get_weighted_minimum_node_degree();
    }
    

    179 => {
        let _ = graph.get_maximum_node_degree();
    }
    

    180 => {
        let _ = graph.get_most_central_node_id();
    }
    

    181 => {
        let _ = graph.get_weighted_mininum_node_degree();
    }
    

    182 => {
        let _ = graph.get_minimum_node_degree();
    }
    

    183 => {
        let _ = graph.get_node_degrees_mode();
    }
    

    184 => {
        let _ = graph.get_selfloop_number();
    }
    

    185 => {
        let _ = graph.get_unique_selfloop_number();
    }
    

    186 => {
        let _ = graph.get_selfloop_nodes_rate();
    }
    

    187 => {
        let _ = graph.get_name();
    }
    

    188 => {
        let _ = graph.get_trap_nodes_number();
    }
    

    189 => {
        let _ = graph.get_source_node_ids(data.getsourcenodeids.directed.clone());
    }
    

    190 => {
        let _ = graph.get_source_names(data.getsourcenames.directed.clone());
    }
    

    191 => {
        let _ = graph.get_destination_node_ids(data.getdestinationnodeids.directed.clone());
    }
    

    192 => {
        let _ = graph.get_destination_names(data.getdestinationnames.directed.clone());
    }
    

    193 => {
        let _ = graph.get_node_names();
    }
    

    194 => {
        let _ = graph.get_node_ids();
    }
    

    195 => {
        let _ = graph.get_edge_type_ids();
    }
    

    196 => {
        let _ = graph.get_unique_edge_type_ids();
    }
    

    197 => {
        let _ = graph.get_edge_type_names();
    }
    

    198 => {
        let _ = graph.get_unique_edge_type_names();
    }
    

    199 => {
        let _ = graph.get_edge_weights();
    }
    

    200 => {
        let _ = graph.get_total_edge_weights();
    }
    

    201 => {
        let _ = graph.get_mininum_edge_weight();
    }
    

    202 => {
        let _ = graph.get_maximum_edge_weight();
    }
    

    203 => {
        let _ = graph.get_node_type_ids();
    }
    

    204 => {
        let _ = graph.get_one_hot_encoded_node_types();
    }
    

    205 => {
        let _ = graph.get_known_node_types_mask();
    }
    

    206 => {
        let _ = graph.get_unknown_node_types_mask();
    }
    

    207 => {
        let _ = graph.get_one_hot_encoded_known_node_types();
    }
    

    208 => {
        let _ = graph.get_one_hot_encoded_edge_types();
    }
    

    209 => {
        let _ = graph.get_one_hot_encoded_known_edge_types();
    }
    

    210 => {
        let _ = graph.get_node_type_names();
    }
    

    211 => {
        let _ = graph.get_unique_node_type_ids();
    }
    

    212 => {
        let _ = graph.get_unique_node_type_names();
    }
    

    213 => {
        let _ = graph.get_unique_directed_edges_number();
    }
    

    214 => {
        let _ = graph.get_nodes_mapping();
    }
    

    215 => {
        let _ = graph.get_edge_node_ids(data.getedgenodeids.directed.clone());
    }
    

    216 => {
        let _ = graph.get_edge_node_names(data.getedgenodenames.directed.clone());
    }
    

    217 => {
        let _ = graph.get_unknown_node_types_number();
    }
    

    218 => {
        let _ = graph.get_known_node_types_number();
    }
    

    219 => {
        let _ = graph.get_unknown_node_types_rate();
    }
    

    220 => {
        let _ = graph.get_known_node_types_rate();
    }
    

    221 => {
        let _ = graph.get_minimum_node_types_number();
    }
    

    222 => {
        let _ = graph.get_maximum_node_types_number();
    }
    

    223 => {
        let _ = graph.get_maximum_multilabel_count();
    }
    

    224 => {
        let _ = graph.get_singleton_node_types_number();
    }
    

    225 => {
        let _ = graph.get_singleton_node_type_ids();
    }
    

    226 => {
        let _ = graph.get_singleton_node_type_names();
    }
    

    227 => {
        let _ = graph.get_unknown_edge_types_number();
    }
    

    228 => {
        let _ = graph.get_edge_ids_with_unknown_edge_types();
    }
    

    229 => {
        let _ = graph.get_edge_ids_with_known_edge_types();
    }
    

    230 => {
        let _ = graph.get_edge_node_ids_with_unknown_edge_types(data.getedgenodeidswithunknownedgetypes.directed.clone());
    }
    

    231 => {
        let _ = graph.get_edge_node_ids_with_known_edge_types(data.getedgenodeidswithknownedgetypes.directed.clone());
    }
    

    232 => {
        let _ = graph.get_edge_node_names_with_unknown_edge_types(data.getedgenodenameswithunknownedgetypes.directed.clone());
    }
    

    233 => {
        let _ = graph.get_edge_node_names_with_known_edge_types(data.getedgenodenameswithknownedgetypes.directed.clone());
    }
    

    234 => {
        let _ = graph.get_edge_ids_with_unknown_edge_types_mask();
    }
    

    235 => {
        let _ = graph.get_edge_ids_with_known_edge_types_mask();
    }
    

    236 => {
        let _ = graph.get_node_ids_with_unknown_node_types();
    }
    

    237 => {
        let _ = graph.get_node_ids_with_known_node_types();
    }
    

    238 => {
        let _ = graph.get_node_names_with_unknown_node_types();
    }
    

    239 => {
        let _ = graph.get_node_names_with_known_node_types();
    }
    

    240 => {
        let _ = graph.get_node_ids_with_unknown_node_types_mask();
    }
    

    241 => {
        let _ = graph.get_node_ids_with_known_node_types_mask();
    }
    

    242 => {
        let _ = graph.get_known_edge_types_number();
    }
    

    243 => {
        let _ = graph.get_unknown_edge_types_rate();
    }
    

    244 => {
        let _ = graph.get_known_edge_types_rate();
    }
    

    245 => {
        let _ = graph.get_minimum_edge_types_number();
    }
    

    246 => {
        let _ = graph.get_singleton_edge_types_number();
    }
    

    247 => {
        let _ = graph.get_singleton_edge_type_ids();
    }
    

    248 => {
        let _ = graph.get_singleton_edge_type_names();
    }
    

    249 => {
        let _ = graph.get_nodes_number();
    }
    

    250 => {
        let _ = graph.get_node_connected_component_ids(data.getnodeconnectedcomponentids.verbose.clone());
    }
    

    251 => {
        let _ = graph.get_directed_edges_number();
    }
    

    252 => {
        let _ = graph.get_edge_types_number();
    }
    

    253 => {
        let _ = graph.get_node_types_number();
    }
    

    254 => {
        let _ = graph.get_node_degrees();
    }
    

    255 => {
        let _ = graph.get_weighted_node_degrees();
    }
    

    256 => {
        let _ = graph.get_not_singletons_node_ids();
    }
    

    257 => {
        let _ = graph.get_dense_nodes_mapping();
    }
    

    258 => {
        let _ = graph.get_parallel_edges_number();
    }
    

    259 => {
        let _ = graph.get_cumulative_node_degrees();
    }
    

    260 => {
        let _ = graph.get_unique_source_nodes_number();
    }
    

    261 => {
        let _ = graph.get_edge_type_id_counts_hashmap();
    }
    

    262 => {
        let _ = graph.get_edge_type_names_counts_hashmap();
    }
    

    263 => {
        let _ = graph.get_node_type_id_counts_hashmap();
    }
    

    264 => {
        let _ = graph.get_node_type_names_counts_hashmap();
    }
    

    265 => {
        let _ = graph.get_dense_binary_adjacency_matrix();
    }
    

    266 => {
        let _ = graph.get_dense_weighted_adjacency_matrix(data.getdenseweightedadjacencymatrix.weight.clone());
    }
    

    267 => {
        
            let _ = graph.iter_node_ids().collect::<Vec<_>>();
            
    }
    

    268 => {
        
            let _ = graph.par_iter_node_ids().collect::<Vec<_>>();
            
    }
    

    269 => {
        
            let _ = graph.iter_node_names().collect::<Vec<_>>();
            
    }
    

    270 => {
        
            let _ = graph.par_iter_node_names().collect::<Vec<_>>();
            
    }
    

    271 => {
        
            let _ = graph.iter_unique_node_type_ids().map(|x| x.collect::<Vec<_>>());
            
    }
    

    272 => {
        
            let _ = graph.iter_node_type_counts().map(|x| x.collect::<Vec<_>>());
            
    }
    

    273 => {
        
            let _ = graph.iter_unique_node_type_ids_and_counts().map(|x| x.collect::<Vec<_>>());
            
    }
    

    274 => {
        
            let _ = graph.iter_unique_node_type_names().map(|x| x.collect::<Vec<_>>());
            
    }
    

    275 => {
        
            let _ = graph.iter_unique_node_type_names_and_counts().map(|x| x.collect::<Vec<_>>());
            
    }
    

    276 => {
        
            let _ = graph.iter_unique_edge_type_ids().map(|x| x.collect::<Vec<_>>());
            
    }
    

    277 => {
        
            let _ = graph.iter_edge_type_counts().map(|x| x.collect::<Vec<_>>());
            
    }
    

    278 => {
        
            let _ = graph.iter_unique_edge_type_ids_and_counts().map(|x| x.collect::<Vec<_>>());
            
    }
    

    279 => {
        
            let _ = graph.iter_unique_edge_type_names_and_counts().map(|x| x.collect::<Vec<_>>());
            
    }
    

    280 => {
        
            let _ = graph.iter_unique_edge_type_names().map(|x| x.collect::<Vec<_>>());
            
    }
    

    281 => {
        
            let _ = graph.iter_node_degrees().collect::<Vec<_>>();
            
    }
    

    282 => {
        
            let _ = graph.par_iter_node_degrees().collect::<Vec<_>>();
            
    }
    

    283 => {
        
            let _ = graph.iter_weighted_node_degrees().map(|x| x.collect::<Vec<_>>());
            
    }
    

    284 => {
        
            let _ = graph.par_iter_weighted_node_degrees().map(|x| x.collect::<Vec<_>>());
            
    }
    

    285 => {
        
            let _ = graph.iter_connected_node_ids().collect::<Vec<_>>();
            
    }
    

    286 => {
        
            let _ = graph.iter_singleton_node_ids().collect::<Vec<_>>();
            
    }
    

    287 => {
        
            let _ = graph.iter_singleton_node_names().collect::<Vec<_>>();
            
    }
    

    288 => {
        
            let _ = graph.iter_singleton_nodes_with_selfloops_node_ids().collect::<Vec<_>>();
            
    }
    

    289 => {
        
            let _ = graph.iter_singleton_nodes_with_selfloops_node_names().collect::<Vec<_>>();
            
    }
    

    290 => {
        
            let _ = graph.iter_singleton_node_type_ids().map(|x| x.collect::<Vec<_>>());
            
    }
    

    291 => {
        
            let _ = graph.iter_singleton_edge_type_ids().map(|x| x.collect::<Vec<_>>());
            
    }
    

    292 => {
        
            let _ = graph.iter_singleton_node_type_names().map(|x| x.collect::<Vec<_>>());
            
    }
    

    293 => {
        
            let _ = graph.iter_singleton_edge_type_names().map(|x| x.collect::<Vec<_>>());
            
    }
    

    294 => {
        
            let _ = graph.iter_source_node_ids(data.itersourcenodeids.directed.clone()).collect::<Vec<_>>();
            
    }
    

    295 => {
        
            let _ = graph.iter_edge_weights().map(|x| x.collect::<Vec<_>>());
            
    }
    

    296 => {
        
            let _ = graph.par_iter_edge_weights().map(|x| x.collect::<Vec<_>>());
            
    }
    

    297 => {
        
            let _ = graph.par_iter_source_node_ids(data.paritersourcenodeids.directed.clone()).collect::<Vec<_>>();
            
    }
    

    298 => {
        
            let _ = graph.iter_destination_node_ids(data.iterdestinationnodeids.directed.clone()).collect::<Vec<_>>();
            
    }
    

    299 => {
        
            let _ = graph.par_iter_destination_node_ids(data.pariterdestinationnodeids.directed.clone()).collect::<Vec<_>>();
            
    }
    

    300 => {
        
            let _ = graph.iter_node_ids_and_node_type_ids().collect::<Vec<_>>();
            
    }
    

    301 => {
        
            let _ = graph.iter_one_hot_encoded_node_type_ids().map(|x| x.collect::<Vec<_>>());
            
    }
    

    302 => {
        
            let _ = graph.iter_one_hot_encoded_known_node_type_ids().map(|x| x.collect::<Vec<_>>());
            
    }
    

    303 => {
        
            let _ = graph.iter_node_names_and_node_type_names().collect::<Vec<_>>();
            
    }
    

    304 => {
        
            let _ = graph.par_iter_node_names_and_node_type_names().collect::<Vec<_>>();
            
    }
    

    305 => {
        
            let _ = graph.iter_edge_node_ids(data.iteredgenodeids.directed.clone()).collect::<Vec<_>>();
            
    }
    

    306 => {
        
            let _ = graph.iter_edges(data.iteredges.directed.clone()).collect::<Vec<_>>();
            
    }
    

    307 => {
        
            let _ = graph.par_iter_edge_node_ids(data.pariteredgenodeids.directed.clone()).collect::<Vec<_>>();
            
    }
    

    308 => {
        
            let _ = graph.par_iter_directed_edge_ids().collect::<Vec<_>>();
            
    }
    

    309 => {
        
            let _ = graph.par_iter_edges(data.pariteredges.directed.clone()).collect::<Vec<_>>();
            
    }
    

    310 => {
        
            let _ = graph.iter_edge_node_ids_and_edge_weight().map(|x| x.collect::<Vec<_>>());
            
    }
    

    311 => {
        
            let _ = graph.par_iter_edge_node_ids_and_edge_weight().map(|x| x.collect::<Vec<_>>());
            
    }
    

    312 => {
        
            let _ = graph.iter_edge_node_ids_and_edge_type_id(data.iteredgenodeidsandedgetypeid.directed.clone()).collect::<Vec<_>>();
            
    }
    

    313 => {
        
            let _ = graph.iter_one_hot_encoded_edge_type_ids().map(|x| x.collect::<Vec<_>>());
            
    }
    

    314 => {
        
            let _ = graph.iter_one_hot_encoded_known_edge_type_ids().map(|x| x.collect::<Vec<_>>());
            
    }
    

    315 => {
        let _ = graph.iter_edge_node_names_and_edge_type_name(data.iteredgenodenamesandedgetypename.directed.clone());
    }
    

    316 => {
        let _ = graph.par_iter_edge_node_names_and_edge_type_name(data.pariteredgenodenamesandedgetypename.directed.clone());
    }
    

    317 => {
        
            let _ = graph.par_iter_edge_node_ids_and_edge_type_id(data.pariteredgenodeidsandedgetypeid.directed.clone()).collect::<Vec<_>>();
            
    }
    

    318 => {
        let _ = graph.par_iter_edge_node_names_and_edge_type_name_and_edge_weight(data.pariteredgenodenamesandedgetypenameandedgeweight.directed.clone());
    }
    

    319 => {
        let _ = graph.iter_edge_node_names_and_edge_type_name_and_edge_weight(data.iteredgenodenamesandedgetypenameandedgeweight.directed.clone());
    }
    

    320 => {
        
            let _ = graph.par_iter_edge_node_ids_and_edge_type_id_and_edge_weight(data.pariteredgenodeidsandedgetypeidandedgeweight.directed.clone()).collect::<Vec<_>>();
            
    }
    

    321 => {
        
            let _ = graph.iter_edge_node_ids_and_edge_type_id_and_edge_weight(data.iteredgenodeidsandedgetypeidandedgeweight.directed.clone()).collect::<Vec<_>>();
            
    }
    

    322 => {
        
            let _ = graph.iter_unique_edge_node_ids(data.iteruniqueedgenodeids.directed.clone()).collect::<Vec<_>>();
            
    }
    

    323 => {
        
            let _ = graph.iter_unique_source_node_ids().collect::<Vec<_>>();
            
    }
    

    324 => {
        
            let _ = graph.iter_edge_ids_with_unknown_edge_types().map(|x| x.collect::<Vec<_>>());
            
    }
    

    325 => {
        
            let _ = graph.iter_edge_ids_with_known_edge_types().map(|x| x.collect::<Vec<_>>());
            
    }
    

    326 => {
        
            let _ = graph.iter_edge_node_ids_with_unknown_edge_types(data.iteredgenodeidswithunknownedgetypes.directed.clone()).map(|x| x.collect::<Vec<_>>());
            
    }
    

    327 => {
        
            let _ = graph.iter_edge_node_ids_with_known_edge_types(data.iteredgenodeidswithknownedgetypes.directed.clone()).map(|x| x.collect::<Vec<_>>());
            
    }
    

    328 => {
        
            let _ = graph.iter_node_ids_with_unknown_node_types().map(|x| x.collect::<Vec<_>>());
            
    }
    

    329 => {
        
            let _ = graph.iter_node_ids_with_known_node_types().map(|x| x.collect::<Vec<_>>());
            
    }
    

    330 => {
        
            let _ = graph.iter_edge_node_names_with_unknown_edge_types(data.iteredgenodenameswithunknownedgetypes.directed.clone()).map(|x| x.collect::<Vec<_>>());
            
    }
    

    331 => {
        
            let _ = graph.iter_edge_node_names_with_known_edge_types(data.iteredgenodenameswithknownedgetypes.directed.clone()).map(|x| x.collect::<Vec<_>>());
            
    }
    

    332 => {
        
            let _ = graph.iter_node_names_with_unknown_node_types().map(|x| x.collect::<Vec<_>>());
            
    }
    

    333 => {
        
            let _ = graph.iter_node_names_with_known_node_types().map(|x| x.collect::<Vec<_>>());
            
    }
    

    334 => {
        
            let _ = graph.par_iter_edge_ids_with_unknown_edge_types().map(|x| x.collect::<Vec<_>>());
            
    }
    

    335 => {
        
            let _ = graph.par_iter_edge_ids_with_known_edge_types().map(|x| x.collect::<Vec<_>>());
            
    }
    

    336 => {
        
            let _ = graph.par_iter_edge_node_ids_with_unknown_edge_types(data.pariteredgenodeidswithunknownedgetypes.directed.clone()).map(|x| x.collect::<Vec<_>>());
            
    }
    

    337 => {
        
            let _ = graph.par_iter_edge_node_ids_with_known_edge_types(data.pariteredgenodeidswithknownedgetypes.directed.clone()).map(|x| x.collect::<Vec<_>>());
            
    }
    

    338 => {
        
            let _ = graph.par_iter_node_ids_with_unknown_node_types().map(|x| x.collect::<Vec<_>>());
            
    }
    

    339 => {
        
            let _ = graph.par_iter_node_ids_with_known_node_types().map(|x| x.collect::<Vec<_>>());
            
    }
    

    340 => {
        
            let _ = graph.par_iter_edge_node_names_with_unknown_edge_types(data.pariteredgenodenameswithunknownedgetypes.directed.clone()).map(|x| x.collect::<Vec<_>>());
            
    }
    

    341 => {
        
            let _ = graph.par_iter_edge_node_names_with_known_edge_types(data.pariteredgenodenameswithknownedgetypes.directed.clone()).map(|x| x.collect::<Vec<_>>());
            
    }
    

    342 => {
        
            let _ = graph.par_iter_node_names_with_unknown_node_types().map(|x| x.collect::<Vec<_>>());
            
    }
    

    343 => {
        
            let _ = graph.par_iter_node_names_with_known_node_types().map(|x| x.collect::<Vec<_>>());
            
    }
    

    344 => {
        graph = graph.get_laplacian_transformed_graph(data.getlaplaciantransformedgraph.verbose.clone());
    }
    

    345 => {
        graph = graph.get_random_walk_normalized_laplacian_transformed_graph(data.getrandomwalknormalizedlaplaciantransformedgraph.verbose.clone());
    }
    

    346 => {
        
        if let Ok(res) = graph.get_symmetric_normalized_laplacian_transformed_graph(data.getsymmetricnormalizedlaplaciantransformedgraph.verbose.clone()) {
            graph = res;
        }
        
    }
    

    347 => {
        
        if let Ok(res) = graph.get_symmetric_normalized_transformed_graph(data.getsymmetricnormalizedtransformedgraph.verbose.clone()) {
            graph = res;
        }
        
    }
    

    348 => {
        
        if let Ok(res) = graph.get_weighted_laplacian_transformed_graph(data.getweightedlaplaciantransformedgraph.verbose.clone()) {
            graph = res;
        }
        
    }
    

    349 => {
        
        if let Ok(res) = graph.get_weighted_symmetric_normalized_laplacian_transformed_graph(data.getweightedsymmetricnormalizedlaplaciantransformedgraph.verbose.clone()) {
            graph = res;
        }
        
    }
    

    350 => {
        
        if let Ok(res) = graph.get_weighted_symmetric_normalized_transformed_graph(data.getweightedsymmetricnormalizedtransformedgraph.verbose.clone()) {
            graph = res;
        }
        
    }
    

    351 => {
        
        if let Ok(res) = graph.get_weighted_random_walk_normalized_laplacian_transformed_graph(data.getweightedrandomwalknormalizedlaplaciantransformedgraph.verbose.clone()) {
            graph = res;
        }
        
    }
    

    352 => {
        let _ = graph.has_default_graph_name();
    }
    

    353 => {
        let _ = graph.has_nodes();
    }
    

    354 => {
        let _ = graph.has_edges();
    }
    

    355 => {
        let _ = graph.has_trap_nodes();
    }
    

    356 => {
        let _ = graph.is_directed();
    }
    

    357 => {
        let _ = graph.has_edge_weights();
    }
    

    358 => {
        let _ = graph.has_edge_weights_representing_probabilities();
    }
    

    359 => {
        let _ = graph.has_weighted_singleton_nodes();
    }
    

    360 => {
        let _ = graph.has_constant_edge_weights();
    }
    

    361 => {
        let _ = graph.has_negative_edge_weights();
    }
    

    362 => {
        let _ = graph.has_edge_types();
    }
    

    363 => {
        let _ = graph.has_selfloops();
    }
    

    364 => {
        let _ = graph.has_disconnected_nodes();
    }
    

    365 => {
        let _ = graph.has_singleton_nodes();
    }
    

    366 => {
        let _ = graph.has_singleton_nodes_with_selfloops();
    }
    

    367 => {
        let _ = graph.is_connected(data.isconnected.verbose.clone());
    }
    

    368 => {
        let _ = graph.has_node_types();
    }
    

    369 => {
        let _ = graph.has_multilabel_node_types();
    }
    

    370 => {
        let _ = graph.has_unknown_node_types();
    }
    

    371 => {
        let _ = graph.has_known_node_types();
    }
    

    372 => {
        let _ = graph.has_unknown_edge_types();
    }
    

    373 => {
        let _ = graph.has_known_edge_types();
    }
    

    374 => {
        let _ = graph.has_homogeneous_node_types();
    }
    

    375 => {
        let _ = graph.has_homogeneous_edge_types();
    }
    

    376 => {
        let _ = graph.has_singleton_node_types();
    }
    

    377 => {
        let _ = graph.has_node_oddities();
    }
    

    378 => {
        let _ = graph.has_node_types_oddities();
    }
    

    379 => {
        let _ = graph.has_singleton_edge_types();
    }
    

    380 => {
        let _ = graph.has_edge_types_oddities();
    }
    

    381 => {
        let _ = graph.is_multigraph();
    }
    

    382 => {
        let _ = graph.has_nodes_sorted_by_decreasing_outbound_node_degree();
    }
    

    383 => {
        let _ = graph.has_nodes_sorted_by_increasing_outbound_node_degree();
    }
    

    384 => {
        let _ = graph.compute_hash();
    }
    

    385 => {
        let _ = graph.to_directed_inplace();
    }
    

    386 => {
        graph = graph.to_directed();
    }
    

    387 => {
        graph = graph.to_upper_triangular(data.touppertriangular.verbose.clone());
    }
    

    388 => {
        graph = graph.to_lower_triangular(data.tolowertriangular.verbose.clone());
    }
    

    389 => {
        graph = graph.to_main_diagonal(data.tomaindiagonal.verbose.clone());
    }
    

    390 => {
        graph = graph.to_anti_diagonal(data.toantidiagonal.verbose.clone());
    }
    

    391 => {
        graph = graph.to_bidiagonal(data.tobidiagonal.verbose.clone());
    }
    

    392 => {
        graph = graph.to_arrowhead(data.toarrowhead.verbose.clone());
    }
    

    393 => {
        graph = graph.to_transposed(data.totransposed.verbose.clone());
    }
    

    394 => {
        graph = graph.to_complementary(data.tocomplementary.verbose.clone());
    }
    

    395 => {
        
        if let Ok((res1, res2)) = graph.get_node_label_random_holdout(data.getnodelabelrandomholdout.train_size.clone(), data.getnodelabelrandomholdout.use_stratification.clone(), data.getnodelabelrandomholdout.random_state.clone()) {
            if rng.next() % 2 == 0 {
                graph = res1;
            } else {
                graph = res2;
            }
        }
        
    }
    

    396 => {
        
        if let Ok((res1, res2)) = graph.get_node_label_kfold(data.getnodelabelkfold.k.clone(), data.getnodelabelkfold.k_index.clone(), data.getnodelabelkfold.use_stratification.clone(), data.getnodelabelkfold.random_state.clone()) {
            if rng.next() % 2 == 0 {
                graph = res1;
            } else {
                graph = res2;
            }
        }
        
    }
    

    397 => {
        
        if let Ok((res1, res2)) = graph.get_edge_label_random_holdout(data.getedgelabelrandomholdout.train_size.clone(), data.getedgelabelrandomholdout.use_stratification.clone(), data.getedgelabelrandomholdout.random_state.clone()) {
            if rng.next() % 2 == 0 {
                graph = res1;
            } else {
                graph = res2;
            }
        }
        
    }
    

    398 => {
        
        if let Ok((res1, res2)) = graph.get_edge_label_kfold(data.getedgelabelkfold.k.clone(), data.getedgelabelkfold.k_index.clone(), data.getedgelabelkfold.use_stratification.clone(), data.getedgelabelkfold.random_state.clone()) {
            if rng.next() % 2 == 0 {
                graph = res1;
            } else {
                graph = res2;
            }
        }
        
    }
    

    399 => {
        
        if let Ok(res) = graph.get_random_subgraph(data.getrandomsubgraph.nodes_number.clone(), data.getrandomsubgraph.random_state.clone(), data.getrandomsubgraph.verbose.clone()) {
            graph = res;
        }
        
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
        match rng.next() % 400 {

    0 => {
        println!("spanning_arborescence_kruskal(verbose: {:?})", &data.spanningarborescencekruskal.verbose);
    }
    

    1 => {
        println!("spanning_arborescence(verbose: {:?})", &data.spanningarborescence.verbose);
    }
    

    2 => {
        println!("connected_components(verbose: {:?})", &data.connectedcomponents.verbose);
    }
    

    3 => {
        println!("get_node_label_prediction_mini_batch(idx: {:?}, batch_size: {:?}, include_central_node: {:?}, return_edge_weights: {:?}, max_neighbours: {:?})", &data.getnodelabelpredictionminibatch.idx, &data.getnodelabelpredictionminibatch.batch_size, &data.getnodelabelpredictionminibatch.include_central_node, &data.getnodelabelpredictionminibatch.return_edge_weights, &data.getnodelabelpredictionminibatch.max_neighbours);
    }
    

    4 => {
        println!("par_iter_edge_prediction_metrics(normalize: {:?}, verbose: {:?})", &data.pariteredgepredictionmetrics.normalize, &data.pariteredgepredictionmetrics.verbose);
    }
    

    5 => {
        println!("get_okapi_bm25_node_feature_propagation(features: {:?}, iterations: {:?}, maximal_distance: {:?}, k1: {:?}, b: {:?}, include_central_node: {:?}, verbose: {:?})", &data.getokapibm25nodefeaturepropagation.features, &data.getokapibm25nodefeaturepropagation.iterations.map(|x| x as usize), &data.getokapibm25nodefeaturepropagation.maximal_distance, &data.getokapibm25nodefeaturepropagation.k1, &data.getokapibm25nodefeaturepropagation.b, &data.getokapibm25nodefeaturepropagation.include_central_node, &data.getokapibm25nodefeaturepropagation.verbose);
    }
    

    6 => {
        println!("get_okapi_bm25_node_label_propagation(iterations: {:?}, maximal_distance: {:?}, k1: {:?}, b: {:?}, verbose: {:?})", &data.getokapibm25nodelabelpropagation.iterations.map(|x| x as usize), &data.getokapibm25nodelabelpropagation.maximal_distance, &data.getokapibm25nodelabelpropagation.k1, &data.getokapibm25nodelabelpropagation.b, &data.getokapibm25nodelabelpropagation.verbose);
    }
    

    7 => {
        println!("sort_by_increasing_outbound_node_degree(verbose: {:?})", &data.sortbyincreasingoutboundnodedegree.verbose);
    }
    

    8 => {
        println!("sort_by_decreasing_outbound_node_degree(verbose: {:?})", &data.sortbydecreasingoutboundnodedegree.verbose);
    }
    

    9 => {
        println!("to_dot(use_node_names: {:?})", &data.todot.use_node_names);
    }
    

    10 => {
        println!("remap_from_node_ids(node_ids: {:?}, verbose: {:?})", &data.remapfromnodeids.node_ids, &data.remapfromnodeids.verbose);
    }
    

    11 => {
        println!("is_singleton_from_node_id(node_id: {:?})", &data.issingletonfromnodeid.node_id);
    }
    

    12 => {
        println!("is_singleton_with_selfloops_from_node_id(node_id: {:?})", &data.issingletonwithselfloopsfromnodeid.node_id);
    }
    

    13 => {
        println!("has_node_type_id(node_type_id: {:?})", &data.hasnodetypeid.node_type_id);
    }
    

    14 => {
        println!("has_edge_type_id(edge_type_id: {:?})", &data.hasedgetypeid.edge_type_id);
    }
    

    15 => {
        println!("has_edge_from_node_ids(src: {:?}, dst: {:?})", &data.hasedgefromnodeids.src, &data.hasedgefromnodeids.dst);
    }
    

    16 => {
        println!("has_selfloop_from_node_id(node_id: {:?})", &data.hasselfloopfromnodeid.node_id);
    }
    

    17 => {
        println!("has_edge_from_node_ids_and_edge_type_id(src: {:?}, dst: {:?}, edge_type: {:?})", &data.hasedgefromnodeidsandedgetypeid.src, &data.hasedgefromnodeidsandedgetypeid.dst, &data.hasedgefromnodeidsandedgetypeid.edge_type);
    }
    

    18 => {
        println!("is_trap_node_from_node_id(node_id: {:?})", &data.istrapnodefromnodeid.node_id);
    }
    

    19 => {
        println!("get_preferential_attachment_from_node_ids(source_node_id: {:?}, destination_node_id: {:?}, normalize: {:?})", &data.getpreferentialattachmentfromnodeids.source_node_id, &data.getpreferentialattachmentfromnodeids.destination_node_id, &data.getpreferentialattachmentfromnodeids.normalize);
    }
    

    20 => {
        println!("get_weighted_preferential_attachment_from_node_ids(source_node_id: {:?}, destination_node_id: {:?}, normalize: {:?})", &data.getweightedpreferentialattachmentfromnodeids.source_node_id, &data.getweightedpreferentialattachmentfromnodeids.destination_node_id, &data.getweightedpreferentialattachmentfromnodeids.normalize);
    }
    

    21 => {
        println!("get_jaccard_coefficient_from_node_ids(source_node_id: {:?}, destination_node_id: {:?})", &data.getjaccardcoefficientfromnodeids.source_node_id, &data.getjaccardcoefficientfromnodeids.destination_node_id);
    }
    

    22 => {
        println!("get_adamic_adar_index_from_node_ids(source_node_id: {:?}, destination_node_id: {:?})", &data.getadamicadarindexfromnodeids.source_node_id, &data.getadamicadarindexfromnodeids.destination_node_id);
    }
    

    23 => {
        println!("get_resource_allocation_index_from_node_ids(source_node_id: {:?}, destination_node_id: {:?})", &data.getresourceallocationindexfromnodeids.source_node_id, &data.getresourceallocationindexfromnodeids.destination_node_id);
    }
    

    24 => {
        println!("get_weighted_resource_allocation_index_from_node_ids(source_node_id: {:?}, destination_node_id: {:?})", &data.getweightedresourceallocationindexfromnodeids.source_node_id, &data.getweightedresourceallocationindexfromnodeids.destination_node_id);
    }
    

    25 => {
        println!("enable(vector_sources: {:?}, vector_destinations: {:?}, vector_cumulative_node_degrees: {:?})", &data.enable.vector_sources, &data.enable.vector_destinations, &data.enable.vector_cumulative_node_degrees);
    }
    

    26 => {
        println!("disable_all()", );
    }
    

    27 => {
        println!("get_number_of_triangles(normalize: {:?}, low_centrality: {:?}, verbose: {:?})", &data.getnumberoftriangles.normalize, &data.getnumberoftriangles.low_centrality, &data.getnumberoftriangles.verbose);
    }
    

    28 => {
        println!("get_triads_number()", );
    }
    

    29 => {
        println!("get_weighted_triads_number()", );
    }
    

    30 => {
        println!("get_transitivity(low_centrality: {:?}, verbose: {:?})", &data.gettransitivity.low_centrality, &data.gettransitivity.verbose);
    }
    

    31 => {
        println!("get_number_of_triangles_per_node(normalize: {:?}, low_centrality: {:?}, verbose: {:?})", &data.getnumberoftrianglespernode.normalize, &data.getnumberoftrianglespernode.low_centrality, &data.getnumberoftrianglespernode.verbose);
    }
    

    32 => {
        println!("iter_clustering_coefficient_per_node(low_centrality: {:?}, verbose: {:?})", &data.iterclusteringcoefficientpernode.low_centrality, &data.iterclusteringcoefficientpernode.verbose);
    }
    

    33 => {
        println!("get_clustering_coefficient_per_node(low_centrality: {:?}, verbose: {:?})", &data.getclusteringcoefficientpernode.low_centrality, &data.getclusteringcoefficientpernode.verbose);
    }
    

    34 => {
        println!("get_clustering_coefficient(low_centrality: {:?}, verbose: {:?})", &data.getclusteringcoefficient.low_centrality, &data.getclusteringcoefficient.verbose);
    }
    

    35 => {
        println!("get_average_clustering_coefficient(low_centrality: {:?}, verbose: {:?})", &data.getaverageclusteringcoefficient.low_centrality, &data.getaverageclusteringcoefficient.verbose);
    }
    

    36 => {
        println!("get_minimum_path_node_ids_from_node_ids(src_node_id: {:?}, dst_node_id: {:?}, maximal_depth: {:?})", &data.getminimumpathnodeidsfromnodeids.src_node_id, &data.getminimumpathnodeidsfromnodeids.dst_node_id, &data.getminimumpathnodeidsfromnodeids.maximal_depth);
    }
    

    37 => {
        println!("get_k_shortest_path_node_ids_from_node_ids(src_node_id: {:?}, dst_node_id: {:?}, k: {:?}, max_path_length: {:?}, verbose: {:?})", &data.getkshortestpathnodeidsfromnodeids.src_node_id, &data.getkshortestpathnodeidsfromnodeids.dst_node_id, &(data.getkshortestpathnodeidsfromnodeids.k as usize), &data.getkshortestpathnodeidsfromnodeids.max_path_length, &data.getkshortestpathnodeidsfromnodeids.verbose);
    }
    

    38 => {
        println!("get_eccentricity_from_node_id(node_id: {:?})", &data.geteccentricityfromnodeid.node_id);
    }
    

    39 => {
        println!("get_weighted_eccentricity_from_node_id(node_id: {:?}, use_edge_weights_as_probabilities: {:?})", &data.getweightedeccentricityfromnodeid.node_id, &data.getweightedeccentricityfromnodeid.use_edge_weights_as_probabilities);
    }
    

    40 => {
        println!("get_weighted_minimum_path_node_ids_from_node_ids(src_node_id: {:?}, dst_node_id: {:?}, use_edge_weights_as_probabilities: {:?}, maximal_depth: {:?})", &data.getweightedminimumpathnodeidsfromnodeids.src_node_id, &data.getweightedminimumpathnodeidsfromnodeids.dst_node_id, &data.getweightedminimumpathnodeidsfromnodeids.use_edge_weights_as_probabilities, &data.getweightedminimumpathnodeidsfromnodeids.maximal_depth);
    }
    

    41 => {
        println!("get_breath_first_search_from_node_ids(src_node_id: {:?}, dst_node_id: {:?}, compute_predecessors: {:?}, maximal_depth: {:?})", &data.getbreathfirstsearchfromnodeids.src_node_id, &data.getbreathfirstsearchfromnodeids.dst_node_id, &data.getbreathfirstsearchfromnodeids.compute_predecessors, &data.getbreathfirstsearchfromnodeids.maximal_depth);
    }
    

    42 => {
        println!("get_dijkstra_from_node_ids(src_node_id: {:?}, maybe_dst_node_id: {:?}, maybe_dst_node_ids: {:?}, compute_predecessors: {:?}, maximal_depth: {:?}, use_edge_weights_as_probabilities: {:?})", &data.getdijkstrafromnodeids.src_node_id, &data.getdijkstrafromnodeids.maybe_dst_node_id, &data.getdijkstrafromnodeids.maybe_dst_node_ids, &data.getdijkstrafromnodeids.compute_predecessors, &data.getdijkstrafromnodeids.maximal_depth, &data.getdijkstrafromnodeids.use_edge_weights_as_probabilities);
    }
    

    43 => {
        println!("get_diameter_naive(ignore_infinity: {:?}, verbose: {:?})", &data.getdiameternaive.ignore_infinity, &data.getdiameternaive.verbose);
    }
    

    44 => {
        println!("get_diameter(ignore_infinity: {:?}, verbose: {:?})", &data.getdiameter.ignore_infinity, &data.getdiameter.verbose);
    }
    

    45 => {
        println!("get_weighted_diameter_naive(ignore_infinity: {:?}, use_edge_weights_as_probabilities: {:?}, verbose: {:?})", &data.getweighteddiameternaive.ignore_infinity, &data.getweighteddiameternaive.use_edge_weights_as_probabilities, &data.getweighteddiameternaive.verbose);
    }
    

    46 => {
        println!("iter_edge_ids_from_node_ids(src: {:?}, dst: {:?})", &data.iteredgeidsfromnodeids.src, &data.iteredgeidsfromnodeids.dst);
    }
    

    47 => {
        println!("iter_edge_node_ids_and_edge_type_id_from_edge_type_id(edge_type_id: {:?}, directed: {:?})", &data.iteredgenodeidsandedgetypeidfromedgetypeid.edge_type_id, &data.iteredgenodeidsandedgetypeidfromedgetypeid.directed);
    }
    

    48 => {
        println!("iter_node_ids_and_node_type_ids_from_node_type_id(node_type_id: {:?})", &data.iternodeidsandnodetypeidsfromnodetypeid.node_type_id);
    }
    

    49 => {
        println!("iter_node_names_and_node_type_names_from_node_type_id(node_type_id: {:?})", &data.iternodenamesandnodetypenamesfromnodetypeid.node_type_id);
    }
    

    50 => {
        println!("iter_edge_node_names_and_edge_type_name_from_edge_type_id(edge_type_id: {:?}, directed: {:?})", &data.iteredgenodenamesandedgetypenamefromedgetypeid.edge_type_id, &data.iteredgenodenamesandedgetypenamefromedgetypeid.directed);
    }
    

    51 => {
        println!("get_transitive_closure(iterations: {:?}, verbose: {:?})", &data.gettransitiveclosure.iterations.map(|x| x as NodeT), &data.gettransitiveclosure.verbose);
    }
    

    52 => {
        println!("get_all_shortest_paths(iterations: {:?}, verbose: {:?})", &data.getallshortestpaths.iterations.map(|x| x as NodeT), &data.getallshortestpaths.verbose);
    }
    

    53 => {
        println!("get_weighted_all_shortest_paths(iterations: {:?}, use_edge_weights_as_probabilities: {:?}, verbose: {:?})", &data.getweightedallshortestpaths.iterations.map(|x| x as NodeT), &data.getweightedallshortestpaths.use_edge_weights_as_probabilities, &data.getweightedallshortestpaths.verbose);
    }
    

    54 => {
        println!("strongly_connected_components()", );
    }
    

    55 => {
        println!("get_source_node_id_from_edge_id(edge_id: {:?})", &data.getsourcenodeidfromedgeid.edge_id);
    }
    

    56 => {
        println!("get_destination_node_id_from_edge_id(edge_id: {:?})", &data.getdestinationnodeidfromedgeid.edge_id);
    }
    

    57 => {
        println!("get_source_node_name_from_edge_id(edge_id: {:?})", &data.getsourcenodenamefromedgeid.edge_id);
    }
    

    58 => {
        println!("get_destination_node_name_from_edge_id(edge_id: {:?})", &data.getdestinationnodenamefromedgeid.edge_id);
    }
    

    59 => {
        println!("get_node_names_from_edge_id(edge_id: {:?})", &data.getnodenamesfromedgeid.edge_id);
    }
    

    60 => {
        println!("get_node_ids_from_edge_id(edge_id: {:?})", &data.getnodeidsfromedgeid.edge_id);
    }
    

    61 => {
        println!("get_edge_id_from_node_ids(src: {:?}, dst: {:?})", &data.getedgeidfromnodeids.src, &data.getedgeidfromnodeids.dst);
    }
    

    62 => {
        println!("get_node_ids_and_edge_type_id_from_edge_id(edge_id: {:?})", &data.getnodeidsandedgetypeidfromedgeid.edge_id);
    }
    

    63 => {
        println!("get_node_ids_and_edge_type_id_and_edge_weight_from_edge_id(edge_id: {:?})", &data.getnodeidsandedgetypeidandedgeweightfromedgeid.edge_id);
    }
    

    64 => {
        println!("get_top_k_central_node_ids(k: {:?})", &data.gettopkcentralnodeids.k);
    }
    

    65 => {
        println!("get_weighted_top_k_central_node_ids(k: {:?})", &data.getweightedtopkcentralnodeids.k);
    }
    

    66 => {
        println!("get_node_degree_from_node_id(node_id: {:?})", &data.getnodedegreefromnodeid.node_id);
    }
    

    67 => {
        println!("get_weighted_node_degree_from_node_id(node_id: {:?})", &data.getweightednodedegreefromnodeid.node_id);
    }
    

    68 => {
        println!("get_top_k_central_node_names(k: {:?})", &data.gettopkcentralnodenames.k);
    }
    

    69 => {
        println!("get_node_type_id_from_node_id(node_id: {:?})", &data.getnodetypeidfromnodeid.node_id);
    }
    

    70 => {
        println!("get_edge_type_id_from_edge_id(edge_id: {:?})", &data.getedgetypeidfromedgeid.edge_id);
    }
    

    71 => {
        println!("get_node_type_names_from_node_id(node_id: {:?})", &data.getnodetypenamesfromnodeid.node_id);
    }
    

    72 => {
        println!("get_edge_type_name_from_edge_id(edge_id: {:?})", &data.getedgetypenamefromedgeid.edge_id);
    }
    

    73 => {
        println!("get_edge_type_name_from_edge_type_id(edge_type_id: {:?})", &data.getedgetypenamefromedgetypeid.edge_type_id);
    }
    

    74 => {
        println!("get_edge_weight_from_edge_id(edge_id: {:?})", &data.getedgeweightfromedgeid.edge_id);
    }
    

    75 => {
        println!("get_edge_weight_from_node_ids(src: {:?}, dst: {:?})", &data.getedgeweightfromnodeids.src, &data.getedgeweightfromnodeids.dst);
    }
    

    76 => {
        println!("get_edge_weight_from_node_ids_and_edge_type_id(src: {:?}, dst: {:?}, edge_type: {:?})", &data.getedgeweightfromnodeidsandedgetypeid.src, &data.getedgeweightfromnodeidsandedgetypeid.dst, &data.getedgeweightfromnodeidsandedgetypeid.edge_type);
    }
    

    77 => {
        println!("get_node_name_from_node_id(node_id: {:?})", &data.getnodenamefromnodeid.node_id);
    }
    

    78 => {
        println!("get_edge_node_names_from_edge_node_ids(edge_node_ids: {:?})", &data.getedgenodenamesfromedgenodeids.edge_node_ids);
    }
    

    79 => {
        println!("get_edge_count_from_edge_type_id(edge_type_id: {:?})", &data.getedgecountfromedgetypeid.edge_type_id);
    }
    

    80 => {
        println!("get_node_count_from_node_type_id(node_type_id: {:?})", &data.getnodecountfromnodetypeid.node_type_id);
    }
    

    81 => {
        println!("get_neighbour_node_ids_from_node_id(node_id: {:?})", &data.getneighbournodeidsfromnodeid.node_id);
    }
    

    82 => {
        println!("get_minmax_edge_ids_from_node_ids(src: {:?}, dst: {:?})", &data.getminmaxedgeidsfromnodeids.src, &data.getminmaxedgeidsfromnodeids.dst);
    }
    

    83 => {
        println!("get_edge_id_from_node_ids_and_edge_type_id(src: {:?}, dst: {:?}, edge_type: {:?})", &data.getedgeidfromnodeidsandedgetypeid.src, &data.getedgeidfromnodeidsandedgetypeid.dst, &data.getedgeidfromnodeidsandedgetypeid.edge_type);
    }
    

    84 => {
        println!("get_minmax_edge_ids_from_source_node_id(src: {:?})", &data.getminmaxedgeidsfromsourcenodeid.src);
    }
    

    85 => {
        println!("get_node_type_name_from_node_type_id(node_type_id: {:?})", &data.getnodetypenamefromnodetypeid.node_type_id);
    }
    

    86 => {
        println!("filter_from_ids(node_ids_to_keep: {:?}, node_ids_to_filter: {:?}, node_type_ids_to_keep: {:?}, node_type_ids_to_filter: {:?}, node_type_id_to_keep: {:?}, node_type_id_to_filter: {:?}, edge_ids_to_keep: {:?}, edge_ids_to_filter: {:?}, edge_node_ids_to_keep: {:?}, edge_node_ids_to_filter: {:?}, edge_type_ids_to_keep: {:?}, edge_type_ids_to_filter: {:?}, min_edge_weight: {:?}, max_edge_weight: {:?}, filter_singleton_nodes: {:?}, filter_singleton_nodes_with_selfloop: {:?}, filter_selfloops: {:?}, filter_parallel_edges: {:?}, verbose: {:?})", &data.filterfromids.node_ids_to_keep, &data.filterfromids.node_ids_to_filter, &data.filterfromids.node_type_ids_to_keep, &data.filterfromids.node_type_ids_to_filter, &data.filterfromids.node_type_id_to_keep, &data.filterfromids.node_type_id_to_filter, &data.filterfromids.edge_ids_to_keep, &data.filterfromids.edge_ids_to_filter, &data.filterfromids.edge_node_ids_to_keep, &data.filterfromids.edge_node_ids_to_filter, &data.filterfromids.edge_type_ids_to_keep, &data.filterfromids.edge_type_ids_to_filter, &data.filterfromids.min_edge_weight, &data.filterfromids.max_edge_weight, &data.filterfromids.filter_singleton_nodes, &data.filterfromids.filter_singleton_nodes_with_selfloop, &data.filterfromids.filter_selfloops, &data.filterfromids.filter_parallel_edges, &data.filterfromids.verbose);
    }
    

    87 => {
        println!("drop_unknown_node_types(verbose: {:?})", &data.dropunknownnodetypes.verbose);
    }
    

    88 => {
        println!("drop_unknown_edge_types(verbose: {:?})", &data.dropunknownedgetypes.verbose);
    }
    

    89 => {
        println!("drop_singleton_nodes(verbose: {:?})", &data.dropsingletonnodes.verbose);
    }
    

    90 => {
        println!("drop_singleton_nodes_with_selfloops(verbose: {:?})", &data.dropsingletonnodeswithselfloops.verbose);
    }
    

    91 => {
        println!("drop_disconnected_nodes(verbose: {:?})", &data.dropdisconnectednodes.verbose);
    }
    

    92 => {
        println!("drop_selfloops(verbose: {:?})", &data.dropselfloops.verbose);
    }
    

    93 => {
        println!("drop_parallel_edges(verbose: {:?})", &data.dropparalleledges.verbose);
    }
    

    94 => {
        println!("validate_node_id(node_id: {:?})", &data.validatenodeid.node_id);
    }
    

    95 => {
        println!("validate_node_ids(node_ids: {:?})", &data.validatenodeids.node_ids);
    }
    

    96 => {
        println!("validate_edge_id(edge_id: {:?})", &data.validateedgeid.edge_id);
    }
    

    97 => {
        println!("validate_edge_ids(edge_ids: {:?})", &data.validateedgeids.edge_ids);
    }
    

    98 => {
        println!("must_not_contain_unknown_node_types()", );
    }
    

    99 => {
        println!("must_not_contain_unknown_edge_types()", );
    }
    

    100 => {
        println!("validate_node_type_id(node_type_id: {:?})", &data.validatenodetypeid.node_type_id);
    }
    

    101 => {
        println!("validate_node_type_ids(node_type_ids: {:?})", &data.validatenodetypeids.node_type_ids);
    }
    

    102 => {
        println!("validate_edge_type_id(edge_type_id: {:?})", &data.validateedgetypeid.edge_type_id);
    }
    

    103 => {
        println!("validate_edge_type_ids(edge_type_ids: {:?})", &data.validateedgetypeids.edge_type_ids);
    }
    

    104 => {
        println!("must_have_node_types()", );
    }
    

    105 => {
        println!("must_have_edge_types()", );
    }
    

    106 => {
        println!("must_be_undirected()", );
    }
    

    107 => {
        println!("must_be_multigraph()", );
    }
    

    108 => {
        println!("must_not_be_multigraph()", );
    }
    

    109 => {
        println!("must_have_edge_weights()", );
    }
    

    110 => {
        println!("must_have_known_node_types()", );
    }
    

    111 => {
        println!("must_have_unknown_node_types()", );
    }
    

    112 => {
        println!("must_have_known_edge_types()", );
    }
    

    113 => {
        println!("must_have_unknown_edge_types()", );
    }
    

    114 => {
        println!("must_have_edge_weights_representing_probabilities()", );
    }
    

    115 => {
        println!("must_have_positive_edge_weights()", );
    }
    

    116 => {
        println!("must_not_contain_weighted_singleton_nodes()", );
    }
    

    117 => {
        println!("must_have_edges()", );
    }
    

    118 => {
        println!("must_have_nodes()", );
    }
    

    119 => {
        println!("iter_degree_centrality()", );
    }
    

    120 => {
        println!("par_iter_weighted_degree_centrality()", );
    }
    

    121 => {
        println!("get_degree_centrality()", );
    }
    

    122 => {
        println!("get_weighted_degree_centrality()", );
    }
    

    123 => {
        println!("par_iter_closeness_centrality(verbose: {:?})", &data.pariterclosenesscentrality.verbose);
    }
    

    124 => {
        println!("par_iter_weighted_closeness_centrality(use_edge_weights_as_probabilities: {:?}, verbose: {:?})", &data.pariterweightedclosenesscentrality.use_edge_weights_as_probabilities, &data.pariterweightedclosenesscentrality.verbose);
    }
    

    125 => {
        println!("get_closeness_centrality(verbose: {:?})", &data.getclosenesscentrality.verbose);
    }
    

    126 => {
        println!("get_weighted_closeness_centrality(use_edge_weights_as_probabilities: {:?}, verbose: {:?})", &data.getweightedclosenesscentrality.use_edge_weights_as_probabilities, &data.getweightedclosenesscentrality.verbose);
    }
    

    127 => {
        println!("par_iter_harmonic_centrality(verbose: {:?})", &data.pariterharmoniccentrality.verbose);
    }
    

    128 => {
        println!("par_iter_weighted_harmonic_centrality(use_edge_weights_as_probabilities: {:?}, verbose: {:?})", &data.pariterweightedharmoniccentrality.use_edge_weights_as_probabilities, &data.pariterweightedharmoniccentrality.verbose);
    }
    

    129 => {
        println!("get_harmonic_centrality(verbose: {:?})", &data.getharmoniccentrality.verbose);
    }
    

    130 => {
        println!("get_weighted_harmonic_centrality(use_edge_weights_as_probabilities: {:?}, verbose: {:?})", &data.getweightedharmoniccentrality.use_edge_weights_as_probabilities, &data.getweightedharmoniccentrality.verbose);
    }
    

    131 => {
        println!("get_stress_centrality(normalize: {:?}, verbose: {:?})", &data.getstresscentrality.normalize, &data.getstresscentrality.verbose);
    }
    

    132 => {
        println!("get_betweenness_centrality(normalize: {:?}, verbose: {:?})", &data.getbetweennesscentrality.normalize, &data.getbetweennesscentrality.verbose);
    }
    

    133 => {
        println!("get_eigenvector_centrality(maximum_iterations_number: {:?}, tollerance: {:?})", &data.geteigenvectorcentrality.maximum_iterations_number.map(|x| x as usize), &data.geteigenvectorcentrality.tollerance);
    }
    

    134 => {
        println!("get_weighted_eigenvector_centrality(maximum_iterations_number: {:?}, tollerance: {:?})", &data.getweightedeigenvectorcentrality.maximum_iterations_number.map(|x| x as usize), &data.getweightedeigenvectorcentrality.tollerance);
    }
    

    135 => {
        println!("remove_inplace_node_type_ids(node_type_ids_to_remove: {:?})", &data.removeinplacenodetypeids.node_type_ids_to_remove);
    }
    

    136 => {
        println!("remove_inplace_singleton_node_types()", );
    }
    

    137 => {
        println!("remove_inplace_edge_type_ids(edge_type_ids_to_remove: {:?})", &data.removeinplaceedgetypeids.edge_type_ids_to_remove);
    }
    

    138 => {
        println!("remove_inplace_singleton_edge_types()", );
    }
    

    139 => {
        println!("remove_node_type_id(node_type_id: {:?})", &data.removenodetypeid.node_type_id);
    }
    

    140 => {
        println!("remove_singleton_node_types()", );
    }
    

    141 => {
        println!("remove_edge_type_id(edge_type_id: {:?})", &data.removeedgetypeid.edge_type_id);
    }
    

    142 => {
        println!("remove_singleton_edge_types()", );
    }
    

    143 => {
        println!("remove_inplace_node_types()", );
    }
    

    144 => {
        println!("remove_node_types()", );
    }
    

    145 => {
        println!("remove_inplace_edge_types()", );
    }
    

    146 => {
        println!("remove_edge_types(verbose: {:?})", &data.removeedgetypes.verbose);
    }
    

    147 => {
        println!("remove_inplace_edge_weights()", );
    }
    

    148 => {
        println!("remove_edge_weights()", );
    }
    

    149 => {
        println!("encode_edge(src: {:?}, dst: {:?})", &data.encodeedge.src, &data.encodeedge.dst);
    }
    

    150 => {
        println!("decode_edge(edge: {:?})", &data.decodeedge.edge);
    }
    

    151 => {
        println!("get_max_encodable_edge_number()", );
    }
    

    152 => {
        println!("par_iter_approximated_vertex_cover()", );
    }
    

    153 => {
        println!("approximated_vertex_cover_set()", );
    }
    

    154 => {
        println!("report()", );
    }
    

    155 => {
        println!("get_node_report_from_node_id(node_id: {:?})", &data.getnodereportfromnodeid.node_id);
    }
    

    156 => {
        println!("textual_report()", );
    }
    

    157 => {
        println!("get_connected_components_number(verbose: {:?})", &data.getconnectedcomponentsnumber.verbose);
    }
    

    158 => {
        println!("get_singleton_nodes_number()", );
    }
    

    159 => {
        println!("get_weighted_singleton_nodes_number()", );
    }
    

    160 => {
        println!("get_disconnected_nodes_number()", );
    }
    

    161 => {
        println!("get_singleton_node_ids()", );
    }
    

    162 => {
        println!("get_singleton_node_names()", );
    }
    

    163 => {
        println!("get_singleton_nodes_with_selfloops_number()", );
    }
    

    164 => {
        println!("get_singleton_with_selfloops_node_ids()", );
    }
    

    165 => {
        println!("get_singleton_with_selfloops_node_names()", );
    }
    

    166 => {
        println!("get_connected_nodes_number()", );
    }
    

    167 => {
        println!("get_density()", );
    }
    

    168 => {
        println!("get_trap_nodes_rate()", );
    }
    

    169 => {
        println!("get_node_degrees_mean()", );
    }
    

    170 => {
        println!("get_weighted_node_degrees_mean()", );
    }
    

    171 => {
        println!("get_undirected_edges_number()", );
    }
    

    172 => {
        println!("get_unique_undirected_edges_number()", );
    }
    

    173 => {
        println!("get_edges_number()", );
    }
    

    174 => {
        println!("get_unique_edges_number()", );
    }
    

    175 => {
        println!("get_node_degrees_median()", );
    }
    

    176 => {
        println!("get_weighted_node_degrees_median()", );
    }
    

    177 => {
        println!("get_weighted_maximum_node_degree()", );
    }
    

    178 => {
        println!("get_weighted_minimum_node_degree()", );
    }
    

    179 => {
        println!("get_maximum_node_degree()", );
    }
    

    180 => {
        println!("get_most_central_node_id()", );
    }
    

    181 => {
        println!("get_weighted_mininum_node_degree()", );
    }
    

    182 => {
        println!("get_minimum_node_degree()", );
    }
    

    183 => {
        println!("get_node_degrees_mode()", );
    }
    

    184 => {
        println!("get_selfloop_number()", );
    }
    

    185 => {
        println!("get_unique_selfloop_number()", );
    }
    

    186 => {
        println!("get_selfloop_nodes_rate()", );
    }
    

    187 => {
        println!("get_name()", );
    }
    

    188 => {
        println!("get_trap_nodes_number()", );
    }
    

    189 => {
        println!("get_source_node_ids(directed: {:?})", &data.getsourcenodeids.directed);
    }
    

    190 => {
        println!("get_source_names(directed: {:?})", &data.getsourcenames.directed);
    }
    

    191 => {
        println!("get_destination_node_ids(directed: {:?})", &data.getdestinationnodeids.directed);
    }
    

    192 => {
        println!("get_destination_names(directed: {:?})", &data.getdestinationnames.directed);
    }
    

    193 => {
        println!("get_node_names()", );
    }
    

    194 => {
        println!("get_node_ids()", );
    }
    

    195 => {
        println!("get_edge_type_ids()", );
    }
    

    196 => {
        println!("get_unique_edge_type_ids()", );
    }
    

    197 => {
        println!("get_edge_type_names()", );
    }
    

    198 => {
        println!("get_unique_edge_type_names()", );
    }
    

    199 => {
        println!("get_edge_weights()", );
    }
    

    200 => {
        println!("get_total_edge_weights()", );
    }
    

    201 => {
        println!("get_mininum_edge_weight()", );
    }
    

    202 => {
        println!("get_maximum_edge_weight()", );
    }
    

    203 => {
        println!("get_node_type_ids()", );
    }
    

    204 => {
        println!("get_one_hot_encoded_node_types()", );
    }
    

    205 => {
        println!("get_known_node_types_mask()", );
    }
    

    206 => {
        println!("get_unknown_node_types_mask()", );
    }
    

    207 => {
        println!("get_one_hot_encoded_known_node_types()", );
    }
    

    208 => {
        println!("get_one_hot_encoded_edge_types()", );
    }
    

    209 => {
        println!("get_one_hot_encoded_known_edge_types()", );
    }
    

    210 => {
        println!("get_node_type_names()", );
    }
    

    211 => {
        println!("get_unique_node_type_ids()", );
    }
    

    212 => {
        println!("get_unique_node_type_names()", );
    }
    

    213 => {
        println!("get_unique_directed_edges_number()", );
    }
    

    214 => {
        println!("get_nodes_mapping()", );
    }
    

    215 => {
        println!("get_edge_node_ids(directed: {:?})", &data.getedgenodeids.directed);
    }
    

    216 => {
        println!("get_edge_node_names(directed: {:?})", &data.getedgenodenames.directed);
    }
    

    217 => {
        println!("get_unknown_node_types_number()", );
    }
    

    218 => {
        println!("get_known_node_types_number()", );
    }
    

    219 => {
        println!("get_unknown_node_types_rate()", );
    }
    

    220 => {
        println!("get_known_node_types_rate()", );
    }
    

    221 => {
        println!("get_minimum_node_types_number()", );
    }
    

    222 => {
        println!("get_maximum_node_types_number()", );
    }
    

    223 => {
        println!("get_maximum_multilabel_count()", );
    }
    

    224 => {
        println!("get_singleton_node_types_number()", );
    }
    

    225 => {
        println!("get_singleton_node_type_ids()", );
    }
    

    226 => {
        println!("get_singleton_node_type_names()", );
    }
    

    227 => {
        println!("get_unknown_edge_types_number()", );
    }
    

    228 => {
        println!("get_edge_ids_with_unknown_edge_types()", );
    }
    

    229 => {
        println!("get_edge_ids_with_known_edge_types()", );
    }
    

    230 => {
        println!("get_edge_node_ids_with_unknown_edge_types(directed: {:?})", &data.getedgenodeidswithunknownedgetypes.directed);
    }
    

    231 => {
        println!("get_edge_node_ids_with_known_edge_types(directed: {:?})", &data.getedgenodeidswithknownedgetypes.directed);
    }
    

    232 => {
        println!("get_edge_node_names_with_unknown_edge_types(directed: {:?})", &data.getedgenodenameswithunknownedgetypes.directed);
    }
    

    233 => {
        println!("get_edge_node_names_with_known_edge_types(directed: {:?})", &data.getedgenodenameswithknownedgetypes.directed);
    }
    

    234 => {
        println!("get_edge_ids_with_unknown_edge_types_mask()", );
    }
    

    235 => {
        println!("get_edge_ids_with_known_edge_types_mask()", );
    }
    

    236 => {
        println!("get_node_ids_with_unknown_node_types()", );
    }
    

    237 => {
        println!("get_node_ids_with_known_node_types()", );
    }
    

    238 => {
        println!("get_node_names_with_unknown_node_types()", );
    }
    

    239 => {
        println!("get_node_names_with_known_node_types()", );
    }
    

    240 => {
        println!("get_node_ids_with_unknown_node_types_mask()", );
    }
    

    241 => {
        println!("get_node_ids_with_known_node_types_mask()", );
    }
    

    242 => {
        println!("get_known_edge_types_number()", );
    }
    

    243 => {
        println!("get_unknown_edge_types_rate()", );
    }
    

    244 => {
        println!("get_known_edge_types_rate()", );
    }
    

    245 => {
        println!("get_minimum_edge_types_number()", );
    }
    

    246 => {
        println!("get_singleton_edge_types_number()", );
    }
    

    247 => {
        println!("get_singleton_edge_type_ids()", );
    }
    

    248 => {
        println!("get_singleton_edge_type_names()", );
    }
    

    249 => {
        println!("get_nodes_number()", );
    }
    

    250 => {
        println!("get_node_connected_component_ids(verbose: {:?})", &data.getnodeconnectedcomponentids.verbose);
    }
    

    251 => {
        println!("get_directed_edges_number()", );
    }
    

    252 => {
        println!("get_edge_types_number()", );
    }
    

    253 => {
        println!("get_node_types_number()", );
    }
    

    254 => {
        println!("get_node_degrees()", );
    }
    

    255 => {
        println!("get_weighted_node_degrees()", );
    }
    

    256 => {
        println!("get_not_singletons_node_ids()", );
    }
    

    257 => {
        println!("get_dense_nodes_mapping()", );
    }
    

    258 => {
        println!("get_parallel_edges_number()", );
    }
    

    259 => {
        println!("get_cumulative_node_degrees()", );
    }
    

    260 => {
        println!("get_unique_source_nodes_number()", );
    }
    

    261 => {
        println!("get_edge_type_id_counts_hashmap()", );
    }
    

    262 => {
        println!("get_edge_type_names_counts_hashmap()", );
    }
    

    263 => {
        println!("get_node_type_id_counts_hashmap()", );
    }
    

    264 => {
        println!("get_node_type_names_counts_hashmap()", );
    }
    

    265 => {
        println!("get_dense_binary_adjacency_matrix()", );
    }
    

    266 => {
        println!("get_dense_weighted_adjacency_matrix(weight: {:?})", &data.getdenseweightedadjacencymatrix.weight);
    }
    

    267 => {
        println!("iter_node_ids()", );
    }
    

    268 => {
        println!("par_iter_node_ids()", );
    }
    

    269 => {
        println!("iter_node_names()", );
    }
    

    270 => {
        println!("par_iter_node_names()", );
    }
    

    271 => {
        println!("iter_unique_node_type_ids()", );
    }
    

    272 => {
        println!("iter_node_type_counts()", );
    }
    

    273 => {
        println!("iter_unique_node_type_ids_and_counts()", );
    }
    

    274 => {
        println!("iter_unique_node_type_names()", );
    }
    

    275 => {
        println!("iter_unique_node_type_names_and_counts()", );
    }
    

    276 => {
        println!("iter_unique_edge_type_ids()", );
    }
    

    277 => {
        println!("iter_edge_type_counts()", );
    }
    

    278 => {
        println!("iter_unique_edge_type_ids_and_counts()", );
    }
    

    279 => {
        println!("iter_unique_edge_type_names_and_counts()", );
    }
    

    280 => {
        println!("iter_unique_edge_type_names()", );
    }
    

    281 => {
        println!("iter_node_degrees()", );
    }
    

    282 => {
        println!("par_iter_node_degrees()", );
    }
    

    283 => {
        println!("iter_weighted_node_degrees()", );
    }
    

    284 => {
        println!("par_iter_weighted_node_degrees()", );
    }
    

    285 => {
        println!("iter_connected_node_ids()", );
    }
    

    286 => {
        println!("iter_singleton_node_ids()", );
    }
    

    287 => {
        println!("iter_singleton_node_names()", );
    }
    

    288 => {
        println!("iter_singleton_nodes_with_selfloops_node_ids()", );
    }
    

    289 => {
        println!("iter_singleton_nodes_with_selfloops_node_names()", );
    }
    

    290 => {
        println!("iter_singleton_node_type_ids()", );
    }
    

    291 => {
        println!("iter_singleton_edge_type_ids()", );
    }
    

    292 => {
        println!("iter_singleton_node_type_names()", );
    }
    

    293 => {
        println!("iter_singleton_edge_type_names()", );
    }
    

    294 => {
        println!("iter_source_node_ids(directed: {:?})", &data.itersourcenodeids.directed);
    }
    

    295 => {
        println!("iter_edge_weights()", );
    }
    

    296 => {
        println!("par_iter_edge_weights()", );
    }
    

    297 => {
        println!("par_iter_source_node_ids(directed: {:?})", &data.paritersourcenodeids.directed);
    }
    

    298 => {
        println!("iter_destination_node_ids(directed: {:?})", &data.iterdestinationnodeids.directed);
    }
    

    299 => {
        println!("par_iter_destination_node_ids(directed: {:?})", &data.pariterdestinationnodeids.directed);
    }
    

    300 => {
        println!("iter_node_ids_and_node_type_ids()", );
    }
    

    301 => {
        println!("iter_one_hot_encoded_node_type_ids()", );
    }
    

    302 => {
        println!("iter_one_hot_encoded_known_node_type_ids()", );
    }
    

    303 => {
        println!("iter_node_names_and_node_type_names()", );
    }
    

    304 => {
        println!("par_iter_node_names_and_node_type_names()", );
    }
    

    305 => {
        println!("iter_edge_node_ids(directed: {:?})", &data.iteredgenodeids.directed);
    }
    

    306 => {
        println!("iter_edges(directed: {:?})", &data.iteredges.directed);
    }
    

    307 => {
        println!("par_iter_edge_node_ids(directed: {:?})", &data.pariteredgenodeids.directed);
    }
    

    308 => {
        println!("par_iter_directed_edge_ids()", );
    }
    

    309 => {
        println!("par_iter_edges(directed: {:?})", &data.pariteredges.directed);
    }
    

    310 => {
        println!("iter_edge_node_ids_and_edge_weight()", );
    }
    

    311 => {
        println!("par_iter_edge_node_ids_and_edge_weight()", );
    }
    

    312 => {
        println!("iter_edge_node_ids_and_edge_type_id(directed: {:?})", &data.iteredgenodeidsandedgetypeid.directed);
    }
    

    313 => {
        println!("iter_one_hot_encoded_edge_type_ids()", );
    }
    

    314 => {
        println!("iter_one_hot_encoded_known_edge_type_ids()", );
    }
    

    315 => {
        println!("iter_edge_node_names_and_edge_type_name(directed: {:?})", &data.iteredgenodenamesandedgetypename.directed);
    }
    

    316 => {
        println!("par_iter_edge_node_names_and_edge_type_name(directed: {:?})", &data.pariteredgenodenamesandedgetypename.directed);
    }
    

    317 => {
        println!("par_iter_edge_node_ids_and_edge_type_id(directed: {:?})", &data.pariteredgenodeidsandedgetypeid.directed);
    }
    

    318 => {
        println!("par_iter_edge_node_names_and_edge_type_name_and_edge_weight(directed: {:?})", &data.pariteredgenodenamesandedgetypenameandedgeweight.directed);
    }
    

    319 => {
        println!("iter_edge_node_names_and_edge_type_name_and_edge_weight(directed: {:?})", &data.iteredgenodenamesandedgetypenameandedgeweight.directed);
    }
    

    320 => {
        println!("par_iter_edge_node_ids_and_edge_type_id_and_edge_weight(directed: {:?})", &data.pariteredgenodeidsandedgetypeidandedgeweight.directed);
    }
    

    321 => {
        println!("iter_edge_node_ids_and_edge_type_id_and_edge_weight(directed: {:?})", &data.iteredgenodeidsandedgetypeidandedgeweight.directed);
    }
    

    322 => {
        println!("iter_unique_edge_node_ids(directed: {:?})", &data.iteruniqueedgenodeids.directed);
    }
    

    323 => {
        println!("iter_unique_source_node_ids()", );
    }
    

    324 => {
        println!("iter_edge_ids_with_unknown_edge_types()", );
    }
    

    325 => {
        println!("iter_edge_ids_with_known_edge_types()", );
    }
    

    326 => {
        println!("iter_edge_node_ids_with_unknown_edge_types(directed: {:?})", &data.iteredgenodeidswithunknownedgetypes.directed);
    }
    

    327 => {
        println!("iter_edge_node_ids_with_known_edge_types(directed: {:?})", &data.iteredgenodeidswithknownedgetypes.directed);
    }
    

    328 => {
        println!("iter_node_ids_with_unknown_node_types()", );
    }
    

    329 => {
        println!("iter_node_ids_with_known_node_types()", );
    }
    

    330 => {
        println!("iter_edge_node_names_with_unknown_edge_types(directed: {:?})", &data.iteredgenodenameswithunknownedgetypes.directed);
    }
    

    331 => {
        println!("iter_edge_node_names_with_known_edge_types(directed: {:?})", &data.iteredgenodenameswithknownedgetypes.directed);
    }
    

    332 => {
        println!("iter_node_names_with_unknown_node_types()", );
    }
    

    333 => {
        println!("iter_node_names_with_known_node_types()", );
    }
    

    334 => {
        println!("par_iter_edge_ids_with_unknown_edge_types()", );
    }
    

    335 => {
        println!("par_iter_edge_ids_with_known_edge_types()", );
    }
    

    336 => {
        println!("par_iter_edge_node_ids_with_unknown_edge_types(directed: {:?})", &data.pariteredgenodeidswithunknownedgetypes.directed);
    }
    

    337 => {
        println!("par_iter_edge_node_ids_with_known_edge_types(directed: {:?})", &data.pariteredgenodeidswithknownedgetypes.directed);
    }
    

    338 => {
        println!("par_iter_node_ids_with_unknown_node_types()", );
    }
    

    339 => {
        println!("par_iter_node_ids_with_known_node_types()", );
    }
    

    340 => {
        println!("par_iter_edge_node_names_with_unknown_edge_types(directed: {:?})", &data.pariteredgenodenameswithunknownedgetypes.directed);
    }
    

    341 => {
        println!("par_iter_edge_node_names_with_known_edge_types(directed: {:?})", &data.pariteredgenodenameswithknownedgetypes.directed);
    }
    

    342 => {
        println!("par_iter_node_names_with_unknown_node_types()", );
    }
    

    343 => {
        println!("par_iter_node_names_with_known_node_types()", );
    }
    

    344 => {
        println!("get_laplacian_transformed_graph(verbose: {:?})", &data.getlaplaciantransformedgraph.verbose);
    }
    

    345 => {
        println!("get_random_walk_normalized_laplacian_transformed_graph(verbose: {:?})", &data.getrandomwalknormalizedlaplaciantransformedgraph.verbose);
    }
    

    346 => {
        println!("get_symmetric_normalized_laplacian_transformed_graph(verbose: {:?})", &data.getsymmetricnormalizedlaplaciantransformedgraph.verbose);
    }
    

    347 => {
        println!("get_symmetric_normalized_transformed_graph(verbose: {:?})", &data.getsymmetricnormalizedtransformedgraph.verbose);
    }
    

    348 => {
        println!("get_weighted_laplacian_transformed_graph(verbose: {:?})", &data.getweightedlaplaciantransformedgraph.verbose);
    }
    

    349 => {
        println!("get_weighted_symmetric_normalized_laplacian_transformed_graph(verbose: {:?})", &data.getweightedsymmetricnormalizedlaplaciantransformedgraph.verbose);
    }
    

    350 => {
        println!("get_weighted_symmetric_normalized_transformed_graph(verbose: {:?})", &data.getweightedsymmetricnormalizedtransformedgraph.verbose);
    }
    

    351 => {
        println!("get_weighted_random_walk_normalized_laplacian_transformed_graph(verbose: {:?})", &data.getweightedrandomwalknormalizedlaplaciantransformedgraph.verbose);
    }
    

    352 => {
        println!("has_default_graph_name()", );
    }
    

    353 => {
        println!("has_nodes()", );
    }
    

    354 => {
        println!("has_edges()", );
    }
    

    355 => {
        println!("has_trap_nodes()", );
    }
    

    356 => {
        println!("is_directed()", );
    }
    

    357 => {
        println!("has_edge_weights()", );
    }
    

    358 => {
        println!("has_edge_weights_representing_probabilities()", );
    }
    

    359 => {
        println!("has_weighted_singleton_nodes()", );
    }
    

    360 => {
        println!("has_constant_edge_weights()", );
    }
    

    361 => {
        println!("has_negative_edge_weights()", );
    }
    

    362 => {
        println!("has_edge_types()", );
    }
    

    363 => {
        println!("has_selfloops()", );
    }
    

    364 => {
        println!("has_disconnected_nodes()", );
    }
    

    365 => {
        println!("has_singleton_nodes()", );
    }
    

    366 => {
        println!("has_singleton_nodes_with_selfloops()", );
    }
    

    367 => {
        println!("is_connected(verbose: {:?})", &data.isconnected.verbose);
    }
    

    368 => {
        println!("has_node_types()", );
    }
    

    369 => {
        println!("has_multilabel_node_types()", );
    }
    

    370 => {
        println!("has_unknown_node_types()", );
    }
    

    371 => {
        println!("has_known_node_types()", );
    }
    

    372 => {
        println!("has_unknown_edge_types()", );
    }
    

    373 => {
        println!("has_known_edge_types()", );
    }
    

    374 => {
        println!("has_homogeneous_node_types()", );
    }
    

    375 => {
        println!("has_homogeneous_edge_types()", );
    }
    

    376 => {
        println!("has_singleton_node_types()", );
    }
    

    377 => {
        println!("has_node_oddities()", );
    }
    

    378 => {
        println!("has_node_types_oddities()", );
    }
    

    379 => {
        println!("has_singleton_edge_types()", );
    }
    

    380 => {
        println!("has_edge_types_oddities()", );
    }
    

    381 => {
        println!("is_multigraph()", );
    }
    

    382 => {
        println!("has_nodes_sorted_by_decreasing_outbound_node_degree()", );
    }
    

    383 => {
        println!("has_nodes_sorted_by_increasing_outbound_node_degree()", );
    }
    

    384 => {
        println!("compute_hash()", );
    }
    

    385 => {
        println!("to_directed_inplace()", );
    }
    

    386 => {
        println!("to_directed()", );
    }
    

    387 => {
        println!("to_upper_triangular(verbose: {:?})", &data.touppertriangular.verbose);
    }
    

    388 => {
        println!("to_lower_triangular(verbose: {:?})", &data.tolowertriangular.verbose);
    }
    

    389 => {
        println!("to_main_diagonal(verbose: {:?})", &data.tomaindiagonal.verbose);
    }
    

    390 => {
        println!("to_anti_diagonal(verbose: {:?})", &data.toantidiagonal.verbose);
    }
    

    391 => {
        println!("to_bidiagonal(verbose: {:?})", &data.tobidiagonal.verbose);
    }
    

    392 => {
        println!("to_arrowhead(verbose: {:?})", &data.toarrowhead.verbose);
    }
    

    393 => {
        println!("to_transposed(verbose: {:?})", &data.totransposed.verbose);
    }
    

    394 => {
        println!("to_complementary(verbose: {:?})", &data.tocomplementary.verbose);
    }
    

    395 => {
        println!("get_node_label_random_holdout(train_size: {:?}, use_stratification: {:?}, random_state: {:?})", &data.getnodelabelrandomholdout.train_size, &data.getnodelabelrandomholdout.use_stratification, &data.getnodelabelrandomholdout.random_state);
    }
    

    396 => {
        println!("get_node_label_kfold(k: {:?}, k_index: {:?}, use_stratification: {:?}, random_state: {:?})", &data.getnodelabelkfold.k, &data.getnodelabelkfold.k_index, &data.getnodelabelkfold.use_stratification, &data.getnodelabelkfold.random_state);
    }
    

    397 => {
        println!("get_edge_label_random_holdout(train_size: {:?}, use_stratification: {:?}, random_state: {:?})", &data.getedgelabelrandomholdout.train_size, &data.getedgelabelrandomholdout.use_stratification, &data.getedgelabelrandomholdout.random_state);
    }
    

    398 => {
        println!("get_edge_label_kfold(k: {:?}, k_index: {:?}, use_stratification: {:?}, random_state: {:?})", &data.getedgelabelkfold.k, &data.getedgelabelkfold.k_index, &data.getedgelabelkfold.use_stratification, &data.getedgelabelkfold.random_state);
    }
    

    399 => {
        println!("get_random_subgraph(nodes_number: {:?}, random_state: {:?}, verbose: {:?})", &data.getrandomsubgraph.nodes_number, &data.getrandomsubgraph.random_state, &data.getrandomsubgraph.verbose);
    }
    
            _ => unreachable!()
        }
    }

    Ok(())
}
