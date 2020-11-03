#![feature(test)]
extern crate test;
use test::{Bencher, black_box};
extern crate graph;
use graph::test_utilities::*;

extern crate graph;
use graph::*;
use graph::test_utilities::{load_ppi, first_order_walker};
use rayon::iter::ParallelIterator;

#[bench]
fn bench_slow(b: &mut Bencher) {
    let mut graph = load_ppi(true, true, true, false, false, false).unwrap();
    let walker = first_order_walker(&graph).unwrap();
    
    b.iter(|| {
        for _ in 0..10 {
            black_box(
                graph.random_walks_iter(1, &walker).unwrap().collect::<Vec<Vec<NodeT>>>()
            );
        }
    });
}

#[bench]
fn bench_fast(b: &mut Bencher) {
    let mut graph = load_ppi(true, true, true, false, false, false).unwrap();
    let walker = first_order_walker(&graph).unwrap();

    graph.enable_fast_walk(true, true, None);
    
    b.iter(|| {
        for _ in 0..10 {
            black_box(
                graph.random_walks_iter(1, &walker).unwrap().collect::<Vec<Vec<NodeT>>>()
            );
        }
    });
}

#[bench]
fn bench_cache_05(b: &mut Bencher) {
    let mut graph = load_ppi(true, true, true, false, false, false).unwrap();
    let walker = first_order_walker(&graph).unwrap();

    graph.enable_fast_walk(false, false, Some(0.05));
    
    b.iter(|| {
        for _ in 0..10 {
            black_box(
                graph.random_walks_iter(1, &walker).unwrap().collect::<Vec<Vec<NodeT>>>()
            );
        }
    });
}
