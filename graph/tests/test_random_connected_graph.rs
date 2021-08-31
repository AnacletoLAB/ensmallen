extern crate graph;
use graph::*;

#[test]
fn test_random_connected_graph() -> Result<()> {
    let mut random_connected_graph = Graph::generate_random_connected_graph(
        None,
        None,
        None,
        None,
        Some(100),
        None,
        None,
        None,
        None,
        None,
        None,
    )
    .unwrap();
    assert!(random_connected_graph.is_connected(Some(true)));
    let _ = graph::test_utilities::default_test_suite(&mut random_connected_graph, None);
    Ok(())
}
