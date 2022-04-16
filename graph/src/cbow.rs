use super::*;
use atomic_float::AtomicF32;
use indicatif::ProgressIterator;
use rayon::iter::IndexedParallelIterator;
use rayon::iter::IntoParallelIterator;
use rayon::iter::IntoParallelRefIterator;
use rayon::iter::ParallelIterator;
use std::sync::atomic::Ordering;
use vec_rand::{random_f32, sample_uniform};

impl Graph {
    #[manual_binding]
    /// Given a memory allocation `embedding` (which HAVE TO be already initialized at
    /// 0.0), write into it the CBOW embeddings
    pub fn compute_cbow_embedding(
        &self,
        embedding: &mut [f32],
        embedding_size: Option<usize>,
        epochs: Option<usize>,
        walk_length: Option<u64>,
        return_weight: Option<f32>,
        explore_weight: Option<f32>,
        change_edge_type_weight: Option<f32>,
        change_node_type_weight: Option<f32>,
        iterations: Option<NodeT>,
        max_neighbours: Option<NodeT>,
        normalize_by_degree: Option<bool>,
        window_size: Option<usize>,
        negatives_number: Option<usize>,
        learning_rate: Option<f32>,
        random_state: Option<u64>,
        verbose: Option<bool>,
    ) -> Result<()> {
        let embedding_size = embedding_size.unwrap_or(100);
        let walk_length = walk_length.unwrap_or(128);
        let window_size = window_size.unwrap_or(4);
        let context_size = (window_size * 2) as f32;
        let epochs = epochs.unwrap_or(1);
        let negatives_number = negatives_number.unwrap_or(5);
        let learning_rate = learning_rate.unwrap_or(0.025);
        let mut random_state = random_state.unwrap_or(42);
        random_state = splitmix64(random_state);
        let verbose = verbose.unwrap_or(true);

        if embedding_size == 0 {
            return Err("The embedding size must be greater than zero.".to_string());
        }

        if epochs == 0 {
            return Err("The number of epochs must be greater than zero.".to_string());
        }

        if !self.has_nodes() {
            return Err("The current graph does not have any node.".to_string());
        }

        if !self.has_nodes_sorted_by_decreasing_outbound_node_degree() {
            return Err("The current graph does not have decreasing outbounds.".to_string());
        }

        if (walk_length as usize) < window_size * 2 + 1 {
            panic!(
                "
            Cannot compute word2vec, got a sequence of length {} and window size {}.
            for the current window_size the minimum sequence length required is {}",
                walk_length,
                window_size,
                window_size * 2 + 1,
            );
        }

        let expected_embedding_len = embedding_size * self.get_nodes_number() as usize;

        if embedding.len() != expected_embedding_len {
            return Err(format!(
                "The given memory allocation for the embeddings is {} long but we expect {}.",
                embedding.len(),
                expected_embedding_len
            ));
        }

        let embedding = unsafe { core::mem::transmute::<&mut [f32], &mut [AtomicF32]>(embedding) };

        embedding.par_iter().enumerate().for_each(|(i, e)| {
            e.store(
                2.0 * random_f32(random_state + i as u64) - 1.0,
                Ordering::SeqCst,
            )
        });

        let negative_embedding = (0..(embedding_size * self.get_nodes_number() as usize))
            .into_par_iter()
            .map(|i| AtomicF32::new(2.0 * random_f32(random_state + i as u64) - 1.0))
            .collect::<Vec<_>>();
        let pb = get_loading_bar(verbose, "Training CBOW model", epochs);

        let number_of_directed_edges = self.get_number_of_directed_edges();

        let weighted_sum = |factor: f32, source: &[AtomicF32], result: &mut [f32]| {
            result
                .iter_mut()
                .zip(source.iter().map(|a| a.load(Ordering::SeqCst)))
                .for_each(|(a, b)| {
                    *a += b * factor;
                });
        };

        let atomic_sum = |source: &[f32], result: &[AtomicF32]| {
            result
                .iter()
                .zip(source.iter().cloned())
                .for_each(|(a, b)| {
                    a.fetch_add(b, Ordering::SeqCst);
                });
        };

        let atomic_weighted_sum = |factor: f32, source: &[f32], result: &[AtomicF32]| {
            result
                .iter()
                .zip(source.iter().cloned())
                .for_each(|(a, b)| {
                    a.fetch_add(b * factor, Ordering::SeqCst);
                });
        };

        let compute_dot_product = |v1: &[f32], v2: &[f32]| -> f32 {
            v1.iter()
                .cloned()
                .zip(v2.iter().cloned())
                .map(|(a, b)| a * b)
                .sum()
        };

        let mut walk_parameters = WalksParameters::new(walk_length)?;
        walk_parameters = walk_parameters
            .set_change_edge_type_weight(change_edge_type_weight)?
            .set_change_node_type_weight(change_node_type_weight)?
            .set_explore_weight(explore_weight)?
            .set_return_weight(return_weight)?
            .set_max_neighbours(max_neighbours)?
            .set_normalize_by_degree(normalize_by_degree)
            .set_iterations(iterations)?;

        for _ in (0..epochs).progress_with(pb) {
            random_state = splitmix64(random_state);

            walk_parameters = walk_parameters.set_random_state(Some(random_state as usize));

            self.iter_complete_walks(&walk_parameters)?
                .enumerate()
                .for_each(|(i, sequence)| {
                    (window_size..(walk_length as usize - window_size)).for_each(|j| {
                        let get_contextual_nodes_indices = || {
                            sequence[j - window_size..j]
                                .iter()
                                .chain(sequence[j + 1..window_size + j + 1].iter())
                                .map(|&contextual_node_index| contextual_node_index as usize)
                        };
                        let central_node_index = sequence[j];
                        let mut random_state = splitmix64(
                            random_state
                                .wrapping_add(i as u64)
                                .wrapping_add((j as u64) * walk_length),
                        );
                        let mut context_mean_embedding = vec![0.0; embedding_size];
                        let mut negative_context_total_embedding = vec![0.0; embedding_size];
                        get_contextual_nodes_indices().for_each(|contextual_node_index| {
                            context_mean_embedding
                                .iter_mut()
                                .zip(
                                    embedding[(contextual_node_index * embedding_size)
                                        ..((contextual_node_index + 1) * embedding_size)]
                                        .iter()
                                        .map(|e| e.load(Ordering::SeqCst)),
                                )
                                .for_each(|(c, e)| *c += e);
                        });

                        // Start to sample negative indices
                        vec![(central_node_index as usize, 1.0)]
                            .iter()
                            .cloned()
                            .chain(
                                (0..negatives_number)
                                    .filter_map(|_| unsafe {
                                        let sampled_node = self
                                            .get_unchecked_node_ids_from_edge_id(sample_uniform(
                                                number_of_directed_edges,
                                                random_state,
                                            )
                                                as EdgeT)
                                            .0;
                                        random_state = splitmix64(random_state);
                                        if sampled_node == central_node_index {
                                            None
                                        } else {
                                            Some(sampled_node)
                                        }
                                    })
                                    .map(|sampled_node| (sampled_node as usize, 0.0)),
                            )
                            .for_each(|(node_index, label): (usize, f32)| {
                                // Sample negative index
                                // Retrieve the node embedding from the negative embedding
                                // curresponding to the `negative_node_index` node.
                                let node_negative_embedding = &negative_embedding[(node_index
                                    * embedding_size)
                                    ..((node_index + 1) * embedding_size)];
                                // Compute the dot product between the negative embedding and the context average.
                                let dot_product: f32 = compute_dot_product(
                                    unsafe {
                                        core::mem::transmute::<&[AtomicF32], &[f32]>(
                                            node_negative_embedding,
                                        )
                                    },
                                    context_mean_embedding.as_slice(),
                                ) / context_size;
                                // Othersiwe, we proceed to retrieve the exponentiated value from
                                // the lookup table.
                                let exponentiated_dot_product = dot_product.exp();
                                // Finally, we compute this portion of the error.
                                let loss = (label
                                    - (exponentiated_dot_product
                                        / (exponentiated_dot_product + 1.0)))
                                    * learning_rate;

                                // We sum the currently sampled negative context node embedding
                                // to the (currently sum of) negative context embeddings,
                                // weighted by the current loss.
                                weighted_sum(
                                    loss,
                                    node_negative_embedding,
                                    &mut negative_context_total_embedding,
                                );

                                // We sum the mean context embedding
                                // to the negative embedding of the currently sampled negative context node
                                // weighted by the current loss.
                                atomic_weighted_sum(
                                    loss / context_size,
                                    context_mean_embedding.as_ref(),
                                    &node_negative_embedding,
                                );
                            });

                        // Update the node embedding of every node in the context.
                        get_contextual_nodes_indices()
                            .map(|contextual_node_index| contextual_node_index as usize)
                            .for_each(|contextual_node_index| {
                                atomic_sum(
                                    negative_context_total_embedding.as_slice(),
                                    &embedding[(contextual_node_index * embedding_size)
                                        ..((contextual_node_index + 1) * embedding_size)],
                                );
                            });
                    });
                });
        }
        Ok(())
    }
}
