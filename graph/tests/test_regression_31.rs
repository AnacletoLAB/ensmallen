extern crate graph;

use graph::{Graph, EdgeFileReader};

#[test]
/// This is a regression test that has been automatically generated
/// by the fuzzer harness.
/// The test originally caused a panic in the file trees.rs,
/// specifically (at the time) line 166 and column 56.
///
fn test_regression_31() -> Result<(), String> {
    let edges_reader = EdgeFileReader::new("tests/data/regression/31.edges")?;

    let nodes_reader = None;

    let mut graph = Graph::from_unsorted_csv(
        edges_reader,
        nodes_reader,
        true, // Directed
        false, // Directed edge list
        "\u{0}\u{0}\u{0}W\u{c}\u{1c}\u{0}\u{0}\u{0}"  // Name of the graph
    )?;

    let _ = graph::test_utilities::default_test_suite(&mut graph, false);
    Ok(())
}
