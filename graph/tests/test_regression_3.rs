extern crate graph;

use graph::{EdgeFileReader, Graph, NodeFileReader};

#[test]
/// This is a regression test that has been automatically generated
/// by the fuzzer harness.
/// The test originally caused a panic in the file getters.rs,
/// specifically (at the time) line 273 and column 29.
///
fn test_regression_3() -> Result<(), String> {
    let edges_reader = EdgeFileReader::new("tests/data/regression/3.edges")?
        .set_rows_to_skip(Some(0))
        .set_header(Some(false))
        .set_separator(Some(","))?
        .set_verbose(Some(false))
        .set_sources_column_number(Some(0))?
        .set_destinations_column_number(Some(1))?
        .set_weights_column_number(Some(3))?
        .set_ignore_duplicates(Some(true))
        .set_skip_selfloops(Some(false))
        .set_numeric_edge_type_ids(Some(false))
        .set_numeric_node_ids(Some(false))
        .set_skip_weights_if_unavailable(Some(false))
        .set_skip_edge_types_if_unavailable(Some(false))
        .set_edge_types_column_number(Some(2))?;

    let nodes_reader = Some(
        NodeFileReader::new("tests/data/regression/3.nodes")?
            .set_rows_to_skip(Some(0))
            .set_separator(Some(","))?
            .set_header(Some(false))
            .set_verbose(Some(false))
            .set_ignore_duplicates(Some(false))
            .set_node_types_separator(Some("|"))?
            .set_nodes_column_number(Some(0))
            .set_node_types_column_number(Some(1)),
    );

    let mut graph = Graph::from_unsorted_csv(
        edges_reader,
        nodes_reader,
        false, // Directed
        false, // Directed edge list
        "Q",   // Name of the graph
    )?;
    let _ = graph::test_utilities::default_test_suite(&mut graph, false);
    Ok(())
}
