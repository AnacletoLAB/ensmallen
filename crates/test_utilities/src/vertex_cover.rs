use graph::Graph;
use shared::types::*;
use rayon::iter::ParallelIterator;

pub fn test_vertex_cover(graph: &mut Graph, _verbose: Option<bool>) -> Result<()> {
    let vertex_cover = graph.approximated_vertex_cover_set();
    graph
        .par_iter_edge_node_ids(true)
        .for_each(|(_, src_node_id, dst_node_id)| {
            assert!(
                vertex_cover.contains(&src_node_id) || vertex_cover.contains(&dst_node_id),
                concat!(
                    "We expected for either the node {} or {} to be in the vertex cover.\n",
                    "The vertex cover is {:?}"
                ),
                src_node_id,
                dst_node_id,
                vertex_cover
            );
        });
    Ok(())
}
