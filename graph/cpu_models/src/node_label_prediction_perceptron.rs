use crate::Optimizer;
use crate::{get_random_weight, must_not_be_zero, FeatureSlice};
use graph::Graph;
use half::f16;
use indicatif::ProgressIterator;
use indicatif::{ProgressBar, ProgressStyle};
use itertools::izip;
use num_traits::AsPrimitive;
use rayon::prelude::*;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use vec_rand::splitmix64;

#[derive(Clone, Deserialize, Serialize)]
pub struct NodeLabelPredictionPerceptron<O> {
    /// Bias Optimizer
    bias_optimizer: O,
    /// Weights optimizer
    weight_optimizers: Vec<O>,
    /// The weights of the model.
    weights: Vec<f32>,
    /// The bias of the model.
    bias: Vec<f32>,
    /// The number of epochs to train the model for.
    number_of_epochs: usize,
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
    /// * `number_of_epochs`: Option<usize> - The number of epochs to train the model for. By default, `100`.
    /// * `random_state`: Option<u64> - The random state to reproduce the model initialization and training. By default, `42`.
    pub fn new(
        optimizer: O,
        number_of_epochs: Option<usize>,
        random_state: Option<u64>,
    ) -> Result<Self, String> {
        let number_of_epochs = must_not_be_zero(number_of_epochs, 100, "number of epochs")?;

        Ok(Self {
            bias_optimizer: optimizer,
            weight_optimizers: Vec::new(),
            weights: Vec::new(),
            bias: Vec::new(),
            number_of_epochs,
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
        self.must_be_trained().map(|_| self.bias.clone())
    }

    fn iterate_feature<'a>(
        &'a self,
        node_id: usize,
        node_features: &'a [FeatureSlice],
        dimensions: &'a [usize],
    ) -> impl Iterator<Item = f32> + 'a {
        use crate::FeatureSlice::*;
        izip!(node_features, dimensions.iter().copied()).map(move |(node_feature, dimension)| {
            let offset = node_id * dimension;
            (0..dimension)
                .map(|position| match node_feature {
                    F16(feature) => <f16 as AsPrimitive<f32>>::as_(feature[offset + position]),
                    F32(feature) => <f32 as AsPrimitive<f32>>::as_(feature[offset + position]),
                    F64(feature) => <f64 as AsPrimitive<f32>>::as_(feature[offset + position]),
                    U8(feature) => <u8 as AsPrimitive<f32>>::as_(feature[offset + position]),
                    U16(feature) => <u16 as AsPrimitive<f32>>::as_(feature[offset + position]),
                    U32(feature) => <u32 as AsPrimitive<f32>>::as_(feature[offset + position]),
                    U64(feature) => <u64 as AsPrimitive<f32>>::as_(feature[offset + position]),
                    I8(feature) => <i8 as AsPrimitive<f32>>::as_(feature[offset + position]),
                    I16(feature) => <i16 as AsPrimitive<f32>>::as_(feature[offset + position]),
                    I32(feature) => <i32 as AsPrimitive<f32>>::as_(feature[offset + position]),
                    I64(feature) => <i64 as AsPrimitive<f32>>::as_(feature[offset + position]),
                })
                .sum::<f32>()
        })
    }

    fn dots_iterator<'a>(
        &'a self,
        node_id: usize,
        weights: &'a [f32],
        node_features: &'a [FeatureSlice],
        dimensions: &'a [usize],
    ) -> impl Iterator<Item = f32> + 'a {
        weights
            .iter()
            .copied()
            .zip(self.iterate_feature(node_id, node_features, dimensions))
            .map(|(weight, feature)| weight * feature)
    }

    fn dot(
        &self,
        node_id: usize,
        weights: &[f32],
        node_features: &[FeatureSlice],
        dimensions: &[usize],
    ) -> f32 {
        self.dots_iterator(node_id, weights, node_features, dimensions)
            .sum()
    }

    fn stable_softmax(
        &self,
        node_id: usize,
        node_features: &[FeatureSlice],
        dimensions: &[usize],
    ) -> Vec<f32> {
        let mut maximum_activation = -f32::INFINITY;
        let mut activations = izip!(
            self.weights.chunks(self.weights.len() / self.bias.len()),
            self.bias.as_slice(),
        )
        .map(|(weights, bias)| {
            let activation = self.dot(node_id, weights, node_features, dimensions) + bias;
            maximum_activation = maximum_activation.max(activation);
            activation
        })
        .collect::<Vec<f32>>();

        // Compute the total activation and exponentiate the
        // single activation.
        let total_activation = activations
            .iter_mut()
            .map(|activation| {
                // Note that here we remove the maximum activation
                // to increase the stability of the softmax.
                // We use the maximum value as it shifts all of elements
                // in the vector to negative to zero,
                // and negatives with large exponents saturate to zero rather than the infinity,
                // avoiding overflowing and resulting in NaN.
                *activation = (*activation - maximum_activation).exp();
                *activation
            })
            .sum::<f32>();

        // Normalize predictions
        activations.iter_mut().for_each(|activation| {
            *activation /= total_activation;
        });

        activations
    }

    fn multi_stable_sigmoid(
        &self,
        node_id: usize,
        node_features: &[FeatureSlice],
        dimensions: &[usize],
    ) -> Vec<f32> {
        izip!(
            self.weights.chunks(self.weights.len() / self.bias.len()),
            self.bias.as_slice(),
        )
        .map(|(weights, bias)| {
            let activation = self.dot(node_id, weights, node_features, dimensions) + bias;
            if activation > 0.0 {
                1.0 / (1.0 + activation.exp())
            } else {
                let exp_activation = activation.exp();
                exp_activation / (1.0 + exp_activation)
            }
        })
        .collect::<Vec<f32>>()
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
        let number_of_nodes = graph.get_number_of_nodes() as f32;
        let multilabel = graph.has_multilabel_node_types()?;
        let random_state: u64 = splitmix64(self.random_state);
        let verbose: bool = verbose.unwrap_or(true);

        self.bias_optimizer.set_capacity(number_of_node_labels);
        self.weight_optimizers = (0..number_of_node_labels)
            .map(|_| {
                let mut optimizer = self.bias_optimizer.clone();
                optimizer.set_capacity(number_of_features);
                optimizer
            })
            .collect::<Vec<O>>();

        let number_of_features_root = (number_of_features as f32).sqrt();

        let number_of_weights = number_of_features * number_of_node_labels;

        self.weights = (0..number_of_weights)
            .map(|i| get_random_weight(random_state + i as u64, number_of_features_root))
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

        // We start to loop over the required amount of epochs.
        for _ in (0..self.number_of_epochs).progress_with(progress_bar) {
            let (mut weights_gradient, mut bias_gradient) =
                graph
                    .get_node_type_ids()?
                    .par_iter()
                    .enumerate()
                    .filter_map(|(node_id, node_label)| {
                        node_label.as_ref().map(|node_label| (node_id, node_label))
                    })
                    .map(|(node_id, node_type_ids)| {
                        let mut predictions = if multilabel {
                            self.multi_stable_sigmoid(node_id, node_features, dimensions)
                        } else {
                            self.stable_softmax(node_id, node_features, dimensions)
                        };

                        // Actually compute the loss
                        node_type_ids.iter().copied().for_each(|node_type_id| {
                            predictions[node_type_id as usize] -= 1.0;
                        });

                        // Compute the gradients
                        (
                            self.dots_iterator(
                                node_id,
                                predictions.as_slice(),
                                node_features,
                                dimensions,
                            )
                            .collect(),
                            predictions,
                        )
                    })
                    .reduce(
                        || {
                            (
                                vec![0.0; number_of_weights],
                                vec![0.0; number_of_node_labels],
                            )
                        },
                        |(mut total_weights_gradient, mut total_bias_gradient): (
                            Vec<f32>,
                            Vec<f32>,
                        ),
                         (partial_weights_gradient, partial_bias_gradient): (
                            Vec<f32>,
                            Vec<f32>,
                        )| {
                            total_weights_gradient
                                .iter_mut()
                                .zip(partial_weights_gradient.into_iter())
                                .for_each(|(total_weight_gradient, partial_weight_gradient)| {
                                    *total_weight_gradient += partial_weight_gradient;
                                });
                            total_bias_gradient
                                .iter_mut()
                                .zip(partial_bias_gradient.into_iter())
                                .for_each(|(total_bias_gradient, partial_bias_gradient)| {
                                    *total_bias_gradient += partial_bias_gradient;
                                });
                            (total_weights_gradient, total_bias_gradient)
                        },
                    );

            weights_gradient
                .par_iter_mut()
                .chain(bias_gradient.par_iter_mut())
                .for_each(|gradient| {
                    *gradient /= number_of_nodes;
                });

            self.bias_optimizer.get_update(&mut bias_gradient);
            self.weight_optimizers
                .par_iter_mut()
                .zip(weights_gradient.par_chunks_mut(number_of_features))
                .for_each(|(weight_optimizer, weights_gradient)| {
                    weight_optimizer.get_update(weights_gradient);
                });

            weights_gradient
                .into_par_iter()
                .zip(self.weights.par_iter_mut())
                .chain(bias_gradient.into_par_iter().zip(self.bias.par_iter_mut()))
                .for_each(|(gradient, weight)| {
                    *weight -= gradient;
                });
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

        // if predictions.len() != graph.get_number_of_directed_edges() as usize {
        //     return Err(format!(
        //         concat!(
        //             "The provided predictions slice has size `{}` ",
        //             "but it was expected to have the same ",
        //             "size of the number of the directed edges in the graph `{}`."
        //         ),
        //         predictions.len(),
        //         graph.get_number_of_directed_edges()
        //     ));
        // }

        // if self.weights.len() != edge_embedding_dimension {
        //     return Err(format!(
        //         concat!(
        //             "This model was not trained on features compatible with ",
        //             "the provided features. Specifically, the model was trained ",
        //             "on features with edge embedding dimension `{}`, while the features you have ",
        //             "provided have edge embedding dimension `{}`."
        //         ),
        //         self.weights.len(),
        //         edge_embedding_dimension
        //     ));
        // }

        // predictions
        //     .par_iter_mut()
        //     .zip(graph.par_iter_directed_edge_node_ids())
        //     .for_each(|(prediction, (_, src, dst))| {
        //         *prediction = unsafe {
        //             self.get_unsafe_prediction(src, dst, support, node_features, dimensions)
        //                 .1
        //         };
        //     });

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
