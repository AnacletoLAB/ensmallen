

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"),
target_feature = "sse"))]
use core::arch::x86_64::{
// info can be found at https://software.intel.com/sites/landingpage/IntrinsicsGuide
__m128,
// sum two vector of f32
_mm_add_ps,
// cast __m128i  to __m128
// it's only for compilation, it does not gen instructions
_mm_castsi128_ps,
// cast __m128 to __m128i
// see _mm_castsi128_ps
_mm_castps_si128,
// shift vector left and insert zeros
_mm_slli_si128,
// set vec to zero
_mm_setzero_ps,
// Memory -> Vec (MUST be 16-bytes aligned)
_mm_load_ps,
// Memory -> Vec but slower
_mm_loadu_ps,
// Vec -> Memory (MUST be 16-bytes aligned)
_mm_store_ps,
// Vec -> Memory but slower
_mm_storeu_ps,
// Shiffle the vecotr according to the mask given
_mm_shuffle_ps
};

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"),
target_feature = "sse"))]
#[inline(always)]
fn scan_sse(mut x: __m128) -> __m128{
// its "equivalent" to
// x += x << (4 * 8);
// x += x << (8 * 8);
//
// first pass:
//      f4,      f3,      f2, f1 +
//      f3,      f2,      f1,  0 =
//     f43,     f32,     f21, f1
// 
// second pass
// f43, f32, f21, f1 +
// f21,  f1,   0,  0 =
// f4321, f321, f21, f1
//
// -> Fast cumulative sum using 2 adds and 2 shifts instead of (3 + 2 + 1) = 6 adds
unsafe{
  x = _mm_add_ps(x, _mm_castsi128_ps(_mm_slli_si128(_mm_castps_si128(x), 4)));
  x = _mm_add_ps(x, _mm_castsi128_ps(_mm_slli_si128(_mm_castps_si128(x), 8)));
}
x
}

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"),
target_feature = "sse"))]
pub fn sse_128_f32_cumulative_sum(random_vec: &Vec<f32>) -> Vec<f32> {
let mut result = vec![0.0f32; random_vec.len()];
unsafe{
  let mut offset: __m128 = _mm_setzero_ps();
  for i in (0..random_vec.len()).step_by(4) {
      // it should be __mm_load_ps but if the values are not aligned it
      // raises a seg-fault so we use the slower _mm_loadu_ps until we figure
      // out how to ensure the alignmenet of the vector
      // loat the 4 values
      let x: __m128 = _mm_loadu_ps(random_vec.as_ptr().wrapping_offset(i as isize));
      // compute the local cumulative sum
      let mut out: __m128 = scan_sse(x);
      // add the local cumulative sum to the current offset
      out = _mm_add_ps(out, offset);
      // get the internal floats array of the result vec
      let ptr: *mut f32 = result.as_mut_ptr();
      // store the value in the vector
      _mm_storeu_ps(ptr.offset(i as isize), out);
      // the mask should be  _MM_SHUFFLE(3, 3, 3, 3)
      // but it's unstable in rust so we manually embed it
      // as 511 because we want to broadcast the last value
      // to all 4 lanes, and the value is choosen with the first 8 bits
      // and since we want 3, 3, 3, 3, it's 8 bits set to 1 so
      // 2**9 - 1 = 511
      // Update the current offset (aka the last value of out)
      offset = _mm_shuffle_ps(out, out, 511); 

  }
}
result
}

