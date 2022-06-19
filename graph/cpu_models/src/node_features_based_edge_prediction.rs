use graph::Graph;

use crate::EdgeEmbeddingMethod;

pub trait NodeFeaturesBasedEdgePrediction {
    /// Fit the edge prediction perceptron model on the provided graph and node features.
    ///
    /// # Arguments
    /// * `graph`: &Graph - The graph whose edges are to be learned.
    /// * `node_features`: &[f32] - A node features matrix.
    /// * `dimension`: usize - The dimensionality of the node features.
    /// * `verbose`: Option<bool> - Whether to show a loading bar for the epochs. By default, True.
    /// * `support`: Option<&'a Graph> - Graph to use to check for false negatives.
    /// * `graph_to_avoid`: &'a Option<&Graph> - The graph whose edges are to be avoided during the generation of false negatives,
    fn fit(
        &mut self,
        graph: &Graph,
        node_features: &[f32],
        dimension: usize,
        verbose: Option<bool>,
        support: Option<&Graph>,
        graph_to_avoid: Option<&Graph>,
    ) -> Result<(), String>;

    /// Writes the predicted probabilities on the provided memory area.
    ///
    /// # Arguments
    /// * `predictions`: &mut [f32] - Area where to write the predictions.
    /// * `graph`: &Graph - The graph whose edges are to be learned.
    /// * `node_features`: &[f32] - A node features matrix.
    /// * `dimension`: usize - The dimensionality of the node features.
    fn predict(
        &self,
        predictions: &mut [f32],
        graph: &Graph,
        node_features: &[f32],
        dimension: usize,
    ) -> Result<(), String>;

    /// Returns whether the model has been trained.
    fn is_trained(&self) -> bool;

    /// Returns the model input size.
    fn get_input_size(&self) -> usize;

    /// Returns the edge embedding method used in the model.
    fn get_edge_embedding_method(&self) -> &EdgeEmbeddingMethod;

    /// Checks that the model has been trained.
    fn must_be_trained(&self) -> Result<(), String> {
        if !self.is_trained() {
            return Err(concat!(
                "This model has not been trained yet. ",
                "Before calling the `.predict` method, you ",
                "should call the `.fit` method."
            )
            .to_string());
        }
        Ok(())
    }

    /// Checks the provided features compatibility with the provided graph.
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

    /// Checks the provided features compatibility for prediction on the provided graph.
    fn validate_features_for_prediction(
        &self,
        predictions: &mut [f32],
        graph: &Graph,
        node_features: &[f32],
        dimension: usize,
    ) -> Result<(), String> {
        self.must_be_trained()?;
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

        let edge_dimension = self
            .get_edge_embedding_method()
            .get_dimensionality(dimension);

        if self.get_input_size() != edge_dimension {
            return Err(format!(
                concat!(
                    "This model was not trained on features compatible with ",
                    "the provided features. Specifically, the model was trained ",
                    "on features with edge embedding dimension `{}`, while the features you have ",
                    "provided have edge embedding dimension `{}`."
                ),
                self.get_input_size(),
                edge_dimension
            ));
        }

        Ok(())
    }
}
