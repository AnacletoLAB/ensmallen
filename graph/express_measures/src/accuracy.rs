use crate::validation::*;
use rayon::prelude::*;

/// Returns the accuracy obtaines in the two provided iterators.
///
/// # Arguments
/// * `ground_truth_iter`: Iter1 - Iterator over the ground truths.
/// * `predictions_iter`: Iter2 - Iterator over the predictions.
fn accuracy_score_from_iter<'a, Iter1, Iter2, C>(
    ground_truth_iter: Iter1,
    predictions_iter: Iter2,
) -> Result<f32, String>
where
    Iter1: IndexedParallelIterator<Item = &'a C>,
    Iter2: IndexedParallelIterator<Item = &'a C>,
    C: Send + Sync + Eq + 'a,
{
    validate_vectors_length(ground_truth_iter.len(), predictions_iter.len())?;
    let number_of_predictions = predictions_iter.len();
    Ok(ground_truth_iter
        .zip(predictions_iter)
        .map(|(ground_truth_value, predicted_value)| {
            if ground_truth_value == predicted_value {
                1
            } else {
                0
            }
        })
        .sum::<usize>() as f32
        / number_of_predictions as f32)
}

/// Returns accuracy score on the provided predictions.
pub fn accuracy_score<C>(ground_truth: &[C], predictions: &[C]) -> Result<f32, String>
where
    C: Send + Sync + Eq,
{
    accuracy_score_from_iter(ground_truth.par_iter(), predictions.par_iter())
}
