extern crate graph;

use graph::test_utilities::*;

#[test]
fn test_cora_diameter() -> Result<(), String> {
    let cora = load_cora().unwrap();
    assert_eq!(
        cora.get_unweighted_diameter(Some(false), None).unwrap(),
        f64::INFINITY
    );
    assert_eq!(cora.get_unweighted_diameter(Some(true), None).unwrap(), 6.0);
    Ok(())
}
