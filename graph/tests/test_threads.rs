use graph::test_utilities::*;
use graph::set_num_threads;

#[test]
/// Test that everything runs properly in the PPI graph.
fn test_set_num_threads() {
    set_num_threads(4);
    let ppi = load_ppi(
        true,
        true,
        true,
        true,
        false,
        true,
    )
    .unwrap();
    default_test_suite(&ppi, false).unwrap();
}
