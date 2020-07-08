extern crate graph;
use graph::graph::Graph;

#[test]
fn test_sum() {
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
    let (train, validation) = graph.connected_holdout(42, 0.7).unwrap();
    let recomposed = train.sum(&validation).unwrap();
    assert!(recomposed.contains(&graph));
    assert!(graph.contains(&recomposed));
}