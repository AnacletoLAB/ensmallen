use graph::Graph;
use shared::types::*;
use crate::graph_properties::test_graph_properties;

pub fn test_selfloops(graph: &mut Graph, verbose: Option<bool>) -> Result<()> {
    assert!(!graph.drop_selfloops().has_selfloops());
    assert_eq!(
        graph.add_selfloops(None, Some(1.0)).is_ok(),
        graph.has_edge_weights()
    );
    let mut graph_with_selfloops = graph
        .add_selfloops(
            None,
            if graph.has_edge_weights() {
                Some(1.0)
            } else {
                None
            },
        )
        .unwrap();
    for node_id in graph.iter_node_ids() {
        assert!(graph_with_selfloops.has_selfloop_from_node_id(node_id));
    }
    test_graph_properties(&mut graph_with_selfloops, verbose)?;

    Ok(())
}