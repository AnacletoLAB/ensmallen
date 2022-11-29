use crate::types::*;
use crate::validation::*;
use core::fmt::Debug;
use num_traits::{AsPrimitive, Float};
use indicatif::{ParallelProgressIterator, ProgressBar, ProgressStyle};
use parallel_frontier::Frontier;
use rayon::prelude::*;

/// Returns the cosine similarity between the two provided vectors computed sequentially.
///
/// # Arguments
/// * `src_features`: Iterator<Item=F> - The first feature.
/// * `dst_features`: Iterator<Item=F> - The second feature.
///
/// # Safety
/// If the two features have different sizes, we will compute
/// the cosine similarity upwards to when the minimum size.
/// No warning will be raised.
pub unsafe fn cosine_similarity_sequential_unchecked_from_iter<
    R: Float + 'static,
    F: AsPrimitive<R> + Copy,
    I1,
    I2,
>(
    src_features_iter: I1,
    dst_features_iter: I2,
) -> (R, R, R)
where
    I1: Iterator<Item = F>,
    I2: Iterator<Item = F>,
{
    let (total_dot_products, total_squared_src_features, total_squared_dst_features) =
        src_features_iter
            .zip(dst_features_iter)
            .map(|(src_feature, dst_feature)| (src_feature.as_(), dst_feature.as_()))
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
pub unsafe fn cosine_similarity_sequential_unchecked<
    R: Float + 'static,
    F: AsPrimitive<R> + Copy,
>(
    src_features: &[F],
    dst_features: &[F],
) -> (R, R, R) {
    cosine_similarity_sequential_unchecked_from_iter(
        src_features.iter().copied(),
        dst_features.iter().copied(),
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
pub fn cosine_similarity_sequential<R: Float + 'static, F: AsPrimitive<R> + Copy>(
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
    R: Float + Send + Sync + 'static,
    F: AsPrimitive<R> + Send + Sync + Copy,
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

/// Compute the cosine similarities between the two provided element lists.
///
/// # Arguments
/// * `matrix`: &[F] - Matrix containing the feaures.
/// * `sources`: &[I] - Indices of the source features.
/// * `destinations`: &[I] - Indices of the destination features.
/// * `dimension`: usize - Dimensionality of the matrix.
/// * `lower_threshold`: Option<R> - Only returns values that are lower than this score. By default `1.0`.
/// * `higher_threshold`: Option<R> - Only returns values that are higher than this score. By default `-1.0`.
/// * `verbose`: Option<bool> - Whether to show loading bars.
///
pub fn pairwise_cosine_similarity<
    R: Float + Send + Sync + 'static,
    F: AsPrimitive<R> + Send + Sync + Copy,
    I: ThreadUnsigned + Ord + std::fmt::Display,
>(
    matrix: &[F],
    sources: &[I],
    destinations: &[I],
    dimension: usize,
    minimum_threshold: Option<R>,
    maximum_threshold: Option<R>,
    verbose: Option<bool>
) -> Result<(Vec<I>, Vec<R>), String>
where
    <I as TryInto<usize>>::Error: Debug,
{
    let minimum_threshold: R = minimum_threshold.unwrap_or(R::one());
    let maximum_threshold: R = maximum_threshold.unwrap_or(-R::one());
    let verbose: bool = verbose.unwrap_or(true);

    let maximum_id = sources
        .par_iter()
        .chain(destinations.par_iter())
        .copied()
        .max()
        .unwrap_or(I::zero())
        .try_into()
        .unwrap();

    if maximum_id >= matrix.len() / dimension {
        return Err(format!(
            concat!(
                "The maximum provided element ID is {}, but ",
                "the matrix only contains {} rows."
            ),
            maximum_id,
            matrix.len() / dimension
        ));
    }

    let nodes: Frontier<I> = Frontier::new();
    let similarities: Frontier<R> = Frontier::new();

    let progress_bar = if verbose {
        let pb = ProgressBar::new(sources.len() as u64);
        pb.set_style(
            ProgressStyle::default_bar()
                .template(concat!(
                    "Computing cosine similarities ",
                    "{spinner:.green} [{elapsed_precise}] ",
                    "[{bar:40.cyan/blue}] ({pos}/{len}, ETA {eta})"
                ))
                .unwrap(),
        );
        pb
    } else {
        ProgressBar::hidden()
    };

    sources
        .par_iter()
        .copied()
        .progress_with(progress_bar)
        .for_each(|src|  {
            let usize_src: usize = src.try_into().unwrap();
            destinations.iter().copied().for_each(|dst| unsafe{
                let usize_dst: usize = dst.try_into().unwrap();

                let similarity = cosine_similarity_sequential_unchecked(
                    &matrix[usize_src * dimension..(usize_src + 1) * dimension],
                    &matrix[usize_dst * dimension..(usize_dst + 1) * dimension],
                )
                .0;

                if similarity < maximum_threshold || similarity > minimum_threshold {
                    return;
                }

                nodes.push(src);
                nodes.push(dst);
                similarities.push(similarity);
            });
        });

    Ok((nodes.into(), similarities.into()))
}
