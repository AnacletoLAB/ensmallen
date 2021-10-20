use super::*;
use indicatif::ParallelProgressIterator;
use indicatif::ProgressIterator;
use funty::IsInteger;
use log::info;
use rayon::prelude::*;
use std::convert::TryFrom;

/// # Shortest path node embedding-based algorithms.
impl Graph {
    /// Validates the parameters provided to the shortest paths embedding method.
    ///
    /// # Arguments
    /// * `node_centralities`: &[f32] - Vector with the importance of the nodes, used to properly sample the anchors. By default node degree centralities are used. Nodes with node centrality zero won't ever be sampled as an anchor, except for when all other nodes were already sampled.
    /// * `validate_node_centralities`: bool - Whether to validate the node centralities. By default true when the node centralities are provided.
    /// * `number_of_nodes_to_sample_per_feature`: NodeT - Number of nodes to sample per feature. By default 10.
    /// * `maximum_number_of_features`: NodeT - Maximum number of node features to generate. By default 50.
    /// * `maximal_depth`: Option<NodeT> - The maximal depth to use if node features are to be focused in a local area of the graph.
    /// * `central_node_name`: Option<&str> - The node name from where to start sampling the BFS mask for the maximal depth. Either the node name of the node ID can be provided at once.
    /// * `central_node_id`: Option<NodeT> - The node ID from where to start sampling the BFS mask for the maximal depth. Either the node name of the node ID can be provided at once.
    fn validate_shortest_paths_node_embedding_parameters(
        &self,
        node_centralities: &[f32],
        validate_node_centralities: bool,
        number_of_nodes_to_sample_per_feature: NodeT,
        maximum_number_of_features: NodeT,
        maximal_depth: Option<NodeT>,
        central_node_name: Option<&str>,
        central_node_id: Option<NodeT>,
    ) -> Result<()> {
        if number_of_nodes_to_sample_per_feature == 0 {
            return Err(
                "The maximum number of nodes to sample per feature cannot be zero.".to_string(),
            );
        }
        if maximum_number_of_features == 0 {
            return Err("The maximum number of node features cannot be zero.".to_string());
        }
        if (central_node_name.is_some() || central_node_id.is_some()) && maximal_depth.is_none() {
            return Err(
                "The central node cannot be provided when the maximal depth is not provided."
                    .to_string(),
            );
        }

        if let Some(central_node_id) = central_node_id {
            self.validate_node_id(central_node_id)?;
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
            info!("Validating node centralities.");
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

        Ok(())
    }

    /// Return vector of vectors of anchor node IDs, samples according to provided node centralities.
    ///
    /// # Arguments
    /// * `node_centralities`: Vec<f32> - Vector with the importance of the nodes, used to properly sample the anchors.
    /// * `node_centralities_distribution`: Option<&str> - Distribution expected out of the provided node centralities distribution.
    /// * `number_of_nodes_to_sample_per_feature`: NodeT - Number of nodes to sample per feature.
    /// * `maximum_number_of_features`: usize - Maximum number of node features to generate.
    /// * `remove_neighbouring_nodes`: Option<bool> - Whether to remove the neighbouring nodes from the set of samplable anchor nodes. By default true.
    /// * `verbose`: bool - Whether to show the loading bar.
    ///
    /// # Raises
    /// * If the provided node centrality distribution is not amongst the supported ones.
    fn get_anchor_node_ids_from_node_centralities(
        &self,
        mut node_centralities: Vec<f32>,
        node_centralities_distribution: Option<&str>,
        number_of_nodes_to_sample_per_feature: NodeT,
        maximum_number_of_features: NodeT,
        remove_neighbouring_nodes: Option<bool>,
        verbose: bool,
    ) -> Result<Vec<Vec<NodeT>>> {
        let pb = get_loading_bar(
            verbose,
            "Sampling anchor node IDs for the required node features",
            maximum_number_of_features as usize,
        );
        let remove_neighbouring_nodes = remove_neighbouring_nodes.unwrap_or(true);
        (0..maximum_number_of_features)
            .progress_with(pb)
            .map(|_| {
                let threshold =
                    if let Some(node_centralities_distribution) = node_centralities_distribution {
                        match node_centralities_distribution {
                            "exponential" => self.get_exponential_distribution_threshold(
                                &node_centralities,
                                number_of_nodes_to_sample_per_feature as usize,
                            ),
                            "geometric" => self.get_geometric_distribution_threshold(
                                &node_centralities,
                                number_of_nodes_to_sample_per_feature as usize,
                            ),
                            distribution => {
                                return Err(format!(
                                    concat!(
                                        "The supported distributions currently are only `exponential`. ",
                                        "You have provided as node centralities distribution {}."
                                    ),
                                    distribution
                                ));
                            }
                        }
                    } else {
                        0.0
                    } as f32;
                let mut node_ids = self
                    .par_iter_node_ids()
                    .zip(node_centralities.par_iter().cloned())
                    .filter_map(|(node_id, node_centrality)| {
                        if node_centrality > threshold {
                            Some(node_id)
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<NodeT>>();

                    node_ids.par_sort_unstable_by(|&a, &b| {
                        node_centralities[b as usize]
                            .partial_cmp(&node_centralities[a as usize])
                            .unwrap()
                    });

                    let anchor_node_ids = node_ids[..node_ids
                        .len()
                        .min(number_of_nodes_to_sample_per_feature as usize)]
                        .to_vec();
                        info!("Setting neighbouring nodes node centralities to zero.");

                        let thread_shared_node_centralities = ThreadDataRaceAware {
                            value: std::cell::UnsafeCell::new(&mut node_centralities),
                        };
                        
                        // Update the centralities vector, setting sampled nodes centralities to zero.
                        anchor_node_ids
                            .par_iter()
                            .cloned()
                            .for_each(|anchor_node_id| unsafe {
                                // Set centrality zero to the node and the neighbouring nodes
                                // so we do not re-sample nodes that would produce extremely similar features.
                                (*thread_shared_node_centralities.value.get())[anchor_node_id as usize] = 0.0;
                                if remove_neighbouring_nodes{
                                    // TODO! Update this when parallel iterator over neighbours is made available!
                                    self.iter_unchecked_neighbour_node_ids_from_source_node_id(anchor_node_id)
                                        .for_each(|neighbour_node_id| {
                                            (*thread_shared_node_centralities.value.get())
                                                [neighbour_node_id as usize] = 0.0;
                                        });
                                }
                            });
                    Ok(anchor_node_ids)
            }).take_while(|anchor_node_ids| anchor_node_ids.as_ref().map_or(false, |anchor_node_ids| !anchor_node_ids.is_empty()))
            .collect()
    }

    /// Return vector of vectors with the node IDs curresponding to the provided node names.
    ///
    /// # Arguments
    /// * `anchor_node_ids`: &[Vec<NodeT>] - Vector of anchor node IDs.
    fn get_anchor_node_names_from_anchor_node_ids(
        &self,
        anchor_node_ids: &[Vec<NodeT>],
    ) -> Vec<Vec<String>> {
        // If the node names are to be returned, we compute them.
        info!("Retrieving the sampled node names");
        anchor_node_ids
            .par_iter()
            .map(|anchor_node_ids| {
                let mut anchor_node_names = vec!["".to_string(); anchor_node_ids.len()];
                anchor_node_ids
                    .par_iter()
                    .cloned()
                    .map(|anchor_node_id| unsafe {
                        self.get_unchecked_node_name_from_node_id(anchor_node_id)
                    })
                    .collect_into_vec(&mut anchor_node_names);
                anchor_node_names
            })
            .collect::<Vec<Vec<String>>>()
    }

    #[manual_binding]
    /// Return node embedding vector obtained from shortest-paths.
    ///
    /// # Arguments
    /// * `node_centralities`: Option<Vec<f32>> - Vector with the importance of the nodes, used to properly sample the anchors. By default node degree centralities are used. Nodes with node centrality zero won't ever be sampled as an anchor, except for when all other nodes were already sampled.
    /// * `node_centralities_distribution`: Option<&str> - Distribution expected out of the provided node centralities distribution. If no distribution is provided and no node centralities are provided, exponential is used as the default node centrality distribution as node degree centrality is employed as centrality.
    /// * `adjust_by_central_node_distance`: Option<bool> - Whether to adjust the node eccentricity by the normalized distance to the most central node. By default true.
    /// * `number_of_nodes_to_sample_per_feature`: Option<NodeT> - Number of nodes to sample per feature. By default 10.
    /// * `maximum_number_of_features`: Option<usize> - Maximum number of node features to generate. By default 50.
    /// * `remove_neighbouring_nodes`: Option<bool> - Whether to remove the neighbouring nodes from the set of samplable anchor nodes. By default true.
    /// * `validate_node_centralities`: Option<bool> - Whether to validate the node centralities. By default true when the node centralities are provided.
    /// * `maximal_depth`: Option<NodeT> - The maximal depth to use if node features are to be focused in a local area of the graph.
    /// * `central_node_name`: Option<&str> - The node name from where to start sampling the BFS mask for the maximal depth. Either the node name of the node ID can be provided at once.
    /// * `central_node_id`: Option<NodeT> - The node ID from where to start sampling the BFS mask for the maximal depth. Either the node name of the node ID can be provided at once.
    /// * `random_state`: Option<u64> - The random state to use to sample the central node. By default 42.
    /// * `return_sampled_node_names`: Option<bool> - Whether to return the name of the sampled nodes. By default true if the `number_of_nodes_to_sample_per_feature` parameter is less than 100.
    /// * `verbose`: Option<bool> - Whether to show the loading bar. By default true.
    ///
    /// # Details on the supported node centrality distributions
    /// The node centrality distributions are used to find an optimal threshold that avoids
    /// sorting nodes that include also non-useful nodes, that is nodes that we will never
    /// be interested in sampling. We currently support the following node centrality distributions:
    ///
    /// ## Exponential
    /// The most common distribution for node centralities is the exponential distribution.
    /// Most likely, your node centralities will follow this distribution.
    ///
    /// ## Geometric
    /// The geometric distribution is to be used for an integer distribution, when the normalization
    /// by the distances from the most central node is disabled (or it will make the distribution a float value).
    ///
    /// ## Unknown
    /// For now we do not have support for other distributions implemented, so if the distribution
    /// is not expected to be one of the aforementioned distributions we will use zero as a threshold.
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
    pub fn get_shortest_paths_node_embedding<'a, T: 'a + TryFrom<u32> + Into<u32> + Send + Sync + IsInteger + TryFrom<usize>>(
        &'a self,
        node_centralities: Option<Vec<f32>>,
        mut node_centralities_distribution: Option<&str>,
        adjust_by_central_node_distance: Option<bool>,
        number_of_nodes_to_sample_per_feature: Option<NodeT>,
        maximum_number_of_features: Option<NodeT>,
        remove_neighbouring_nodes: Option<bool>,
        validate_node_centralities: Option<bool>,
        maximal_depth: Option<T>,
        central_node_name: Option<&str>,
        central_node_id: Option<NodeT>,
        random_state: Option<u64>,
        return_sampled_node_names: Option<bool>,
        verbose: Option<bool>,
    ) -> Result<(
        NodeT,
        impl Iterator<Item = impl IndexedParallelIterator<Item = T> + 'a> + 'a,
        Option<Vec<Vec<String>>>,
    )> {
        let number_of_nodes_to_sample_per_feature =
            number_of_nodes_to_sample_per_feature.unwrap_or(10);
        let return_sampled_node_names =
            return_sampled_node_names.unwrap_or(number_of_nodes_to_sample_per_feature < 100);
        let maximum_number_of_features = maximum_number_of_features.unwrap_or(50);
        let validate_node_centralities =
            validate_node_centralities.unwrap_or(true) && node_centralities.is_some();

        let random_state = random_state.unwrap_or(42);
        if node_centralities.is_none() {
            info!("Computing node degree centralities.");
            node_centralities_distribution = Some("exponential");
        }
        let mut node_centralities = node_centralities.unwrap_or(self.get_degree_centrality()?);

        self.validate_shortest_paths_node_embedding_parameters(
            &node_centralities,
            validate_node_centralities,
            number_of_nodes_to_sample_per_feature,
            maximum_number_of_features,
            maximal_depth.map(|maximal_depth| maximal_depth.into()),
            central_node_name,
            central_node_id,
        )?;

        let adjust_by_central_node_distance = adjust_by_central_node_distance.unwrap_or(true);

        if adjust_by_central_node_distance {
            info!("Computing most central node ID.");
            let most_central_node_id = self.get_most_central_node_id()?;

            info!("Computing min-paths using BFS for weighting centralities.");
            let (distances, eccentricity, _, ) = unsafe {
                self.get_unchecked_generic_breadth_first_search_distances_parallel_from_node_ids::<T>(
                    vec![most_central_node_id],
                    None,
                )
            };
            let eccentricity: u32 = eccentricity.try_into().ok().unwrap();
            let eccentricity: f32 = eccentricity as f32;

            distances
                .into_par_iter()
                .zip(node_centralities.par_iter_mut())
                .for_each(|(distance, node_centrality)| {
                    if distance != T::MAX {
                        let distance: u32 = distance.try_into().ok().unwrap();
                        *node_centrality *=  distance as f32/ eccentricity;
                    }
                });
        }

        let verbose = verbose.unwrap_or(true);

        if let Some(maximal_depth) = maximal_depth {
            let central_node_id = if let Some(central_node_name) = central_node_name {
                self.get_node_id_from_node_name(central_node_name)?
            } else if let Some(central_node_id) = central_node_id {
                central_node_id
            } else {
                self.get_random_node(random_state)
            };

            info!("Computing min-paths using BFS for masking centralities.");
            unsafe {
                self.get_unchecked_generic_breadth_first_search_distances_parallel_from_node_ids::<T>(
                    vec![central_node_id],
                    Some(maximal_depth),
                )
            }.0
            .into_par_iter()
            .zip(node_centralities.par_iter_mut())
            .for_each(|(distance, node_centrality)| {
                if distance == T::MAX {
                    *node_centrality = 0.0;
                }
            });
        }

        // Compute the anchor node IDs.
        let anchor_node_ids = self.get_anchor_node_ids_from_node_centralities(
            node_centralities,
            node_centralities_distribution,
            number_of_nodes_to_sample_per_feature,
            maximum_number_of_features,
            remove_neighbouring_nodes,
            verbose,
        )?;

        info!("Starting to compute node features.");
        let pb = get_loading_bar(verbose, "Computing node features", anchor_node_ids.len());
        let anchor_node_names = if return_sampled_node_names {
            Some(self.get_anchor_node_names_from_anchor_node_ids(&anchor_node_ids))
        } else {
            None
        };
        Ok((
            anchor_node_ids.len() as NodeT,
            anchor_node_ids
                .into_iter()
                .progress_with(pb)
                .map(move |anchor_node_ids| {
                    // Compute the node features
                    let (distances, eccentricity, _) = unsafe {
                        self.get_unchecked_generic_breadth_first_search_distances_parallel_from_node_ids::<T>(
                            anchor_node_ids,
                            None,
                        )
                    };
                    distances
                        .into_par_iter()
                        .map(move |distance| {
                            T::try_from(if distance == T::MAX {
                                eccentricity
                            } else {
                                distance
                            })
                            .ok()
                            .unwrap()
                        })
                }),
            anchor_node_names,
        ))
    }

    #[manual_binding]
    /// Return node embedding vector obtained from weighted shortest-paths.
    ///
    /// # Arguments
    /// * `node_centralities`: Option<Vec<f32>> - Vector with the importance of the nodes, used to properly sample the anchors. By default node degree centralities are used. Nodes with node centrality zero won't ever be sampled as an anchor, except for when all other nodes were already sampled.
    /// * `node_centralities_distribution`: Option<&str> - Distribution expected out of the provided node centralities distribution. If no distribution is provided and no node centralities are provided, exponential is used as the default node centrality distribution as node degree centrality is employed as centrality.
    /// * `adjust_by_central_node_distance`: Option<bool> - Whether to adjust the node eccentricity by the normalized distance to the most central node. By default true.
    /// * `number_of_nodes_to_sample_per_feature`: Option<NodeT> - Number of nodes to sample per feature. By default 10.
    /// * `maximum_number_of_features`: Option<usize> - Maximum number of node features to generate. By default 50.
    /// * `remove_neighbouring_nodes`: Option<bool> - Whether to remove the neighbouring nodes from the set of samplable anchor nodes. By default true.
    /// * `validate_node_centralities`: Option<bool> - Whether to validate the node centralities. By default true when the node centralities are provided.
    /// * `maximal_depth`: Option<NodeT> - The maximal depth to use if node features are to be focused in a local area of the graph.
    /// * `central_node_name`: Option<&str> - The node name from where to start sampling the BFS mask for the maximal depth. Either the node name of the node ID can be provided at once.
    /// * `central_node_id`: Option<NodeT> - The node ID from where to start sampling the BFS mask for the maximal depth. Either the node name of the node ID can be provided at once.
    /// * `use_edge_weights_as_probabilities`: Option<bool> - Whether to use the probabilities. By default false.
    /// * `random_state`: Option<u64> - The random state to use to sample the central node. By default 42.
    /// * `return_sampled_node_names`: Option<bool> - Whether to return the name of the sampled nodes. By default true if the `number_of_nodes_to_sample_per_feature` parameter is less than 100.
    /// * `verbose`: Option<bool> - Whether to show the loading bar. By default true.
    ///
    /// # Details on the supported node centrality distributions
    /// The node centrality distributions are used to find an optimal threshold that avoids
    /// sorting nodes that include also non-useful nodes, that is nodes that we will never
    /// be interested in sampling. We currently support the following node centrality distributions:
    ///
    /// ## Exponential
    /// The most common distribution for node centralities is the exponential distribution.
    /// Most likely, your node centralities will follow this distribution.
    ///
    /// ## Geometric
    /// The geometric distribution is to be used for an integer distribution, when the normalization
    /// by the distances from the most central node is disabled (or it will make the distribution a float value).
    ///
    /// ## Unknown
    /// For now we do not have support for other distributions implemented, so if the distribution
    /// is not expected to be one of the aforementioned distributions we will use zero as a threshold.
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
    pub fn get_weighted_shortest_paths_node_embedding(
        &self,
        node_centralities: Option<Vec<f32>>,
        mut node_centralities_distribution: Option<&str>,
        adjust_by_central_node_distance: Option<bool>,
        number_of_nodes_to_sample_per_feature: Option<NodeT>,
        maximum_number_of_features: Option<NodeT>,
        remove_neighbouring_nodes: Option<bool>,
        validate_node_centralities: Option<bool>,
        maximal_depth: Option<NodeT>,
        central_node_name: Option<&str>,
        central_node_id: Option<NodeT>,
        use_edge_weights_as_probabilities: Option<bool>,
        random_state: Option<u64>,
        return_sampled_node_names: Option<bool>,
        verbose: Option<bool>,
    ) -> Result<(
        NodeT,
        impl IndexedParallelIterator<Item = impl IndexedParallelIterator<Item = f32> + '_> + '_,
        Option<Vec<Vec<String>>>,
    )> {
        self.must_have_positive_edge_weights()?;
        let number_of_nodes_to_sample_per_feature =
            number_of_nodes_to_sample_per_feature.unwrap_or(10);
        let return_sampled_node_names =
            return_sampled_node_names.unwrap_or(number_of_nodes_to_sample_per_feature < 100);
        let maximum_number_of_features = maximum_number_of_features.unwrap_or(50);
        let validate_node_centralities =
            validate_node_centralities.unwrap_or(true) && node_centralities.is_some();
        let use_edge_weights_as_probabilities = use_edge_weights_as_probabilities.unwrap_or(false);
        if use_edge_weights_as_probabilities {
            self.must_have_edge_weights_representing_probabilities()?;
        }
        let random_state = random_state.unwrap_or(42);
        if node_centralities.is_none() {
            info!("Computing node degree centralities.");
            node_centralities_distribution = Some("exponential");
        }
        let mut node_centralities = node_centralities.unwrap_or(self.get_degree_centrality()?);

        self.validate_shortest_paths_node_embedding_parameters(
            &node_centralities,
            validate_node_centralities,
            number_of_nodes_to_sample_per_feature,
            maximum_number_of_features,
            maximal_depth,
            central_node_name,
            central_node_id,
        )?;

        let adjust_by_central_node_distance = adjust_by_central_node_distance.unwrap_or(true);

        if adjust_by_central_node_distance {
            info!("Computing most central node ID.");
            let most_central_node_id = self.get_most_central_node_id()?;

            info!("Computing min-paths using Dijkstra for weighting centralities.");
            let result = unsafe {
                self.get_unchecked_dijkstra_from_node_id(
                    most_central_node_id,
                    None,
                    None,
                    None,
                    None,
                    Some(use_edge_weights_as_probabilities),
                )
            };
            let eccentricity = result.get_eccentricity() as f32;
            result
                .into_distances()
                .into_par_iter()
                .zip(node_centralities.par_iter_mut())
                .for_each(|(distance, node_centrality)| {
                    if distance.is_finite() {
                        *node_centrality *= distance as f32 / eccentricity;
                    }
                });
        }

        let verbose = verbose.unwrap_or(true);

        if let Some(maximal_depth) = maximal_depth {
            let central_node_id = if let Some(central_node_name) = central_node_name {
                self.get_node_id_from_node_name(central_node_name)?
            } else if let Some(central_node_id) = central_node_id {
                central_node_id
            } else {
                self.get_random_node(random_state)
            };

            info!("Computing min-paths using BFS for masking centralities.");
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
        }

        // Compute the anchor node IDs.
        let anchor_node_ids = self.get_anchor_node_ids_from_node_centralities(
            node_centralities,
            node_centralities_distribution,
            number_of_nodes_to_sample_per_feature,
            maximum_number_of_features,
            remove_neighbouring_nodes,
            verbose,
        )?;

        info!("Starting to compute node features.");
        let pb = get_loading_bar(verbose, "Computing node features", anchor_node_ids.len());
        let anchor_node_names = if return_sampled_node_names {
            Some(self.get_anchor_node_names_from_anchor_node_ids(&anchor_node_ids))
        } else {
            None
        };
        Ok((
            anchor_node_ids.len() as NodeT,
            anchor_node_ids
                .into_par_iter()
                .progress_with(pb)
                .map(move |anchor_node_ids| {
                    // Compute the node features
                    let result = unsafe {
                        self.get_unchecked_dijkstra_from_node_ids(
                            anchor_node_ids,
                            None,
                            None,
                            Some(false),
                            None,
                            Some(use_edge_weights_as_probabilities),
                        )
                    };
                    let eccentricity = result.get_eccentricity() as f32;
                    result
                        .into_distances()
                        .into_par_iter()
                        .map(move |distance| {
                            if distance.is_infinite() {
                                1.0
                            } else {
                                distance as f32 / eccentricity
                            }
                        })
                }),
            anchor_node_names,
        ))
    }

    #[manual_binding]
    /// Return node embedding vector obtained from symmetric laplacian shortest-paths.
    ///
    /// # Arguments
    /// * `node_centralities`: Option<Vec<f32>> - Vector with the importance of the nodes, used to properly sample the anchors. By default node degree centralities are used. Nodes with node centrality zero won't ever be sampled as an anchor, except for when all other nodes were already sampled.
    /// * `node_centralities_distribution`: Option<&str> - Distribution expected out of the provided node centralities distribution. If no distribution is provided and no node centralities are provided, exponential is used as the default node centrality distribution as node degree centrality is employed as centrality.
    /// * `adjust_by_central_node_distance`: Option<bool> - Whether to adjust the node eccentricity by the normalized distance to the most central node. By default true.
    /// * `number_of_nodes_to_sample_per_feature`: Option<NodeT> - Number of nodes to sample per feature. By default 10.
    /// * `maximum_number_of_features`: Option<usize> - Maximum number of node features to generate. By default 50.
    /// * `remove_neighbouring_nodes`: Option<bool> - Whether to remove the neighbouring nodes from the set of samplable anchor nodes. By default true.
    /// * `validate_node_centralities`: Option<bool> - Whether to validate the node centralities. By default true when the node centralities are provided.
    /// * `maximal_depth`: Option<NodeT> - The maximal depth to use if node features are to be focused in a local area of the graph.
    /// * `central_node_name`: Option<&str> - The node name from where to start sampling the BFS mask for the maximal depth. Either the node name of the node ID can be provided at once.
    /// * `central_node_id`: Option<NodeT> - The node ID from where to start sampling the BFS mask for the maximal depth. Either the node name of the node ID can be provided at once.
    /// * `random_state`: Option<u64> - The random state to use to sample the central node. By default 42.
    /// * `return_sampled_node_names`: Option<bool> - Whether to return the name of the sampled nodes. By default true if the `number_of_nodes_to_sample_per_feature` parameter is less than 100.
    /// * `verbose`: Option<bool> - Whether to show the loading bar. By default true.
    ///
    /// # Details on the supported node centrality distributions
    /// The node centrality distributions are used to find an optimal threshold that avoids
    /// sorting nodes that include also non-useful nodes, that is nodes that we will never
    /// be interested in sampling. We currently support the following node centrality distributions:
    ///
    /// ## Exponential
    /// The most common distribution for node centralities is the exponential distribution.
    /// Most likely, your node centralities will follow this distribution.
    ///
    /// ## Geometric
    /// The geometric distribution is to be used for an integer distribution, when the normalization
    /// by the distances from the most central node is disabled (or it will make the distribution a float value).
    ///
    /// ## Unknown
    /// For now we do not have support for other distributions implemented, so if the distribution
    /// is not expected to be one of the aforementioned distributions we will use zero as a threshold.
    ///
    /// # Raises
    /// * If the provided node centralities are not provided for all features.
    /// * If the provided node centralities contain illegal values, like NaNs or infinities.
    /// * If the provided node centralities are not normalized.
    /// * If the number of maximum features is zero.
    ///
    pub fn get_symmetric_laplacian_shortest_paths_node_embedding(
        &self,
        node_centralities: Option<Vec<f32>>,
        mut node_centralities_distribution: Option<&str>,
        adjust_by_central_node_distance: Option<bool>,
        number_of_nodes_to_sample_per_feature: Option<NodeT>,
        maximum_number_of_features: Option<NodeT>,
        remove_neighbouring_nodes: Option<bool>,
        validate_node_centralities: Option<bool>,
        maximal_depth: Option<NodeT>,
        central_node_name: Option<&str>,
        central_node_id: Option<NodeT>,
        random_state: Option<u64>,
        return_sampled_node_names: Option<bool>,
        verbose: Option<bool>,
    ) -> Result<(
        NodeT,
        impl IndexedParallelIterator<Item = impl IndexedParallelIterator<Item = f32> + '_> + '_,
        Option<Vec<Vec<String>>>,
    )> {
        self.must_not_have_trap_nodes()?;
        let number_of_nodes_to_sample_per_feature =
            number_of_nodes_to_sample_per_feature.unwrap_or(10);
        let return_sampled_node_names =
            return_sampled_node_names.unwrap_or(number_of_nodes_to_sample_per_feature < 100);
        let maximum_number_of_features = maximum_number_of_features.unwrap_or(50);
        let validate_node_centralities =
            validate_node_centralities.unwrap_or(true) && node_centralities.is_some();
        let random_state = random_state.unwrap_or(42);
        if node_centralities.is_none() {
            info!("Computing node degree centralities.");
            node_centralities_distribution = Some("exponential");
        }
        let mut node_centralities = node_centralities.unwrap_or(self.get_degree_centrality()?);

        self.validate_shortest_paths_node_embedding_parameters(
            &node_centralities,
            validate_node_centralities,
            number_of_nodes_to_sample_per_feature,
            maximum_number_of_features,
            maximal_depth,
            central_node_name,
            central_node_id,
        )?;

        let adjust_by_central_node_distance = adjust_by_central_node_distance.unwrap_or(true);

        if adjust_by_central_node_distance {
            info!("Computing most central node ID.");
            let most_central_node_id = self.get_most_central_node_id()?;

            info!("Computing min-paths using Dijkstra for weighting centralities.");
            let result = unsafe {
                self.get_unchecked_symmetric_laplacian_dijkstra_from_node_ids(
                    vec![most_central_node_id],
                    None,
                    None,
                    None,
                    None,
                )
            };
            let eccentricity = result.get_eccentricity() as f32;
            result
                .into_distances()
                .into_par_iter()
                .zip(node_centralities.par_iter_mut())
                .for_each(|(distance, node_centrality)| {
                    if distance.is_finite() {
                        *node_centrality *= distance as f32 / eccentricity;
                    }
                });
        }

        let verbose = verbose.unwrap_or(true);

        if let Some(maximal_depth) = maximal_depth {
            let central_node_id = if let Some(central_node_name) = central_node_name {
                self.get_node_id_from_node_name(central_node_name)?
            } else if let Some(central_node_id) = central_node_id {
                central_node_id
            } else {
                self.get_random_node(random_state)
            };

            info!("Computing min-paths using BFS for masking centralities.");
            unsafe {
                self.get_unchecked_symmetric_laplacian_dijkstra_from_node_ids(
                    vec![central_node_id],
                    None,
                    None,
                    None,
                    Some(maximal_depth),
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
        }

        // Compute the anchor node IDs.
        let anchor_node_ids = self.get_anchor_node_ids_from_node_centralities(
            node_centralities,
            node_centralities_distribution,
            number_of_nodes_to_sample_per_feature,
            maximum_number_of_features,
            remove_neighbouring_nodes,
            verbose,
        )?;

        info!("Starting to compute node features.");
        let pb = get_loading_bar(verbose, "Computing node features", anchor_node_ids.len());
        let anchor_node_names = if return_sampled_node_names {
            Some(self.get_anchor_node_names_from_anchor_node_ids(&anchor_node_ids))
        } else {
            None
        };
        Ok((
            anchor_node_ids.len() as NodeT,
            anchor_node_ids
                .into_par_iter()
                .progress_with(pb)
                .map(move |anchor_node_ids| {
                    // Compute the node features
                    let result = unsafe {
                        self.get_unchecked_symmetric_laplacian_dijkstra_from_node_ids(
                            anchor_node_ids,
                            None,
                            None,
                            Some(false),
                            None,
                        )
                    };
                    let eccentricity = result.get_eccentricity() as f32;
                    result
                        .into_distances()
                        .into_par_iter()
                        .map(move |distance| {
                            if distance.is_infinite() {
                                1.0
                            } else {
                                distance as f32 / eccentricity
                            }
                        })
                }),
            anchor_node_names,
        ))
    }

    #[manual_binding]
    /// Return node embedding vector obtained from random walk laplacian shortest-paths.
    ///
    /// # Arguments
    /// * `node_centralities`: Option<Vec<f32>> - Vector with the importance of the nodes, used to properly sample the anchors. By default node degree centralities are used. Nodes with node centrality zero won't ever be sampled as an anchor, except for when all other nodes were already sampled.
    /// * `node_centralities_distribution`: Option<&str> - Distribution expected out of the provided node centralities distribution. If no distribution is provided and no node centralities are provided, exponential is used as the default node centrality distribution as node degree centrality is employed as centrality.
    /// * `adjust_by_central_node_distance`: Option<bool> - Whether to adjust the node eccentricity by the normalized distance to the most central node. By default true.
    /// * `number_of_nodes_to_sample_per_feature`: Option<NodeT> - Number of nodes to sample per feature. By default 10.
    /// * `maximum_number_of_features`: Option<usize> - Maximum number of node features to generate. By default 50.
    /// * `remove_neighbouring_nodes`: Option<bool> - Whether to remove the neighbouring nodes from the set of samplable anchor nodes. By default true.
    /// * `validate_node_centralities`: Option<bool> - Whether to validate the node centralities. By default true when the node centralities are provided.
    /// * `maximal_depth`: Option<NodeT> - The maximal depth to use if node features are to be focused in a local area of the graph.
    /// * `central_node_name`: Option<&str> - The node name from where to start sampling the BFS mask for the maximal depth. Either the node name of the node ID can be provided at once.
    /// * `central_node_id`: Option<NodeT> - The node ID from where to start sampling the BFS mask for the maximal depth. Either the node name of the node ID can be provided at once.
    /// * `random_state`: Option<u64> - The random state to use to sample the central node. By default 42.
    /// * `return_sampled_node_names`: Option<bool> - Whether to return the name of the sampled nodes. By default true if the `number_of_nodes_to_sample_per_feature` parameter is less than 100.
    /// * `verbose`: Option<bool> - Whether to show the loading bar. By default true.
    ///
    /// # Details on the supported node centrality distributions
    /// The node centrality distributions are used to find an optimal threshold that avoids
    /// sorting nodes that include also non-useful nodes, that is nodes that we will never
    /// be interested in sampling. We currently support the following node centrality distributions:
    ///
    /// ## Exponential
    /// The most common distribution for node centralities is the exponential distribution.
    /// Most likely, your node centralities will follow this distribution.
    ///
    /// ## Geometric
    /// The geometric distribution is to be used for an integer distribution, when the normalization
    /// by the distances from the most central node is disabled (or it will make the distribution a float value).
    ///
    /// ## Unknown
    /// For now we do not have support for other distributions implemented, so if the distribution
    /// is not expected to be one of the aforementioned distributions we will use zero as a threshold.
    ///
    /// # Raises
    /// * If the provided node centralities are not provided for all features.
    /// * If the provided node centralities contain illegal values, like NaNs or infinities.
    /// * If the provided node centralities are not normalized.
    /// * If the number of maximum features is zero.
    ///
    pub fn get_random_walk_laplacian_shortest_paths_node_embedding(
        &self,
        node_centralities: Option<Vec<f32>>,
        mut node_centralities_distribution: Option<&str>,
        adjust_by_central_node_distance: Option<bool>,
        number_of_nodes_to_sample_per_feature: Option<NodeT>,
        maximum_number_of_features: Option<NodeT>,
        remove_neighbouring_nodes: Option<bool>,
        validate_node_centralities: Option<bool>,
        maximal_depth: Option<NodeT>,
        central_node_name: Option<&str>,
        central_node_id: Option<NodeT>,
        random_state: Option<u64>,
        return_sampled_node_names: Option<bool>,
        verbose: Option<bool>,
    ) -> Result<(
        NodeT,
        impl IndexedParallelIterator<Item = impl IndexedParallelIterator<Item = f32> + '_> + '_,
        Option<Vec<Vec<String>>>,
    )> {
        self.must_not_have_trap_nodes()?;
        let number_of_nodes_to_sample_per_feature =
            number_of_nodes_to_sample_per_feature.unwrap_or(10);
        let return_sampled_node_names =
            return_sampled_node_names.unwrap_or(number_of_nodes_to_sample_per_feature < 100);
        let maximum_number_of_features = maximum_number_of_features.unwrap_or(50);
        let validate_node_centralities =
            validate_node_centralities.unwrap_or(true) && node_centralities.is_some();
        let random_state = random_state.unwrap_or(42);
        if node_centralities.is_none() {
            info!("Computing node degree centralities.");
            node_centralities_distribution = Some("exponential");
        }
        let mut node_centralities = node_centralities.unwrap_or(self.get_degree_centrality()?);

        self.validate_shortest_paths_node_embedding_parameters(
            &node_centralities,
            validate_node_centralities,
            number_of_nodes_to_sample_per_feature,
            maximum_number_of_features,
            maximal_depth,
            central_node_name,
            central_node_id,
        )?;

        let adjust_by_central_node_distance = adjust_by_central_node_distance.unwrap_or(true);

        if adjust_by_central_node_distance {
            info!("Computing most central node ID.");
            let most_central_node_id = self.get_most_central_node_id()?;

            info!("Computing min-paths using Dijkstra for weighting centralities.");
            let result = unsafe {
                self.get_unchecked_random_walk_laplacian_dijkstra_from_node_ids(
                    vec![most_central_node_id],
                    None,
                    None,
                    None,
                    None,
                )
            };
            let eccentricity = result.get_eccentricity() as f32;
            result
                .into_distances()
                .into_par_iter()
                .zip(node_centralities.par_iter_mut())
                .for_each(|(distance, node_centrality)| {
                    if distance.is_finite() {
                        *node_centrality *= distance as f32 / eccentricity;
                    }
                });
        }

        let verbose = verbose.unwrap_or(true);

        if let Some(maximal_depth) = maximal_depth {
            let central_node_id = if let Some(central_node_name) = central_node_name {
                self.get_node_id_from_node_name(central_node_name)?
            } else if let Some(central_node_id) = central_node_id {
                central_node_id
            } else {
                self.get_random_node(random_state)
            };

            info!("Computing min-paths using BFS for masking centralities.");
            unsafe {
                self.get_unchecked_random_walk_laplacian_dijkstra_from_node_ids(
                    vec![central_node_id],
                    None,
                    None,
                    None,
                    Some(maximal_depth),
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
        }

        // Compute the anchor node IDs.
        let anchor_node_ids = self.get_anchor_node_ids_from_node_centralities(
            node_centralities,
            node_centralities_distribution,
            number_of_nodes_to_sample_per_feature,
            maximum_number_of_features,
            remove_neighbouring_nodes,
            verbose,
        )?;

        info!("Starting to compute node features.");
        let pb = get_loading_bar(verbose, "Computing node features", anchor_node_ids.len());
        let anchor_node_names = if return_sampled_node_names {
            Some(self.get_anchor_node_names_from_anchor_node_ids(&anchor_node_ids))
        } else {
            None
        };
        Ok((
            anchor_node_ids.len() as NodeT,
            anchor_node_ids
                .into_par_iter()
                .progress_with(pb)
                .map(move |anchor_node_ids| {
                    // Compute the node features
                    let result = unsafe {
                        self.get_unchecked_random_walk_laplacian_dijkstra_from_node_ids(
                            anchor_node_ids,
                            None,
                            None,
                            Some(false),
                            None,
                        )
                    };
                    let eccentricity = result.get_eccentricity() as f32;
                    result
                        .into_distances()
                        .into_par_iter()
                        .map(move |distance| {
                            if distance.is_infinite() {
                                1.0
                            } else {
                                distance as f32 / eccentricity
                            }
                        })
                }),
            anchor_node_names,
        ))
    }

    #[manual_binding]
    /// Return node-type-aware node embedding vector obtained from shortest-paths.
    ///
    /// # Arguments
    /// * `node_centralities`: Option<Vec<f32>> - Vector with the importance of the nodes, used to properly sample the anchors. By default node degree centralities are used. Nodes with node centrality zero won't ever be sampled as an anchor, except for when all other nodes were already sampled.
    /// * `node_centralities_distribution`: Option<&str> - Distribution expected out of the provided node centralities distribution. If no distribution is provided and no node centralities are provided, exponential is used as the default node centrality distribution as node degree centrality is employed as centrality.
    /// * `adjust_by_central_node_distance`: Option<bool> - Whether to adjust the node eccentricity by the normalized distance to the most central node. By default true.
    /// * `number_of_nodes_to_sample_per_feature`: Option<NodeT> - Number of nodes to sample per feature. By default 10.
    /// * `maximum_number_of_features_per_node_type`: Option<usize> - Maximum number of node features to generate. By default 50.
    /// * `remove_neighbouring_nodes`: Option<bool> - Whether to remove the neighbouring nodes from the set of samplable anchor nodes. By default true.
    /// * `validate_node_centralities`: Option<bool> - Whether to validate the node centralities. By default true when the node centralities are provided.
    /// * `maximal_depth`: Option<NodeT> - The maximal depth to use if node features are to be focused in a local area of the graph.
    /// * `central_node_name`: Option<&str> - The node name from where to start sampling the BFS mask for the maximal depth. Either the node name of the node ID can be provided at once.
    /// * `central_node_id`: Option<NodeT> - The node ID from where to start sampling the BFS mask for the maximal depth. Either the node name of the node ID can be provided at once.
    /// * `random_state`: Option<u64> - The random state to use to sample the central node. By default 42.
    /// * `return_sampled_node_names`: Option<bool> - Whether to return the name of the sampled nodes. By default true if the `number_of_nodes_to_sample_per_feature` parameter is less than 100.
    /// * `verbose`: Option<bool> - Whether to show the loading bar. By default true.
    ///
    /// # Details on the supported node centrality distributions
    /// The node centrality distributions are used to find an optimal threshold that avoids
    /// sorting nodes that include also non-useful nodes, that is nodes that we will never
    /// be interested in sampling. We currently support the following node centrality distributions:
    ///
    /// ## Exponential
    /// The most common distribution for node centralities is the exponential distribution.
    /// Most likely, your node centralities will follow this distribution.
    ///
    /// ## Geometric
    /// The geometric distribution is to be used for an integer distribution, when the normalization
    /// by the distances from the most central node is disabled (or it will make the distribution a float value).
    ///
    /// ## Unknown
    /// For now we do not have support for other distributions implemented, so if the distribution
    /// is not expected to be one of the aforementioned distributions we will use zero as a threshold.
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
    pub fn get_shortest_paths_node_embedding_per_node_type<
        'a,
        T: 'a + TryFrom<u32> + Send + Sync+ TryFrom<usize> + IsInteger + Into<u32>,
    >(
        &'a self,
        node_centralities: Option<Vec<f32>>,
        node_centralities_distribution: Option<&str>,
        adjust_by_central_node_distance: Option<bool>,
        number_of_nodes_to_sample_per_feature: Option<NodeT>,
        maximum_number_of_features_per_node_type: Option<NodeT>,
        remove_neighbouring_nodes: Option<bool>,
        validate_node_centralities: Option<bool>,
        maximal_depth: Option<T>,
        central_node_name: Option<&str>,
        central_node_id: Option<NodeT>,
        random_state: Option<u64>,
        return_sampled_node_names: Option<bool>,
        verbose: Option<bool>,
    ) -> Result<(
        NodeT,
        impl Iterator<Item = impl IndexedParallelIterator<Item = T> + '_> + '_,
        Vec<String>,
        Option<Vec<Vec<String>>>,
    )> {
        let validate_node_centralities =
            validate_node_centralities.unwrap_or(node_centralities.is_some());
        let node_centralities = node_centralities.unwrap_or(self.get_degree_centrality()?);
        let return_sampled_node_names = return_sampled_node_names.unwrap_or(false);
        let verbose = verbose.unwrap_or(true);
        let pb = get_loading_bar(
            verbose,
            "Obtaining paths embedding per node-type",
            self.get_node_types_number()? as usize,
        );
        let mut node_type_names = Vec::new();
        let mut iterators = Vec::new();
        let mut total_anchor_node_names = Vec::new();
        let mut total_number_of_node_features = 0;
        for result in self
            .iter_unique_node_type_ids()?
            .zip(self.iter_unique_node_type_names()?)
            .progress_with(pb)
            .map(|(node_type_id, node_type_name)| {
                let (number_of_node_features, iterator, anchor_node_names) = self
                    .get_shortest_paths_node_embedding::<T>(
                        Some(
                            node_centralities
                                .par_iter()
                                .cloned()
                                .zip(self.par_iter_node_ids_and_node_type_ids())
                                .map(|(node_centrality, (_, node_type_ids))| {
                                    if let Some(node_type_ids) = node_type_ids {
                                        if node_type_ids.into_iter().any(|this_node_type_id| {
                                            this_node_type_id == node_type_id
                                        }) {
                                            return node_centrality;
                                        }
                                    }
                                    0.0
                                })
                                .collect::<Vec<f32>>(),
                        ),
                        node_centralities_distribution,
                        adjust_by_central_node_distance,
                        number_of_nodes_to_sample_per_feature,
                        maximum_number_of_features_per_node_type,
                        remove_neighbouring_nodes,
                        Some(validate_node_centralities),
                        maximal_depth,
                        central_node_name,
                        central_node_id,
                        random_state,
                        Some(return_sampled_node_names),
                        Some(verbose),
                    )?;
                Ok::<_, String>((
                    number_of_node_features,
                    iterator,
                    node_type_name,
                    anchor_node_names,
                ))
            })
        {
            let (number_of_node_features, iterator, node_type_name, anchor_node_names) = result?;
            total_number_of_node_features += number_of_node_features;
            iterators.push(iterator);
            node_type_names.extend(vec![node_type_name; number_of_node_features as usize]);
            if let Some(anchor_node_names) = anchor_node_names {
                total_anchor_node_names.extend(anchor_node_names);
            }
        }
        Ok((
            total_number_of_node_features,
            iterators.into_iter().flat_map(|iter| iter),
            node_type_names,
            if return_sampled_node_names {
                Some(total_anchor_node_names)
            } else {
                None
            },
        ))
    }

    #[manual_binding]
    /// Return node-type-aware node embedding vector obtained from weighted shortest-paths.
    ///
    /// # Arguments
    /// * `node_centralities`: Option<Vec<f32>> - Vector with the importance of the nodes, used to properly sample the anchors. By default node degree centralities are used. Nodes with node centrality zero won't ever be sampled as an anchor, except for when all other nodes were already sampled.
    /// * `node_centralities_distribution`: Option<&str> - Distribution expected out of the provided node centralities distribution. If no distribution is provided and no node centralities are provided, exponential is used as the default node centrality distribution as node degree centrality is employed as centrality.
    /// * `adjust_by_central_node_distance`: Option<bool> - Whether to adjust the node eccentricity by the normalized distance to the most central node. By default true.
    /// * `number_of_nodes_to_sample_per_feature`: Option<NodeT> - Number of nodes to sample per feature. By default 10.
    /// * `maximum_number_of_features_per_node_type`: Option<usize> - Maximum number of node features to generate. By default 50.
    /// * `remove_neighbouring_nodes`: Option<bool> - Whether to remove the neighbouring nodes from the set of samplable anchor nodes. By default true.
    /// * `validate_node_centralities`: Option<bool> - Whether to validate the node centralities. By default true when the node centralities are provided.
    /// * `maximal_depth`: Option<NodeT> - The maximal depth to use if node features are to be focused in a local area of the graph.
    /// * `central_node_name`: Option<&str> - The node name from where to start sampling the BFS mask for the maximal depth. Either the node name of the node ID can be provided at once.
    /// * `central_node_id`: Option<NodeT> - The node ID from where to start sampling the BFS mask for the maximal depth. Either the node name of the node ID can be provided at once.
    /// * `use_edge_weights_as_probabilities`: Option<bool> - Whether to use the probabilities. By default false.
    /// * `random_state`: Option<u64> - The random state to use to sample the central node. By default 42.
    /// * `return_sampled_node_names`: Option<bool> - Whether to return the name of the sampled nodes. By default true if the `number_of_nodes_to_sample_per_feature` parameter is less than 100.
    /// * `verbose`: Option<bool> - Whether to show the loading bar. By default true.
    ///
    /// # Details on the supported node centrality distributions
    /// The node centrality distributions are used to find an optimal threshold that avoids
    /// sorting nodes that include also non-useful nodes, that is nodes that we will never
    /// be interested in sampling. We currently support the following node centrality distributions:
    ///
    /// ## Exponential
    /// The most common distribution for node centralities is the exponential distribution.
    /// Most likely, your node centralities will follow this distribution.
    ///
    /// ## Geometric
    /// The geometric distribution is to be used for an integer distribution, when the normalization
    /// by the distances from the most central node is disabled (or it will make the distribution a float value).
    ///
    /// ## Unknown
    /// For now we do not have support for other distributions implemented, so if the distribution
    /// is not expected to be one of the aforementioned distributions we will use zero as a threshold.
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
    pub fn get_weighted_shortest_paths_node_embedding_per_node_type(
        &self,
        node_centralities: Option<Vec<f32>>,
        node_centralities_distribution: Option<&str>,
        adjust_by_central_node_distance: Option<bool>,
        number_of_nodes_to_sample_per_feature: Option<NodeT>,
        maximum_number_of_features_per_node_type: Option<NodeT>,
        remove_neighbouring_nodes: Option<bool>,
        validate_node_centralities: Option<bool>,
        maximal_depth: Option<NodeT>,
        central_node_name: Option<&str>,
        central_node_id: Option<NodeT>,
        use_edge_weights_as_probabilities: Option<bool>,
        random_state: Option<u64>,
        return_sampled_node_names: Option<bool>,
        verbose: Option<bool>,
    ) -> Result<(
        NodeT,
        Vec<
            impl IndexedParallelIterator<Item = impl IndexedParallelIterator<Item = f32> + '_> + '_,
        >,
        Vec<String>,
        Option<Vec<Vec<String>>>,
    )> {
        let validate_node_centralities =
            validate_node_centralities.unwrap_or(node_centralities.is_some());
        let node_centralities = node_centralities.unwrap_or(self.get_degree_centrality()?);
        let return_sampled_node_names = return_sampled_node_names.unwrap_or(false);
        let verbose = verbose.unwrap_or(true);
        let pb = get_loading_bar(
            verbose,
            "Obtaining paths embedding per node-type",
            self.get_node_types_number()? as usize,
        );
        let mut node_type_names = Vec::new();
        let mut iterators = Vec::new();
        let mut total_anchor_node_names = Vec::new();
        let mut total_number_of_node_features = 0;
        for result in self
            .iter_unique_node_type_ids()?
            .zip(self.iter_unique_node_type_names()?)
            .progress_with(pb)
            .map(|(node_type_id, node_type_name)| {
                let (number_of_node_features, iterator, anchor_node_names) = self
                    .get_weighted_shortest_paths_node_embedding(
                        Some(
                            node_centralities
                                .par_iter()
                                .cloned()
                                .zip(self.par_iter_node_ids_and_node_type_ids())
                                .map(|(node_centrality, (_, node_type_ids))| {
                                    if let Some(node_type_ids) = node_type_ids {
                                        if node_type_ids.into_iter().any(|this_node_type_id| {
                                            this_node_type_id == node_type_id
                                        }) {
                                            return node_centrality;
                                        }
                                    }
                                    0.0
                                })
                                .collect::<Vec<f32>>(),
                        ),
                        node_centralities_distribution,
                        adjust_by_central_node_distance,
                        number_of_nodes_to_sample_per_feature,
                        maximum_number_of_features_per_node_type,
                        remove_neighbouring_nodes,
                        Some(validate_node_centralities),
                        maximal_depth,
                        central_node_name,
                        central_node_id,
                        use_edge_weights_as_probabilities,
                        random_state,
                        Some(return_sampled_node_names),
                        Some(verbose),
                    )?;
                Ok::<_, String>((
                    number_of_node_features,
                    iterator,
                    node_type_name,
                    anchor_node_names,
                ))
            })
        {
            let (number_of_node_features, iterator, node_type_name, anchor_node_names) = result?;
            total_number_of_node_features += number_of_node_features;
            iterators.push(iterator);
            node_type_names.extend(vec![node_type_name; number_of_node_features as usize]);
            if let Some(anchor_node_names) = anchor_node_names {
                total_anchor_node_names.extend(anchor_node_names);
            }
        }
        Ok((
            total_number_of_node_features,
            iterators,
            node_type_names,
            if return_sampled_node_names {
                Some(total_anchor_node_names)
            } else {
                None
            },
        ))
    }

    #[manual_binding]
    /// Return edge-type-aware node embedding vector obtained from shortest-paths.
    ///
    /// # Arguments
    /// * `node_centralities`: Option<Vec<f32>> - Vector with the importance of the nodes, used to properly sample the anchors. By default node degree centralities are used. Nodes with node centrality zero won't ever be sampled as an anchor, except for when all other nodes were already sampled.
    /// * `node_centralities_distribution`: Option<&str> - Distribution expected out of the provided node centralities distribution. If no distribution is provided and no node centralities are provided, exponential is used as the default node centrality distribution as node degree centrality is employed as centrality.
    /// * `adjust_by_central_node_distance`: Option<bool> - Whether to adjust the node eccentricity by the normalized distance to the most central node. By default true.
    /// * `number_of_nodes_to_sample_per_feature`: Option<NodeT> - Number of nodes to sample per feature. By default 10.
    /// * `maximum_number_of_features_per_node_type`: Option<usize> - Maximum number of node features to generate. By default 50.
    /// * `remove_neighbouring_nodes`: Option<bool> - Whether to remove the neighbouring nodes from the set of samplable anchor nodes. By default true.
    /// * `validate_node_centralities`: Option<bool> - Whether to validate the node centralities. By default true when the node centralities are provided.
    /// * `maximal_depth`: Option<NodeT> - The maximal depth to use if node features are to be focused in a local area of the graph.
    /// * `central_node_name`: Option<&str> - The node name from where to start sampling the BFS mask for the maximal depth. Either the node name of the node ID can be provided at once.
    /// * `central_node_id`: Option<NodeT> - The node ID from where to start sampling the BFS mask for the maximal depth. Either the node name of the node ID can be provided at once.
    /// * `random_state`: Option<u64> - The random state to use to sample the central node. By default 42.
    /// * `return_sampled_node_names`: Option<bool> - Whether to return the name of the sampled nodes. By default true if the `number_of_nodes_to_sample_per_feature` parameter is less than 100.
    /// * `verbose`: Option<bool> - Whether to show the loading bar. By default true.
    ///
    /// # Details on the supported node centrality distributions
    /// The node centrality distributions are used to find an optimal threshold that avoids
    /// sorting nodes that include also non-useful nodes, that is nodes that we will never
    /// be interested in sampling. We currently support the following node centrality distributions:
    ///
    /// ## Exponential
    /// The most common distribution for node centralities is the exponential distribution.
    /// Most likely, your node centralities will follow this distribution.
    ///
    /// ## Geometric
    /// The geometric distribution is to be used for an integer distribution, when the normalization
    /// by the distances from the most central node is disabled (or it will make the distribution a float value).
    ///
    /// ## Unknown
    /// For now we do not have support for other distributions implemented, so if the distribution
    /// is not expected to be one of the aforementioned distributions we will use zero as a threshold.
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
    pub fn get_shortest_paths_node_embedding_per_edge_type<
        'a,
        T: 'a + TryFrom<u32> + TryFrom<usize> + Send + Sync + IsInteger + Into<u32>,
    >(
        &'a self,
        node_centralities: Option<Vec<f32>>,
        node_centralities_distribution: Option<&str>,
        adjust_by_central_node_distance: Option<bool>,
        number_of_nodes_to_sample_per_feature: Option<NodeT>,
        maximum_number_of_features_per_edge_type: Option<NodeT>,
        remove_neighbouring_nodes: Option<bool>,
        validate_node_centralities: Option<bool>,
        maximal_depth: Option<T>,
        central_node_name: Option<&str>,
        central_node_id: Option<NodeT>,
        random_state: Option<u64>,
        return_sampled_node_names: Option<bool>,
        verbose: Option<bool>,
    ) -> Result<(
        NodeT,
        impl Iterator<Item = impl IndexedParallelIterator<Item = T> + '_> + '_,
        Vec<String>,
        Option<Vec<Vec<String>>>,
    )> {
        let validate_node_centralities =
            validate_node_centralities.unwrap_or(node_centralities.is_some());
        let node_centralities = node_centralities.unwrap_or(self.get_degree_centrality()?);
        let return_sampled_node_names = return_sampled_node_names.unwrap_or(false);
        let verbose = verbose.unwrap_or(true);
        let pb = get_loading_bar(
            verbose,
            "Obtaining paths embedding per edge-type",
            self.get_edge_types_number()? as usize,
        );
        let mut edge_type_names = Vec::new();
        let mut iterators = Vec::new();
        let mut total_anchor_node_names = Vec::new();
        let mut total_number_of_node_features = 0;
        for result in self
            .iter_unique_edge_type_ids()?
            .zip(self.iter_unique_edge_type_names()?)
            .progress_with(pb)
            .map(|(edge_type_id, edge_type_name)| {
                let (number_of_node_features, iterator, anchor_node_names) = self
                    .get_shortest_paths_node_embedding::<T>(
                        Some(
                            node_centralities
                                .par_iter()
                                .cloned()
                                .zip(self.par_iter_node_ids())
                                .map(|(node_centrality, node_id)| unsafe {
                                    if self
                                        .iter_unchecked_edge_types_from_source_node_id(node_id)
                                        .filter_map(|edge_type_id| edge_type_id)
                                        .any(|this_edge_type_id| this_edge_type_id == edge_type_id)
                                    {
                                        node_centrality
                                    } else {
                                        0.0
                                    }
                                })
                                .collect::<Vec<f32>>(),
                        ),
                        node_centralities_distribution,
                        adjust_by_central_node_distance,
                        number_of_nodes_to_sample_per_feature,
                        maximum_number_of_features_per_edge_type,
                        remove_neighbouring_nodes,
                        Some(validate_node_centralities),
                        maximal_depth,
                        central_node_name,
                        central_node_id,
                        random_state,
                        Some(return_sampled_node_names),
                        Some(verbose),
                    )?;
                Ok::<_, String>((
                    number_of_node_features,
                    iterator,
                    edge_type_name,
                    anchor_node_names,
                ))
            })
        {
            let (number_of_node_features, iterator, edge_type_name, anchor_node_names) = result?;
            total_number_of_node_features += number_of_node_features;
            iterators.push(iterator);
            edge_type_names.extend(vec![edge_type_name; number_of_node_features as usize]);
            if let Some(anchor_node_names) = anchor_node_names {
                total_anchor_node_names.extend(anchor_node_names);
            }
        }
        Ok((
            total_number_of_node_features,
            iterators.into_iter().flat_map(|iter| iter),
            edge_type_names,
            if return_sampled_node_names {
                Some(total_anchor_node_names)
            } else {
                None
            },
        ))
    }

    #[manual_binding]
    /// Return edge-type-aware node embedding vector obtained from weighted shortest-paths.
    ///
    /// # Arguments
    /// * `node_centralities`: Option<Vec<f32>> - Vector with the importance of the nodes, used to properly sample the anchors. By default node degree centralities are used. Nodes with node centrality zero won't ever be sampled as an anchor, except for when all other nodes were already sampled.
    /// * `node_centralities_distribution`: Option<&str> - Distribution expected out of the provided node centralities distribution. If no distribution is provided and no node centralities are provided, exponential is used as the default node centrality distribution as node degree centrality is employed as centrality.
    /// * `adjust_by_central_node_distance`: Option<bool> - Whether to adjust the node eccentricity by the normalized distance to the most central node. By default true.
    /// * `number_of_nodes_to_sample_per_feature`: Option<NodeT> - Number of nodes to sample per feature. By default 10.
    /// * `maximum_number_of_features_per_edge_type`: Option<usize> - Maximum number of node features to generate. By default 50.
    /// * `remove_neighbouring_nodes`: Option<bool> - Whether to remove the neighbouring nodes from the set of samplable anchor nodes. By default true.
    /// * `validate_node_centralities`: Option<bool> - Whether to validate the node centralities. By default true when the node centralities are provided.
    /// * `maximal_depth`: Option<NodeT> - The maximal depth to use if node features are to be focused in a local area of the graph.
    /// * `central_node_name`: Option<&str> - The node name from where to start sampling the BFS mask for the maximal depth. Either the node name of the node ID can be provided at once.
    /// * `central_node_id`: Option<NodeT> - The node ID from where to start sampling the BFS mask for the maximal depth. Either the node name of the node ID can be provided at once.
    /// * `use_edge_weights_as_probabilities`: Option<bool> - Whether to use the probabilities. By default false.
    /// * `random_state`: Option<u64> - The random state to use to sample the central node. By default 42.
    /// * `return_sampled_node_names`: Option<bool> - Whether to return the name of the sampled nodes. By default true if the `number_of_nodes_to_sample_per_feature` parameter is less than 100.
    /// * `verbose`: Option<bool> - Whether to show the loading bar. By default true.
    ///
    /// # Details on the supported node centrality distributions
    /// The node centrality distributions are used to find an optimal threshold that avoids
    /// sorting nodes that include also non-useful nodes, that is nodes that we will never
    /// be interested in sampling. We currently support the following node centrality distributions:
    ///
    /// ## Exponential
    /// The most common distribution for node centralities is the exponential distribution.
    /// Most likely, your node centralities will follow this distribution.
    ///
    /// ## Geometric
    /// The geometric distribution is to be used for an integer distribution, when the normalization
    /// by the distances from the most central node is disabled (or it will make the distribution a float value).
    ///
    /// ## Unknown
    /// For now we do not have support for other distributions implemented, so if the distribution
    /// is not expected to be one of the aforementioned distributions we will use zero as a threshold.
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
    pub fn get_weighted_shortest_paths_node_embedding_per_edge_type(
        &self,
        node_centralities: Option<Vec<f32>>,
        node_centralities_distribution: Option<&str>,
        adjust_by_central_node_distance: Option<bool>,
        number_of_nodes_to_sample_per_feature: Option<NodeT>,
        maximum_number_of_features_per_edge_type: Option<NodeT>,
        remove_neighbouring_nodes: Option<bool>,
        validate_node_centralities: Option<bool>,
        maximal_depth: Option<NodeT>,
        central_node_name: Option<&str>,
        central_node_id: Option<NodeT>,
        use_edge_weights_as_probabilities: Option<bool>,
        random_state: Option<u64>,
        return_sampled_node_names: Option<bool>,
        verbose: Option<bool>,
    ) -> Result<(
        NodeT,
        Vec<
            impl IndexedParallelIterator<Item = impl IndexedParallelIterator<Item = f32> + '_> + '_,
        >,
        Vec<String>,
        Option<Vec<Vec<String>>>,
    )> {
        let validate_node_centralities =
            validate_node_centralities.unwrap_or(node_centralities.is_some());
        let node_centralities = node_centralities.unwrap_or(self.get_degree_centrality()?);
        let return_sampled_node_names = return_sampled_node_names.unwrap_or(false);
        let verbose = verbose.unwrap_or(true);
        let pb = get_loading_bar(
            verbose,
            "Obtaining paths embedding per edge-type",
            self.get_edge_types_number()? as usize,
        );
        let mut edge_type_names = Vec::new();
        let mut iterators = Vec::new();
        let mut total_anchor_node_names = Vec::new();
        let mut total_number_of_node_features = 0;
        for result in self
            .iter_unique_edge_type_ids()?
            .zip(self.iter_unique_edge_type_names()?)
            .progress_with(pb)
            .map(|(edge_type_id, edge_type_name)| {
                let (number_of_node_features, iterator, anchor_node_names) = self
                    .get_weighted_shortest_paths_node_embedding(
                        Some(
                            node_centralities
                                .par_iter()
                                .cloned()
                                .zip(self.par_iter_node_ids())
                                .map(|(node_centrality, node_id)| unsafe {
                                    if self
                                        .iter_unchecked_edge_types_from_source_node_id(node_id)
                                        .filter_map(|edge_type_id| edge_type_id)
                                        .any(|this_edge_type_id| this_edge_type_id == edge_type_id)
                                    {
                                        node_centrality
                                    } else {
                                        0.0
                                    }
                                })
                                .collect::<Vec<f32>>(),
                        ),
                        node_centralities_distribution,
                        adjust_by_central_node_distance,
                        number_of_nodes_to_sample_per_feature,
                        maximum_number_of_features_per_edge_type,
                        remove_neighbouring_nodes,
                        Some(validate_node_centralities),
                        maximal_depth,
                        central_node_name,
                        central_node_id,
                        use_edge_weights_as_probabilities,
                        random_state,
                        Some(return_sampled_node_names),
                        Some(verbose),
                    )?;
                Ok::<_, String>((
                    number_of_node_features,
                    iterator,
                    edge_type_name,
                    anchor_node_names,
                ))
            })
        {
            let (number_of_node_features, iterator, edge_type_name, anchor_node_names) = result?;
            total_number_of_node_features += number_of_node_features;
            iterators.push(iterator);
            edge_type_names.extend(vec![edge_type_name; number_of_node_features as usize]);
            if let Some(anchor_node_names) = anchor_node_names {
                total_anchor_node_names.extend(anchor_node_names);
            }
        }
        Ok((
            total_number_of_node_features,
            iterators,
            edge_type_names,
            if return_sampled_node_names {
                Some(total_anchor_node_names)
            } else {
                None
            },
        ))
    }
}
