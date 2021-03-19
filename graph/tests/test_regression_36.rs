extern crate graph;

use graph::{Graph, EdgeFileReader};

#[test]
/// This is a regression test that has been automatically generated
/// by the fuzzer harness.
/// The test originally caused a panic in the file test_utilities.rs,
/// specifically (at the time) line 230 and column 9.
///
fn test_regression_36() -> Result<(), String> {
    let edges_reader = EdgeFileReader::new("tests/data/regression/36.edges")?
        .set_verbose(Some(false));

    let nodes_reader = None;

    assert!(
        Graph::from_unsorted_csv(
            edges_reader,
            nodes_reader,
            false, // Directed
            true, // Directed edge list
            "nan" // Name of the graph
        ).is_err(),
        concat!(
            "It should not be possible to load this graph because ",
            "it is made of an incomplete directed edge list."
        )
    );
    Ok(())
}
