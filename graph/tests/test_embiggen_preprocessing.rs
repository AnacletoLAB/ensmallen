extern crate graph;

use graph::test_utilities::*;

#[test]

fn test_regression_29() -> Result<(), String> {
    let mut graph = load_cora().unwrap();
    test_embiggen_preprocessing(&mut graph, false);
    Ok(())
}