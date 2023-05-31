use crate::*;
use express_measures::{
    dot_product_sequential_unchecked, element_wise_addition_inplace,
    element_wise_weighted_addition_inplace, ThreadFloat,
};
use graph::{Graph, NodeT, ThreadDataRaceAware};
use indicatif::ProgressIterator;
use num_traits::AsPrimitive;
use rayon::prelude::*;
use vec_rand::{sample_uniform, splitmix64};

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
    {
        let scale_factor = (self.get_embedding_size() as f32).sqrt().as_();
        let mut walk_parameters = self.walk_parameters.clone();
        let mut random_state = splitmix64(self.walk_parameters.get_random_state() as u64);
        let mut learning_rate = self.learning_rate.as_();
        let cv = self.clipping_value.as_();
        let number_of_nodes = graph.get_number_of_nodes();

        let shared_embedding = ThreadDataRaceAware::new(embedding);

        // Depending whether verbosity was requested by the user
        // we create or not a visible progress bar to show the progress
        // in the training epochs.
        let pb = self.get_progress_bar();

        let compute_mini_batch_step = |central_node_embedding: &[F],
                                       cumulative_central_node_gradient: &mut [F],
                                       contextual_node_id: NodeT,
                                       label: F,
                                       learning_rate: F| {
            let node_hidden = unsafe {
                &mut (*shared_embedding.get())[1][(contextual_node_id as usize
                    * self.embedding_size)
                    ..((contextual_node_id as usize + 1) * self.embedding_size)]
            };

            let dot: F =
                unsafe { dot_product_sequential_unchecked(node_hidden, central_node_embedding) }
                    / scale_factor;

            if dot > cv || dot < -cv {
                return;
            }

            let mut variation = (label - sigmoid(dot)) * learning_rate;

            if self.normalize_learning_rate_by_degree {
                variation *= get_node_prior(graph, contextual_node_id, F::one());
            }

            unsafe {
                element_wise_weighted_addition_inplace(
                    node_hidden,
                    central_node_embedding,
                    variation,
                )
            }

            unsafe {
                element_wise_weighted_addition_inplace(
                    cumulative_central_node_gradient,
                    node_hidden,
                    variation,
                )
            };
        };

        // We start to loop over the required amount of epochs.
        for _ in (0..self.epochs).progress_with(pb) {
            // We update the random state used to generate the random walks
            // and the negative samples.
            random_state = splitmix64(random_state);
            walk_parameters = walk_parameters.set_random_state(Some(random_state as usize));

            // We start to compute the new gradients.
            graph
                .par_iter_complete_walks(&walk_parameters)?
                .enumerate()
                .for_each(|(walk_number, random_walk)| {
                    (0..random_walk.len())
                        .filter(|&central_index| {
                            if !self.stochastic_downsample_by_degree {
                                true
                            } else {
                                let degree = unsafe {
                                    graph.get_unchecked_node_degree_from_node_id(
                                        random_walk[central_index as usize],
                                    )
                                };
                                let seed = splitmix64(
                                    random_state + central_index as u64 + walk_number as u64,
                                );
                                degree < sample_uniform(number_of_nodes as _, seed) as _
                            }
                        })
                        .map(|central_index| {
                            (
                                &random_walk[central_index.saturating_sub(self.window_size)
                                    ..(central_index + self.window_size).min(random_walk.len())],
                                random_walk[central_index],
                                central_index,
                            )
                        })
                        .for_each(|(context, central_node_id, central_index)| {
                            let mut cumulative_central_node_gradient =
                                vec![F::zero(); self.get_embedding_size()];
                            let central_node_embedding = unsafe {
                                &mut (*shared_embedding.get())[0][central_node_id as usize
                                    * self.embedding_size
                                    ..(central_node_id as usize + 1) * self.embedding_size]
                            };

                            // We now compute the gradient relative to the positive
                            context
                                .iter()
                                .copied()
                                .filter(|&context_node_id| context_node_id != central_node_id)
                                .for_each(|context_node_id| {
                                    compute_mini_batch_step(
                                        &central_node_embedding,
                                        cumulative_central_node_gradient.as_mut_slice(),
                                        context_node_id,
                                        F::one(),
                                        learning_rate,
                                    );
                                });

                            // We compute the gradients relative to the negative classes.
                            if self.use_scale_free_distribution {
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
                                    .for_each(|non_central_node_id| {
                                        compute_mini_batch_step(
                                            &central_node_embedding,
                                            cumulative_central_node_gradient.as_mut_slice(),
                                            non_central_node_id,
                                            F::zero(),
                                            learning_rate,
                                        )
                                    });
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
                                    .for_each(|non_central_node_id| {
                                        compute_mini_batch_step(
                                            &central_node_embedding,
                                            cumulative_central_node_gradient.as_mut_slice(),
                                            non_central_node_id,
                                            F::zero(),
                                            learning_rate,
                                        )
                                    });
                            };
                            // apply the accumulated gradient to the central node
                            unsafe {
                                element_wise_addition_inplace(
                                    central_node_embedding,
                                    cumulative_central_node_gradient.as_slice(),
                                )
                            }
                        });
                });
            learning_rate *= self.learning_rate_decay.as_()
        }
        Ok(())
    }
}
