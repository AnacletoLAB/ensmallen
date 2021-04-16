extern crate graph;

use graph::{EdgeFileReader, Graph, NodeFileReader};

#[test]
/// This is a regression test that has been automatically generated
/// by the fuzzer harness.
/// The test originally caused a panic in the file node_type_vocabulary.rs,
/// specifically (at the time) line 147 and column 10.
/// The provided message was: 'called `Option::unwrap()` on a `None` value'
///
fn test_regression_17() -> Result<(), String> {
    let edges_reader = EdgeFileReader::new("tests/data/regression/17.edges")?
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

    let nodes_reader = Some(
        NodeFileReader::new("tests/data/regression/17.nodes")?
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
        "\u{1b}\u{1b}\u{1b}\u{5}\u{5}\u{5}\u{5}\u{5}\u{5}\u{5}\u{5}\u{5}\u{5}\u{5}\u{5}\u{5}\u{5}\u{5}\u{5}\u{5}\u{5}\u{5}\u{5}\u{5}\u{5}\u{5}\u{5}\u{5}\u{5}\u{5}\u{5}\u{5}\u{5}\u{5}\u{5}\u{15}\u{5}\u{5}\u{5}\u{5}\u{5}\u{5}\u{5}\u{5}\u{5}\u{5}\u{5}\u{5}\u{5}\u{5}\u{5}\u{5}\u{5}\u{5}\u{5}\u{5}\u{5}\u{5}\u{5}\u{5}\u{5}\u{5}\u{5}\u{5}\u{5}\u{5}\u{5}\u{5}\u{5}\u{5}\u{5}\u{5}\u{5}\u{5}\u{5}\u{5}\u{5}\u{5}\u{5}\u{5}\u{5}\u{5}\u{5}\u{5}\u{5}\u{5}\u{5}\u{5}\u{5}\u{0}\u{0}\u{0}\u{0}\u{0}\u{0}\u{0}\n\u{5}\u{5}\u{5}\u{5}\u{5}\u{5}\u{5}\u{5}\u{5}\u{5}\u{5}\u{5}\u{5}\u{5}\u{5}\u{5}\u{5}\u{5}\r\u{5}\u{5}\u{5}\u{1}\u{1}\u{1}\u{1}\u{1}\u{1}\u{1}\u{1}\u{1}\u{1}\u{1}\u{1}\u{1}\u{1}\u{1}\u{1}\u{1}\u{1}\u{1}\u{1}997\u{1}\u{e}\u{1}\u{1}\u{1}\u{1}\u{2}\u{1}\u{1}\u{1}\u{1}\u{1}\u{1}\u{1}\u{1}\u{1}\u{1}\u{1}\u{1}\u{1}\u{1}\u{1}\u{1}\u{1}\u{1}\u{1}\u{1}\u{1}\u{1}\u{1}\u{1}\u{1}\u{1}\u{1}\u{1}\u{1}\u{1}\u{1}\u{1}\u{0}\u{0}\u{0}\u{0}\u{1}\u{1}\u{1}\u{1}\u{1}\u{1}\u{1}\u{1}\u{1}\u{1}\u{1}\u{1}\u{1}\u{1}\u{1}\u{1}\u{1}\u{1}\u{1}\u{1}\u{1}\u{1}\u{1}\u{1}\u{1}\u{1}\u{1}\u{1}\u{1}\u{1}\u{1}\u{1}\u{1}\u{1}\u{1}\u{1}\u{1}\u{1}\u{1}\u{1}\u{1}\u{1}\u{1}\u{1}\u{1}\u{1}\u{1}\u{1}\u{1}\u{1}\u{1}\u{1}\u{1}\u{1}\u{1}\u{1}\u{1}\u{1}\u{1}\u{1}\u{1}\u{1}\u{1}\u{1}\u{1}\u{1}\u{1}\u{1}\u{1}\u{1}\u{1}\u{1}" // Name of the graph
    )?;
    let _ = graph::test_utilities::default_test_suite(&mut graph, false);
    Ok(())
}
