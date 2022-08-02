use crate::absolute_distance;
use crate::types::*;
use crate::validation::*;
use core::fmt::Debug;
use std::iter::Sum;
use num_traits::Float;
use rayon::prelude::*;
use std::ops::Mul;
use std::ops::Sub;

/// Returns the squared euclidean distance between the two provided vectors computed sequentially.
///
/// # Arguments
/// * `src_features`: &[F] - The first feature.
/// * `dst_features`: &[F] - The second feature.
///
/// # Safety
/// If the two features have different sizes, we will compute
/// the squared euclidean distance upwards to when the minimum size.
/// No warning will be raised.
pub unsafe fn squared_euclidean_distance_sequential_unchecked<
    F: Mul<Output = F> + Sub<Output = F> + PartialOrd + Sum + Copy,
>(
    src_features: &[F],
    dst_features: &[F],
) -> F {
    src_features
        .iter()
        .zip(dst_features.iter())
        .map(|(&src_feature, &dst_feature)| {
            absolute_distance(src_feature * src_feature, dst_feature * dst_feature)
        })
        .sum()
}

/// Returns the euclidean distance between the two provided vectors computed sequentially.
///
/// # Arguments
/// * `src_features`: &[F] - The first feature.
/// * `dst_features`: &[F] - The second feature.
///
/// # Safety
/// If the two features have different sizes, we will compute
/// the euclidean distance upwards to when the minimum size.
/// No warning will be raised.
pub unsafe fn euclidean_distance_sequential_unchecked<R: Float, F: Coerced<R>>(
    src_features: &[F],
    dst_features: &[F],
) -> R {
    squared_euclidean_distance_sequential_unchecked(src_features, dst_features)
        .coerce_into()
        .sqrt()
}

/// Returns the squared euclidean distance between the two provided vectors computed in parallel.
///
/// # Arguments
/// * `src_features`: &[F] - The first feature.
/// * `dst_features`: &[F] - The second feature.
///
/// # Safety
/// If the two features have different sizes, we will compute
/// the squared euclidean distance upwards to when the minimum size.
/// No warning will be raised.
pub unsafe fn squared_euclidean_distance_parallel_unchecked<
    F: Mul<Output = F> + Sub<Output = F> + Send + Sync + PartialOrd + Sum + Copy,
>(
    src_features: &[F],
    dst_features: &[F],
) -> F {
    src_features
        .par_iter()
        .zip(dst_features.par_iter())
        .map(|(&src_feature, &dst_feature)| {
            absolute_distance(src_feature * src_feature, dst_feature * dst_feature)
        })
        .sum()
}

/// Returns the euclidean distance between the two provided vectors computed in parallel.
///
/// # Arguments
/// * `src_features`: &[F] - The first feature.
/// * `dst_features`: &[F] - The second feature.
///
/// # Safety
/// If the two features have different sizes, we will compute
/// the euclidean distance upwards to when the minimum size.
/// No warning will be raised.
pub unsafe fn euclidean_distance_parallel_unchecked<R: Float, F: Coerced<R>>(
    src_features: &[F],
    dst_features: &[F],
) -> R {
    squared_euclidean_distance_parallel_unchecked(src_features, dst_features).coerce_into().sqrt()
}

/// Returns the euclidean distance between the two provided vectors computed sequentially.
///
/// # Arguments
/// * `src_features`: &[F] - The first feature.
/// * `dst_features`: &[F] - The second feature.
///
/// # Raises
/// * If one of the two vectors are empty.
/// * If the two vectors have different sizes.
pub fn euclidean_distance_sequential<R: Float, F: Coerced<R>>(
    src_features: &[F],
    dst_features: &[F],
) -> Result<R, String> {
    validate_features(src_features, dst_features)?;
    Ok(unsafe { euclidean_distance_sequential_unchecked(src_features, dst_features) })
}

/// Returns the euclidean distance between the two provided vectors computed in parallel.
///
/// # Arguments
/// * `src_features`: &[F] - The first feature.
/// * `dst_features`: &[F] - The second feature.
///
/// # Raises
/// * If one of the two vectors are empty.
/// * If the two vectors have different sizes.
pub fn euclidean_distance_parallel<R: Float, F: Coerced<R>>(
    src_features: &[F],
    dst_features: &[F],
) -> Result<R, String> {
    validate_features(src_features, dst_features)?;
    Ok(unsafe { euclidean_distance_parallel_unchecked(src_features, dst_features) })
}

/// Write the squared euclidean distance in the provided slice.
///
/// # Arguments
/// * `similarities`: &mut [F] - Vector where to store the computed similarities.
/// * `matrix`: &[F] - Matrix containing the feaures.
/// * `sources`: &[I] - Indices of the source features.
/// * `destinations`: &[I] - Indices of the destination features.
/// * `dimension`: usize - Dimensionality of the matrix.
///
/// # Raises
/// * If the matrix is not compatible with the provided dimensions.
/// * If the provided similarities are not of the same size as the destination or sources.
/// * If the provided dimension is zero.
///
/// # Safety
/// If the source and destination indices have values higher
/// than the provided matrix, the method will panic.
pub unsafe fn squared_euclidean_distance_from_indices_unchecked<
    R: Float,
    F: Coerced<R>,
    I: ThreadUnsigned,
>(
    similarities: &mut [F],
    matrix: &[F],
    sources: &[I],
    destinations: &[I],
    dimension: usize,
) -> Result<(), String>
where
    <I as TryInto<usize>>::Error: Debug,
{
    validate_features_from_indices(similarities, matrix, sources, destinations, dimension)?;
    similarities
        .par_iter_mut()
        .zip(
            sources
                .par_iter()
                .copied()
                .zip(destinations.par_iter().copied()),
        )
        .for_each(|(similarity, (src, dst))| {
            let src: usize = src.try_into().unwrap();
            let dst: usize = dst.try_into().unwrap();

            *similarity = squared_euclidean_distance_sequential_unchecked(
                &matrix[src * dimension..(src + 1) * dimension],
                &matrix[dst * dimension..(dst + 1) * dimension],
            );
        });
    Ok(())
}

/// Write the euclidean distance in the provided slice.
///
/// # Arguments
/// * `distances`: &mut [F] - Vector where to store the computed distances.
/// * `matrix`: &[F] - Matrix containing the feaures.
/// * `sources`: &[I] - Indices of the source features.
/// * `destinations`: &[I] - Indices of the destination features.
/// * `dimension`: usize - Dimensionality of the matrix.
///
/// # Raises
/// * If the matrix is not compatible with the provided dimensions.
/// * If the provided distances are not of the same size as the destination or sources.
/// * If the provided dimension is zero.
///
/// # Safety
/// If the source and destination indices have values higher
/// than the provided matrix, the method will panic.
pub unsafe fn euclidean_distance_from_indices_unchecked<
    R: Float + Send + Sync,
    F: Coerced<R>,
    I: ThreadUnsigned,
>(
    distances: &mut [R],
    matrix: &[F],
    sources: &[I],
    destinations: &[I],
    dimension: usize,
) -> Result<(), String>
where
    <I as TryInto<usize>>::Error: Debug,
{
    validate_features_from_indices(distances, matrix, sources, destinations, dimension)?;
    distances
        .par_iter_mut()
        .zip(
            sources
                .par_iter()
                .copied()
                .zip(destinations.par_iter().copied()),
        )
        .for_each(|(distance, (src, dst))| {
            let src: usize = src.try_into().unwrap();
            let dst: usize = dst.try_into().unwrap();

            *distance = euclidean_distance_sequential_unchecked(
                &matrix[src * dimension..(src + 1) * dimension],
                &matrix[dst * dimension..(dst + 1) * dimension],
            );
        });
    Ok(())
}
