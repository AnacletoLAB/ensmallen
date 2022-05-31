use crate::types::*;

/// Validate whether the two provided features are compatible.
///
/// # Arguments
/// * `src_features`: &[F] - The first feature.
/// * `dst_features`: &[F] - The second feature.
///
/// # Raises
/// * If one of the two vectors are empty.
/// * If the two vectors have different sizes.
pub(crate) fn validate_features<F: ThreadFloat>(
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
                "but the provided destination features have size `{}`.",
            ),
            src_features.len(),
            dst_features.len()
        ));
    }
    Ok(())
}

/// Validates whether the two provided vector lengths are compatible.
///
/// # Arguments
/// * `ground_truth_len`: usize - The length of the ground truths vector.
/// * `predictions_len`: usize - The length of the predictions vector.
pub(crate) fn validate_vectors_length(
    ground_truth_len: usize,
    predictions_len: usize,
) -> Result<(), String> {
    if ground_truth_len == 0 {
        return Err("The provided ground truths vector is empty!".to_string());
    }
    if predictions_len == 0 {
        return Err("The provided predictions vector is empty!".to_string());
    }
    if ground_truth_len != predictions_len {
        return Err(format!(
            concat!(
                "The provided ground truth have length `{}` ",
                "but the provided predictions have length `{}`. ",
                "The two vectors should have the same length."
            ),
            ground_truth_len, predictions_len,
        ));
    }
    Ok(())
}

/// Validates the provided features.
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
pub(crate) fn validate_features_from_indices<F: ThreadFloat, I: ThreadUnsigned>(
    similarities: &mut [F],
    matrix: &[F],
    sources: &[I],
    destinations: &[I],
    dimension: usize,
) -> Result<(), String> {
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
    Ok(())
}
