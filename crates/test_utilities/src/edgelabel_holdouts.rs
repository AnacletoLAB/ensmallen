use graph::Graph;
use shared::types::*;


pub fn test_edgelabel_holdouts(graph: &mut Graph, _verbose: Option<bool>) -> Result<()> {
    for use_stratification in [true, false].iter() {
        if *use_stratification && graph.has_singleton_edge_types()?
            || graph.get_directed_edges_number() - graph.get_unknown_edge_types_number()? < 2
            || !graph.has_edge_types()
        {
            assert!(graph
                .edge_label_holdout(0.8, Some(*use_stratification), None)
                .is_err());
            continue;
        }
        let (train, test) = graph.edge_label_holdout(0.8, Some(*use_stratification), None)?;
        assert!(train.has_unknown_edge_types()?);
        assert!(test.has_unknown_edge_types()?);
        assert!(
            train.edge_types.as_ref().map_or(false, |train_nts| {
                test.edge_types.as_ref().map_or(false, |test_nts| {
                    train_nts.ids.iter().zip(test_nts.ids.iter()).all(
                        |(train_edge_type, test_edge_type)| {
                            !(train_edge_type.is_some() && test_edge_type.is_some())
                        },
                    )
                })
            }),
            "The train and test edge-label graphs are overlapping!"
        );
    }
    Ok(())
}