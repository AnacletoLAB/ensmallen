//! Fast functions to generare pesudo-random numbers and extract samples from a given distribution.
use super::types::*;
use ::core::cmp::Ordering;

/// Global static seed, this could be moved inside a struct
/// WARNING
/// the current implementation is not thread safe because we
/// mutate a shared state between threads without any locks.
/// This should not create any problem since we do not need
/// a strong PRNG so for speed sake it's intentionally let
/// this way.
/// The only real problem could be that we lose determinism
pub static mut GLOBAL_SEED: [u64; 4] = [
    6591408588322595484,
    5451729388608518856,
    8913376598984957243,
    17912695770704705270,
];

#[inline(always)]
fn rotl(x: u64, k: u64) -> u64 {
    (x << k) | (x >> (64 - k))
}


#[inline(always)]
/// Return a random u64.
/// 
/// # Implementation details
/// The implementations is based on xorshiro256+ which seems to be the fastest floating point generator.
/// The reference implementation can be found [here](http://prng.di.unimi.it/xoshiro256plus.c).
/// Xorshiro256+ generate a  random u64 so we need to convert it to f64.
/// 
/// One important detail about xorshiro256+ is that it has low entropy in the lower 3 bits.
/// 
/// One possible optimization might be to generate several random values in parallel exploiting
/// AVX / SSE instructions and then use these values. An implemnetation could be found [here](http://prng.di.unimi.it/xoshiro256+-vect-speed.c)
/// 
/// # Examples
/// ```
/// use graph::random::random_u64;
/// 
/// let rnd: u64 = random_u64();
/// println!("The random value is: {}", rnd);
/// ```
pub fn random_u64() -> u64 {
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
/// Return a random f64 between 0 and 1.
/// 
/// # Implementation details
/// We generate a pseudo-random number using xorshiro256+ and then we convert it to a float.`
/// 
/// One important detail about xorshiro256+ is that it has low entropy in the lower bits.
/// This is not a problem since we generate 64bits but we will only need 53. 
/// 
/// There are two main methods to convert from u64 to f64 and they can be found [here](http://prng.di.unimi.it/)
/// 
/// 
/// Basically we are setupping the exponent and mantissa of the float and then punning the value to a float
/// 
/// The "simplest" is to multiply the value for the right exponent:alloc
/// ```
/// # use graph::random::random_u64;
/// let result: f64 = (random_u64() >> 11) as f64 * 2.0f64.powf(-53.0);
/// ```
/// 
/// There is also a second way that exploit type punning:
/// 
/// ```
/// # use graph::random::random_u64;
/// let v: u64 = (random_u64() >> 11) | (1023 << 52);
/// let r: f64 = unsafe{f64::from_le_bytes(v.to_le_bytes())};
/// let result: f64 = r - 1f64;
/// ```
/// the informations about the structure of a f64 was taken from [IEEE 754](https://standards.ieee.org/content/ieee-standards/en/standard/754-2019.html)
/// 
/// First we shift the value in order to fit the high-entropy values in the mantissa of the float.
/// 
/// Then we se the bits from 1 to 12 to 1023 so that we set the exponent to 1.
/// (Since the computed exponent is e - 1022 where e is the value we set)
///
/// Then we convert this u64 to a random f64 from 1 to 2.
/// 
/// The type punning is made with:
/// ```
/// # let v: u64 = 100;
/// let r: f64 = unsafe{f64::from_le_bytes(v.to_le_bytes())};
/// ```
/// The C equivalent is:
/// ```C
/// double r = *((double *)&v);
/// ```
/// 
/// The last step is to fix the range form 1 - 2, to 0 - 1.
/// 
/// As Vigna [says](http://prng.di.unimi.it/), these two methods should have equivalent performances on modern hardware.
/// But in our benchmarks we found the second (and more complicated) one to be slightly faster.
/// 
/// # Examples
/// ```
/// use graph::random::random_f64;
/// 
/// let frnd: f64 = random_f64();
/// assert!(0.0 <= frnd && frnd <= 1.0);
/// println!("The random value is: {}", frnd);
/// ```
pub fn random_f64() -> f64 {
    let v: u64 = (random_u64() >> 11) | (1023 << 52);
    let r: f64 = f64::from_le_bytes(v.to_le_bytes());
    r - 1f64
}




pub fn unrolled_cumulative_f64_sum(random_vec: &Vec<WeightT>) -> Vec<f64> {
    let mut result = vec![0.0f64; random_vec.len()];
    let mut offset = 0.0f64;

    for i in (0..random_vec.len()).step_by(4){
        let mut a = random_vec[i];
        let mut b = random_vec[i+1];
        let mut c = random_vec[i+2];
        let mut d = random_vec[i+3];

        d += c + b + a + offset;
        c += b + a + offset;
        b += a + offset;
        a += offset;

        result[i] = a;
        result[i+1] = b;
        result[i+2] = c;
        result[i+3] = d;

        offset = d;
    }

    for _ in 0..(random_vec.len() % 4){
        result.pop();
    }

    result
}

pub fn naife_cumulative_f64_sum(random_vec: &Vec<f64>) -> Vec<f64> {
        let mut cumulative_sum: Vec<f64> = Vec::with_capacity(random_vec.len());
        let mut total_weight = 0f64;
        for w in random_vec {
            total_weight += w;
            cumulative_sum.push(total_weight.clone());
        }
        cumulative_sum
}



#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"),
target_feature = "sse"))]
use core::arch::x86_64::{
    // info can be found at https://software.intel.com/sites/landingpage/IntrinsicsGuide
    __m128d,
    // sum two vector of f64
    _mm_add_pd,
    // cast __m128di  to __m128d
    // it's only for compilation, it does not gen instructions
    _mm_castsi128_pd,
    // cast __m128d to __m128di
    // see _mm_castsi128_ps
    _mm_castpd_si128,
    // shift vector left and insert zeros
    _mm_slli_si128,
    // set vec to zero
    _mm_setzero_pd,
    // Memory -> Vec (MUST be 16-bytes aligned)
    _mm_load_pd,
    // Memory -> Vec but slower
    _mm_loadu_pd,
    // Vec -> Memory (MUST be 16-bytes aligned)
    _mm_store_pd,
    // Vec -> Memory but slower
    _mm_storeu_pd,
    // Shiffle the vecotr according to the mask given
    _mm_shuffle_pd
};

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"),
target_feature = "sse"))]
#[inline(always)]
fn scan_sse(mut x: __m128d) -> __m128d{
    //
    // pass:
    //      f2, f1 +
    //      f1,  0 =
    //      f21, f1
    // 
    // -> Meh, 1 add + 1 shift instead of 1 add, not great
    unsafe{
        x = _mm_add_pd(x, _mm_castsi128_pd(_mm_slli_si128(_mm_castpd_si128(x), 8)));
    }
    x
}

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"),
target_feature = "sse"))]
pub fn sse_128_f64_cumulative_sum(random_vec: &Vec<f64>) -> Vec<f64> {
let mut result = vec![0.0f64; random_vec.len()];
    unsafe{
        let mut offset: __m128d = _mm_setzero_pd();
        for i in (0..random_vec.len()).step_by(2) {
            // it should be __mm_load_ps but if the values are not aligned it
            // raises a seg-fault so we use the slower _mm_loadu_ps until we figure
            // out how to ensure the alignmenet of the vector
            // loat the 4 values
            let x: __m128d = _mm_loadu_pd(random_vec.as_ptr().wrapping_offset(i as isize));
            // compute the local cumulative sum
            let mut out: __m128d = scan_sse(x);
            // add the local cumulative sum to the current offset
            out = _mm_add_pd(out, offset);
            // get the internal floats array of the result vec
            let ptr: *mut f64= result.as_mut_ptr();
            // store the value in the vector
            _mm_storeu_pd(ptr.offset(i as isize), out);
            // Update the current offset (aka the last value of out)
            offset = _mm_shuffle_pd(out, out, 3); 

        }
    }
    result
}





/// Given a vector of scores (non-zero positive values), convert it to a 
/// probability distribution and extract a random indices accodringly.`
///
/// # Implementation details
/// The implemented method is O(n) because the first operations is to calculate
/// the cumulative sum of the weights, then we extract a random floating value
/// between 0 and the last value of the cumulative sum. 
/// Finally, we find the index of the first value bigger than it by binary search.
/// 
/// Further optimization could be about using SSE4 / AVX2 / AVX512 instructions
/// to calculate the cumulative sum in parallel as explained in this blog.
/// https://github.com/joelangeway/CumulativeSum
/// But this could be improved using the HADDPS instruction as specified in
/// https://www.felixcloutier.com/x86/haddps
/// Or in the (Volume 2A of intel's architecture)[https://software.intel.com/content/www/us/en/develop/download/intel-64-and-ia-32-architectures-sdm-combined-volumes-2a-2b-2c-and-2d-instruction-set-reference-a-z.html]
/// at page Vol. 2A 3-449.
pub fn sample(weights:  & mut Vec<WeightT>) -> usize {
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
    

    let rnd: f64 = random_f64() * cumulative_sum[cumulative_sum.len() - 1];

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
