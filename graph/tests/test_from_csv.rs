extern crate graph;
use graph::graph::Graph;
use graph::{SingleWalkParameters, WalkWeights, WalksParameters};
use linecount::count_lines;
use std::fs::File;

#[test]
fn test_graph_from_csv_edge_only() {
    let path = "tests/data/edge_file.tsv";
    for directed in &[true, false] {
        let graph = Graph::from_csv(
            &path, "subject", "object", *directed, None, None, None, None, None, None, None, None,
            None, None, None, None, None,
        )
        .unwrap();
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
        .unwrap();
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
        assert!(wrong_walk_parameters1.validate(&graph).is_err());
        assert!(wrong_walk_parameters2.validate(&graph).is_err());
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
fn test_graph_from_csv_edge_types() {
    let path = "tests/data/edge_file.tsv";
    for directed in &[true, false] {
        let graph = Graph::from_csv(
            &path,
            "subject",
            "object",
            *directed,
            Some("edge_label"),
            Some("biolink:Association"),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
        )
        .unwrap();
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
fn test_graph_directed_forced_conversion_to_undirected() {
    assert!(Graph::from_csv(
        "tests/data/directed_with_bidirectionals.tsv",
        "subject",
        "object",
        false,
        Some("edge_label"),
        Some("biolink:Association"),
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None
    )
    .is_err());
    assert!(Graph::from_csv(
        "tests/data/directed_with_bidirectionals.tsv",
        "subject",
        "object",
        false,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None
    )
    .is_err());
    assert!(Graph::from_csv(
        "tests/data/directed_with_bidirectionals.tsv",
        "subject",
        "object",
        false,
        Some("edge_label"),
        Some("biolink:Association"),
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        Some(true)
    )
    .is_ok());
}

#[test]
fn test_graph_from_csv_zero_weights_error() {
    let path = "tests/data/zero_weights.tsv";
    assert!(Graph::from_csv(
        path,
        "subject",
        "object",
        true,
        Some("edge_label"),
        Some("biolink:Association"),
        Some("weight"),
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None
    )
    .is_err());
}

#[test]
fn test_graph_from_csv_duplicated_edges() {
    let path = "tests/data/duplicated_edge.tsv";
    assert!(Graph::from_csv(
        path,
        "subject",
        "object",
        true,
        Some("edge_label"),
        Some("biolink:Association"),
        Some("weight"),
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None
    )
    .is_err());
    assert!(Graph::from_csv(
        path,
        "subject",
        "object",
        true,
        None,
        None,
        Some("weight"),
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None
    )
    .is_err());
    assert!(Graph::from_csv(
        path,
        "subject",
        "object",
        true,
        Some("edge_label"),
        Some("biolink:Association"),
        Some("weight"),
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        Some(true),
        None,
        None
    )
    .is_ok());
}

#[test]
fn test_graph_from_csv_duplicated_nodes() {
    let edge_path = "tests/data/edge_file.tsv";
    let node_path = "tests/data/duplicated_node.tsv";
    assert!(Graph::from_csv(
        edge_path,
        "subject",
        "object",
        true,
        Some("edge_label"),
        Some("biolink:Association"),
        Some("weight"),
        Some(1.0),
        Some(node_path),
        Some("id"),
        Some("category"),
        Some("biolink:NamedThing"),
        None,
        None,
        None,
        None,
        None
    )
    .is_err());
    Graph::from_csv(
        edge_path,
        "subject",
        "object",
        true,
        Some("edge_label"),
        Some("biolink:Association"),
        Some("weight"),
        Some(1.0),
        Some(node_path),
        Some("id"),
        Some("category"),
        Some("biolink:NamedThing"),
        None,
        None,
        None,
        Some(true),
        None,
    )
    .unwrap();
}

#[test]
fn test_graph_from_csv_no_nodes_column_panic() {
    let edge_path = "tests/data/edge_file.tsv";
    let node_path = "tests/data/node_file.tsv";
    assert!(Graph::from_csv(
        edge_path,
        "subject",
        "object",
        true,
        Some("edge_label"),
        Some("biolink:Association"),
        Some("weight"),
        Some(1.0),
        Some(node_path),
        None,
        Some("category"),
        Some("biolink:NamedThing"),
        None,
        None,
        None,
        None,
        None
    )
    .is_err());
}

#[test]
fn test_graph_from_csv_no_node_types_column() {
    let edge_path = "tests/data/edge_file.tsv";
    let node_path = "tests/data/node_file.tsv";
    assert!(Graph::from_csv(
        edge_path,
        "subject",
        "object",
        true,
        Some("edge_label"),
        Some("biolink:Association"),
        Some("weight"),
        Some(1.0),
        Some(node_path),
        Some("id"),
        None,
        Some("biolink:NamedThing"),
        None,
        None,
        None,
        None,
        None
    )
    .is_err());
}

#[test]
fn test_graph_from_csv_no_default_weight() {
    let edge_path = "tests/data/edge_file_missing_weights.tsv";
    assert!(Graph::from_csv(
        edge_path,
        "subject",
        "object",
        true,
        Some("edge_label"),
        Some("biolink:Association"),
        Some("weight"),
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None
    )
    .is_err());
    assert!(Graph::from_csv(
        edge_path,
        "subject",
        "object",
        true,
        None,
        None,
        Some("weight"),
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None
    )
    .is_err());
    assert!(Graph::from_csv(
        edge_path,
        "subject",
        "object",
        true,
        Some("edge_label"),
        Some("biolink:Association"),
        Some("weight"),
        Some(1.0),
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None
    )
    .is_ok());
}

#[test]
fn test_graph_from_csv_no_default_node_types() {
    let edge_path = "tests/data/edge_file.tsv";
    let node_path = "tests/data/missing_node_types.tsv";
    assert!(Graph::from_csv(
        edge_path,
        "subject",
        "object",
        true,
        Some("edge_label"),
        None,
        Some("weight"),
        None,
        Some(node_path),
        Some("id"),
        Some("category"),
        None,
        None,
        None,
        None,
        None,
        None
    )
    .is_err());
    assert!(Graph::from_csv(
        edge_path,
        "subject",
        "object",
        true,
        Some("edge_label"),
        None,
        Some("weight"),
        Some(1.0),
        Some(node_path),
        Some("id"),
        Some("category"),
        Some("default_node_type"),
        None,
        None,
        None,
        None,
        None
    )
    .is_ok());
}

#[test]
fn test_graph_from_csv_no_default_edge_types() {
    let edge_path = "tests/data/edge_file_missing_edge_type.tsv";
    assert!(Graph::from_csv(
        edge_path,
        "subject",
        "object",
        true,
        Some("edge_label"),
        None,
        Some("weight"),
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None
    )
    .is_err());
    assert!(Graph::from_csv(
        edge_path,
        "subject",
        "object",
        true,
        Some("edge_label"),
        Some("biolink:Association"),
        Some("weight"),
        Some(1.0),
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None
    )
    .is_ok());
}

#[test]
#[should_panic]
fn test_graph_from_csv_weird_edge_nodes() {
    let edge_path = "tests/data/edge_file_with_weird_nodes.tsv";
    let node_path = "tests/data/node_file.tsv";
    Graph::from_csv(
        edge_path,
        "subject",
        "object",
        true,
        Some("edge_label"),
        Some("biolink:Association"),
        Some("weight"),
        Some(1.0),
        Some(node_path),
        Some("id"),
        Some("category"),
        Some("biolink:NamedThing"),
        None,
        None,
        None,
        None,
        None,
    )
    .unwrap();
}

#[test]
fn test_graph_from_csv_with_edge_and_nodes() {
    let edge_path = "tests/data/edge_file.tsv";
    let node_path = "tests/data/node_file.tsv";
    for directed in &[true, false] {
        let graph = Graph::from_csv(
            edge_path,
            "subject",
            "object",
            *directed,
            Some("edge_label"),
            Some("biolink:Association"),
            Some("weight"),
            Some(1.0),
            Some(node_path),
            Some("id"),
            Some("category"),
            Some("biolink:NamedThing"),
            None,
            None,
            None,
            None,
            None,
        )
        .unwrap();
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
        let graph = Graph::from_csv(
            edge_path,
            "subject",
            "object",
            *directed,
            Some("edge_label"),
            Some("biolink:Association"),
            Some("weight"),
            Some(1.0),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
        );
        assert!(graph.is_err());
    }
}

#[test]
fn test_graph_invalid_edge_weights() {
    let edge_path = "tests/data/invalid_edge_weights.tsv";
    for directed in &[true, false] {
        assert!(Graph::from_csv(
            edge_path,
            "subject",
            "object",
            *directed,
            Some("edge_label"),
            Some("biolink:Association"),
            Some("weight"),
            Some(1.0),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None
        )
        .is_err());
        assert!(Graph::from_csv(
            edge_path,
            "subject",
            "object",
            *directed,
            None,
            None,
            Some("weight"),
            Some(1.0),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None
        )
        .is_err());
    }
}

#[test]
fn test_graph_nan_edge_weights() {
    let edge_path = "tests/data/nan_edge_weights.tsv";
    for directed in &[true, false] {
        let graph = Graph::from_csv(
            edge_path,
            "subject",
            "object",
            *directed,
            Some("edge_label"),
            Some("biolink:Association"),
            Some("weight"),
            Some(1.0),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
        );
        assert!(graph.is_err());
    }
}

#[test]
fn test_graph_inf_edge_weights() {
    let edge_path = "tests/data/infinite_edge_weights.tsv";
    for directed in &[true, false] {
        let graph: Result<Graph, String> = Graph::from_csv(
            edge_path,
            "subject",
            "object",
            *directed,
            Some("edge_label"),
            Some("biolink:Association"),
            Some("weight"),
            Some(1.0),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
        );
        assert!(graph.is_err());
    }
}

#[test]
fn test_graph_from_csv_with_edge_and_nodes_types() {
    let edge_path = "tests/data/edge_file.tsv";
    let node_path = "tests/data/node_file.tsv";
    for directed in &[true, false] {
        let graph = Graph::from_csv(
            edge_path,
            "subject",
            "object",
            *directed,
            Some("edge_label"),
            Some("biolink:Association"),
            Some("weight"),
            Some(1.0),
            Some(node_path),
            Some("id"),
            Some("category"),
            Some("biolink:NamedThing"),
            None,
            None,
            None,
            None,
            None,
        )
        .unwrap();
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
        let graph = Graph::from_csv(
            edge_path,
            "subject",
            "object",
            *directed,
            None,
            None,
            Some("weight"),
            Some(1.0),
            Some(node_path),
            Some("id"),
            Some("category"),
            Some("biolink:NamedThing"),
            None,
            None,
            None,
            None,
            None,
        )
        .unwrap();
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
        if *directed {
            assert!(graph.connected_holdout(42, 0.7).is_err());
        } else {
            let (train, validation) = graph.connected_holdout(42, 0.7).unwrap();
            assert!(train.sum(&validation).unwrap().contains(&graph));
            assert!(graph.components_holdout(35, 5).is_ok());
        }
        let _ = graph.report();
    }
}
