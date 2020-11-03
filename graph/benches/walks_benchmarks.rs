use criterion::{black_box, criterion_group, criterion_main, Criterion, Bencher};
extern crate graph;
use graph::test_utilities::*;

use graph::*;
use graph::test_utilities::{load_ppi, first_order_walker};
use rayon::iter::ParallelIterator;


fn bench_slow(c: &mut Criterion) {
    c.bench_function("walk_slow", |b| {
        let mut graph = load_ppi(true, true, true, false, false, false).unwrap();
        let walker = first_order_walker(&graph).unwrap();
        
        b.iter(|| {
            for _ in 0..10 {
                black_box(
                    graph.random_walks_iter(1, &walker).unwrap().collect::<Vec<Vec<NodeT>>>()
                );
            }
        });
    });
}


fn bench_fast(c: &mut Criterion) {
    c.bench_function("walk_fast", |b| {
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
    });
}

fn bench_cache(b: &mut Bencher, level: f64) {
    let mut graph = load_ppi(true, true, true, false, false, false).unwrap();
    let walker = first_order_walker(&graph).unwrap();

    graph.enable_fast_walk(false, false, Some(level)).unwrap();
    
    b.iter(|| {
        for _ in 0..10 {
            black_box(
                graph.random_walks_iter(1, &walker).unwrap().collect::<Vec<Vec<NodeT>>>()
            );
        }
    });
}

fn bench_caches(c: &mut Criterion) {
    for level in &[0.05, 0.25, 0.50, 0.75, 0.95] {
        c.bench_function(&format!("walk_cache_{}", level), |b| {
            bench_cache(b, *level)
        });
    }
}

criterion_group!(benches, bench_fast, bench_caches, bench_slow);
criterion_main!(benches);