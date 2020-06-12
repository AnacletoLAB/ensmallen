extern crate graph;
use graph::validate;
use std::{
    collections::{HashMap}
};

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
    )
}
