#![feature(abi_ptx, core_intrinsics)]
#![no_std]
#![feature(asm_experimental_arch)]

mod intrinsics;
use intrinsics::*;

#[no_mangle]
/// Actual function called by the CPU code in the GPU
pub unsafe extern "ptx-kernel" fn add_one(
    input: *mut u32,
    input_len: usize,
) {
    let idx = ((block_idx_x() * 1024) + thread_idx_x()) as isize;
    if idx < input_len {
        *input.offset(idx) += 1;
    }
}