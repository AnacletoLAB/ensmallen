#![feature(asm)]

use core::arch::x86_64::{
    // info can be found at https://software.intel.com/sites/landingpage/IntrinsicsGuide
    __m256i,
    // << 
    _mm256_slli_epi64,
    // >>
    _mm256_srli_epi64,
    // ^ 
    _mm256_xor_si256,
    // mem -> reg
    _mm256_loadu_si256,
    // reg -> mem
    _mm256_storeu_si256

};

#[inline(always)]
pub fn avx2_xorshift() -> [u64; 4] {
    // else the load and store will fail, if this doesn't work we must use loadu and storeu
    let mut result: [u64; 4] = [1337, 0xdeadbeef, 0xc0febabe, 0xbad5eed];
    unsafe{
        let mut temp: __m256i;
        let mut seed: __m256i = _mm256_loadu_si256(result.as_mut_ptr() as * mut __m256i);
        temp = _mm256_slli_epi64(seed, 13);
        seed = _mm256_xor_si256(seed, temp);
        temp = _mm256_srli_epi64(seed, 7);
        seed = _mm256_xor_si256(seed, temp);
        temp = _mm256_slli_epi64(seed, 17);
        seed = _mm256_xor_si256(seed, temp);
        _mm256_storeu_si256(result.as_mut_ptr() as * mut __m256i, seed);
    }
    result
}

pub fn ax2_xorshift_assembly(seed: & mut [u64; 4]) {
    unsafe {
        asm!(
        concat!(
            // Load the data
            "vmovdqu ymm0, ymmword ptr [rdi]\n",
            // << 13
            "vpsllq ymm1, ymm0, 13\n",
            // ^
            "vpxor ymm0, ymm0, ymm1\n",
            // >> 7
            "vpsrlq ymm1, ymm0, 7\n",
            // ^
            "vpxor ymm0, ymm0, ymm1\n",
            // << 17
            "vpsllq ymm1, ymm0, 17\n",
            // ^c
            "vpxor ymm0, ymm0, ymm1\n",
            // Store the data
            "vmovdqu ymmword ptr [rdi], ymm0"
        ),
        inout("rdi") seed => _,
        );
    }
}

// FASTEST
pub fn ax2_ss4_xorshift_assembly(seed: & mut [u64; 16]) {
    unsafe {
        asm!(
        concat!(
            // Load the data
            "vmovdqu ymm0, ymmword ptr [rdi]\n",
            "vmovdqu ymm2, ymmword ptr [rdi + 32]\n",
            "vmovdqu ymm4, ymmword ptr [rdi + 64]\n",
            "vmovdqu ymm6, ymmword ptr [rdi + 96]\n",
            // tmp = seed << 13
            "vpsllq ymm1, ymm0, 13\n",
            "vpsllq ymm3, ymm2, 13\n",
            "vpsllq ymm5, ymm4, 13\n",
            "vpsllq ymm7, ymm6, 13\n",
            // seed ^= tmp
            "vpxor ymm0, ymm0, ymm1\n",
            "vpxor ymm2, ymm2, ymm3\n",
            "vpxor ymm4, ymm4, ymm5\n",
            "vpxor ymm6, ymm6, ymm7\n",
            // tmp = seed >> 7
            "vpsrlq ymm1, ymm0, 7\n",
            "vpsrlq ymm3, ymm2, 7\n",
            "vpsrlq ymm5, ymm4, 7\n",
            "vpsrlq ymm7, ymm6, 7\n",
            // seed ^= tmp
            "vpxor ymm0, ymm0, ymm1\n",
            "vpxor ymm2, ymm2, ymm3\n",
            "vpxor ymm4, ymm4, ymm5\n",
            "vpxor ymm6, ymm6, ymm7\n",
            // tmp = seed << 17
            "vpsllq ymm1, ymm0, 17\n",
            "vpsllq ymm3, ymm2, 17\n",
            "vpsllq ymm5, ymm4, 17\n",
            "vpsllq ymm7, ymm6, 17\n",
            // seed ^= tmp
            "vpxor ymm0, ymm0, ymm1\n",
            "vpxor ymm2, ymm2, ymm3\n",
            "vpxor ymm4, ymm4, ymm5\n",
            "vpxor ymm6, ymm6, ymm7\n",
            // Store the data
            "vmovdqu ymmword ptr [rdi], ymm0\n",
            "vmovdqu ymmword ptr [rdi + 32], ymm2\n",
            "vmovdqu ymmword ptr [rdi + 64], ymm4\n",
            "vmovdqu ymmword ptr [rdi + 96], ymm6\n"
        ),
        inout("rdi") seed => _,
        );
    }
}


pub fn ax2_ss8_xorshift_assembly(seed: & mut [u64; 32]) {
    unsafe {
        asm!(
        concat!(
            // Load the data
            "vmovdqu ymm0, ymmword ptr [rdi]\n",
            "vmovdqu ymm2, ymmword ptr [rdi + 32]\n",
            "vmovdqu ymm4, ymmword ptr [rdi + 64]\n",
            "vmovdqu ymm6, ymmword ptr [rdi + 96]\n",
            "vmovdqu ymm8, ymmword ptr [rdi + 128]\n",
            "vmovdqu ymm10, ymmword ptr [rdi + 160]\n",
            "vmovdqu ymm12, ymmword ptr [rdi + 192]\n",
            "vmovdqu ymm14, ymmword ptr [rdi + 224]\n",
            // << 13
            "vpsllq ymm1, ymm0, 13\n",
            "vpsllq ymm3, ymm2, 13\n",
            "vpsllq ymm5, ymm4, 13\n",
            "vpsllq ymm7, ymm6, 13\n",
            "vpsllq ymm9, ymm8, 13\n",
            "vpsllq ymm11, ymm10, 13\n",
            "vpsllq ymm13, ymm12, 13\n",
            "vpsllq ymm15, ymm14, 13\n",
            // ^
            "vpxor ymm0, ymm0, ymm1\n",
            "vpxor ymm2, ymm2, ymm3\n",
            "vpxor ymm4, ymm4, ymm5\n",
            "vpxor ymm6, ymm6, ymm7\n",
            "vpxor ymm8, ymm9, ymm1\n",
            "vpxor ymm10, ymm11, ymm3\n",
            "vpxor ymm12, ymm13, ymm5\n",
            "vpxor ymm14, ymm15, ymm7\n",
            // >> 7
            "vpsrlq ymm1, ymm0, 7\n",
            "vpsrlq ymm3, ymm2, 7\n",
            "vpsrlq ymm5, ymm4, 7\n",
            "vpsrlq ymm7, ymm6, 7\n",
            "vpsrlq ymm9, ymm8, 7\n",
            "vpsrlq ymm11, ymm10, 7\n",
            "vpsrlq ymm13, ymm12, 7\n",
            "vpsrlq ymm15, ymm14, 7\n",
            // ^
            "vpxor ymm0, ymm0, ymm1\n",
            "vpxor ymm2, ymm2, ymm3\n",
            "vpxor ymm4, ymm4, ymm5\n",
            "vpxor ymm6, ymm6, ymm7\n",
            "vpxor ymm8, ymm9, ymm1\n",
            "vpxor ymm10, ymm11, ymm3\n",
            "vpxor ymm12, ymm13, ymm5\n",
            "vpxor ymm14, ymm15, ymm7\n",
            // << 17
            "vpsllq ymm1, ymm0, 17\n",
            "vpsllq ymm3, ymm2, 17\n",
            "vpsllq ymm5, ymm4, 17\n",
            "vpsllq ymm7, ymm6, 17\n",
            "vpsllq ymm9, ymm8, 17\n",
            "vpsllq ymm11, ymm10, 17\n",
            "vpsllq ymm13, ymm12, 17\n",
            "vpsllq ymm15, ymm14, 17\n",
            // ^
            "vpxor ymm0, ymm0, ymm1\n",
            "vpxor ymm2, ymm2, ymm3\n",
            "vpxor ymm4, ymm4, ymm5\n",
            "vpxor ymm6, ymm6, ymm7\n",
            "vpxor ymm8, ymm9, ymm1\n",
            "vpxor ymm10, ymm11, ymm3\n",
            "vpxor ymm12, ymm13, ymm5\n",
            "vpxor ymm14, ymm15, ymm7\n",
            // Store the data
            "vmovdqu ymmword ptr [rdi], ymm0\n",
            "vmovdqu ymmword ptr [rdi + 32], ymm2\n",
            "vmovdqu ymmword ptr [rdi + 64], ymm4\n",
            "vmovdqu ymmword ptr [rdi + 96], ymm6\n",
            "vmovdqu ymmword ptr [rdi + 128], ymm8\n",
            "vmovdqu ymmword ptr [rdi + 160], ymm10\n",
            "vmovdqu ymmword ptr [rdi + 192], ymm12\n",
            "vmovdqu ymmword ptr [rdi + 224], ymm14\n"
        ),
        inout("rdi") seed => _,
        );
    }
}