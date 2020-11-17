#![feature(test)]
extern crate test;
use test::{black_box, Bencher};
extern crate graph;
use graph::test_utilities::load_ppi;

#[bench]
fn bench_link_prediction(b: &mut Bencher) {
    let graph = load_ppi(true, true, true, false, false, false).unwrap();

    b.iter(|| {
        for _ in 0..10 {
            black_box(graph.link_prediction(0, 4096, 1.0, false, None).unwrap());
        }
    });
}

#[bench]
fn bench_link_prediction_fast(b: &mut Bencher) {
    let mut graph = load_ppi(true, true, true, false, false, false).unwrap();

    graph.enable(true, true, true, None).unwrap();

    b.iter(|| {
        for _ in 0..10 {
            black_box(graph.link_prediction(0, 4096, 1.0, false, None).unwrap());
        }
    });
}
