use super::*;

#[derive(Debug, Clone)]
pub(crate) struct PropertyCache {
    pub(crate) min_edge_weight: Option<Result<WeightT>>,
    pub(crate) max_edge_weight: Option<Result<WeightT>>,
    pub(crate) min_node_degree: Option<NodeT>,
    pub(crate) max_node_degree: Option<NodeT>,
    pub(crate) nodes_sorted_by_increasing_outbound_node_degree: Option<bool>,
    pub(crate) nodes_sorted_by_decreasing_outbound_node_degree: Option<bool>,
    pub(crate) most_central_node_id: Option<NodeT>
}

impl Default for PropertyCache {
    fn default() -> Self {
        PropertyCache{
            min_edge_weight: None,
            max_edge_weight: None,
            nodes_sorted_by_increasing_outbound_node_degree: None,
        }
    }
}