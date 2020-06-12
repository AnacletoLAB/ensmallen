extern crate graph;
use graph::csv_utils::*;

#[test]
fn test_check_consistent_lines() {
    check_consistent_lines("tests/data/edge_file.tsv", "\t");
    check_consistent_lines("tests/data/node_file.tsv", "\t");
}

#[test]
#[should_panic]
fn test_check_consistent_lines_should_panic() {
    check_consistent_lines("tests/data/should_panic.csv", ",");
}

#[test]
fn test_has_columns() {
    has_columns(
        "tests/data/edge_file.tsv",
        "\t",
        &[
            "subject",
            "object",
            "relation",
            "edge_label",
        ],
        &[],
    );
    has_columns(
        "tests/data/edge_file.tsv",
        "\t",
        &[
            "subject",
            "object",
            "relation",
            "edge_label",
        ],
        &[&None],
    );
    has_columns(
        "tests/data/edge_file.tsv",
        "\t",
        &["subject", "object"],
        &[&Some("relation"), &Some("edge_label")],
    );
    has_columns(
        "tests/data/edge_file.tsv",
        "\t",
        &["subject"],
        &[],
    );
    has_columns(
        "tests/data/node_file.tsv",
        "\t",
        &[
            "id",
            "name",
            "category",
            "description",
            "alias",
        ],
        &[],
    );
    has_columns("tests/data/node_file.tsv", "\t", &["id"], &[]);
}

#[test]
#[should_panic]
fn test_has_columns_should_panic() {
    has_columns(
        "tests/data/should_panic.csv",
        ",",
        &["a", "b", "c"],
        &[],
    );
}
