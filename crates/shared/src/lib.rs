#![feature(core_intrinsics)]

pub mod method_caller;
pub use method_caller::*;

pub mod parallel_lines_reader;
pub use parallel_lines_reader::ParallelLines;

pub mod parallel_lines_reader_with_index;
pub use parallel_lines_reader_with_index::*;

pub mod iters_wrapper;
pub use iters_wrapper::ItersWrapper;

pub mod argmax_argmin;
pub use argmax_argmin::*;

pub mod thread_data_race_aware;
pub use thread_data_race_aware::*;

pub mod clonable_unsafe_cell;
pub use clonable_unsafe_cell::*;

pub mod to_from_usize;
pub use to_from_usize::*;

pub mod get_loading_bar;
pub use get_loading_bar::*;

pub mod get_thread_pool;
pub use get_thread_pool::*;

pub mod validate_features;
pub use validate_features::*;

pub mod types;
pub use types::*;

#[macro_export]
/// Take a vector and make it a None if its empty, Some(vector) otherwise
macro_rules! optionify {
    ($val:expr) => {
        if $val.is_empty() {
            None
        } else {
            Some($val)
        }
    };
}

#[macro_export]
/// Create a vector of atomic using a default value.
/// the syntax is:
/// `vec_atomic[AtomicTYPE; DEFAULT_VALUE; SIZE]`
macro_rules! vec_atomic {
    [$atomic_type:ty ; $default_value:expr ; $size:expr] => {{
        (0..$size)
            .map(|_| <$atomic_type>::new($default_value))
            .collect()
    }}
}

/// Return true if the given weight is near to one.
pub fn not_one(weight: WeightT) -> bool {
    (weight - 1.0).abs() > WeightT::EPSILON
}

/// Return given weight parsed from string to float.
///
/// # Arguments
///
/// * `weight`: String - The weight to be parsed.
///
/// # Example
/// The weight can be validated as follows:
/// ```rust
/// # use graph::utils::parse_weight;
/// assert!(parse_weight("0.0".to_string()).is_ok());
/// assert!(parse_weight("-1.0".to_string()).is_ok());
/// assert!(parse_weight("2.0".to_string()).is_ok());
/// assert!(parse_weight("2ghgjh.0".to_string()).is_err());
/// assert_eq!(parse_weight("2.0".to_string()).unwrap(), 2.0);
/// ```
///
pub fn parse_weight(weight: String) -> Result<WeightT> {
    weight
        .parse::<WeightT>()
        .map_err(|_| format!("Cannot parse weight {} as a float.", weight))
}
