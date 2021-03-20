extern crate graph;

use graph::{Graph, EdgeFileReader};

#[test]
/// This is a regression test that has been automatically generated
/// by the fuzzer harness.
/// The test originally caused a panic in the file test_utilities.rs",
/// specifically (at the time) line 150 and column 9.
///
fn test_regression_234() -> Result<(), String> {
    let edges_reader = EdgeFileReader::new("tests/data/regression/234.edges")?
        .set_rows_to_skip(Some(0))
        .set_header(Some(false))
        .set_separator(Some(","))?
        .set_verbose(Some(false))
        .set_ignore_duplicates(Some(false))
        .set_skip_self_loops(Some(false))
        .set_numeric_edge_type_ids(Some(false))
        .set_numeric_node_ids(Some(true))
        .set_skip_weights_if_unavailable(Some(false))
        .set_skip_edge_types_if_unavailable(Some(false));

    let nodes_reader = None;

    assert!(
        Graph::from_unsorted_csv(
            edges_reader,
            nodes_reader,
            true, // Directed
            true, // Directed edge list
            "\u{1}" // Name of the graph
        ).is_err(),
        "The graph does not have numeric node IDs."
    );
    Ok(())
}
