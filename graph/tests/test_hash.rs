use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use graph::test_utilities::*;

#[test]
/// Test that everything runs properly in the PPI graph.
fn test_hash() {
    let ppi = load_ppi(true, true, true, false, true, false);

    let mut hasher = DefaultHasher::new();
    ppi.hash(&mut hasher);
    let h1 = hasher.finish();

    let mut hasher = DefaultHasher::new();
    ppi.hash(&mut hasher);
    let h2 = hasher.finish();

    assert_eq!(h1, h2);

    let ppi2 = load_ppi(true, false, true, false, true, false);

    let mut hasher = DefaultHasher::new();
    ppi2.hash(&mut hasher);
    let h3 = hasher.finish();

    assert!(h1 != h3);
}
