#![feature(test)]
extern crate test;
use test::Bencher;

mod prng;
use prng::*;

#[bench]
fn test_gen_xorshift_random_float(b: &mut Bencher) {
    b.iter(|| {
        gen_xorshift_random_float()
    });
}

#[bench]
fn test_gen_xorshiro256plus_mul(b: &mut Bencher) {
    b.iter(|| {
        xorshiro256plus_mul()
    });
}

#[bench]
fn test_gen_xorshiro256plus(b: &mut Bencher) {
    b.iter(|| {
        xorshiro256plus()
    });
}