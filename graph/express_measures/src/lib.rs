#![allow(internal_features)]
#![feature(core_intrinsics)]
mod cosine_similarity;
mod element_wise_operations;
mod dot;
mod matrix_vector_dot;
mod euclidean_distance;
mod metrics;
mod types;
mod validation;
mod dynamic_time_warping;

pub use cosine_similarity::*;
pub use dot::*;
pub use element_wise_operations::*;
pub use matrix_vector_dot::*;
pub use euclidean_distance::*;
pub use metrics::*;
pub use types::*;
pub use dynamic_time_warping::*;
