use crate::{get_random_weight, must_not_be_zero, FeatureSlice};
use crate::{NodeLabelPredictionPerceptron, Optimizer};
use graph::Graph;
use half::f16;
use indicatif::ProgressIterator;
use indicatif::{ProgressBar, ProgressStyle};
use num_traits::AsPrimitive;
use rayon::prelude::*;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use vec_rand::splitmix64;

#[derive(Clone, Deserialize, Serialize)]
pub struct DistanceNodeLabelPredictionPerceptron<O> {
    /// Vector where we store the centroids of the clusters.
    centroids: Vec<f32>,
    /// The number of clusters to compute for each of the classes.
    number_of_clusters_per_class: usize,
    /// The perceptron model to train it on.
    perceptron: NodeLabelPredictionPerceptron<O>,
}

impl<O> DistanceNodeLabelPredictionPerceptron<O>
where
    O: Optimizer<Vec<f32>, T = [f32]> + Serialize + DeserializeOwned,
{
    /// Return new instance of Perceptron for edge prediction.
    ///
    /// # Arguments
    /// * `optimizer`: Optimizer - The optimizer to be used for the training.
    /// * `number_of_epochs`: Option<usize> - The number of epochs to train the model for. By default, `100`.
    /// * `number_of_clusters_per_class`: Option<usize> - The number of clusters to compute for each class. By default, `1`.
    /// * `random_state`: Option<u64> - The random state to reproduce the model initialization and training. By default, `42`.
    pub fn new(
        optimizer: O,
        number_of_epochs: Option<usize>,
        number_of_clusters_per_class: Option<usize>,
        random_state: Option<u64>,
    ) -> Result<Self, String> {
        Ok(Self {
            perceptron: NodeLabelPredictionPerceptron::new(
                optimizer,
                number_of_epochs,
                random_state,
            )?,
            number_of_clusters_per_class: must_not_be_zero(
                number_of_clusters_per_class,
                1,
                "number of clusters per class",
            )?,
            centroids: Vec::new(),
        })
    }

    /// Returns the weights of the model.
    pub fn get_weights(&self) -> Result<Vec<Vec<f32>>, String> {
        self.perceptron.get_weights()
    }

    /// Returns the bias of the model.
    pub fn get_bias(&self) -> Result<Vec<f32>, String> {
        self.perceptron.get_bias()
    }

    /// Returns the number of outputs.
    pub fn get_number_of_outputs(&self) -> Result<usize, String> {
        self.perceptron.get_number_of_outputs()
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
        self.perceptron.validate_features(graph, node_features, dimensions)?;

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
    ) -> Result<(), String> {
        self.perceptron.validate_features(graph, node_features, dimensions)?;
        self.perceptron.must_be_trained()?;

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
