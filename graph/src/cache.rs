use super::*;

#[derive(Debug, Clone)]
pub(crate) struct PropertyCache {
    min_edge_weight: Option<Result<WeightT>>,
    max_edge_weight: Option<Result<WeightT>>,
}

impl Default for PropertyCache {
    fn default() -> Self {
        PropertyCache{
            min_edge_weight: None,
            max_edge_weight: None,
        }
    }
}