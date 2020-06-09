extern crate ensmallen_graph;
use ensmallen_graph::csv_utils::*;

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
            "subject".to_string(),
            "object".to_string(),
            "relation".to_string(),
            "edge_label".to_string(),
        ],
        &[],
    );
    has_columns(
        "tests/data/edge_file.tsv",
        "\t",
        &[
            "subject".to_string(),
            "object".to_string(),
            "relation".to_string(),
            "edge_label".to_string(),
        ],
        &[None],
    );
    has_columns(
        "tests/data/edge_file.tsv",
        "\t",
        &["subject".to_string(), "object".to_string()],
        &[Some("relation".to_string()), Some("edge_label".to_string())],
    );
    has_columns(
        "tests/data/edge_file.tsv",
        "\t",
        &["subject".to_string()],
        &[],
    );
    has_columns(
        "tests/data/node_file.tsv",
        "\t",
        &[
            "id".to_string(),
            "name".to_string(),
            "category".to_string(),
            "description".to_string(),
            "alias".to_string(),
        ],
        &[],
    );
    has_columns("tests/data/node_file.tsv", "\t", &["id".to_string()], &[]);
}

#[test]
#[should_panic]
fn test_has_columns_should_panic() {
    has_columns(
        "tests/data/should_panic.csv",
        ",",
        &["a".to_string(), "b".to_string(), "c".to_string()],
        &[],
    );
}
