use crate::{get_random_vector, must_not_be_zero, BasicEmbeddingModel, GraphEmbedder};
use graph::{Graph, ThreadDataRaceAware, WalksParameters};
use num_traits::Zero;
use rayon::prelude::*;
use vec_rand::splitmix64;

#[derive(Clone, Debug)]
pub struct GloVe {
    walk_parameters: WalksParameters,
    window_size: usize,
    clipping_value: f32,
    alpha: f32,
    model: BasicEmbeddingModel,
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
    /// * `epochs`: Option<usize> - The number of epochs to run the model for, by default 10.
    /// * `learning_rate`: Option<f32> - The learning rate to update the gradient, by default 0.01.
    /// * `learning_rate_decay`: Option<f32> - Factor to reduce the learning rate for at each epoch. By default 0.9.
    /// * `random_state`: Option<u64> - The random state to use to reproduce the training.
    /// * `verbose`: Option<bool> - Whether to show loading bar.
    pub fn new(
        embedding_size: Option<usize>,
        walk_parameters: Option<WalksParameters>,
        window_size: Option<usize>,
        clipping_value: Option<f32>,
        alpha: Option<f32>,
        epochs: Option<usize>,
        learning_rate: Option<f32>,
        learning_rate_decay: Option<f32>,
        random_state: Option<u64>,
        verbose: Option<bool>,
    ) -> Result<Self, String> {
        // Handle the values of the default parameters.
        let clipping_value = clipping_value.unwrap_or(100.0);
        let alpha = alpha.unwrap_or(0.75);
        let walk_parameters = walk_parameters.unwrap_or_else(|| WalksParameters::default());

        // Validate that the provided parameters are within
        // reasonable bounds.
        let window_size = must_not_be_zero(window_size, 10, "window size")?;

        Ok(Self {
            window_size,
            walk_parameters,
            clipping_value,
            alpha,
            model: BasicEmbeddingModel::new(
                embedding_size,
                epochs,
                learning_rate,
                learning_rate_decay,
                random_state,
                verbose,
            )?,
        })
    }
}

impl GraphEmbedder for GloVe {
    fn get_model_name(&self) -> String {
        "GloVE".to_string()
    }

    fn get_number_of_epochs(&self) -> usize {
        self.model.epochs
    }

    fn is_verbose(&self) -> bool {
        self.model.verbose
    }

    fn get_random_state(&self) -> u64 {
        self.model.random_state
    }

    fn get_embedding_shapes(&self, graph: &Graph) -> Result<Vec<(usize, usize)>, String> {
        Ok(vec![(
            graph.get_number_of_nodes() as usize,
            self.model.embedding_size,
        )])
    }

    fn _fit_transform(&self, graph: &Graph, embedding: &mut [&mut [f32]]) -> Result<(), String> {
        let embedding_size = self.model.get_embedding_size();
        let mut walk_parameters = self.walk_parameters.clone();
        let mut random_state = splitmix64(self.walk_parameters.get_random_state() as u64);
        let mut learning_rate = self.model.get_learning_rate();
        // This is used to scale the dot product to avoid getting NaN due to
        // exp(dot) being inf and the sigmoid becomes Nan
        // we multiply by context size so we have a faster division when computing
        // the dotproduct of the mean contexted mebedding
        let scale_factor = (embedding_size as f32).sqrt();

        // Allocate and populate the hidden layer
        let mut hidden_layer = get_random_vector(embedding[0].len(), random_state, scale_factor);

        // Update the random state
        random_state = splitmix64(random_state);

        // Wrapping the layers into shared structures.
        let shared_embedding = ThreadDataRaceAware::new(&mut embedding[0]);
        let shared_hidden_layer = ThreadDataRaceAware::new(hidden_layer.as_mut_slice());

        let pb = self.get_loading_bar();

        // We start to loop over the required amount of epochs.
        for _ in 0..self.get_number_of_epochs() {
            // We update the random state used to generate the random walks
            // and the negative samples.
            random_state = splitmix64(random_state);
            walk_parameters = walk_parameters.set_random_state(Some(random_state as usize));

            // We start to compute the new gradients.
            let total_variation = graph
                .par_iter_log_normalized_cooccurence_matrix(
                    &walk_parameters,
                    self.window_size,
                    None,
                )?
                .map(|(src, dst, freq)| unsafe {
                    let src_embedding = &mut (*shared_embedding.get())
                        [(src as usize) * embedding_size..((src as usize) + 1) * embedding_size];
                    let dst_hidden = &mut (*shared_hidden_layer.get())
                        [(dst as usize) * embedding_size..(dst as usize + 1) * embedding_size];

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

                    let loss: f32 =
                        (2.0 * freq.powf(self.alpha) * (dot - freq.ln()) * learning_rate) as f32;

                    src_embedding
                        .iter_mut()
                        .zip(dst_hidden.iter_mut())
                        .for_each(|(src_feature, dst_feature)| {
                            *src_feature -= *dst_feature * loss;
                            *dst_feature -= *src_feature * loss;
                        });
                    loss.abs()
                })
                .sum::<f32>();

            if total_variation.is_zero() {
                break;
            }

            pb.inc(1);
            pb.set_message(format!("variation {:.4}", total_variation));
            learning_rate *= self.model.get_learning_rate_decay();
        }
        Ok(())
    }
}
