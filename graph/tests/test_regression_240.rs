extern crate graph;

use graph::{Graph, EdgeFileReader};

#[test]
/// This is a regression test that has been automatically generated
/// by the fuzzer harness.
/// The test originally caused a panic in the file test_utilities.rs",
/// specifically (at the time) line 645 and column 10.
///
fn test_regression_240() -> Result<(), String> {
    let edges_reader = EdgeFileReader::new("tests/data/regression/240.edges")?
        .set_default_weight(Some(826428600.0));

    let nodes_reader = None;

    let mut graph = Graph::from_unsorted_csv(
        edges_reader,
        nodes_reader,
        false, // Directed
        false, // Directed edge list
        "AltroGrafo" // Name of the graph
    )?;
    let _ = graph::test_utilities::default_test_suite(&mut graph, false);
    Ok(())
}
