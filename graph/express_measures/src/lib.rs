#![feature(core_intrinsics)]
mod cosine_similarity;
mod dot;
mod euclidean_distance;
mod types;
mod validation;
mod metrics;

pub use cosine_similarity::*;
pub use metrics::*;
pub use dot::*;
pub use euclidean_distance::*;
pub use types::*;
