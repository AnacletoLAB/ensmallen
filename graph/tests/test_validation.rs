extern crate graph;
use graph::graph::{check_uniqueness, validate};

#[test]
#[should_panic]
fn test_different_nodes_and_nodes_type() {
    validate(&["a".to_string(), "b".to_string(), "c".to_string()], &[], &[], &Some(vec![]), &None, &None)
}
