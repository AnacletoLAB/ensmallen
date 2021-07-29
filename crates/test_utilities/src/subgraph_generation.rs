use graph::Graph;
use shared::types::*;

pub fn test_subgraph_generation(graph: &mut Graph, verbose: Option<bool>) -> Result<()> {
    let expected_nodes = graph.get_connected_nodes_number() / 10;
    let subgraph = graph.random_subgraph(expected_nodes, None, verbose)?;
    assert!(subgraph.overlaps(&graph)?);
    assert!(subgraph.get_connected_nodes_number() <= expected_nodes + 1);
    Ok(())
}