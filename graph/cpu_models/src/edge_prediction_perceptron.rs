use express_measures::{
    cosine_similarity_sequential_unchecked, dot_product_sequential_unchecked,
    euclidean_distance_sequential_unchecked,
};
use graph::Graph;
use indicatif::ProgressIterator;
use indicatif::{ProgressBar, ProgressStyle};
use rayon::prelude::*;
use vec_rand::{random_f32, splitmix64};

#[derive(Clone, Debug, Copy)]
pub enum EdgeEmbeddingMethods {
    CosineSimilarity,
    EuclideanDistance,
    Hadamard,
}

pub fn get_edge_embedding_method_dimensionality(
    method: EdgeEmbeddingMethods,
    dimension: usize,
) -> usize {
    match method {
        EdgeEmbeddingMethods::CosineSimilarity => 1,
        EdgeEmbeddingMethods::EuclideanDistance => 1,
        EdgeEmbeddingMethods::Hadamard => dimension,
    }
}

pub fn get_edge_embedding_method_name_from_string(
    candidate_method_name: &str,
) -> Result<EdgeEmbeddingMethods, String> {
    match candidate_method_name {
        "CosineSimilarity" => Ok(EdgeEmbeddingMethods::CosineSimilarity),
        "EuclideanDistance" => Ok(EdgeEmbeddingMethods::EuclideanDistance),
        "Hadamard" => Ok(EdgeEmbeddingMethods::Hadamard),
        _ => Err(format!(
            concat!(
                "The provided edge embedding method name {} is not supported. ",
                "The supported edge embedding method names are `CosineSimilarity`, ",
                "`EuclideanDistance` and `Hadamard`."
            ),
            candidate_method_name
        ))
    }
}

#[derive(Clone, Debug)]
pub struct EdgePredictionPerceptron {
    /// The name of the method to use to compute the edge embedding.
    edge_embedding_method_name: EdgeEmbeddingMethods,
    /// The weights of the model.
    weights: Vec<f32>,
    /// The bias of the model.
    bias: f32,
    /// The number of epochs to train the model for.
    number_of_epochs: usize,
    /// Number of samples in a mini-batch. By default 1024.
    number_of_edges_per_mini_batch: usize,
    /// Whether to train this model by sampling only edges with nodes with different node types.
    sample_only_edges_with_heterogeneous_node_types: bool,
    /// Learning rate to use to train the model.
    learning_rate: f32,
    /// The random state to reproduce the model initialization and training.
    random_state: u64,
}

impl EdgePredictionPerceptron {
    /// Return new instance of Perceptron for edge prediction.
    ///
    /// # Arguments
    /// * `edge_embedding_method_name`: Option<EdgeEmbeddingMethods> - The embedding method to use. By default the cosine similarity is used.
    /// * `number_of_epochs`: Option<usize> - The number of epochs to train the model for. By default, 100.
    /// * `number_of_edges_per_mini_batch`: Option<usize> - The number of samples to include for each mini-batch. By default 1024.
    /// * `sample_only_edges_with_heterogeneous_node_types`: Option<bool> - Whether to sample negative edges only with source and destination nodes that have different node types. By default false.
    /// * `learning_rate`: Option<f32> - Learning rate to use while training the model. By default 0.001.
    /// * `random_state`: Option<u64> - The random state to reproduce the model initialization and training. By default, 42.
    pub fn new(
        edge_embedding_method_name: Option<EdgeEmbeddingMethods>,
        number_of_epochs: Option<usize>,
        number_of_edges_per_mini_batch: Option<usize>,
        sample_only_edges_with_heterogeneous_node_types: Option<bool>,
        learning_rate: Option<f32>,
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
        let learning_rate = learning_rate.unwrap_or(0.001);
        if learning_rate <= 0.0 {
            return Err(concat!(
                "The provided learning rate must be a value strictly greater than zero."
            )
            .to_string());
        }

        let edge_embedding_method_name =
            edge_embedding_method_name.unwrap_or(EdgeEmbeddingMethods::CosineSimilarity);
        Ok(Self {
            edge_embedding_method_name,
            weights: Vec::new(),
            bias: 0.0,
            number_of_epochs,
            number_of_edges_per_mini_batch,
            sample_only_edges_with_heterogeneous_node_types:
                sample_only_edges_with_heterogeneous_node_types.unwrap_or(false),
            learning_rate,
            random_state: random_state.unwrap_or(42),
        })
    }

    fn validate_features(
        &self,
        graph: &Graph,
        node_features: &[f32],
        dimension: usize,
    ) -> Result<(), String> {
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
        Ok(())
    }

    /// Returns method to compute the edge embedding.
    fn get_edge_embedding_method(&self) -> fn(&[f32], &[f32]) -> Vec<f32> {
        match self.edge_embedding_method_name {
            EdgeEmbeddingMethods::CosineSimilarity => {
                |a: &[f32], b: &[f32]| vec![unsafe { cosine_similarity_sequential_unchecked(a, b) }]
            }
            EdgeEmbeddingMethods::EuclideanDistance => |a: &[f32], b: &[f32]| {
                vec![unsafe { euclidean_distance_sequential_unchecked(a, b) }]
            },
            EdgeEmbeddingMethods::Hadamard => |a: &[f32], b: &[f32]| {
                a.iter()
                    .copied()
                    .zip(b.iter().copied())
                    .map(|(feature_a, feature_b)| feature_a * feature_b)
                    .collect::<Vec<f32>>()
            },
        }
    }

    /// Fit the edge prediction perceptron model on the provided graph and node features.
    ///
    /// # Arguments
    /// * `graph`: &Graph - The graph whose edges are to be learned.
    /// * `node_features`: &[f32] - A node features matrix.
    /// * `dimension`: usize - The dimensionality of the node features.
    /// * `verbose`: Option<bool> - Whether to show a loading bar for the epochs. By default, True.
    /// * `support`: Option<&'a Graph> - Graph to use to check for false negatives.
    /// * `graph_to_avoid`: &'a Option<&Graph> - The graph whose edges are to be avoided during the generation of false negatives,
    pub fn fit(
        &mut self,
        graph: &Graph,
        node_features: &[f32],
        dimension: usize,
        verbose: Option<bool>,
        support: Option<&Graph>,
        graph_to_avoid: Option<&Graph>,
    ) -> Result<(), String> {
        self.validate_features(graph, node_features, dimension)?;

        let mut random_state: u64 = splitmix64(self.random_state);
        let scale_factor: f32 = (dimension as f32).sqrt();
        let verbose: bool = verbose.unwrap_or(true);

        // Initialize the model with weights and bias in the range (-1 / sqrt(k), +1 / sqrt(k))
        let get_random_weight = |seed: usize| {
            (2.0 * random_f32(splitmix64(random_state + seed as u64)) - 1.0) / scale_factor
        };
        let edge_dimension =
            get_edge_embedding_method_dimensionality(self.edge_embedding_method_name, dimension);
        self.weights = (0..edge_dimension)
            .map(|i| get_random_weight(i))
            .collect::<Vec<f32>>();
        self.bias = get_random_weight(self.weights.len());

        // Depending whether verbosity was requested by the user
        // we create or not a visible progress bar to show the progress
        // in the training epochs.
        let progress_bar = if verbose {
            let pb = ProgressBar::new(self.number_of_epochs as u64);
            pb.set_style(ProgressStyle::default_bar().template(concat!(
                "Edge Prediction Perceptron Epochs ",
                "{spinner:.green} [{elapsed_precise}] ",
                "[{bar:40.cyan/blue}] ({pos}/{len}, ETA {eta})"
            )));
            pb
        } else {
            ProgressBar::hidden()
        };

        let number_of_batches_per_epoch = (graph.get_number_of_directed_edges() as f32
            / self.number_of_edges_per_mini_batch as f32)
            .ceil() as usize;

        let method = self.get_edge_embedding_method();

        // We start to loop over the required amount of epochs.
        (0..self.number_of_epochs)
            .progress_with(progress_bar)
            .map(|_| {
                (0..number_of_batches_per_epoch)
                    .map(|_| {
                        random_state = splitmix64(random_state);
                        let (total_weights_gradient, total_bias_gradient, total_samples) = graph
                            .par_iter_edge_prediction_mini_batch(
                                random_state,
                                self.number_of_edges_per_mini_batch,
                                self.sample_only_edges_with_heterogeneous_node_types,
                                Some(1.0),
                                Some(true),
                                None,
                                Some(true),
                                support,
                                graph_to_avoid,
                            )?
                            .map(|(src, dst, label)| {
                                let src: usize = src as usize;
                                let dst: usize = dst as usize;
                                let src_features = &node_features
                                    [src * edge_dimension..(src + 1) * edge_dimension];
                                let dst_features = &node_features
                                    [dst * edge_dimension..(dst + 1) * edge_dimension];
                                (method(src_features, dst_features), label)
                            })
                            .filter_map(|(mut edge_embedding, label)| {
                                let dot = unsafe {
                                    dot_product_sequential_unchecked(&edge_embedding, &self.weights)
                                } / scale_factor
                                    + self.bias;

                                // If the current signal is already quite strong, it does not make sense
                                // to update the loss function as we may start cause propagating NaNs.
                                if dot < -6.0 || dot > 6.0 {
                                    return None;
                                }

                                let exponentiated_dot = dot.exp();
                                let mut variation = -exponentiated_dot / (exponentiated_dot + 1.0);
                                if label {
                                    variation += 1.0;
                                }
                                variation *= self.learning_rate;
                                edge_embedding.iter_mut().for_each(|edge_feature| {
                                    *edge_feature *= variation;
                                });
                                Some((edge_embedding, variation, 1))
                            })
                            .reduce(
                                || (vec![0.0; edge_dimension], 0.0, 0),
                                |(
                                    mut total_weights_gradient,
                                    mut total_bias_gradient,
                                    mut total_samples,
                                ): (Vec<f32>, f32, usize),
                                 (
                                    partial_weights_gradient,
                                    partial_bias_gradient,
                                    partial_samples,
                                ): (Vec<f32>, f32, usize)| {
                                    total_weights_gradient
                                        .iter_mut()
                                        .zip(partial_weights_gradient.into_iter())
                                        .for_each(
                                            |(total_weight_gradient, partial_weight_gradient)| {
                                                *total_weight_gradient += partial_weight_gradient;
                                            },
                                        );
                                    total_bias_gradient += partial_bias_gradient;
                                    total_samples += partial_samples;
                                    (total_weights_gradient, total_bias_gradient, total_samples)
                                },
                            );
                        let total_samples = total_samples as f32;
                        self.bias += total_bias_gradient / total_samples;
                        self.weights
                            .par_iter_mut()
                            .zip(total_weights_gradient.into_par_iter())
                            .for_each(|(weight, total_weight_gradient)| {
                                *weight += total_weight_gradient / total_samples;
                            });
                        Ok(())
                    })
                    .collect::<Result<(), String>>()
            })
            .collect::<Result<(), String>>()
    }

    /// Writes the predicted probabilities on the provided memory area.
    ///
    /// # Arguments
    /// * `predictions`: &mut [f32] - Area where to write the predictions.
    /// * `graph`: &Graph - The graph whose edges are to be learned.
    /// * `node_features`: &[f32] - A node features matrix.
    /// * `dimension`: usize - The dimensionality of the node features.
    pub fn predict(
        &self,
        predictions: &mut [f32],
        graph: &Graph,
        node_features: &[f32],
        dimension: usize,
    ) -> Result<(), String> {
        self.validate_features(graph, node_features, dimension)?;

        if predictions.len() != graph.get_number_of_directed_edges() as usize {
            return Err(format!(
                concat!(
                    "The provided predictions slice has size `{}` ",
                    "but it was expected to have the same ",
                    "size of the number of the directed edges in the graph `{}`."
                ),
                predictions.len(),
                graph.get_number_of_directed_edges()
            ));
        }

        if self.weights.is_empty() {
            return Err(concat!(
                "This model has not been trained yet. ",
                "Before calling the `.predict` method, you ",
                "should call the `.fit` method."
            )
            .to_string());
        }

        let edge_dimension =
            get_edge_embedding_method_dimensionality(self.edge_embedding_method_name, dimension);

        if self.weights.len() != edge_dimension {
            return Err(format!(
                concat!(
                    "This model was not trained on features compatible with ",
                    "the provided features. Specifically, the model was trained ",
                    "on features with edge embedding dimension `{}`, while the features you have ",
                    "provided have edge embedding dimension `{}`."
                ),
                self.weights.len(),
                edge_dimension
            ));
        }

        let scale_factor: f32 = (dimension as f32).sqrt();
        let method = self.get_edge_embedding_method();

        predictions
            .par_iter_mut()
            .zip(graph.par_iter_directed_edge_node_ids())
            .for_each(|(prediction, (_, src, dst))| {
                let src = src as usize;
                let dst = dst as usize;
                let src_features = &node_features[src * edge_dimension..(src + 1) * edge_dimension];
                let dst_features = &node_features[dst * edge_dimension..(dst + 1) * edge_dimension];
                let dot_product = unsafe {
                    dot_product_sequential_unchecked(
                        &method(src_features, dst_features),
                        &self.weights,
                    ) / scale_factor
                } + self.bias;
                // The following if-else are needed to ensure numerical stability.
                if dot_product < -10.0 {
                    *prediction = 0.0;
                } else if dot_product > 10.0 {
                    *prediction = 1.0;
                } else {
                    let exponentiated_dot_product = dot_product.exp();
                    *prediction = exponentiated_dot_product / (1.0 + exponentiated_dot_product);
                }
            });

        Ok(())
    }
}
