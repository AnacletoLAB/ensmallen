static mut s: [u64; 4] = [0xdeadbeefc0febabe, 0xdeadbeefc0febabe, 0xdeadbeefc0febabe, 0xdeadbeefc0febabe];

#[inline(always)]
fn rotl(x : u64, k: u64) -> u64{
	return (x << k) | (x >> (64 - k));
}

#[inline(always)]
pub fn xorshiro256plus() -> f64{
    /// based on
    /// https://experilous.com/1/blog/post/perfect-fast-random-floating-point-numbers
    /// http://prng.di.unimi.it/xoshiro256plus.c
    unsafe {
	let (result, _): (u64, bool) = s[0].overflowing_add(s[3]);

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

pub fn sample(weights: &Vec<f64>) -> usize {
    let rnd: f64 = xorshiro256plus();

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