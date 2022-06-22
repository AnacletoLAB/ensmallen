use super::*;
use graph::{NodeT, WalksParameters, WeightT};

/// Return new walk parameters object from provided kwargs.
pub(crate) fn build_walk_parameters(kwargs: &PyDict) -> PyResult<WalksParameters> {
    let walk_length = extract_value_rust_result!(kwargs, "walk_length", u64);
    Ok(pe!(pe!(pe!(pe!(pe!(pe!(pe!(walk_length
        .map_or_else(
            || Ok(WalksParameters::default()),
            |walk_length| WalksParameters::new(walk_length),
        ))?
    .set_change_edge_type_weight(
        extract_value_rust_result!(kwargs, "change_edge_type_weight", WeightT)
    ))?
    .set_change_node_type_weight(
        extract_value_rust_result!(kwargs, "change_node_type_weight", WeightT)
    ))?
    .set_explore_weight(extract_value_rust_result!(
        kwargs,
        "explore_weight",
        WeightT
    )))?
    .set_return_weight(extract_value_rust_result!(
        kwargs,
        "return_weight",
        WeightT
    )))?
    .set_random_state(extract_value_rust_result!(kwargs, "random_state", usize))
    .set_max_neighbours(extract_value_rust_result!(
        kwargs,
        "max_neighbours",
        NodeT
    )))?
    .set_normalize_by_degree(extract_value_rust_result!(
        kwargs,
        "normalize_by_degree",
        bool
    ))
    .set_iterations(extract_value_rust_result!(
        kwargs,
        "iterations",
        NodeT
    )))?)
}
