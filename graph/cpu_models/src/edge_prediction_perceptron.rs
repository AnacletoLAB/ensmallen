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

#[derive(Clone, Debug, Copy, PartialEq, EnumIter, Deserialize, Serialize)]
pub enum EdgeEmbedding {
    CosineSimilarity,
    EuclideanDistance,
    Concatenate,
    Hadamard,
    L1,
    L2,
    Add,
    Sub,
    Maximum,
    Minimum,
}

impl std::fmt::Display for EdgeEmbedding {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl<'a> TryFrom<&'a str> for EdgeEmbedding {
    type Error = String;
    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        for edge_embedding in EdgeEmbedding::iter() {
            if edge_embedding.to_string().as_str() == value {
                return Ok(edge_embedding);
            }
        }
        Err(format!(
            concat!(
                "The provided edge embedding candidate {} ",
                "is not supported. The supported edge embedding ",
                "method are {:?}."
            ),
            value,
            EdgeEmbedding::get_edge_embedding_methods()
        ))
    }
}

impl TryFrom<String> for EdgeEmbedding {
    type Error = String;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        EdgeEmbedding::try_from(value.as_str())
    }
}

impl EdgeEmbedding {
    /// Returns dimensionality of the edge embedding.
    ///
    /// # Arguments
    /// * `dimension`: usize - The dimension of the edge embedding.
    pub fn get_dimensionality(&self, dimension: usize) -> usize {
        match self {
            EdgeEmbedding::CosineSimilarity => 1,
            EdgeEmbedding::EuclideanDistance => 1,
            EdgeEmbedding::Hadamard => dimension,
            EdgeEmbedding::Concatenate => 2 * dimension,
            EdgeEmbedding::L1 => dimension,
            EdgeEmbedding::L2 => dimension,
            EdgeEmbedding::Add => dimension,
            EdgeEmbedding::Sub => dimension,
            EdgeEmbedding::Maximum => dimension,
            EdgeEmbedding::Minimum => dimension,
        }
    }

    pub fn get_edge_embedding_methods() -> Vec<EdgeEmbedding> {
        EdgeEmbedding::iter()
            .map(|edge_embedding| edge_embedding)
            .collect()
    }

    pub fn get_edge_embedding_method_names() -> Vec<String> {
        EdgeEmbedding::iter()
            .map(|edge_embedding| edge_embedding.to_string())
            .collect()
    }

    pub fn get_method<F>(&self) -> fn(&[F], &[F]) -> Vec<f32>
    where
        F: AsPrimitive<f32> + Copy + Sub<Output = F> + PartialOrd,
    {
        match self {
            EdgeEmbedding::CosineSimilarity => {
                |a: &[F], b: &[F]| vec![unsafe { cosine_similarity_sequential_unchecked(a, b).0 }]
            }
            EdgeEmbedding::EuclideanDistance => {
                |a: &[F], b: &[F]| vec![unsafe { euclidean_distance_sequential_unchecked(a, b) }]
            }
            EdgeEmbedding::Hadamard => |a: &[F], b: &[F]| {
                a.iter()
                    .copied()
                    .zip(b.iter().copied())
                    .map(|(feature_a, feature_b)| feature_a.as_() * feature_b.as_())
                    .collect::<Vec<f32>>()
            },
            EdgeEmbedding::Concatenate => |a: &[F], b: &[F]| {
                a.iter()
                    .copied()
                    .chain(b.iter().copied())
                    .map(|feature| feature.as_())
                    .collect::<Vec<f32>>()
            },
            EdgeEmbedding::L1 => |a: &[F], b: &[F]| {
                a.iter()
                    .copied()
                    .zip(b.iter().copied())
                    .map(|(feature_a, feature_b)| absolute_distance(feature_a, feature_b).as_())
                    .collect::<Vec<f32>>()
            },
            EdgeEmbedding::L2 => |a: &[F], b: &[F]| {
                a.iter()
                    .copied()
                    .zip(b.iter().copied())
                    .map(|(feature_a, feature_b)| {
                        let l1 = absolute_distance(feature_a, feature_b).as_();
                        l1 * l1
                    })
                    .collect::<Vec<f32>>()
            },
            EdgeEmbedding::Add => |a: &[F], b: &[F]| {
                a.iter()
                    .copied()
                    .zip(b.iter().copied())
                    .map(|(feature_a, feature_b)| feature_a.as_() + feature_b.as_())
                    .collect::<Vec<f32>>()
            },
            EdgeEmbedding::Sub => |a: &[F], b: &[F]| {
                a.iter()
                    .zip(b.iter())
                    .map(|(&first_feature, &second_feature)| {
                        first_feature.as_() - second_feature.as_()
                    })
                    .collect()
            },
            EdgeEmbedding::Maximum => |a: &[F], b: &[F]| {
                a.iter()
                    .copied()
                    .zip(b.iter().copied())
                    .map(|(feature_a, feature_b)| feature_a.as_().max(feature_b.as_()))
                    .collect::<Vec<f32>>()
            },
            EdgeEmbedding::Minimum => |a: &[F], b: &[F]| {
                a.iter()
                    .copied()
                    .zip(b.iter().copied())
                    .map(|(feature_a, feature_b)| feature_a.as_().min(feature_b.as_()))
                    .collect::<Vec<f32>>()
            },
        }
    }

    pub fn embed<F>(&self, source_feature: &[F], destination_features: &[F]) -> Vec<f32>
    where
        F: AsPrimitive<f32> + Copy + Sub<Output = F> + PartialOrd,
    {
        self.get_method()(source_feature, destination_features)
    }
}

#[derive(Clone, Debug, Copy, PartialEq, EnumIter, Deserialize, Serialize)]
pub enum EdgeFeature {
    Degree,
    LogDegree,
    AdamicAdar,
    JaccardCoefficient,
    Cooccurrence,
    ResourceAllocationIndex,
    PreferentialAttachment,
}

impl std::fmt::Display for EdgeFeature {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl<'a> TryFrom<&'a str> for EdgeFeature {
    type Error = String;
    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        for edge_feature in EdgeFeature::iter() {
            if edge_feature.to_string().as_str() == value {
                return Ok(edge_feature);
            }
        }
        Err(format!(
            concat!(
                "The provided edge features candidate {} ",
                "is not supported. The supported edge features ",
                "method are {:?}."
            ),
            value,
            EdgeFeature::get_edge_feature_methods()
        ))
    }
}

impl TryFrom<String> for EdgeFeature {
    type Error = String;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        EdgeFeature::try_from(value.as_str())
    }
}

impl EdgeFeature {
    /// Returns dimensionality of the edge feature.
    pub fn get_dimensionality(&self) -> usize {
        match &self {
            EdgeFeature::Degree => 2,
            EdgeFeature::LogDegree => 2,
            EdgeFeature::AdamicAdar => 1,
            EdgeFeature::JaccardCoefficient => 1,
            EdgeFeature::Cooccurrence => 1,
            EdgeFeature::ResourceAllocationIndex => 1,
            EdgeFeature::PreferentialAttachment => 1,
        }
    }

    pub fn get_edge_feature_methods() -> Vec<EdgeFeature> {
        EdgeFeature::iter()
            .map(|edge_feature| edge_feature)
            .collect()
    }

    pub fn get_edge_feature_method_names() -> Vec<String> {
        EdgeEmbedding::iter()
            .map(|edge_embedding| edge_embedding.to_string())
            .collect()
    }

    /// Returns method to compute the edge embedding.
    fn get_method<O1: Optimizer<f32, T = f32>, O2: Optimizer<Vec<f32>, T = [f32]>>(
        &self,
    ) -> fn(
        model: &EdgePredictionPerceptron<O1, O2>,
        support: &Graph,
        src: NodeT,
        dst: NodeT,
        random_state: u64,
    ) -> Vec<f32> {
        match self {
            EdgeFeature::Degree => |_model: &EdgePredictionPerceptron<O1, O2>,
                                    support: &Graph,
                                    src: NodeT,
                                    dst: NodeT,
                                    _random_state: u64| {
                let maximum_node_degree =
                    unsafe { support.get_unchecked_maximum_node_degree() as f32 } + 1.0;
                vec![
                    unsafe { support.get_unchecked_node_degree_from_node_id(src) } as f32
                        / maximum_node_degree,
                    unsafe { support.get_unchecked_node_degree_from_node_id(dst) } as f32
                        / maximum_node_degree,
                ]
            },
            EdgeFeature::LogDegree => |_model: &EdgePredictionPerceptron<O1, O2>,
                                       support: &Graph,
                                       src: NodeT,
                                       dst: NodeT,
                                       _random_state: u64| {
                vec![
                    1.0 / unsafe {
                        support.get_unchecked_node_degree_from_node_id(src) as f32
                            + std::f32::consts::E
                    }
                    .ln(),
                    1.0 / unsafe {
                        support.get_unchecked_node_degree_from_node_id(dst) as f32
                            + std::f32::consts::E
                    }
                    .ln(),
                ]
            },
            EdgeFeature::AdamicAdar => |_model: &EdgePredictionPerceptron<O1, O2>,
                                        support: &Graph,
                                        src: NodeT,
                                        dst: NodeT,
                                        _random_state: u64| unsafe {
                vec![support.get_unchecked_adamic_adar_index_from_node_ids(src, dst)]
            },
            EdgeFeature::JaccardCoefficient => {
                |_model: &EdgePredictionPerceptron<O1, O2>,
                 support: &Graph,
                 src: NodeT,
                 dst: NodeT,
                 _random_state: u64| unsafe {
                    vec![support.get_unchecked_jaccard_coefficient_from_node_ids(src, dst)]
                }
            }
            EdgeFeature::Cooccurrence => {
                |model: &EdgePredictionPerceptron<O1, O2>,
                 support: &Graph,
                 src: NodeT,
                 dst: NodeT,
                 random_state: u64| {
                    let mut random_state = splitmix64(random_state);
                    let mut encounters = 0;
                    (0..model.cooccurrence_iterations).for_each(|_| {
                        random_state = splitmix64(random_state);
                        if unsafe {
                            support
                                .iter_uniform_walk(
                                    src,
                                    random_state,
                                    model.cooccurrence_window_size,
                                )
                                .any(|node| node == dst)
                        } {
                            encounters += 1;
                        }
                    });
                    vec![encounters as f32 / model.cooccurrence_iterations as f32]
                }
            }
            EdgeFeature::ResourceAllocationIndex => {
                |_model: &EdgePredictionPerceptron<O1, O2>,
                 support: &Graph,
                 src: NodeT,
                 dst: NodeT,
                 _random_state: u64| unsafe {
                    vec![support.get_unchecked_resource_allocation_index_from_node_ids(src, dst)]
                }
            }
            EdgeFeature::PreferentialAttachment => {
                |_model: &EdgePredictionPerceptron<O1, O2>,
                 support: &Graph,
                 src: NodeT,
                 dst: NodeT,
                 _random_state: u64| unsafe {
                    vec![support.get_unchecked_preferential_attachment_from_node_ids(src, dst, true)]
                }
            }
        }
    }

    pub fn embed<
        O1: Optimizer<f32, T = f32>,
        O2: Optimizer<Vec<f32>, T = [f32]>,
        F: AsPrimitive<f32>,
    >(
        &self,
        model: &EdgePredictionPerceptron<O1, O2>,
        support: &Graph,
        src: NodeT,
        dst: NodeT,
        random_state: u64,
    ) -> Vec<f32> {
        self.get_method()(model, support, src, dst, random_state)
    }
}

#[derive(Clone, Deserialize, Serialize)]
pub struct EdgePredictionPerceptron<O1, O2> {
    /// The edge embedding methods to use.
    edge_embeddings: Vec<EdgeEmbedding>,
    /// The edge feature methods to use.
    edge_features: Vec<EdgeFeature>,
    /// Bias Optimizer
    bias_optimizer: O1,
    /// Weights optimizer
    weight_optimizer: O2,
    /// The weights of the model.
    weights: Vec<f32>,
    /// The bias of the model.
    bias: f32,
    /// Whether to avoid sampling false negatives. This may cause a slower training.
    avoid_false_negatives: bool,
    /// The number of epochs to train the model for.
    number_of_epochs: usize,
    /// Number of samples in a mini-batch. By default 1024.
    number_of_edges_per_mini_batch: usize,
    /// Whether to train this model by sampling only edges with nodes with different node types.
    sample_only_edges_with_heterogeneous_node_types: bool,
    /// Number of iterations to run when computing the cooccurrence metric.
    cooccurrence_iterations: u64,
    /// Window size to consider to measure the cooccurrence.
    cooccurrence_window_size: u64,
    /// Whether to sample using scale free distribution.
    use_scale_free_distribution: bool,
    /// Precomputed boolean representing whether the model has only a single embedding.
    has_single_embedding: bool,
    /// The random state to reproduce the model initialization and training.
    random_state: u64,
}

impl<O1, O2> EdgePredictionPerceptron<O1, O2>
where
    O1: Optimizer<f32, T = f32> + Serialize + From<O2> + DeserializeOwned,
    O2: Optimizer<Vec<f32>, T = [f32]> + Serialize + DeserializeOwned,
{
    /// Return new instance of Perceptron for edge prediction.
    ///
    /// # Arguments
    /// * `edge_embeddings`: Vec<EdgeEmbedding> - The embedding methods to use for the provided node features.
    /// * `edge_features`: Vec<EdgeFeature> - The edge features to compute for each edge.
    /// * `optimizer`: Optimizer - The optimizer to be used for the training.
    /// * `avoid_false_negatives`: Option<bool> - Whether to avoid sampling false negatives. This may cause a slower training..
    /// * `cooccurrence_iterations`: Option<u64> - Number of iterations to run when computing the cooccurrence metric. By default `100`.
    /// * `cooccurrence_window_size`: Option<u64> - Window size to consider to measure the cooccurrence. By default `10`.
    /// * `number_of_epochs`: Option<usize> - The number of epochs to train the model for. By default, `100`.
    /// * `number_of_edges_per_mini_batch`: Option<usize> - The number of samples to include for each mini-batch. By default `256`.
    /// * `sample_only_edges_with_heterogeneous_node_types`: Option<bool> - Whether to sample negative edges only with source and destination nodes that have different node types. By default false.
    /// * `use_scale_free_distribution`: Option<bool> - Whether to sample using scale free distribution. By default, true.
    /// * `random_state`: Option<u64> - The random state to reproduce the model initialization and training. By default, `42`.
    pub fn new(
        edge_embeddings: Vec<EdgeEmbedding>,
        edge_features: Vec<EdgeFeature>,
        optimizer: O2,
        avoid_false_negatives: Option<bool>,
        cooccurrence_iterations: Option<u64>,
        cooccurrence_window_size: Option<u64>,
        number_of_epochs: Option<usize>,
        number_of_edges_per_mini_batch: Option<usize>,
        sample_only_edges_with_heterogeneous_node_types: Option<bool>,
        use_scale_free_distribution: Option<bool>,
        random_state: Option<u64>,
    ) -> Result<Self, String> {
        let number_of_epochs = must_not_be_zero(number_of_epochs, 100, "number of epochs")?;
        let number_of_edges_per_mini_batch = must_not_be_zero(
            number_of_edges_per_mini_batch,
            256,
            "number of edges per mini-batch",
        )?;
        let cooccurrence_iterations =
            must_not_be_zero(cooccurrence_iterations, 100, "cooccurrence iterations")?;
        let cooccurrence_window_size =
            must_not_be_zero(cooccurrence_window_size, 10, "cooccurrence window size")?;

        if edge_features.is_empty() && edge_embeddings.is_empty() {
            return Err(concat!(
                "No edge feature or embedding was selected, and it ",
                "is not possible to train a model without input features."
            )
            .to_string());
        }

        Ok(Self {
            has_single_embedding: edge_embeddings.len() == 1 && edge_features.is_empty(),
            edge_embeddings,
            edge_features,
            bias_optimizer: optimizer.clone().into(),
            weight_optimizer: optimizer,
            cooccurrence_iterations,
            cooccurrence_window_size,
            weights: Vec::new(),
            bias: 0.0,
            avoid_false_negatives: avoid_false_negatives.unwrap_or(false),
            number_of_epochs,
            number_of_edges_per_mini_batch,
            sample_only_edges_with_heterogeneous_node_types:
                sample_only_edges_with_heterogeneous_node_types.unwrap_or(false),
            use_scale_free_distribution: use_scale_free_distribution.unwrap_or(true),
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
    pub fn get_weights(&self) -> Result<Vec<f32>, String> {
        self.must_be_trained().map(|_| self.weights.clone())
    }

    /// Returns the bias of the model.
    pub fn get_bias(&self) -> Result<f32, String> {
        self.must_be_trained().map(|_| self.bias)
    }

    fn validate_features(
        &self,
        graph: &Graph,
        node_features: &[FeatureSlice],
        dimensions: &[usize],
    ) -> Result<(), String> {
        if !self.edge_embeddings.is_empty() && node_features.is_empty() {
            return Err(format!(
                concat!(
                    "This edge prediction perceptron expected node features ",
                    "as some edge embedding procedure has been specified {:?}, ",
                    "yet you have provided no node features."
                ),
                self.edge_embeddings,
            ));
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

        if !graph.has_edges() {
            return Err("The provided graph does not have any edge.".to_string());
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

    /// Returns the edge embedding for the provided input.
    ///
    /// # Arguments
    /// `src`: NodeT - The source node whose features are to be extracted.
    /// `dst`: NodeT - The destination node whose features are to be extracted.
    /// `support`: &Graph - The support graph to use for the topological features.
    /// `node_features`: &[&[F]] - The node features to use.
    /// `dimensions`: &[usize] - The dimension of the provided node features.
    ///
    /// # Safety
    /// In this method we do not execute any checks such as whether the
    /// node features are compatible with the provided node IDs, and therefore
    /// improper parametrization may lead to panic or undefined behaviour.
    unsafe fn get_unsafe_edge_embedding(
        &self,
        src: NodeT,
        dst: NodeT,
        support: &Graph,
        node_features: &[FeatureSlice],
        dimensions: &[usize],
    ) -> Vec<f32> {
        use crate::FeatureSlice::*;
        if self.has_single_embedding && node_features.len() == 1 {
            let dimension = dimensions[0];
            let edge_embedding = self.edge_embeddings[0];
            return match node_features[0] {
                F16(feature) => edge_embedding.get_method()(
                    &feature[(src as usize) * dimension..((src as usize) + 1) * dimension],
                    &feature[(dst as usize) * dimension..((dst as usize) + 1) * dimension],
                ),
                F32(feature) => edge_embedding.get_method()(
                    &feature[(src as usize) * dimension..((src as usize) + 1) * dimension],
                    &feature[(dst as usize) * dimension..((dst as usize) + 1) * dimension],
                ),
                F64(feature) => edge_embedding.get_method()(
                    &feature[(src as usize) * dimension..((src as usize) + 1) * dimension],
                    &feature[(dst as usize) * dimension..((dst as usize) + 1) * dimension],
                ),
                U8(feature) => edge_embedding.get_method()(
                    &feature[(src as usize) * dimension..((src as usize) + 1) * dimension],
                    &feature[(dst as usize) * dimension..((dst as usize) + 1) * dimension],
                ),
                U16(feature) => edge_embedding.get_method()(
                    &feature[(src as usize) * dimension..((src as usize) + 1) * dimension],
                    &feature[(dst as usize) * dimension..((dst as usize) + 1) * dimension],
                ),
                U32(feature) => edge_embedding.get_method()(
                    &feature[(src as usize) * dimension..((src as usize) + 1) * dimension],
                    &feature[(dst as usize) * dimension..((dst as usize) + 1) * dimension],
                ),
                U64(feature) => edge_embedding.get_method()(
                    &feature[(src as usize) * dimension..((src as usize) + 1) * dimension],
                    &feature[(dst as usize) * dimension..((dst as usize) + 1) * dimension],
                ),
                I8(feature) => edge_embedding.get_method()(
                    &feature[(src as usize) * dimension..((src as usize) + 1) * dimension],
                    &feature[(dst as usize) * dimension..((dst as usize) + 1) * dimension],
                ),
                I16(feature) => edge_embedding.get_method()(
                    &feature[(src as usize) * dimension..((src as usize) + 1) * dimension],
                    &feature[(dst as usize) * dimension..((dst as usize) + 1) * dimension],
                ),
                I32(feature) => edge_embedding.get_method()(
                    &feature[(src as usize) * dimension..((src as usize) + 1) * dimension],
                    &feature[(dst as usize) * dimension..((dst as usize) + 1) * dimension],
                ),
                I64(feature) => edge_embedding.get_method()(
                    &feature[(src as usize) * dimension..((src as usize) + 1) * dimension],
                    &feature[(dst as usize) * dimension..((dst as usize) + 1) * dimension],
                ),
            };
        };

        node_features
            .iter()
            .zip(dimensions.iter().copied())
            .flat_map(|(node_feature, dimension)| {
                self.edge_embeddings
                    .iter()
                    .flat_map(move |edge_embedding| match node_feature {
                        F16(feature) => edge_embedding.get_method()(
                            &feature[(src as usize) * dimension..((src as usize) + 1) * dimension],
                            &feature[(dst as usize) * dimension..((dst as usize) + 1) * dimension],
                        ),
                        F32(feature) => edge_embedding.get_method()(
                            &feature[(src as usize) * dimension..((src as usize) + 1) * dimension],
                            &feature[(dst as usize) * dimension..((dst as usize) + 1) * dimension],
                        ),
                        F64(feature) => edge_embedding.get_method()(
                            &feature[(src as usize) * dimension..((src as usize) + 1) * dimension],
                            &feature[(dst as usize) * dimension..((dst as usize) + 1) * dimension],
                        ),
                        U8(feature) => edge_embedding.get_method()(
                            &feature[(src as usize) * dimension..((src as usize) + 1) * dimension],
                            &feature[(dst as usize) * dimension..((dst as usize) + 1) * dimension],
                        ),
                        U16(feature) => edge_embedding.get_method()(
                            &feature[(src as usize) * dimension..((src as usize) + 1) * dimension],
                            &feature[(dst as usize) * dimension..((dst as usize) + 1) * dimension],
                        ),
                        U32(feature) => edge_embedding.get_method()(
                            &feature[(src as usize) * dimension..((src as usize) + 1) * dimension],
                            &feature[(dst as usize) * dimension..((dst as usize) + 1) * dimension],
                        ),
                        U64(feature) => edge_embedding.get_method()(
                            &feature[(src as usize) * dimension..((src as usize) + 1) * dimension],
                            &feature[(dst as usize) * dimension..((dst as usize) + 1) * dimension],
                        ),
                        I8(feature) => edge_embedding.get_method()(
                            &feature[(src as usize) * dimension..((src as usize) + 1) * dimension],
                            &feature[(dst as usize) * dimension..((dst as usize) + 1) * dimension],
                        ),
                        I16(feature) => edge_embedding.get_method()(
                            &feature[(src as usize) * dimension..((src as usize) + 1) * dimension],
                            &feature[(dst as usize) * dimension..((dst as usize) + 1) * dimension],
                        ),
                        I32(feature) => edge_embedding.get_method()(
                            &feature[(src as usize) * dimension..((src as usize) + 1) * dimension],
                            &feature[(dst as usize) * dimension..((dst as usize) + 1) * dimension],
                        ),
                        I64(feature) => edge_embedding.get_method()(
                            &feature[(src as usize) * dimension..((src as usize) + 1) * dimension],
                            &feature[(dst as usize) * dimension..((dst as usize) + 1) * dimension],
                        ),
                    })
            })
            .chain(self.edge_features.iter().flat_map(|edge_feature| {
                edge_feature.get_method()(self, support, src, dst, self.random_state)
            }))
            .collect()
    }

    /// Returns the prediction for the provided nodes, edge embedding method and current model.
    ///
    /// # Arguments
    /// `src`: NodeT - The source node whose features are to be extracted.
    /// `dst`: NodeT - The destination node whose features are to be extracted.
    /// `support`: &Graph - The support graph to use for the topological features.
    /// `node_features`: &[&Vec<f32>] - The node features to use.
    /// `dimensions`: &[usize] - The dimension of the provided node features.
    ///
    /// # Safety
    /// In this method we do not execute any checks such as whether the
    /// node features are compatible with the provided node IDs, and therefore
    /// improper parametrization may lead to panic or undefined behaviour.
    unsafe fn get_unsafe_prediction(
        &self,
        src: NodeT,
        dst: NodeT,
        support: &Graph,
        node_features: &[FeatureSlice],
        dimensions: &[usize],
    ) -> (Vec<f32>, f32) {
        let edge_embedding =
            self.get_unsafe_edge_embedding(src, dst, support, node_features, dimensions);
        let dot = dot_product_sequential_unchecked(&edge_embedding, &self.weights) + self.bias;
        (edge_embedding, 1.0 / (1.0 + (-dot).exp()))
    }

    /// Fit the edge prediction perceptron model on the provided graph and node features.
    ///
    /// # Arguments
    /// * `graph`: &Graph - The graph whose edges are to be learned.
    /// * `node_features`: &[&Vec<f32>] - List of node features matrices.
    /// * `dimensions`: &[usize] - The dimensionality of the node features.
    /// * `support`: Option<&Graph> - Graph to use for the topological features.
    /// * `verbose`: Option<bool> - Whether to show a loading bar for the epochs. By default, True.
    /// * `graph_to_avoid`: &'a Option<&Graph> - The graph whose edges are to be avoided during the generation of false negatives,
    pub fn fit(
        &mut self,
        graph: &Graph,
        node_features: &[FeatureSlice],
        dimensions: &[usize],
        verbose: Option<bool>,
        support: Option<&Graph>,
        graph_to_avoid: Option<&Graph>,
    ) -> Result<(), String> {
        let support = support.unwrap_or(graph);
        self.validate_features(support, node_features, dimensions)?;

        let mut random_state: u64 = splitmix64(self.random_state);
        let verbose: bool = verbose.unwrap_or(true);
        let edge_embedding_dimension =
            unsafe { self.get_unsafe_edge_embedding(0, 0, support, node_features, dimensions) }
                .len();

        self.bias_optimizer.set_capacity(1);
        self.weight_optimizer.set_capacity(edge_embedding_dimension);
        let edge_embedding_dimension_root = (edge_embedding_dimension as f32).sqrt();

        if self.weights.len() != edge_embedding_dimension {
            self.weights = (0..edge_embedding_dimension)
            .map(|i| get_random_weight(i as u64, edge_embedding_dimension_root))
            .collect::<Vec<f32>>();
            self.bias = get_random_weight(self.weights.len() as u64, 1.0);
        }

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

        let number_of_batches_per_epoch = (graph.get_number_of_directed_edges() as f32
            / self.number_of_edges_per_mini_batch as f32)
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
                        .map(|(_, src, dst, label)| {
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

        let edge_embedding_dimension =
            unsafe { self.get_unsafe_edge_embedding(0, 0, support, node_features, dimensions) }
                .len();

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
