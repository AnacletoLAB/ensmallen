extern crate graph;
use graph::validate;
use hashbrown::{HashMap};

#[test]
#[should_panic]
fn test_validation() {
    validate(
        &vec![0, 1, 2], 
        &vec![1, 2, 3], 
        &(vec![
            (String::from("a"), 0),
            (String::from("b"), 1),
        ].iter().cloned().collect::<HashMap<_, _>>()), 
        &vec![String::from("a"), String::from("b"), String::from("c")],
        &None, 
        &None, 
        &Some(vec![1.0, 1.0, 1.0])
    ).unwrap();
}


#[test]
fn test_validation_edge_duplication() {
    assert!(validate(
        &vec![0, 1, 1], 
        &vec![1, 2, 2], 
        &(vec![
            (String::from("a"), 0),
            (String::from("b"), 1),
            (String::from("c"), 2),
        ].iter().cloned().collect::<HashMap<_, _>>()), 
        &vec![String::from("a"), String::from("b"), String::from("c")],
        &None, 
        &None, 
        &Some(vec![1.0, 1.0, 1.0])
    ).is_err());
}

#[test]
fn test_validation_edge_duplication_with_edges() {
    assert!(validate(
        &vec![0, 1, 1], 
        &vec![1, 2, 2], 
        &(vec![
            (String::from("a"), 0),
            (String::from("b"), 1),
            (String::from("c"), 2),
        ].iter().cloned().collect::<HashMap<_, _>>()), 
        &vec![String::from("a"), String::from("b"), String::from("c")],
        &None, 
        &Some(vec![1,2,2]), 
        &Some(vec![1.0, 1.0, 1.0])
    ).is_err());
}

#[test]
#[should_panic]
fn test_validation_wrong_node_types_number() {
    validate(
        &vec![0, 1, 2], 
        &vec![1, 2, 3], 
        &(vec![
            (String::from("a"), 0),
            (String::from("b"), 1),
            (String::from("c"), 3),
        ].iter().cloned().collect::<HashMap<_, _>>()), 
        &vec![String::from("a"), String::from("b"), String::from("c")],
        &Some(vec![8, 6]), 
        &None, 
        &Some(vec![1.0, 1.0, 1.0])
    ).unwrap();
}

#[test]
#[should_panic]
fn test_validation_wrong_edge_types_number() {
    validate(
        &vec![0, 1, 2], 
        &vec![1, 2, 3], 
        &(vec![
            (String::from("a"), 0),
            (String::from("b"), 1),
            (String::from("c"), 3),
        ].iter().cloned().collect::<HashMap<_, _>>()), 
        &vec![String::from("a"), String::from("b"), String::from("c")],
        &None,
        &Some(vec![8, 6]), 
        &Some(vec![1.0, 1.0, 1.0])
    ).unwrap();
}


#[test]
#[should_panic]
fn test_validation_wrong_weights_number() {
    validate(
        &vec![0, 1, 2], 
        &vec![1, 2, 3], 
        &(vec![
            (String::from("a"), 0),
            (String::from("b"), 1),
            (String::from("c"), 3),
        ].iter().cloned().collect::<HashMap<_, _>>()), 
        &vec![String::from("a"), String::from("b"), String::from("c")],
        &None,
        &None, 
        &Some(vec![1.0, 1.0])
    ).unwrap();
}

#[test]
#[should_panic]
fn test_validation_wrong_weights_zeros() {
    validate(
        &vec![0, 1, 2], 
        &vec![1, 2, 3], 
        &(vec![
            (String::from("a"), 0),
            (String::from("b"), 1),
            (String::from("c"), 3),
        ].iter().cloned().collect::<HashMap<_, _>>()), 
        &vec![String::from("a"), String::from("b"), String::from("c")],
        &None,
        &None, 
        &Some(vec![1.0, 1.0, 0.0])
    ).unwrap();
}

#[test]
#[should_panic]
fn test_validation_wrong_edges_with_non_existant_nodes() {
    validate(
        &vec![0, 1, 999], 
        &vec![1, 2, 3], 
        &(vec![
            (String::from("a"), 0),
            (String::from("b"), 1),
            (String::from("c"), 3),
        ].iter().cloned().collect::<HashMap<_, _>>()), 
        &vec![String::from("a"), String::from("b"), String::from("c")],
        &Some(vec![1,2, 3]),
        &None,
        &Some(vec![1.0, 1.0, 1.0])
    ).unwrap();
}
