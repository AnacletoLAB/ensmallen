extern crate graph;
use graph::graph::Graph;

#[test]
fn test_link_predictions() {
    let graph = Graph::from_csv(
        "tests/data/ppi/edges.tsv",
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

    assert!(graph
        .link_prediction(0, 100000, Some(-1.0), None, Some(true))
        .is_err());

    assert!(graph
        .link_prediction(0, 100000, Some(f64::INFINITY), None, Some(true))
        .is_err());

    let (train, valid) = graph.random_holdout(42, 0.7).unwrap();

    for t in [true, false].iter() {
        for i in 0..20 {
            for graph_to_avoid in [None, Some(&valid)].iter(){
                let (edges, labels) = train
                    .link_prediction(i, 100000, Some(1000.0), *graph_to_avoid, Some(*t))
                    .unwrap();
                assert!(graph
                    .link_prediction(i, 100000, Some(0.0), None, Some(*t))
                    .is_ok());
                assert!(labels.iter().any(|label| *label == 1u8));
                assert!(labels.iter().any(|label| *label == 0u8));
                if *t {
                    assert!(edges.iter().all(|edge| edge[0] != edge[1]));
                }
            }
        }
    }
}
