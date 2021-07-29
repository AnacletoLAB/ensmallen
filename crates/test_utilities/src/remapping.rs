use graph::Graph;
use shared::types::*;

pub fn test_graph_remapping(graph: &mut Graph, _verbose: Option<bool>) -> Result<()> {
    assert!(
        graph.are_nodes_remappable(&graph),
        "Graph always should be remappable to itself."
    );
    assert!(
        graph.remap_from_graph(&graph).is_ok(),
        "Graph always should be remappable to itself."
    );
    Ok(())
}