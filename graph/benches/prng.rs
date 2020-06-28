#![feature(asm)]
use core::arch::x86_64::_rdtsc;

static mut XOR_SHIFT_GLOBAL_SEED: [f32; 8] = [80085.0, 12341251.0, 12341251.0, 12341251.0, 12341251.0, 12341251.0, 12341251.0, 12341251.0];

#[inline(always)]
pub fn xorshift() -> u64 {
    let mut seed = unsafe{_rdtsc()};
    for _ in 0..2 {
        seed ^= seed << 17;
        seed ^= seed >> 7;
        seed ^= seed << 13;
    }
    seed
}

#[inline(always)]
pub fn gen_xorshift_random_float() -> f64 {
    let v: u64 = (xorshift() >> 11) | (1023 << 52);
    let r: f64 = f64::from_le_bytes(v.to_le_bytes());
    r - 1f64
}

#[inline(always)]
fn rotl(x : u64, k: u64) -> u64{
	return (x << k) | (x >> (64 - k));
}

static mut GLOBAL_SEED: [u64; 4] = [0xdeadbeefc0febabe, 0xdeadbeefc0febabe, 0xdeadbeefc0febabe, 0xdeadbeefc0febabe];

#[inline(always)]
pub fn xorshiro256plus() -> u64{
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
        let (result, _): (u64, bool) = GLOBAL_SEED[0].overflowing_add(GLOBAL_SEED[3]);
    
        let t: u64 = GLOBAL_SEED[1] << 17;
    
        GLOBAL_SEED[2] ^= GLOBAL_SEED[0];
        GLOBAL_SEED[3] ^= GLOBAL_SEED[1];
        GLOBAL_SEED[1] ^= GLOBAL_SEED[2];
        GLOBAL_SEED[0] ^= GLOBAL_SEED[3];
    
        GLOBAL_SEED[2] ^= t;
    
        GLOBAL_SEED[3] = rotl(GLOBAL_SEED[3], 45);
        result
    }
}

#[inline(always)]
pub fn xorshiro256plus_mul() -> f64{
    // method proposed by vigna on http://prng.di.unimi.it/ 
    (xorshiro256plus() >> 11) as f64 * 2.0f64.powf(-53.0)
}


#[inline(always)]
/// based on
/// https://experilous.com/1/blog/post/perfect-fast-random-floating-point-numbers
/// http://prng.di.unimi.it/xoshiro256plus.c
pub fn xorshiro256plus_no_mul() -> f64{
    let result: u64 = xorshiro256plus();
    let v: u64 = (result >> 11) | (1023 << 52);
    let r: f64 = unsafe{f64::from_le_bytes(v.to_le_bytes())};
    r - 1f64
}
