use crate::Optimizer;
use crate::{get_random_weight, must_not_be_zero, FeatureSlice};
use core::ops::Sub;
use express_measures::{
    absolute_distance, cosine_similarity_sequential_unchecked, dot_product_sequential_unchecked,
    euclidean_distance_sequential_unchecked,
};
use graph::{Graph, NodeT};
use indicatif::ProgressIterator;
use indicatif::{ProgressBar, ProgressStyle};
use num_traits::{AsPrimitive, Zero};
use rayon::prelude::*;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;
use vec_rand::splitmix64;

#[derive(Clone, Deserialize, Serialize)]
pub struct NodeLabelPredictionPerceptron<O> {
    /// Bias Optimizer
    bias_optimizer: O,
    /// Weights optimizer
    weight_optimizer: Vec<O>,
    /// The weights of the model.
    weights: Vec<f32>,
    /// The bias of the model.
    bias: Vec<f32>,
    /// Whether to employ negative sampling. Mostly useful when the node-labels are many.
    use_negative_sampling: bool,
    /// Whether to avoid sampling false negatives. This may cause a slower training. Only meaningful when negative sampling is used.
    avoid_false_negatives: bool,
    /// The number of epochs to train the model for.
    number_of_epochs: usize,
    /// Number of samples in a mini-batch. By default 256.
    number_of_nodes_per_mini_batch: usize,
    /// The random state to reproduce the model initialization and training.
    random_state: u64,
}

impl<O> NodeLabelPredictionPerceptron<O>
where
    O: Optimizer<Vec<f32>> + Serialize + DeserializeOwned,
{
    /// Return new instance of Perceptron for edge prediction.
    ///
    /// # Arguments
    /// * `optimizer`: Optimizer - The optimizer to be used for the training.
    /// * `use_negative_sampling`: Option<bool> - Whether to employ negative sampling. Mostly useful when the node-labels are many.
    /// * `avoid_false_negatives`: Option<bool> - Whether to avoid sampling false negatives. This may cause a slower training.
    /// * `number_of_epochs`: Option<usize> - The number of epochs to train the model for. By default, `100`.
    /// * `number_of_nodes_per_mini_batch`: Option<usize> - The number of samples to include for each mini-batch. By default `256`.
    /// * `random_state`: Option<u64> - The random state to reproduce the model initialization and training. By default, `42`.
    pub fn new(
        optimizer: O,
        use_negative_sampling: Option<bool>,
        avoid_false_negatives: Option<bool>,
        number_of_epochs: Option<usize>,
        number_of_nodes_per_mini_batch: Option<usize>,
        random_state: Option<u64>,
    ) -> Result<Self, String> {
        let number_of_epochs = must_not_be_zero(number_of_epochs, 100, "number of epochs")?;
        let number_of_nodes_per_mini_batch = must_not_be_zero(
            number_of_nodes_per_mini_batch,
            256,
            "number of edges per mini-batch",
        )?;

        Ok(Self {
            bias_optimizer: optimizer,
            weight_optimizer: Vec::new(),
            weights: Vec::new(),
            bias: Vec::new(),
            use_negative_sampling: use_negative_sampling.unwrap_or(false),
            avoid_false_negatives: avoid_false_negatives.unwrap_or(false),
            number_of_epochs,
            number_of_nodes_per_mini_batch,
            random_state: splitmix64(random_state.unwrap_or(42)),
        })
    }

    fn must_be_trained(&self) -> Result<(), String> {
        if self.weights.is_empty() {
            return Err(concat!(
                "This model has not been trained yet. ",
                "You should call the `.fit` method first."
            )
            .to_string());
        }
        Ok(())
    }

    /// Returns the weights of the model.
    pub fn get_weights(&self) -> Result<Vec<Vec<f32>>, String> {
        self.must_be_trained().map(|_| {
            self.weights
                .chunks(self.bias.len())
                .map(|weights| weights.to_vec())
                .collect::<Vec<Vec<f32>>>()
        })
    }

    /// Returns the bias of the model.
    pub fn get_bias(&self) -> Result<Vec<f32>, String> {
        self.must_be_trained().map(|_| self.bias)
    }

    fn validate_features(
        &self,
        graph: &Graph,
        node_features: &[FeatureSlice],
        dimensions: &[usize],
    ) -> Result<(), String> {
        if node_features.is_empty() {
            return Err("The provided node features are empty".to_string());
        }

        if node_features.len() != dimensions.len() {
            return Err(format!(
                concat!(
                    "You have provided {} node features, but ",
                    "you have provided {} dimensions."
                ),
                node_features.len(),
                dimensions.len()
            ));
        }

        if !graph.has_nodes() {
            return Err("The provided graph does not have any node.".to_string());
        }

        for (node_feature, dimension) in node_features.iter().zip(dimensions.iter()) {
            if *dimension == 0 {
                return Err(concat!(
                    "The provided feature dimensions is zero. ",
                    "The number of node features should be a strictly positive value."
                )
                .to_string());
            }

            if node_feature.len() != graph.get_number_of_nodes() as usize * dimension {
                return Err(format!(
                    concat!(
                        "The provided node features have size {}, but the expected size ",
                        "based on the provided graph and dimension is {}. Specifically, ",
                        "the expected shape of the matrix is ({}, {})."
                    ),
                    node_feature.len(),
                    graph.get_number_of_nodes() as usize * dimension,
                    graph.get_number_of_nodes(),
                    dimension
                ));
            }
        }

        Ok(())
    }

    /// Returns the prediction for the provided nodes, edge embedding method and current model.
    ///
    /// # Arguments
    /// `node`: NodeT - The source node whose features are to be extracted.
    /// `node_features`: &[&Vec<f32>] - The node features to use.
    /// `dimensions`: &[usize] - The dimension of the provided node features.
    ///
    /// # Safety
    /// In this method we do not execute any checks such as whether the
    /// node features are compatible with the provided node IDs, and therefore
    /// improper parametrization may lead to panic or undefined behaviour.
    unsafe fn get_unsafe_prediction(
        &self,
        node: NodeT,
        node_features: &[FeatureSlice],
        dimensions: &[usize],
    ) -> (Vec<f32>, f32) {
        let dot = dot_product_sequential_unchecked(&edge_embedding, &self.weights) + self.bias;
        (edge_embedding, 1.0 / (1.0 + (-dot).exp()))
    }

    /// Fit the edge prediction perceptron model on the provided graph and node features.
    ///
    /// # Arguments
    /// * `graph`: &Graph - The graph whose edges are to be learned.
    /// * `node_features`: &[&Vec<f32>] - List of node features matrices.
    /// * `dimensions`: &[usize] - The dimensionality of the node features.
    /// * `verbose`: Option<bool> - Whether to show a loading bar for the epochs. By default, True.
    pub fn fit(
        &mut self,
        graph: &Graph,
        node_features: &[FeatureSlice],
        dimensions: &[usize],
        verbose: Option<bool>,
    ) -> Result<(), String> {
        self.validate_features(graph, node_features, dimensions)?;

        let number_of_features = dimensions.iter().sum::<usize>();
        let number_of_node_labels = graph.get_number_of_node_types()? as usize;
        let mut random_state: u64 = splitmix64(self.random_state);
        let verbose: bool = verbose.unwrap_or(true);

        self.bias_optimizer.set_capacity(number_of_node_labels);
        self.weight_optimizer = (0..number_of_node_labels)
            .map(|_| {
                let mut optimizer = self.bias_optimizer.clone();
                optimizer.set_capacity(number_of_features);
                optimizer
            })
            .collect::<Vec<O>>();
        let number_of_features_root = (number_of_features as f32).sqrt();
        self.weights = (0..(number_of_features * number_of_node_labels))
            .map(|i| get_random_weight(i as u64, number_of_features_root))
            .collect::<Vec<f32>>();
        self.bias = vec![0.0; number_of_node_labels];

        // Depending whether verbosity was requested by the user
        // we create or not a visible progress bar to show the progress
        // in the training epochs.
        let progress_bar = if verbose {
            let pb = ProgressBar::new(self.number_of_epochs as u64);
            pb.set_style(
                ProgressStyle::default_bar()
                    .template(concat!(
                        "Perceptron ",
                        "{spinner:.green} [{elapsed_precise}] ",
                        "[{bar:40.cyan/blue}] ({pos}/{len}, ETA {eta})"
                    ))
                    .unwrap(),
            );
            pb
        } else {
            ProgressBar::hidden()
        };

        let number_of_batches_per_epoch = (graph.get_number_of_nodes() as f32
            / self.number_of_nodes_per_mini_batch as f32)
            .ceil() as usize;

        // We start to loop over the required amount of epochs.
        for _ in (0..self.number_of_epochs).progress_with(progress_bar) {
            let total_variation = (0..number_of_batches_per_epoch)
                .map(|_| {
                    random_state = splitmix64(random_state);
                    let (mut total_weights_gradient, mut total_variation) = graph
                        .par_iter_edge_prediction_mini_batch(
                            random_state,
                            self.number_of_edges_per_mini_batch,
                            self.sample_only_edges_with_heterogeneous_node_types,
                            Some(0.5),
                            Some(self.avoid_false_negatives),
                            None,
                            Some(self.use_scale_free_distribution),
                            Some(support),
                            graph_to_avoid,
                        )?
                        .map(|(src, dst, label)| {
                            let (mut edge_embedding, prediction) = unsafe {
                                self.get_unsafe_prediction(
                                    src,
                                    dst,
                                    support,
                                    node_features,
                                    dimensions,
                                )
                            };

                            let variation = if label { prediction - 1.0 } else { prediction };

                            edge_embedding.iter_mut().for_each(|edge_feature| {
                                *edge_feature *= variation;
                            });

                            (edge_embedding, variation)
                        })
                        .reduce(
                            || (vec![0.0; edge_embedding_dimension], 0.0),
                            |(mut total_weights_gradient, mut total_variation): (Vec<f32>, f32),
                             (
                                partial_weights_gradient,
                                partial_variation,
                            ): (Vec<f32>, f32)| {
                                total_weights_gradient
                                    .iter_mut()
                                    .zip(partial_weights_gradient.into_iter())
                                    .for_each(
                                        |(total_weight_gradient, partial_weight_gradient)| {
                                            *total_weight_gradient += partial_weight_gradient;
                                        },
                                    );
                                total_variation += partial_variation;
                                (total_weights_gradient, total_variation)
                            },
                        );

                    total_variation /= self.number_of_edges_per_mini_batch as f32;
                    total_weights_gradient
                        .iter_mut()
                        .for_each(|total_weight_gradient| {
                            *total_weight_gradient /= self.number_of_edges_per_mini_batch as f32;
                        });

                    self.bias_optimizer.get_update(&mut total_variation);
                    self.weight_optimizer
                        .get_update(&mut total_weights_gradient);

                    self.bias -= total_variation;
                    self.weights
                        .iter_mut()
                        .zip(total_weights_gradient.into_iter())
                        .for_each(|(weight, total_weight_gradient)| {
                            *weight -= total_weight_gradient;
                        });

                    Ok(total_variation.abs())
                })
                .sum::<Result<f32, String>>()?;
            if total_variation.is_zero() {
                break;
            }
        }
        Ok(())
    }

    /// Writes the predicted probabilities on the provided memory area.
    ///
    /// # Arguments
    /// * `predictions`: &mut [f32] - Area where to write the predictions.
    /// * `graph`: &Graph - The graph whose edges are to be learned.
    /// * `node_features`: &[FeatureSlice] - A node features matrix.
    /// * `dimension`: &[usize] - The dimensionality of the node features.
    /// * `support`: Option<&Graph> - Graph to use for the topological features.
    pub fn predict(
        &self,
        predictions: &mut [f32],
        graph: &Graph,
        node_features: &[FeatureSlice],
        dimensions: &[usize],
        support: Option<&Graph>,
    ) -> Result<(), String> {
        let support = support.unwrap_or(graph);
        self.validate_features(support, node_features, dimensions)?;
        self.must_be_trained()?;

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

        if self.weights.len() != edge_embedding_dimension {
            return Err(format!(
                concat!(
                    "This model was not trained on features compatible with ",
                    "the provided features. Specifically, the model was trained ",
                    "on features with edge embedding dimension `{}`, while the features you have ",
                    "provided have edge embedding dimension `{}`."
                ),
                self.weights.len(),
                edge_embedding_dimension
            ));
        }

        predictions
            .par_iter_mut()
            .zip(graph.par_iter_directed_edge_node_ids())
            .for_each(|(prediction, (_, src, dst))| {
                *prediction = unsafe {
                    self.get_unsafe_prediction(src, dst, support, node_features, dimensions)
                        .1
                };
            });

        Ok(())
    }

    pub fn dump(&self, path: &str) -> Result<(), String> {
        serde_json::to_writer(
            std::fs::File::create(path).map_err(|e| e.to_string())?,
            self,
        )
        .map_err(|e| e.to_string())?;
        Ok(())
    }

    pub fn dumps(&self) -> Result<String, String> {
        serde_json::to_string(self).map_err(|e| e.to_string())
    }

    pub fn load(path: &str) -> Result<Self, String> {
        serde_json::from_reader(std::fs::File::open(path).map_err(|e| e.to_string())?)
            .map_err(|e| e.to_string())
    }

    pub fn loads(json: &str) -> Result<Self, String> {
        serde_json::from_str(json).map_err(|e| e.to_string())
    }
}
