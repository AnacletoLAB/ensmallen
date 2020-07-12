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
        let negative_edges_number = 1000;
        let negatives = graph.sample_negatives(42, negative_edges_number, false).unwrap();
        assert_eq!(negatives.get_edges_number(), negative_edges_number);
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
        assert!(train1.sum(&test1).unwrap().contains(&graph));
        if *directed {
            // Testing error of singleton components
            assert!(graph.components_holdout(35, 10).is_err());
            assert!(graph
                .components_holdout(35, graph.get_edges_number() * 2)
                .is_err());
        } else {
            // Test determinism
            let graph1 = graph.components_holdout(35, 1000).unwrap();
            let graph2 = graph.components_holdout(35, 1000).unwrap();
            assert_eq!(graph1, graph2);
        }
    }
}
