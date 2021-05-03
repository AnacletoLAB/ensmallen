extern crate graph;

use graph::test_utilities::*;

#[test]
fn test_cora_diameter() -> Result<(), String> {
    let mut cora = load_cora().unwrap();
    assert_eq!(cora.get_unweighted_diameter(None, None).unwrap(), 6);
    Ok(())
}
