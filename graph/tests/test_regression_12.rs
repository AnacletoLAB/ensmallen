extern crate graph;

use graph::{Graph, EdgeFileReader};

#[test]
/// This is a regression test that has been automatically generated
/// by the fuzzer harness.
/// The test originally caused a panic in the file holdouts.rs,
/// specifically (at the time) line 208 and column 17.
///
fn test_regression_12() -> Result<(), String> {
    let edges_reader = EdgeFileReader::new("tests/data/regression/12.edges", "RegressionTest".to_owned())?
        .set_skip_self_loops(Some(false));

    let nodes_reader = None;

    let mut graph = Graph::from_unsorted_csv(
        edges_reader,
        nodes_reader,
        false, // Directed
        false, // Directed edge list
        "\r\"" // Name of the graph
    )?;
    
    let _ = graph::test_utilities::default_test_suite(&mut graph, false);
    Ok(())
}
