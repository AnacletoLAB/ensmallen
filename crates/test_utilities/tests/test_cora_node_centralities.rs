use shared::*;
use test_utilities::*;
#[test]
fn test_cora_node_centralities() -> Result<()> {
    let mut cora = load_cora();
    let _ = test_node_centralities(&mut cora, Some(true));
    Ok(())
}
