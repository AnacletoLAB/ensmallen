mod utilities;
use utilities::*;

#[test]
/// Test that everything runs properly in the PPI graph.
fn test_ppi_with_nodes_file() {
    let ppi = load_ppi(true).unwrap();
    assert_eq!(*ppi.is_directed(), false);
    assert_eq!(ppi.get_edges_number(), 588748);
    assert_eq!(ppi.get_nodes_number(), 37163);
    default_test_suite(&ppi);
}

#[test]
fn test_ppi_without_nodes_file() {
    let ppi = load_ppi(false).unwrap();
    assert_eq!(*ppi.is_directed(), false);
    assert_eq!(ppi.get_edges_number(), 588748);
    assert_eq!(ppi.get_nodes_number(), 17185);
    default_test_suite(&ppi);
}
