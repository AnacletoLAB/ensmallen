extern crate graph;

use graph::{EdgeFileReader, Graph};

#[test]
/// This is a regression test that has been automatically generated
/// by the fuzzer harness.
/// The test originally caused a panic in the file holdouts.rs,
/// specifically (at the time) line 208 and column 17.
///
fn test_regression_11() -> Result<(), String> {
    let edges_reader = EdgeFileReader::new("tests/data/regression/11.edges")?;

    let nodes_reader = None;

    let mut graph = Graph::from_unsorted_csv(
        edges_reader,
        nodes_reader,
        false,         // Directed
        false,         // Directed edge list
        "TestGraph11", // Name of the graph
    )?;

    for _ in 0..1000 {
        let _ = graph::test_utilities::test_vertex_cover(&mut graph, false);
    }
    let _ = graph::test_utilities::default_test_suite(&mut graph, false);

    Ok(())
}
