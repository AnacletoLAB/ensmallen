extern crate graph;

use graph::test_utilities::*;

#[test]

fn test_cora_embiggen_preprocessing() -> Result<(), String> {
    let mut graph = load_cora().unwrap();
    let _ = test_embiggen_preprocessing(&mut graph, false);
    Ok(())
}
