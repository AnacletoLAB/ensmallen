use super::*;
use graph::{NodeT, WalksParameters, WeightT};
use std::collections::HashMap;

impl EnsmallenGraph {
    /// Return start node and end node for given batch.
    pub(crate) fn get_batch_range(&self, idx: usize, batch_size: usize) -> (usize, usize) {
        let (start_node, end_node) = (idx * batch_size, (idx + 1) * batch_size);
        (
            start_node,
            if end_node > self.graph.get_not_trap_nodes_number() {
                self.graph.get_not_trap_nodes_number()
            } else {
                end_node
            },
        )
    }

    pub(crate) fn build_walk_parameters(
        &self,
        length: usize,
        start: NodeT,
        end: NodeT,
        kwargs: &PyDict,
    ) -> PyResult<WalksParameters> {
        Ok(
            pyex!(pyex!(pyex!(pyex!(pyex!(pyex!(pyex!(WalksParameters::new(
                length, start, end
            ))?
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
            .set_min_length(extract_value!(kwargs, "min_length", usize)))?
            .set_dense_nodes_mapping(
                extract_value!(kwargs, "dense_nodes_mapping", HashMap<NodeT, NodeT>),
            ),
        )
    }
}
