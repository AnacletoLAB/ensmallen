extern crate graph;

use graph::test_utilities::*;

#[test]
fn test_cora_arrowhead_negatives() -> Result<(), String> {
    let cora = load_cora();
    let mut arrowhead = cora.to_arrowhead(Some(true));
    let _ = test_negative_edges_generation(&mut arrowhead, Some(true));
    Ok(())
}
