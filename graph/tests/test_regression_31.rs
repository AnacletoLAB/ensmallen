extern crate graph;

use graph::{Graph, EdgeFileReader};

#[test]
/// This is a regression test that has been automatically generated
/// by the fuzzer harness.
/// The test originally caused a panic in the file test_utilities.rs,
/// specifically (at the time) line 477 and column 9.
/// The provided message was: 'assertion failed: `(left == right)` left: `9`, right: `8`: We expected the connected components to be a dense set. The obtained components are: [7, 7, 7, 6, 7, 7, 7, 8, 8, 9, 0, 0, 0, 0, 0, 0, 0, 1, 1, 2, 2, 2, 3, 3, 2, 7, 9, 0, 0, 0, 0, 4, 4, 7, 7, 7, 9, 9, 5, 5, 6, 6]'
///
fn test_regression_31() -> Result<(), String> {
    let edges_reader = EdgeFileReader::new("tests/data/regression/31.edges")?
        .set_rows_to_skip(Some(0))
        .set_header(Some(false))
        .set_separator(Some(","))?
        .set_verbose(Some(false))
        .set_sources_column_number(Some(0))?
        .set_destinations_column_number(Some(1))?
        .set_ignore_duplicates(Some(true))
        .set_skip_self_loops(Some(false))
        .set_numeric_edge_type_ids(Some(true))
        .set_numeric_node_ids(Some(false))
        .set_skip_weights_if_unavailable(Some(false))
        .set_skip_edge_types_if_unavailable(Some(false));

    let nodes_reader = None;

    let mut graph = Graph::from_unsorted_csv(
        edges_reader,
        nodes_reader,
        false, // Directed
        false, // Directed edge list
        "Fuzz Graph" // Name of the graph
    )?;
    let _ = graph::test_utilities::default_test_suite(&mut graph, false);
    Ok(())
}
