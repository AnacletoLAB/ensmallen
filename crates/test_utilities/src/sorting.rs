use graph::Graph;
use shared::types::*;

pub fn test_sorting(graph: &mut Graph, _verbose: Option<bool>) -> Result<()> {
    let sorted_increasing = graph.sort_by_increasing_outbound_node_degree();
    assert!(sorted_increasing.has_nodes_sorted_by_increasing_outbound_node_degree());
    let sorted_decreasing = graph.sort_by_decreasing_outbound_node_degree();
    assert!(sorted_decreasing.has_nodes_sorted_by_decreasing_outbound_node_degree());
    let sorted_lexicographical = graph.sort_by_node_lexicographic_order();
    assert!(sorted_lexicographical.has_nodes_sorted_by_lexicographic_order());

    Ok(())
}