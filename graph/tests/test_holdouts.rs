extern crate graph;
use graph::graph::Graph;

#[test]
fn test_holdout() {
    let path = "tests/data/ppi.tsv";
    for directed in &[false, true] {
        let graph = Graph::from_csv(
            &path, "subject", "object", *directed, None, None, None, None, None, None, None, None,
            None, None, None, None, None,
        )
        .unwrap();
        let (train, validation) = graph.connected_holdout(42, 0.7).unwrap();
        assert!(graph.contains(&train));
        assert!(graph.contains(&validation));
        assert!(train.overlaps(&graph));
        assert!(validation.overlaps(&graph));
        assert!(!validation.overlaps(&train));
        assert!(!train.overlaps(&validation));
        let (train, validation) = graph.random_holdout(42, 0.7).unwrap();
        assert!(graph.contains(&train));
        assert!(graph.contains(&validation));
        assert!(train.overlaps(&graph));
        assert!(validation.overlaps(&graph));
        assert!(!validation.overlaps(&train));
        assert!(!train.overlaps(&validation));
    }
}

#[test]
fn test_holdout_determinism() {
    let path = "tests/data/ppi.tsv";
    for directed in &[false, true] {
        let graph = Graph::from_csv(
            &path, "subject", "object", *directed, None, None, None, None, None, None, None, None,
            None, None, None, None, None,
        )
        .unwrap();
        let (train1, test1) = graph.connected_holdout(35, 0.8).unwrap();
        let (train2, test2) = graph.connected_holdout(35, 0.8).unwrap();
        assert_eq!(train1, train2);
        assert_eq!(test1, test2);
        let (train1, test1) = graph.random_holdout(35, 0.8).unwrap();
        let (train2, test2) = graph.random_holdout(35, 0.8).unwrap();
        assert_eq!(train1, train2);
        assert_eq!(test1, test2);
    }
}
