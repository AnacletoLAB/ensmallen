extern crate graph;

use graph::{EdgeFileReader, Graph};

#[test]
/// This is a regression test that has been automatically generated
/// by the fuzzer harness.
///
/// THIS IS A NON DETERMINISTIC TEST!!!
///
/// The test originally caused a panic in the file test_utilities.rs,
/// specifically (at the time) line 644 and column 9.
/// The provided message was: 'assertion failed: `(left == right)` left: `(2, 2, 4)`, right: `(2, 2, 3)`: We expected the graph to have the same components once we remove the selfloops.'
///
fn test_regression_28() -> Result<(), String> {
    let edges_reader = EdgeFileReader::new("tests/data/regression/28.edges")?
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

    let mut graph = Graph::from_unsorted_csv(
        edges_reader,
        nodes_reader,
        false,        // Directed
        false,        // Directed edge list
        "Fuzz Graph", // Name of the graph
    )?;

    graph.enable(Some(true), Some(true), Some(true))?;
    let iterations = 1_000;
    let pb =
        graph::utils::get_loading_bar(true, "Running non-deterministic component test", iterations);

    for _ in 0..iterations {
        pb.inc(1);
        let _ = graph::test_utilities::test_remove_components(&mut graph, None);
    }
    Ok(())
}
