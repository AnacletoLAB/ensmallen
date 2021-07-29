use graph::Graph;
use shared::types::*;
use crate::utils::*;

pub fn test_graph_removes(graph: &mut Graph, _verbose: Option<bool>) -> Result<()> {
    let without_edge_types = graph.remove_edge_types()?;
    validate_vocabularies(&without_edge_types);
    assert!(!without_edge_types.has_edge_types());
    assert_eq!(
        without_edge_types.has_edge_weights(),
        graph.has_edge_weights()
    );
    assert_eq!(without_edge_types.node_types, graph.node_types);
    if !graph.is_multigraph() {
        assert_eq!(
            without_edge_types.get_unique_edges_number(),
            graph.get_unique_edges_number(),
            concat!(
                "Number of unique edges does not match in graph without edge types.\n",
                "The report of the original graph is \n{:?}\n",
                "The report of the graph without edge types is \n{:?}",
            ),
            graph.textual_report(),
            without_edge_types.textual_report()
        );
        assert_eq!(
            without_edge_types.get_unique_selfloops_number(),
            graph.get_unique_selfloops_number(),
            "Number of unique self loops does not match in graph without edge types."
        );
    }
    assert_eq!(without_edge_types.nodes, graph.nodes);
    let without_node_types = graph.remove_node_types()?;
    validate_vocabularies(&without_node_types);
    assert!(!without_node_types.has_node_types());
    assert_eq!(
        graph.is_multigraph(),
        without_node_types.is_multigraph(),
        "If the original graph is a multigraph, the removal of node types should not change that."
    );
    assert_eq!(
        without_node_types.weights,
        graph.weights,
        concat!(
            "We expected the weights not to change when removig node types.",
            "\nThe report of the original graph is {:?}.",
            "\nThe report of the filtered graph is {:?}."
        ),
        graph.textual_report(),
        without_node_types.textual_report()
    );
    assert_eq!(without_node_types.has_selfloops(), graph.has_selfloops());
    assert_eq!(without_node_types.nodes, graph.nodes);
    let without_weights = graph.remove_edge_weights()?;
    validate_vocabularies(&without_weights);
    assert!(!without_weights.has_edge_weights());
    assert_eq!(without_weights.node_types, graph.node_types);
    assert_eq!(without_weights.has_selfloops(), graph.has_selfloops());
    assert_eq!(without_weights.nodes, graph.nodes);

    Ok(())
}
