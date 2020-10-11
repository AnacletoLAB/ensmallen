#![feature(test)]
extern crate test;
use test::{Bencher, black_box};

extern crate graph;
use graph::test_utilities::*;

#[bench]
fn bench_sample_negatives(b: &mut Bencher) {
    let ppi = load_ppi(
        false,
        false,
        true,
        false,
        false,
        false,
    )
    .unwrap();
    println!("SAMPLE NEGATIVESI");
    b.iter(|| {
        ppi.sample_negatives(43, 10_000, false, false)
    });
}