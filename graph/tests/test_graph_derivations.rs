extern crate graph;
use graph::graph::Graph;
use std::fs::File;
use linecount::count_lines;

#[test]
fn test_graph_clone() {
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
        ).unwrap();
        // it's not a great test but the getters are automatically derived
        // so there shouldn't be a lot of problems
        graph.clone();
    }
}