extern crate graph;
use graph::graph::Graph;

#[test]
fn test_random_walk() {
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
            None,
        ).unwrap();
        graph.walk(10, None, None, None, Some(0),  Some(0.5), Some(2.0), Some(3.0), Some(4.0), None).unwrap();
    };
}
