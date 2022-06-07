extern crate graph;
use graph::*;

#[test]
fn test_star_graph() -> Result<()> {
    let mut star_graph = Graph::generate_star_graph(
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
    assert!(star_graph.is_connected(Some(true)));
    let _ = graph::test_utilities::default_test_suite(&mut star_graph, None);
    Ok(())
}
