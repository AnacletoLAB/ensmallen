extern crate graph;

use graph::{Graph, EdgeFileReader};

#[test]
/// This is a regression test that has been automatically generated
/// by the fuzzer harness.
/// The test originally caused a panic in the file test_utilities.rs,
/// specifically (at the time) line 377 and column 13.
///
fn test_regression_125() -> Result<(), String> {
    let edges_reader = EdgeFileReader::new("tests/data/regression/125.edges")?
        .set_verbose(Some(false));

    let nodes_reader = None;

    let mut graph = Graph::from_unsorted_csv(
        edges_reader,
        nodes_reader,
        true, // Directed
        true, // Directed edge list
        "nan" // Name of the graph
    )?;
    let _ = graph::test_utilities::default_test_suite(&mut graph, false);
    Ok(())
}
