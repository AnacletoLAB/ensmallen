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
    has_columns("tests/data/edge_file.tsv", "\t", vec!["subject", "object", "relation", "edge_label"]);
    has_columns("tests/data/edge_file.tsv", "\t", vec!["subject"]);
    has_columns("tests/data/node_file.tsv", "\t", vec!["id", "name", "category", "description", "alias"]);
    has_columns("tests/data/node_file.tsv", "\t", vec!["id"]);
}


#[test]
#[should_panic]
fn test_has_columns_should_panic() {
    has_columns("tests/data/should_panic.csv", ",", vec!["a", "b", "c"]);
}

