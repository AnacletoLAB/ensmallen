#![feature(test)]
extern crate test;
use test::Bencher;

const NUMBER: u64 = 100000;

mod utils;
use utils::*;

#[bench]
fn test_naife_cumulative_sum(b: &mut Bencher) {
    let random_vec = gen_random_vec(NUMBER);
    b.iter(|| {
        let mut cumulative_sum: Vec<f64> = Vec::with_capacity(random_vec.len());
        let mut total_weight = 0f64;
        for w in &random_vec {
            total_weight += w;
            cumulative_sum.push(total_weight.clone());
        }
    });
}

#[bench]
fn test_scan_cumulative_sum(b: &mut Bencher) {
    let random_vec = gen_random_vec(NUMBER);
    b.iter(|| {
        let cumulative_sum: Vec<f64> = random_vec
        .iter()
        .scan(0f64, |acc, &x| {
            *acc = *acc + x;
            Some(*acc)
        })
        .collect();
    });
}