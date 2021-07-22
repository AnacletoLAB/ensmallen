extern crate graph;

use graph::{EdgeFileReader, Graph};

#[test]
/// This is a regression test that has been automatically generated
/// by the fuzzer harness.
/// The test originally caused a panic in the file holdouts.rs,
/// specifically (at the time) line 932 and column 59.
///
fn test_regression_1() -> Result<(), String> {
    let edges_reader = EdgeFileReader::new("tests/data/regression/1.edges")?
        .set_rows_to_skip(Some(0))
        .unwrap()
        .set_header(Some(false))
        .unwrap()
        .set_separator(Some(","))?
        .set_verbose(Some(false))
        .set_sources_column_number(Some(0))?
        .set_destinations_column_number(Some(1))?
        .set_ignore_duplicates(Some(false))
        .set_numeric_edge_type_ids(Some(false))
        .set_numeric_node_ids(Some(false))
        .set_skip_weights_if_unavailable(Some(false))
        .set_skip_edge_types_if_unavailable(Some(false))
        .set_edge_types_column_number(Some(2))?;

    let mut graph = Graph::from_file_readers(
        Some(edges_reader),
        None,
        None, 
        None, 
        true,
        true,
        true,
        "Regression1", // Name of the graph
    )?;
    let _ = graph::test_utilities::default_test_suite(&mut graph, None);
    Ok(())
}
