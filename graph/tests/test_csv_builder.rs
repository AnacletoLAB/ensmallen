extern crate graph;
use graph::*;
use graph::{SingleWalkParameters, WalkWeights, WalksParameters};
use linecount::count_lines;
use std::collections::HashMap;
use std::fs::File;


#[test]
fn test_csv_builder_only_edges() {
    let path = "tests/data/edge_file.tsv";
    for directed in &[true, false] {
        let graph = FromCsvBuilder::new(
            path, 
            "subject", 
            "object", 
            *directed, 
            None
        ).unwrap().build().unwrap();
        assert_eq!(graph.get_edge_types_number(), 0);
        assert_eq!(graph.get_node_types_number(), 0);
        let lines: usize = count_lines(File::open(path).unwrap()).unwrap();
        if *directed {
            assert_eq!(lines, graph.get_edges_number());
        }
        let walk_parameters = WalksParameters::new(
            SingleWalkParameters::new(10, WalkWeights::default()).unwrap(),
            0,
            graph.get_not_trap_nodes_number(),
        )
        .unwrap()
        .set_dense_nodes_mapping(Some(graph.get_dense_nodes_mapping()));
        let wrong_walk_parameters1 = WalksParameters::new(
            SingleWalkParameters::new(10, WalkWeights::default()).unwrap(),
            0,
            graph.get_not_trap_nodes_number() * 2,
        )
        .unwrap();
        let wrong_walk_parameters2 = WalksParameters::new(
            SingleWalkParameters::new(10, WalkWeights::default()).unwrap(),
            graph.get_not_trap_nodes_number() * 2,
            graph.get_not_trap_nodes_number() * 4,
        )
        .unwrap();
        let nodes_map: HashMap<NodeT, NodeT> = HashMap::new();
        let wrong_walk_parameters3 = WalksParameters::new(
            SingleWalkParameters::new(10, WalkWeights::default()).unwrap(),
            0,
            graph.get_not_trap_nodes_number(),
        )
        .unwrap()
        .set_dense_nodes_mapping(Some(nodes_map));
        assert!(wrong_walk_parameters1.validate(&graph).is_err());
        assert!(wrong_walk_parameters2.validate(&graph).is_err());
        assert!(wrong_walk_parameters3.validate(&graph).is_err());
        assert_eq!(graph, graph);
        assert!(graph.sum(&graph).is_err());
        assert!(graph.get_node_type_id(0).is_err());
        assert!(graph
            .cooccurence_matrix(&walk_parameters, None, Some(true))
            .is_ok());
        assert!(graph
            .cooccurence_matrix(&walk_parameters, None, None)
            .is_ok());
        assert!(graph
            .binary_skipgrams(0, &walk_parameters, None, Some(7.0), Some(true))
            .is_ok());
        assert!(graph
            .binary_skipgrams(0, &walk_parameters, None, Some(-7.0), Some(true))
            .is_err());
        assert!(graph
            .binary_skipgrams(
                67676676676676,
                &walk_parameters,
                None,
                Some(0.5),
                Some(true)
            )
            .is_ok());
        assert!(graph.walk(&walk_parameters).is_ok());
        assert!(graph.node2vec(&walk_parameters, None, Some(true)).is_ok());
        assert!(graph.node2vec(&walk_parameters, None, None).is_ok());
        assert!(graph.get_top_k_nodes_by_node_type(10).is_err());
        assert!(graph.get_top_k_edges_by_edge_type(10).is_err());
        assert!(graph.get_node_type_counts().is_err());
        assert!(graph.get_edge_type_counts().is_err());
    }
}

#[test]
fn test_csv_builder_only_edges_with_edge_types() {
    let path = "tests/data/edge_file.tsv";
    for directed in &[true, false] {
        let graph = FromCsvBuilder::new(
            path, 
            "subject", 
            "object", 
            *directed, 
            None
        ).unwrap().set_edge_types(
            "edge_label",
            Some("biolink:Association"),
        )
        .build().unwrap();
        
        let walk_parameters = WalksParameters::new(
            SingleWalkParameters::new(
                10,
                WalkWeights::default()
                    .set_return_weight(Some(2.0))
                    .unwrap()
                    .set_explore_weight(Some(2.0))
                    .unwrap()
                    .set_change_node_type_weight(Some(2.0))
                    .unwrap()
                    .set_change_edge_type_weight(Some(2.0))
                    .unwrap(),
            )
            .unwrap(),
            0,
            graph.get_not_trap_nodes_number(),
        )
        .unwrap();
        let lines: usize = count_lines(File::open(path).unwrap()).unwrap();
        if *directed {
            assert_eq!(lines, graph.get_edges_number());
        }
        assert_eq!(graph.get_edge_types_number(), 2);
        assert_eq!(graph.get_node_types_number(), 0);
        assert!(graph.walk(&walk_parameters).is_ok());
    }
}


#[test]
fn test_csv_builder_forced_conversion_to_undirected() {
    assert!(
        FromCsvBuilder::new(
            "tests/data/directed_with_bidirectionals.tsv",
            "subject", 
            "object", 
            false, 
            None
        ).unwrap().set_edge_types(
            "edge_label",
            Some("biolink:Association"),
        )
        .build().is_err()
    );
    assert!(
        FromCsvBuilder::new(
            "tests/data/directed_with_bidirectionals.tsv",
            "subject", 
            "object", 
            false, 
            None
        ).unwrap()
        .build().is_err()
    );

    assert!(
        FromCsvBuilder::new(
            "tests/data/directed_with_bidirectionals.tsv",
            "subject", 
            "object", 
            false, 
            None
        ).unwrap()
        .set_force_conversion_to_undirected()
        .build().is_ok()
    );
}

#[test]
fn test_csv_builder_zero_weights_error() {
        assert!(
        FromCsvBuilder::new(
            "tests/data/zero_weights.tsv",
            "subject", 
            "object", 
            false, 
            None
        ).unwrap().set_edge_types(
            "edge_label",
            Some("biolink:Association"),
        )
        .set_weights(
            "weight",
            None
        )
        .build().is_err()
    );
}


#[test]
fn test_csv_builder_duplicated_edges() {
    let path = "tests/data/duplicated_edge.tsv";
    assert!(
        FromCsvBuilder::new(
            path,
            "subject", 
            "object", 
            true, 
            None
        ).unwrap().set_edge_types(
            "edge_label",
            Some("biolink:Association"),
        )
        .set_weights(
            "weight",
            None
        )
        .build().is_err()
    );
    assert!(
        FromCsvBuilder::new(
            path,
            "subject", 
            "object", 
            true, 
            None
        ).unwrap()
        .set_weights(
            "weight",
            None
        )
        .build().is_err()
    );
    assert!(
        FromCsvBuilder::new(
            path,
            "subject", 
            "object", 
            true, 
            None
        ).unwrap().set_edge_types(
            "edge_label",
            Some("biolink:Association"),
        )
        .set_weights(
            "weight",
            None
        )
        .set_ignore_duplicated_edges()
        .build().is_ok()
    );
}


#[test]
fn test_csv_builder_two_node_files() {
    let edge_path = "tests/data/edge_file.tsv";
    let node_path = "tests/data/duplicated_node.tsv";
    assert!(
        FromCsvBuilder::new(
            edge_path,
            "subject", 
            "object", 
            true, 
            None
        ).unwrap().set_edge_types(
            "edge_label",
            Some("biolink:Association"),
        )
        .set_weights(
            "weight",
            Some(1.0)
        )
        .load_nodes_csv(
            node_path,
            "id", 
            "category", 
            Some("biolink:NamedThing"), 
            None, 
            Some(false)
        ).unwrap()
        .load_nodes_csv(
            node_path,
            "id", 
            "category", 
            Some("biolink:NamedThing"), 
            None, 
            Some(false)
        ).is_err()
    );
}

#[test]
fn test_csv_builder_duplicated_nodes() {
    let edge_path = "tests/data/edge_file.tsv";
    let node_path = "tests/data/duplicated_node.tsv";
    assert!(
        FromCsvBuilder::new(
            edge_path,
            "subject", 
            "object", 
            true, 
            None
        ).unwrap().set_edge_types(
            "edge_label",
            Some("biolink:Association"),
        )
        .set_weights(
            "weight",
            Some(1.0)
        )
        .load_nodes_csv(
            node_path,
            "id", 
            "category", 
            Some("biolink:NamedThing"), 
            None, 
            Some(false)
        ).unwrap()
        .build().is_err()
    );
    assert!(
        FromCsvBuilder::new(
            edge_path,
            "subject", 
            "object", 
            true, 
            None
        ).unwrap().set_edge_types(
            "edge_label",
            Some("biolink:Association"),
        )
        .set_weights(
            "weight",
            Some(1.0)
        )
        .load_nodes_csv(
            node_path,
            "id", 
            "category", 
            Some("biolink:NamedThing"), 
            None, 
            Some(true)
        ).unwrap()
        .build().is_ok()
    );
}

#[test]
fn test_csv_builder_no_column_panic() {
    let edge_path = "tests/data/edge_file.tsv";
    let node_path = "tests/data/node_file.tsv";
    assert!(
        FromCsvBuilder::new(
            edge_path,
            "subject", 
            "object", 
            true, 
            Some("")
        ).is_err()
    );
    assert!(
        FromCsvBuilder::new(
            edge_path,
            "subject", 
            "object", 
            true, 
            Some("\t")
        ).is_ok()
    );
    assert!(
        FromCsvBuilder::new(
            edge_path,
            "subject", 
            "object", 
            true, 
            None
        ).unwrap().set_edge_types(
            "edge_label",
            Some("biolink:Association"),
        )
        .set_weights(
            "weight",
            Some(1.0)
        )
        .load_nodes_csv(
            node_path,
            "id", 
            "category", 
            Some("biolink:NamedThing"), 
            Some(""), 
            None
        ).is_err()
    );
    assert!(
        FromCsvBuilder::new(
            edge_path,
            "subject", 
            "object", 
            true, 
            None
        ).unwrap().set_edge_types(
            "edge_label",
            Some("biolink:Association"),
        )
        .set_weights(
            "weight",
            Some(1.0)
        )
        .load_nodes_csv(
            node_path,
            "id", 
            "", 
            Some("biolink:NamedThing"), 
            None, 
            None
        ).is_err()
    );
    assert!(
        FromCsvBuilder::new(
            edge_path,
            "subject", 
            "object", 
            true, 
            None
        ).unwrap().set_edge_types(
            "edge_label",
            Some("biolink:Association"),
        )
        .set_weights(
            "weight",
            Some(1.0)
        )
        .load_nodes_csv(
            node_path,
            "", 
            "category", 
            Some("biolink:NamedThing"), 
            None, 
            None
        ).is_err()
    );
    assert!(
        FromCsvBuilder::new(
            edge_path,
            "subject", 
            "object", 
            true, 
            None
        ).unwrap().set_edge_types(
            "edge_label",
            Some("biolink:Association"),
        )
        .set_weights(
            "weight",
            Some(1.0)
        )
        .load_nodes_csv(
            node_path,
            "id", 
            "", 
            Some("biolink:NamedThing"), 
            None, 
            None
        ).is_err()
    );
}


#[test]
fn test_csv_builder_no_default_weight() {
    let edge_path = "tests/data/edge_file_missing_weights.tsv";
    assert!(
        FromCsvBuilder::new(
            edge_path,
            "subject", 
            "object", 
            true, 
            None
        ).unwrap().set_edge_types(
            "edge_label",
            None,
        ).build().is_err()
    );
    assert!(
        FromCsvBuilder::new(
            edge_path,
            "subject", 
            "object", 
            true, 
            None
        ).unwrap().set_edge_types(
            "edge_label",
            Some("default_type"),
        ).build().is_ok()
    );
    assert!(
        FromCsvBuilder::new(
            edge_path,
            "subject", 
            "object", 
            true, 
            None
        ).unwrap().set_edge_types(
            "edge_label",
            Some("default_type"),
        ).set_weights(
            "weight",
            None
        ).build().is_err()
    );
    assert!(
        FromCsvBuilder::new(
            edge_path,
            "subject", 
            "object", 
            true, 
            None
        ).unwrap().set_edge_types(
            "edge_label",
            Some("default_type"),
        ).set_weights(
            "weight",
            Some(1.0)
        ).build().is_ok()
    );
}

#[test]
fn test_csv_builder_default_node_types() {
    let edge_path = "tests/data/edge_file.tsv";
    let node_path = "tests/data/missing_node_types.tsv";
    assert!(
        FromCsvBuilder::new(
            edge_path,
            "subject", 
            "object", 
            true, 
            None
        ).unwrap().set_edge_types(
            "edge_label",
            Some("biolink:Association"),
        )
        .set_weights(
            "weight",
            Some(1.0)
        )
        .load_nodes_csv(
            node_path,
            "id", 
            "category", 
            None, 
            None, 
            None
        ).unwrap().build().is_err()
    );
    assert!(
        FromCsvBuilder::new(
            edge_path,
            "subject", 
            "object", 
            true, 
            None
        ).unwrap().set_edge_types(
            "edge_label",
            Some("biolink:Association"),
        )
        .set_weights(
            "weight",
            Some(1.0)
        )
        .load_nodes_csv(
            node_path,
            "id", 
            "category", 
            Some("biolink:NamedThing"), 
            None, 
            None
        ).unwrap().build().is_ok()
    );
}

#[test]
fn test_graph_from_csv_weird_edge_nodes() {
    let edge_path = "tests/data/edge_file_with_weird_nodes.tsv";
    let node_path = "tests/data/node_file.tsv";
    assert!(
        FromCsvBuilder::new(
            edge_path,
            "subject", 
            "object", 
            true, 
            None
        ).unwrap().set_edge_types(
            "edge_label",
            Some("biolink:Association"),
        )
        .set_weights(
            "weight",
            Some(1.0)
        )
        .load_nodes_csv(
            node_path,
            "id", 
            "category", 
            Some("biolink:NamedThing"), 
            None, 
            None
        ).unwrap().build().is_err()
    );
}

#[test]
fn test_graph_from_csv_with_edge_and_nodes() {
    let edge_path = "tests/data/edge_file.tsv";
    let node_path = "tests/data/node_file.tsv";
    for directed in &[true, false] {
        let graph = FromCsvBuilder::new(
            edge_path,
            "subject", 
            "object", 
            *directed, 
            None
        ).unwrap().set_edge_types(
            "edge_label",
            Some("biolink:Association"),
        )
        .set_weights(
            "weight",
            Some(1.0)
        )
        .load_nodes_csv(
            node_path,
            "id", 
            "category", 
            Some("biolink:NamedThing"), 
            None, 
            None
        ).unwrap().build().unwrap();

        let edge_lines: usize = count_lines(File::open(edge_path).unwrap()).unwrap();
        if *directed {
            assert_eq!(edge_lines, graph.get_edges_number());
        }
        assert_eq!(graph.get_edge_types_number(), 2);
        let walk_parameters = WalksParameters::new(
            SingleWalkParameters::new(10, WalkWeights::default()).unwrap(),
            0,
            graph.get_not_trap_nodes_number(),
        )
        .unwrap();
        graph.walk(&walk_parameters).unwrap();
        assert!(graph.get_top_k_nodes_by_node_type(10).is_ok());
        assert!(graph.get_top_k_edges_by_edge_type(10).is_ok());
        assert!(graph.get_node_type_counts().is_ok());
        assert!(graph.get_edge_type_counts().is_ok());
    }
}

#[test]
fn test_graph_negative_edge_weights() {
    let edge_path = "tests/data/negative_edge_weights.tsv";
    for directed in &[true, false] {
        assert!(
            FromCsvBuilder::new(
                edge_path,
                "subject", 
                "object", 
                *directed, 
                None
            ).unwrap().set_edge_types(
                "edge_label",
                Some("biolink:Association"),
            )
            .set_weights(
                "weight",
                Some(1.0)
            ).build().is_err()
        );
    }
}

#[test]
fn test_graph_invalid_edge_weights() {
    let edge_path = "tests/data/invalid_edge_weights.tsv";
    for directed in &[true, false] {
        assert!(
            FromCsvBuilder::new(
                edge_path,
                "subject", 
                "object", 
                *directed, 
                None
            ).unwrap().set_edge_types(
                "edge_label",
                Some("biolink:Association"),
            )
            .set_weights(
                "weight",
                Some(1.0)
            ).build().is_err()
        );
        assert!(
            FromCsvBuilder::new(
                edge_path,
                "subject", 
                "object", 
                *directed, 
                None
            ).unwrap()
            .set_weights(
                "weight",
                Some(1.0)
            ).build().is_err()
        );
    }
}

#[test]
fn test_graph_nan_edge_weights() {
    let edge_path = "tests/data/nan_edge_weights.tsv";
    for directed in &[true, false] {
        assert!(
            FromCsvBuilder::new(
                edge_path,
                "subject", 
                "object", 
                *directed, 
                None
            ).unwrap().set_edge_types(
                "edge_label",
                Some("biolink:Association"),
            )
            .set_weights(
                "weight",
                Some(1.0)
            ).build().is_err()
        );
    }
}

#[test]
fn test_graph_inf_edge_weights() {
    let edge_path = "tests/data/nan_edge_weights.tsv";
    for directed in &[true, false] {
        assert!(
            FromCsvBuilder::new(
                edge_path,
                "subject", 
                "object", 
                *directed, 
                None
            ).unwrap().set_edge_types(
                "edge_label",
                Some("biolink:Association"),
            )
            .set_weights(
                "weight",
                Some(1.0)
            ).build().is_err()
        );
    }
}

#[test]
fn test_graph_from_csv_empty_node_sep() {
    let edge_path = "tests/data/edge_file.tsv";
    let node_path = "tests/data/node_file.tsv";
    assert!(
        FromCsvBuilder::new(
            edge_path,
            "subject", 
            "object", 
            true, 
            None
        ).unwrap().set_edge_types(
            "edge_label",
            Some("biolink:Association"),
        ).set_weights(
            "weight",
            Some(1.0)
        ).load_nodes_csv(
            node_path,
            "id", 
            "category", 
            Some("biolink:NamedThing"), 
            Some(""), 
            None
        ).is_err()
    );
    assert!(
        FromCsvBuilder::new(
            edge_path,
            "subject", 
            "object", 
            true, 
            None
        ).unwrap().set_edge_types(
            "edge_label",
            Some("biolink:Association"),
        ).set_weights(
            "weight",
            Some(1.0)
        ).load_nodes_csv(
            node_path,
            "id", 
            "category", 
            Some("biolink:NamedThing"), 
            Some("\t"), 
            None
        ).unwrap().build().is_ok()
    );
    assert!(
        FromCsvBuilder::new(
            edge_path,
            "subject", 
            "object", 
            true, 
            None
        ).unwrap().set_edge_types(
            "edge_label",
            Some("biolink:Association"),
        ).set_weights(
            "weight",
            Some(1.0)
        ).load_nodes_csv(
            node_path,
            "id", 
            "", 
            Some("biolink:NamedThing"), 
            None, 
            None
        ).is_err()
    );
}

#[test]
fn test_graph_from_csv_with_edge_and_nodes_types() {
    let edge_path = "tests/data/edge_file.tsv";
    let node_path = "tests/data/node_file.tsv";
    for directed in &[true, false] {
        let graph = FromCsvBuilder::new(
            edge_path,
            "subject", 
            "object", 
            *directed, 
            None
        ).unwrap().set_edge_types(
            "edge_label",
            Some("biolink:Association"),
        ).set_weights(
            "weight",
            Some(1.0)
        ).load_nodes_csv(
            node_path,
            "id", 
            "category", 
            Some("biolink:NamedThing"), 
            None, 
            None
        ).unwrap().build().unwrap();

        let edge_lines: usize = count_lines(File::open(edge_path).unwrap()).unwrap();
        if *directed {
            assert_eq!(edge_lines, graph.get_edges_number());
        }
        assert_eq!(graph.get_edge_types_number(), 2);
        let walk_parameters = WalksParameters::new(
            SingleWalkParameters::new(10, WalkWeights::default()).unwrap(),
            0,
            graph.get_not_trap_nodes_number(),
        )
        .unwrap();
        assert!(graph.connected_holdout(42, 0.7).is_err());
        graph.walk(&walk_parameters).unwrap();
        for one in 0..graph.get_nodes_number() {
            graph.get_node_type_id(one).unwrap();
            for two in 0..graph.get_nodes_number() {
                if graph.has_edge(one, two) {
                    graph
                        .get_edge_type_id(graph.get_edge_id(one, two).unwrap())
                        .unwrap();
                } else {
                    assert!(graph.get_edge_id(one, two).is_err());
                }
            }
        }
        assert!(graph.get_edge_type_id(100000000000).is_err());
    }
}

#[test]
fn test_graph_from_csv_het() {
    let edge_path = "tests/data/het_graph_edges.tsv";
    let node_path = "tests/data/het_graph_nodes.tsv";
    for directed in &[true, false] {
        let graph = FromCsvBuilder::new(
            edge_path,
            "subject", 
            "object", 
            *directed, 
            None
        ).unwrap()
        .set_weights(
            "weight",
            Some(1.0)
        ).load_nodes_csv(
            node_path,
            "id", 
            "category", 
            Some("biolink:NamedThing"), 
            None, 
            None
        ).unwrap().build().unwrap();
        let edge_lines: usize = count_lines(File::open(edge_path).unwrap()).unwrap();
        if *directed {
            assert_eq!(edge_lines, graph.get_edges_number());
        }
        assert_eq!(4, graph.get_node_types_number());
        let walk_parameters = WalksParameters::new(
            SingleWalkParameters::new(10, WalkWeights::default()).unwrap(),
            0,
            graph.get_not_trap_nodes_number(),
        )
        .unwrap();
        graph.walk(&walk_parameters).unwrap();
        assert!(graph.get_node_type_id(100000000000).is_err());
        assert!(graph.get_edge_type_id(100000000000).is_err());
        assert!(graph.connected_holdout(42, 0.7).is_err());
        let _ = graph.report();
        let _ = graph.random_holdout(42, 0.7);
    }
}
