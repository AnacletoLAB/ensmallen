extern crate graph;

use graph::{EdgeFileReader, Graph};

#[test]
/// This is a regression test that has been automatically generated
/// by the fuzzer harness.
/// The test originally caused a panic in the file test_utilities.rs,
/// specifically (at the time) line 858 and column 17.
/// The provided message was: 'The path from source to destination has distance 0.06048208102583885 while the distance from destination to source has destination 0.057867418974637985. The path from source to destination is [0, 1], while the path from destination to source is [1, 0]. The two paths should be symmetric and with the same distance.'
///
fn test_regression_40() -> Result<(), String> {
    let edges_reader = EdgeFileReader::new("tests/data/regression/40.edges")?
        .set_rows_to_skip(Some(0))
        .set_header(Some(false))
        .set_separator(Some(","))?
        .set_verbose(Some(false))
        .set_sources_column_number(Some(0))?
        .set_destinations_column_number(Some(1))?
        .set_weights_column_number(Some(3))?
        .set_ignore_duplicates(Some(true))
        .set_skip_selfloops(Some(false))
        .set_numeric_edge_type_ids(Some(true))
        .set_numeric_node_ids(Some(false))
        .set_skip_weights_if_unavailable(Some(false))
        .set_skip_edge_types_if_unavailable(Some(false));

    let nodes_reader = None;

    let mut graph = Graph::from_unsorted_csv(
        edges_reader,
        nodes_reader,
        false,        // Directed
        false,        // Directed edge list
        "Fuzz Graph", // Name of the graph
    )?;
    let _ = graph::test_utilities::default_test_suite(&mut graph, Some(false));
    Ok(())
}
