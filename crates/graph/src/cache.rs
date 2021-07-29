use shared::types::*;

#[derive(Debug, Clone)]
pub struct PropertyCache {
    pub min_edge_weight: Option<Result<WeightT>>,
    pub max_edge_weight: Option<Result<WeightT>>,
    pub total_edge_weight: Option<Result<f64>>,
    pub min_node_degree: Option<NodeT>,
    pub max_node_degree: Option<NodeT>,
    pub nodes_sorted_by_increasing_outbound_node_degree: Option<bool>,
    pub nodes_sorted_by_decreasing_outbound_node_degree: Option<bool>,
    pub nodes_sorted_by_lexicographic_order: Option<bool>,
    pub most_central_node_id: Option<NodeT>,
    pub max_weighted_node_degree: Option<Result<f64>>,
    pub min_weighted_node_degree: Option<Result<f64>>,
    pub weighted_singleton_nodes_number: Option<Result<NodeT>>,
    pub trap_nodes_number: Option<NodeT>,
    pub selfloops_number: Option<EdgeT>,
    pub selfloops_number_unique: Option<NodeT>,
    pub singleton_nodes_with_selfloops_number: Option<NodeT>,
    pub unique_directed_edges_number: Option<EdgeT>,
}

impl Default for PropertyCache {
    fn default() -> Self {
        PropertyCache {
            min_edge_weight: None,
            max_edge_weight: None,
            total_edge_weight: None,
            min_node_degree: None,
            max_node_degree: None,
            nodes_sorted_by_increasing_outbound_node_degree: None,
            nodes_sorted_by_decreasing_outbound_node_degree: None,
            nodes_sorted_by_lexicographic_order: None,
            most_central_node_id: None,
            max_weighted_node_degree: None,
            min_weighted_node_degree: None,
            weighted_singleton_nodes_number: None,
            trap_nodes_number: None,
            selfloops_number: None,
            selfloops_number_unique: None,
            singleton_nodes_with_selfloops_number: None,
            unique_directed_edges_number: None,
        }
    }
}

impl PropertyCache {
    pub fn total(&self) -> usize {
        use std::mem::size_of;
        size_of::<PropertyCache>()
    }
}
