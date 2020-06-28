#![feature(test, asm)]
extern crate test;
use test::Bencher;
use rand::prelude::*;
use ::core::cmp::Ordering;


mod prng;
use prng::*;
mod sampling;
use sampling::*;
mod utils;
use utils::*;

mod cumulative_sum_sse_128_f64;
use cumulative_sum_sse_128_f64::*;

const NUMBER: u64 = 10;

use rand::distributions::WeightedIndex;

#[bench]
fn using_weighted_index_sample(b: &mut Bencher) {
    let random_vec = gen_random_f64_vec(NUMBER);
    b.iter(|| {
        WeightedIndex::new(&random_vec)
            .unwrap()
            .sample(&mut thread_rng())
    });
}

#[bench]
fn using_scan(b: &mut Bencher) {
    let random_vec = gen_random_f64_vec(NUMBER);
    b.iter(|| {
        extract_with_scan(&random_vec, xorshiro256plus_no_mul())
    })
}

#[bench]
fn using_while(b: &mut Bencher) {
    let random_vec = gen_random_f64_vec(NUMBER);
    b.iter(|| {
        extract_with_while(&random_vec, xorshiro256plus_no_mul())
    })
}

#[bench]
fn using_sse(b: &mut Bencher) {
    let mut random_vec = gen_random_f64_vec(NUMBER);
    b.iter(|| {
        extract_with_sse(& mut random_vec, xorshiro256plus_no_mul())
    })
}

#[inline(always)]
pub fn extract_with_sse(weights:  & mut Vec<f64>, frnd: f64) -> usize {
    if weights.len() == 1{
        return 0;
    }

    // pad the vector with zeros so that its length is a multiple of 4
    // so we can transform
    for _ in 0..(4 - (weights.len() & 3)) & 3 {
        weights.push(0.0);
    }

    #[cfg(all(any(target_arch = "x86", target_arch = "x86_64"),
    target_feature = "sse"))]
    let cumulative_sum: Vec<f64> = sse_128_f64_cumulative_sum(&weights);

    #[cfg(not(all(any(target_arch = "x86", target_arch = "x86_64"),
    target_feature = "sse")))]
    let cumulative_sum: Vec<f64> = unrolled_cumulative_f64_sum(&weights);
    

    let rnd: f64 = frnd * cumulative_sum[cumulative_sum.len() - 1];

    // Find the first item which has a weight *higher* than the chosen weight.
    match cumulative_sum
        .binary_search_by(|w| {
            if *w <= rnd {
                Ordering::Less
            } else {
                Ordering::Greater
            }
        })
    {
        // this could be an unwrap_err but there is a small chance that
        // the value could exactly match one of the cumulative sums
        // and therefore return Ok.
        Ok(g) => g,
        Err(g) => g 
    }
}
