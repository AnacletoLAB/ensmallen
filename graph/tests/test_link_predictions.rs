extern crate graph;
use graph::graph::Graph;

#[test]
fn test_link_predictions() {
    let edge_path = "tests/data/edge_file.tsv";
    let node_path = "tests/data/node_file.tsv";
    let graph = Graph::from_csv(
        edge_path,
        "subject",
        "object",
        false,
        Some("edge_label"),
        Some("biolink:Association"),
        Some("weight"),
        Some(1.0),
        Some(node_path),
        Some("id"),
        Some("category"),
        Some("biolink:NamedThing"),
        None,
        None,
        None,
        None,
        None
    ).unwrap();
    graph.walk(10, None, None, None, Some(0),  Some(0.5), Some(2.0), Some(3.0), Some(4.0), None).unwrap();

    for i in 0..20 {
        let (edges, labels) = graph.link_prediction(
            i,
            100000,
            Some(1000.0),
            None,
            Some(true)
        ).unwrap();   
        println!("{}", labels.iter().position(|&r| r == 1u8).unwrap());
        println!("{}", labels.iter().position(|&r| r == 0u8).unwrap());

        assert!(
            edges.iter().all(
                |edge|
                    edge[0] != edge[1]
            )
        )
    }
}
