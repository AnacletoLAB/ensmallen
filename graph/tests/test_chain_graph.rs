extern crate graph;
use graph::*;

#[test]
fn test_chain_graph() -> Result<()> {
    let number_of_nodes = 100;
    let mut chain_graph =
        Graph::generate_chain_graph(None, Some(number_of_nodes), None, None, None, None, None, None)
            .unwrap();
    assert_eq!(chain_graph.get_number_of_nodes(), number_of_nodes);
    assert!(chain_graph.is_connected(Some(true)));
    let circles = chain_graph.get_circles(None, None).unwrap();
    assert!(circles.is_empty());
    let chains = chain_graph.get_chains(None, None).unwrap();
    assert_eq!(chains.len(), 0);
    let _ = graph::test_utilities::default_test_suite(&mut chain_graph, None);
    Ok(())
}
