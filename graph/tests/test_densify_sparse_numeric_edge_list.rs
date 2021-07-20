extern crate graph;

use graph::densify_sparse_numeric_edge_list;

#[test]
fn test_densify_sparse_numeric_edge_list() -> Result<(), String> {
    densify_sparse_numeric_edge_list(
        None,
        "tests/data/sparse_numeric_macaque.tsv",
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
        "tests/data/dense_macaque.tsv",
        Some("\t".to_string()),
        Some(true),
        Some("subject".to_string()),
        None,
        Some("object".to_string()),
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
    )?;
    Ok(())
}
