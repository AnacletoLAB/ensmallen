use graph::Graph;
use shared::types::*;
use crate::utils::*;
use log::warn;
use rayon::iter::ParallelIterator;


pub fn test_random_walks(graph: &mut Graph, _verbose: Option<bool>) -> Result<()> {
    // Testing principal random walk algorithms
    let walker = first_order_walker(&graph)?;
    assert_eq!(walker.clone(), walker);
    let walker2 = second_order_walker(&graph, 2.0, 2.0)?;
    assert_eq!(walker2.clone(), walker2);

    if !graph.directed {
        warn!("Executing random walks tests.");
        for mode in 0..2 {
            if mode == 1 {
                graph.enable(None, None, None)?;
                if let Some(cumulative_node_degrees) = &graph.cumulative_node_degrees {
                    assert_eq!(
                        cumulative_node_degrees.len(),
                        graph.get_nodes_number() as usize,
                        "Length of cumulative_node_degrees does not match number of nodes in the graph."
                    );
                }
                if let Some(destinations) = &graph.destinations {
                    assert_eq!(
                        destinations.len(),
                        graph.get_directed_edges_number() as usize,
                        "Length of destinations does not match number of edges in the graph."
                    );
                }
            }
            assert_eq!(
                graph
                    .iter_random_walks(1, &walker)
                    .map(|iter| iter.collect::<Vec<Vec<NodeT>>>()),
                graph
                    .iter_random_walks(1, &walker)
                    .map(|iter| iter.collect::<Vec<Vec<NodeT>>>()),
                "Walks of first order are not reproducible!"
            );

            assert_eq!(
                graph
                    .iter_random_walks(1, &second_order_walker(&graph, 2.0, 2.0)?)
                    .map(|iter| iter.collect::<Vec<Vec<NodeT>>>()),
                graph
                    .iter_random_walks(1, &second_order_walker(&graph, 2.0, 2.0)?)
                    .map(|iter| iter.collect::<Vec<Vec<NodeT>>>()),
                "Walks of second order are not reproducible!"
            );

            assert_eq!(
                graph
                    .iter_complete_walks(&walker)
                    .map(|iter| iter.collect::<Vec<Vec<NodeT>>>()),
                graph
                    .iter_complete_walks(&walker)
                    .map(|iter| iter.collect::<Vec<Vec<NodeT>>>()),
                "Complete first order walks are not reproducible!"
            );

            assert_eq!(
                graph
                    .iter_complete_walks(&second_order_walker(&graph, 2.0, 2.0)?)
                    .map(|iter| iter.collect::<Vec<Vec<NodeT>>>()),
                graph
                    .iter_complete_walks(&second_order_walker(&graph, 2.0, 2.0)?)
                    .map(|iter| iter.collect::<Vec<Vec<NodeT>>>()),
                "Complete second order walks are not reproducible!"
            );

            assert_eq!(
                graph
                    .iter_complete_walks(&second_order_walker(&graph, 2.0, 1.0)?)
                    .map(|iter| iter.collect::<Vec<Vec<NodeT>>>()),
                graph
                    .iter_complete_walks(&second_order_walker(&graph, 2.0, 1.0)?)
                    .map(|iter| iter.collect::<Vec<Vec<NodeT>>>()),
                "Complete second order walks are not reproducible!"
            );

            assert_eq!(
                graph
                    .iter_complete_walks(&second_order_walker(&graph, 1.0, 2.0)?)
                    .map(|iter| iter.collect::<Vec<Vec<NodeT>>>()),
                graph
                    .iter_complete_walks(&second_order_walker(&graph, 1.0, 2.0)?)
                    .map(|iter| iter.collect::<Vec<Vec<NodeT>>>()),
                "Complete second order walks are not reproducible!"
            );
        }
    } else {
        assert!(graph.iter_complete_walks(&walker).is_err());
    }
    Ok(())
}
