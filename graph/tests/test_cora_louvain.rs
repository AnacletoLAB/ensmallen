extern crate graph;

use graph::test_utilities::*;

#[test]
fn test_cora_louvain() -> Result<(), String> {
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
    cora.get_undirected_louvain_community_detection(None, None, None, None)
        .unwrap();
    cora_with_no_words
        .get_undirected_louvain_community_detection(None, None, None, None)
        .unwrap();
    Ok(())
}
