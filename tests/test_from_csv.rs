extern crate ensmallen_graph;
use ensmallen_graph::graph::Graph;

#[test]
fn test_check_consistent_lines() {
    Graph::from_csv("tests/data/edge_file.tsv", "subject", "object", true);
}
