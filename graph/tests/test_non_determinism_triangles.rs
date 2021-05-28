extern crate graph;

use graph::{Graph, EdgeFileReader, NodeFileReader};

#[test]
/// This is a regression test that has been automatically generated
/// by the fuzzer harness.
/// The test originally caused a panic in the file test_utilities.rs,
/// specifically (at the time) line 762 and column 5.
/// The provided message was: 'assertion failed: `(left == right)`  left: `0`, right: `3`'
///
fn test_non_determinism_triangles() -> Result<(), String> {
    let edges_reader = EdgeFileReader::new("tests/data/regression/37.edges")?
        .set_rows_to_skip(Some(0))
        .set_header(Some(false))
        .set_separator(Some(","))?
        .set_verbose(Some(false))
        .set_sources_column_number(Some(0))?
        .set_destinations_column_number(Some(1))?
        .set_weights_column_number(Some(3))?
        .set_ignore_duplicates(Some(false))
        .set_skip_selfloops(Some(false))
        .set_numeric_edge_type_ids(Some(false))
        .set_numeric_node_ids(Some(false))
        .set_skip_weights_if_unavailable(Some(false))
        .set_skip_edge_types_if_unavailable(Some(false))
        .set_edge_types_column_number(Some(2))?;

    let mut graph = Graph::from_unsorted_csv(
        edges_reader,
        None,
        true, // Directed
        true, // Directed edge list
        "Fuzz Graph" // Name of the graph
    )?;
    for _ in 0..10_000 {
        let _ = graph::test_utilities::test_polygons(&mut graph, Some(false));
    }
    Ok(())
}
