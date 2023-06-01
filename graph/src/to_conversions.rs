use rayon::prelude::*;

use super::*;
use crate::constructors::build_graph_from_integers;
use log::info;
use num_traits::Zero;

/// # Conversion of the graph.
impl Graph {
    /// Convert inplace the graph to directed.
    ///
    /// # Implementative details
    /// The conversion to a directed graph is trivial as only requires to
    /// switch the flag for directed to true.
    pub fn to_directed_inplace(&mut self) -> &mut Graph {
        self.directed = true;
        self
    }

    /// Return a new instance of the current graph as directed.
    pub fn to_directed(&self) -> Graph {
        let mut new_graph = self.clone();
        new_graph.to_directed_inplace();
        new_graph
    }

    /// Return the directed graph from the upper triangular adjacency matrix.
    ///
    /// # Implementative details
    /// Filtering a graph to the upper triangular matrix means that the
    /// resulting graph will exclusively have edges so that `dst > src`.
    ///
    pub fn to_upper_triangular(&self) -> Graph {
        build_graph_from_integers(
            Some(
                self.par_iter_directed_edge_node_ids_and_edge_type_id_and_edge_weight()
                    .filter_map(|(_, src, dst, edge_type, weight)| {
                        if dst > src {
                            Some((0, (src, dst, edge_type, weight.unwrap_or(WeightT::NAN))))
                        } else {
                            None
                        }
                    }),
            ),
            self.nodes.clone(),
            self.node_types.clone(),
            self.edge_types
                .as_ref()
                .as_ref()
                .map(|ets| ets.vocabulary.clone()),
            self.has_edge_weights(),
            true,
            Some(true),
            Some(false),
            Some(false),
            // TODO: possibly the edges number can be precomputed.
            None,
            true,
            true,
            self.get_name(),
        )
        .unwrap()
    }

    /// Return the directed graph from the lower triangular adjacency matrix.
    ///
    /// # Implementative details
    /// Filtering a graph to the lower triangular matrix means that the
    /// resulting graph will exclusively have edges so that `src > dst`.
    ///
    pub fn to_lower_triangular(&self) -> Graph {
        build_graph_from_integers(
            Some(
                self.par_iter_directed_edge_node_ids_and_edge_type_id_and_edge_weight()
                    .filter_map(|(_, src, dst, edge_type, weight)| {
                        if src > dst {
                            Some((0, (src, dst, edge_type, weight.unwrap_or(WeightT::NAN))))
                        } else {
                            None
                        }
                    }),
            ),
            self.nodes.clone(),
            self.node_types.clone(),
            self.edge_types
                .as_ref()
                .as_ref()
                .map(|ets| ets.vocabulary.clone()),
            self.has_edge_weights(),
            true,
            Some(true),
            Some(false),
            Some(false),
            // TODO: possibly the edges number can be precomputed.
            None,
            true,
            true,
            self.get_name(),
        )
        .unwrap()
    }

    /// Return the graph from the main diagonal adjacency matrix.
    ///
    /// # Implementative details
    /// The resulting graph will only contain the selfloops present in the
    /// original graph.
    ///
    pub fn to_main_diagonal(&self) -> Graph {
        build_graph_from_integers(
            Some(
                self.par_iter_directed_edge_node_ids_and_edge_type_id_and_edge_weight()
                    .filter_map(|(_, src, dst, edge_type, weight)| {
                        if src == dst {
                            Some((0, (src, dst, edge_type, weight.unwrap_or(WeightT::NAN))))
                        } else {
                            None
                        }
                    }),
            ),
            self.nodes.clone(),
            self.node_types.clone(),
            self.edge_types
                .as_ref()
                .as_ref()
                .map(|ets| ets.vocabulary.clone()),
            self.has_edge_weights(),
            self.is_directed(),
            Some(true),
            Some(false),
            Some(false),
            // TODO: possibly the edges number can be precomputed.
            None,
            true,
            true,
            self.get_name(),
        )
        .unwrap()
    }

    /// Return the graph from the anti-diagonal adjacency matrix.
    ///
    /// # Implementative details
    /// The resulting graph will include only the edges present on the
    /// anti-diagonal of the graph.
    ///
    pub fn to_anti_diagonal(&self) -> Graph {
        let number_of_nodes = self.get_number_of_nodes();
        build_graph_from_integers(
            Some(
                self.par_iter_directed_edge_node_ids_and_edge_type_id_and_edge_weight()
                    .filter_map(|(_, src, dst, edge_type, weight)| {
                        if src == number_of_nodes - dst {
                            Some((0, (src, dst, edge_type, weight.unwrap_or(WeightT::NAN))))
                        } else {
                            None
                        }
                    }),
            ),
            self.nodes.clone(),
            self.node_types.clone(),
            self.edge_types
                .as_ref()
                .as_ref()
                .map(|ets| ets.vocabulary.clone()),
            self.has_edge_weights(),
            self.is_directed(),
            Some(true),
            Some(false),
            Some(false),
            // TODO: possibly the edges number can be precomputed.
            None,
            true,
            true,
            self.get_name(),
        )
        .unwrap()
    }

    /// Return the graph from the bidiagonal adjacency matrix.
    ///
    /// # Implementative details
    /// The resulting graph will include only the edges present on either
    /// the diagonal or anti-diagonal matrix.
    ///
    pub fn to_bidiagonal(&self) -> Graph {
        let number_of_nodes = self.get_number_of_nodes();
        build_graph_from_integers(
            Some(
                self.par_iter_directed_edge_node_ids_and_edge_type_id_and_edge_weight()
                    .filter_map(|(_, src, dst, edge_type, weight)| {
                        if src == dst || src == number_of_nodes - dst {
                            Some((0, (src, dst, edge_type, weight.unwrap_or(WeightT::NAN))))
                        } else {
                            None
                        }
                    }),
            ),
            self.nodes.clone(),
            self.node_types.clone(),
            self.edge_types
                .as_ref()
                .as_ref()
                .map(|ets| ets.vocabulary.clone()),
            self.has_edge_weights(),
            self.is_directed(),
            Some(true),
            Some(false),
            Some(false),
            // TODO: possibly the edges number can be precomputed.
            None,
            true,
            true,
            self.get_name(),
        )
        .unwrap()
    }

    /// Return the graph from the arrowhead adjacency matrix.
    ///
    pub fn to_arrowhead(&self) -> Graph {
        build_graph_from_integers(
            Some(
                self.par_iter_directed_edge_node_ids_and_edge_type_id_and_edge_weight()
                    .filter_map(|(_, src, dst, edge_type, weight)| {
                        if src == 0 || dst == 0 || src == dst {
                            Some((0, (src, dst, edge_type, weight.unwrap_or(WeightT::NAN))))
                        } else {
                            None
                        }
                    }),
            ),
            self.nodes.clone(),
            self.node_types.clone(),
            self.edge_types
                .as_ref()
                .as_ref()
                .map(|ets| ets.vocabulary.clone()),
            self.has_edge_weights(),
            self.is_directed(),
            Some(true),
            Some(false),
            Some(false),
            // TODO: possibly the edges number can be precomputed.
            None,
            true,
            true,
            self.get_name(),
        )
        .unwrap()
    }

    /// Return the graph from the transposed adjacency matrix.
    ///
    pub fn to_transposed(&self) -> Graph {
        if !self.is_directed() {
            return self.clone();
        }
        build_graph_from_integers(
            Some(
                self.par_iter_directed_edge_node_ids_and_edge_type_id_and_edge_weight()
                    .map(|(_, src, dst, edge_type, weight)| {
                        (0, (dst, src, edge_type, weight.unwrap_or(WeightT::NAN)))
                    }),
            ),
            self.nodes.clone(),
            self.node_types.clone(),
            self.edge_types
                .as_ref()
                .as_ref()
                .map(|ets| ets.vocabulary.clone()),
            self.has_edge_weights(),
            self.is_directed(),
            Some(true),
            Some(false),
            Some(false),
            Some(self.get_number_of_directed_edges()),
            self.has_singleton_nodes(),
            self.has_singleton_nodes_with_selfloops(),
            self.get_name(),
        )
        .unwrap()
    }

    /// Return the graph with all symmetric edges.
    pub fn to_undirected(&self) -> Graph {
        if !self.is_directed() {
            return self.clone();
        }
        build_graph_from_integers(
            Some(
                self.par_iter_directed_edge_node_ids_and_edge_type_id_and_edge_weight()
                    .flat_map(|(_, src, dst, edge_type, weight)| {
                        vec![
                            (0, (dst, src, edge_type, weight.unwrap_or(WeightT::NAN))),
                            (0, (src, dst, edge_type, weight.unwrap_or(WeightT::NAN))),
                        ]
                    }),
            ),
            self.nodes.clone(),
            self.node_types.clone(),
            self.edge_types
                .as_ref()
                .as_ref()
                .map(|ets| ets.vocabulary.clone()),
            self.has_edge_weights(),
            false,
            Some(true),
            Some(true),
            Some(false),
            None,
            self.has_singleton_nodes(),
            self.has_singleton_nodes_with_selfloops(),
            self.get_name(),
        )
        .unwrap()
    }

    /// Return the complementary graph.
    ///
    /// # Implementative details
    /// Note that the resulting graph may require a significant amount
    /// of memory.
    ///
    pub fn to_complementary(&self) -> Graph {
        build_graph_from_integers(
            Some(self.par_iter_node_ids().flat_map(|src| {
                self.iter_node_ids()
                    .filter_map(|dst| {
                        if self.has_edge_from_node_ids(src, dst) {
                            None
                        } else {
                            Some((0, (src, dst, None, WeightT::NAN)))
                        }
                    })
                    .collect::<Vec<_>>()
            })),
            self.nodes.clone(),
            self.node_types.clone(),
            self.edge_types
                .as_ref()
                .as_ref()
                .map(|ets| ets.vocabulary.clone()),
            self.has_edge_weights(),
            self.is_directed(),
            Some(true),
            Some(false),
            Some(false),
            Some(
                (self.get_number_of_nodes() as EdgeT).pow(2)
                    - self.get_number_of_unique_directed_edges(),
            ),
            true,
            true,
            self.get_name(),
        )
        .unwrap()
    }

    /// Returns structural similarity multi-graph.
    ///
    /// # Arguments
    /// * `maximal_hop_distance`: Option<usize> - The maximal hop distance. By default, equal to the graph diameter. By default, equal to the diameter.
    /// * `change_layer_probability`: Option<f32> - Probability to change the layer during the random walk. By default 0.5.
    /// * `random_walk_length`: Option<u64> - Length of the random walk to be used to compute the approximated stationary distribution. By default, 1024.
    /// * `iterations`: Option<NodeT> - Number of iterations per node to compute the approximated stationary distribution. By default 1.
    ///
    /// # Raises
    /// * If the provided graph does not have any edges.
    /// * If the provided change layer probability is not a probability.
    /// * If the provided random walk parameters are not valid.
    pub fn to_structural_similarity_multi_graph(
        &self,
        maximal_hop_distance: Option<usize>,
        change_layer_probability: Option<f32>,
        random_walk_length: Option<u64>,
        iterations: Option<NodeT>,
    ) -> Result<Graph> {
        if !self.has_edges() {
            return Err("The current graph has no edges.".to_string());
        }
        let random_walk_length = random_walk_length.unwrap_or(1024);
        let maximal_hop_distance = maximal_hop_distance.unwrap_or_else(|| {
            info!("Computing diameter.");
            self.get_diameter(Some(true), Some(false)).unwrap() as usize
        });
        let number_of_layers = maximal_hop_distance + 1; // we also consider the 0

        let change_layer_probability = change_layer_probability.unwrap_or(0.5);
        if change_layer_probability.is_zero()
            || change_layer_probability < 0.0
            || change_layer_probability >= 1.0
        {
            return Err(
                "The change layer probability must be strictly positive and between 0 and 1."
                    .to_string(),
            );
        }

        info!("Creating and sorting reverse index.");

        // We want to create a list of node IDs that may be used
        // do determine the subset of neighbouring nodes with most
        // similar node degree.
        // This list is first sorted by node degree and secondarily by node ID.
        let mut reverse_index = self.get_node_ids();
        reverse_index.par_sort_unstable_by(|&first_node_id, &second_node_id| unsafe {
            (
                self.get_unchecked_node_degree_from_node_id(first_node_id),
                first_node_id,
            )
                .cmp(&(
                    self.get_unchecked_node_degree_from_node_id(second_node_id),
                    second_node_id,
                ))
        });

        let number_of_nodes = self.get_number_of_nodes() as usize;

        info!("Creating and sorting positions.");
        // this is the reverse-(reverse-index) and stores the index in the reverse_index of a given node
        // which means that positions[node_id] returns its index in the sorted array
        let positions = ThreadDataRaceAware::new(vec![0; number_of_nodes]);
        reverse_index
            .par_iter()
            .copied()
            .enumerate()
            .for_each(|(i, node_id)| unsafe {
                *(*positions.get()).get_unchecked_mut(node_id as usize) = i as NodeT;
            });
        let positions = positions.into_inner();

        let half_number_of_destinations = (self.get_number_of_nodes() as f32).ln().ceil() as usize;
        // it's the degree of the node on the multigraph composed by all the layers
        let number_of_destinations = 2 * half_number_of_destinations;
        let number_of_edges_per_node = number_of_destinations * number_of_layers;
        let number_of_edges_per_layer = number_of_destinations * number_of_nodes;

        info!("Allocating edge weights.");
        let mut weights: Vec<WeightT> = vec![0.0; number_of_edges_per_node * number_of_nodes];
        let total_number_of_edges = number_of_edges_per_layer;

        let get_destinations = |position_idx: usize| {
            // First we get the node IDs range.
            let mut min_node_id = position_idx.saturating_sub(1 + half_number_of_destinations);
            // Note that we have not yet MODDED the max node id to the number of nodes!
            let mut max_node_id = min_node_id + half_number_of_destinations * 2 + 1;

            if max_node_id > number_of_nodes {
                min_node_id -= max_node_id - number_of_nodes;
                max_node_id = number_of_nodes;
            }

            &reverse_index[min_node_id..max_node_id]
        };

        info!("Computing structural similarities.");
        positions
            .par_iter()
            .copied()
            .map(|position| (position as usize, reverse_index[position as usize]))
            .zip(weights.par_chunks_mut(number_of_edges_per_node))
            .for_each(|((position, src), src_edge_weights)| {
                get_destinations(position)
                    .iter()
                    .copied()
                    .filter(move |&dst| dst != src)
                    .zip(src_edge_weights.chunks_mut(number_of_layers))
                    .for_each(|(dst, src_layer_weights)| unsafe {
                        self.get_unchecked_structural_distance_from_node_ids(
                            src,
                            dst,
                            maximal_hop_distance,
                        )
                        .into_iter()
                        .zip(src_layer_weights.iter_mut())
                        .for_each(|(cost, src_layer_weight)| {
                            *src_layer_weight = (-cost).exp();
                        });
                    });
            });

        info!("Computing average edge weight.");
        let mut average_weights = weights
            .par_chunks(number_of_layers)
            .map(|weights| weights.to_vec())
            .reduce(
                || vec![0.0; number_of_layers],
                |mut a, b| {
                    a.iter_mut().zip(b.iter()).for_each(|(a_weight, b_weight)| {
                        *a_weight += b_weight;
                    });
                    a
                },
            );

        average_weights.iter_mut().for_each(|weight| {
            *weight /= number_of_edges_per_layer as f32;
        });

        // We create the dictionary of nodes we will be using for
        // constructing the graph to compute the stationary distribution.
        let nodes = Vocabulary::from_range(0..number_of_layers as NodeT, "Nodes".to_string());

        // We compute the number of edges in the layer transition graph.
        let number_of_edges = (number_of_layers + (number_of_layers - 1) * 2) as EdgeT;
        let walk_parameters =
            WalksParameters::new(random_walk_length)?.set_iterations(iterations)?;

        info!("Normalizing weights and computing stationary layer switching distribution.");
        // We proceed to normalize for each node the weights and
        // to compute the stationary distribution of the layer switches
        // so we can directly weight the various layers of a given
        // node of the appropriate amount to obtain an asintotically identical
        // distribution of sampling as if we were using the
        // more complex layer switching described in the paper.
        weights
            .par_chunks_mut(number_of_edges_per_node)
            .for_each(|weights| {
                // First of all, we count the number of weights for each of the layers
                // that is higher than the global average of that layer.
                let counts: Vec<NodeT> = weights.chunks(number_of_layers).fold(
                    vec![0; number_of_layers],
                    |mut partial_count, weights| {
                        partial_count
                            .iter_mut()
                            .zip(weights.iter().zip(average_weights.iter()))
                            .for_each(|(partial_count, (weight, average_weight))| {
                                if weight > average_weight {
                                    *partial_count += 1
                                }
                            });
                        partial_count
                    },
                );
                // We compute the total weights and then proceed to normalize layer wise.
                let total_weights: Vec<WeightT> = weights.chunks(number_of_layers).fold(
                    vec![0.0; number_of_layers],
                    |mut partial_sum, weights| {
                        partial_sum.iter_mut().zip(weights.iter()).for_each(
                            |(partial_sum, weight)| {
                                *partial_sum += weight;
                            },
                        );
                        partial_sum
                    },
                );

                // Convert the counts to the probability of transition from a lower
                // layer to an upper layer.
                let probability_of_transition_to_superior_layer = counts
                    .into_iter()
                    .map(|count| {
                        let w = (count as f32 + std::f32::consts::E).ln();
                        w / (1.0 + w)
                    })
                    .collect::<Vec<f32>>();

                // Create the transitions graph to compute the stationary distribution
                // to normalize the weights.
                let graph = build_graph_from_integers(
                    Some(
                        probability_of_transition_to_superior_layer
                            .into_par_iter()
                            .enumerate()
                            .flat_map(|(src, weight)| {
                                let src = src as NodeT;
                                // start of the chain
                                let forward_change_layer_probability =
                                    weight * change_layer_probability;
                                let backward_change_layer_probability =
                                    (1.0 - weight) * change_layer_probability;
                                if src == 0 {
                                    vec![
                                        // self loop
                                        (0, (0, 0, None, 1.0 - forward_change_layer_probability)),
                                        // forward edge
                                        (1, (0, 1, None, forward_change_layer_probability)),
                                    ]
                                } else if src == number_of_layers as NodeT - 1 {
                                    // end of the chain
                                    vec![
                                        // backward edge
                                        (
                                            number_of_edges as usize - 2,
                                            (src, src - 1, None, forward_change_layer_probability),
                                        ),
                                        // selfloop
                                        (
                                            number_of_edges as usize - 1,
                                            (
                                                src,
                                                src,
                                                None,
                                                1.0 - forward_change_layer_probability,
                                            ),
                                        ),
                                    ]
                                } else {
                                    // inner nodes in the chain
                                    let min_edge_id = ((src - 1) * 3 + 2) as usize;
                                    vec![
                                        // backward edge
                                        (
                                            min_edge_id,
                                            (src, src - 1, None, backward_change_layer_probability),
                                        ),
                                        // selfloop
                                        (
                                            min_edge_id + 1,
                                            (src, src, None, (1.0 - change_layer_probability)),
                                        ),
                                        // forward edge
                                        (
                                            min_edge_id + 2,
                                            (src, src + 1, None, forward_change_layer_probability),
                                        ),
                                    ]
                                }
                            }),
                    ),
                    Arc::new(nodes.clone()),
                    Arc::new(None),
                    None,
                    true,
                    true,
                    Some(true),
                    Some(false),
                    Some(true),
                    Some(number_of_edges),
                    false,
                    false,
                    "Layer Transition",
                )
                .unwrap();

                let visits_per_node: Vec<usize> = graph
                    .iter_complete_walks(&walk_parameters)
                    .unwrap()
                    .fold(vec![0; number_of_layers], |mut counts, walk| {
                        walk.into_iter().for_each(|node_id| {
                            counts[node_id as usize] += 1;
                        });
                        counts
                    });

                let total_visits = visits_per_node
                    .iter()
                    .copied()
                    .map(|visits| visits)
                    .sum::<usize>() as f32;

                // Finally normalize the visits by the total and obtain the
                // approximated stationary distribution.
                let stationary_distribution: Vec<f32> = visits_per_node
                    .into_iter()
                    .map(|visits| visits as f32 / total_visits)
                    .collect();

                // Nomalize the weights layer wise.
                weights.chunks_mut(number_of_layers).for_each(|weights| {
                    weights
                        .iter_mut()
                        .zip(
                            total_weights
                                .iter()
                                .copied()
                                .zip(stationary_distribution.iter().copied())
                                .map(|(total_weight, probability)| {
                                    probability / (total_weight + f32::EPSILON)
                                }),
                        )
                        .for_each(|(weight, probability)| {
                            *weight *= probability;
                        });
                });
            });

        info!("Building the structural similarity graph.");
        build_graph_from_integers(
            Some(
                positions
                    .par_iter()
                    .copied()
                    .map(|position| (position as usize, reverse_index[position as usize]))
                    .zip(weights.par_chunks(number_of_edges_per_node))
                    .flat_map(move |((position, src), weights)| {
                        let destinations = get_destinations(position);
                        let mut destinations_reverse_index =
                            (0..destinations.len() as NodeT).collect::<Vec<NodeT>>();
                        destinations_reverse_index.sort_unstable_by(|&a, &b| {
                            destinations[a as usize].cmp(&destinations[b as usize])
                        });

                        let source_reverse_index = destinations_reverse_index
                            .iter()
                            .find(|&&reverse_index| destinations[reverse_index as usize] == src)
                            .unwrap()
                            .clone() as usize;

                        destinations_reverse_index
                            .into_iter()
                            .filter_map(|destination_reverse_index| {
                                let mut destination_reverse_index =
                                    destination_reverse_index as usize;
                                let dst = destinations[destination_reverse_index];
                                if dst == src {
                                    None
                                } else {
                                    // We need to account for the selfloop, which
                                    // we need to remove. This check is not equal to
                                    // checking whether dst > src because the weights
                                    // are not sorted according to their node IDs, but
                                    // their node degrees to have a maximal structural similarity.
                                    if destination_reverse_index > source_reverse_index {
                                        destination_reverse_index -= 1;
                                    }
                                    Some((
                                        dst,
                                        &weights[destination_reverse_index * number_of_layers
                                            ..(destination_reverse_index + 1) * number_of_layers],
                                    ))
                                }
                            })
                            .enumerate()
                            .map(move |(i, (dst, weights))| {
                                (
                                    (src as usize) * number_of_destinations + i,
                                    (
                                        src,
                                        dst,
                                        None,
                                        weights.iter().copied().sum::<f32>() + f32::EPSILON,
                                    ),
                                )
                            })
                            .collect::<Vec<_>>()
                    }),
            ),
            self.nodes.clone(),
            self.node_types.clone(),
            None,
            true,
            true,
            Some(true),
            Some(false),
            Some(true),
            Some(total_number_of_edges as EdgeT),
            false,
            false,
            format!("Structural {}", self.get_name()),
        )
    }
}
