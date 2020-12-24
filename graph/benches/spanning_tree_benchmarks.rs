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
            black_box(graph.spanning_arborescence(false).unwrap());
        }
    });
}

#[bench]
fn bench_spanning_arborescence_with_fast_graph(b: &mut Bencher) {
    let mut graph = load_ppi(true, true, true, false, false, false).unwrap();

    graph.enable(false, true, true, None).unwrap();

    b.iter(|| {
        for _ in 0..10 {
            black_box(graph.spanning_arborescence(false).unwrap());
        }
    });
}

#[bench]
fn bench_spanning_arborescence_kruskal(b: &mut Bencher) {
    let graph = load_ppi(true, true, true, false, false, false).unwrap();

    b.iter(|| {
        for _ in 0..10 {
            black_box(graph.spanning_arborescence_kruskal(false));
        }
    });
}

#[bench]
fn bench_spanning_arborescence_kruskal_with_fast_graph(b: &mut Bencher) {
    let mut graph = load_ppi(true, true, true, false, false, false).unwrap();

    graph.enable(false, true, true, None).unwrap();

    b.iter(|| {
        for _ in 0..10 {
            black_box(graph.spanning_arborescence_kruskal(false));
        }
    });
}
