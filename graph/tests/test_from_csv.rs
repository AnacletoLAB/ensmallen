extern crate graph;
use graph::graph::Graph;
use std::fs::File;
use linecount::count_lines;

#[test]
fn test_graph_from_csv_edge_only() {
    let path = "tests/data/edge_file.tsv";
    for directed in &[true, false]{
        let graph = Graph::from_csv(
            &path,
            "subject",
            "object",
            *directed,
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
        );
        assert_eq!(graph.get_edge_types_number(), 0);
        assert_eq!(graph.get_node_types_number(), 0);
        let lines: usize = count_lines(File::open(path).unwrap()).unwrap();
        if *directed{
            assert_eq!(lines, graph.get_edges_number());
        }
        // TODO! Make more tests on the walks!
        let _walks = graph.walk(10, 10, Some(0), Some(0.5), Some(2.0), Some(3.0), Some(4.0));
    }
}



#[test]
fn test_graph_from_csv_edge_types() {
    let path = "tests/data/edge_file.tsv";
    for directed in &[true, false]{
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
            None
        );
        let lines: usize = count_lines(File::open(path).unwrap()).unwrap();
        if *directed{
            assert_eq!(lines, graph.get_edges_number());
        }
        assert_eq!(graph.get_edge_types_number(), 3);
        assert_eq!(graph.get_node_types_number(), 0);
        graph.walk(10, 10, Some(0), Some(0.5), Some(2.0), Some(3.0), Some(4.0));
    }
}


#[test]
#[should_panic]
fn test_walk_wrong_return_weights_parameter() {
    let path = "tests/data/edge_file.tsv";
    Graph::from_csv(
        &path,
        "subject",
        "object",
        true,
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
        None
    ).walk(10, 10, Some(0), Some(0.0), Some(2.0), Some(3.0), Some(4.0));
}

#[test]
#[should_panic]
fn test_walk_wrong_explore_weight_parameter() {
    let path = "tests/data/edge_file.tsv";
    Graph::from_csv(
        &path,
        "subject",
        "object",
        true,
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
        None
    ).walk(10, 10, Some(0), Some(1.0), Some(0.0), Some(3.0), Some(4.0));
}

#[test]
#[should_panic]
fn test_walk_wrong_change_node_type_weight_parameter() {
    let path = "tests/data/edge_file.tsv";
    Graph::from_csv(
        &path,
        "subject",
        "object",
        true,
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
        None
    ).walk(10, 10, Some(0), Some(1.0), Some(1.0), Some(0.0), Some(4.0));
}

#[test]
#[should_panic]
fn test_walk_wrong_change_edge_type_weight_parameter() {
    let path = "tests/data/edge_file.tsv";
    Graph::from_csv(
        &path,
        "subject",
        "object",
        true,
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
        None
    ).walk(10, 10, Some(0), Some(1.0), Some(1.0), Some(1.0), Some(0.0));
}

#[test]
#[should_panic]
fn test_graph_from_csv_weights_panic() {
    let path = "tests/data/zero_weights.tsv";
    Graph::from_csv(
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
        None
    );
}

#[test]
#[should_panic]
fn test_graph_from_csv_duplicated_edges_panic() {
    let path = "tests/data/duplicated_edge.tsv";
    Graph::from_csv(
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
        None
    );
}

#[test]
#[should_panic]
fn test_graph_from_csv_duplicated_edges_without_label_panic() {
    let path = "tests/data/duplicated_edge.tsv";
    Graph::from_csv(
        path,
        "subject",
        "object",
        true,
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
        None
    );
}

#[test]
#[should_panic]
fn test_graph_from_csv_duplicated_nodes_panic() {
    let edge_path = "tests/data/edge_file.tsv";
    let node_path = "tests/data/duplicated_node.tsv";
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
    );
}

#[test]
#[should_panic]
fn test_graph_from_csv_no_nodes_column_panic() {
    let edge_path = "tests/data/edge_file.tsv";
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
        None,
        Some("category"),
        Some("biolink:NamedThing"),
        None,
        None,
        None
    );
}


#[test]
#[should_panic]
fn test_graph_from_csv_no_node_types_column_panic() {
    let edge_path = "tests/data/edge_file.tsv";
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
        None,
        Some("biolink:NamedThing"),
        None,
        None,
        None
    );
}


#[test]
#[should_panic]
fn test_graph_from_csv_no_default_node_types_panic() {
    let edge_path = "tests/data/edge_file.tsv";
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
        None,
        None,
        None,
        None
    );
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
        None
    );
}

#[test]
fn test_graph_from_csv_with_edge_and_nodes() {
    let edge_path = "tests/data/edge_file.tsv";
    let node_path = "tests/data/node_file.tsv";
    for directed in &[true, false]{
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
            None
        );
        let edge_lines: usize = count_lines(File::open(edge_path).unwrap()).unwrap();
        if *directed{
            assert_eq!(edge_lines, graph.get_edges_number());
        }
        assert_eq!(graph.get_edge_types_number(), 3);
        assert_eq!(graph.get_node_types_number(), 3);
        graph.walk(10, 10, Some(0), Some(0.5), Some(2.0), Some(3.0), Some(4.0));
    };
}

#[test]
fn test_graph_from_csv_with_edge_and_nodes_types() {
    let edge_path = "tests/data/edge_file.tsv";
    let node_path = "tests/data/node_file.tsv";
    for directed in &[true, false]{
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
        );
        let edge_lines: usize = count_lines(File::open(edge_path).unwrap()).unwrap();
        if *directed{
            assert_eq!(edge_lines, graph.get_edges_number());
        }
        assert_eq!(graph.get_edge_types_number(), 3);
        graph.walk(10, 10, Some(0), Some(0.5), Some(2.0), Some(3.0), Some(4.0));
    };
}

#[test]
fn test_graph_from_csv_het() {
    let edge_path = "tests/data/het_graph_edges.tsv";
    let node_path = "tests/data/het_graph_nodes.tsv";
    for directed in &[true, false]{
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
        );
        let edge_lines: usize = count_lines(File::open(edge_path).unwrap()).unwrap();
        if *directed{
            assert_eq!(edge_lines, graph.get_edges_number());
        }
        graph.walk(10, 10, Some(0), Some(0.5), Some(2.0), Some(3.0), Some(4.0));
    };
}

