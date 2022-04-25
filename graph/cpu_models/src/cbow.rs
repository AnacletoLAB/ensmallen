use graph::{Graph, NodeT, ThreadDataRaceAware, WalksParameters};
use indicatif::{ProgressBar, ProgressIterator, ProgressStyle};
use rayon::iter::IndexedParallelIterator;
use rayon::iter::IntoParallelIterator;
use rayon::iter::IntoParallelRefMutIterator;
use rayon::iter::ParallelIterator;
use rayon::prelude::ParallelSlice;
use rayon::prelude::ParallelSliceMut;
use vec_rand::{random_f32, splitmix64};

pub struct CBOW {
    embedding_size: usize,
    window_size: usize,
    walk_parameters: WalksParameters,
    number_of_negative_samples: usize,
    siamese: bool,
}

impl CBOW {
    /// Return new instance of CBOW model.
    pub fn new(
        embedding_size: Option<usize>,
        walk_parameters: Option<WalksParameters>,
        window_size: Option<usize>,
        number_of_negative_samples: Option<usize>,
        siamese: Option<bool>,
    ) -> Result<Self, String> {
        // Handle the values of the default parameters.
        let embedding_size = embedding_size.unwrap_or(100);
        let window_size = window_size.unwrap_or(10);
        let walk_parameters = walk_parameters.unwrap_or_else(|| WalksParameters::default());
        let number_of_negative_samples = number_of_negative_samples.unwrap_or(5);
        let siamese = siamese.unwrap_or(true);

        // Validate that the provided parameters are within
        // reasonable bounds.
        if embedding_size == 0 {
            return Err(concat!("The embedding size cannot be equal to zero.").to_string());
        }
        if window_size == 0 {
            return Err(concat!("The window size cannot be equal to zero.").to_string());
        }
        if number_of_negative_samples == 0 {
            return Err(
                concat!("The number of negative samples cannot be equal to zero.").to_string(),
            );
        }

        Ok(Self {
            embedding_size,
            window_size,
            walk_parameters,
            number_of_negative_samples,
            siamese,
        })
    }

    pub fn fit_transform(
        &self,
        graph: &Graph,
        mut embedding: &mut [f32],
        epochs: Option<usize>,
        learning_rate: Option<f32>,
        batch_size: Option<usize>,
        verbose: Option<bool>,
    ) -> Result<(), String> {
        let apply_l2_norm = true;
        let max_weights = 1000.0;
        let epochs = epochs.unwrap_or(10);
        let batch_size = batch_size.unwrap_or(32);
        let number_of_batches_per_epoch =
            (graph.get_nodes_number() as f64 / batch_size as f64).ceil() as usize;

        let scale_factor = (self.embedding_size as f32).sqrt();

        let mut walk_parameters = self.walk_parameters.clone();
        let mut random_state = splitmix64(self.walk_parameters.get_random_state() as u64);
        let random_walk_length = walk_parameters.get_random_walk_length() as usize;
        let iterations = walk_parameters.get_iterations() as usize;
        let verbose = verbose.unwrap_or(true);
        let cpu_number = rayon::current_num_threads() as NodeT;
        let context_size = (self.window_size * 2) as f32;
        let number_of_random_walks = batch_size * iterations;
        let learning_rate = learning_rate.unwrap_or(0.025);

        if epochs == 0 {
            return Err("The number of epochs must be strictly greater than zero.".to_string());
        }

        if !graph.has_nodes() {
            return Err("The provided graph does not have any node.".to_string());
        }

        if !graph.has_nodes_sorted_by_decreasing_outbound_node_degree() {
            return Err(concat!(
                "The provided graph does not have nodes sorted by decreasing node degrees ",
                "and therefore the negative sampling used to approximate the sigmoid and ",
                "binary cross-entropy loss. You can sort this graph the desired way by ",
                "using the `graph.sort_by_decreasing_outbound_node_degree()` method. ",
                "Do note that this method does not sort in-place ",
                "but creates a new instance of the provided graph. "
            )
            .to_string());
        }

        let expected_embedding_len = self.embedding_size * graph.get_nodes_number() as usize;

        if embedding.len() != expected_embedding_len {
            return Err(format!(
                "The given memory allocation for the embeddings is {} long but we expect {}.",
                embedding.len(),
                expected_embedding_len
            ));
        }

        // Populate the embedding layer with random uniform value
        // This matrix has size:
        // height = number of nodes in the graph
        // width  = number of features in embedding
        embedding
            .par_iter_mut()
            .enumerate()
            .for_each(|(i, e)| *e = 2.0 * random_f32(splitmix64(random_state + i as u64)) - 1.0);

        // Update the random state to avoid populating the hidden layer with
        // the same exact values as the embedding.
        random_state = splitmix64(random_state);

        // Create and allocate the hidden layer
        // This matrix has the same size of the embedding layer:
        // height = number of nodes in the graph
        // width  = number of features in embedding
        let mut hidden = (0..expected_embedding_len)
            .into_par_iter()
            .map(|i| 2.0 * random_f32(splitmix64(random_state + i as u64)) - 1.0)
            .collect::<Vec<_>>();

        // Create and allocate the gradient for the central terms
        // This particular gradient has a size equal to:
        // height = batch_size * (walk_length - window_size * 2) * iterations
        // width  = embedding_size
        let number_of_central_terms_in_walk = random_walk_length - self.window_size * 2;
        let number_of_central_terms_in_batch =
            number_of_central_terms_in_walk * number_of_random_walks;

        // We initialize the central terms gradient to zeros.
        let mut central_terms_batch_gradient =
            vec![0.0; number_of_central_terms_in_batch * self.embedding_size];

        // Create and allocate the gradient for the non-central terms
        // that are also referred to as negative terms.
        // This particular gradient has a size equal to:
        // height = number_of_central_terms_in_batch * self.number_of_negative_samples
        // width  = embedding_size
        let number_of_non_central_terms_in_batch =
            number_of_central_terms_in_batch * self.number_of_negative_samples;

        // We initialize the central terms gradient to zeros.
        let mut non_central_terms_batch_gradient =
            vec![0.0; number_of_non_central_terms_in_batch * self.embedding_size];

        // Create and allocate the gradient for the contextual terms
        // This particular gradient has a size equal to:
        // height = number_of_central_terms_in_batch
        // width  = embedding_size
        // because the gradient of each element in the context is identical
        let mut contextual_terms_batch_gradient =
            vec![0.0; number_of_central_terms_in_batch * self.embedding_size];

        // Create the vector we will populate with the random walks.
        let mut random_walks: Vec<NodeT> =
            vec![0; number_of_random_walks * random_walk_length as usize];

        // Create the vector we will populate with the non-central nodes.
        let mut non_central_terms: Vec<NodeT> = vec![0; number_of_non_central_terms_in_batch];

        // We create a closure that will be used within the threads to check whether a
        // thread can execute the update of a given node embedding.
        let can_update = |node_id: NodeT, thread_id: NodeT| node_id % cpu_number == thread_id;

        // Depending whether verbosity was requested by the user
        // we create or not a visible progress bar to show the progress
        // in the training epochs.
        let epochs_progress_bar = if verbose {
            let pb = ProgressBar::new(epochs as u64);
            pb.set_style(ProgressStyle::default_bar().template(
                "CBOW Epochs {spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] ({pos}/{len}, ETA {eta})",
            ));
            pb
        } else {
            ProgressBar::hidden()
        };

        // We start to loop over the required amount of epochs.
        for _ in (0..epochs).progress_with(epochs_progress_bar) {
            // Depending whether verbosity was requested by the user
            // we create or not a visible progress bar to show the progress
            // in the training batches.
            let batches_progress_bar = if verbose {
                let pb = ProgressBar::new(number_of_batches_per_epoch as u64);
                pb.set_style(ProgressStyle::default_bar().template(
                    "Batches {spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] ({pos}/{len}, ETA {eta})",
                ));
                pb
            } else {
                ProgressBar::hidden()
            };

            // We start to loop over the required amount of batches.
            for _ in (0..number_of_batches_per_epoch).progress_with(batches_progress_bar) {
                // We update the random state used to generate the random walks
                // and the negative samples.
                random_state = splitmix64(random_state);
                walk_parameters = walk_parameters.set_random_state(Some(random_state as usize));

                // The first part of the current training batch is constituted by the random walks
                graph
                    .populate_random_walks_slice(
                        batch_size as NodeT,
                        &self.walk_parameters,
                        random_walks.as_mut_slice(),
                    )
                    .unwrap();

                // The second part by the negative node IDs
                graph
                    .par_iter_random_source_node_ids(
                        number_of_non_central_terms_in_batch,
                        random_state,
                    )
                    .collect_into_vec(&mut non_central_terms);

                {
                    // We define a closure that returns a reference to the embedding of the given node.
                    let get_node_embedding = |node_id: NodeT| {
                        let node_id = node_id as usize;
                        &embedding
                            [(node_id * self.embedding_size)..((node_id + 1) * self.embedding_size)]
                    };

                    // We define a closure for better code readability that computes the loss
                    // given a TOTAL context embedding, a node ID and its label.
                    let compute_mini_batch_step =
                        |total_context_embedding: &[f32],
                         node_id: NodeT,
                         label: f32,
                         node_gradient: &mut [f32],
                         context_gradient: &mut [f32]| {
                            // We compute the average exponentiated dot product
                            let node_id = node_id as usize;
                            // We retrieve the hidden weights of the current node ID.
                            let hidden_embedding = &hidden[(node_id * self.embedding_size)
                                ..((node_id + 1) * self.embedding_size)];
                            // Within this computation, we also do a conversion to f64
                            // that we convert back to f32 afterwards. This is done because
                            // we want to avoid as much numerical instability as possible.
                            let dot = hidden_embedding
                                .iter()
                                .cloned()
                                .zip(total_context_embedding.iter().cloned())
                                .map(|(node_feature, contextual_feature)| {
                                    node_feature * contextual_feature
                                })
                                .sum::<f32>()
                                / context_size
                                / scale_factor;

                            let hidden_embedding_l2 = hidden_embedding
                                .iter()
                                .cloned()
                                .map(|value| value.powf(2.0))
                                .sum::<f32>()
                                .sqrt();

                            let context_embedding_l2 = total_context_embedding
                                .iter()
                                .cloned()
                                .map(|value| value.powf(2.0))
                                .sum::<f32>()
                                .sqrt();

                            assert!(
                                dot.is_finite(),
                                "The dot product is not finite! dot: {}",
                                dot
                            );

                            let loss = (if dot > 10.0 || dot < -10.0 {
                                0.0
                            } else {
                                let exp_dot = dot.exp();
                                label - exp_dot / (exp_dot + 1.0).powf(2.0)
                            } + hidden_embedding_l2
                                + context_embedding_l2)
                                * learning_rate;

                            assert!(
                                loss.is_finite(),
                                "The loss is not finite! loss: {}, dot: {}",
                                loss,
                                dot
                            );

                            // We compute the average loss to update the central gradient by the total central embedding.
                            let mean_loss = (loss / context_size) as f32;

                            // We convert the loss to f32, now that we are done using it as f64.
                            let loss = loss as f32;

                            // We update the gradients of the current node ID using the loss.
                            node_gradient
                                .iter_mut()
                                .zip(total_context_embedding.iter())
                                .for_each(|(feature_gradient, total_context_feature)| {
                                    // Note that we are setting this gradient EQUAL to the
                                    // contextual node feature.
                                    *feature_gradient = *total_context_feature * mean_loss;
                                });

                            context_gradient
                                .iter_mut()
                                .zip(hidden_embedding.iter())
                                .for_each(|(feature_gradient, current_node_feature)| {
                                    // Note that we are SUMMING the current node feature
                                    *feature_gradient += *current_node_feature * loss;
                                });
                        };

                    // We start to compute the new gradients.
                    random_walks
                        .par_chunks(random_walk_length)
                        .zip(non_central_terms.par_chunks(
                            number_of_central_terms_in_walk * self.number_of_negative_samples,
                        ))
                        .zip(
                            central_terms_batch_gradient
                                .par_chunks_mut(
                                    number_of_central_terms_in_walk * self.embedding_size,
                                )
                                .zip(non_central_terms_batch_gradient.par_chunks_mut(
                                    number_of_central_terms_in_walk
                                        * self.number_of_negative_samples
                                        * self.embedding_size,
                                ))
                                .zip(contextual_terms_batch_gradient.par_chunks_mut(
                                    number_of_central_terms_in_walk * self.embedding_size,
                                )),
                        )
                        .for_each(
                            |(
                                (random_walk, non_central_terms),
                                (
                                    (central_terms_gradients, non_central_terms_gradients),
                                    contextual_terms_gradients,
                                ),
                            )| {
                                (self.window_size..random_walk_length - self.window_size)
                                    .map(|central_index| {
                                        (
                                            &random_walk
                                                [(central_index - self.window_size)..central_index],
                                            &random_walk[(central_index + 1)
                                                ..(central_index + self.window_size)],
                                                random_walk[central_index]
                                        )
                                    })
                                    .zip(non_central_terms.chunks(self.number_of_negative_samples))
                                    .zip(
                                        central_terms_gradients
                                            .chunks_mut(self.embedding_size)
                                            .zip(non_central_terms_gradients.chunks_mut(
                                                self.embedding_size
                                                    * self.number_of_negative_samples,
                                            ))
                                            .zip(
                                                contextual_terms_gradients
                                                    .chunks_mut(self.embedding_size),
                                            ),
                                    )
                                    .for_each(
                                        |(
                                            (
                                                (left_context, right_context, central_node_id),
                                                non_central_terms,
                                            ),
                                            (
                                                (central_term_gradient, non_central_term_gradients),
                                                contextual_terms_gradient,
                                            ),
                                        )| {
                                            // We reset the contextual terms gradient to zeros,
                                            // since we will add the deltas by summation.
                                            contextual_terms_gradient.iter_mut().for_each(
                                                |feature_gradient| {
                                                    *feature_gradient = 0.0;
                                                },
                                            );
                                            // We compute the total context embedding.
                                            // First, we assign to it the embedding of the first context.
                                            let mut total_context_embedding =
                                                get_node_embedding(left_context[0]).to_vec();
                                            // Then we sum over it the other values.
                                            left_context[1..]
                                                .iter()
                                                .chain(right_context.iter())
                                                .for_each(|contextual_node_id| {
                                                    get_node_embedding(*contextual_node_id)
                                                        .iter()
                                                        .zip(total_context_embedding.iter_mut())
                                                        .for_each(|(feature, total_feature)| {
                                                            *total_feature += *feature;
                                                        });
                                                });
                                            // We have now finished to compute the total context embedding.

                                            // We now compute the gradient relative to the positive
                                            compute_mini_batch_step(
                                                total_context_embedding.as_slice(),
                                                central_node_id,
                                                1.0,
                                                central_term_gradient,
                                                contextual_terms_gradient,
                                            );

                                            // We compute the gradients relative to the negative classes.
                                            non_central_terms
                                            .iter()
                                            .cloned()
                                            .zip(
                                                non_central_term_gradients
                                                    .chunks_mut(self.embedding_size),
                                            )
                                            .filter(|(non_central_node_id, _)| *non_central_node_id != central_node_id)
                                            .for_each(
                                                |(
                                                    non_central_node_id,
                                                    non_central_term_gradient,
                                                )| {
                                                    compute_mini_batch_step(
                                                        total_context_embedding.as_slice(),
                                                        non_central_node_id,
                                                        0.0,
                                                        non_central_term_gradient,
                                                        contextual_terms_gradient,
                                                    );
                                                },
                                            );
                                        },
                                    );
                            },
                        );
                }
                // We have now finished to compute the updated gradients.

                // Start to apply the computed gradients

                // Create the thread shared version of the hidden layer.
                let shared_hidden = ThreadDataRaceAware::new(&mut hidden);

                // Create the thread shared version of the embedding layer.
                let shared_embedding = ThreadDataRaceAware::new(embedding);

                // Create the closure to apply a gradient to a provided node's hidden
                let update_hidden = |node_id: NodeT, gradient: &[f32]| {
                    let node_id = node_id as usize;
                    unsafe {
                        (*shared_hidden.get())
                            [node_id * self.embedding_size..(node_id + 1) * self.embedding_size]
                            .iter_mut()
                            .zip(gradient.iter())
                            .for_each(|(hidden_feature, gradient_feature): (&mut f32, &f32)| {
                                *hidden_feature = (*hidden_feature + *gradient_feature)
                                    .max(-max_weights)
                                    .min(max_weights);
                            });
                    }
                };

                // Create the closure to apply a gradient to a provided node's embedding
                let update_embedding = |node_id: NodeT, gradient: &[f32]| {
                    let node_id = node_id as usize;
                    unsafe {
                        (*shared_embedding.get())
                            [node_id * self.embedding_size..(node_id + 1) * self.embedding_size]
                            .iter_mut()
                            .zip(gradient.iter())
                            .for_each(|(embedding_feature, gradient_feature): (&mut f32, &f32)| {
                                *embedding_feature = (*embedding_feature + *gradient_feature)
                                    .max(-max_weights)
                                    .min(max_weights);
                            });
                    }
                };

                // We start the thread pool
                (0..cpu_number).into_par_iter().for_each(|thread_id| {
                    random_walks
                        .chunks(random_walk_length)
                        .zip(non_central_terms.chunks(
                            number_of_central_terms_in_walk * self.number_of_negative_samples,
                        ))
                        .zip(
                            central_terms_batch_gradient
                                .chunks(number_of_central_terms_in_walk * self.embedding_size)
                                .zip(non_central_terms_batch_gradient.chunks(
                                    number_of_central_terms_in_walk
                                        * self.number_of_negative_samples
                                        * self.embedding_size,
                                ))
                                .zip(
                                    contextual_terms_batch_gradient.chunks(
                                        number_of_central_terms_in_walk * self.embedding_size,
                                    ),
                                ),
                        )
                        .for_each(
                            |(
                                (random_walk, non_central_terms_per_walk),
                                (
                                    (central_terms_gradients, non_central_terms_gradients),
                                    contextual_terms_gradients,
                                ),
                            )| {
                                (self.window_size..random_walk_length - self.window_size)
                                    .map(|central_index| {
                                        (
                                            &random_walk
                                                [(central_index - self.window_size)..central_index],
                                            &random_walk[(central_index + 1)
                                                ..(central_index + self.window_size)],
                                            random_walk[central_index],
                                        )
                                    })
                                    .zip(
                                        non_central_terms_per_walk
                                            .chunks(self.number_of_negative_samples),
                                    )
                                    .zip(
                                        central_terms_gradients
                                            .chunks(self.embedding_size)
                                            .zip(non_central_terms_gradients.chunks(
                                                self.embedding_size
                                                    * self.number_of_negative_samples,
                                            ))
                                            .zip(
                                                contextual_terms_gradients
                                                    .chunks(self.embedding_size),
                                            ),
                                    )
                                    .for_each(
                                        |(
                                            (
                                                (left_context, right_context, central_node_id),
                                                non_central_terms,
                                            ),
                                            ((central_term_gradient, non_central_terms_gradients), contextual_terms_gradient),
                                        )| {
                                            // Update the hidden layer for the current central node.
                                            if can_update(central_node_id, thread_id) {
                                                update_hidden(
                                                    central_node_id,
                                                    central_term_gradient,
                                                );
                                            }
                                            // Update the contexts.
                                            left_context
                                                .iter()
                                                .chain(right_context.iter())
                                                .cloned()
                                                .filter(|contextual_node_id| {
                                                    can_update(*contextual_node_id, thread_id)
                                                })
                                                .for_each(|contextual_node_id| {
                                                    update_embedding(
                                                        contextual_node_id,
                                                        contextual_terms_gradient,
                                                    );
                                                });
                                            // Update the non-central nodes.
                                            non_central_terms
                                            .iter()
                                            .cloned()
                                            .zip(non_central_terms_gradients.chunks(self.embedding_size))
                                            .filter(|(non_central_node_id, _)| {
                                                *non_central_node_id != central_node_id && can_update(*non_central_node_id, thread_id)
                                            })
                                            .for_each(|(non_central_node_id, non_central_node_gradient)| {
                                                update_hidden(non_central_node_id, non_central_node_gradient);
                                            });
                                        },
                                    )
                            },
                        );
                });
                // We recover the reference to the embedding.
                embedding = shared_embedding.into_inner();
            }
        }
        Ok(())
    }
}
