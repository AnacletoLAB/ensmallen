use express_measures::dot_product_sequential_unchecked;
use graph::{Graph, NodeT};
use indicatif::ProgressIterator;
use indicatif::{ProgressBar, ProgressStyle};
use rayon::prelude::*;
use vec_rand::{random_f64, splitmix64};

#[derive(Clone, Debug, Copy)]
pub enum EdgeFeatureName {
    Degree,
    AdamicAdar,
    JaccardCoefficient,
    Cooccurrence,
    ResourceAllocationIndex,
    PreferentialAttachment,
}

impl TryFrom<&str> for EdgeFeatureName {
    type Error = String;
    fn try_from(candidate_edge_feature_name: &str) -> Result<Self, String> {
        match candidate_edge_feature_name {
            "Degree" => Ok(EdgeFeatureName::Degree),
            "AdamicAdar" => Ok(EdgeFeatureName::AdamicAdar),
            "JaccardCoefficient" => Ok(EdgeFeatureName::JaccardCoefficient),
            "Cooccurrence" => Ok(EdgeFeatureName::Cooccurrence),
            "ResourceAllocationIndex" => Ok(EdgeFeatureName::ResourceAllocationIndex),
            "PreferentialAttachment" => Ok(EdgeFeatureName::PreferentialAttachment),
            _ => Err(format!(
                concat!(
                    "The provided edge feature name {} is not supported. ",
                    "The supported edge feature names are `Degree`, `AdamicAdar`, ",
                    "`JaccardCoefficient`, `ResourceAllocationIndex`, and `PreferentialAttachment`.",
                ),
                candidate_edge_feature_name
            )),
        }
    }
}

pub fn get_edge_feature_dimensionality(method: EdgeFeatureName) -> usize {
    match method {
        EdgeFeatureName::Degree => 2,
        EdgeFeatureName::AdamicAdar => 1,
        EdgeFeatureName::JaccardCoefficient => 1,
        EdgeFeatureName::Cooccurrence => 1,
        EdgeFeatureName::ResourceAllocationIndex => 1,
        EdgeFeatureName::PreferentialAttachment => 1,
    }
}

#[derive(Clone, Debug)]
pub struct EdgePredictionFeaturePerceptron {
    /// The name of the method to use to compute the edge embedding.
    edge_feature_name: EdgeFeatureName,
    /// The weights of the model.
    weights: Vec<f64>,
    /// The bias of the model.
    bias: f64,
    /// The number of epochs to train the model for.
    number_of_epochs: usize,
    /// Number of samples in a mini-batch. By default 1024.
    number_of_edges_per_mini_batch: usize,
    /// Whether to train this model by sampling only edges with nodes with different node types.
    sample_only_edges_with_heterogeneous_node_types: bool,
    /// Learning rate to use to train the model.
    learning_rate: f64,
    /// The random state to reproduce the model initialization and training.
    random_state: u64,
    /// Number of iterations to run when computing the cooccurrence metric.
    iterations: u64,
    /// Window size to consider to measure the cooccurrence.
    window_size: u64,
}

impl EdgePredictionFeaturePerceptron {
    /// Return new instance of Perceptron for edge prediction.
    ///
    /// # Arguments
    /// * `edge_feature_name`: Option<EdgeFeatureName> - The embedding method to use. By default the Jaccard is used.
    /// * `number_of_epochs`: Option<usize> - The number of epochs to train the model for. By default, 100.
    /// * `number_of_edges_per_mini_batch`: Option<usize> - The number of samples to include for each mini-batch. By default 1024.
    /// * `sample_only_edges_with_heterogeneous_node_types`: Option<bool> - Whether to sample negative edges only with source and destination nodes that have different node types. By default false.
    /// * `learning_rate`: Option<f64> - Learning rate to use while training the model. By default 0.001.
    /// * `iterations`: Option<u64> - Number of iterations to run when computing the cooccurrence metric. By default 10 when the edge embedding is cooccurrence.
    /// * `window_size`: Option<u64> - Window size to consider to measure the cooccurrence. By default 10 when the edge embedding is cooccurrence.
    /// * `random_state`: Option<u64> - The random state to reproduce the model initialization and training. By default, 42.
    pub fn new(
        edge_feature_name: Option<EdgeFeatureName>,
        number_of_epochs: Option<usize>,
        number_of_edges_per_mini_batch: Option<usize>,
        sample_only_edges_with_heterogeneous_node_types: Option<bool>,
        learning_rate: Option<f64>,
        iterations: Option<u64>,
        window_size: Option<u64>,
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

        let edge_feature_name = edge_feature_name.unwrap_or(EdgeFeatureName::JaccardCoefficient);
        Ok(Self {
            edge_feature_name,
            weights: Vec::new(),
            bias: 0.0,
            number_of_epochs,
            number_of_edges_per_mini_batch,
            sample_only_edges_with_heterogeneous_node_types:
                sample_only_edges_with_heterogeneous_node_types.unwrap_or(false),
            learning_rate,
            iterations: iterations.unwrap_or(10),
            window_size: window_size.unwrap_or(10),
            random_state: random_state.unwrap_or(42),
        })
    }

    /// Returns the weights of the model.
    pub fn get_weights(&self) -> Result<Vec<f64>, String> {
        if self.weights.is_empty() {
            return Err(concat!(
                "This model has not been trained yet. ",
                "You should call the `.fit` method first."
            )
            .to_string());
        }
        Ok(self.weights.clone())
    }

    /// Returns the bias of the model.
    pub fn get_bias(&self) -> Result<f64, String> {
        if self.weights.is_empty() {
            return Err(concat!(
                "This model has not been trained yet. ",
                "You should call the `.fit` method first."
            )
            .to_string());
        }
        Ok(self.bias)
    }

    /// Returns method to compute the edge embedding.
    fn get_edge_feature_method(
        &self,
    ) -> fn(
        model: &EdgePredictionFeaturePerceptron,
        support: &Graph,
        src: NodeT,
        dst: NodeT,
        random_state: u64,
    ) -> Vec<f64> {
        match self.edge_feature_name {
            EdgeFeatureName::Degree => |_model: &EdgePredictionFeaturePerceptron,
                                        support: &Graph,
                                        src: NodeT,
                                        dst: NodeT,
                                        _random_state: u64| {
                let maximum_node_degree =
                    unsafe { support.get_unchecked_maximum_node_degree() as f64 };
                vec![
                    unsafe { support.get_unchecked_node_degree_from_node_id(src) } as f64
                        / maximum_node_degree,
                    unsafe { support.get_unchecked_node_degree_from_node_id(dst) } as f64
                        / maximum_node_degree,
                ]
            },
            EdgeFeatureName::AdamicAdar => {
                |_model: &EdgePredictionFeaturePerceptron,
                 support: &Graph,
                 src: NodeT,
                 dst: NodeT,
                 _random_state: u64| unsafe {
                    vec![support.get_unchecked_adamic_adar_index_from_node_ids(src, dst) as f64]
                }
            }
            EdgeFeatureName::JaccardCoefficient => {
                |_model: &EdgePredictionFeaturePerceptron,
                 support: &Graph,
                 src: NodeT,
                 dst: NodeT,
                 _random_state: u64| unsafe {
                    vec![support.get_unchecked_jaccard_coefficient_from_node_ids(src, dst) as f64]
                }
            }
            EdgeFeatureName::Cooccurrence => {
                |model: &EdgePredictionFeaturePerceptron,
                 support: &Graph,
                 src: NodeT,
                 dst: NodeT,
                 random_state: u64| {
                    let mut random_state = splitmix64(random_state);
                    let mut encounters = 0;
                    (0..model.iterations).for_each(|_| {
                        random_state = splitmix64(random_state);
                        if unsafe {
                            support
                                .iter_uniform_walk(src, random_state, model.window_size)
                                .any(|node| node == dst)
                        } {
                            encounters += 1;
                        }
                    });
                    vec![encounters as f64 / model.iterations as f64]
                }
            }
            EdgeFeatureName::ResourceAllocationIndex => {
                |_model: &EdgePredictionFeaturePerceptron,
                 support: &Graph,
                 src: NodeT,
                 dst: NodeT,
                 _random_state: u64| unsafe {
                    vec![
                        support.get_unchecked_resource_allocation_index_from_node_ids(src, dst)
                            as f64,
                    ]
                }
            }
            EdgeFeatureName::PreferentialAttachment => {
                |_model: &EdgePredictionFeaturePerceptron,
                 support: &Graph,
                 src: NodeT,
                 dst: NodeT,
                 _random_state: u64| unsafe {
                    vec![
                        support.get_unchecked_preferential_attachment_from_node_ids(src, dst, true)
                            as f64,
                    ]
                }
            }
        }
    }

    /// Returns the prediction for the provided nodes, edge embedding method and current model.
    ///
    /// # Arguments
    /// `support`: &Graph - The graph on which to compute the desired features.
    /// `src`: NodeT - The source node whose features are to be extracted.
    /// `dst`: NodeT - The destination node whose features are to be extracted.
    /// `edge_feature_method`: fn - Callback to the edge embedding method to use.
    ///
    /// # Safety
    /// In this method we do not execute any checks such as whether the
    /// node features are compatible with the provided node IDs, and therefore
    /// improper parametrization may lead to panic or undefined behaviour.
    unsafe fn get_unsafe_prediction(
        &self,
        support: &Graph,
        src: NodeT,
        dst: NodeT,
        random_state: u64,
        method: fn(
            model: &EdgePredictionFeaturePerceptron,
            support: &Graph,
            src: NodeT,
            dst: NodeT,
            random_state: u64,
        ) -> Vec<f64>,
    ) -> (Vec<f64>, f64) {
        let edge_feature = method(&self, support, src, dst, random_state);
        let dot = dot_product_sequential_unchecked(&edge_feature, &self.weights) + self.bias;
        (edge_feature, 1.0 / (1.0 + (-dot).exp()))
    }

    /// Fit the edge prediction perceptron model on the provided graph and node features.
    ///
    /// # Arguments
    /// * `graph`: &Graph - The graph whose edges are to be learned.
    /// * `verbose`: Option<bool> - Whether to show a loading bar for the epochs. By default, True.
    /// * `support`: Option<&'a Graph> - Graph to use to check for false negatives and for the edge features. When one is not provided, the `graph` parameter is used.
    /// * `graph_to_avoid`: &'a Option<&Graph> - The graph whose edges are to be avoided during the generation of false negatives,
    pub fn fit(
        &mut self,
        graph: &Graph,
        verbose: Option<bool>,
        support: Option<&Graph>,
        graph_to_avoid: Option<&Graph>,
    ) -> Result<(), String> {
        let mut random_state: u64 = splitmix64(self.random_state);
        let verbose: bool = verbose.unwrap_or(true);
        let support = support.unwrap_or(graph);

        // Initialize the model with weights and bias
        let get_random_weight =
            |seed: usize| (2.0 * random_f64(splitmix64(random_state + seed as u64)) - 1.0);

        let edge_dimension = get_edge_feature_dimensionality(self.edge_feature_name);

        self.weights = (0..edge_dimension)
            .map(|i| get_random_weight(i))
            .collect::<Vec<f64>>();
        self.bias = get_random_weight(self.weights.len());

        // Depending whether verbosity was requested by the user
        // we create or not a visible progress bar to show the progress
        // in the training epochs.
        let progress_bar = if verbose {
            let pb = ProgressBar::new(self.number_of_epochs as u64);
            pb.set_style(ProgressStyle::default_bar().template(concat!(
                "Feature Perceptron ",
                "{spinner:.green} [{elapsed_precise}] ",
                "[{bar:40.cyan/blue}] ({pos}/{len}, ETA {eta})"
            )));
            pb
        } else {
            ProgressBar::hidden()
        };

        let number_of_batches_per_epoch = (graph.get_number_of_directed_edges() as f64
            / self.number_of_edges_per_mini_batch as f64)
            .ceil() as usize;

        let method = self.get_edge_feature_method();
        let batch_learning_rate: f64 =
            self.learning_rate / self.number_of_edges_per_mini_batch as f64;

        // We start to loop over the required amount of epochs.
        (0..self.number_of_epochs)
            .progress_with(progress_bar)
            .map(|_| {
                (0..number_of_batches_per_epoch)
                    .map(|_| {
                        random_state = splitmix64(random_state);
                        let (total_weights_gradient, total_bias_gradient) = graph
                            .par_iter_edge_prediction_mini_batch(
                                random_state,
                                self.number_of_edges_per_mini_batch,
                                self.sample_only_edges_with_heterogeneous_node_types,
                                Some(0.5),
                                Some(true),
                                None,
                                Some(true),
                                Some(support),
                                graph_to_avoid,
                            )?
                            .map(|(src, dst, label)| {
                                let (mut edge_feature, prediction) = unsafe {
                                    self.get_unsafe_prediction(
                                        graph,
                                        src,
                                        dst,
                                        random_state,
                                        method,
                                    )
                                };

                                let variation = if label { prediction - 1.0 } else { prediction };

                                edge_feature.iter_mut().for_each(|edge_feature| {
                                    *edge_feature *= variation;
                                });

                                (edge_feature, variation)
                            })
                            .reduce(
                                || (vec![0.0; edge_dimension], 0.0),
                                |(mut total_weights_gradient, mut total_bias_gradient): (
                                    Vec<f64>,
                                    f64,
                                ),
                                 (partial_weights_gradient, partial_bias_gradient): (
                                    Vec<f64>,
                                    f64,
                                )| {
                                    total_weights_gradient
                                        .iter_mut()
                                        .zip(partial_weights_gradient.into_iter())
                                        .for_each(
                                            |(total_weight_gradient, partial_weight_gradient)| {
                                                *total_weight_gradient += partial_weight_gradient;
                                            },
                                        );
                                    total_bias_gradient += partial_bias_gradient;
                                    (total_weights_gradient, total_bias_gradient)
                                },
                            );
                        self.bias -= total_bias_gradient * batch_learning_rate;
                        self.weights
                            .par_iter_mut()
                            .zip(total_weights_gradient.into_par_iter())
                            .for_each(|(weight, total_weight_gradient)| {
                                *weight -= total_weight_gradient * batch_learning_rate;
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
    /// * `support`: Option<&Graph> - The graph to use for the edge features. When one is not provided, the `graph` parameter is used,
    pub fn predict(
        &self,
        predictions: &mut [f32],
        graph: &Graph,
        support: Option<&Graph>,
    ) -> Result<(), String> {
        let support = support.unwrap_or(graph);

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

        let edge_dimension = get_edge_feature_dimensionality(self.edge_feature_name);

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

        let method = self.get_edge_feature_method();
        let random_state = splitmix64(self.random_state);

        predictions
            .par_iter_mut()
            .zip(graph.par_iter_directed_edge_node_ids())
            .for_each(|(prediction, (_, src, dst))| {
                *prediction = unsafe {
                    self.get_unsafe_prediction(support, src, dst, random_state, method)
                        .1
                } as f32;
            });

        Ok(())
    }
}
