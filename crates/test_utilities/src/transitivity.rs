use graph::Graph;
use shared::types::*;
use crate::graph_properties::test_graph_properties;

pub fn test_transitivity(graph: &mut Graph, verbose: Option<bool>) -> Result<()> {
    // We skip this test of graph with more than 1000 nodes to avoid slowing down
    // too much the test suite.
    if graph.get_nodes_number() > 1000 {
        return Ok(());
    }
    if !graph.has_edge_weights() && !graph.has_edge_types() {
        // We define the 0-th iteration of transitive closure as the graph itself
        assert_eq!(
            graph.clone(),
            graph.get_transitive_closure(Some(0), verbose)
        );
        // We define the first iteration of transitive closure as the graph itself
        let graph_with_selfloops = graph.add_selfloops(None, None).unwrap();
        assert_eq!(
            graph_with_selfloops,
            graph_with_selfloops.get_transitive_closure(Some(1), verbose),
            concat!(
                "We expected the original graph to equal to the graph obtained after ",
                "a single iteration of transitive closure, but they are different.\n",
                "The report of the first graph is: \n {}\n",
                "The report of the second graph is: \n {}\n",
            ),
            graph_with_selfloops.textual_report(),
            graph_with_selfloops
                .get_transitive_closure(Some(1), verbose)
                .textual_report(),
        );
        // Doing multiple iterations should be equal to doing the same iteration multiple times
        let four_iterations = graph_with_selfloops.get_transitive_closure(Some(4), verbose);
        let two_times_two = graph_with_selfloops
            .get_transitive_closure(Some(2), verbose)
            .get_transitive_closure(Some(2), verbose);
        assert_eq!(
            four_iterations,
            two_times_two,
            concat!(
                "We expected the graph after 4 transitive closures to be ",
                "equal to the graph after two times two transitive closures.\n",
                "The to_dot of the first graph is: \n {}\n",
                "The to_dot of the second graph is: \n {}\n",
            ),
            four_iterations.to_dot(Some(false)),
            two_times_two.to_dot(Some(false))
        );
    }
    let mut transitive_closure = graph.get_transitive_closure(None, verbose);
    let connected_components = graph.get_node_connected_component_ids(verbose);
    if !graph.is_directed() {
        for (src_node_id, src_component_id) in connected_components.iter().cloned().enumerate() {
            if unsafe { graph.is_unchecked_singleton_from_node_id(src_node_id as NodeT) } {
                continue;
            }
            for (dst_node_id, dst_component_id) in connected_components.iter().cloned().enumerate()
            {
                assert_eq!(
                    transitive_closure
                        .has_edge_from_node_ids(src_node_id as NodeT, dst_node_id as NodeT),
                    src_component_id == dst_component_id,
                    concat!(
                        "In an undirected graph, the transitive closure of the graph should ",
                        "contain an edge between all nodes in the same component, but ",
                        "the node {} and {} have as component IDs {} and {} respectively, ",
                        "and the test has edge has returned {}."
                    ),
                    src_node_id,
                    dst_node_id,
                    src_component_id,
                    dst_component_id,
                    transitive_closure
                        .has_edge_from_node_ids(src_node_id as NodeT, dst_node_id as NodeT)
                );
            }
        }
    }
    test_graph_properties(&mut transitive_closure, verbose)?;

    Ok(())
}
