#![feature(test)]
extern crate test;
use test::{black_box, Bencher};
extern crate graph;

use graph::test_utilities::{first_order_walker, load_ppi};
use graph::*;
use rayon::iter::ParallelIterator;

#[bench]
fn bench_slow(b: &mut Bencher) {
    let graph = load_ppi(false, false, false, false, false, false).unwrap();
    let walker = first_order_walker(&graph).unwrap();

    b.iter(|| {
        for _ in 0..10 {
            black_box(
                graph
                    .random_walks_iter(1, &walker)
                    .unwrap()
                    .collect::<Vec<Vec<NodeT>>>(),
            );
        }
    });
}

#[bench]
fn bench_fast(b: &mut Bencher) {
    let mut graph = load_ppi(false, true, false, false, false, false).unwrap();
    let walker = first_order_walker(&graph).unwrap();

    graph.enable_fast_walk(true, true, None).unwrap();

    b.iter(|| {
        for _ in 0..10 {
            black_box(
                graph
                    .random_walks_iter(1, &walker)
                    .unwrap()
                    .collect::<Vec<Vec<NodeT>>>(),
            );
        }
    });
}

fn bench_cache(b: &mut Bencher, level: f64) {
    let mut graph = load_ppi(false, true, false, false, false, false).unwrap();
    let walker = first_order_walker(&graph).unwrap();

    graph.enable_fast_walk(false, false, Some(level)).unwrap();

    b.iter(|| {
        for _ in 0..10 {
            black_box(
                graph
                    .random_walks_iter(1, &walker)
                    .unwrap()
                    .collect::<Vec<Vec<NodeT>>>(),
            );
        }
    });
}

#[bench]
fn bench_cache_05(b: &mut Bencher) {
    bench_cache(b, 0.05)
}

#[bench]
fn bench_cache_25(b: &mut Bencher) {
    bench_cache(b, 0.25)
}

#[bench]
fn bench_cache_50(b: &mut Bencher) {
    bench_cache(b, 0.5)
}

#[bench]
fn bench_cache_75(b: &mut Bencher) {
    bench_cache(b, 0.75)
}

#[bench]
fn bench_cache_95(b: &mut Bencher) {
    bench_cache(b, 0.95)
}
