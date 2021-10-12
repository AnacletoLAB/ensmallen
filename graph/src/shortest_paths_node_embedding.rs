use super::*;
use num_traits::Zero;
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
    /// * `node_centralities`: Option<Vec<f64>> - Vector with the importance of the nodes, used to properly sample the anchors. By default node degree centralities are used. Nodes with node centrality zero won't ever be sampled as an anchor, except for when all other nodes were already sampled.
    /// * `adjust_by_central_node_distance`: Option<bool> - Whether to adjust the node eccentricity by the normalized distance to the most central node. By default true.
    /// * `number_of_nodes_to_sample_per_feature`: Option<NodeT> - Number of nodes to sample per feature. By default 10.
    /// * `reduce_method`: Option<&str> - The reduce method to use when reducing the distances for a node feature. The parameter can either be "sum", "mean" or "min". By default "min".
    /// * `maximum_number_of_features`: Option<usize> - Maximum number of node features to generate. By default 50.
    /// * `validate_node_centralities`: Option<bool> - Whether to validate the node centralities. By default true when the node centralities are provided.
    /// * `maximal_depth`: Option<NodeT> - The maximal depth to use if node features are to be focused in a local area of the graph.
    /// * `central_node_name`: Option<&str> - The node name from where to start sampling the BFS mask for the maximal depth. Either the node name of the node ID can be provided at once.
    /// * `central_node_id`: Option<NodeT> - The node ID from where to start sampling the BFS mask for the maximal depth. Either the node name of the node ID can be provided at once.
    /// * `use_edge_weights`: Option<bool> - Whether to use the edge weights to compute the min paths. By default false.
    /// * `use_edge_weights_as_probabilities`: Option<bool> - Whether to use the probabilities. By default false.
    /// * `random_state`: Option<u64> - The random state to use to sample the central node. By default 42.
    /// * `verbose`: Option<bool> - Whether to show the loading bar. By default true.
    ///
    /// # Raises
    /// * If the provided node centralities are not provided for all features.
    /// * If the provided node centralities contain illegal values, like NaNs or infinities.
    /// * If the provided node centralities are not normalized.
    /// * If the number of maximum features is zero.
    /// * If the edge weights are requested but the graph does not have edge weights.
    /// * If the use edge weights as probabilities is requested, but the graph does not have edge weights as probabilities (between 0 and 1).
    /// * If the use edge weights as probabilities is requested, but not the edge weights.
    ///
    /// TODO: Add weighted diameter to normalize weighted paths!
    /// TODO: Add the early stopping
    pub fn get_shortest_paths_node_embedding(
        &self,
        node_centralities: Option<Vec<f64>>,
        adjust_by_central_node_distance: Option<bool>,
        number_of_nodes_to_sample_per_feature: Option<NodeT>,
        reduce_method: Option<&str>,
        maximum_number_of_features: Option<usize>,
        validate_node_centralities: Option<bool>,
        maximal_depth: Option<NodeT>,
        central_node_name: Option<&str>,
        central_node_id: Option<NodeT>,
        use_edge_weights: Option<bool>,
        use_edge_weights_as_probabilities: Option<bool>,
        random_state: Option<u64>,
        verbose: Option<bool>,
    ) -> Result<(Vec<Vec<f64>>, Vec<Vec<String>>)> {
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
        let reduce_method = reduce_method.unwrap_or("min");
        if reduce_method != "mean" && reduce_method != "min" && reduce_method != "sum" {
            return Err("The reduce method must either be 'sum', 'mean' or 'min'.".to_string());
        }
        let use_edge_weights = use_edge_weights.unwrap_or(false);
        if use_edge_weights {
            self.must_have_positive_edge_weights()?;
        }
        let use_edge_weights_as_probabilities = use_edge_weights_as_probabilities.unwrap_or(false);
        if use_edge_weights_as_probabilities {
            self.must_have_edge_weights_representing_probabilities()?;
        }
        if !use_edge_weights && use_edge_weights_as_probabilities {
            return Err("The `use_edge_weights_as_probabilities` parameter was provided as true, but the `use_edge_weights` was false.".to_string());
        }
        let validate_node_centralities =
            validate_node_centralities.unwrap_or(true) && node_centralities.is_some();
        let random_state = random_state.unwrap_or(42);
        let mut node_centralities = node_centralities.unwrap_or(self.get_degree_centrality()?);
        let adjust_by_central_node_distance = adjust_by_central_node_distance.unwrap_or(true);
        let diameter = self.get_diameter(Some(true), Some(false))? as f64;

        if adjust_by_central_node_distance {
            let most_central_node_id = self.get_most_central_node_id()?;
            if use_edge_weights {
                unsafe {
                    self.get_unchecked_dijkstra_from_node_ids(
                        most_central_node_id,
                        None,
                        None,
                        None,
                        None,
                        Some(use_edge_weights_as_probabilities),
                    )
                }
                .into_distances()
                .into_par_iter()
                .zip(node_centralities.par_iter_mut())
                .for_each(|(distance, node_centrality)| {
                    if use_edge_weights_as_probabilities {
                        // If we are considering the distances as probabilities,
                        // we want to multiply by the probability of the opposite event.
                        *node_centrality *= 1.0 - distance;
                    } else if distance.is_finite() {
                        // If we are treating the computed distances as "normal" distances,
                        // we want to multiply by the distance.
                        // TODO: divide by the weighted diameter!
                        *node_centrality *= distance;
                    }
                });
            } else {
                unsafe {
                    self.get_unchecked_breadth_first_search_distances_parallel_from_node_id(
                        most_central_node_id,
                    )
                }
                .into_distances()
                .into_par_iter()
                .zip(node_centralities.par_iter_mut())
                .for_each(|(distance, node_centrality)| {
                    if distance != NODE_NOT_PRESENT {
                        *node_centrality *= distance as f64 / diameter;
                    }
                });
            }
        }

        let verbose = verbose.unwrap_or(true);
        let pb = get_loading_bar(
            verbose,
            "Computing shortest path node features",
            maximum_number_of_features * number_of_nodes_to_sample_per_feature as usize,
        );

        if (central_node_name.is_some() || central_node_id.is_some()) && maximal_depth.is_none() {
            return Err(
                "The central node cannot be provided when the maximal depth is not provided."
                    .to_string(),
            );
        }

        if let Some(maximal_depth) = maximal_depth {
            let central_node_id = if let Some(central_node_name) = central_node_name {
                self.get_node_id_from_node_name(central_node_name)?
            } else if let Some(central_node_id) = central_node_id {
                central_node_id
            } else {
                self.get_random_node(random_state)
            };
            if use_edge_weights {
                unsafe {
                    self.get_unchecked_dijkstra_from_node_ids(
                        central_node_id,
                        None,
                        None,
                        None,
                        Some(maximal_depth),
                        Some(use_edge_weights_as_probabilities),
                    )
                }
                .into_distances()
                .into_par_iter()
                .zip(node_centralities.par_iter_mut())
                .for_each(|(distance, node_centrality)| {
                    if distance.is_infinite() {
                        *node_centrality = 0.0;
                    }
                });
            } else {
                unsafe {
                    self.get_unchecked_breadth_first_search_distances_parallel_from_node_id(
                        central_node_id,
                    )
                }
                .into_distances()
                .into_par_iter()
                .zip(node_centralities.par_iter_mut())
                .for_each(|(distance, node_centrality)| {
                    if distance == NODE_NOT_PRESENT {
                        *node_centrality = 0.0;
                    }
                });
            }
        }

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

        let mut node_embedding: Vec<Vec<f64>> = self.iter_node_ids().map(|_| Vec::new()).collect();
        let mut anchor_names: Vec<Vec<String>> = Vec::new();

        let mut early_stopped = false;
        for current_node_features_number in 0..maximum_number_of_features {
            // Extend the new node embedding feature
            node_embedding
                .par_iter_mut()
                .for_each(|node_features| node_features.push(0.0));
            let mut this_feature_anchor_names = Vec::new();
            for i in 0..number_of_nodes_to_sample_per_feature {
                // Getting the next anchor node ID
                let (anchor_node_id, node_centrality) =
                    node_centralities.par_iter().argmax().unwrap();
                // If the next best candidate is zero, we can stop the procedure.
                if node_centrality.is_zero() {
                    // We set the flag that we have an early stopping.
                    early_stopped = true;
                    // If we are at the first new iteration, we need to remove
                    // the feature that is currently entirely set to zero.
                    if i == 0 {
                        node_embedding
                            .par_iter_mut()
                            .for_each(|node_features| {node_features.pop();});
                    }
                    break;
                }
                let anchor_node_id = anchor_node_id as NodeT;
                // Add the node name to the list of nodes used to build this
                // node feature.
                this_feature_anchor_names
                    .push(unsafe { self.get_unchecked_node_name_from_node_id(anchor_node_id) });
                // Set centrality zero to the node and the neighbouring nodes
                // so we do not re-sample nodes that would produce extremely similar features.
                node_centralities[anchor_node_id as usize] = 0.0;
                unsafe {
                    self.iter_unchecked_neighbour_node_ids_from_source_node_id(anchor_node_id)
                        .for_each(|neighbour_node_id| {
                            node_centralities[neighbour_node_id as usize] = 0.0;
                        });
                };
                // Update the node features
                if use_edge_weights {
                    unsafe {
                        self.get_unchecked_dijkstra_from_node_ids(
                            anchor_node_id,
                            None,
                            None,
                            Some(false),
                            None,
                            Some(use_edge_weights_as_probabilities),
                        )
                    }
                    .into_distances()
                } else {
                    unsafe {
                        self.get_unchecked_breadth_first_search_distances_parallel_from_node_id(
                            anchor_node_id,
                        )
                    }
                    .into_distances()
                    .into_par_iter()
                    .map(|distance| {
                        if distance == NODE_NOT_PRESENT {
                            1.0
                        } else {
                            distance as f64 / diameter
                        }
                    })
                    .collect::<Vec<f64>>()
                }
                .into_par_iter()
                .zip(node_embedding.par_iter_mut())
                .for_each(|(distance, node_feature)| {
                    node_feature[current_node_features_number] = if i > 0 {
                        match reduce_method {
                            "sum" => node_feature[current_node_features_number] + distance,
                            "mean" => {
                                (node_feature[current_node_features_number] * i as f64 + distance)
                                    / (i as f64 + 1.0)
                            }
                            "min" => {
                                if use_edge_weights_as_probabilities {
                                    node_feature[current_node_features_number].max(distance)
                                } else {
                                    node_feature[current_node_features_number].min(distance)
                                }
                            }
                            _ => unreachable!("Only sum, min and mean are supported!"),
                        }
                    } else {
                        distance
                    }
                });
                pb.inc(1);
            }
            // If the list of node names is not empty
            if !this_feature_anchor_names.is_empty() {
                anchor_names.push(this_feature_anchor_names);
            }
            if early_stopped {
                break;
            }
        }
        Ok((node_embedding, anchor_names))
    }
}
