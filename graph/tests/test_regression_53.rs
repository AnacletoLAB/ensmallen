extern crate graph;

use graph::{Graph, EdgeFileReader};

#[test]
/// This is a regression test that has been automatically generated
/// by the fuzzer harness.
/// The test originally caused a panic in the file test_utilities.rs,
/// specifically (at the time) line 337 and column 21.
/// The provided message was: 'In an undirected graph, we expect for the edge weights to be symmetrical but in the provided graph there has been found a case where the edge from 1 to 3 has weight 1.3193052, while the edge from 3 to 1 has weight 0.77283114, creating an asymetrical case.'
///
fn test_regression_53() -> Result<(), String> {
    let edges_reader = EdgeFileReader::new("tests/data/regression/53.edges")?
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
        false, // Directed
        false, // Directed edge list
        "Fuzz Graph" // Name of the graph
    )?;
    let _ = graph::test_utilities::default_test_suite(&mut graph, Some(false));
    Ok(())
}
