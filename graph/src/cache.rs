use super::*;

pub(crate) struct PropertyCache {
    pub(crate) min_edge_weight: Option<Result<WeightT>>,
    pub(crate) max_edge_weight: Option<Result<WeightT>>,
    pub(crate) nodes_sorted_by_increasing_outbound_node_degree: Option<bool>,
}

unsafe impl Sync for PropertyCache {}
