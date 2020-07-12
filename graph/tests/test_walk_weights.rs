use graph::WalkWeights;

#[test]
fn test_invalid_weights() {
    assert!(WalkWeights::default().set_return_weight(0.0).is_err());
    assert!(WalkWeights::default().set_explore_weight(0.0).is_err());
    assert!(WalkWeights::default().set_change_node_type_weight(0.0).is_err());
    assert!(WalkWeights::default().set_change_edge_type_weight(0.0).is_err());
}
