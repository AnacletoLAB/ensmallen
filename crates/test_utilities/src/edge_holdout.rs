use graph::Graph;
use shared::types::*;

use crate::holdouts::default_holdout_test_suite;

pub fn test_edge_holdouts(graph: &Graph, verbose: Option<bool>) -> Result<()> {
    if !graph.has_edge_types() {
        assert!(graph
            .connected_holdout(0.8, None, Some(vec![None]), Some(false), None)
            .is_err());
    }
    for include_all_edge_types in &[false, true] {
        let (train, test) = graph.random_holdout(
            0.6,
            None,
            Some(*include_all_edge_types),
            None,
            None,
            verbose,
        )?;
        default_holdout_test_suite(graph, &train, &test)?;
        let (train, test) =
            graph.connected_holdout(0.8, None, None, Some(*include_all_edge_types), verbose)?;
        assert_eq!(graph.get_nodes_number(), train.get_nodes_number());
        assert_eq!(graph.get_nodes_number(), test.get_nodes_number());

        let (original_total, original_min_comp, original_max_comp) =
            graph.get_connected_components_number(verbose);
        let (train_total, train_min_comp, train_max_comp) =
            train.get_connected_components_number(verbose);
        if original_total == 1 {
            assert!(original_min_comp == original_max_comp);
            assert_eq!(original_min_comp, graph.get_nodes_number());
        }
        if original_total == 2 {
            assert!(original_min_comp <= original_max_comp);
            assert_eq!(
                original_min_comp + original_max_comp,
                graph.get_nodes_number(),
                concat!(
                    "When a graph contains two connected components, ",
                    "summing the two connected components should give ",
                    "the number of nodes in the graph.\n",
                    "The graph is {}."
                ),
                if graph.is_directed() {
                    "directed"
                } else {
                    "undirected"
                }
            );
        }
        if train_total == 1 {
            assert!(train_min_comp == train_max_comp);
            assert_eq!(train_min_comp, graph.get_nodes_number());
        }
        if train_total == 2 {
            assert!(train_min_comp <= train_max_comp);
            assert_eq!(train_min_comp + train_max_comp, train.get_nodes_number());
        }
        assert_eq!(
            train_total, original_total,
            concat!(
                "In a connected holdout the training graph must have the ",
                "same number of connected components as in the original ",
                "graph, but here the training graph has {} components ",
                "while the original graph has {} components."
            ),
            train_total, original_total
        );
        assert_eq!(
            train_min_comp, original_min_comp,
            concat!(
                "In a connected holdout the training graph must have the ",
                "same number of connected components as in the original ",
                "graph, but here the minimum connected component size ",
                "of the training graph has size {} while the corresponding one ",
                "from the original graph has size {}."
            ),
            train_min_comp, original_min_comp
        );
        assert_eq!(
            train_max_comp, original_max_comp,
            concat!(
                "In a connected holdout the training graph must have the ",
                "same number of connected components as in the original ",
                "graph, but here the maximum connected component size ",
                "of the training graph has size {} while the corresponding one ",
                "from the original graph has size {}."
            ),
            train_max_comp, original_max_comp
        );

        default_holdout_test_suite(graph, &train, &test)?;
    }
    Ok(())
}
