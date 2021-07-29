use shared::*;
use test_utilities::*;

#[test]
fn test_cora() -> Result<()> {
    let mut cora = load_cora();
    let _ = default_test_suite(&mut cora, None);
    Ok(())
}
