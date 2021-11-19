use super::*;
use core::sync::atomic::AtomicUsize;
use core::sync::atomic::Ordering;
use num_traits::Zero;
use rayon::iter::IntoParallelRefIterator;
use rayon::iter::ParallelIterator;

#[no_binding]
/// Return threshold representing cutuoff point in given exponential distribution to have the given amount of elements above cutoff.
///
/// # Implementative details
/// Note that if the number of required elements is higher than the number of elements in the array,
/// the threshold returned will be equal to zero.
///
/// # Arguments
/// * `scores`: &[T] - The scores to be used to compute the threshold.
/// * `number_of_elements_above_threshold`: usize - Number of elements expected to be above cutoff threshold.
pub fn get_exponential_distribution_threshold<T: Into<f64> + Send + Sync + Clone>(
    scores: &[T],
    number_of_elements_above_threshold: usize,
) -> f64 {
    // If the number of requested elements is higher than the number of available elements
    // the threshold to cutoff that numbeer of elements is surely zero.
    if number_of_elements_above_threshold >= scores.len() {
        return 0.0;
    }
    // We compute the mean of the provided scores
    let mean_score = scores
        .par_iter()
        .cloned()
        .map(|score| score.into())
        .sum::<f64>()
        / scores.len() as f64;
    // And then, using the exponential distribution formula,
    // we compute the cutoff threshold.
    mean_score * (-(number_of_elements_above_threshold as f64 / scores.len() as f64).ln())
}

#[no_binding]
/// Return threshold representing cutuoff point in given geometric distribution to have the given amount of elements above cutoff.
///
/// # Implementative details
/// Note that if the number of required elements is higher than the number of elements in the array,
/// the threshold returned will be equal to zero.
///
/// # Arguments
/// * `scores`: &[T] - The scores to be used to compute the threshold.
/// * `number_of_elements_above_threshold`: usize - Number of elements expected to be above cutoff threshold.
pub fn get_geometric_distribution_threshold<T: Into<f64> + Send + Sync + Clone>(
    scores: &[T],
    number_of_elements_above_threshold: usize,
) -> f64 {
    // If the number of requested elements is higher than the number of available elements
    // the threshold to cutoff that numbeer of elements is surely zero.
    if number_of_elements_above_threshold >= scores.len() {
        return 0.0;
    }
    // We compute the mean of the provided scores
    let (total_score, has_zeros) = scores
        .par_iter()
        .cloned()
        .map(|score| {
            let f64_score: f64 = score.into();
            (f64_score, f64_score.is_zero())
        })
        .reduce(
            || (0.0, false),
            |(score_a, a_is_zero), (score_b, b_is_zero)| (score_a + score_b, a_is_zero | b_is_zero),
        );
    let mean_score = total_score / scores.len() as f64;
    // And then, using the geometric distribution formula,
    // we compute the cutoff threshold.
    let numerator = (number_of_elements_above_threshold as f64 / scores.len() as f64).ln();
    if has_zeros {
        numerator / (1.0 - 1.0 / (mean_score + 1.0)) - 1.0
    } else {
        numerator / (1.0 - 1.0 / mean_score)
    }
}

#[no_binding]
/// Return threshold representing cutuoff point in given unknown distribution to have the given amount of elements above cutoff.
///
/// # Implementative details
/// Note that if the number of required elements is higher than the number of elements in the array,
/// the threshold returned will be equal to zero.
///
/// # Arguments
/// * `scores`: &[T] - The scores to be used to compute the threshold.
/// * `number_of_elements_above_threshold`: usize - Number of elements expected to be above cutoff threshold.
/// * `number_of_bins`: usize - Number of bins to use to try to identify the best cutoff.
pub fn get_unknown_distribution_threshold<
    T: Into<f32> + Sync + Clone + PartialOrd + Copy + Sized + Send,
>(
    scores: &[T],
    number_of_elements_above_threshold: usize,
    number_of_bins: usize,
) -> f32 {
    // If the number of requested elements is higher than the number of available elements
    // the threshold to cutoff that numbeer of elements is surely zero.
    if number_of_elements_above_threshold >= scores.len() {
        return 0.0;
    }
    // We compute the minimum and maximum value of the provided scores
    let (min_value, max_value) = scores.par_iter().cloned().minmax().unwrap();
    if min_value == max_value {
        return 0.0;
    }
    // Compute the hashing coefficient
    let min_value: f32 = min_value.into();
    let delta: f32 = max_value.into() - min_value;
    let hashing_coefficient: f32 = number_of_bins as f32 / delta;
    // Create the vector of Atomics
    let counters = (0..number_of_bins)
        .map(|_| AtomicUsize::new(0))
        .collect::<Vec<AtomicUsize>>();
    // Populate the counters
    scores
        .par_iter()
        .cloned()
        .map(|score| score.into() - min_value)
        .for_each(|score| {
            let index: usize = number_of_bins - 1 - (score * hashing_coefficient).ceil() as usize;
            counters[index].fetch_add(1, Ordering::Relaxed);
        });
    // Find the first counter that curresponding to the maximum threshold
    let mut optimal_index = 0;
    let mut comulative_sum = 0;
    for (i, counter) in counters
        .into_iter()
        .map(|counter| counter.into_inner())
        .enumerate()
    {
        comulative_sum += counter;
        if comulative_sum >= number_of_elements_above_threshold {
            optimal_index = i;
            break;
        }
    }
    // Compute and return the threshold
    min_value + (number_of_bins - optimal_index) as f32 / number_of_bins as f32 * delta
}

/// Implementation of methods relative to statistical tools.
impl Graph {
    /// Return threshold representing cutuoff point in graph node degree geometric distribution to have the given amount of elements above cutoff.
    ///
    /// # Implementative details
    /// Note that if the number of required elements is higher than the number of elements in the array,
    /// the threshold returned will be equal to zero.
    ///
    /// # Arguments
    /// * `number_of_elements_above_threshold`: usize - Number of elements expected to be above cutoff threshold.
    pub fn get_node_degree_geometric_distribution_threshold(
        &self,
        number_of_nodes_above_threshold: NodeT,
    ) -> f64 {
        // If the number of requested elements is higher than the number of available elements
        // the threshold to cutoff that numbeer of elements is surely zero.
        if number_of_nodes_above_threshold >= self.get_nodes_number() {
            return 0.0;
        }
        // We compute the mean of the node degrees
        // We can surely unwrap because if the number of nodes were to be zero
        // the check above would handle that.
        let mean_node_degree = self.get_node_degrees_mean().unwrap();
        // Check if the graph contains zero degree nodes
        let has_zero_degree_nodes = self.has_singleton_nodes() | self.has_trap_nodes();
        // And then, using the geometric distribution formula,
        // we compute the cutoff threshold.
        let numerator =
            (number_of_nodes_above_threshold as f64 / self.get_nodes_number() as f64).ln();
        if has_zero_degree_nodes {
            numerator / (1.0 - 1.0 / (mean_node_degree + 1.0)) - 1.0
        } else {
            numerator / (1.0 - 1.0 / mean_node_degree)
        }
    }
}
