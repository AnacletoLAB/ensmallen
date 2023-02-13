use core::cmp::PartialOrd;
use core::ops::Sub;

#[inline(always)]
pub fn absolute_distance<T>(a: T, b: T) -> T
where
    T: Sub + Sub<Output = T> + PartialOrd,
{
    if a > b {
        a - b
    } else {
        b - a
    }
}

/// Returns dynamic time warping for two given sequences on an arbitrary cost function.
///
/// # Arguments
/// * `first_sequence`: &[T] - A sequence of values.
/// * `second_sequence`: &[T] - Another sequence of values.
/// * `cost`: fn(a: &T, b: &T) -> F - A cost function defined on the type of values present in the provided sequences.
///
/// # Example
/// To compute the DTW cost between two sequences:
///
///```rust
/// use express_measures::dynamic_time_warping;
///
/// let first_sequence = vec![1, 2, 3, 4, 5];
/// let second_sequence = vec![6, 2, 3, 4];
///
/// let cost = |a: &usize, b: &usize| {
///     (if a > b {
///         a - b
///     } else {
///         b - a
///     }) as f32
/// };
///
/// let dtw_value = dynamic_time_warping(&first_sequence, &second_sequence, cost);
///
/// ```
pub fn dynamic_time_warping<T, F: ThreadFloat>(
    first_sequence: &[T],
    second_sequence: &[T],
    cost: fn(a: &T, b: &T) -> F,
) -> F {
    if first_sequence.is_empty() || second_sequence.is_empty() {
        return F::infinity();
    }
    let number_of_rows = first_sequence.len() + 1;
    let number_of_columns = second_sequence.len() + 1;
    let mut costs = vec![F::infinity(); number_of_rows * number_of_columns];
    costs[0] = F::zero();
    first_sequence
        .iter()
        .enumerate()
        .for_each(|(i, first_sequence_value)| {
            second_sequence
                .iter()
                .enumerate()
                .for_each(|(j, second_sequence_value)| {
                    let upper_cost = costs[i * number_of_columns + (j + 1)];
                    let left_cost = costs[(i + 1) * number_of_columns + j];
                    let left_upper_cost = costs[i * number_of_columns + j];
                    let minimum_cost = upper_cost.min(left_cost).min(left_upper_cost);
                    costs[(i+1) * number_of_columns + (j+1)] =
                        cost(first_sequence_value, second_sequence_value) + minimum_cost;
                });
        });
    costs[costs.len() - 1]
}

use crate::types::*;

/// Returns local dynamic time warping for two given sequences on an arbitrary cost function.
///
/// # Arguments
/// * `first_sequence`: &[T] - A sequence of values.
/// * `second_sequence`: &[T] - Another sequence of values.
/// * `cost`: fn(a: &T, b: &T) -> F - A cost function defined on the type of values present in the provided sequences.
/// * `local_window_size`: usize - Size of the local window to consider.
///
/// # Example
/// To compute the local DTW cost between two sequences:
///
///```rust
/// use express_measures::local_dynamic_time_warping;
///
/// let first_sequence = vec![1, 2, 3, 4, 5];
/// let second_sequence = vec![6, 2, 3, 4];
/// let local_window_size = 2;
///
/// let cost = |a: &usize, b: &usize| {
///     (if a > b {
///         a - b
///     } else {
///         b - a
///     }) as f32
/// };
///
/// let dtw_value = local_dynamic_time_warping(
///     &first_sequence,
///     &second_sequence,
///     cost,
///     local_window_size
/// );
///
/// ```
pub fn local_dynamic_time_warping<T, F: ThreadFloat>(
    first_sequence: &[T],
    second_sequence: &[T],
    cost: fn(a: &T, b: &T) -> F,
    mut local_window_size: usize,
) -> F {
    if first_sequence.is_empty() || second_sequence.is_empty() {
        return F::infinity();
    }
    let number_of_rows = first_sequence.len() + 1;
    let number_of_columns = second_sequence.len() + 1;
    let mut costs = vec![F::infinity(); number_of_rows * number_of_columns];
    local_window_size = local_window_size.max(absolute_distance(number_of_rows, number_of_columns));

    costs[0] = F::zero();
    (1..number_of_rows).for_each(|i| {
        (1.max(absolute_distance(i, local_window_size))
            ..number_of_columns.min(i + local_window_size))
            .for_each(|j| {
                costs[i * number_of_columns + j] = F::zero();
            });
    });

    first_sequence
        .iter()
        .enumerate()
        .for_each(|(i, first_sequence_value)| {
            (1.max(absolute_distance(i, local_window_size))
                ..number_of_columns.min(i + local_window_size))
                .map(|j| (j, &second_sequence[j - 1]))
                .for_each(|(j, second_sequence_value)| {
                    let upper_cost = costs[i * number_of_columns + (j + 1)];
                    let left_cost = costs[(i + 1) * number_of_columns + j];
                    let left_upper_cost = costs[i * number_of_columns + j];
                    let minimum_cost = upper_cost.min(left_cost).min(left_upper_cost);
                    costs[(i +1 )* number_of_columns + (j + 1)] =
                        cost(first_sequence_value, second_sequence_value) + minimum_cost;
                });
        });
    costs[costs.len() - 1]
}
