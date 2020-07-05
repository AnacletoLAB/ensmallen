extern crate graph;
use graph::graph::Graph;


#[test]
fn test_negatives() {
    let graph = Graph::from_csv(
        "tests/data/ppi.tsv",
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
    ).unwrap();
    graph.sample_negatives(42, 100).unwrap();
}