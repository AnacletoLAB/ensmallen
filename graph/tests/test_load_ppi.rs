extern crate graph;
use graph::graph::Graph;

#[test]
fn test_load_ppi() {
    let edge_path = "tests/data/ppi.tsv";
    let node_path = "tests/data/ppi_nodes.tsv";
    for directed in &[true, false]{
        let _ = Graph::from_csv(
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
            None
        ).unwrap();
    };
}