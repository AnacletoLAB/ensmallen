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
        None
    ).unwrap();

    for t in [true, false].iter() {
         for i in 0..20 {
            let (edges, labels) = graph.link_prediction(
                i,
                100000,
                Some(1000.0),
                None,
                Some(*t)
            ).unwrap();   
            assert!(
                labels.iter().any(
                    |label|
                        *label == 1u8
                )
            );
            assert!(
                labels.iter().any(
                    |label|
                        *label == 0u8
                )
            );
            if *t {
                assert!(
                    edges.iter().all(
                        |edge|
                            edge[0] != edge[1]
                    )
                );
            }
        }
    }
}
