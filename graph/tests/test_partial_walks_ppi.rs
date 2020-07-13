extern crate graph;
use graph::graph::Graph;
use graph::{WalksParameters, WalkWeights, SingleWalkParameters};


#[test]
fn test_partial_walks_ppi() {
    let edge_path = "tests/data/ppi.tsv";
    let node_path = "tests/data/ppi_nodes.tsv";
    for directed in &[true, false] {
        let graph = Graph::from_csv(
            edge_path,
            "subject",
            "object",
            *directed,
            None,
            None,
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
            None,
        )
        .unwrap().components_holdout(0, 10000).unwrap();
        let walk_parameters = WalksParameters::new(
            SingleWalkParameters::new(100, WalkWeights::default()).unwrap(),
            0,
            graph.get_not_trap_nodes_number(),
        )
        .unwrap()
        .set_dense_nodes_mapping(Some(graph.get_dense_nodes_mapping()));
        assert!(graph.walk(&walk_parameters).is_ok());
    }
}
