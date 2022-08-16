use crate::*;
use express_measures::{
    dot_product_sequential_unchecked, element_wise_addition_inplace,
    element_wise_weighted_addition_inplace, ThreadFloat,
};
use graph::{Graph, NodeT, ThreadDataRaceAware};
use indicatif::ProgressIterator;
use rayon::prelude::*;
use num_traits::Coerced;
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
    pub(crate) fn fit_transform_skipgram<F: ThreadFloat>(
        &self,
        graph: &Graph,
        embedding: &mut [&mut [F]],
    ) -> Result<(), String>
    where
        NodeT: Coerced<F>,
    {
        let scale_factor = F::coerce_from((self.get_embedding_size() as f32).sqrt());
        let mut walk_parameters = self.walk_parameters.clone();
        let mut random_state = splitmix64(self.walk_parameters.get_random_state() as u64);
        let mut learning_rate = F::coerce_from(self.learning_rate);
        let cv = F::coerce_from(self.clipping_value);
        let nodes_number = graph.get_number_of_nodes();

        let shared_embedding = ThreadDataRaceAware::new(embedding);

        // Depending whether verbosity was requested by the user
        // we create or not a visible progress bar to show the progress
        // in the training epochs.
        let pb = self.get_progress_bar();

        // Create the closure to apply a gradient to a provided node's embedding
        let update_central_node_embedding = |node_id: NodeT, variation: &[F]| {
            let node_id = node_id as usize;
            unsafe {
                element_wise_addition_inplace(
                    &mut (*shared_embedding.get())[0]
                        [node_id * self.embedding_size..(node_id + 1) * self.embedding_size],
                    variation,
                )
            }
        };

        // Create the closure to apply a gradient to a provided node's hidden layer weights
        let update_contextual_node_embedding = |node_id: NodeT, variation: &[F], weight: F| {
            let node_id = node_id as usize;
            unsafe {
                element_wise_weighted_addition_inplace(
                    &mut (*shared_embedding.get())[1]
                        [node_id * self.embedding_size..(node_id + 1) * self.embedding_size],
                    variation,
                    weight,
                )
            }
        };

        // We define a closure that returns a reference to the embedding of the given node.
        let get_central_node_embedding = |node_id: NodeT| {
            let node_id = node_id as usize;
            unsafe {
                &(*shared_embedding.get())[0]
                    [(node_id * self.embedding_size)..((node_id + 1) * self.embedding_size)]
            }
        };

        // We define a closure that returns a reference to the hidden of the given node.
        let get_contextual_node_embedding = |node_id: NodeT| {
            let node_id = node_id as usize;
            unsafe {
                &(*shared_embedding.get())[1]
                    [(node_id * self.embedding_size)..((node_id + 1) * self.embedding_size)]
            }
        };

        let compute_mini_batch_step = |total_context_embedding: &[F],
                                       context_embedding_gradient: &mut [F],
                                       node_id: NodeT,
                                       label: F,
                                       learning_rate: F| {
            let node_hidden = get_contextual_node_embedding(node_id);
            let dot: F =
                unsafe { dot_product_sequential_unchecked(node_hidden, total_context_embedding) }
                    / scale_factor;

            if dot > cv || dot < cv {
                return;
            }

            let exp_dot = dot.exp();
            let mut variation = (label - exp_dot / (exp_dot + F::one())) * learning_rate;

            if self.normalize_learning_rate_by_degree {
                variation *= get_node_prior(graph, node_id, F::one());
            }

            let variation = F::coerce_from(variation);

            update_contextual_node_embedding(node_id, total_context_embedding, variation);
            unsafe {
                element_wise_weighted_addition_inplace(
                    context_embedding_gradient,
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
                                degree < sample_uniform(nodes_number as _, seed) as _
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
                            context
                                .iter()
                                .copied()
                                .filter(|&context_node_id| context_node_id != central_node_id)
                                .for_each(|context_node_id| {
                                    let mut context_gradient =
                                        vec![F::coerce_from(0.0); self.get_embedding_size()];
                                    let context_node_embedding =
                                        get_central_node_embedding(context_node_id);
                                    // We now compute the gradient relative to the positive
                                    compute_mini_batch_step(
                                        &context_node_embedding,
                                        context_gradient.as_mut_slice(),
                                        central_node_id,
                                        F::one(),
                                        learning_rate,
                                    );

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
                                                    && non_central_node_id != context_node_id
                                            })
                                            .for_each(|non_central_node_id| {
                                                compute_mini_batch_step(
                                                    &context_node_embedding,
                                                    context_gradient.as_mut_slice(),
                                                    non_central_node_id,
                                                    F::zero(),
                                                    learning_rate,
                                                )
                                            });
                                    } else {
                                        (0..self.number_of_negative_samples)
                                            .map(|i| {
                                                let seed = splitmix64(
                                                    random_state
                                                        + central_index as u64
                                                        + walk_number as u64
                                                        + i as u64,
                                                );
                                                sample_uniform(nodes_number as _, seed) as NodeT
                                            })
                                            .filter(|&non_central_node_id| {
                                                non_central_node_id != central_node_id
                                                    && non_central_node_id != context_node_id
                                            })
                                            .for_each(|non_central_node_id| {
                                                compute_mini_batch_step(
                                                    &context_node_embedding,
                                                    context_gradient.as_mut_slice(),
                                                    non_central_node_id,
                                                    F::one(),
                                                    learning_rate,
                                                )
                                            });
                                    };
                                    update_central_node_embedding(
                                        context_node_id,
                                        &context_gradient,
                                    );
                                });
                        });
                });
            learning_rate *= F::coerce_from(self.learning_rate_decay);
        }
        Ok(())
    }
}
