use graph::test_utilities::*;
use tests_macro::built_tests;

fn inner_test(load_nodes: bool, load_edge_types: bool, load_weights: bool, directed: bool, verbose: bool, skip_self_loops: bool) {
    let ppi = load_ppi(
        load_nodes,
        load_edge_types,
        load_weights,
        directed,
        verbose,
        skip_self_loops,
    )
    .unwrap();
    assert_eq!(ppi.is_directed(), directed);
    assert_eq!(ppi.has_node_types(), load_nodes);
    assert_eq!(ppi.has_edge_types(), load_edge_types);
    assert_eq!(ppi.has_weights(), load_weights);
    default_test_suite(&ppi, verbose).unwrap();
}


// create all the tests
// built_tests!();