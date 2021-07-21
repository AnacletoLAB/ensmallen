#![feature(test)]
extern crate test;
use test::{black_box, Bencher};

extern crate graph;
use graph::{get_minmax_node_from_numeric_edge_list, EdgeT};

fn load_edge_list_and_apply_operation(
    load_edge_list_in_parallel: bool,
) -> Result<(EdgeT, EdgeT), String> {
    get_minmax_node_from_numeric_edge_list(
        "tests/data/macaque.tsv",
        Some("\t".to_string()),
        Some(false),
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        Some(load_edge_list_in_parallel),
        Some(false),
        None,
    )
}

#[bench]
fn bench_load_sequential(b: &mut Bencher) {
    b.iter(|| {
        let _ = black_box(load_edge_list_and_apply_operation(false));
    });
}

#[bench]
fn bench_load_parallel(b: &mut Bencher) {
    b.iter(|| {
        let _ = black_box(load_edge_list_and_apply_operation(true));
    });
}
