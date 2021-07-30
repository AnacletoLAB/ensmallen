use super::*;
use graph::{NodeT, Result, WalksParameters, WeightT};

use std::collections::HashMap;

impl EnsmallenGraph {
    pub(crate) fn build_walk_parameters(
        &self,
        walk_length: u64,
        kwargs: &PyDict,
    ) -> Result<WalksParameters> {
        Ok(WalksParameters::new(walk_length)?
            .set_change_edge_type_weight(extract_value_rust_result!(
                kwargs,
                "change_edge_type_weight",
                WeightT
            ))?
            .set_change_node_type_weight(extract_value_rust_result!(
                kwargs,
                "change_node_type_weight",
                WeightT
            ))?
            .set_explore_weight(extract_value_rust_result!(
                kwargs,
                "explore_weight",
                WeightT
            ))?
            .set_return_weight(extract_value_rust_result!(kwargs, "return_weight", WeightT))?
            .set_random_state(extract_value_rust_result!(kwargs, "random_state", usize))
            .set_max_neighbours(extract_value_rust_result!(kwargs, "max_neighbours", NodeT))?
            .set_iterations(extract_value_rust_result!(kwargs, "iterations", NodeT))?
            .set_dense_node_mapping(
                extract_value_rust_result!(kwargs, "dense_node_mapping", HashMap<NodeT, NodeT>),
            ))
    }
}
