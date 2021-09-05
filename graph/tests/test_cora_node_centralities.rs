extern crate graph;

use graph::test_utilities::*;

#[test]
fn test_cora_node_centralities() -> Result<(), String> {
    let mut cora = load_cora();
    let _ = graph::test_utilities::test_node_centralities(&mut cora, Some(true));
    Ok(())
}
