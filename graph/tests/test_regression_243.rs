extern crate graph;

use graph::{Graph, EdgeFileReader, NodeFileReader};

#[test]
/// This is a regression test that has been automatically generated
/// by the fuzzer harness.
/// The test originally caused a panic in the file node_type_vocabulary.rs,
/// specifically (at the time) line 102 and column 54.
///
fn test_regression_243() -> Result<(), String> {
    let edges_reader = EdgeFileReader::new("tests/data/regression/243.edges")?
        .set_rows_to_skip(Some(0))
        .set_header(Some(false))
        .set_separator(Some(","))?
        .set_verbose(Some(false))
        .set_ignore_duplicates(Some(false))
        .set_skip_self_loops(Some(false))
        .set_numeric_edge_type_ids(Some(false))
        .set_numeric_node_ids(Some(false))
        .set_skip_weights_if_unavailable(Some(false))
        .set_skip_edge_types_if_unavailable(Some(false));

    let nodes_reader = Some(NodeFileReader::new("tests/data/regression/243.nodes")?
        .set_rows_to_skip(Some(0))
        .set_separator(Some(","))?
        .set_header(Some(false))
        .set_verbose(Some(false))
        .set_ignore_duplicates(Some(false)));

    assert!(
        Graph::from_unsorted_csv(
            edges_reader,
            nodes_reader,
            false, // Directed
            false, // Directed edge list
            "graphname" // Name of the graph
        ).is_err(),
        "Empty edge file must cause error."
    );
    Ok(())
}
