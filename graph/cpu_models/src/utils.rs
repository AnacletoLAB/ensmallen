use num::Zero;
use rayon::prelude::*;
use vec_rand::{random_f32, splitmix64};

pub(crate) fn must_not_be_zero<F>(
    value: Option<F>,
    default: F,
    variable_name: &str,
) -> Result<F, String>
where
    F: Zero,
{
    let value = value.unwrap_or(default);
    if value.is_zero() {
        return Err(format!(
            concat!(
                "The provided {variable_name} is zero. ",
                "The {variable_name} should be strictly greater than zero."
            ),
            variable_name = variable_name
        ));
    }
    Ok(value)
}

// Initialize the model with weights and bias in the range (-1 / sqrt(k), +1 / sqrt(k))
fn get_random_weight(random_state: u64, scale_factor: f32) -> f32 {
    (2.0 * random_f32(splitmix64(random_state)) - 1.0) * 6.0 / scale_factor
}

pub(crate) fn populate_vectors(vectors: &mut [&mut [f32]], random_state: u64, scale_factor: f32) {
    vectors.iter_mut().for_each(|vector| {
        vector.par_iter_mut().enumerate().for_each(|(i, weight)| {
            *weight = get_random_weight(random_state + i as u64, scale_factor);
        })
    });
}

pub(crate) fn get_random_vector(capacity: usize, random_state: u64, scale_factor: f32) -> Vec<f32> {
    (0..capacity)
        .map(|i| get_random_weight(random_state + i as u64, scale_factor))
        .collect()
}

pub(crate) fn norm(vector: &[f32]) -> f32 {
    (vector
        .iter()
        .map(|value| value.powf(2.0))
        .sum::<f32>()
        .sqrt()
        + f32::EPSILON)
        .min(f32::MAX)
}

pub(crate) fn compute_prior(subset_size: f32, total_size: f32) -> f32 {
    (1.0 + subset_size)
            / total_size
            // Adding the epsilon is necessary because the division may destroy enough
            // resolution to make the prior equal to zero.
            + f32::EPSILON
}