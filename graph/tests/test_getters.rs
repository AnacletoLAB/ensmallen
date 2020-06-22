extern crate graph;
use graph::graph::Graph;
use std::fs::File;
use linecount::count_lines;

#[test]
fn test_getters() {
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
        graph.sources();
        graph.destinations();         
        graph.nodes_mapping();
        graph.nodes_reverse_mapping();         
        graph.unique_edges();
        graph.outbounds();         
        graph.weights(); 
        graph.node_types(); 
        graph.node_types_mapping();
        graph.node_types_reverse_mapping();
        graph.edge_types();
        graph.edge_types_mapping();
        graph.edge_types_reverse_mapping();
    }
}