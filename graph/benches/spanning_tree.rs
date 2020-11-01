#![feature(test)]
extern crate test;
use test::{Bencher, black_box};
extern crate graph;
use graph::test_utilities::*;

#[bench]
fn bench_spanning_tree(b: &mut Bencher) {
    let graph = load_ppi(true, true, true, false, false, false).unwrap();

    b.iter(|| {
        for _ in 0..10 {
            black_box(graph.spanning_tree(1337, false, &None, false));
        }
    });
}
