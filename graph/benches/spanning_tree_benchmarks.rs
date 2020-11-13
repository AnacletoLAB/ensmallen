#![feature(test)]
extern crate test;
use test::{black_box, Bencher};
extern crate graph;
use graph::test_utilities::load_ppi;

#[bench]
fn bench_spanning_arborescence(b: &mut Bencher) {
    let graph = load_ppi(true, true, true, false, false, false).unwrap();

    b.iter(|| {
        for _ in 0..10 {
            black_box(graph.spanning_arborescence().unwrap());
        }
    });
}

#[bench]
fn bench_spanning_arborescence_with_fast_graph(b: &mut Bencher) {
    let mut graph = load_ppi(true, true, true, false, false, false).unwrap();

    graph.enable_fast_walk(true, true, None).unwrap();

    b.iter(|| {
        for _ in 0..10 {
            black_box(graph.spanning_arborescence().unwrap());
        }
    });
}

#[bench]
fn bench_random_spanning_tree(b: &mut Bencher) {
    let graph = load_ppi(true, true, true, false, false, false).unwrap();

    b.iter(|| {
        for _ in 0..10 {
            black_box(graph.random_spanning_tree(42, false, &None, false));
        }
    });
}

#[bench]
fn bench_random_spanning_tree_with_fast_graph(b: &mut Bencher) {
    let mut graph = load_ppi(true, true, true, false, false, false).unwrap();

    graph.enable_fast_walk(true, true, None).unwrap();

    b.iter(|| {
        for _ in 0..10 {
            black_box(graph.random_spanning_tree(42, false, &None, false));
        }
    });
}
