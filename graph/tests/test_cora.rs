extern crate graph;

use graph::test_utilities::*;

#[test]
fn test_cora() -> Result<(), String> {
    let mut cora = load_cora();
    let _ = graph::test_utilities::default_test_suite(&mut cora, None);
    Ok(())
}
