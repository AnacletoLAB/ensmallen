extern crate graph;
use graph::*;

#[test]
fn test_wheel_graph() -> Result<()> {
    let mut wheel_graph = Graph::generate_wheel_graph(
        None,
        Some(100), // 9900 edges
        None,
        None,
        None,
        None,
        None,
        None,
    )
    .unwrap();
    assert!(wheel_graph.is_connected(Some(true)));
    let _ = graph::test_utilities::default_test_suite(&mut wheel_graph, None);
    Ok(())
}
