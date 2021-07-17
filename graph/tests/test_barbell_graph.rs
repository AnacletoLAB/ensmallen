extern crate graph;
use graph::*;

#[test]
fn test_barbell_graph() -> Result<()> {
    let mut barbell_graph = Graph::generate_barbell_graph(
        None,
        Some(100), // 9900 edges
        Some(100), // 198 edges
        Some(100), // 9900 edges
        None,
        None,
        None,
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
    assert!(barbell_graph.is_connected(Some(true)));
    let _ = graph::test_utilities::default_test_suite(&mut barbell_graph, None);
    Ok(())
}
