use graph::Graph;
use shared::types::*;

pub fn test_nodelabel_holdouts(graph: &mut Graph, _verbose: Option<bool>) -> Result<()> {
    for use_stratification in [true, false] {
        if graph.get_known_node_types_number()? < 2
            || (use_stratification
                && (graph.has_multilabel_node_types()? || graph.has_singleton_node_types()?))
        {
            assert!(graph
                .node_label_holdout(0.8, Some(use_stratification), Some(42))
                .is_err());
            continue;
        }

        let (train, test) = graph.node_label_holdout(0.8, Some(use_stratification), Some(42))?;
        assert!(train.has_unknown_node_types()?);
        assert!(test.has_unknown_node_types()?);
        let remerged = &mut (&train | &test)?;
        assert_eq!(remerged.node_types, graph.node_types);
        assert!(
            remerged.contains(graph)?,
            "The re-merged holdouts does not contain the original graph."
        );
        assert!(
            graph.contains(remerged)?,
            "The re-merged holdouts does not contain the original graph."
        );
        assert!(
            train.node_types.as_ref().map_or(false, |train_nts| {
                test.node_types.as_ref().map_or(false, |test_nts| {
                    train_nts.ids.iter().zip(test_nts.ids.iter()).all(
                        |(train_node_type, test_node_type)| {
                            !(train_node_type.is_some() && test_node_type.is_some())
                        },
                    )
                })
            }),
            "The train and test node-label graphs are overlapping!"
        );
    }
    Ok(())
}
