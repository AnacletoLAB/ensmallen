use super::*;
use indicatif::{ProgressBar, ProgressStyle};
use rayon::ThreadPool;

use num_traits::pow::Pow;

mod method_caller;
pub use method_caller::*;

mod parallel_lines_reader;
pub use parallel_lines_reader::ParallelLines;

mod parallel_lines_reader_with_index;
pub use parallel_lines_reader_with_index::*;

mod iters_wrapper;
pub use iters_wrapper::ItersWrapper;

mod argmax_argmin;
pub use argmax_argmin::*;

mod minmax;
pub use minmax::*;

mod splitter;
pub use splitter::*;

mod clonable_unsafe_cell;
pub(crate) use clonable_unsafe_cell::*;

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

pub fn get_loading_bar(verbose: bool, desc: &str, total_iterations: usize) -> ProgressBar {
    if verbose {
        let pb = ProgressBar::new(total_iterations as u64);
        let candidate_iterations = total_iterations as u64 / 1000;
        let candidate_iterations = candidate_iterations.max(1);
        pb.set_draw_delta(candidate_iterations);
        pb.set_style(ProgressStyle::default_bar().template(&format!(
            "{desc} {{spinner:.green}} [{{elapsed_precise}}] [{{bar:40.cyan/blue}}] ({{pos}}/{{len}}, ETA {{eta}})",
            desc=desc
        )));
        pb
    } else {
        ProgressBar::hidden()
    }
}

/// Returns a rayon thread pool handling Creation errors.
///
/// Getting a thread pool might return the error "Resource temporarly unavailable"
/// if the number of processes currently on the system is more than what set in
/// `ulimit -a`, which by default is 256851.
///
/// Moreover, we return an error if the number of selected CPUS is 1 or less.
/// Because the algorithms which use the pool requires at least 2 threads, and
/// we generally provide also an optimized single-thread version.
pub(crate) fn get_thread_pool() -> Result<(usize, ThreadPool)> {
    let cpu_number = rayon::current_num_threads();

    if cpu_number <= 1 {
        return Err(concat!(
            "Cannot execute the parallel connected_components method when",
            " only a single CPU is made available.\n",
            "This might be an erroroneus configuration of the envionment",
            " variable RAYON_NUM_THREADS.\n",
            "If you really want to compute the connected components with",
            " these configurations, consider using random_spanning_arborescence_kruskal."
        )
        .to_string());
    }

    let mut attempts_left = 1_000_000;
    loop {
        match rayon::ThreadPoolBuilder::new()
            .num_threads(cpu_number)
            .build()
        {
            Ok(thread_pool) => return Ok((cpu_number, thread_pool)),
            Err(internal_error) => {
                if attempts_left == 0 {
                    return Err(format!(
                        concat!(
                            "Unknown error while trying to allocate the thread pool for ",
                            "executing the parallel connected components algorithm.\n",
                            "In our experience this happens once in every 100 milions calls\n",
                            "The interal error is {:?}."
                        ),
                        internal_error
                    ));
                }
                let delay = std::time::Duration::from_millis(50);
                std::thread::sleep(delay);
                attempts_left -= 1;
            }
        }
    }
}

/// Validated the provided features.
///
/// Specifically, the features must:
/// - Be provided for all of the expected elements.
/// - Be non-empty.
/// - Be of a consistent size, that is the number of features for each element must be equal.
///
/// # Arguments
/// * `features`: Vec<Vec<f64>> - The features to validate.
/// * `expected_elements_number`: usize - The number of expected elements.
pub(crate) fn validate_features(
    features: &[Vec<f64>],
    expected_elements_number: usize,
) -> Result<()> {
    if features.len() != expected_elements_number {
        return Err(format!(
            concat!(
                "The expected features vector length was expected to be {}, ",
                "but is {}."
            ),
            expected_elements_number,
            features.len()
        ));
    }
    let expected_node_features_length = features.first().unwrap().len();
    if expected_node_features_length == 0 {
        return Err("The node features length must be greater than zero.".to_string());
    }
    for node_features in features.iter() {
        if expected_node_features_length != node_features.len() {
            return Err(format!(
                concat!(
                    "The node features length needs to be consistent: the expected ",
                    "size was {} while the found length was {}."
                ),
                expected_node_features_length,
                node_features.len()
            ));
        }
    }
    Ok(())
}

/// Return true if the given weight is near to one.
pub(crate) fn not_one(weight: WeightT) -> bool {
    (weight - 1.0).abs() > WeightT::EPSILON
}

impl Graph {
    /// Return vector of edges to be inserted in the holdout.
    pub(crate) fn compute_edge_ids_vector(
        &self,
        edge_id: EdgeT,
        src: NodeT,
        dst: NodeT,
        include_all_edge_types: bool,
    ) -> Vec<EdgeT> {
        if include_all_edge_types {
            let (min_edge_id, max_edge_id) =
                unsafe { self.get_unchecked_minmax_edge_ids_from_node_ids(src, dst) };
            (min_edge_id..max_edge_id).collect::<Vec<EdgeT>>()
        } else {
            vec![edge_id]
        }
    }
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
/// assert!(parse_weight("0.0").is_ok());
/// assert!(parse_weight("-1.0").is_ok());
/// assert!(parse_weight("2.0").is_ok());
/// assert!(parse_weight("2ghgjh.0").is_err());
/// assert_eq!(parse_weight("2.0").unwrap(), 2.0);
/// ```
///
pub fn parse_weight(weight: &str) -> Result<WeightT> {
    weight
        .parse::<WeightT>()
        .map_err(|_| format!("Cannot parse weight `{}` as a float.", weight))
}

/// Convert a strig to integer ASSUMING IT IS CORRECT
pub unsafe fn atoi_c(val: &str) -> u32 {
    let mut result: u32 = 0;
    for b in val.as_bytes() {
        result = result * 10 + (*b - b'0') as u32;
    }
    result
}

/// Return given number converted to a human readable value.
///
/// # Arguments
/// `number`: usize - The value to convert into a human readable string.
pub(crate) fn to_human_readable_high_integer(number: usize) -> String {
    let (exponent, unit) = match number {
        0..1_000 => return number.to_string(),
        1_000..1_000_000 => (1, "K"),
        1_000_000..1_000_000_000 => (2, "M"),
        1_000_000_000..1_000_000_000_000 => (3, "G"),
        _ => (4, "T"),
    };
    format!(
        "{amount:.2}{unit}",
        amount = number as f64 / (1000.0 as f64).pow(exponent),
        unit = unit
    )
}

/// Returns given list in a uman readable format.
///
/// # Safety
/// If the list is empty the method will raise a panic.
///
/// # Arguments
/// `elements`: &[String] - The elements to format.
/// `max_number_of_elements`: Option<usize> - Maximum number of elements to display.
pub unsafe fn get_unchecked_formatted_list(
    elements: &[String],
    max_number_of_elements: Option<usize>,
) -> String {
    let max_number_of_elements = max_number_of_elements.unwrap_or(elements.len());
    if elements.is_empty() {
        panic!("Cannot format a list with no elements.");
    }
    if elements.len() == 1 {
        return elements.first().unwrap().clone();
    }
    let all_minus_last: String = elements[0..elements.len() - 1]
        .iter()
        .cloned()
        .take(max_number_of_elements)
        .collect::<Vec<String>>()
        .join(", ");
    format!(
        "{all_minus_last} and {last}",
        all_minus_last = all_minus_last,
        last = if elements.len() <= max_number_of_elements {
            elements.last().unwrap().clone()
        } else {
            let remaining_values = elements.len() - max_number_of_elements;
            if remaining_values == 1 {
                "another one".to_string()
            } else {
                format!("other {}", to_human_readable_high_integer(remaining_values))
            }
        }
    )
}

pub trait ToAtomicVec<T> {
    fn to_atomic(self: Self) -> Vec<T>;
}
pub trait RemoveAtomicVec<T> {
    fn remove_atomic(self: Self) -> Vec<T>;
}

#[macro_export]
/// Create a vector of atomic using a default value.
/// the syntax is:
/// `vec_atomic[AtomicTYPE; DEFAULT_VALUE; SIZE]`
macro_rules! impl_to_atomic_vec {
    ($atomic_type:ty, $normal_type:ty) => {
        impl ToAtomicVec<$atomic_type> for Vec<$normal_type> {
            fn to_atomic(self) -> Vec<$atomic_type> {
                unsafe { std::mem::transmute::<Vec<$normal_type>, Vec<$atomic_type>>(self) }
            }
        }

        impl RemoveAtomicVec<$normal_type> for Vec<$atomic_type> {
            fn remove_atomic(self) -> Vec<$normal_type> {
                unsafe { std::mem::transmute::<Vec<$atomic_type>, Vec<$normal_type>>(self) }
            }
        }
    };
}

impl_to_atomic_vec!(std::sync::atomic::AtomicU8, u8);
impl_to_atomic_vec!(std::sync::atomic::AtomicU16, u16);
impl_to_atomic_vec!(std::sync::atomic::AtomicU32, u32);
impl_to_atomic_vec!(std::sync::atomic::AtomicU64, u64);
impl_to_atomic_vec!(std::sync::atomic::AtomicUsize, usize);
