extern crate graph;
use graph::xorshiro256plus;

#[test]
fn test_xorshiro256plus_validity() {
    for _ in 0..100000000 {
        let v = xorshiro256plus();
        assert!(v >= 0.0 && v <= 1.0);
    }
}
