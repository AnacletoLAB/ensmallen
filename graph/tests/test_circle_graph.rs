extern crate graph;
use graph::*;

#[test]
fn test_circle_graph() -> Result<()> {
    let mut circle_graph =
        Graph::generate_circle_graph(None, Some(100), None, None, None, None, None, None).unwrap();
    assert!(circle_graph.is_connected(Some(true)));
    let chains = circle_graph.get_chains(None, None).unwrap();
    assert!(chains.is_empty());
    let circles = circle_graph.get_circles(None, None).unwrap();
    assert_eq!(circles.len(), 1);
    assert_eq!(circles[0].len(), 100);
    assert_eq!(circles[0].get_root_node_id(), 0);
    let _ = graph::test_utilities::default_test_suite(&mut circle_graph, None);
    Ok(())
}
