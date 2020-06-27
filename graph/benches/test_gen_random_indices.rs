#![feature(test)]
extern crate test;
use test::Bencher;
use rand::Rng;

use rayon::prelude::*;

mod prng;
use prng::*;

const NUMBER: usize = 10000;

pub fn gen_random_usize_vec(num: usize, max: usize) -> Vec<usize> {
    let mut rng = rand::thread_rng();
    (0..num).map(|_| rng.gen_range(0, max)).collect()
}

pub fn gen_usize_xorshiro(num: usize, max: usize) -> Vec<usize> {
    (0..num).map(|_| xorshiro256plus() as usize % max).collect()
}

pub fn gen_usize_par_xorshiro(num: usize, max: usize) -> Vec<usize> {
    (0..num).into_par_iter().map(|_| xorshiro256plus() as usize % max).collect()
}

#[bench]
fn test_gen_random_usize_vec(b: &mut Bencher) {
    let random_vec = 
    b.iter(|| {
            gen_usize_xorshiro(NUMBER, NUMBER);
        }
    );
}

#[bench]
fn test_gen_usize_xorshiro(b: &mut Bencher) {
    let random_vec = 
    b.iter(|| {
            gen_usize_xorshiro(NUMBER, NUMBER);
        }
    );
}

#[bench]
fn test_gen_usize_par_xorshiro(b: &mut Bencher) {
    let random_vec = 
    b.iter(|| {
            gen_usize_par_xorshiro(NUMBER, NUMBER);
        }
    );
}