use graph::{Graph, NodeT, ThreadDataRaceAware, WalksParameters};
use indicatif::{ProgressBar, ProgressStyle};
use rayon::prelude::*;
use vec_rand::{random_f32, splitmix64};

#[derive(Clone, Debug)]
pub enum EdgeEmbeddingMethods {
    CosineSimilarity,
    L2Distance,
    Hadamard,
    SquaredL2,
}

#[derive(Clone, Debug)]
pub struct EdgePredictionPerceptron {
    /// The method to use to compute the edge embedding.
    edge_embedding_method: EdgeEmbeddingMethods,
    /// The weights of the model.
    weights: Vec<f32>,
    /// The bias of the model.
    bias: f32,
    /// The number of epochs to train the model for.
    number_of_epochs: usize,
    /// Number of samples in a mini-batch. By default 1024.
    number_of_edges_per_mini_batch: usize,
    /// The random state to reproduce the model initialization and training.
    random_state: u64,
}

impl EdgePredictionPerceptron {
    /// Return new instance of Perceptron for edge prediction.
    ///
    /// # Arguments
    /// * `edge_embedding_method`: Option<EdgeEmbeddingMethods> - The embedding method to use. By default the cosine similarity is used.
    /// * `number_of_epochs`: Option<usize> - The number of epochs to train the model for. By default, 100.
    /// * `number_of_edges_per_mini_batch`: Option<usize> - The number of samples to include for each mini-batch. By default 1024.
    /// * `random_state`: Option<u64> - The random state to reproduce the model initialization and training. By default, 42.
    pub fn new(
        edge_embedding_method: Option<EdgeEmbeddingMethods>,
        number_of_epochs: Option<usize>,
        number_of_edges_per_mini_batch: Option<usize>,
        random_state: Option<u64>,
    ) -> Result<Self, String> {
        let number_of_epochs = number_of_epochs.unwrap_or(100);
        if number_of_epochs == 0 {
            return Err(concat!(
                "The provided number of epochs is zero. ",
                "The number of epochs should be strictly greater than zero."
            )
            .to_string());
        }
        let number_of_edges_per_mini_batch = number_of_edges_per_mini_batch.unwrap_or(1024);
        if number_of_edges_per_mini_batch == 0 {
            return Err(concat!(
                "The provided number of edges per mini-batch is zero. ",
                "The number of edges per mini-batch should be strictly greater than zero."
            )
            .to_string());
        }
        Ok(Self {
            edge_embedding_method: edge_embedding_method
                .unwrap_or(EdgeEmbeddingMethods::CosineSimilarity),
            weights: Vec::new(),
            bias: 0.0,
            number_of_epochs,
            number_of_edges_per_mini_batch,
            random_state: random_state.unwrap_or(42),
        })
    }

    /// Fit the edge prediction perceptron model on the provided graph and node features.
    ///
    /// # Arguments
    /// `graph`: &Graph - The graph to embed
    /// `node_features`: &[F] - A node features matrix.
    /// `dimension`: usize - The dimensionality of the node features.
    /// `verbose`: Option<bool> - Whether to show a loading bar for the epochs. By default, True.
    pub fn fit<F>(
        &mut self,
        graph: &Graph,
        node_features: &[F],
        dimension: usize,
        verbose: Option<bool>,
    ) -> Result<(), String>
    where
        F: Into<f32>,
    {
        if !graph.has_edges() {
            return Err("The provided graph does not have any edge.".to_string());
        }

        if dimension == 0 {
            return Err(concat!(
                "The provided feature dimensions is zero. ",
                "The number of node features should be a strictly positive value."
            )
            .to_string());
        }

        if node_features.len() != graph.get_nodes_number() as usize * dimension {
            return Err(format!(
                concat!(
                    "The provided node features have size {}, but the expected size ",
                    "based on the provided graph and dimension is {}. Specifically, ",
                    "the expected shape of the matrix is ({}, {})."
                ),
                node_features.len(),
                graph.get_nodes_number() as usize * dimension,
                graph.get_nodes_number(),
                dimension
            ));
        }

        let mut random_state: u64 = splitmix64(self.random_state);
        let scale_factor: f32 = (dimension as f32).sqrt();
        let verbose: bool = verbose.unwrap_or(true);

        // Initialize the model with weights and bias in the range (-1 / sqrt(k), +1 / sqrt(k))
        let get_random_weight = |seed: usize| {
            (2.0 * random_f32(splitmix64(random_state + seed as u64)) - 1.0) / scale_factor
        };
        self.weights = (0..dimension)
            .map(|i| get_random_weight(i))
            .collect::<Vec<f32>>();
        self.bias = get_random_weight(self.weights.len());

        // Compute minimum and

        // Depending whether verbosity was requested by the user
        // we create or not a visible progress bar to show the progress
        // in the training epochs.
        let epochs_progress_bar = if verbose {
            let pb = ProgressBar::new(self.number_of_epochs as u64);
            pb.set_style(ProgressStyle::default_bar().template(concat!(
                "Edge Prediction Perceptron Epochs ",
                "{msg} {spinner:.green} [{elapsed_precise}] ",
                "[{bar:40.cyan/blue}] ({pos}/{len}, ETA {eta})"
            )));
            pb
        } else {
            ProgressBar::hidden()
        };

        // We start to loop over the required amount of epochs.
        (0..self.number_of_epochs).for_each(|epoch| {
            graph.par_iter_attributed_edge_prediction_mini_batch()
        });
        Ok(())
    }
}
