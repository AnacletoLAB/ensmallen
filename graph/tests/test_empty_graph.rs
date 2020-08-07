extern crate graph;
use graph::*;

#[test]
fn test_builder() {
    for directed in &[true, false] {
        let graph = Graph::builder(vec![], vec![], *directed)
            .build(None)
            .unwrap();
    }
}
