use crate::types::*;
use crate::validation::*;
use core::fmt::Debug;

use num_traits::Float;
use rayon::prelude::*;

/// Returns the cosine similarity between the two provided vectors computed sequentially.
///
/// # Arguments
/// * `src_features`: &[F] - The first feature.
/// * `dst_features`: &[F] - The second feature.
///
/// # Safety
/// If the two features have different sizes, we will compute
/// the cosine similarity upwards to when the minimum size.
/// No warning will be raised.
pub unsafe fn cosine_similarity_sequential_unchecked<R: Float, F: Coerced<R>>(
    src_features: &[F],
    dst_features: &[F],
) -> (R, R, R) {
    let (total_dot_products, total_squared_src_features, total_squared_dst_features) = src_features
        .iter()
        .zip(dst_features.iter())
        .map(|(&src_feature, &dst_feature)| (src_feature.coerce_into(), dst_feature.coerce_into()))
        .map(|(src_feature, dst_feature)| {
            (
                src_feature * dst_feature,
                src_feature * src_feature,
                dst_feature * dst_feature,
            )
        })
        .reduce(
            |(total_dot_products, total_squared_src_features, total_squared_dst_features),
             (dot_products, squared_src_features, squared_dst_features)| {
                (
                    total_dot_products + dot_products,
                    total_squared_src_features + squared_src_features,
                    total_squared_dst_features + squared_dst_features,
                )
            },
        )
        .unwrap();

    let src_features_norm = total_squared_src_features.sqrt();
    let dst_features_norm = total_squared_dst_features.sqrt();

    (
        total_dot_products / (src_features_norm * dst_features_norm + R::epsilon()),
        src_features_norm,
        dst_features_norm,
    )
}

/// Returns the cosine similarity between the two provided vectors computed in parallel.
///
/// # Arguments
/// * `src_features`: &[F] - The first feature.
/// * `dst_features`: &[F] - The second feature.
///
/// # Safety
/// If the two features have different sizes, we will compute
/// the cosine similarity upwards to when the minimum size.
/// No warning will be raised.
pub unsafe fn cosine_similarity_parallel_unchecked<F: ThreadFloat>(
    src_features: &[F],
    dst_features: &[F],
) -> F {
    let (total_dot_products, total_squared_src_features, total_squared_dst_features) = src_features
        .par_iter()
        .zip(dst_features.par_iter())
        .map(|(&src_feature, &dst_feature)| {
            (
                src_feature * dst_feature,
                src_feature * src_feature,
                dst_feature * dst_feature,
            )
        })
        .reduce(
            || (F::zero(), F::zero(), F::zero()),
            |(total_dot_products, total_squared_src_features, total_squared_dst_features),
             (dot_products, squared_src_features, squared_dst_features)| {
                (
                    total_dot_products + dot_products,
                    total_squared_src_features + squared_src_features,
                    total_squared_dst_features + squared_dst_features,
                )
            },
        );

    let src_features_norm = total_squared_src_features.sqrt();
    let dst_features_norm = total_squared_dst_features.sqrt();

    total_dot_products / (src_features_norm * dst_features_norm + F::epsilon())
}

/// Returns the cosine similarity between the two provided vectors computed sequentially.
///
/// # Arguments
/// * `src_features`: &[F] - The first feature.
/// * `dst_features`: &[F] - The second feature.
///
/// # Raises
/// * If one of the two vectors are empty.
/// * If the two vectors have different sizes.
pub fn cosine_similarity_sequential<R: Float, F: Coerced<R>>(
    src_features: &[F],
    dst_features: &[F],
) -> Result<R, String> {
    validate_features(src_features, dst_features)?;
    Ok(unsafe { cosine_similarity_sequential_unchecked(src_features, dst_features).0 })
}

/// Returns the cosine similarity between the two provided vectors computed in parallel.
///
/// # Arguments
/// * `src_features`: &[F] - The first feature.
/// * `dst_features`: &[F] - The second feature.
///
/// # Raises
/// * If one of the two vectors are empty.
/// * If the two vectors have different sizes.
pub fn cosine_similarity_parallel<F: ThreadFloat>(
    src_features: &[F],
    dst_features: &[F],
) -> Result<F, String> {
    validate_features(src_features, dst_features)?;
    Ok(unsafe { cosine_similarity_parallel_unchecked(src_features, dst_features) })
}

/// Write the cosine similarity in the provided slice.
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
pub unsafe fn cosine_similarity_from_indices_unchecked<
    R: Float + Send + Sync,
    F: Coerced<R>,
    I: ThreadUnsigned,
>(
    similarities: &mut [R],
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

            *similarity = cosine_similarity_sequential_unchecked(
                &matrix[src * dimension..(src + 1) * dimension],
                &matrix[dst * dimension..(dst + 1) * dimension],
            )
            .0;
        });
    Ok(())
}
