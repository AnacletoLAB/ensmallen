extern crate graph;

use graph::{EdgeFileReader, Graph};

#[test]
/// This is a regression test that has been automatically generated
/// by the fuzzer harness.
/// The test originally caused a panic in the file holdouts.rs,
/// specifically (at the time) line 196 and column 17.
///
fn test_regression_10() -> Result<(), String> {
    let edges_reader =
        EdgeFileReader::new("tests/data/regression/10.edges")?.set_skip_selfloops(Some(false));

    let nodes_reader = None;

    let mut graph = Graph::from_unsorted_csv(
        edges_reader,
        nodes_reader,
        false, // Directed
        false, // Directed edge list
        "",    // Name of the graph
    )?;

    let _ = graph::test_utilities::default_test_suite(&mut graph, None);
    Ok(())
}
