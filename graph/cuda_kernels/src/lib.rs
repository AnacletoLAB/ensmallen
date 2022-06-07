#![feature(abi_ptx, core_intrinsics)]
#![no_std]
#![feature(asm_experimental_arch)]

mod intrinsics;
mod cbow;
mod skipgram;
use intrinsics::*;
pub use cbow::*;
pub use skipgram::*;