

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

