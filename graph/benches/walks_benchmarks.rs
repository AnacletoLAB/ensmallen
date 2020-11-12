use criterion::{black_box, criterion_group, criterion_main, Criterion, Bencher};
extern crate graph;
use graph::test_utilities::*;

use graph::*;
use graph::test_utilities::load_ppi;
use rayon::iter::ParallelIterator;


fn walker(graph: &Graph) -> Result<WalksParameters, String> {
    Ok(WalksParameters::new(10)?
        .set_iterations(Some(1))?
        .set_return_weight(Some(1.0))?
        .set_explore_weight(Some(2.0))?
        .set_change_edge_type_weight(Some(2.0))?
        .set_change_node_type_weight(Some(2.0))?
        .set_dense_node_mapping(Some(graph.get_dense_node_mapping()))
        .set_random_state(Some(43)))
}

fn bench_slow(c: &mut Criterion) {
    c.bench_function("walk_slow", |b| {
        let mut graph = load_ppi(true, true, true, false, false, false).unwrap();
        let walker = walker(&graph).unwrap();
        
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
        let walker = walker(&graph).unwrap();

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
    let walker = walker(&graph).unwrap();

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

fn bench_spanning_tree(c: &mut Criterion) {
    c.bench_function("spanning_tree", |b| {
        let graph = load_ppi(true, true, true, false, false, false).unwrap();

        b.iter(|| {
            for _ in 0..10 {
                black_box(graph.random_spanning_tree(1337, false, &None, false));
            }
        });
    });
}

criterion_group!(benches, bench_spanning_tree, bench_fast, bench_caches, bench_slow);
criterion_main!(benches);