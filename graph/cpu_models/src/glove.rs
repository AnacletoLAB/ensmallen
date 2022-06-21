use graph::{Graph, ThreadDataRaceAware, WalksParameters};
use indicatif::{ProgressBar, ProgressStyle};
use rayon::prelude::*;
use vec_rand::{random_f32, splitmix64};

#[derive(Clone, Debug)]
pub struct GloVe {
    embedding_size: usize,
    walk_parameters: WalksParameters,
    window_size: usize,
    clipping_value: f32,
    alpha: f32,
}

impl GloVe {
    /// Return new instance of GloVe model.
    ///
    /// # Arguments
    /// * `embedding_size`: Option<usize> - Size of the embedding.
    /// * `walk_parameters`: Option<WalksParameters> - Parameters to be used within the walks.
    /// * `window_size`: Option<usize> - Window size defining the contexts.
    /// * `clipping_value`: Option<f32> - Value at which we clip the dot product, mostly for numerical stability issues.
    /// * `alpha`: Option<f32> - Alpha to use for the loss. By default `0.75`.
    pub fn new(
        embedding_size: Option<usize>,
        walk_parameters: Option<WalksParameters>,
        window_size: Option<usize>,
        clipping_value: Option<f32>,
        alpha: Option<f32>,
    ) -> Result<Self, String> {
        // Handle the values of the default parameters.
        let embedding_size = embedding_size.unwrap_or(100);
        let window_size = window_size.unwrap_or(10);
        let clipping_value = clipping_value.unwrap_or(100.0);
        let alpha = alpha.unwrap_or(0.75);
        let walk_parameters = walk_parameters.unwrap_or_else(|| WalksParameters::default());

        // Validate that the provided parameters are within
        // reasonable bounds.
        if embedding_size == 0 {
            return Err(concat!("The embedding size cannot be equal to zero.").to_string());
        }
        if window_size == 0 {
            return Err(concat!("The window size cannot be equal to zero.").to_string());
        }

        Ok(Self {
            embedding_size,
            window_size,
            walk_parameters,
            clipping_value,
            alpha,
        })
    }

    /// Returns the used embedding size.
    pub fn get_embedding_size(&self) -> usize {
        self.embedding_size
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
        let mut walk_parameters = self.walk_parameters.clone();
        let mut random_state = splitmix64(self.walk_parameters.get_random_state() as u64);
        let verbose = verbose.unwrap_or(true);
        let mut learning_rate = learning_rate.unwrap_or(0.01);
        let learning_rate_decay = learning_rate_decay.unwrap_or(0.9);

        // This is used to scale the dot product to avoid getting NaN due to
        // exp(dot) being inf and the sigmoid becomes Nan
        // we multiply by context size so we have a faster division when computing
        // the dotproduct of the mean contexted mebedding
        let scale_factor = (self.embedding_size as f32).sqrt();

        if !graph.has_nodes() {
            return Err("The provided graph does not have any node.".to_string());
        }

        let nodes_number = graph.get_nodes_number();
        let log_squared_nodes_number = 2.0 * (nodes_number as f32).ln();
        let expected_node_embedding_len = self.embedding_size * nodes_number as usize;
        if node_embedding.len() != expected_node_embedding_len {
            return Err(format!(
                "The given memory allocation for the node embeddings is {} long but we expect {}.",
                node_embedding.len(),
                expected_node_embedding_len
            ));
        }

        // Populate the node embedding layer with random uniform value
        node_embedding
            .par_iter_mut()
            .enumerate()
            .for_each(|(i, e)| *e = 2.0 * random_f32(splitmix64(random_state + i as u64)) - 1.0);

        // Update the random state
        random_state = splitmix64(random_state);

        // Allocate and populate the hidden layer
        let hidden_layer = (0..node_embedding.len())
            .into_par_iter()
            .map(|i| 2.0 * random_f32(splitmix64(random_state + i as u64)) - 1.0)
            .collect::<Vec<f32>>();

        // Update the random state
        random_state = splitmix64(random_state);

        // Wrapping the layers into shared structures.
        let shared_node_embedding = ThreadDataRaceAware::new(node_embedding);
        let shared_hidden_layer = ThreadDataRaceAware::new(hidden_layer);

        // Depending whether verbosity was requested by the user
        // we create or not a visible progress bar to show the progress
        // in the training epochs.
        let epochs_progress_bar = if verbose {
            let pb = ProgressBar::new(epochs as u64);
            pb.set_style(ProgressStyle::default_bar().template(
                "GloVe {msg} {spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] ({pos}/{len}, ETA {eta})",
            ));
            pb
        } else {
            ProgressBar::hidden()
        };

        // We start to loop over the required amount of epochs.
        for _ in 0..epochs {
            // We update the random state used to generate the random walks
            // and the negative samples.
            random_state = splitmix64(random_state);
            walk_parameters = walk_parameters.set_random_state(Some(random_state as usize));

            // We start to compute the new gradients.
            let total_variation = graph
                .par_iter_cooccurence_matrix(
                    &walk_parameters,
                    self.window_size,
                )?
                .map(|(src, dst, freq)| unsafe {
                    let src_embedding = &mut (*shared_node_embedding.get())
                        [(src as usize) * self.embedding_size..((src as usize) + 1) * self.embedding_size];
                    let dst_hidden = &mut (*shared_hidden_layer.get())
                        [(dst as usize) * self.embedding_size..(dst as usize + 1) * self.embedding_size];

                    let dot = src_embedding
                        .iter()
                        .copied()
                        .zip(dst_hidden.iter().copied())
                        .map(|(src_feature, dst_feature)| src_feature * dst_feature)
                        .sum::<f32>()
                        / scale_factor;

                    if dot > self.clipping_value || dot < -self.clipping_value {
                        return 0.0;
                    }

                    let log_src_degree =
                        (graph.get_unchecked_node_degree_from_node_id(src) as f32).ln();
                    let log_dst_degree =
                        (graph.get_unchecked_node_degree_from_node_id(dst) as f32).ln();

                    let normalized_log_preferential_attachment =
                        log_src_degree + log_dst_degree - log_squared_nodes_number;

                    let loss: f32 = (2.0
                        * freq.powf(self.alpha)
                        * (dot + normalized_log_preferential_attachment - freq.ln())
                        * learning_rate) as f32;

                    src_embedding
                        .iter_mut()
                        .zip(dst_hidden.iter_mut())
                        .for_each(|(src_feature, dst_feature)| {
                            *src_feature -= *dst_feature * loss;
                            *dst_feature -= *src_feature * loss;
                        });
                    loss
                })
                .sum::<f32>();
            epochs_progress_bar.inc(1);
            epochs_progress_bar.set_message(format!("variation {:.4}", total_variation));
            learning_rate *= learning_rate_decay;
        }
        Ok(())
    }
}
