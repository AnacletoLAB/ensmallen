extern crate graph;
use graph::*;

#[test]
fn test_graph_clone() {
    let path = "tests/data/edge_file.tsv";
    for directed in &[true, false] {
        let graph = FromCsvBuilder::new(path, "subject", "object", *directed, None)
            .unwrap()
            .build()
            .unwrap();
        // it's not a great test but the getters are automatically derived
        // so there shouldn't be a lot of problems
        let _ = graph.clone();
    }
}
