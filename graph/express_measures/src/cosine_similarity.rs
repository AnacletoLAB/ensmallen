use core::fmt::Debug;
use core::intrinsics::unlikely;
use num_traits::{Float, Unsigned};
use rayon::prelude::*;

pub trait ThreadFloat: Float + Send + Sync + Copy {}
pub trait ThreadUnsigned: Unsigned + Send + Sync + Copy + TryInto<usize> + Debug {}

impl<T> ThreadFloat for T where T: Float + Send + Sync + Copy {}
impl<T> ThreadUnsigned for T where T: Unsigned + Send + Sync + Copy + TryInto<usize> + Debug {}

/// Returns the cosine similarity between the two provided vectors computed sequentially.
///
/// # Arguments
/// * `src_features`: &[F] - The first feature.
/// * `dst_features`: &[F] - The second feature.
///
/// # Safety
/// If the two features have different sizes, we will compute
/// the cosine distnce upwards to when the minimum size.
/// No warning will be raised.
pub unsafe fn cosine_similarity_sequential_unchecked<F: ThreadFloat>(
    src_features: &[F],
    dst_features: &[F],
) -> F {
    let (total_dot_products, total_squared_src_features, total_squared_dst_features) = src_features
        .iter()
        .zip(dst_features.iter())
        .map(|(&src_feature, &dst_feature)| {
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

    total_dot_products / (src_features_norm * dst_features_norm + F::epsilon())
}

/// Returns the cosine similarity between the two provided vectors computed in parallel.
///
/// # Arguments
/// * `src_features`: &[F] - The first feature.
/// * `dst_features`: &[F] - The second feature.
///
/// # Safety
/// If the two features have different sizes, we will compute
/// the cosine distnce upwards to when the minimum size.
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

/// Validate whether the two provided features are compatible to compute a cosine similarity.
///
/// # Arguments
/// * `src_features`: &[F] - The first feature.
/// * `dst_features`: &[F] - The second feature.
///
/// # Raises
/// * If one of the two vectors are empty.
/// * If the two vectors have different sizes.
fn validate_cosine_similarity_features<F: ThreadFloat>(
    src_features: &[F],
    dst_features: &[F],
) -> Result<(), String> {
    if src_features.is_empty() {
        return Err(concat!("The provided source features are an empty slice. ").to_string());
    }
    if dst_features.is_empty() {
        return Err(concat!("The provided destination features are an empty slice. ").to_string());
    }
    if src_features.len() != dst_features.len() {
        return Err(format!(
            concat!(
                "The provided source features have size `{}` ",
                "but the provided destination features have size `{}`. ",
                "It is not possible to compute cosine distances between ",
                "vectors of different sizes."
            ),
            src_features.len(),
            dst_features.len()
        ));
    }
    Ok(())
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
pub fn cosine_similarity_sequential<F: ThreadFloat>(
    src_features: &[F],
    dst_features: &[F],
) -> Result<F, String> {
    validate_cosine_similarity_features(src_features, dst_features)?;
    Ok(unsafe { cosine_similarity_sequential_unchecked(src_features, dst_features) })
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
    validate_cosine_similarity_features(src_features, dst_features)?;
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
pub unsafe fn cosine_similarity_from_indices_unchecked<F: ThreadFloat, I: ThreadUnsigned>(
    similarities: &mut [F],
    matrix: &[F],
    sources: &[I],
    destinations: &[I],
    dimension: usize,
) -> Result<(), String>
where
    <I as TryInto<usize>>::Error: Debug,
{
    if matrix.is_empty() {
        return Err("The provided matrix is empty!".to_string());
    }
    if sources.is_empty() {
        return Err("The provided sources vector is empty!".to_string());
    }
    if destinations.is_empty() {
        return Err("The provided destinations vector is empty!".to_string());
    }
    if sources.len() != destinations.len() {
        return Err(format!(
            concat!(
                "The provided sources vector has length {}, while ",
                "the provided destinations vector has length {}. ",
                "The two vectors should have the same size."
            ),
            sources.len(),
            destinations.len()
        ));
    }
    if sources.len() != similarities.len() {
        return Err(format!(
            concat!(
                "The provided sources vector has length {}, while ",
                "the provided similarities vector has length {}. ",
                "The two vectors should have the same size."
            ),
            sources.len(),
            similarities.len()
        ));
    }
    if matrix.len() % dimension != 0 {
        return Err(format!(
            concat!(
                "The provided matrix has a size {}, while ",
                "the provided dimension is {}. ",
                "The matrix size should be exactly divisible ",
                "by the provided dimension."
            ),
            matrix.len(),
            dimension
        ));
    }
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

            *similarity = cosine_similarity_sequential_unchecked(
                &matrix[src * dimension..(src + 1) * dimension],
                &matrix[dst * dimension..(dst + 1) * dimension],
            );
        });
    Ok(())
}
