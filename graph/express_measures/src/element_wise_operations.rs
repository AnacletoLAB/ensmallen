use std::{
    iter::Sum,
    ops::{AddAssign, DivAssign},
};

use crate::types::*;
use num_traits::{AsPrimitive, Float};

/// Returns vector with the element-wise subtraction between the two vectors.
///
/// # Arguments
/// * `first_vector`: &[F] - The first vector of the subtraction.
/// * `second_vector`: &[F] - The second vector of the subtraction.
///
/// # Safety
/// If the two features have different sizes, we will compute
/// the subtraction upwards to when the minimum size.
/// No warning will be raised.
pub unsafe fn element_wise_subtraction<F: Into<R> + Copy, R: Float>(
    first_vector: &[F],
    second_vector: &[F],
) -> Vec<R> {
    first_vector
        .iter()
        .zip(second_vector.iter())
        .map(|(&first_feature, &second_feature)| first_feature.into() - second_feature.into())
        .collect()
}

/// Returns vector with the element-wise weighted subtraction between the two vectors.
///
/// # Arguments
/// * `first_vector`: &[F] - The first vector of the subtraction.
/// * `second_vector`: &[F] - The second vector of the subtraction.
/// * `weight`: F - The weight to apply to the second vector.
///
/// # Safety
/// If the two features have different sizes, we will compute
/// the subtraction upwards to when the minimum size.
/// No warning will be raised.
pub unsafe fn element_wise_weighted_subtraction<F: Into<R> + Copy, R: ThreadFloat>(
    first_vector: &[F],
    second_vector: &[F],
    weight: F,
) -> Vec<R> {
    first_vector
        .iter()
        .zip(second_vector.iter())
        .map(|(&first_feature, &second_feature)| {
            first_feature.into() - second_feature.into() * weight.into()
        })
        .collect()
}

/// Executes element-wise subtraction inplace.
///
/// # Arguments
/// * `first_vector`: &mut [F] - The first vector of the subtraction, where to store the subtraction.
/// * `second_vector`: &[F] - The second vector of the subtraction.
///
/// # Safety
/// If the two features have different sizes, we will compute
/// the subtraction upwards to when the minimum size.
/// No warning will be raised.
pub unsafe fn element_wise_subtraction_inplace<F: Into<R> + Copy, R: ThreadFloat>(
    first_vector: &mut [R],
    second_vector: &[F],
) {
    first_vector
        .iter_mut()
        .zip(second_vector.iter())
        .map(|(first_feature, &second_feature)| *first_feature -= second_feature.into())
        .collect()
}

/// Executes element-wise weighted subtraction inplace.
///
/// # Arguments
/// * `first_vector`: &mut [F] - The first vector of the subtraction, where to store the subtraction.
/// * `second_vector`: &[F] - The second vector of the subtraction.
/// * `weight`: F - The weight to apply to the second vector.
///
/// # Safety
/// If the two features have different sizes, we will compute
/// the subtraction upwards to when the minimum size.
/// No warning will be raised.
pub unsafe fn element_wise_weighted_subtraction_inplace<F: Into<R> + Copy, R: ThreadFloat>(
    first_vector: &mut [R],
    second_vector: &[F],
    weight: F,
) {
    first_vector
        .iter_mut()
        .zip(second_vector.iter())
        .map(|(first_feature, &second_feature)| {
            *first_feature -= second_feature.into() * weight.into()
        })
        .collect()
}

/// Returns vector with the element-wise addition between the two vectors.
///
/// # Arguments
/// * `first_vector`: &[F] - The first vector of the addition.
/// * `second_vector`: &[F] - The second vector of the addition.
///
/// # Safety
/// If the two features have different sizes, we will compute
/// the addition upwards to when the minimum size.
/// No warning will be raised.
pub unsafe fn element_wise_addition<F: Into<R> + Copy, R: ThreadFloat>(
    first_vector: &[F],
    second_vector: &[F],
) -> Vec<R> {
    first_vector
        .iter()
        .zip(second_vector.iter())
        .map(|(&first_feature, &second_feature)| first_feature.into() + second_feature.into())
        .collect()
}

/// Returns vector with the element-wise weighted addition between the two vectors.
///
/// # Arguments
/// * `first_vector`: &[F] - The first vector of the addition.
/// * `second_vector`: &[F] - The second vector of the addition.
/// * `weight`: F - The weight to apply to the second vector.
///
/// # Safety
/// If the two features have different sizes, we will compute
/// the addition upwards to when the minimum size.
/// No warning will be raised.
pub unsafe fn element_wise_weighted_addition<F: Into<R> + Copy, R: ThreadFloat>(
    first_vector: &[F],
    second_vector: &[F],
    weight: F,
) -> Vec<R> {
    first_vector
        .iter()
        .zip(second_vector.iter())
        .map(|(&first_feature, &second_feature)| {
            first_feature.into() + second_feature.into() * weight.into()
        })
        .collect()
}

/// Executes element-wise addition inplace.
///
/// # Arguments
/// * `first_vector`: &mut [F] - The first vector of the addition, where to store the addition.
/// * `second_vector`: &[F] - The second vector of the addition.
///
/// # Safety
/// If the two features have different sizes, we will compute
/// the addition upwards to when the minimum size.
/// No warning will be raised.
pub unsafe fn element_wise_addition_inplace<F: AddAssign<F> + Copy>(
    first_vector: &mut [F],
    second_vector: &[F],
) {
    first_vector
        .iter_mut()
        .zip(second_vector.iter())
        .for_each(|(first_feature, &second_feature)| *first_feature += second_feature);
}

/// Executes element-wise weighted addition inplace.
///
/// # Arguments
/// * `first_vector`: &mut [F] - The first vector of the addition, where to store the addition.
/// * `second_vector`: &[F] - The second vector of the addition.
/// * `weight`: F - The weight to apply to the second vector.
///
/// # Safety
/// If the two features have different sizes, we will compute
/// the addition upwards to when the minimum size.
/// No warning will be raised.
pub unsafe fn element_wise_weighted_addition_inplace<F: Into<R> + Copy, R: ThreadFloat>(
    first_vector: &mut [R],
    second_vector: &[F],
    weight: R,
) {
    first_vector
        .iter_mut()
        .zip(second_vector.iter())
        .map(|(first_feature, &second_feature)| *first_feature += second_feature.into() * weight)
        .collect()
}

/// Returns the squared norm of the provided vector.
///
/// # Arguments
/// * `vector`: &mut [F] - The vector to compute the squared norm for.
pub fn squared_vector_norm<
    F: Copy + AsPrimitive<R>,
    R: Sum + Float + 'static,
>(
    vector: &[F],
) -> R {
    (vector
        .iter()
        .copied()
        .map(|value| value.as_().powf(R::one() + R::one()))
        .sum::<R>()
        + R::epsilon())
    .min(R::max_value())
}

/// Returns the norm of the provided vector.
///
/// # Arguments
/// * `vector`: &mut [F] - The vector to compute the norm for.
pub fn vector_norm<F: Copy + AsPrimitive<R>, R: Sum + Float + 'static>(
    vector: &[F],
) -> R {
    squared_vector_norm(vector).sqrt()
}

/// Normalize inplace the provided vector.
///
/// # Arguments
/// * `vector`: &mut [F] - The vector to normalize in place.
pub fn normalize_vector_inplace<F: Into<F> + AsPrimitive<F> + Copy + Float + Sum + DivAssign>(
    vector: &mut [F],
) -> F {
    let norm: F = vector_norm(vector);
    vector.iter_mut().for_each(|value| {
        *value /= norm;
    });
    norm
}
