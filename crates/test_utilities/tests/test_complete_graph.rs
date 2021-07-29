use shared::*;
use graph::Graph;
use test_utilities::*;

#[test]
fn test_complete_graph() -> Result<()> {
    let mut complete_graph =
        Graph::generate_complete_graph(None, Some(10), None, None, None, None, None, None).unwrap();
    assert!(complete_graph.is_connected(Some(true)));
    let _ = default_test_suite(&mut complete_graph, None);
    Ok(())
}
