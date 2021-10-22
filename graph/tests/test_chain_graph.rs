extern crate graph;
use graph::*;

#[test]
fn test_chain_graph() -> Result<()> {
    let nodes_number = 100;
    let mut chain_graph =
        Graph::generate_chain_graph(None, Some(nodes_number), None, None, None, None, None, None)
            .unwrap();
    assert_eq!(chain_graph.get_nodes_number(), nodes_number);
    assert!(chain_graph.is_connected(Some(true)));
    let circles = chain_graph.get_circles(None, None, None).unwrap();
    assert!(circles.is_empty());
    let chains = chain_graph.get_chains(None, None, None).unwrap();
    assert_eq!(chains.len(), 1);
    assert_eq!(chains[0].get_root_node_id(), 0);
    assert_eq!(chains[0].len(), nodes_number);
    assert_eq!(
        chains[0].get_chain_node_ids(),
        (0..nodes_number).collect::<Vec<NodeT>>()
    );
    let _ = graph::test_utilities::default_test_suite(&mut chain_graph, None);
    Ok(())
}
