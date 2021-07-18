extern crate graph;
use graph::*;

#[test]
fn test_chain_graph() -> Result<()> {
    let mut chain_graph = Graph::generate_chain_graph(
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
    assert!(chain_graph.is_connected(Some(true)));
    let _ = graph::test_utilities::default_test_suite(&mut chain_graph, None);
    Ok(())
}
