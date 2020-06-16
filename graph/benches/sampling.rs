#![feature(test)]
extern crate test;
use test::Bencher;
use rand::rngs::mock::StepRng;
use rand::prelude::*;
use rayon::prelude::*;
use std::mem;
use rand::Rng;
use rand::distributions::WeightedIndex;

use ::core::cmp::Ordering;
use core::arch::x86_64::_rdtsc;

const NUMBER: u64 = 100000;


fn gen_random_vec() -> Vec<f64> {
    let mut rng = rand::thread_rng();
    let vals: Vec<u64> = (0..NUMBER).map(|_| rng.gen_range(0, NUMBER)).collect();
    let total: f64 = vals.iter().sum::<u64>() as f64;
    let weights = vals.iter().map(|x| *x as f64 / total).collect::<Vec<f64>>();
    //println!("{:?}", weights);
    weights
}

#[inline(always)]
fn gen_xorshift_random_float() -> f64 {
    let mut seed = unsafe{_rdtsc()};
    for _ in 0..2 {
        seed ^= seed << 17;
        seed ^= seed >> 7;
        seed ^= seed << 13;
    }
   seed as f64 / u64::max_value() as f64
}

#[bench]
fn test_gen_xorshift_random_float(b: &mut Bencher) {
    b.iter(|| {
        gen_xorshift_random_float()
    });
}

#[inline(always)]
fn rotl(x : u64, k: u64) -> u64{
	return (x << k) | (x >> (64 - k));
}


static mut s: [u64; 4] = [0xdeadbeefc0febabe, 0xdeadbeefc0febabe, 0xdeadbeefc0febabe, 0xdeadbeefc0febabe];

#[inline(always)]
pub fn xorshiro256plus_mul() -> f64{
    // based on xorshiro256+ which seems to be the fastest floating point generator
    // http://prng.di.unimi.it/xoshiro256plus.c
    // the conversion from u64 to f64 is taken from:
    // http://prng.di.unimi.it/
    // the informations about the structure of a f64 was taken from IEEE 754
    // https://standards.ieee.org/content/ieee-standards/en/standard/754-2019.html
    // https://en.wikipedia.org/wiki/Double-precision_floating-point_format
    // if this is still a bottleneck we can consider to implement
    // http://prng.di.unimi.it/xoshiro256+-vect-speed.c
    // which exploits avx to generate in parallel 8 random numbers and fill a 
    // cache with it
    unsafe {
    // normal xorshiro implementation
	let (result, _): (u64, bool) = s[0].overflowing_add(s[3]);

	let t: u64 = s[1] << 17;

	s[2] ^= s[0];
	s[3] ^= s[1];
	s[1] ^= s[2];
	s[0] ^= s[3];

	s[2] ^= t;

    s[3] = rotl(s[3], 45);
    // method proposed by vigna on http://prng.di.unimi.it/ 
    //(result >> 11) as f64 * 1.0e-53f64
    // the original value didn't work. after thinkering around we found
    // this value which seems to work
    (result >> 11) as f64 * 1.0e-16f64
    }
}

#[bench]
fn test_gen_xorshiro256plus_mul(b: &mut Bencher) {
    b.iter(|| {
        xorshiro256plus_mul()
    });
}

#[inline(always)]
fn xorshiro256plus() -> f64{
    /// based on
    /// https://experilous.com/1/blog/post/perfect-fast-random-floating-point-numbers
    /// http://prng.di.unimi.it/xoshiro256plus.c
    unsafe {
	let result: u64 = s[0] + s[3];

	let t: u64 = s[1] << 17;

	s[2] ^= s[0];
	s[3] ^= s[1];
	s[1] ^= s[2];
	s[0] ^= s[3];

	s[2] ^= t;

    s[3] = rotl(s[3], 45);
    
    let v: u64 = (result >> 11) | (1023 << 52);
    let r: f64 = f64::from_le_bytes(v.to_le_bytes());
    r - 1f64
    }
}


#[bench]
fn test_gen_xorshiro256plus(b: &mut Bencher) {
    b.iter(|| {
        xorshiro256plus()
    });
}

#[inline(always)]
fn extract_with_scan(weight: &Vec<f64>, rnd_val: f64) -> usize{
    let cumulative_sum: Vec<f64> = weight
    .iter()
    .scan(0f64, |acc, &x| {
        *acc = *acc + x;
        Some(*acc)
    })
    .collect();

    let rnd: f64 = rnd_val * cumulative_sum[cumulative_sum.len() - 1];

    // Find the first item which has a weight *higher* than the chosen weight.
    cumulative_sum.binary_search_by(
        |w|
            if *w <= rnd { 
                Ordering::Less 
            } else { 
                Ordering::Greater 
            }
        ).unwrap_err()
}

#[inline(always)]
fn extract_with_while(weight: &Vec<f64>, rnd_val: f64) -> usize{
    let mut cumulative_sum: Vec<f64> = Vec::with_capacity(weight.len());
    let mut total_weight = 0f64;
    for w in weight {
        total_weight += w;
        cumulative_sum.push(total_weight.clone());
    }

    let rnd: f64 = rnd_val * cumulative_sum[cumulative_sum.len() - 1];

    // Find the first item which has a weight *higher* than the chosen weight.
    cumulative_sum.binary_search_by(
        |w|
            if *w <= rnd { 
                Ordering::Less 
            } else { 
                Ordering::Greater 
            }
        ).unwrap_err()
}

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