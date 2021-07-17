extern crate graph;
use graph::*;

#[test]
fn test_barbell_graph() -> Result<()> {
    let mut barbell_graph = Graph::generate_barbell_graph(
        None,
        Some(100),
        Some(100),
        Some(100),
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
