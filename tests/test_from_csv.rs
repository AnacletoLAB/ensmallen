extern crate ensmallen_graph;
use ensmallen_graph::graph::Graph;
use std::fs::File;
use linecount::count_lines;

#[test]
fn test_graph_from_csv_edge_only() {
    let path = "tests/data/edge_file.tsv";
    let graph = Graph::from_csv(
        path.to_string(),
        "subject".to_string(),
        "object".to_string(),
        true,
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
    );
    let lines: usize = count_lines(File::open(path).unwrap()).unwrap();
    assert_eq!(lines, graph.get_edges_number());
}



#[test]
fn test_graph_from_csv_edge_types() {
    let path = "tests/data/edge_file.tsv";
    let graph = Graph::from_csv(
        path.to_string(),
        "subject".to_string(),
        "object".to_string(),
        true,
        Some("edge_label".to_string()),
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
    let lines: usize = count_lines(File::open(path).unwrap()).unwrap();
    assert_eq!(lines, graph.get_edges_number());
    assert_eq!(graph.get_edge_types_number(), 2);
}

#[test]
#[should_panic]
fn test_graph_from_csv_weights_paninc() {
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
        None,
        None,
    );
}
