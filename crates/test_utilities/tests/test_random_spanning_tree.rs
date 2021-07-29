use shared::*;
use graph::Graph;
use test_utilities::*;

#[test]
fn test_random_spanning_tree() -> Result<()> {
    let mut random_spanning_tree = Graph::generate_random_spanning_tree(
        None,
        None,
        Some(100),
        None,
        None,
        None,
        None,
        None,
        None,
    )
    .unwrap();
    assert!(random_spanning_tree.is_connected(Some(true)));
    let _ = default_test_suite(&mut random_spanning_tree, None);
    Ok(())
}
