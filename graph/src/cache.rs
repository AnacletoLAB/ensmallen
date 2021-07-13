use super::*;

pub(crate) struct PropertyCache {
    min_edge_weight: Option<Result<WeightT>>,
    max_edge_weight: Option<Result<WeightT>>,
}