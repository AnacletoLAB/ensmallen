use graph::Graph;
use shared::types::*;
use crate::utils::*;
use crate::graph_properties::*;

/// Executes the default test suite for holdouts.
pub fn default_holdout_test_suite(graph: &Graph, train: &Graph, test: &Graph) -> Result<()> {
    for g in &[graph, train, test] {
        validate_vocabularies(g);
    }
    test_graph_properties(train, None)?;
    test_graph_properties(test, None)?;
    assert!(
        !train.overlaps(&test)?,
        "Training graph overlaps with test graph!"
    );
    assert!(
        !test.overlaps(&train)?,
        "Test graph overlaps with training graph!"
    );
    assert!(graph.contains(&train)?, "Graph does not training graph.");
    assert!(graph.contains(&test)?, "Graph does not contain test graph.");
    let summed = (train | test)?;
    validate_vocabularies(&summed);
    assert!(
        summed.contains(&graph)?,
        "Composed train and test graph do not contained original graph."
    );
    let subtracted = (graph - test)?;
    validate_vocabularies(&subtracted);
    assert!(
        subtracted.contains(&train)?,
        "Main graph subtracted test does not contain training graph."
    );
    assert!(
        !subtracted.overlaps(&test)?,
        "Main graph subtracted train does not contain test graph."
    );
    let xorred = (graph ^ test)?;
    validate_vocabularies(&xorred);
    assert!(
        xorred.contains(&train)?,
        "Main graph xorred test does not contain training graph."
    );
    assert!(
        !xorred.overlaps(&test)?,
        "Main graph xorred train does not contain testing graph."
    );
    let anded = (graph & test)?;
    validate_vocabularies(&anded);
    assert!(
        anded.contains(&test)?,
        "Main graph anded test does not contain training graph."
    );

    Ok(())
}