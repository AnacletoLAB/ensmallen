use graph::Graph;
use shared::types::*;

pub fn test_bfs(graph: &mut Graph, verbose: Option<bool>) -> Result<()> {
    // We avoid running this test on too big graphs so to avoid slowing down the test suite
    if graph.get_nodes_number() > 100 {
        return Ok(());
    }

    // If the graph is empty the other tests on BFS make little sense
    if !graph.has_nodes() {
        assert!(graph
            .get_breath_first_search_from_node_ids(0, None, None, None)
            .is_err());
        return Ok(());
    }

    // BFS on an unweighted graph gives simmetric results.
    if !graph.is_directed() {
        let components_ids = graph.get_node_connected_component_ids(verbose);
        for maximal_depth in [None, Some(1), Some(2), Some(3)] {
            graph.iter_node_ids().for_each(|src_node_id| {
                graph.iter_node_ids().for_each(|dst_node_id| unsafe {
                    // Check that the obtained results are simmetric
                    let src_to_dst = graph.get_unchecked_minimum_path_node_ids_from_node_ids(
                        src_node_id,
                        dst_node_id,
                        maximal_depth,
                    );
                    let dst_to_src = graph.get_unchecked_minimum_path_node_ids_from_node_ids(
                        dst_node_id,
                        src_node_id,
                        maximal_depth,
                    );
                    if src_node_id == dst_node_id {
                        assert!(src_to_dst.is_err());
                        assert!(dst_to_src.is_err());
                        return;
                    }
                    if components_ids[src_node_id as usize] != components_ids[dst_node_id as usize]
                    {
                        assert!(src_to_dst.is_err());
                        assert!(dst_to_src.is_err());
                        return;
                    }
                    if let (Ok(src_to_dst), Ok(dst_to_src)) = (src_to_dst, dst_to_src) {
                        // Check that the two paths have the same length
                        assert_eq!(src_to_dst.len(), dst_to_src.len());
                        // Test that the k-paths return a compatible result
                        let kpaths = graph.get_unchecked_k_shortest_path_node_ids_from_node_ids(
                            src_node_id,
                            dst_node_id,
                            5,
                        );
                        let min_length = kpaths.into_iter().map(|path| path.len()).min().unwrap();
                        assert_eq!(min_length, src_to_dst.len());
                    }
                });
            });
        }
    }
    Ok(())
}