extern crate graph;

use graph::{test_utilities::*, utils::get_loading_bar};

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
    test_graph_properties(&cora, None);
    assert_eq!(cora.get_diameter(Some(false), None).unwrap(), f32::INFINITY);
    assert_eq!(cora.get_diameter_naive(Some(true), None).unwrap(), 6.0);
    // THIS IS NOT DETERMINISTIC
    let n = 100;
    let pb = get_loading_bar(true, "Executing diameter test", n);
    for _ in 0..n {
        pb.inc(1);
        assert_eq!(cora.get_diameter(Some(true), None).unwrap(), 6.0);
    }
    assert_eq!(
        cora_with_no_words.get_diameter(Some(true), None).unwrap(),
        19.0
    );
    Ok(())
}
