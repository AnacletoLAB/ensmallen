use crate::types::*;

/// Returns the dot product between the provided matrix and vector computed sequentially.
///
/// # Arguments
/// * `matrix`: &[F] - The matrix to be multiplied.
/// * `vector`: &[F] - The second feature.
///
/// # Safety
/// If the two features have different sizes, we will compute
/// the dot product upwards to when the minimum size.
/// No warning will be raised.
pub unsafe fn matrix_vector_dot_product_sequential_unchecked<F: ThreadFloat>(
    matrix: &[F],
    vector: &[F],
) -> Vec<F> {
    matrix
        .chunks(matrix.len() / vector.len())
        .zip(vector.iter().copied())
        .map(|(row, vector_value)| {
            row.iter()
                .copied()
                .map(|row_value| row_value * vector_value)
                .sum()
        })
        .collect()
}
