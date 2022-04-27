use graph::{Graph, NodeT, ThreadDataRaceAware, WalksParameters};
use indicatif::{ProgressBar, ProgressIterator, ProgressStyle};
use rayon::iter::IndexedParallelIterator;
use rayon::iter::IntoParallelIterator;
use rayon::iter::IntoParallelRefMutIterator;
use rayon::iter::ParallelIterator;
use rayon::prelude::ParallelSlice;
use rayon::prelude::ParallelSliceMut;
use vec_rand::{random_f32, splitmix64};

#[derive(Clone, Debug)]
pub struct CBOW {
    embedding_size: usize,
    walk_parameters: WalksParameters,
    window_size: usize,
    clipping_value: f32,
    number_of_negative_samples: usize,
    log_sigmoid: bool,
    siamese: bool,
}

impl CBOW {
    /// Return new instance of CBOW model.
    ///
    /// # Arguments
    /// `embedding_size`: Option<usize> - Size of the embedding.
    /// `walk_parameters`: Option<WalksParameters> - Parameters to be used within the walks.
    /// `window_size`: Option<usize> - Window size defining the contexts.
    /// `clipping_value`: Option<f32> - Value at which we clip the dot product, mostly for numerical stability issues. By default, `6.0`, where the loss is already close to zero.
    /// `number_of_negative_samples`: Option<usize> - Number of negative samples to extract for each context.
    pub fn new(
        embedding_size: Option<usize>,
        walk_parameters: Option<WalksParameters>,
        window_size: Option<usize>,
        clipping_value: Option<f32>,
        number_of_negative_samples: Option<usize>,
        log_sigmoid: Option<bool>,
        siamese: Option<bool>,
    ) -> Result<Self, String> {
        // Handle the values of the default parameters.
        let embedding_size = embedding_size.unwrap_or(100);
        let window_size = window_size.unwrap_or(10);
        let clipping_value = clipping_value.unwrap_or(6.0);
        let walk_parameters = walk_parameters.unwrap_or_else(|| WalksParameters::default());
        let number_of_negative_samples = number_of_negative_samples.unwrap_or(5);
        let log_sigmoid = log_sigmoid.unwrap_or(false);
        let siamese = siamese.unwrap_or(false);

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
            clipping_value,
            number_of_negative_samples,
            log_sigmoid,
            siamese,
        })
    }

    /// Returns the used embedding size.
    pub fn get_embedding_size(&self) -> usize {
        self.embedding_size
    }

    /// Computes in the provided slice of embedding the CBOW node embedding.
    ///
    /// # Implementative details
    /// This implementation is thread safe, that is, there is no possibility
    /// for memory race in this version. Do note that this make this version a bit
    /// slower and requires more memory than the racing version.
    /// For most use cases, likely you would prefer to use the racing version.
    ///
    /// # Arguments
    /// `graph`: &Graph - The graph to embed
    /// `embedding`: &mut [f32] - The memory area where to write the embedding.
    /// `epochs`: Option<usize> - The number of epochs to run the model for, by default 10.
    /// `learning_rate`: Option<f32> - The learning rate to update the gradient, by default 0.005.
    /// `verbose`: Option<bool> - Whether to show the loading bar, by default true.
    pub fn fit_transform(
        &self,
        graph: &Graph,
        embedding: &mut [f32],
        epochs: Option<usize>,
        learning_rate: Option<f32>,
        batch_size: Option<usize>,
        verbose: Option<bool>,
    ) -> Result<(), String> {
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

        // if epochs == 0 {
        //     return Err("The number of epochs must be strictly greater than zero.".to_string());
        // }

        if !graph.has_nodes() {
            return Err("The provided graph does not have any node.".to_string());
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
        // let mut hidden = (0..expected_embedding_len)
        //     .into_par_iter()
        //     .map(|i| 2.0 * random_f32(splitmix64(random_state + i as u64)) - 1.0)
        //     .collect::<Vec<_>>();

        let mut hidden = if self.siamese {
            Vec::new()
        } else {
            (0..embedding.len()).into_par_iter()
                .map(|i| 2.0 * random_f32(splitmix64(random_state + i as u64)) - 1.0)
                .collect::<Vec<_>>()
        };

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
                    let not_mutable_embedding_ref = embedding.as_ref();

                    let hidden_ref = if self.siamese {
                        not_mutable_embedding_ref
                    } else {
                        &hidden
                    };

                    // We define a closure that returns a reference to the embedding of the given node.
                    let get_node_embedding = |node_id: NodeT| {
                        let node_id = node_id as usize;
                        &not_mutable_embedding_ref
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
                            let hidden_embedding = &hidden_ref[(node_id * self.embedding_size)
                                ..((node_id + 1) * self.embedding_size)];
                            // Within this computation, we also do a conversion to f64
                            // that we convert back to f32 afterwards. This is done because
                            // we want to avoid as much numerical instability as possible.
                            let dot = hidden_embedding
                                .iter()
                                .copied()
                                .zip(total_context_embedding.iter().copied())
                                .map(|(node_feature, contextual_feature)| {
                                    node_feature * contextual_feature
                                })
                                .sum::<f32>()
                                / context_size
                                / scale_factor;

                            if dot > self.clipping_value || dot < -self.clipping_value {
                                node_gradient.iter_mut().for_each(|feature_gradient| {
                                    // Note that we are setting this gradient EQUAL to the
                                    // contextual node feature.
                                    *feature_gradient = 0.0;
                                });
                            } else {
                                let exp_dot = dot.exp();
                                let loss =
                                    (label - exp_dot / if self.log_sigmoid {
                                        exp_dot + 1.0
                                    } else {
                                        (exp_dot + 1.0).powf(2.0)
                                    }) * learning_rate;
                                // We compute the average loss to update the central gradient by the total central embedding.
                                let mean_loss = (loss / context_size) as f32;

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
                                            .copied()
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
                let shared_hidden = ThreadDataRaceAware::new(hidden.as_mut_slice());

                // Create the thread shared version of the embedding layer.
                let shared_embedding = ThreadDataRaceAware::new(embedding.as_mut());
        
                let shared_embedding_ref = &shared_embedding;
                let shared_hidden_ref = if self.siamese {
                    &shared_embedding
                } else {
                    &shared_hidden
                };

                // Create the closure to apply a gradient to a provided node's hidden
                let update_hidden = |node_id: NodeT, gradient: &[f32]| {
                    let node_id = node_id as usize;
                    unsafe {
                        (*shared_hidden_ref.get())
                            [node_id * self.embedding_size..(node_id + 1) * self.embedding_size]
                            .iter_mut()
                            .zip(gradient.iter())
                            .for_each(|(hidden_feature, gradient_feature): (&mut f32, &f32)| {
                                *hidden_feature = *hidden_feature + *gradient_feature;
                            });
                    }
                };

                // Create the closure to apply a gradient to a provided node's embedding
                let update_embedding = |node_id: NodeT, gradient: &[f32]| {
                    let node_id = node_id as usize;
                    unsafe {
                        (*shared_embedding_ref.get())
                            [node_id * self.embedding_size..(node_id + 1) * self.embedding_size]
                            .iter_mut()
                            .zip(gradient.iter())
                            .for_each(|(embedding_feature, gradient_feature): (&mut f32, &f32)| {
                                *embedding_feature = *embedding_feature + *gradient_feature;
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
                                                .copied()
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
                                            .copied()
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
            }
        }
        Ok(())
    }

    /// Computes in the provided slice of embedding the CBOW node embedding.
    ///
    /// # Implementative details
    /// This implementation is NOT thread safe, that is, different threads may try
    /// to overwrite each others memory. This version is faster than the memory safe
    /// version and requires less memory. In most use cases, you would prefer to use
    /// this version over the memory safe version.
    ///
    /// # Arguments
    /// `graph`: &Graph - The graph to embed
    /// `embedding`: &mut [f32] - The memory area where to write the embedding.
    /// `epochs`: Option<usize> - The number of epochs to run the model for, by default 10.
    /// `learning_rate`: Option<f32> - The learning rate to update the gradient, by default 0.005.
    /// `verbose`: Option<bool> - Whether to show the loading bar, by default true.
    pub fn fit_transform_racing(
        &self,
        graph: &Graph,
        embedding: &mut [f32],
        epochs: Option<usize>,
        learning_rate: Option<f32>,
        verbose: Option<bool>,
    ) -> Result<(), String> {
        let epochs = epochs.unwrap_or(10);

        let mut walk_parameters = self.walk_parameters.clone();
        let mut random_state = splitmix64(self.walk_parameters.get_random_state() as u64);
        let random_walk_length = walk_parameters.get_random_walk_length() as usize;
        let verbose = verbose.unwrap_or(true);
        let context_size = (self.window_size * 2) as f32;
        let mut learning_rate = learning_rate.unwrap_or(
            if self.log_sigmoid {
                0.01
            } else {
                0.01
            }
        );

        // This is used to scale the dot product to avoid getting NaN due to
        // exp(dot) being inf and the sigmoid becomes Nan
        // we multiply by context size so we have a faster division when computing
        // the dotproduct of the mean contexted mebedding
        let scale_factor = context_size; //(self.embedding_size as f32).sqrt() * context_size;

        // if epochs == 0 {
        //     return Err("The number of epochs must be strictly greater than zero.".to_string());
        // }

        if !graph.has_nodes() {
            return Err("The provided graph does not have any node.".to_string());
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

        let mut hidden = if self.siamese {
            Vec::new()
        } else {
            (0..embedding.len()).into_par_iter()
                .map(|i| 2.0 * random_f32(splitmix64(random_state + i as u64)) - 1.0)
                .collect::<Vec<_>>()
        };

        let shared_hidden = ThreadDataRaceAware::new(hidden.as_mut_slice());
        let shared_embedding = ThreadDataRaceAware::new(embedding);

        let shared_embedding_ref = &shared_embedding;
        let shared_hidden_ref = if self.siamese {
            &shared_embedding
        } else {
            &shared_hidden
        };

        // Depending whether verbosity was requested by the user
        // we create or not a visible progress bar to show the progress
        // in the training epochs.
        let epochs_progress_bar = if verbose {
            let pb = ProgressBar::new(epochs as u64);
            pb.set_style(ProgressStyle::default_bar().template(
                "CBOW Epochs {msg} {spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] ({pos}/{len}, ETA {eta})",
            ));
            pb
        } else {
            ProgressBar::hidden()
        };

        // Create the closure to apply a gradient to a provided node's embedding
        let weighted_vector_sum = |vector: &mut [f32], variation: &[f32], weight: f32| {
            vector.iter_mut().zip(variation.iter().copied()).for_each(
                |(feature, gradient_feature): (&mut f32, f32)| {
                    *feature += weight * gradient_feature;
                },
            );
        };

        // Create the closure to apply a gradient to a provided node's embedding
        let vector_sum = |vector: &mut [f32], variation: &[f32]| {
            vector.iter_mut().zip(variation.iter().copied()).for_each(
                |(feature, gradient_feature): (&mut f32, f32)| {
                    *feature += gradient_feature;
                },
            );
        };

        // Create the closure to apply a gradient to a provided node's embedding
        let update_embedding = |node_id: NodeT, variation: &[f32]| {
            let node_id = node_id as usize;
            unsafe {
                vector_sum(
                    &mut (*shared_embedding_ref.get())
                        [node_id * self.embedding_size..(node_id + 1) * self.embedding_size],
                    variation,
                )
            }
        };

        // Create the closure to apply a gradient to a provided node's hidden layer weights
        let update_hidden = |node_id: NodeT, variation: &[f32], weight: f32| {
            let node_id = node_id as usize;
            unsafe {
                weighted_vector_sum(
                    &mut (*shared_hidden_ref.get())
                        [node_id * self.embedding_size..(node_id + 1) * self.embedding_size],
                    variation,
                    weight,
                )
            }
        };

        // We define a closure that returns a reference to the embedding of the given node.
        let get_node_embedding = |node_id: NodeT| {
            let node_id = node_id as usize;
            unsafe {
                &(*shared_embedding_ref.get())
                    [(node_id * self.embedding_size)..((node_id + 1) * self.embedding_size)]
            }
        };

        // We define a closure that returns a reference to the hidden of the given node.
        let get_node_hidden = |node_id: NodeT| {
            let node_id = node_id as usize;
            unsafe {
                &(*shared_hidden_ref.get())
                    [(node_id * self.embedding_size)..((node_id + 1) * self.embedding_size)]
            }
        };

        let compute_mini_batch_step = |total_context_embedding: &[f32],
                                       context_embedding_gradient: &mut [f32],
                                       node_id: NodeT,
                                       label: f32,
                                       learning_rate: f32| {
            let node_hidden = get_node_hidden(node_id);       
            let dot = node_hidden.iter()
                .copied()
                .zip(total_context_embedding.iter().copied())
                .map(|(node_feature, contextual_feature)| node_feature * contextual_feature)
                .sum::<f32>()
                / scale_factor;

            if dot > 20.0 || dot < -20.0 {
                return 0.0;
            }

            let exp_dot = dot.exp();
            let loss = (
                label - (
                    exp_dot / if self.log_sigmoid {
                        exp_dot + 1.0
                    } else {
                        (exp_dot + 1.0).powf(2.0)
                    }
                )) * learning_rate;

            weighted_vector_sum(
                context_embedding_gradient,
                node_hidden,
                loss,
            );
            update_hidden(node_id, total_context_embedding, loss / context_size);

            loss.abs() / learning_rate
        };

        // We start to loop over the required amount of epochs.
        for _ in 0..epochs {
            // We update the random state used to generate the random walks
            // and the negative samples.
            random_state = splitmix64(random_state);
            walk_parameters = walk_parameters.set_random_state(Some(random_state as usize));

            learning_rate = learning_rate * 0.9;

            // We start to compute the new gradients.
            let total_loss = graph
                .par_iter_complete_walks(&walk_parameters)?
                .enumerate()
                .map(|(walk_number, random_walk)| {
                    (self.window_size..random_walk_length - self.window_size)
                        .map(|central_index| {
                            (
                                &random_walk[(central_index - self.window_size)..central_index + self.window_size],
                                random_walk[central_index],
                                central_index,
                            )
                        })
                        .map(
                            |(context, central_node_id, central_index)| {
                                // We compute the total context embedding.
                                // First, we assign to it the embedding of the first context.
                                let mut total_context_embedding = vec![0.0; self.embedding_size];
                                
                                // Then we sum over it the other values.
                                for contextual_node_id in context.iter().copied() {
                                    if contextual_node_id == central_node_id {
                                        continue;
                                    }
                                    get_node_embedding(contextual_node_id)
                                        .iter()
                                        .zip(total_context_embedding.iter_mut())
                                        .for_each(|(feature, total_feature)| {
                                            *total_feature += *feature;
                                        });
                                }

                                let mut context_gradient = vec![0.0; self.embedding_size];

                                // We now compute the gradient relative to the positive
                                let positive_loss = compute_mini_batch_step(
                                    total_context_embedding.as_slice(),
                                    context_gradient.as_mut_slice(),
                                    central_node_id,
                                    1.0,
                                    learning_rate,
                                );

                                // We compute the gradients relative to the negative classes.
                               let negative_loss = graph
                                    .iter_random_source_node_ids(
                                        self.number_of_negative_samples,
                                        splitmix64(
                                            random_state
                                                + central_index as u64
                                                + walk_number as u64,
                                        ),
                                    )
                                        .filter(|non_central_node_id| {
                                            *non_central_node_id != central_node_id
                                        })
                                    .map(|non_central_node_id| {
                                        compute_mini_batch_step(
                                            total_context_embedding.as_slice(),
                                            context_gradient.as_mut_slice(),
                                            non_central_node_id,
                                            0.0,
                                            learning_rate,
                                        )
                                    }).sum::<f32>();

                                for contextual_node_id in context.iter().copied() {
                                    if contextual_node_id == central_node_id {
                                        continue
                                    }
                                    update_embedding(
                                        contextual_node_id,
                                        &context_gradient,
                                    );
                                }
                                positive_loss + negative_loss
                            },
                        ).sum::<f32>()
                }).sum::<f32>();
            epochs_progress_bar.inc(1);
            epochs_progress_bar.set_message(format!("{:.4}", total_loss));
        }
        Ok(())
    }
}
