#![feature(portable_simd)]
mod cbow;
mod skipgram;

pub use cbow::*;
pub use skipgram::*;