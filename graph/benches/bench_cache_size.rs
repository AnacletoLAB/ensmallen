#![feature(test)]
extern crate test;
use graph::test_utilities::*;
use test::{black_box, Bencher};

#[bench]
fn bench_no_cache(b: &mut Bencher) {
    let ppi = load_ppi(true, true, true, false, false, false).unwrap();
    let second_order = second_order_walker(&ppi, false).unwrap();
    b.iter(|| ppi.node2vec(&second_order, 256, 3).unwrap());
}
