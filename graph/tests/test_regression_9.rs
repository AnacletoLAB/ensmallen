extern crate graph;

use graph::{EdgeFileReader, Graph};

#[test]
/// This is a regression test that has been automatically generated
/// by the fuzzer harness.
/// The test originally caused a panic in the file elias_fano.rs,
/// specifically (at the time) line 174 and column 25.
/// The provided message was: 'Cannot execute the select1 inside the RsDict with index 7, the high bits currently have 7 ones and have size 13'
///
fn test_regression_9() -> Result<(), String> {
    let edges_reader = EdgeFileReader::new("tests/data/regression/9.edges")?;

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
