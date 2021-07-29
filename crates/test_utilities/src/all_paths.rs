use graph::Graph;
use shared::types::*;
use crate::graph_properties::*;

pub fn test_all_paths(graph: &mut Graph, verbose: Option<bool>) -> Result<()> {
    // We skip this test of graph with more than 1000 nodes to avoid slowing down
    // too much the test suite.
    if graph.get_nodes_number() > 1000 {
        return Ok(());
    }
    for iteration in [None, Some(0), Some(1), Some(2)] {
        let mut unweighted_all_paths = graph.get_all_shortest_paths(iteration, verbose);
        test_graph_properties(&mut unweighted_all_paths, verbose)?;
    }

    if !graph.has_edge_weights() || graph.has_negative_edge_weights().unwrap() {
        assert!(graph
            .get_weighted_all_shortest_paths(None, None, verbose)
            .is_err());
        return Ok(());
    }

    for iteration in [None, Some(0), Some(1), Some(2)] {
        let mut weighted_all_paths = graph
            .get_weighted_all_shortest_paths(iteration, None, verbose)
            .unwrap();
        test_graph_properties(&mut weighted_all_paths, verbose)?;
    }

    Ok(())
}