use graph::{SingleWalkParameters, WalkWeights, WalksParameters};

#[test]
fn test_invalid_walks_parameter() {
    assert!(WalksParameters::new(
        SingleWalkParameters::new(10, WalkWeights::default()).unwrap(),
        100,
        50
    )
    .is_err());
}

#[test]
fn test_walks_parameter_verbose() {
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
    .set_verbose(Some(false));
}
