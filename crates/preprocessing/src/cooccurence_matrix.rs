use std::collections::HashMap;
use indicatif::ProgressIterator;
use rayon::prelude::*;
use shared::*;
use tags::*;

#[manual_binding]
/// Return triple with CSR representation of cooccurrence matrix.
///
/// The first vector has the sources, the second vector the destinations
/// and the third one contains the min-max normalized frequencies.
///
/// # Arguments
///
/// * `sequences`: impl ParallelIterator<Item = Vec<NodeT>> - the sequence of sequences of integers to preprocess.
/// * `window_size`: Option<usize> - Window size to consider for the sequences.
/// * `verbose`: Option<bool> - Whether to show the progress bars. The default behaviour is false.
///     
pub fn cooccurence_matrix(
    sequences: impl ParallelIterator<Item = Vec<NodeT>>,
    window_size: usize,
    number_of_sequences: usize,
    verbose: Option<bool>,
) -> Result<(usize, impl Iterator<Item = (NodeT, NodeT, f64)>)> {
    let verbose = verbose.unwrap_or(false);
    let mut cooccurence_matrix: HashMap<(NodeT, NodeT), f64> = HashMap::new();
    let mut max_frequency = 0.0;
    let pb1 = get_loading_bar(verbose, "Computing frequencies", number_of_sequences);

    // TODO!: Avoid this collect and create the cooccurrence matrix in a parallel way.
    // We are currently working on this but is terribly non-trivial,
    // as most parallel implementations end up being slower than sequential
    // ones or require massive amounts of additional memory.
    let vec = sequences.collect::<Vec<Vec<NodeT>>>();
    vec.iter().progress_with(pb1).for_each(|sequence| {
        let walk_length = sequence.len();
        for (central_index, &central_word_id) in sequence.iter().enumerate() {
            let upperbound = std::cmp::min(1 + window_size, walk_length - central_index);

            for distance in 1..upperbound {
                let context_id = sequence[central_index + distance];

                let (smaller, bigger) = (
                    std::cmp::min(central_word_id, context_id),
                    std::cmp::max(central_word_id, context_id),
                );

                let freq = 1.0 / distance as f64;

                // Get the current value for this pair of nodes
                let ptr = cooccurence_matrix
                    .entry((smaller, bigger))
                    .and_modify(|e| *e += freq)
                    .or_insert(freq);
                // Update the max
                if *ptr > max_frequency {
                    max_frequency = *ptr;
                }
            }
        }
    });

    let number_of_elements = cooccurence_matrix.len();
    let pb2 = get_loading_bar(
        verbose,
        "Converting mapping into CSR matrix",
        cooccurence_matrix.len(),
    );
    Ok((
        number_of_elements,
        cooccurence_matrix
            .into_iter()
            .progress_with(pb2)
            .map(move |((word, context), frequency)| (word, context, frequency / max_frequency)),
    ))
}