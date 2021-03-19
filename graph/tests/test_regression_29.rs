extern crate graph;

use graph::{EdgeFileReader, Graph, NodeFileReader};

#[test]
/// This is a regression test that has been automatically generated
/// by the fuzzer harness.
/// The test originally caused a panic in the file test_utilities.rs,
/// specifically (at the time) line 379 and column 13.
///
fn test_regression_number_29() -> Result<(), String> {
    let edges_reader = EdgeFileReader::new("tests/data/regression/29.edges")?;

    let nodes_reader = None;

    let mut graph = Graph::from_unsorted_csv(
        edges_reader,
        nodes_reader,
        false, // Directed
        false, // Directed edge list
        "Graph",
    )?;
    let _ = graph::test_utilities::default_test_suite(&mut graph, false);
    Ok(())
}
