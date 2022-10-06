use crate::{must_not_be_zero, FeatureSlice};
use crate::{NodeLabelPredictionPerceptron, Optimizer};
use express_measures::cosine_similarity_sequential_unchecked_from_iter;
use graph::{Graph, NodeT};
use indicatif::ProgressIterator;
use indicatif::{ProgressBar, ProgressStyle};
use num_traits::AsPrimitive;
use rayon::prelude::*;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use vec_rand::{sorted_unique_sub_sampling, splitmix64};

#[derive(Clone, Deserialize, Serialize)]
pub struct DistanceNodeLabelPredictionPerceptron<O> {
    /// Vector where we store the centroids of the clusters.
    centroids: Vec<f32>,
    /// The number of clusters to compute for each of the classes.
    number_of_centroids_per_class: usize,
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
    /// * `number_of_centroids_per_class`: Option<usize> - The number of clusters to compute for each class. By default, `1`.
    /// * `random_state`: Option<u64> - The random state to reproduce the model initialization and training. By default, `42`.
    pub fn new(
        optimizer: O,
        number_of_epochs: Option<usize>,
        number_of_centroids_per_class: Option<usize>,
        random_state: Option<u64>,
    ) -> Result<Self, String> {
        Ok(Self {
            perceptron: NodeLabelPredictionPerceptron::new(
                optimizer,
                number_of_epochs,
                random_state,
            )?,
            number_of_centroids_per_class: must_not_be_zero(
                number_of_centroids_per_class,
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

    /// Returns the random state.
    pub fn get_random_state(&self) -> u64 {
        self.perceptron.get_random_state()
    }
    
    fn compute_similarities(
        &self,
        graph: &Graph,
        node_features: &[FeatureSlice],
        dimensions: &[usize],
        verbose: Option<bool>,
    ) -> Result<(Vec<f32>, usize), String> {
        // Then we compute the distances of each node to all centroids.
        let features_per_node = self.number_of_centroids_per_class as usize * graph.get_number_of_node_types()? as usize;
        let total_dimensions = dimensions.iter().sum::<usize>();
        let mut node_distances = vec![0.0; graph.get_number_of_nodes() as usize * features_per_node];
        graph
            .par_iter_node_ids()
            .zip(node_distances.par_chunks_mut(features_per_node))
            .for_each(|(node_id, node_distances)| {
                self.centroids
                    .chunks(total_dimensions)
                    .zip(node_distances.iter_mut())
                    .for_each(move |(centroid, node_distance)| unsafe {
                        *node_distance = cosine_similarity_sequential_unchecked_from_iter(
                            centroid.iter().copied(),
                            NodeLabelPredictionPerceptron::<O>::iterate_feature(
                                node_id as usize,
                                node_features,
                                dimensions,
                            ),
                        )
                        .0;
                    });
            });
        
        Ok((node_distances, features_per_node))
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
        self.perceptron
            .validate_features(graph, node_features, dimensions)?;

        let total_dimensions = dimensions.iter().sum::<usize>();

        // First we compute the labels clusters.
        let mut centroids: Vec<f32> = vec![
            0.0;
            total_dimensions
                * self.number_of_centroids_per_class
                * graph.get_number_of_node_types()? as usize
        ];
        graph
            .par_iter_unique_node_type_ids()?
            .zip(centroids.par_chunks_mut(total_dimensions * self.number_of_centroids_per_class))
            .map(|(node_type_id, node_type_centroids)| {
                // First we sample `number_of_centroids_per_class` nodes for this
                // class and initialize the node type centroids with their features.
                let number_of_nodes_in_class =
                    unsafe { graph.get_unchecked_number_of_nodes_from_node_type_id(node_type_id) };

                // We get the k unique indices, and we proceed to retrieve the curresponding
                // k node IDs from where we will be growing these clusters.
                let indices = sorted_unique_sub_sampling(
                    0,
                    number_of_nodes_in_class as u64,
                    self.number_of_centroids_per_class as u64,
                    splitmix64(
                        self.get_random_state() + self.get_random_state() * node_type_id as u64,
                    ),
                )?;

                // We retrieve the node IDs curresponding to the indices
                let node_ids = graph
                    .iter_node_ids_from_node_type_id(node_type_id)?
                    .enumerate()
                    .filter_map(|(index, node_id)| {
                        if indices.contains(&(index as u64)) {
                            Some(node_id)
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<NodeT>>();

                // We initialize the centroids to the features of these nodes.
                node_type_centroids
                    .chunks_mut(total_dimensions)
                    .zip(node_ids.into_iter())
                    .for_each(|(node_type_centroid, node_id)| {
                        NodeLabelPredictionPerceptron::<O>::iterate_feature(node_id as usize, node_features, dimensions)
                            .zip(node_type_centroid.iter_mut())
                            .for_each(|(feature, target)| {
                                *target = feature;
                            });
                    });

                // We initialize the size of the clusters
                let mut cluster_sizes = vec![1.0; self.number_of_centroids_per_class];

                // We start to iterate across all nodes of this feature, and we
                // update the centroid according to which node is most similar
                // according to a cosine similarity.
                graph
                    .iter_node_ids_from_node_type_id(node_type_id)?
                    .for_each(|node_id| {
                        // We find which of the centroids is closest to the node.
                        let closest_centroid_number = node_type_centroids
                            .chunks(total_dimensions)
                            .enumerate()
                            .map(|(centroid_number, node_type_centroid)| unsafe {
                                let similarity: f32 =
                                    cosine_similarity_sequential_unchecked_from_iter(
                                        node_type_centroid.iter().copied(),
                                        NodeLabelPredictionPerceptron::<O>::iterate_feature(
                                            node_id as usize,
                                            node_features,
                                            dimensions,
                                        ),
                                    )
                                    .0;
                                (centroid_number, similarity)
                            })
                            .max_by(|(_, a), (_, y)| a.partial_cmp(y).unwrap())
                            .unwrap()
                            .0;

                        // We update the features of the centroid closest to the current node.
                        node_type_centroids[total_dimensions * closest_centroid_number
                            ..(closest_centroid_number + 1) * total_dimensions]
                            .iter_mut()
                            .zip(NodeLabelPredictionPerceptron::<O>::iterate_feature(
                                node_id as usize,
                                node_features,
                                dimensions,
                            ))
                            .for_each(|(centroid_feature, feature)| {
                                *centroid_feature = (*centroid_feature
                                    * cluster_sizes[closest_centroid_number]
                                    + feature)
                                    / (cluster_sizes[closest_centroid_number] + 1.0)
                            });

                        // And increase the cardinality of the cluster.
                        cluster_sizes[closest_centroid_number] += 1.0;
                    });
                Ok(())
            })
            .collect::<Result<(), String>>()?;

        // We now have computed the centroids for all classes,
        // and we can assign them to the model.
        self.centroids = centroids;

        let (node_distances, features_per_node) = self.compute_similarities(graph, node_features, dimensions, verbose)?;

        self.perceptron.fit(
            graph,
            vec![FeatureSlice::F32(&node_distances)].as_slice(),
            vec![features_per_node].as_slice(),
            verbose
        )?;

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
        self.perceptron
            .validate_features(graph, node_features, dimensions)?;
        self.perceptron.must_be_trained()?;

        let (node_distances, features_per_node) = self.compute_similarities(graph, node_features, dimensions, None)?;

        self.perceptron.predict(
            predictions,
            graph,
            vec![FeatureSlice::F32(&node_distances)].as_slice(),
            vec![features_per_node].as_slice(),
        )?;

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
