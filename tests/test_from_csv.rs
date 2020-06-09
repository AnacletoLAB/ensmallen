extern crate ensmallen_graph;
use ensmallen_graph::graph::Graph;
use std::fs::File;
use linecount::count_lines;

#[test]
fn test_graph_from_csv_edge_only() {
    let path = "tests/data/edge_file.tsv";
    for directed in &[true, false]{
        let graph = Graph::from_csv(
            path.to_string(),
            "subject".to_string(),
            "object".to_string(),
            *directed,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
        );
        let lines: usize = count_lines(File::open(path).unwrap()).unwrap();
        assert_eq!(lines, graph.get_edges_number());
        graph.walk(10, 10, 1.0, 2.0, 3.0, 4.0);
    }
}



#[test]
fn test_graph_from_csv_edge_types() {
    let path = "tests/data/edge_file.tsv";
    for directed in &[true, false]{
        let graph = Graph::from_csv(
            path.to_string(),
            "subject".to_string(),
            "object".to_string(),
            *directed,
            Some("edge_label".to_string()),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
        );
        let lines: usize = count_lines(File::open(path).unwrap()).unwrap();
        assert_eq!(lines, graph.get_edges_number());
        assert_eq!(graph.get_edge_types_number(), 2);
        graph.walk(10, 10, 1.0, 2.0, 3.0, 4.0);
    }
}

#[test]
#[should_panic]
fn test_graph_from_csv_weights_panic() {
    let path = "tests/data/zero_weights.tsv";
    Graph::from_csv(
        path.to_string(),
        "subject".to_string(),
        "object".to_string(),
        true,
        Some("edge_label".to_string()),
        Some("weight".to_string()),
        None,
        None,
        None,
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
        edge_path.to_string(),
        "subject".to_string(),
        "object".to_string(),
        true,
        Some("edge_label".to_string()),
        Some("weight".to_string()),
        Some(node_path.to_string()),
        None,
        None,
        None,
        None,
        None,
    );
}

#[test]
#[should_panic]
fn test_graph_from_csv_weird_edge_nodes() {
    let edge_path = "tests/data/edge_file_with_weird_nodes.tsv";
    let node_path = "tests/data/node_file.tsv";
    Graph::from_csv(
        edge_path.to_string(),
        "subject".to_string(),
        "object".to_string(),
        true,
        Some("edge_label".to_string()),
        Some("weight".to_string()),
        Some(node_path.to_string()),
        Some("id".to_string()),
        None,
        None,
        None,
        None,
    );
}

#[test]
fn test_graph_from_csv_with_edge_and_nodes() {
    let edge_path = "tests/data/edge_file.tsv";
    let node_path = "tests/data/node_file.tsv";
    for directed in &[true, false]{
        let graph = Graph::from_csv(
            edge_path.to_string(),
            "subject".to_string(),
            "object".to_string(),
            *directed,
            Some("edge_label".to_string()),
            Some("weight".to_string()),
            Some(node_path.to_string()),
            Some("id".to_string()),
            None,
            None,
            None,
            None,
        );
        let edge_lines: usize = count_lines(File::open(edge_path).unwrap()).unwrap();
        assert_eq!(edge_lines, graph.get_edges_number());
        assert_eq!(graph.get_edge_types_number(), 2);
        let node_lines: usize = count_lines(File::open(node_path).unwrap()).unwrap();
        assert_eq!(node_lines, graph.get_nodes_number());
    };
}