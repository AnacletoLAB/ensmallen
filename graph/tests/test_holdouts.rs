extern crate graph;
use graph::graph::Graph;

#[test]
fn test_holdout() {
    let path = "tests/data/ppi.tsv";
    for directed in &[false, true]{
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
            None,
            None,
            None,
            None
        ).unwrap();
        let (train, validation) = graph.holdout(42, 0.8).unwrap();
    }
}