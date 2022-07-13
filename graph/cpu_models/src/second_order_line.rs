use crate::{get_random_vector, populate_vectors};
use express_measures::dot_product_sequential_unchecked;
use graph::{Graph, NodeT, ThreadDataRaceAware};
use indicatif::{ProgressBar, ProgressStyle};
use rayon::prelude::*;
use vec_rand::splitmix64;

#[derive(Clone, Debug)]
pub struct SecondOrderLINE {
    embedding_size: usize,
    random_state: u64,
}

impl SecondOrderLINE {
    /// Return new instance of second-order LINE model.
    ///
    /// # Arguments
    /// `embedding_size`: Option<usize> - Size of the embedding.
    /// `random_state`: Option<u64> - The random state to use to reproduce the training.
    pub fn new(embedding_size: Option<usize>, random_state: Option<u64>) -> Result<Self, String> {
        // Handle the values of the default parameters.
        let embedding_size = embedding_size.unwrap_or(100);
        let random_state = random_state.unwrap_or(42);

        // Validate that the provided parameters are within
        // reasonable bounds.
        if embedding_size == 0 {
            return Err(concat!("The embedding size cannot be equal to zero.").to_string());
        }

        Ok(Self {
            embedding_size,
            random_state,
        })
    }

    /// Returns the used embedding size.
    pub fn get_embedding_size(&self) -> usize {
        self.embedding_size
    }

    /// Computes in the provided slice of embedding the second-order LINE node embedding.
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
    ///
    /// # Raises
    /// * If graph does not have node types and node types should be used.
    /// * If graph contains unknown node types and node types should be used.
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

        let norm = |vector: &[f32]| {
            (vector
                .iter()
                .map(|value| value.powf(2.0))
                .sum::<f32>()
                .sqrt()
                + f32::EPSILON)
                .min(f32::MAX)
        };

        let compute_prior = |subset_size: f32, total_size: f32| {
            (1.0 + subset_size)
                    / total_size
                    // Adding the epsilon is necessary because the division may destroy enough
                    // resolution to make the prior equal to zero.
                    + f32::EPSILON
        };

        // Populate the embedding layers with random uniform value
        populate_vectors(&mut [node_embedding], random_state, scale_factor);
        random_state = splitmix64(random_state);
        let mut hidden = get_random_vector(node_embedding.len(), random_state, scale_factor);
        random_state = splitmix64(random_state);

        node_embedding
            .par_chunks_mut(self.embedding_size)
            .for_each(|chunk| {
                let chunk_norm = norm(chunk);
                chunk.iter_mut().for_each(|value| {
                    *value /= chunk_norm;
                });
            });

        hidden
            .par_chunks_mut(self.embedding_size)
            .for_each(|chunk| {
                let chunk_norm = norm(chunk);
                chunk.iter_mut().for_each(|value| {
                    *value /= chunk_norm;
                });
            });

        let mut hidden_ref = hidden.as_mut_slice();
        let shared_hidden = ThreadDataRaceAware::new(&mut hidden_ref);
        let shared_node_embedding = ThreadDataRaceAware::new(node_embedding);

        // Depending whether verbosity was requested by the user
        // we create or not a visible progress bar to show the progress
        // in the training epochs.
        let pb = if verbose {
            let pb = ProgressBar::new(epochs as u64);
            pb.set_style(ProgressStyle::default_bar().template(
                "Second-order LINE {msg} {spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] ({pos}/{len}, ETA {eta})",
            ));
            pb
        } else {
            ProgressBar::hidden()
        };

        let compute_mini_batch_step = |src: usize, dst: usize, label: bool, learning_rate: f32| {
            let src_embedding = unsafe {
                &mut (*shared_node_embedding.get())
                    [(src * self.embedding_size)..((src + 1) * self.embedding_size)]
            };
            let dst_embedding = unsafe {
                &mut (*shared_hidden.get())
                    [(dst * self.embedding_size)..((dst + 1) * self.embedding_size)]
            };

            let src_norm = norm(src_embedding);
            let dst_norm = norm(dst_embedding);

            let dot = unsafe { dot_product_sequential_unchecked(src_embedding, dst_embedding) }
                / (dst_norm * src_norm * scale_factor);

            if dot > 6.0 || dot < -6.0 {
                return 0.0;
            }

            let prediction = 1.0 / (1.0 + (-dot).exp());

            let variation = if label { prediction - 1.0 } else { prediction } * learning_rate;

            let src_prior = compute_prior(
                unsafe { graph.get_unchecked_node_degree_from_node_id(src as NodeT) as f32 },
                nodes_number as f32,
            );
            let dst_prior = compute_prior(
                unsafe { graph.get_unchecked_node_degree_from_node_id(dst as NodeT) as f32 },
                nodes_number as f32,
            );

            let src_variation = variation / src_prior;
            let dst_variation = variation / dst_prior;

            src_embedding
                .iter_mut()
                .zip(dst_embedding.iter_mut())
                .for_each(|(src_feature, dst_feature)| {
                    *src_feature /= src_norm;
                    *dst_feature /= dst_norm;
                    *src_feature -= *dst_feature * src_variation;
                    *dst_feature -= *src_feature * dst_variation;
                });

            variation.abs()
        };

        // We start to loop over the required amount of epochs.
        for _ in 0..epochs {
            // We update the random state used to generate the random walks
            // and the negative samples.
            random_state = splitmix64(random_state);
            // We iterate over the graph edges.
            let total_variation = graph
                .par_iter_edge_prediction_mini_batch(
                    random_state,
                    graph.get_number_of_directed_edges() as usize,
                    false,
                    Some(0.5),
                    Some(true),
                    None,
                    Some(true),
                    None,
                    None,
                )?
                .map(|(src, dst, label)| {
                    compute_mini_batch_step(src as usize, dst as usize, label, learning_rate)
                })
                .sum::<f32>();

            pb.inc(1);
            pb.set_message(format!(", variation: {:.4}", total_variation));
            learning_rate *= learning_rate_decay;
        }
        Ok(())
    }
}
