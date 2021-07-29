use graph::Graph;
use shared::types::*;

pub fn test_graph_diameter(graph: &mut Graph, verbose: Option<bool>) -> Result<()> {
    // TODO! update this when we will support the graph diameter on directed graphs
    let (n_of_components, _, _) = graph.get_connected_components_number(verbose);

    assert_eq!(
        graph.get_diameter_naive(Some(false), verbose),
        graph.get_diameter(Some(false), verbose),
    );

    match n_of_components {
        0 => {
            // on an empty graph this should always fail
            assert!(graph.get_diameter(Some(false), verbose).is_err());
            assert!(graph.get_diameter(Some(true), verbose).is_err());
        }

        1 => {
            // by definition the diameter of a graph with a single component
            // cannot be infinite unless it's just a singleton and it does not have edges.
            if graph.get_nodes_number() == 1 && !graph.has_edges() {
                assert!(graph
                    .get_diameter(Some(false), verbose)
                    .unwrap()
                    .is_infinite());
                assert!(graph
                    .get_diameter(Some(true), verbose)
                    .unwrap()
                    .is_infinite());
            } else {
                assert!(graph
                    .get_diameter(Some(false), verbose)
                    .unwrap()
                    .is_finite());
                assert!(graph.get_diameter(Some(true), verbose).unwrap().is_finite());
            }
        }

        _ => {
            assert!(graph
                .get_diameter(Some(false), verbose)
                .unwrap()
                .is_infinite());
        }
    }

    Ok(())
}
