#![feature(abi_ptx, core_intrinsics)]
#![no_std]
#![feature(asm_experimental_arch)]

mod intrinsics;
mod first_order_line;
use intrinsics::*;
pub use first_order_line::*;