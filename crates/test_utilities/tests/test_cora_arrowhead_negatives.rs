use shared::*;
use test_utilities::*;

#[test]
fn test_cora_arrowhead_negatives() -> Result<()> {
    let cora = load_cora();
    let mut arrowhead = cora.to_arrowhead();
    let _ = test_negative_edges_generation(&mut arrowhead, Some(true));
    Ok(())
}
