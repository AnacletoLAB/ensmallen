#![feature(abi_ptx, core_intrinsics)]
#![no_std]
#![feature(asm_experimental_arch)]

mod intrinsics;
use intrinsics::*;

#[no_mangle]
/// Actual function called by the CPU code in the GPU
pub unsafe extern "ptx-kernel" fn test_kernel(
    _input: *mut u32,
    _input_len: usize,
    output: *mut u32,
    _output_len: usize,
) {
    let idx = (block_idx_x() * 1024 + thread_idx_x()) as isize;

    let start = clock();
    let end = clock();

    *output.offset(idx) = end - start;
}