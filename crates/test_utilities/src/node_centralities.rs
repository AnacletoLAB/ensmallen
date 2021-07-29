use graph::Graph;
use shared::types::*;

pub fn test_node_centralities(graph: &mut Graph, verbose: Option<bool>) -> Result<()> {
    if graph.has_edges() {
        let node_degree_centralities = graph.get_degree_centrality().unwrap();

        assert_eq!(
            node_degree_centralities.len(),
            graph.get_nodes_number() as usize
        );

        assert!(
            node_degree_centralities
                .iter()
                .cloned()
                .all(|value| value <= 1.0 && value >= 0.0),
            "All node degrees centralities are expected to be within 0 and 1, but are {:?}.",
            node_degree_centralities
        );
    }

    if graph.has_edge_weights() && !graph.has_negative_edge_weights().unwrap() {
        let node_degree_centralities = graph.get_weighted_degree_centrality().unwrap();

        assert_eq!(
            node_degree_centralities.len(),
            graph.get_nodes_number() as usize
        );

        assert!(
            node_degree_centralities
                .iter()
                .cloned()
                .all(|value| value <= 1.0 && value >= 0.0),
            concat!(
                "All weighted node degrees centralities ",
                "are expected to be within 0 and 1, ",
                "but are {:?} and the node degrees are {:?}, with the ",
                "minimum weighted node degree being {} and ",
                "maximum weighted node degree being {}.",
            ),
            node_degree_centralities,
            graph.get_weighted_node_degrees(),
            graph.get_weighted_minimum_node_degree().clone().unwrap(),
            graph.get_weighted_maximum_node_degree().clone().unwrap(),
        );
    }

    let node_betweenness_centralities = graph.get_betweenness_centrality(None, verbose);
    assert_eq!(
        node_betweenness_centralities.len(),
        graph.get_nodes_number() as usize
    );
    node_betweenness_centralities
        .into_iter()
        .enumerate()
        .for_each(|(node_id, value)| {
            if unsafe { graph.is_unchecked_singleton_from_node_id(node_id as NodeT) } {
                assert!(value.abs() < f64::EPSILON);
            }
        });
    Ok(())
}