use graph::Graph;
use shared::types::*;

pub fn test_dijkstra(graph: &mut Graph, _verbose: Option<bool>) -> Result<()> {
    // We avoid running this test on too big graphs so to avoid slowing down the test suite
    if graph.get_nodes_number() > 100 {
        return Ok(());
    }
    // If the graph is empty the other tests on Dijkstra make little sense
    if !graph.has_nodes() {
        assert!(graph
            .get_dijkstra_from_node_ids(0, None, None, None, None, None)
            .is_err());
        return Ok(());
    }
    // Dijkstra on unweighted graphs does not make sense
    // Dijkstra on weighted graphs with negative weights does not make sense
    if !graph.has_edge_weights() || graph.has_negative_edge_weights().unwrap() {
        assert!(graph
            .get_dijkstra_from_node_names(
                unsafe { graph.get_unchecked_node_name_from_node_id(0).as_ref() },
                None,
                None,
                None,
                None,
                None
            )
            .is_err());
        return Ok(());
    }
    // Dijkstra on an unweighted graph gives simmetric results.
    if !graph.is_directed() {
        for use_edge_weights_as_probabilities in [true, false] {
            if use_edge_weights_as_probabilities
                && !graph.has_edge_weights_representing_probabilities().unwrap()
            {
                continue;
            }
            graph.iter_node_ids().for_each(|src_node_id| {
                graph.iter_node_ids().for_each(|dst_node_id| unsafe {
                    // Check that the obtained results are simmetric
                    let (src_to_dst_distance, src_to_dst) = graph
                        .get_unchecked_weighted_minimum_path_node_ids_from_node_ids(
                            src_node_id,
                            dst_node_id,
                            Some(use_edge_weights_as_probabilities),
                            None,
                        );
                    let (dst_to_src_distance, dst_to_src) = graph
                        .get_unchecked_weighted_minimum_path_node_ids_from_node_ids(
                            dst_node_id,
                            src_node_id,
                            Some(use_edge_weights_as_probabilities),
                            None,
                        );
                    let src_to_dst_distance = src_to_dst_distance as WeightT;
                    let dst_to_src_distance = dst_to_src_distance as WeightT;
                    assert!(
                        // We need both checks because both distances
                        // my be infinite, and therefore the epsilon check
                        // may not be enough.
                        src_to_dst_distance.is_infinite() && dst_to_src_distance.is_infinite()
                            || (src_to_dst_distance - dst_to_src_distance).abs() < WeightT::EPSILON,
                        concat!(
                            "The path from source to destination has distance {} ",
                            "while the distance from destination to source has ",
                            "destination {}. The path from source to destination ",
                            "is {:?}, while the path from destination to source ",
                            "is {:?}. The two paths should be symmetric and with ",
                            "the same distance.\nThe graph report is:\n{:?}"
                        ),
                        src_to_dst_distance,
                        dst_to_src_distance,
                        src_to_dst,
                        dst_to_src,
                        graph.textual_report()
                    );
                });
            });
        }
    }
    Ok(())
}
