extern crate graph;
use graph::*;

#[test]
fn test_sum() {
    let graph = FromCsvBuilder::new(
        "tests/data/ppi.tsv",
        "subject", 
        "object", 
        false, 
        None
    ).unwrap().build().unwrap();
    let (train, validation) = graph.connected_holdout(42, 0.7).unwrap();
    let recomposed = train.sum(&validation).unwrap();
    assert!(recomposed.contains(&graph));
    assert!(graph.contains(&recomposed));
}
