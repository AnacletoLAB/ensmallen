use graph::types::*;
use graph::{SingleWalkParameters, WalkWeights};
use std::collections::HashMap;

#[test]
fn test_invalid_single_walk_parameters() {
    assert!(SingleWalkParameters::new(0, WalkWeights::default()).is_err());
}

#[test]
fn test_set_nodes_mapping() {
    let nodes_map: HashMap<NodeT, NodeT> = HashMap::new();
    SingleWalkParameters::new(5, WalkWeights::default())
        .unwrap()
        .set_dense_nodes_mapping(Some(nodes_map));
}
