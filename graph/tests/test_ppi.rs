mod utilities;
use utilities::*;

#[test]
/// Test that everything runs properly in the PPI graph.
fn test_ppi() {
    let ppi = load_ppi().unwrap();
    assert_eq!(*ppi.is_directed(), false);
    assert_eq!(ppi.get_edges_number(), 588748);
    assert_eq!(ppi.get_nodes_number(), 37163);
    println!("{:?}", ppi.report());
    ppi.walk(&first_order_walker(&ppi)).unwrap();
    ppi.walk(&second_order_walker(&ppi)).unwrap();
}
