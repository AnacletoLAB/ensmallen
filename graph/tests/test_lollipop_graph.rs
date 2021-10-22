extern crate graph;
use graph::*;

#[test]
fn test_lollipop_graph() -> Result<()> {
    let mut lollipop_graph = Graph::generate_lollipop_graph(
        None,
        Some(100), // 9900 edges
        Some(100), // 198 edges
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
    )
    .unwrap();
    assert!(lollipop_graph.is_connected(Some(true)));
    let _ = graph::test_utilities::default_test_suite(&mut lollipop_graph, None);
    Ok(())
}
