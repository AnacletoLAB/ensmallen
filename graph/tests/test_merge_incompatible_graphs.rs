extern crate graph;

use graph::test_utilities::*;

#[test]
fn test_merge_incompatible_graphs() -> Result<(), String> {
    let cora = load_cora().unwrap();
    let ppi = load_ppi(true, true, false, false, false, false);
    let directed_ppi = load_ppi(true, true, false, true, false, false);
    assert!((&cora | &ppi).is_ok());
    assert!((&ppi | &directed_ppi).is_err());
    assert!((&ppi
        | &ppi
            .remove(
                None, None, None, None, None, None, None, None, false, true, false, false, false,
                false
            )
            .unwrap())
        .is_err());
    assert!((&ppi
        | &ppi
            .remove(
                None, None, None, None, None, None, None, None, false, false, true, false, false,
                false
            )
            .unwrap())
        .is_err());
    Ok(())
}
