use graph::test_utilities::*;

#[test]
/// Test that everything runs properly in the PPI graph.
fn test_speed() { 
    let ppi = load_ppi(
        false,
        false,
        true,
        false,
        false,
        false,
    )
    .unwrap();
    ppi.random_walks(1, &second_order_walker(&ppi, false).unwrap()).unwrap(),
}