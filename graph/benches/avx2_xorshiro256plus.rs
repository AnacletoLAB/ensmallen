#![feature(asm)]

#[inline(always)]
fn rotl(x : u64, k: u64) -> u64{
	return (x << k) | (x >> (64 - k));
}

static mut GLOBAL_SEED: [u64; 4] = [0xdeadbeefc0febabe, 0xdeadbeefc0febabe, 0xdeadbeefc0febabe, 0xdeadbeefc0febabe];

#[inline(always)]

pub fn xorshiro26plus(s: & mut [u64; 5]){
    s[4] = s[0] + s[3];
    let t: u64 = s[1] << 17;
    s[2] ^= s[0];
    s[3] ^= s[1];
    s[1] ^= s[2];
    s[0] ^= s[3];
    
    s[2] ^= t;

    s[3] = (s[3] << 45) | (s[3] >> 19);
}
// Equivalent assembly with -O3
// mov     rax, qword ptr [rdi + 24]
// mov     rcx, qword ptr [rdi]
// mov     rdx, qword ptr [rdi + 8]
// lea     rsi, [rax + rcx]
// mov     qword ptr [rdi + 32], rsi
// mov     r8, rdx
// shl     r8, 17
// mov     rsi, qword ptr [rdi + 16]
// xor     rsi, rcx
// xor     rax, rdx
// xor     rdx, rsi
// mov     qword ptr [rdi + 8], rdx
// xor     rcx, rax
// mov     qword ptr [rdi], rcx
// xor     rsi, r8
// mov     qword ptr [rdi + 16], rsi
// rol     rax, 45
// mov     qword ptr [rdi + 24], rax
// ret

pub fn ax2_xorshiro_assembly(seed: & mut [u64; 20]) {
    unsafe {
        asm!(
        concat!(
            // rcx = ymm0
            // rdx = ymm1
            // rsi = ymm2
            // rax = ymm3
            // r8  = ymm4
            // Load the data
            "vmovdqu ymm3, ymmword ptr [rdi + 96]\n",   // mov     rax, qword ptr [rdi + 24]
            "vmovdqu ymm0, ymmword ptr [rdi]\n",        // mov     rcx, qword ptr [rdi]
            "vmovdqu ymm1, ymmword ptr [rdi + 32]\n",   // mov     rdx, qword ptr [rdi + 8]
            "vpaddq ymm4, ymm0, ymm3\n",                // lea     rsi, [rax + rcx]
            "vmovdqu ymmword ptr [rdi + 128], ymm4\n",  // mov     qword ptr [rdi + 32], rsi
            "vpsllq ymm4, ymm1, 17\n",                  // mov     r8, rdx; shl     r8, 17
            "vmovdqu ymm2, ymmword ptr [rdi + 64]\n",   // mov     rsi, qword ptr [rdi + 16]
            "vpxor ymm2, ymm2, ymm0\n",                 // xor     rsi, rcx
            "vpxor ymm3, ymm3, ymm1\n",                 // xor     rax, rdx
            "vpxor ymm1, ymm1, ymm2\n",                 // xor     rdx, rsi
            "vmovdqu ymmword ptr [rdi + 32], ymm1\n",   // mov     qword ptr [rdi + 8], rdx
            "vpxor ymm0, ymm0, ymm3\n",                 // xor     rcx, rax 
            "vmovdqu ymmword ptr [rdi], ymm0\n",        // mov     qword ptr [rdi], rcx
            "vpxor ymm2, ymm2, ymm4\n",                 // xor      rsi, r8
            "vmovdqu ymmword ptr [rdi + 64], ymm2\n",   // mov     qword ptr [rdi + 16], rsi
            // s[3] = (s[3] << 45) | (s[3] >> 19)
            // the or is only for avx512 so we use the xor
            // which is equivalent since both vpsllq and vpsrlq insert zeros
            "vpsllq ymm0, ymm1, 45\n",
            "vpsrlq ymm2, ymm1, 19\n",
            "vpxor ymm0, ymm0, ymm2\n",
            // store s[3]
            "vmovdqu ymmword ptr [rdi + 96], ymm0\n",
        ),
        inout("rdi") seed => _,
        );
    }
}

pub fn ax2_ss4_xorshiro_assembly(seed: & mut [u64; 80]) {
    // for info about the scheduling of registers / operation
    //I made https://docs.google.com/spreadsheets/d/1tOgA91OVw9GBKVIXVDeAsQKar3IMXAZXBn3aZwdeDug/edit?usp=sharing
    unsafe {
        asm!(
        concat!(
            // a = s[0]
            "vmovdqu ymm0, ymmword ptr [rdi]\n",
            "vmovdqu ymm1, ymmword ptr [rdi + 32]\n",
            "vmovdqu ymm2, ymmword ptr [rdi + 64]\n",
            "vmovdqu ymm3, ymmword ptr [rdi + 96]\n",
            // b = s[3]
            "vmovdqu ymm4, ymmword ptr [rdi + 128]\n",
            "vmovdqu ymm5, ymmword ptr [rdi + 160]\n",
            "vmovdqu ymm6, ymmword ptr [rdi + 192]\n",
            "vmovdqu ymm7, ymmword ptr [rdi + 224]\n",
            // c = a + b
            "vpaddq ymm8,  ymm0, ymm4\n",
            "vpaddq ymm9,  ymm1, ymm5\n",
            "vpaddq ymm10, ymm2, ymm6\n",
            "vpaddq ymm11, ymm3, ymm7\n",
            // r[4] = c
            "vmovdqu ymmword ptr [rdi + 512], ymm8\n",
            "vmovdqu ymmword ptr [rdi + 544], ymm9\n",
            "vmovdqu ymmword ptr [rdi + 576], ymm10\n",
            "vmovdqu ymmword ptr [rdi + 608], ymm11\n",
            // d = s[2]
            "vmovdqu ymm12, ymmword ptr [rdi + 256]\n",
            "vmovdqu ymm13, ymmword ptr [rdi + 288]\n",
            "vmovdqu ymm14, ymmword ptr [rdi + 320]\n",
            "vmovdqu ymm15, ymmword ptr [rdi + 352]\n",
            // c = a ^ d
            "vpxor ymm8,  ymm0, ymm4\n",
            "vpxor ymm9,  ymm1, ymm5\n",
            "vpxor ymm10, ymm2, ymm6\n",
            "vpxor ymm11, ymm3, ymm7\n",
            // d = s[1]
            "vmovdqu ymm12, ymmword ptr [rdi + 128]\n",
            "vmovdqu ymm13, ymmword ptr [rdi + 160]\n",
            "vmovdqu ymm14, ymmword ptr [rdi + 192]\n",
            "vmovdqu ymm15, ymmword ptr [rdi + 224]\n",
            // b = b ^ d
            "vpxor ymm4, ymm4, ymm12\n",
            "vpxor ymm5, ymm5, ymm13\n",
            "vpxor ymm6, ymm6, ymm14\n",
            "vpxor ymm7, ymm7, ymm15\n",
            // a = a ^ b
            "vpxor ymm0, ymm0, ymm4\n",
            "vpxor ymm1, ymm1, ymm5\n",
            "vpxor ymm2, ymm2, ymm6\n",
            "vpxor ymm3, ymm3, ymm7\n",
            // r[0] = a
            "vmovdqu ymmword ptr [rdi + 0],  ymm0\n",
            "vmovdqu ymmword ptr [rdi + 32], ymm1\n",
            "vmovdqu ymmword ptr [rdi + 64], ymm2\n",
            "vmovdqu ymmword ptr [rdi + 96], ymm3\n",
            // a = c ^ d
            "vpxor ymm0, ymm8,  ymm12\n",
            "vpxor ymm1, ymm9,  ymm13\n",
            "vpxor ymm2, ymm10, ymm14\n",
            "vpxor ymm3, ymm11, ymm15\n",
            // r[1] = a
            "vmovdqu ymmword ptr [rdi + 128], ymm0\n",
            "vmovdqu ymmword ptr [rdi + 160], ymm1\n",
            "vmovdqu ymmword ptr [rdi + 192], ymm2\n",
            "vmovdqu ymmword ptr [rdi + 224], ymm3\n",
            // d = d << 17
            "vpsllq ymm12, ymm12, 17\n",
            "vpsllq ymm13, ymm13, 17\n",
            "vpsllq ymm14, ymm14, 17\n",
            "vpsllq ymm15, ymm15, 17\n",
            // a = c ^ d
            "vpxor ymm0, ymm8,  ymm12\n",
            "vpxor ymm1, ymm9,  ymm13\n",
            "vpxor ymm2, ymm10, ymm14\n",
            "vpxor ymm3, ymm11, ymm15\n",
            // r[2] = a
            "vmovdqu ymmword ptr [rdi + 256], ymm0\n",
            "vmovdqu ymmword ptr [rdi + 288], ymm1\n",
            "vmovdqu ymmword ptr [rdi + 320], ymm2\n",
            "vmovdqu ymmword ptr [rdi + 352], ymm3\n",
            // a = b << 45
            "vpsllq ymm0, ymm4, 45\n",
            "vpsllq ymm1, ymm5, 45\n",
            "vpsllq ymm2, ymm6, 45\n",
            "vpsllq ymm3, ymm7, 45\n",
            // c = b >> 19
            "vpsrlq ymm8,  ymm4, 19\n",
            "vpsrlq ymm9,  ymm5, 19\n",
            "vpsrlq ymm10, ymm6, 19\n",
            "vpsrlq ymm11, ymm7, 19\n",
            // d = a ^ c
            "vpxor ymm12, ymm0, ymm8\n",
            "vpxor ymm13, ymm1, ymm9\n",
            "vpxor ymm14, ymm2, ymm10\n",
            "vpxor ymm15, ymm3, ymm11\n",
            // r[3] = d
            "vmovdqu ymmword ptr [rdi + 384], ymm12\n",
            "vmovdqu ymmword ptr [rdi + 416], ymm13\n",
            "vmovdqu ymmword ptr [rdi + 448], ymm14\n",
            "vmovdqu ymmword ptr [rdi + 480], ymm15\n"
        ),
        inout("rdi") seed => _,
        );
    }
}
