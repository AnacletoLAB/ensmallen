extern crate graph;
use graph::*;

#[test]
fn test_tarjan() {
    let graph = FromCsvBuilder::new(
        "tests/data/ppi/edges.tsv",
        "subject", 
        "object", 
        false, 
        None
    ).unwrap().build().unwrap();
    
    assert_eq!(
        graph.connected_components_number(),
        graph.strongly_connected_components().len()
    )
}
