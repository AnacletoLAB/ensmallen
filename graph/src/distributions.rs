use super::*;
use num_traits::Zero;
use rayon::iter::IntoParallelRefIterator;
use rayon::iter::ParallelIterator;

/// Implementation of methods relative to statistical tools.
impl Graph {
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
        &self,
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
        &self,
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
                |(score_a, a_is_zero), (score_b, b_is_zero)| {
                    (score_a + score_b, a_is_zero | b_is_zero)
                },
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
