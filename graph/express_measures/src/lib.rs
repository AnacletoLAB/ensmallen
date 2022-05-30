#![feature(core_intrinsics)]
mod cosine_similarity;
mod dot;
mod euclidean_distance;
mod types;
mod validation;
mod accuracy;

pub use cosine_similarity::*;
pub use accuracy::*;
pub use dot::*;
pub use euclidean_distance::*;
pub use types::*;
