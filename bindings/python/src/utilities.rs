use super::*;
use graph::{NodeT, WalksParameters, WeightT};
use std::collections::HashMap;

impl EnsmallenGraph {
    pub(crate) fn build_walk_parameters(
        &self,
        length: usize,
        kwargs: &PyDict,
    ) -> PyResult<WalksParameters> {
        Ok(pyex!(
            pyex!(pyex!(pyex!(pyex!(pyex!(pyex!(WalksParameters::new(length))?
                .set_change_edge_type_weight(extract_value!(
                    kwargs,
                    "change_edge_type_weight",
                    WeightT
                )))?
            .set_change_node_type_weight(extract_value!(
                kwargs,
                "change_node_type_weight",
                WeightT
            )))?
            .set_explore_weight(extract_value!(kwargs, "explore_weight", WeightT)))?
            .set_return_weight(extract_value!(kwargs, "return_weight", WeightT)))?
            .set_seed(extract_value!(kwargs, "seed", usize))
            .set_verbose(extract_value!(kwargs, "verbose", bool))
            .set_iterations(extract_value!(kwargs, "iterations", usize)))?
            .set_min_length(extract_value!(kwargs, "min_length", usize))
        )?
        .set_dense_node_mapping(
            extract_value!(kwargs, "dense_node_mapping", HashMap<NodeT, NodeT>),
        ))
    }
}
