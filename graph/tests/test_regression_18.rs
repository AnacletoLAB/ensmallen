extern crate graph;

use graph::{EdgeFileReader, Graph};

#[test]
/// This is a regression test that has been automatically generated
/// by the fuzzer harness.
/// The test originally caused a panic in the file getters.rs,
/// specifically (at the time) line 1033 and column 21.
/// The provided message was: 'index out of bounds: the len is 2 but the index is 2021161079'
///
fn test_regression_18() -> Result<(), String> {
    let edges_reader = EdgeFileReader::new("tests/data/regression/18.edges")?
        .set_rows_to_skip(Some(0))
        .set_header(Some(false))
        .set_separator(Some(","))?
        .set_verbose(Some(false))
        .set_sources_column_number(Some(0))?
        .set_destinations_column_number(Some(1))?
        .set_ignore_duplicates(Some(false))
        .set_skip_selfloops(Some(false))
        .set_numeric_edge_type_ids(Some(false))
        .set_numeric_node_ids(Some(false))
        .set_skip_weights_if_unavailable(Some(false))
        .set_skip_edge_types_if_unavailable(Some(false));

    let nodes_reader = None;

    let mut graph = Graph::from_unsorted_csv(
        edges_reader,
        nodes_reader,
        false,                                                    // Directed
        false,                                                    // Directed edge list
        "xxxxxxxxxxxxx55555555555555555555555555555555555555555", // Name of the graph
    )?;
    let _ = graph::test_utilities::default_test_suite(&mut graph, None);
    Ok(())
}
