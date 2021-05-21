extern crate graph;

use graph::{Graph, EdgeFileReader, NodeFileReader};

#[test]
/// This is a regression test that has been automatically generated
/// by the fuzzer harness.
/// The test originally caused a panic in the file test_utilities.rs,
/// specifically (at the time) line 330 and column 9.
/// The provided message was: 'assertion failed: graph.unique_sources.is_some()'
///
fn test_regression_35() -> Result<(), String> {
    let mut graph = Graph::from_string_unsorted(
        Vec::new().into_iter(),
        Some(vec![
            Ok(("0".to_string(), Some(vec!["0".to_string()]))),
            Ok(("1".to_string(), Some(vec!["0".to_string()]))),
            Ok(("2".to_string(), Some(vec!["0".to_string()]))),
            Ok(("3".to_string(), Some(vec!["0".to_string()]))),
            Ok(("4".to_string(), Some(vec!["0".to_string()]))),
            Ok(("5".to_string(), Some(vec!["0".to_string()]))),
        ].into_iter()),
        false, // Directed
        true, // Directed edge list
        "Fuzz Graph", // Name of the graph
        true,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        true,
        true,
        false,
        true,
        true,
        true,
        false,
    )?;
    let _ = graph::test_utilities::default_test_suite(&mut graph, false);
    Ok(())
}
