use core::arch::x86_64::_rdtsc;


#[cfg(all(
    any(target_arch = "x86", target_arch = "x86_64"),
    target_feature = "sse"
))]
use core::arch::x86_64::{
    // info can be found at https://software.intel.com/sites/landingpage/IntrinsicsGuide
    __m128,
    // sum two vector of f32
    _mm_add_ps,
    // cast __m128 to __m128i
    // see _mm_castsi128_ps
    _mm_castps_si128,
    // cast __m128i  to __m128
    // it's only for compilation, it does not gen instructions
    _mm_castsi128_ps,
    // Memory -> Vec (MUST be 16-bytes aligned)
    _mm_load_ps,
    // Memory -> Vec but slower
    _mm_loadu_ps,
    // set vec to zero
    _mm_setzero_ps,
    // Shiffle the vecotr according to the mask given
    _mm_shuffle_ps,
    // shift vector left and insert zeros
    _mm_slli_si128,
    _mm_srli_si128,
    // Vec -> Memory (MUST be 16-bytes aligned)
    _mm_store_ps,
    // Vec -> Memory but slower
    _mm_storeu_ps,
    _mm_xor_si128,
};

static mut XOR_SHIFT_GLOBAL_SEED: [f32; 8] = [80085.0, 12341251.0, 12341251.0, 12341251.0, 12341251.0, 12341251.0, 12341251.0, 12341251.0];

#[cfg(all(
    any(target_arch = "x86", target_arch = "x86_64"),
    target_feature = "sse"
))]
#[inline(always)]
pub fn sse_xorshift_random_float() -> [f32; 8] {
    unsafe {
        // load the seeds into a 128bit register
        let mut tmp = _mm_castps_si128(_mm_load_ps(XOR_SHIFT_GLOBAL_SEED.as_ptr()));

        // seed ^= seed << 17;
        let mut shifted = _mm_slli_si128(tmp, 17);
        tmp = _mm_xor_si128(tmp, shifted);

        //seed ^= seed >> 7;
        let mut shifted = _mm_srli_si128(tmp, 7);
        tmp = _mm_xor_si128(tmp, shifted);

        // seed ^= seed << 13;
        let mut shifted = _mm_slli_si128(tmp, 13);
        tmp = _mm_xor_si128(tmp, shifted);

        // Store the result inside of v
        _mm_store_ps(XOR_SHIFT_GLOBAL_SEED.as_mut_ptr(), _mm_castsi128_ps(tmp));
        
        // reutrn v
        XOR_SHIFT_GLOBAL_SEED
    }
}


#[inline(always)]
pub fn gen_xorshift_random_float() -> f64 {
    let mut seed = unsafe{_rdtsc()};
    for _ in 0..2 {
        seed ^= seed << 17;
        seed ^= seed >> 7;
        seed ^= seed << 13;
    }
    let v: u64 = (seed >> 11) | (1023 << 52);
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
