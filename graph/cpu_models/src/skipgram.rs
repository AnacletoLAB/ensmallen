use crate::*;
use express_measures::{
    dot_product_sequential_unchecked, element_wise_weighted_addition_inplace,
    element_wise_weighted_subtraction_inplace, ThreadFloat,
};
use graph::{Graph, NodeT, ThreadDataRaceAware};
use num_traits::AsPrimitive;
use rayon::{current_num_threads, current_thread_index, prelude::*};
use vec_rand::splitmix64;

impl<W> Node2Vec<W>
where
    W: WalkTransformer,
{
    /// Computes in the provided slice of embedding the SkipGram node embedding.
    ///
    /// # Implementative details
    /// This implementation is NOT thread safe, that is, different threads may try
    /// to overwrite each others memory.
    ///
    /// # Arguments
    /// `graph`: &Graph - The graph to embed
    /// `embedding`: &mut [&mut [f32]] - The memory area where to write the embedding.
    pub(crate) fn fit_transform_skipgram<F: ThreadFloat + 'static>(
        &self,
        graph: &Graph,
        embedding: &mut [&mut [F]],
    ) -> Result<(), String>
    where
        f32: AsPrimitive<F>,
        NodeT: AsPrimitive<F>,
        usize: AsPrimitive<F>,
    {
        let scale_factor = (self.get_embedding_size() as f32).sqrt().as_();
        let mut walk_parameters = self.walk_parameters.clone();
        let mut random_state = splitmix64(self.walk_parameters.get_random_state() as u64);
        let mut learning_rate = self.learning_rate.as_();

        let shared_embedding = ThreadDataRaceAware::new(embedding);

        let number_of_threads = current_num_threads();
        let shared_gradients = ThreadDataRaceAware::new(vec![
            F::zero();
            self.get_embedding_size()
                * number_of_threads
        ]);

        // Depending whether verbosity was requested by the user
        // we create or not a visible progress bar to show the progress
        // in the training epochs.
        let pb = self.get_progress_bar();

        let compute_mini_batch_step = |central_node_embedding: &[F],
                                       cumulative_central_node_gradient: &mut [F],
                                       contextual_node_id: NodeT,
                                       label: bool,
                                       learning_rate: F| {
            let node_hidden = unsafe {
                &mut (*shared_embedding.get())[1][(contextual_node_id as usize
                    * self.embedding_size)
                    ..((contextual_node_id as usize + 1) * self.embedding_size)]
            };

            let prediction_proba: F = sigmoid(
                unsafe { dot_product_sequential_unchecked(node_hidden, central_node_embedding) }
                    / scale_factor,
            );

            let loss = binary_crossentropy(label, prediction_proba);
            let loss_derivative =
                binary_crossentropy_derivative(label, prediction_proba) * learning_rate;

            unsafe {
                element_wise_weighted_subtraction_inplace(
                    node_hidden,
                    central_node_embedding,
                    loss_derivative,
                )
            }

            unsafe {
                element_wise_weighted_addition_inplace(
                    cumulative_central_node_gradient,
                    node_hidden,
                    loss_derivative,
                )
            };

            loss
        };

        // We start to loop over the required amount of epochs.
        for _ in 0..self.epochs {
            pb.tick();
            // We update the random state used to generate the random walks
            // and the negative samples.
            random_state = splitmix64(random_state);
            walk_parameters = walk_parameters.set_random_state(Some(random_state as usize));

            // We start to compute the new gradients.
            let total_loss = graph
                .par_iter_complete_walks(&walk_parameters)?
                .enumerate()
                .map(|(walk_number, random_walk)| {
                    let thread_id = current_thread_index().unwrap_or(0);
                    let gradient = unsafe {
                        &mut (*shared_gradients.get())[thread_id * self.get_embedding_size()
                            ..(thread_id + 1) * self.get_embedding_size()]
                    };
                    (0..random_walk.len())
                        .map(|central_index| {
                            (
                                &random_walk[central_index.saturating_sub(self.window_size)
                                    ..(central_index + self.window_size).min(random_walk.len())],
                                random_walk[central_index],
                                central_index,
                            )
                        })
                        .map(|(context, central_node_id, central_index)| {
                            let central_node_embedding = unsafe {
                                &mut (*shared_embedding.get())[0][central_node_id as usize
                                    * self.embedding_size
                                    ..(central_node_id as usize + 1) * self.embedding_size]
                            };

                            // We now compute the gradient relative to the positive
                            let positive_loss: F = context
                                .iter()
                                .copied()
                                .filter(|&context_node_id| context_node_id != central_node_id)
                                .map(|context_node_id| {
                                    compute_mini_batch_step(
                                        &central_node_embedding,
                                        gradient,
                                        context_node_id,
                                        true,
                                        learning_rate,
                                    )
                                })
                                .sum();

                            // We compute the gradients relative to the negative classes.
                            let negative_loss: F = if self.use_scale_free_distribution {
                                graph
                                    .iter_random_outbounds_scale_free_node_ids(
                                        self.number_of_negative_samples,
                                        splitmix64(
                                            random_state
                                                + central_index as u64
                                                + walk_number as u64,
                                        ),
                                    )
                                    .filter(|&non_central_node_id| {
                                        non_central_node_id != central_node_id
                                    })
                                    .map(|non_central_node_id| {
                                        compute_mini_batch_step(
                                            &central_node_embedding,
                                            gradient,
                                            non_central_node_id,
                                            false,
                                            learning_rate,
                                        )
                                    })
                                    .sum()
                            } else {
                                graph
                                    .iter_random_node_ids(
                                        self.number_of_negative_samples,
                                        splitmix64(
                                            random_state
                                                + central_index as u64
                                                + walk_number as u64,
                                        ),
                                    )
                                    .filter(|&non_central_node_id| {
                                        non_central_node_id != central_node_id
                                    })
                                    .map(|non_central_node_id| {
                                        compute_mini_batch_step(
                                            &central_node_embedding,
                                            gradient,
                                            non_central_node_id,
                                            false,
                                            learning_rate,
                                        )
                                    })
                                    .sum()
                            };
                            // apply the accumulated gradient to the central node
                            central_node_embedding
                                .iter_mut()
                                .zip(gradient.iter_mut())
                                .for_each(|(node_feature, gradient)| {
                                    // We subtract the gradient
                                    *node_feature -= *gradient;
                                    // And reset it to zero for the next iteration.
                                    *gradient = F::zero();
                                });
                            positive_loss / (2 * self.window_size).as_()
                                + negative_loss / self.number_of_negative_samples.as_()
                        })
                        .sum::<F>()
                })
                .sum::<F>();
            let loss_per_node: F = total_loss / graph.get_number_of_nodes().as_();
            let loss_per_node: f32 = loss_per_node.as_();
            pb.set_message(format!("Loss: {:.4}", loss_per_node));
            pb.inc(1);
            learning_rate *= self.learning_rate_decay.as_()
        }
        Ok(())
    }
}
