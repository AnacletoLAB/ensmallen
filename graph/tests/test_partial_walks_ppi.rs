extern crate graph;
use graph::*;
use graph::{SingleWalkParameters, WalkWeights, WalksParameters};

#[test]
fn test_partial_walks_ppi() {
    let edge_path = "tests/data/ppi/edges.tsv";
    let node_path = "tests/data/ppi/nodes.tsv";
    let graph = FromCsvBuilder::new(
        edge_path,
        "subject", 
        "object", 
        false, 
        None
    ).unwrap()
    .set_weights("weight", Some(1.0))
    .load_nodes_csv(
        node_path, 
        "id", 
        "category",
        Some("biolink:NamedThing"), 
        None, 
        None
    ).unwrap()
    .build().unwrap()
    .random_subgraph(0, 10000).unwrap();
    let walk_parameters = WalksParameters::new(
        SingleWalkParameters::new(100, WalkWeights::default()).unwrap(),
        0,
        graph.get_not_trap_nodes_number(),
    )
    .unwrap()
    .set_dense_nodes_mapping(Some(graph.get_dense_nodes_mapping()));
    assert!(graph.walk(&walk_parameters).is_ok());
}
