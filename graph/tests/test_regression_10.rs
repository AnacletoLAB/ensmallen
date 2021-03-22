extern crate graph;

use graph::{EdgeFileReader};

#[test]
/// This is a regression test that has been automatically generated
/// by the fuzzer harness.
/// The test originally caused a panic in the file node_type_vocabulary.rs,
/// specifically (at the time) line 69 and column 13.
///
fn test_regression_10() -> Result<(), String> {
    assert!(
        EdgeFileReader::new("tests/data/regression/10.edges")?
        .set_rows_to_skip(Some(0))
        .set_header(Some(false))
        .set_separator(Some(","))?
        .set_verbose(Some(false))
        .set_sources_column_number(Some(0)).is_err(),
        "We expect this to crash because it has an empty edge list."
    );
    Ok(())
}
