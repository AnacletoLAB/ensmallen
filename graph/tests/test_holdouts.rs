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
        let _cooccurrence = train.cooccurence_matrix(10, None, None, None, Some(0.5), Some(2.0), Some(3.0), Some(4.0), Some(false));
        let _skipgrams = train.skipgrams(0, 128, 80, None, None, None, None, None, None, None, None, None);
        let _ = train.walk(80, Some(1), Some(0), None, None, None, None, None, None, Some(false));
        let _cooccurrence = validation.cooccurence_matrix(10, None, None, None, Some(0.5), Some(2.0), Some(3.0), Some(4.0), Some(false));
        let _skipgrams = validation.skipgrams(0, 128, 80, None, None, None, None, None, None, None, None, None);
        let _ = validation.walk(80, Some(1), Some(0), None, None, None, None, None, None, Some(false));
    }
}