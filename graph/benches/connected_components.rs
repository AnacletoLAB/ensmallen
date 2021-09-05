#![feature(test)]
extern crate test;
use test::{black_box, Bencher};

extern crate graph;
use graph::test_utilities::load_cora;

#[bench]
fn bench_connected_components(b: &mut Bencher) {
    let cora = load_cora();
    b.iter(|| {
        let _ = black_box(cora.connected_components(Some(false)));
    });
}
