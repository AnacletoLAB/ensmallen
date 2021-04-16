extern crate graph;

use graph::{EdgeFileReader, Graph};

#[test]
/// This is a regression test that has been automatically generated
/// by the fuzzer harness.
/// The test originally caused a panic in the file getters.rs,
/// specifically (at the time) line 665 and column 20.
///
fn test_regression_7() -> Result<(), String> {
    let edges_reader = EdgeFileReader::new("tests/data/regression/7.edges")?
        .set_rows_to_skip(Some(0))
        .set_header(Some(false))
        .set_separator(Some(","))?
        .set_verbose(Some(false))
        .set_sources_column_number(Some(0))?
        .set_destinations_column_number(Some(1))?
        .set_ignore_duplicates(Some(true))
        .set_skip_selfloops(Some(false))
        .set_numeric_edge_type_ids(Some(false))
        .set_numeric_node_ids(Some(false))
        .set_skip_weights_if_unavailable(Some(false))
        .set_skip_edge_types_if_unavailable(Some(false))
        .set_edge_types_column_number(Some(2))?;

    let nodes_reader = None;

    assert!(
        Graph::from_unsorted_csv(
            edges_reader,
            nodes_reader,
            false,                       // Directed
            true,                        // Directed edge list
            "\u{5}\u{5}\u{5}\u{5}\u{5}"  // Name of the graph
        )
        .is_err(),
        "We expect this graph build to fail because it is a directed multigraph"
    );
    Ok(())
}
