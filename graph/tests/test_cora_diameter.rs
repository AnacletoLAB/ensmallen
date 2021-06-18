extern crate graph;

use graph::test_utilities::*;

#[test]
fn test_cora_diameter() -> Result<(), String> {
    let cora = load_cora();
    let cora_with_no_words = cora
        .filter_from_names(
            None,
            None,
            None,
            None,
            None,
            Some(vec![Some("Word".to_string())]),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
        )
        .unwrap();
    assert_eq!(
        cora.get_diameter(Some(false), None).unwrap(),
        f64::INFINITY
    );
    assert_eq!(cora.get_diameter(Some(true), None).unwrap(), 6.0);
    assert_eq!(
        cora_with_no_words
            .get_diameter(Some(true), None)
            .unwrap(),
        19.0
    );
    Ok(())
}
