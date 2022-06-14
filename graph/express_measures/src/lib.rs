#![feature(core_intrinsics)]
mod cosine_similarity;
mod dot;
mod euclidean_distance;
mod metrics;
mod types;
mod validation;
mod dynamic_time_warping;

pub use cosine_similarity::*;
pub use dot::*;
pub use euclidean_distance::*;
pub use metrics::*;
pub use types::*;
pub use dynamic_time_warping::*;
