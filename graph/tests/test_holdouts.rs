extern crate graph;
use graph::graph::Graph;
use indicatif::{ProgressIterator};

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
        for seed in vec![0, 67, 34567, 6786757].iter().progress(){
            for percentange in vec![0.3, 0.5, 0.8, 0.9].iter(){
                let (train, validation) = graph.holdout(*seed as usize, *percentange as f64).unwrap();
                assert!(train.cooccurence_matrix(10, None, None, None, Some(0.5), Some(2.0), Some(3.0), Some(4.0), Some(false)).is_ok());
                assert!(train.skipgrams(0, 128, 80, None, None, None, None, None, None, None, None, None).is_ok());
                assert!(train.walk(80, Some(1), Some(0), None, None, None, None, None, None, Some(false)).is_ok());
                assert!(validation.cooccurence_matrix(10, None, None, None, Some(0.5), Some(2.0), Some(3.0), Some(4.0), Some(false)).is_ok());
                assert!(validation.skipgrams(0, 128, 80, None, None, None, None, None, None, None, None, None).is_ok());
                assert!(validation.walk(80, Some(1), Some(0), None, None, None, None, None, None, Some(false)).is_ok());
                assert!(train.link_prediction(128, None, None, None).is_ok());
                assert!(train.link_prediction(128, None, Some(&validation), None).is_ok());
                assert!(train.link_prediction(128, Some(2.0), Some(&validation), None).is_ok());
                assert!(validation.link_prediction(128, Some(2.0), Some(&train), None).is_ok());
                assert!(validation.link_prediction(128, Some(0.5), Some(&train), None).is_ok());
                assert_eq!(train.get_nodes_number(), graph.get_nodes_number());
                assert_eq!(validation.get_nodes_number(), graph.get_nodes_number());
                assert!(graph.contains(&train));
                assert!(graph.contains(&validation));
                assert!(train.overlaps(&graph));
                assert!(validation.overlaps(&graph));
                assert!(!validation.overlaps(&train));
                assert!(!train.overlaps(&validation));
            }
        }
    }
}