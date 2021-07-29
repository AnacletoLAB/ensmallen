use graph::Graph;
use shared::types::*;
use std::collections::HashSet;
use log::warn;

pub fn test_edgelist_generation(graph: &mut Graph, _verbose: Option<bool>) -> Result<()> {
    let _clique = graph.get_clique_edge_names(
        None,
        None,
        Some(false),
        None,
        // limit to compute the clique for at most the first 3 nodes
        // because it's really expensive computationally.
        Some(
            graph
                .get_node_names()
                .iter()
                .take(3)
                .cloned()
                .collect::<HashSet<String>>(),
        ),
    );
    warn!("Running edge lists generator tests.");
    if graph.get_nodes_number() > 1 {
        let _bipartite = graph.get_bipartite_edge_names(
            None,
            Some(
                [unsafe { graph.get_unchecked_node_name_from_node_id(0) }]
                    .iter()
                    .cloned()
                    .collect::<HashSet<String>>(),
            ),
            Some(
                [unsafe { graph.get_unchecked_node_name_from_node_id(1) }]
                    .iter()
                    .cloned()
                    .collect::<HashSet<String>>(),
            ),
            None,
            None,
        )?;
        let _star = graph.get_star_edges(
            unsafe { graph.get_unchecked_node_name_from_node_id(0) },
            Some(false),
            Some(
                [unsafe { graph.get_unchecked_node_name_from_node_id(1) }]
                    .iter()
                    .cloned()
                    .collect::<HashSet<String>>(),
            ),
            None,
        )?;
        let _star = graph.get_star_edge_names(
            unsafe { graph.get_unchecked_node_name_from_node_id(0) },
            Some(false),
            Some(
                [unsafe { graph.get_unchecked_node_name_from_node_id(1) }]
                    .iter()
                    .cloned()
                    .collect::<HashSet<String>>(),
            ),
            None,
        )?;
    }
    Ok(())
}
