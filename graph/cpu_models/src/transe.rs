use graph::{Graph, ThreadDataRaceAware};
use indicatif::ProgressIterator;
use indicatif::{ProgressBar, ProgressStyle};
use rayon::prelude::*;
use vec_rand::{random_f32, splitmix64};

#[derive(Clone, Debug)]
pub struct TransE {
    embedding_size: usize,
    renormalize: bool,
    relu_bias: f32,
    random_state: u64,
}

impl TransE {
    /// Return new instance of TransE model.
    ///
    /// # Arguments
    /// `embedding_size`: Option<usize> - Size of the embedding.
    /// `renormalize`: Option<bool> - Whether to renormalize at each loop, by default true.
    /// `relu_bias`: Option<f32> - The bias to apply to the relu. By default, 1.0.
    /// `random_state`: Option<u64> - The random state to use to reproduce the training.
    pub fn new(
        embedding_size: Option<usize>,
        renormalize: Option<bool>,
        relu_bias: Option<f32>,
        random_state: Option<u64>,
    ) -> Result<Self, String> {
        // Handle the values of the default parameters.
        let embedding_size = embedding_size.unwrap_or(100);
        let renormalize = renormalize.unwrap_or(true);
        let relu_bias = relu_bias.unwrap_or(1.0);
        let random_state = random_state.unwrap_or(42);

        // Validate that the provided parameters are within
        // reasonable bounds.
        if embedding_size == 0 {
            return Err(concat!("The embedding size cannot be equal to zero.").to_string());
        }

        Ok(Self {
            embedding_size,
            renormalize,
            relu_bias,
            random_state,
        })
    }

    /// Returns the used embedding size.
    pub fn get_embedding_size(&self) -> usize {
        self.embedding_size
    }

    /// Computes in the provided slice of embedding the TransE node and edge type embedding.
    ///
    /// # Implementative details
    /// This implementation is NOT thread safe, that is, different threads may try
    /// to overwrite each others memory. This version is faster than the memory safe
    /// version and requires less memory. In most use cases, you would prefer to use
    /// this version over the memory safe version.
    ///
    /// # Arguments
    /// `graph`: &Graph - The graph to embed
    /// `node_embedding`: &mut [f32] - The memory area where to write the node embedding.
    /// `edge_type_embedding`: &mut [f32] - The optional memory area where to write the edge type embedding.
    /// `epochs`: Option<usize> - The number of epochs to run the model for, by default 10.
    /// `learning_rate`: Option<f32> - The learning rate to update the gradient, by default 0.01.
    /// `learning_rate_decay`: Option<f32> - Factor to reduce the learning rate for at each epoch. By default 0.9.
    /// `verbose`: Option<bool> - Whether to show the loading bar, by default true.
    ///
    /// # Raises
    /// * If graph does not have node types and node types should be used.
    /// * If graph contains unknown node types and node types should be used.
    /// * If graph does not have edge types and edge types should be used.
    /// * If graph contains unknown edge types and edge types should be used.
    pub fn fit_transform(
        &self,
        graph: &Graph,
        node_embedding: &mut [f32],
        edge_type_embedding: &mut [f32],
        epochs: Option<usize>,
        learning_rate: Option<f32>,
        learning_rate_decay: Option<f32>,
        verbose: Option<bool>,
    ) -> Result<(), String> {
        let epochs = epochs.unwrap_or(10);
        let verbose = verbose.unwrap_or(true);
        let scale_factor = (self.embedding_size as f32).sqrt();
        let mut learning_rate = learning_rate.unwrap_or(0.001) / scale_factor;
        let learning_rate_decay = learning_rate_decay.unwrap_or(0.9);
        let mut random_state = splitmix64(self.random_state);

        if !graph.has_edge_types() {
            return Err(concat!(
                "The edge types should be used, but the provided ",
                "graph does not contain edge types."
            )
            .to_string());
        }

        if graph.has_unknown_edge_types().unwrap() {
            return Err(concat!(
                "The edge types should be used, but the provided ",
                "graph contains unknown edge types and it is not ",
                "well-defined how to use them."
            )
            .to_string());
        }

        if graph.has_homogeneous_edge_types().unwrap() {
            return Err(concat!(
                "The edge types should be used, but the provided ",
                "graph contains exclusively a single edge type ",
                "making using edge types useless."
            )
            .to_string());
        }

        let expected_edge_embedding_size =
            self.embedding_size * graph.get_edge_types_number().unwrap() as usize;

        if edge_type_embedding.len() != expected_edge_embedding_size {
            return Err(format!(
                "The given memory allocation for the edge type embeddings is {} long but we expect {}.",
                edge_type_embedding.len(),
                expected_edge_embedding_size
            ));
        }

        if !graph.has_nodes() {
            return Err("The provided graph does not have any node.".to_string());
        }

        let nodes_number = graph.get_nodes_number();
        let expected_node_embedding_size = self.embedding_size * nodes_number as usize;

        if node_embedding.len() != expected_node_embedding_size {
            return Err(format!(
                "The given memory allocation for the embeddings is {} long but we expect {}.",
                node_embedding.len(),
                expected_node_embedding_size
            ));
        }

        let initialization_radius = 6.0 / scale_factor;

        let norm = |vector: &[f32]| {
            vector
                .iter()
                .map(|value| value.powf(2.0))
                .sum::<f32>()
                .sqrt()
                + f32::EPSILON
        };

        // Populate the embedding layers with random uniform value
        node_embedding
            .par_iter_mut()
            .enumerate()
            .for_each(|(i, e)| {
                *e = 2.0 * initialization_radius * random_f32(splitmix64(random_state + i as u64))
                    - initialization_radius
            });

        node_embedding
            .par_chunks_mut(self.embedding_size)
            .for_each(|chunk| {
                let chunk_norm = norm(chunk);
                chunk.iter_mut().for_each(|value| {
                    *value /= chunk_norm;
                });
            });

        random_state = splitmix64(random_state);

        edge_type_embedding
            .par_iter_mut()
            .enumerate()
            .for_each(|(i, e)| {
                *e = 2.0 * initialization_radius * random_f32(splitmix64(random_state + i as u64))
                    - initialization_radius
            });

        edge_type_embedding
            .par_chunks_mut(self.embedding_size)
            .for_each(|chunk| {
                let chunk_norm = norm(chunk);
                chunk.iter_mut().for_each(|value| {
                    *value /= chunk_norm;
                });
            });

        let shared_node_embedding = ThreadDataRaceAware::new(node_embedding);
        let shared_edge_type_embedding = ThreadDataRaceAware::new(edge_type_embedding);

        // Depending whether verbosity was requested by the user
        // we create or not a visible progress bar to show the progress
        // in the training epochs.
        let epochs_progress_bar = if verbose {
            let pb = ProgressBar::new(epochs as u64);
            pb.set_style(ProgressStyle::default_bar().template(
                "TransE Epochs {msg} {spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] ({pos}/{len}, ETA {eta})",
            ));
            pb
        } else {
            ProgressBar::hidden()
        };

        let compute_mini_batch_step = |src: usize,
                                       not_src: usize,
                                       dst: usize,
                                       not_dst: usize,
                                       edge_type: usize,
                                       learning_rate: f32| {
            let src_embedding = unsafe {
                &mut (*shared_node_embedding.get())
                    [(src * self.embedding_size)..((src + 1) * self.embedding_size)]
            };
            let not_src_embedding = unsafe {
                &mut (*shared_node_embedding.get())
                    [(not_src * self.embedding_size)..((not_src + 1) * self.embedding_size)]
            };
            let dst_embedding = unsafe {
                &mut (*shared_node_embedding.get())
                    [(dst * self.embedding_size)..((dst + 1) * self.embedding_size)]
            };
            let not_dst_embedding = unsafe {
                &mut (*shared_node_embedding.get())
                    [(not_dst * self.embedding_size)..((not_dst + 1) * self.embedding_size)]
            };
            let edge_type_embedding = unsafe {
                &mut (*shared_edge_type_embedding.get())
                    [(edge_type * self.embedding_size)..((edge_type + 1) * self.embedding_size)]
            };

            let (dst_norm, not_dst_norm, src_norm, not_src_norm) = if self.renormalize {
                (
                    norm(dst_embedding),
                    norm(not_dst_embedding),
                    norm(src_embedding),
                    norm(not_src_embedding),
                )
            } else {
                (1.0, 1.0, 1.0, 1.0)
            };

            src_embedding
                .iter_mut()
                .zip(not_src_embedding.iter_mut())
                .zip(dst_embedding.iter_mut().zip(not_dst_embedding.iter_mut()))
                .zip(edge_type_embedding.iter_mut())
                .for_each(
                    |(
                        ((src_feature, not_src_feature), (dst_feature, not_dst_feature)),
                        edge_type_feature,
                    )| {
                        if self.renormalize {
                            *src_feature /= src_norm;
                            *not_src_feature /= not_src_norm;
                            *dst_feature /= dst_norm;
                            *not_dst_feature /= not_dst_norm;
                        }

                        let mut positive_distance =
                            *src_feature + *edge_type_feature - *dst_feature;
                        let mut negative_distance =
                            *not_src_feature + *edge_type_feature - *not_dst_feature;
                        let loss = positive_distance.powf(2.0) - negative_distance.powf(2.0);

                        if loss > -self.relu_bias {
                            positive_distance *= learning_rate;
                            negative_distance *= learning_rate;
                            *src_feature -= positive_distance;
                            *dst_feature += positive_distance;
                            *not_src_feature += negative_distance;
                            *not_dst_feature -= negative_distance;
                            *edge_type_feature -= positive_distance - negative_distance;
                        }
                    },
                );
        };

        // We start to loop over the required amount of epochs.
        (0..epochs)
            .progress_with(epochs_progress_bar)
            .for_each(|_| {
                // We update the random state used to generate the random walks
                // and the negative samples.
                random_state = splitmix64(random_state);

                // We iterate over the graph edges.
                graph
                    .par_iter_siamese_mini_batch(
                        random_state,
                        graph.get_number_of_directed_edges() as usize,
                        Some(true),
                    )
                    .for_each(|(src, _, dst, _, not_src, _, not_dst, _, edge_type_id)| {
                        compute_mini_batch_step(
                            src as usize,
                            not_src as usize,
                            dst as usize,
                            not_dst as usize,
                            edge_type_id.unwrap() as usize,
                            learning_rate,
                        );
                    });

                learning_rate *= learning_rate_decay;
            });
        Ok(())
    }
}
