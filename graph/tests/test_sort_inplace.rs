extern crate graph;
use std::fs;

use graph::sort_numeric_edge_list_inplace;

#[test]
fn test_sort_inplace() -> Result<(), String> {
    let target_path = "tests/data/unsorted_macaque_copy.tsv";
    fs::copy("tests/data/unsorted_macaque.tsv", target_path).unwrap();
    sort_numeric_edge_list_inplace(
        target_path,
        Some("\t".to_string()),
        Some(false),
        None,
        Some(0),
        None,
        Some(1),
        None,
        None,
        None,
        None,
    )?;
    Ok(())
}
