#![feature(test)]
extern crate test;
use test::{black_box, Bencher};

extern crate graph;
use graph::test_utilities::*;

#[bench]
fn bench_spanning_arborescence(b: &mut Bencher) {
    let mut graph = load_cora().unwrap();
    b.iter(|| {
        let _ = black_box(test_spanning_arborescence_bader(&mut graph, false));
    });
}

#[bench]
fn bench_graph_properties(b: &mut Bencher) {
    let mut graph = load_cora().unwrap();
    b.iter(|| {
        let _ = black_box(test_graph_properties(&mut graph, false));
    });
}

#[bench]
fn bench_random_walks(b: &mut Bencher) {
    let mut graph = load_cora().unwrap();
    b.iter(|| {
        let _ = black_box(test_random_walks(&mut graph));
    });
}

#[bench]
fn bench_edge_holdouts(b: &mut Bencher) {
    let mut graph = load_cora().unwrap();
    b.iter(|| {
        let _ = black_box(test_edge_holdouts(&mut graph, false));
    });
}

#[bench]
fn bench_remove_components(b: &mut Bencher) {
    let mut graph = load_cora().unwrap();
    b.iter(|| {
        let _ = black_box(test_remove_components(&mut graph, false));
    });
}

#[bench]
fn bench_kfold(b: &mut Bencher) {
    let mut graph = load_cora().unwrap();
    b.iter(|| {
        let _ = black_box(test_kfold(&mut graph));
    });
}

#[bench]
fn bench_negative_edges_generation(b: &mut Bencher) {
    let mut graph = load_cora().unwrap();
    b.iter(|| {
        let _ = black_box(test_negative_edges_generation(&mut graph, false));
    });
}

#[bench]
fn bench_subgraph_generation(b: &mut Bencher) {
    let mut graph = load_cora().unwrap();
    b.iter(|| {
        let _ = black_box(test_subgraph_generation(&mut graph, false));
    });
}

#[bench]
fn bench_dump_graph(b: &mut Bencher) {
    let mut graph = load_cora().unwrap();
    b.iter(|| {
        let _ = black_box(test_dump_graph(&mut graph, false));
    });
}

#[bench]
fn bench_embiggen_preprocessing(b: &mut Bencher) {
    let mut graph = load_cora().unwrap();
    b.iter(|| {
        let _ = black_box(test_embiggen_preprocessing(&mut graph, false));
    });
}

#[bench]
fn bench_graph_filter(b: &mut Bencher) {
    let mut graph = load_cora().unwrap();
    b.iter(|| {
        let _ = black_box(test_graph_filter(&mut graph, false));
    });
}

#[bench]
fn bench_edgelist_generationr(b: &mut Bencher) {
    let mut graph = load_cora().unwrap();
    b.iter(|| {
        let _ = black_box(test_edgelist_generation(&mut graph));
    });
}

#[bench]
fn bench_nodelabel_holdouts(b: &mut Bencher) {
    let mut graph = load_cora().unwrap();
    b.iter(|| {
        let _ = black_box(test_nodelabel_holdouts(&mut graph));
    });
}

#[bench]
fn bench_edgelabel_holdouts(b: &mut Bencher) {
    let mut graph = load_cora().unwrap();
    b.iter(|| {
        let _ = black_box(test_edgelabel_holdouts(&mut graph));
    });
}

#[bench]
fn bench_graph_removes(b: &mut Bencher) {
    let mut graph = load_cora().unwrap();
    b.iter(|| {
        let _ = black_box(test_graph_removes(&mut graph, false));
    });
}

#[bench]
fn bench_clone_and_setters(b: &mut Bencher) {
    let mut graph = load_cora().unwrap();
    b.iter(|| {
        let _ = black_box(test_clone_and_setters(&mut graph));
    });
}