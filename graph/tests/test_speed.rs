use graph::test_utilities::*;

#[test]
/// Test that everything runs properly in the PPI graph.
fn test_speed() { 
    let ppi = load_ppi(
        true,
        true,
        true,
        false,
        true,
        false,
    )
    .unwrap();
    ppi.random_walks_iter(1, &second_order_walker(&ppi, 2.0, 2.0).unwrap()).unwrap();
}