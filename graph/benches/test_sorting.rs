#![feature(test)]
extern crate test;
use test::Bencher;
use rand::prelude::*;

const NUMBER: u64 = 100000;

mod sorting;
use sorting::*;
mod utils;
use utils::*;

#[bench]
fn using_parallel_unstable_sort(b: &mut Bencher) {
    let random_vec = gen_random_u64_vec(NUMBER);
    b.iter(|| {
        parallel_unstable_sorting(&random_vec);
    });
}

#[bench]
fn using_sequential_unstable_sort(b: &mut Bencher) {
    let random_vec = gen_random_u64_vec(NUMBER);
    b.iter(|| {
        sequential_unstable_sorting(&random_vec);
    });
}

#[bench]
fn using_parallel_stable_sort(b: &mut Bencher) {
    let random_vec = gen_random_u64_vec(NUMBER);
    b.iter(|| {
        parallel_stable_sorting(&random_vec);
    });
}

#[bench]
fn using_sequential_stable_sort(b: &mut Bencher) {
    let random_vec = gen_random_u64_vec(NUMBER);
    b.iter(|| {
        sequential_stable_sorting(&random_vec);
    });
}
