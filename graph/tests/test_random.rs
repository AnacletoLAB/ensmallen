extern crate graph;
use graph::random_f64;

#[test]
fn test_random_float_validity() {
    for _ in 0..100000000 {
        let v = random_f64();
        assert!(v >= 0.0 && v <= 1.0);
    }
}
