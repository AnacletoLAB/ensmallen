use graph::test_utilities::*;

#[test]
/// Test that everything runs properly in the PPI graph.
fn test_ppi() {
    let bools = &[true, false];
    for load_nodes in bools {
        for load_edge_types in bools {
            for load_weights in bools {
                for directed in bools {
                    for skip_self_loops in bools {
                        for verbose in bools {
                            let ppi = load_ppi(
                                *load_nodes,
                                *load_edge_types,
                                *load_weights,
                                *directed,
                                *verbose,
                                *skip_self_loops,
                            )
                            .unwrap();
                            assert_eq!(*ppi.is_directed(), *directed);
                            assert_eq!(ppi.has_node_types(), *load_nodes);
                            assert_eq!(ppi.has_edge_types(), *load_edge_types);
                            assert_eq!(ppi.has_weights(), *load_weights);
                            default_test_suite(&ppi, *verbose).unwrap();
                        }
                    }
                }
            }
        }
    }
}
