#![feature(abi_ptx, core_intrinsics)]
#![no_std]
#![feature(asm_experimental_arch)]

mod intrinsics;
mod cbow;
use intrinsics::*;
pub use cbow::*;

#[no_mangle]
/// Actual function called by the CPU code in the GPU
pub unsafe extern "ptx-kernel" fn add_one(
    input: *mut f32,
    input_len: usize,
) {
    let idx = ((block_idx_x() * block_dim_x()) + thread_idx_x()) as usize;
    if idx < input_len {
        let v = &mut (*input.add(idx));
        *v = v.log2();
    }
}
