use crate::types::*;
use crate::validation::*;
use core::fmt::Debug;
use core::ops::Mul;
use rayon::prelude::*;
use std::iter::Sum;
use core::intrinsics::unlikely;

#[inline(always)]
/// Returns the dot product between the two provided vectors computed sequentially.
///
/// # Arguments
/// * `src_features`: &[F] - The first feature.
/// * `dst_features`: &[F] - The second feature.
///
/// # Safety
/// If the two features have different sizes, we will compute
/// the dot product upwards to when the minimum size.
/// No warning will be raised.
pub unsafe fn dot_product_sequential_unchecked<F: Copy + Sum + Mul<Output = F>>(
    src_features: &[F],
    dst_features: &[F],
) -> F {
    src_features
        .iter()
        .zip(dst_features.iter())
        .map(|(&src_feature, &dst_feature)| src_feature * dst_feature)
        .sum()
}

#[inline(always)]
/// Returns the dot product between the two provided vectors computed in parallel.
///
/// # Arguments
/// * `src_features`: &[F] - The first feature.
/// * `dst_features`: &[F] - The second feature.
///
/// # Safety
/// If the two features have different sizes, we will compute
/// the dot product upwards to when the minimum size.
/// No warning will be raised.
pub unsafe fn dot_product_parallel_unchecked<F: Copy + Sum + Mul<Output = F> + Send + Sync>(
    src_features: &[F],
    dst_features: &[F],
) -> F {
    src_features
        .par_iter()
        .zip(dst_features.par_iter())
        .map(|(&src_feature, &dst_feature)| src_feature * dst_feature)
        .sum()
}

#[inline(always)]
/// Returns the dot product between the two provided vectors computed sequentially.
///
/// # Arguments
/// * `src_features`: &[F] - The first feature.
/// * `dst_features`: &[F] - The second feature.
///
/// # Raises
/// * If one of the two vectors are empty.
/// * If the two vectors have different sizes.
pub fn dot_product_sequential<F: ThreadFloat>(
    src_features: &[F],
    dst_features: &[F],
) -> Result<F, String> {
    validate_features(src_features, dst_features)?;
    Ok(unsafe { dot_product_sequential_unchecked(src_features, dst_features) })
}

#[inline(always)]
/// Returns the dot product between the two provided vectors computed in parallel.
///
/// # Arguments
/// * `src_features`: &[F] - The first feature.
/// * `dst_features`: &[F] - The second feature.
///
/// # Raises
/// * If one of the two vectors are empty.
/// * If the two vectors have different sizes.
pub fn dot_product_parallel<F: ThreadFloat>(
    src_features: &[F],
    dst_features: &[F],
) -> Result<F, String> {
    validate_features(src_features, dst_features)?;
    Ok(unsafe { dot_product_parallel_unchecked(src_features, dst_features) })
}

#[inline(always)]
/// Write the dot product in the provided slice.
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
pub unsafe fn dot_product_from_indices_unchecked<F: ThreadFloat, I: ThreadUnsigned>(
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

            if unlikely(src == dst) {
                *similarity = F::one();
            }

            *similarity = dot_product_sequential_unchecked(
                &matrix[src * dimension..(src + 1) * dimension],
                &matrix[dst * dimension..(dst + 1) * dimension],
            );
        });
    Ok(())
}
