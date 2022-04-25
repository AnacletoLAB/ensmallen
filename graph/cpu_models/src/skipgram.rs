use graph::{Graph, NodeT, ThreadDataRaceAware, WalksParameters};
use indicatif::{ProgressBar, ProgressIterator, ProgressStyle};
use rayon::iter::IndexedParallelIterator;
use rayon::iter::IntoParallelRefMutIterator;
use rayon::iter::ParallelIterator;
use vec_rand::{random_f32, splitmix64};

pub struct SkipGram {
    embedding_size: usize,
    window_size: usize,
    walk_parameters: WalksParameters,
    number_of_negative_samples: usize,
}

impl SkipGram {
    /// Return new instance of SkipGram model.
    pub fn new(
        embedding_size: Option<usize>,
        walk_parameters: Option<WalksParameters>,
        window_size: Option<usize>,
        number_of_negative_samples: Option<usize>,
    ) -> Result<Self, String> {
        // Handle the values of the default parameters.
        let embedding_size = embedding_size.unwrap_or(100);
        let window_size = window_size.unwrap_or(10);
        let walk_parameters = walk_parameters.unwrap_or_else(|| WalksParameters::default());
        let number_of_negative_samples = number_of_negative_samples.unwrap_or(5);

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
        })
    }

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
        let context_size = (self.window_size * 2) as f32;
        let learning_rate = learning_rate.unwrap_or(0.001);

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

        let shared_embedding = ThreadDataRaceAware::new(embedding);

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
            vector.iter_mut().zip(variation.iter().cloned()).for_each(
                |(feature, gradient_feature): (&mut f32, f32)| {
                    *feature += weight * gradient_feature;
                },
            );
        };

        // Create the closure to apply a gradient to a provided node's embedding
        let update_embedding = |node_id: NodeT, variation: &[f32], weight: f32| {
            let node_id = node_id as usize;
            unsafe {
                weighted_vector_sum(
                    &mut (*shared_embedding.get())
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
                &(*shared_embedding.get())
                    [(node_id * self.embedding_size)..((node_id + 1) * self.embedding_size)]
            }
        };

        let compute_mini_batch_step = |total_context_embedding: &[f32],
                                       context_embedding_gradient: &mut [f32],
                                       node_id: NodeT,
                                       label: f32| {
            let dot = get_node_embedding(node_id)
                .iter()
                .cloned()
                .zip(total_context_embedding.iter().cloned())
                .map(|(node_feature, contextual_feature)| node_feature * contextual_feature)
                .sum::<f32>()
                / context_size
                / scale_factor;

            if dot > 20.0 || dot < -20.0 {
                return;
            }

            let exp_dot = dot.exp();
            let loss = (label - exp_dot / (exp_dot + 1.0).powf(2.0)) * learning_rate;

            update_embedding(node_id, total_context_embedding, loss / context_size);
            weighted_vector_sum(
                context_embedding_gradient,
                get_node_embedding(node_id),
                loss,
            );
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
                                &random_walk[(central_index - self.window_size)..central_index],
                                &random_walk
                                    [(central_index + 1)..(central_index + self.window_size)],
                                random_walk[central_index],
                                central_index,
                            )
                        })
                        .for_each(
                            |(left_context, right_context, central_node_id, central_index)| {
                                left_context
                                    .iter()
                                    .chain(right_context.iter())
                                    .cloned()
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
                                            .filter(|non_central_node_id| {
                                                *non_central_node_id != central_node_id
                                            })
                                            .for_each(|non_central_node_id| {
                                                compute_mini_batch_step(
                                                    &context_node_embedding,
                                                    context_gradient.as_mut_slice(),
                                                    non_central_node_id,
                                                    0.0,
                                                );
                                            });
                                        update_embedding(
                                            context_node_id,
                                            &context_gradient,
                                            1.0,
                                        );
                                    });
                            },
                        );
                });
        }
        Ok(())
    }
}
