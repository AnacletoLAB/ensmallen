#![feature(test, asm)]
extern crate test;
use test::Bencher;
use rand::Rng;

use rayon::prelude::*;

mod prng;
use prng::*;

mod avx2_xorshift;
use avx2_xorshift::*;

const NUMBER: usize = 10000;

pub fn gen_random_usize_vec(num: usize, max: usize) -> Vec<usize> {
    let mut rng = rand::thread_rng();
    (0..num).map(|_| rng.gen_range(0, max)).collect()
}

pub fn gen_usize_xorshiro(num: usize, max: usize) -> Vec<u64> {
    (0..num).map(|_| xorshiro256plus()).collect()
}

pub fn gen_usize_par_xorshiro(num: usize, max: usize) -> Vec<u64> {
    (0..num).into_par_iter().map(|_| xorshiro256plus()).collect()
}

pub fn gen_while_xorshiro(num: usize, max: usize) -> Vec<u64> {
    let mut result = Vec::with_capacity(num);
    for _ in 0..num{
        result.push(xorshiro256plus());
    }
    result
}

pub fn optimized(num: usize, max: usize) -> Vec<u64> {
    let mut result = Vec::with_capacity(num);
    let mut seed: [u64; 16] = [
        0x1337, 0xdeadbeef, 0xc0febabe, 0xbad5eed,
        0x1337, 0xdeadbeef, 0xc0febabe, 0xbad5eed,
        0x1337, 0xdeadbeef, 0xc0febabe, 0xbad5eed,
        0x1337, 0xdeadbeef, 0xc0febabe, 0xbad5eed
    ];
    for i in 0..(num >> 4) {
        ax2_ss4_xorshift_assembly(& mut seed);
        for x in seed.iter() {
            result.push(*x);
        }
    }

    for i in (num >> 4)..num{
        result.push(xorshiro256plus());
    }
    result
}

#[bench]
fn test_gen_random_usize_vec(b: &mut Bencher) {
    b.iter(|| {
            gen_usize_xorshiro(NUMBER, NUMBER)
        }
    );
}

#[bench]
fn test_gen_usize_xorshiro(b: &mut Bencher) {
    b.iter(|| {
            gen_usize_xorshiro(NUMBER, NUMBER)
        }
    );
}


#[bench]
fn test_gen_while_xorshiro(b: &mut Bencher) {
    b.iter(|| {
            gen_while_xorshiro(NUMBER, NUMBER);
        }
    );
}

#[bench]
fn test_gen_usize_par_xorshiro(b: &mut Bencher) {
    b.iter(|| {
            gen_usize_par_xorshiro(NUMBER, NUMBER);
        }
    );
}

#[bench]
fn test_optimized(b: &mut Bencher) {
    b.iter(|| {
            optimized(NUMBER, NUMBER);
        }
    );
}