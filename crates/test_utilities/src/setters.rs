use graph::Graph;
use shared::types::*;


pub fn test_clone_and_setters(graph: &mut Graph, _verbose: Option<bool>) -> Result<()> {
    let mut clone = graph.clone();
    clone = clone.set_all_edge_types("TEST_SET_ALL_EDGE_TYPES")?;
    assert!(!clone.is_multigraph());
    clone = clone.set_all_node_types("TEST_SET_ALL_NODE_TYPES")?;

    assert_eq!(
        clone.get_edge_types_number().unwrap(),
        1,
        "Number of edge types of the graph is not 1."
    );
    if !graph.is_multigraph() {
        assert_eq!(
            unsafe{clone.get_unchecked_edge_count_from_edge_type_id(Some(0))},
            graph.get_directed_edges_number(),
            "Number of edges with the unique edge type does not match number of edges in the graph."
        );
    }

    assert_eq!(
        clone.get_node_types_number().unwrap(),
        1,
        "Number of node types of the graph is not 1."
    );
    assert_eq!(
        unsafe{clone.get_unchecked_node_count_from_node_type_id(Some(0))},
        graph.get_nodes_number(),
        "Number of nodes with the unique node type does not match number of nodes in the graph."
    );

    Ok(())
}
