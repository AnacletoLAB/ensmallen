extern crate graph;

use graph::test_utilities::*;

#[test]
fn test_merge_incompatible_graphs() -> Result<(), String> {
    let cora = load_cora();
    let ppi = load_ppi(true, true, true, false, false, false);
    let directed_ppi = load_ppi(true, true, false, true, false, false);
    assert!((&cora | &ppi.remove_edge_weights().unwrap()).is_ok());
    assert!((&ppi | &directed_ppi).is_err());
    assert!((&ppi | &ppi.remove_node_types().unwrap()).is_err());
    assert!((&ppi | &ppi.remove_edge_types(Some(false)).unwrap()).is_err());
    assert!((&ppi | &ppi.remove_edge_weights().unwrap()).is_err());
    Ok(())
}
