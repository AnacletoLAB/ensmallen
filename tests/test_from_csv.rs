extern crate ensmallen_graph;
use ensmallen_graph::graph::Graph;

#[test]
fn test_graph_from_csv_edge_only() {
    let graph = Graph::from_csv(
        "tests/data/edge_file.tsv".to_string(),
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
}
