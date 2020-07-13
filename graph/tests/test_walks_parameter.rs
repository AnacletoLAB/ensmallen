use graph::types::*;
use graph::{SingleWalkParameters, WalkWeights, WalksParameters};
use std::collections::HashMap;

#[test]
fn test_invalid_walks_parameter() {
    assert!(WalksParameters::new(
        SingleWalkParameters::new(10, WalkWeights::default()).unwrap(),
        100,
        50
    )
    .is_err());

    assert!(WalksParameters::new(
        SingleWalkParameters::new(10, WalkWeights::default()).unwrap(),
        0,
        50
    )
    .unwrap()
    .set_iterations(Some(0))
    .is_err());

    assert!(WalksParameters::new(
        SingleWalkParameters::new(10, WalkWeights::default()).unwrap(),
        0,
        50
    )
    .unwrap()
    .set_min_length(Some(0))
    .is_err());
}

#[test]
fn test_walks_parameter_verbose() {
    let nodes_map: HashMap<NodeT, NodeT> = HashMap::new();
    WalksParameters::new(
        SingleWalkParameters::new(10, WalkWeights::default()).unwrap(),
        0,
        50,
    )
    .unwrap()
    .set_iterations(Some(10))
    .unwrap()
    .set_min_length(Some(10))
    .unwrap()
    .set_verbose(Some(false))
    .set_dense_nodes_mapping(Some(nodes_map));
}
