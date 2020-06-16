#![feature(test)]
extern crate test;
use test::Bencher;
use rand::prelude::*;

mod prng;
use prng::*;
mod sampling;
use sampling::*;
mod utils;
use utils::*;

use rand::distributions::WeightedIndex;

#[bench]
fn using_weighted_index_sample(b: &mut Bencher) {
    let random_vec = gen_random_vec();
    b.iter(|| {
        WeightedIndex::new(&random_vec)
            .unwrap()
            .sample(&mut thread_rng())
    });
}


#[bench]
fn using_xorshift_and_scan(b: &mut Bencher) {
    let random_vec = gen_random_vec();
    b.iter(|| {
        extract_with_scan(&random_vec, gen_xorshift_random_float())
    })
}

#[bench]
fn using_xorshiro256plus_and_scan(b: &mut Bencher) {
    let random_vec = gen_random_vec();
    b.iter(|| {
        extract_with_scan(&random_vec, xorshiro256plus())
    })
}

#[bench]
fn using_xorshiro256plus_mul_and_scan(b: &mut Bencher) {
    let random_vec = gen_random_vec();
    b.iter(|| {
        extract_with_scan(&random_vec, xorshiro256plus_mul())
    })
}

#[bench]
fn using_xorshift_and_while(b: &mut Bencher) {
    let random_vec = gen_random_vec();
    b.iter(|| {
        extract_with_while(&random_vec, gen_xorshift_random_float())
    })
}

#[bench]
fn using_xorshiro256plus_and_while(b: &mut Bencher) {
    let random_vec = gen_random_vec();
    b.iter(|| {
        extract_with_while(&random_vec, xorshiro256plus())
    })
}

#[bench]
fn using_xorshiro256plus_mul_and_while(b: &mut Bencher) {
    let random_vec = gen_random_vec();
    b.iter(|| {
        extract_with_while(&random_vec, xorshiro256plus_mul())
    })
}