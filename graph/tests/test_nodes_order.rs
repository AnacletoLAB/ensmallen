use graph::test_utilities::*;

#[test]
/// Test that the ppi nodes are sorted.
fn test_ppi_node_order() {
    let ppi1 = load_ppi(true, true, true, false, true, false);
    let ppi2 = load_ppi(true, true, true, false, true, false);
    assert_eq!(ppi1.get_number_of_nodes(), ppi2.get_number_of_nodes());
    assert_eq!(ppi1.get_node_names()[..10], ppi2.get_node_names()[..10]);
}
