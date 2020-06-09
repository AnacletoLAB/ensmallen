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
    has_columns("tests/data/edge_file.tsv", "\t",
        vec!["subject", "object", "relation", "edge_label"].iter().map(|s| &s.to_string()).collect(),
        vec![]
    );
    has_columns("tests/data/edge_file.tsv", "\t", vec![&"subject".to_string(), &"object".to_string()], vec![Some("relation"), Some("edge_label")]);
    has_columns("tests/data/edge_file.tsv", "\t", vec!["subject"], vec![]);
    has_columns("tests/data/node_file.tsv", "\t", vec!["id", "name", "category", "description", "alias"], vec![]);
    has_columns("tests/data/node_file.tsv", "\t", vec!["id"], vec![]);
}


#[test]
#[should_panic]
fn test_has_columns_should_panic() {
    has_columns("tests/data/should_panic.csv", ",", vec!["a", "b", "c"], vec![]);
}

