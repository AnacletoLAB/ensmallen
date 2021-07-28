extern crate graph;
use graph::*;

#[test]
fn test_circle_graph() -> Result<()> {
    let mut circle_graph =
        Graph::generate_circle_graph(None, Some(100), None, None, None, None, None, None).unwrap();
    assert!(circle_graph.is_connected(Some(true)));
    let _ = graph::test_utilities::default_test_suite(&mut circle_graph, None);
    Ok(())
}
