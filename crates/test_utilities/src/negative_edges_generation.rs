use graph::Graph;
use shared::types::*;
use crate::holdouts::default_holdout_test_suite;
use crate::utils::*;

pub fn test_negative_edges_generation(graph: &mut Graph, verbose: Option<bool>) -> Result<()> {
    for only_from_same_component in &[true, false] {
        // If the graph is very sparse, this takes a lot of time
        // and makes the test suite very slow.
        if *only_from_same_component && graph.get_directed_edges_number() < 100 {
            continue;
        }
        let negatives = graph.sample_negatives(
            graph.get_edges_number(),
            None,
            None,
            Some(*only_from_same_component),
            verbose,
        )?;
        assert_eq!(
            graph.get_edges_number(),
            negatives.get_edges_number(),
            "We expect the graph and its negative graph to have the same number of edges but we got {} and {}.",
            graph.get_edges_number(),
            negatives.get_edges_number()
        );
        validate_vocabularies(&negatives);
        if !graph.has_edge_types() {
            assert!(!graph.overlaps(&negatives)?);
            assert!(!negatives.overlaps(&graph)?);
        }
        // Testing holdouts executed on negative edges.
        let (neg_train, neg_test) =
            negatives.random_holdout(0.8, None, None, None, None, verbose)?;

        neg_test.get_trap_nodes_number();

        default_holdout_test_suite(&negatives, &neg_train, &neg_test)?;
    }

    Ok(())
}
