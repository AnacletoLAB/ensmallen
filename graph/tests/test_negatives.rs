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
    let negatives = graph.sample_negatives(42, 10000, false).unwrap();
    assert_eq!(negatives.get_edges_number(), 10000);
    let _ = graph.sample_negatives(42, 10000, true).unwrap();
}