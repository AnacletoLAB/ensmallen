use super::*;

#[derive(Debug, Clone)]
pub(crate) struct PropertyCache {
    pub(crate) min_edge_weight: Option<Result<WeightT>>,
    pub(crate) max_edge_weight: Option<Result<WeightT>>,
    pub(crate) nodes_sorted_by_increasing_outbound_node_degree: Option<bool>,
    pub(crate) selfloops_number: Option<EdgeT>,
    pub(crate) selfloops_number_unique: Option<NodeT>,
}

impl Default for PropertyCache {
    fn default() -> Self {
        PropertyCache{
            min_edge_weight: None,
            max_edge_weight: None,
            nodes_sorted_by_increasing_outbound_node_degree: None,
            selfloops_number: None,
            selfloops_number_unique: None,
        }
    }
}