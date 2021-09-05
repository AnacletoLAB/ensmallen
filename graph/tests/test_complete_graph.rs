extern crate graph;
use graph::*;

#[test]
fn test_complete_graph() -> Result<()> {
    let mut complete_graph =
        Graph::generate_complete_graph(None, Some(10), None, None, None, None, None, None).unwrap();
    assert!(complete_graph.is_connected(Some(true)));
    let _ = graph::test_utilities::default_test_suite(&mut complete_graph, None);
    Ok(())
}
