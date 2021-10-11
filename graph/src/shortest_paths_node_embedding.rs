use super::*;
use rayon::prelude::*;

/// # Sorting algorithms.
impl Graph {
    /// Return node embedding vector obtained from shortest-paths.
    ///
    /// # Implementation details
    /// Note that the algorithm requires the diameter of the graph, which on
    /// large DIRECTED graphs is still not implemented as efficiently as it could
    /// be because of limitations in the current data-structure (computation of indegree).
    ///
    /// Additionally, note that the diameter is computed out of the component containing
    /// the most central node, which may not be the component containing the largest
    /// diameter. This is generally an acceptable heuristic.
    ///
    /// # Arguments
    /// * `node_centralities`: Option<Vec<f64>> - Vector with the importance of the nodes, used to properly sample the anchors. By default node degree centralities are used.
    /// * `number_of_nodes_to_sample_per_feature`: Option<NodeT> - Number of nodes to sample per feature. By default 10.
    /// * `reduce_method`: Option<&str> - The reduce method to use when reducing the distances for a node feature. The parameter can either be "mean" or "min". By default "mean".
    /// * `maximum_number_of_features`: Option<usize> - Maximum number of node features to generate. By default 50.
    /// * `validate_node_centralities`: Option<bool> - Whether to validate the node centralities. By default true when the node centralities are provided.
    ///
    /// # Raises
    /// * If the provided node centralities are not provided for all features.
    /// * If the provided node centralities contain illegal values, like NaNs or infinities.
    /// * If the provided node centralities are not normalized.
    /// * If the number of maximum features is zero.
    ///
    /// TODO: Add the local sampling
    /// TODO: Add the early stopping
    /// TODO: Add weighted version
    pub fn get_shortest_paths_node_embedding(
        &self,
        node_centralities: Option<Vec<f64>>,
        number_of_nodes_to_sample_per_feature: Option<NodeT>,
        reduce_method: Option<&str>,
        maximum_number_of_features: Option<usize>,
        validate_node_centralities: Option<bool>,
    ) -> Result<Vec<Vec<f64>>> {
        let number_of_nodes_to_sample_per_feature =
            number_of_nodes_to_sample_per_feature.unwrap_or(10);
        if number_of_nodes_to_sample_per_feature == 0 {
            return Err(
                "The maximum number of nodes to sample per feature cannot be zero.".to_string(),
            );
        }
        let maximum_number_of_features = maximum_number_of_features.unwrap_or(50);
        if maximum_number_of_features == 0 {
            return Err("The maximum number of node features cannot be zero.".to_string());
        }
        let reduce_method = reduce_method.unwrap_or("mean");
        if reduce_method != "mean" && reduce_method != "min" {
            return Err("The reduce method must either be 'mean' or 'min'.".to_string());
        }
        let validate_node_centralities =
            validate_node_centralities.unwrap_or(true) && node_centralities.is_some();
        let node_centralities = node_centralities.unwrap_or(self.get_degree_centrality()?);

        if node_centralities.len() as NodeT != self.get_nodes_number() {
            return Err(format!(
                concat!(
                    "The provided node centralities were not provided for exactly all of the ",
                    "nodes of the graph: the node centralities were {} but the number of nodes ",
                    "in the graph is {}."
                ),
                node_centralities.len(),
                self.get_nodes_number()
            ));
        }
        if validate_node_centralities {
            if node_centralities
                .par_iter()
                .any(|node_centrality| node_centrality.is_infinite())
            {
                return Err(
                    concat!("The provided node centralities contain an infinite value.")
                        .to_string(),
                );
            }
            if node_centralities
                .par_iter()
                .any(|node_centrality| node_centrality.is_nan())
            {
                return Err(
                    concat!("The provided node centralities contain a NaN value.").to_string(),
                );
            }
            if node_centralities
                .par_iter()
                .any(|&node_centrality| node_centrality < 0.0 || node_centrality > 1.0)
            {
                return Err(concat!(
                    "The provided node centralities contain a value ",
                    "that is not not normalized between zero and one."
                )
                .to_string());
            }
        }

        let diameter = self.get_diameter(Some(true), Some(false))? as f64;

        let mut node_embedding: Vec<Vec<f64>> = self.iter_node_ids().map(|_| Vec::new()).collect();

        for current_node_features_number in 0..maximum_number_of_features {
            // Extend the new node embedding feature
            node_embedding
                .par_iter_mut()
                .for_each(|node_features| node_features.push(0.0));
            for i in 0..number_of_nodes_to_sample_per_feature {
                // Getting the next anchor node ID
                let anchor_node_id = node_centralities
                    .par_iter()
                    .zip(node_embedding.par_iter())
                    .map(|(&node_centrality, node_features)| {
                        node_centrality
                            * (node_features[0..current_node_features_number]
                                .iter()
                                .sum::<f64>()
                                * number_of_nodes_to_sample_per_feature as f64
                                + node_features[current_node_features_number] * i as f64)
                    })
                    .argmax()
                    .unwrap()
                    .0 as NodeT;
                // Update the node features
                unsafe {
                    self.get_unchecked_breadth_first_search_predecessors_parallel_from_node_id(
                        anchor_node_id,
                    )
                }
                .into_distances()
                .into_par_iter()
                .zip(node_embedding.par_iter_mut())
                .for_each(|(distance, node_feature)| {
                    node_feature[current_node_features_number] = match reduce_method {
                        "mean" => {
                            node_feature[current_node_features_number] * i as f64
                                + if distance == NODE_NOT_PRESENT {
                                    1.0
                                } else {
                                    distance as f64 / diameter
                                } / (i as f64 + 1.0)
                        }
                        "min" => (node_feature[current_node_features_number]).min(
                            if distance == NODE_NOT_PRESENT {
                                1.0
                            } else {
                                distance as f64 / diameter
                            },
                        ),
                        _ => unreachable!("Only min and mean are supported!"),
                    }
                });
            }
        }
        Ok(node_embedding)
    }
}
