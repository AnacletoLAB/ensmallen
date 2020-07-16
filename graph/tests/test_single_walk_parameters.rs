use graph::{SingleWalkParameters, WalkWeights};

#[test]
fn test_invalid_single_walk_parameters() {
    assert!(SingleWalkParameters::new(0, WalkWeights::default()).is_err());
}
