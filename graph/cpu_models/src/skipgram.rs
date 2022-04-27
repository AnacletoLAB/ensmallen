use graph::{Graph, NodeT, ThreadDataRaceAware, WalksParameters};
use indicatif::{ProgressBar, ProgressIterator, ProgressStyle};
use rayon::iter::IndexedParallelIterator;
use rayon::iter::IntoParallelIterator;
use rayon::iter::IntoParallelRefMutIterator;
use rayon::iter::ParallelIterator;
use vec_rand::{random_f32, splitmix64};

#[derive(Clone, Debug)]
pub struct SkipGram {
    embedding_size: usize,
    window_size: usize,
    walk_parameters: WalksParameters,
    clipping_value: f32,
    number_of_negative_samples: usize,
    log_sigmoid: bool,
    siamese: bool,
}

impl SkipGram {
    /// Return new instance of SkipGram model.
    ///
    /// # Arguments
    /// `embedding_size`: Option<usize> - Size of the embedding.
    /// `walk_parameters`: Option<WalksParameters> - Parameters to be used within the walks.
    /// `window_size`: Option<usize> - Window size defining the contexts.
    /// `clipping_value`: Option<f32> - Value at which we clip the dot product, mostly for numerical stability issues. By default, `6.0`, where the loss is already close to zero.
    /// `number_of_negative_samples`: Option<usize> - Number of negative samples to extract for each context.
    /// `log_sigmoid: Option<bool> - Whether to use the model using a sigmoid or log sigmoid. By default, log sigmoid.
    /// `siamese: Option<bool> - Whether to use the model in Siamese mode, using half the weights.
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
        let walk_parameters = walk_parameters.unwrap_or_else(|| WalksParameters::default());
        let clipping_value = clipping_value.unwrap_or(6.0);
        let number_of_negative_samples = number_of_negative_samples.unwrap_or(5);
        let log_sigmoid = log_sigmoid.unwrap_or(true);
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

    /// Computes in the provided slice of embedding the SkipGram node embedding.
    ///
    /// # Implementative details
    /// This implementation is NOT thread safe, that is, different threads may try
    /// to overwrite each others memory.
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
        verbose: Option<bool>,
    ) -> Result<(), String> {
        let epochs = epochs.unwrap_or(10);
        let scale_factor = (self.embedding_size as f32).sqrt();

        let mut walk_parameters = self.walk_parameters.clone();
        let mut random_state = splitmix64(self.walk_parameters.get_random_state() as u64);
        let random_walk_length = walk_parameters.get_random_walk_length() as usize;
        let verbose = verbose.unwrap_or(true);
        let learning_rate = learning_rate.unwrap_or(0.005);

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
            (0..embedding.len())
                .into_par_iter()
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
                "SkipGram Epochs {spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] ({pos}/{len}, ETA {eta})",
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
                    &mut (*shared_embedding.get())
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
                                       label: f32| {
            let node_hidden = get_node_hidden(node_id);
            let dot = node_hidden
                .iter()
                .copied()
                .zip(total_context_embedding.iter().copied())
                .map(|(node_feature, contextual_feature)| node_feature * contextual_feature)
                .sum::<f32>()
                / scale_factor;

            if dot > self.clipping_value || dot < -self.clipping_value {
                return;
            }

            let exp_dot = dot.exp();
            let loss = (label
                - exp_dot
                    / if self.log_sigmoid {
                        exp_dot + 1.0
                    } else {
                        (exp_dot + 1.0).powf(2.0)
                    })
                * learning_rate;

            update_hidden(node_id, total_context_embedding, loss);
            weighted_vector_sum(context_embedding_gradient, node_hidden, loss);
        };

        // We start to loop over the required amount of epochs.
        for _ in (0..epochs).progress_with(epochs_progress_bar) {
            // We update the random state used to generate the random walks
            // and the negative samples.
            random_state = splitmix64(random_state);
            walk_parameters = walk_parameters.set_random_state(Some(random_state as usize));

            // We start to compute the new gradients.
            graph
                .par_iter_complete_walks(&walk_parameters)?
                .enumerate()
                .for_each(|(walk_number, random_walk)| {
                    (self.window_size..random_walk_length - self.window_size)
                        .map(|central_index| {
                            (
                                &random_walk[central_index - self.window_size
                                    ..central_index + self.window_size],
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
                                    let mut context_gradient = vec![0.0; self.embedding_size];
                                    let context_node_embedding =
                                        get_node_embedding(context_node_id);
                                    // We now compute the gradient relative to the positive
                                    compute_mini_batch_step(
                                        &context_node_embedding,
                                        context_gradient.as_mut_slice(),
                                        central_node_id,
                                        1.0,
                                    );

                                    // We compute the gradients relative to the negative classes.
                                    graph
                                        .iter_random_source_node_ids(
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
                                                0.0,
                                            );
                                        });
                                    update_embedding(context_node_id, &context_gradient);
                                });
                        });
                });
        }
        Ok(())
    }
}
