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
    // https://experilous.com/1/blog/post/perfect-fast-random-floating-point-numbers
    // this exploits type punning to build a float with constant exponent
    // and use only the "high-quality" bits from xorshiro for the mantissa
    // the informations about the structure of a f64 was taken from IEEE 754
    // https://standards.ieee.org/content/ieee-standards/en/standard/754-2019.html
    // https://en.wikipedia.org/wiki/Double-precision_floating-point_format
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
    
    // convert the u64 to f64
    // the first 12 bits of the representation of f64 are the sign and the exponent
    // therefore we fill them with zero and get rid of the low quality lower bits
    // then we set 1023 as the exponent of the f64 so that it's a number between 1 and 2
    // (this offset is due to how the floating value is computed: 
    //      -1^(sign) * (1 + mantissa) * 2 ** (exponent - 1023) 
    //
    let v: u64 = (result >> 12) | (1023 << 52);
    // use type punning to make rust happy, this should result in 0 assembly instructions
    // since it should have 0 effects on the memory
    let r: f64 = f64::from_le_bytes(v.to_le_bytes());
    // subtract 1 from the generate number
    r - 1f64
    }
}

pub fn sample(weights: &Vec<f64>) -> usize {
    let rnd: f64 = xorshiro256plus();

    // this should be faster than binary search.
    // of course the binary search take O(ln(n))
    // but to build the cumulative sum it tale O(n)
    // so we can just do a linear scan which is at most O(n)
    // And, the average complexity should be the median of the vector.
    // Of-course this could not hold for parallel / AVX implementations
    // but this function is called in a loop that already satruate all the
    // cores so the benefits from parallelization sould be marginal.
    let mut acc: f64 = 0f64;
    let mut i: usize = 0;
    for w in weights {
        acc += w;
        if acc > rnd{
            return i;
        }
        i += 1;
    }
    i
}