use super::*;
use indicatif::ProgressIterator;
use rayon::prelude::*;

/// # Shortest path node embedding-based algorithms.
impl Graph {
    /// Return node embedding vector obtained from shortest-paths.
    ///
    /// # Arguments
    /// * `node_centralities`: Option<Vec<f64>> - Vector with the importance of the nodes, used to properly sample the anchors. By default node degree centralities are used. Nodes with node centrality zero won't ever be sampled as an anchor, except for when all other nodes were already sampled.
    /// * `adjust_by_central_node_distance`: Option<bool> - Whether to adjust the node eccentricity by the normalized distance to the most central node. By default true.
    /// * `number_of_nodes_to_sample_per_feature`: Option<NodeT> - Number of nodes to sample per feature. By default 10.
    /// * `maximum_number_of_features`: Option<usize> - Maximum number of node features to generate. By default 50.
    /// * `validate_node_centralities`: Option<bool> - Whether to validate the node centralities. By default true when the node centralities are provided.
    /// * `maximal_depth`: Option<NodeT> - The maximal depth to use if node features are to be focused in a local area of the graph.
    /// * `central_node_name`: Option<&str> - The node name from where to start sampling the BFS mask for the maximal depth. Either the node name of the node ID can be provided at once.
    /// * `central_node_id`: Option<NodeT> - The node ID from where to start sampling the BFS mask for the maximal depth. Either the node name of the node ID can be provided at once.
    /// * `use_edge_weights`: Option<bool> - Whether to use the edge weights to compute the min paths. By default false.
    /// * `use_edge_weights_as_probabilities`: Option<bool> - Whether to use the probabilities. By default false.
    /// * `max_neighbours`: Option<u64> - Maximum number of neighbours to sample per node. By default, all of them.
    /// * `random_state`: Option<u64> - The random state to use to sample the central node. By default 42.
    /// * `return_sampled_node_names`: Option<bool> - Whether to return the name of the sampled nodes. By default true if the `number_of_nodes_to_sample_per_feature` parameter is less than 100.
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
    /// TODO: Add parallelization for Dijkstra
    pub fn get_shortest_paths_node_embedding(
        &self,
        node_centralities: Option<Vec<f64>>,
        adjust_by_central_node_distance: Option<bool>,
        number_of_nodes_to_sample_per_feature: Option<NodeT>,
        maximum_number_of_features: Option<usize>,
        validate_node_centralities: Option<bool>,
        maximal_depth: Option<NodeT>,
        central_node_name: Option<&str>,
        central_node_id: Option<NodeT>,
        use_edge_weights: Option<bool>,
        use_edge_weights_as_probabilities: Option<bool>,
        max_neighbours: Option<u64>,
        random_state: Option<u64>,
        return_sampled_node_names: Option<bool>,
        verbose: Option<bool>,
    ) -> Result<(Vec<Vec<f32>>, Option<Vec<Vec<String>>>)> {
        let number_of_nodes_to_sample_per_feature =
            number_of_nodes_to_sample_per_feature.unwrap_or(10);
        let return_sampled_node_names =
            return_sampled_node_names.unwrap_or(number_of_nodes_to_sample_per_feature < 100);
        if number_of_nodes_to_sample_per_feature == 0 {
            return Err(
                "The maximum number of nodes to sample per feature cannot be zero.".to_string(),
            );
        }
        let maximum_number_of_features = maximum_number_of_features.unwrap_or(50);
        if maximum_number_of_features == 0 {
            return Err("The maximum number of node features cannot be zero.".to_string());
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
        if node_centralities.is_none() {
            println!("Computing node degree centralities.");
        }
        let mut node_centralities = node_centralities.unwrap_or(self.get_degree_centrality()?);
        let adjust_by_central_node_distance = adjust_by_central_node_distance.unwrap_or(true);

        if adjust_by_central_node_distance {
            println!("Computing most central node ID.");
            let most_central_node_id = self.get_most_central_node_id()?;
            if use_edge_weights {
                println!("Computing weighted min-paths using Dijkstra for weighting centralities.");
                unsafe {
                    self.get_unchecked_dijkstra_from_node_id(
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
                        *node_centrality *= distance;
                    }
                });
            } else {
                println!("Computing min-paths using BFS for weighting centralities.");
                unsafe {
                    self.get_unchecked_breadth_first_search_distances_parallel_from_node_id(
                        most_central_node_id,
                        None,
                        Some(random_state),
                    )
                }
                .into_distances()
                .into_par_iter()
                .zip(node_centralities.par_iter_mut())
                .for_each(|(distance, node_centrality)| {
                    if distance != NODE_NOT_PRESENT {
                        *node_centrality *= distance as f64;
                    }
                });
            }
        }

        let verbose = verbose.unwrap_or(true);
        let pb = get_loading_bar(
            verbose,
            "Computing shortest path node features",
            maximum_number_of_features,
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
                println!("Computing weighted min-paths using Dijkstra for masking centralities.");
                unsafe {
                    self.get_unchecked_dijkstra_from_node_id(
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
                println!("Computing min-paths using BFS for masking centralities.");
                unsafe {
                    self.get_unchecked_breadth_first_search_distances_parallel_from_node_id(
                        central_node_id,
                        max_neighbours,
                        Some(random_state),
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
            println!("Validating node centralities.");
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
        }

        println!("Allocating node embedding vector.");
        let mut node_embedding: Vec<Vec<f32>> =
            self.par_iter_node_ids().map(|_| Vec::new()).collect();
        let mut anchor_node_names: Vec<Vec<String>> = Vec::new();

        println!("Starting to compute node features.");
        for feature_number in (0..maximum_number_of_features).progress_with(pb) {
            // Sample the new anchor node IDs
            println!(
                "Sampling anchor node IDs for node feature #{}.",
                feature_number
            );

            let this_feature_anchor_node_ids = {
                let mut node_ids_and_centrality = node_centralities
                    .par_iter()
                    .cloned()
                    .enumerate()
                    .filter(|&(_, node_centrality)| node_centrality > 0.0)
                    .map(|(node_id, node_centrality)| (node_id as NodeT, node_centrality))
                    .collect::<Vec<(NodeT, f64)>>();

                if node_ids_and_centrality.is_empty() {
                    break;
                }

                node_ids_and_centrality
                    .par_sort_unstable_by(|(_, a), (_, b)| b.partial_cmp(a).unwrap());
                node_ids_and_centrality
                    .into_iter()
                    .take(number_of_nodes_to_sample_per_feature as usize)
                    .map(|(node_id, _)| node_id)
                    .collect::<Vec<NodeT>>()
            };

            {
                let thread_shared_node_centralities = ThreadDataRaceAware {
                    value: std::cell::UnsafeCell::new(&mut node_centralities),
                };

                // Update the centralities vector, setting sampled nodes centralities to zero.
                this_feature_anchor_node_ids.par_iter().cloned().for_each(
                    |anchor_node_id| unsafe {
                        // Set centrality zero to the node and the neighbouring nodes
                        // so we do not re-sample nodes that would produce extremely similar features.
                        (*thread_shared_node_centralities.value.get())[anchor_node_id as usize] =
                            0.0;
                        // TODO! Update this when parallel iterator over neighbours is made available!
                        self.iter_unchecked_neighbour_node_ids_from_source_node_id(anchor_node_id)
                            .for_each(|neighbour_node_id| {
                                (*thread_shared_node_centralities.value.get())
                                    [neighbour_node_id as usize] = 0.0;
                            });
                    },
                );
            };

            // If the node names are to be returned, we compute them.
            if return_sampled_node_names {
                println!("Retrieving the sampled node names");
                let mut this_anchor_node_names =
                    vec!["".to_string(); this_feature_anchor_node_ids.len()];
                this_feature_anchor_node_ids
                    .par_iter()
                    .cloned()
                    .map(|anchor_node_id| unsafe {
                        self.get_unchecked_node_name_from_node_id(anchor_node_id)
                    })
                    .collect_into_vec(&mut this_anchor_node_names);
                anchor_node_names.push(this_anchor_node_names);
            }

            // Compute the node features
            if use_edge_weights {
                println!(
                    "Computing weghted distances using Dijkstra for node feature #{}.",
                    feature_number
                );
                let result = unsafe {
                    self.get_unchecked_dijkstra_from_node_ids(
                        this_feature_anchor_node_ids,
                        None,
                        None,
                        Some(false),
                        None,
                        Some(use_edge_weights_as_probabilities),
                    )
                };
                let eccentricity = result.get_eccentricity();
                result
                    .into_distances()
                    .into_par_iter()
                    .zip(node_embedding.par_iter_mut())
                    .for_each(|(distance, node_feature)| {
                        node_feature.push((distance / eccentricity) as f32);
                    })
            } else {
                println!(
                    "Computing distances using BFS for node feature #{}.",
                    feature_number
                );
                let result = unsafe {
                    self.get_unchecked_breadth_first_search_distances_parallel_from_node_ids(
                        this_feature_anchor_node_ids,
                        None,
                        None,
                    )
                };
                let eccentricity = result.get_eccentricity() as f32;
                result
                    .into_distances()
                    .into_par_iter()
                    .zip(node_embedding.par_iter_mut())
                    .for_each(|(distance, node_feature)| {
                        node_feature.push(if distance == NODE_NOT_PRESENT {
                            1.0
                        } else {
                            distance as f32 / eccentricity
                        });
                    });
            }
        }
        Ok((
            node_embedding,
            if return_sampled_node_names {
                Some(anchor_node_names)
            } else {
                None
            },
        ))
    }

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
    /// * `maximum_number_of_features_per_node_type`: Option<usize> - Maximum number of node features to generate. By default 50.
    /// * `validate_node_centralities`: Option<bool> - Whether to validate the node centralities. By default true when the node centralities are provided.
    /// * `maximal_depth`: Option<NodeT> - The maximal depth to use if node features are to be focused in a local area of the graph.
    /// * `central_node_name`: Option<&str> - The node name from where to start sampling the BFS mask for the maximal depth. Either the node name of the node ID can be provided at once.
    /// * `central_node_id`: Option<NodeT> - The node ID from where to start sampling the BFS mask for the maximal depth. Either the node name of the node ID can be provided at once.
    /// * `use_edge_weights`: Option<bool> - Whether to use the edge weights to compute the min paths. By default false.
    /// * `use_edge_weights_as_probabilities`: Option<bool> - Whether to use the probabilities. By default false.
    /// * `max_neighbours`: Option<u64> - Maximum number of neighbours to sample per node. By default, all of them.
    /// * `random_state`: Option<u64> - The random state to use to sample the central node. By default 42.
    /// * `return_sampled_node_names`: Option<bool> - Whether to return the name of the sampled nodes. By default true if the `number_of_nodes_to_sample_per_feature` parameter is less than 100.
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
    pub fn get_shortest_paths_node_embedding_per_node_type(
        &self,
        node_centralities: Option<Vec<f64>>,
        adjust_by_central_node_distance: Option<bool>,
        number_of_nodes_to_sample_per_feature: Option<NodeT>,
        maximum_number_of_features_per_node_type: Option<usize>,
        validate_node_centralities: Option<bool>,
        maximal_depth: Option<NodeT>,
        central_node_name: Option<&str>,
        central_node_id: Option<NodeT>,
        use_edge_weights: Option<bool>,
        use_edge_weights_as_probabilities: Option<bool>,
        max_neighbours: Option<u64>,
        random_state: Option<u64>,
        return_sampled_node_names: Option<bool>,
        verbose: Option<bool>,
    ) -> Result<(Vec<Vec<f32>>, Option<Vec<Vec<String>>>)> {
        let validate_node_centralities =
            validate_node_centralities.unwrap_or(node_centralities.is_some());
        let node_centralities = node_centralities.unwrap_or(self.get_degree_centrality()?);
        let verbose = verbose.unwrap_or(true);
        let number_of_nodes_to_sample_per_feature =
            number_of_nodes_to_sample_per_feature.unwrap_or(10);
        let return_sampled_node_names =
            return_sampled_node_names.unwrap_or(number_of_nodes_to_sample_per_feature < 100);
        let pb = get_loading_bar(
            verbose,
            "Obtaining paths embedding per node-type",
            self.get_node_types_number()? as usize,
        );
        let mut node_embedding: Vec<Vec<f32>> =
            self.par_iter_node_ids().map(|_| Vec::new()).collect();
        let mut anchor_node_names: Vec<Vec<String>> = Vec::new();
        for (node_type_id, node_type_name) in self
            .iter_unique_node_type_ids()?
            .zip(self.iter_unique_node_type_names()?)
            .progress_with(pb)
        {
            let (this_node_embedding, this_anchor_node_names) = self
                .get_shortest_paths_node_embedding(
                    Some(
                        node_centralities
                            .par_iter()
                            .cloned()
                            .zip(self.par_iter_node_ids_and_node_type_ids())
                            .map(|(node_centrality, (_, node_type_ids))| {
                                if let Some(node_type_ids) = node_type_ids {
                                    if node_type_ids
                                        .into_iter()
                                        .any(|this_node_type_id| this_node_type_id == node_type_id)
                                    {
                                        return node_centrality;
                                    }
                                }
                                0.0
                            })
                            .collect::<Vec<f64>>(),
                    ),
                    adjust_by_central_node_distance,
                    Some(number_of_nodes_to_sample_per_feature),
                    maximum_number_of_features_per_node_type,
                    Some(validate_node_centralities),
                    maximal_depth,
                    central_node_name,
                    central_node_id,
                    use_edge_weights,
                    use_edge_weights_as_probabilities,
                    max_neighbours,
                    random_state,
                    Some(return_sampled_node_names),
                    Some(verbose),
                )?;
            if let Some(mut this_anchor_node_names) = this_anchor_node_names {
                this_anchor_node_names
                    .iter_mut()
                    .for_each(|anchors| anchors.push(node_type_name.clone()));
                anchor_node_names.extend(this_anchor_node_names);
            }

            node_embedding
                .par_iter_mut()
                .zip(this_node_embedding.into_par_iter())
                .for_each(|(node_features, this_node_features)| {
                    node_features.extend(this_node_features);
                });
        }
        Ok((
            node_embedding,
            if return_sampled_node_names {
                Some(anchor_node_names)
            } else {
                None
            },
        ))
    }
}
