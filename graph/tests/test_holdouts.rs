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
        assert!(train.cooccurence_matrix(10, None, None, None, Some(0.5), Some(2.0), Some(3.0), Some(4.0), Some(false)).is_ok());
        assert!(train.skipgrams(0, 128, 80, None, None, None, None, None, None, None, None, None).is_ok());
        assert!(train.walk(80, Some(1), Some(0), None, None, None, None, None, None, Some(false)).is_ok());
        assert!(validation.cooccurence_matrix(10, None, None, None, Some(0.5), Some(2.0), Some(3.0), Some(4.0), Some(false)).is_ok());
        assert!(validation.skipgrams(0, 128, 80, None, None, None, None, None, None, None, None, None).is_ok());
        assert!(validation.walk(80, Some(1), Some(0), None, None, None, None, None, None, Some(false)).is_ok());
        train.link_prediction(128, None, None, None);
        train.link_prediction(128, None, Some(&validation), None);
        train.link_prediction(128, Some(2.0), Some(&validation), None);
        validation.link_prediction(128, Some(2.0), Some(&train), None);
        validation.link_prediction(128, Some(0.5), Some(&train), None);
    }
}