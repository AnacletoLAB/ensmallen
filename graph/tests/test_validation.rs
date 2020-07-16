extern crate graph;
use graph::validate;
use std::collections::HashMap;

#[test]
fn test_validation() {
    assert!(validate(
        &[0, 1, 2],
        &[1, 2, 3],
        &(vec![(String::from("a"), 0), (String::from("b"), 1)]
            .iter()
            .cloned()
            .collect::<HashMap<_, _>>()),
        &[String::from("a"), String::from("b"), String::from("c")],
        &None,
        &None,
        &Some(vec![1.0, 1.0, 1.0]),
    )
    .is_err());
}

#[test]
fn test_validation_edge_duplication() {
    assert!(validate(
        &[0, 1, 1],
        &[1, 2, 2],
        &(vec![
            (String::from("a"), 0),
            (String::from("b"), 1),
            (String::from("c"), 2),
        ]
        .iter()
        .cloned()
        .collect::<HashMap<_, _>>()),
        &[String::from("a"), String::from("b"), String::from("c")],
        &None,
        &None,
        &Some(vec![1.0, 1.0, 1.0])
    )
    .is_err());
}

#[test]
fn test_validation_edge_duplication_with_edges() {
    assert!(validate(
        &[0, 1, 1],
        &[1, 2, 2],
        &(vec![
            (String::from("a"), 0),
            (String::from("b"), 1),
            (String::from("c"), 2),
        ]
        .iter()
        .cloned()
        .collect::<HashMap<_, _>>()),
        &[String::from("a"), String::from("b"), String::from("c")],
        &None,
        &Some(vec![1, 2, 2]),
        &Some(vec![1.0, 1.0, 1.0])
    )
    .is_err());
}

#[test]
fn test_validation_wrong_node_types_number() {
    assert!(validate(
        &[0, 1, 2],
        &[1, 2, 3],
        &(vec![
            (String::from("a"), 0),
            (String::from("b"), 1),
            (String::from("c"), 3),
        ]
        .iter()
        .cloned()
        .collect::<HashMap<_, _>>()),
        &[String::from("a"), String::from("b"), String::from("c")],
        &Some(vec![8, 6]),
        &None,
        &Some(vec![1.0, 1.0, 1.0]),
    )
    .is_err());
}

#[test]
fn test_validation_wrong_edge_types_number() {
    assert!(validate(
        &[0, 1, 2],
        &[1, 2, 3],
        &(vec![
            (String::from("a"), 0),
            (String::from("b"), 1),
            (String::from("c"), 3),
        ]
        .iter()
        .cloned()
        .collect::<HashMap<_, _>>()),
        &[String::from("a"), String::from("b"), String::from("c")],
        &None,
        &Some(vec![8, 6]),
        &Some(vec![1.0, 1.0, 1.0]),
    )
    .is_err());
}

#[test]
fn test_validation_wrong_weights_number() {
    assert!(validate(
        &[0, 1, 2],
        &[1, 2, 3],
        &(vec![
            (String::from("a"), 0),
            (String::from("b"), 1),
            (String::from("c"), 3),
        ]
        .iter()
        .cloned()
        .collect::<HashMap<_, _>>()),
        &[String::from("a"), String::from("b"), String::from("c")],
        &None,
        &None,
        &Some(vec![1.0, 1.0]),
    )
    .is_err());
}

#[test]
fn test_validation_wrong_weights_zeros() {
    assert!(validate(
        &[0, 1, 2],
        &[1, 2, 3],
        &(vec![
            (String::from("a"), 0),
            (String::from("b"), 1),
            (String::from("c"), 3),
        ]
        .iter()
        .cloned()
        .collect::<HashMap<_, _>>()),
        &[String::from("a"), String::from("b"), String::from("c")],
        &None,
        &None,
        &Some(vec![1.0, 1.0, 0.0]),
    )
    .is_err());
}

#[test]
fn test_validation_wrong_edges_with_non_existant_nodes() {
    assert!(validate(
        &[0, 1, 999],
        &[1, 2, 3],
        &(vec![
            (String::from("a"), 0),
            (String::from("b"), 1),
            (String::from("c"), 3),
        ]
        .iter()
        .cloned()
        .collect::<HashMap<_, _>>()),
        &[String::from("a"), String::from("b"), String::from("c")],
        &Some(vec![1, 2, 3]),
        &None,
        &Some(vec![1.0, 1.0, 1.0]),
    )
    .is_err());
}
