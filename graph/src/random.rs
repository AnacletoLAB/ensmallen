use super::types::*;
use ::core::cmp::Ordering;

// global static seed, this could be moved inside a struct
// WARNING
// the current implementation is not thread safe because we
// mutate a shared state between threads without any locks.
// This should not create any problem since we do not need
// a strong PRNG so for speed sake it's intentionally let
// this way.
// The only real problem could be that we lose determinism
static mut seed: [u64; 4] = [0xdeadbeefc0febabe, 0xdeadbeefc0febabe, 0xdeadbeefc0febabe, 0xdeadbeefc0febabe];


#[inline(always)]
fn rotl(x : u64, k: u64) -> u64{
	return (x << k) | (x >> (64 - k));
}

#[inline(always)]
pub fn xorshiro256plus() -> f64{
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
	let (result, _): (u64, bool) = seed[0].overflowing_add(seed[3]);

	let t: u64 = seed[1] << 17;

	seed[2] ^= seed[0];
	seed[3] ^= seed[1];
	seed[1] ^= seed[2];
	seed[0] ^= seed[3];

	seed[2] ^= t;

    seed[3] = rotl(seed[3], 45);
    // method proposed by vigna on http://prng.di.unimi.it/ 
    // (result >> 11) as f64 * 1.0e-53f64
    // this method doesn't seems to work in rust
    // we found a differnt value which seems to work
    // (result >> 11) as f64 * 1.0e-16f64
    // but we prefer to craft the float by hand 
    // (which is also expained on the same page of the other method)
    // because it's faster (even though Vigna says that on modern hardware this
    // difference in performances should be negligible)
    let v: u64 = (result >> 11) | (1023 << 52);
    let r: f64 = f64::from_le_bytes(v.to_le_bytes());
    r - 1f64
    }
}

pub fn sample(weights: &Vec<WeightT>) -> usize {
    let mut cumulative_sum: Vec<f64> = Vec::with_capacity(weights.len());
    let mut total_weight = 0f64;
    for w in weights {
        total_weight += w;
        cumulative_sum.push(total_weight.clone());
    }

    let rnd: f64 = xorshiro256plus() * cumulative_sum[cumulative_sum.len() - 1];

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