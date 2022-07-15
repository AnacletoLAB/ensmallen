use crate::*;
use graph::{Graph, NodeT, ThreadDataRaceAware};
use indicatif::ProgressIterator;
use indicatif::{ProgressBar, ProgressStyle};
use rayon::prelude::*;
use vec_rand::splitmix64;

#[derive(Clone, Debug)]
pub struct Unstructured {
    embedding_size: usize,
    relu_bias: f32,
    random_state: u64,
}

impl Unstructured {
    /// Return new instance of Unstructured model.
    ///
    /// # Arguments
    /// `embedding_size`: Option<usize> - Size of the embedding.
    /// `relu_bias`: Option<f32> - The bias to apply to the relu. By default, 1.0.
    /// `random_state`: Option<u64> - The random state to use to reproduce the training.
    pub fn new(
        embedding_size: Option<usize>,
        relu_bias: Option<f32>,
        random_state: Option<u64>,
    ) -> Result<Self, String> {
        // Handle the values of the default parameters.
        let relu_bias = relu_bias.unwrap_or(1.0);
        let random_state = random_state.unwrap_or(42);

        // Validate that the provided parameters are within
        // reasonable bounds.
        let embedding_size = must_not_be_zero(embedding_size, 100, "embedding size")?;

        Ok(Self {
            embedding_size,
            relu_bias,
            random_state,
        })
    }

    /// Returns the used embedding size.
    pub fn get_embedding_size(&self) -> usize {
        self.embedding_size
    }

    /// Computes in the provided slice of embedding the Unstructured node and edge type embedding.
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
    /// `epochs`: Option<usize> - The number of epochs to run the model for, by default 10.
    /// `learning_rate`: Option<f32> - The learning rate to update the gradient, by default 0.01.
    /// `learning_rate_decay`: Option<f32> - Factor to reduce the learning rate for at each epoch. By default 0.9.
    /// `verbose`: Option<bool> - Whether to show the loading bar, by default true.
    pub fn fit_transform(
        &self,
        graph: &Graph,
        node_embedding: &mut [f32],
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

        if !graph.has_nodes() {
            return Err("The provided graph does not have any node.".to_string());
        }

        let nodes_number = graph.get_number_of_nodes();
        let expected_node_embedding_size = self.embedding_size * nodes_number as usize;

        if node_embedding.len() != expected_node_embedding_size {
            return Err(format!(
                "The given memory allocation for the embeddings is {} long but we expect {}.",
                node_embedding.len(),
                expected_node_embedding_size
            ));
        }

        // Populate the embedding layers with random uniform value
        populate_vectors(&mut [node_embedding], random_state, scale_factor);
        random_state = splitmix64(random_state);
        let shared_node_embedding = ThreadDataRaceAware::new(node_embedding);

        // Depending whether verbosity was requested by the user
        // we create or not a visible progress bar to show the progress
        // in the training epochs.
        let epochs_progress_bar = if verbose {
            let pb = ProgressBar::new(epochs as u64);
            pb.set_style(ProgressStyle::default_bar().template(
                "Unstructured {msg} {spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] ({pos}/{len}, ETA {eta})",
            ));
            pb
        } else {
            ProgressBar::hidden()
        };

        let compute_mini_batch_step = |src: usize,
                                       not_src: usize,
                                       dst: usize,
                                       not_dst: usize,
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

            let (dst_norm, not_dst_norm, src_norm, not_src_norm) = (
                norm(dst_embedding),
                norm(not_dst_embedding),
                norm(src_embedding),
                norm(not_src_embedding),
            );
            let src_prior = compute_prior(
                unsafe { graph.get_unchecked_node_degree_from_node_id(src as NodeT) as f32 },
                nodes_number as f32,
            );
            let dst_prior = compute_prior(
                unsafe { graph.get_unchecked_node_degree_from_node_id(dst as NodeT) as f32 },
                nodes_number as f32,
            );
            let not_src_prior = compute_prior(
                unsafe { graph.get_unchecked_node_degree_from_node_id(not_src as NodeT) as f32 },
                nodes_number as f32,
            );
            let not_dst_prior = compute_prior(
                unsafe { graph.get_unchecked_node_degree_from_node_id(not_dst as NodeT) as f32 },
                nodes_number as f32,
            );

            src_embedding
                .iter_mut()
                .zip(not_src_embedding.iter_mut())
                .zip(dst_embedding.iter_mut().zip(not_dst_embedding.iter_mut()))
                .for_each(
                    |((src_feature, not_src_feature), (dst_feature, not_dst_feature))| {
                        *src_feature /= src_norm;
                        *not_src_feature /= not_src_norm;
                        *dst_feature /= dst_norm;
                        *not_dst_feature /= not_dst_norm;

                        let mut positive_distance = *src_feature - *dst_feature;
                        let mut negative_distance = *not_src_feature - *not_dst_feature;
                        let loss = positive_distance.powf(2.0) - negative_distance.powf(2.0);

                        if loss > -self.relu_bias {
                            positive_distance *= learning_rate;
                            negative_distance *= learning_rate;
                            *src_feature -= positive_distance / src_prior;
                            *dst_feature += positive_distance / dst_prior;
                            *not_src_feature += negative_distance / not_src_prior;
                            *not_dst_feature -= negative_distance / not_dst_prior;
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
                    )
                    .for_each(|(_, src, dst, not_src, not_dst)| {
                        compute_mini_batch_step(
                            src as usize,
                            not_src as usize,
                            dst as usize,
                            not_dst as usize,
                            learning_rate,
                        );
                    });

                learning_rate *= learning_rate_decay;
            });
        Ok(())
    }
}
