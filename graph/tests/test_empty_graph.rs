extern crate graph;

use graph::test_utilities::*;

#[test]
fn test_empty_graph() -> Result<(), String> {
    for directed in &[true, false] {
        let mut empty_graph = load_empty_graph(*directed);
        let _ = graph::test_utilities::default_test_suite(&mut empty_graph, None);
    }
    Ok(())
}
