extern crate graph;
use graph::graph::Graph;

#[test]
fn test_tarjan() {
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
        None,
    )
    .unwrap();
    assert_eq!(
        graph.connected_components_number(),
        graph.strongly_connected_components().len()
    )
}
