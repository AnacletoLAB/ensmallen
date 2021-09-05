extern crate graph;

use graph::build_empty_graph;

#[test]
fn test_empty_graph() -> Result<(), String> {
    for directed in &[true, false] {
        let mut empty_graph = build_empty_graph(*directed, "Graph")?;
        let _ = graph::test_utilities::default_test_suite(&mut empty_graph, None);
    }
    Ok(())
}
