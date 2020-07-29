extern crate graph;
use graph::*;

#[test]
fn test_negatives() {
    let graph = FromCsvBuilder::new("tests/data/ppi/edges.tsv", "subject", "object", false, None)
        .unwrap()
        .build()
        .unwrap();
    let negatives = graph.sample_negatives(42, 10000, false).unwrap();
    let negatives2 = graph.sample_negatives(42, 10000, false).unwrap();
    assert!(!negatives.overlaps(&graph));
    assert!(!graph.overlaps(&negatives));
    assert_eq!(negatives.get_edges_number(), 10000);
    assert_eq!(negatives, negatives2);
    let _ = graph.sample_negatives(42, 10000, true).unwrap();
}
