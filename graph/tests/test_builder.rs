extern crate graph;
use std::collections::HashMap;
use graph::*;

#[test]
fn test_builder() {
    for directed in &[true, false] {
        let graph = Graph::builder(
            vec![0, 1, 2, 3, 0, 0],
            vec![3, 1, 0, 0, 0, 1],
            *directed
        ).add_weights(
            vec![1.0, 1.0, 1.0, 1.0, 2.0, 3.0]
        ).add_edge_types(
            vec![0, 0, 1, 1, 1, 0],
            [("PP".to_string(), 0), ("GG".to_string(), 1)].iter().cloned().collect(),
            vec!["PP".to_string(), "GG".to_string()]
        ).add_node_mapping(
            [("A".to_string(), 0), ("B".to_string(), 1), ("C".to_string(), 2), ("D".to_string(), 3)].iter().cloned().collect(),
            vec!["A".to_string(), "B".to_string(), "C".to_string(), "D".to_string()]
        ).add_node_types(
            vec![0, 0, 1, 1],
            [("Protein".to_string(), 0), ("Gene".to_string(), 1)].iter().cloned().collect(),
            vec!["Protein".to_string(), "Gene".to_string()]
        ).build(None).unwrap();

        let walk_parameters = WalksParameters::new(
            SingleWalkParameters::new(10, WalkWeights::default()).unwrap(),
            0,
            graph.get_not_trap_nodes_number(),
        )
        .unwrap();

        graph.walk(&walk_parameters).unwrap();
    }
}