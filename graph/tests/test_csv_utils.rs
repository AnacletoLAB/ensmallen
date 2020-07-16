extern crate graph;
use graph::csv_utils::*;

#[test]
fn test_check_consistent_lines() {
    check_consistent_lines("tests/data/edge_file.tsv", "\t").unwrap();
    check_consistent_lines("tests/data/node_file.tsv", "\t").unwrap();
}

#[test]
fn test_check_consistent_lines_should_panic() {
    assert!(
        check_consistent_lines("tests/data/should_panic.csv", ",").is_err()
    );
}

#[test]
fn test_has_columns() {
    has_columns(
        "tests/data/edge_file.tsv",
        "\t",
        &["subject", "object", "relation", "edge_label"],
        &[],
    )
    .unwrap();
    has_columns(
        "tests/data/edge_file.tsv",
        "\t",
        &["subject", "object", "relation", "edge_label"],
        &[&None],
    )
    .unwrap();
    has_columns(
        "tests/data/edge_file.tsv",
        "\t",
        &["subject", "object"],
        &[&Some("relation"), &Some("edge_label")],
    )
    .unwrap();
    has_columns("tests/data/edge_file.tsv", "\t", &["subject"], &[]).unwrap();
    has_columns(
        "tests/data/node_file.tsv",
        "\t",
        &["id", "name", "category", "description", "alias"],
        &[],
    )
    .unwrap();
    has_columns("tests/data/node_file.tsv", "\t", &["id"], &[]).unwrap();
}

#[test]
fn test_has_columns_should_panic() {
    assert!(has_columns("tests/data/should_panic.csv", ",", &["a", "b", "c"], &[]).is_err());
}
