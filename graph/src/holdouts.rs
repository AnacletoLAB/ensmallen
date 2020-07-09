use super::types::*;
use super::Graph;
use super::SEED_XOR;
use hashbrown::HashSet;
use rand::rngs::SmallRng;
use rand::seq::SliceRandom;
use rand::SeedableRng;
use vec_rand::xorshift::xorshift as rand_u64;

/// # Holdouts.
impl Graph {
    fn copy_from_index(
        &self,
        index: EdgeT,
        sources: &mut Vec<NodeT>,
        destinations: &mut Vec<NodeT>,
        weights: &mut Vec<WeightT>,
        edge_types: &mut Vec<EdgeTypeT>,
    ) {
        let src = self.sources[index];
        let dst = self.destinations[index];
        sources.push(src);
        destinations.push(dst);
        if let Some(w) = &self.weights {
            weights.push(w[index]);
        }
        if let Some(et) = &self.edge_types {
            edge_types.push(et[index]);
        }
    }

    /// Returns Graph with given amount of negative edges as positive edges.
    ///
    /// The graph generated may be used as a testing negatives partition to be
    /// fed into the argument "graph_to_avoid" of the link_prediction or the
    /// skipgrams algorithm.
    ///
    ///
    /// # Arguments
    ///
    /// * seed: EdgeT - Seed to use to reproduce negative edge set.
    /// * negatives_number: EdgeT - Number of negatives edges to include.
    /// * allow_selfloops: EdgeT - Wethever to allow creation of selfloops or not.
    ///
    pub fn sample_negatives(
        &self,
        seed: EdgeT,
        negatives_number: EdgeT,
        allow_selfloops: bool,
    ) -> Result<Graph, String> {
        let _negatives_number = if !self.is_directed {
            negatives_number / 2
        } else {
            negatives_number
        };

        let mut unique_edges: HashSet<(NodeT, NodeT)> = HashSet::with_capacity(_negatives_number);

        // initialize the vectors for the result
        let mut sources: Vec<NodeT> = Vec::with_capacity(_negatives_number);
        let mut destinations: Vec<NodeT> = Vec::with_capacity(_negatives_number);

        let mut new_seed = seed ^ SEED_XOR;

        loop {
            new_seed = rand_u64(new_seed as u64) as usize;
            let src: NodeT = self.sources[new_seed % self.get_edges_number()];
            new_seed = rand_u64(new_seed as u64) as usize;
            let dst: NodeT = self.destinations[new_seed % self.get_edges_number()];
            if !unique_edges.contains(&(src, dst))
                && (self.is_directed || !unique_edges.contains(&(dst, src)))
                && (allow_selfloops || src != dst)
                && !self.has_edge(src, dst)
            {
                unique_edges.insert((src, dst));
                sources.push(src);
                destinations.push(dst);
            }
            if unique_edges.len() == _negatives_number {
                break;
            }
        }

        Ok(if self.is_directed {
            Graph::new_directed(
                sources,
                destinations,
                Some(self.nodes_mapping.clone()),
                Some(self.nodes_reverse_mapping.clone()),
                self.node_types.clone(),
                self.node_types_mapping.clone(),
                self.node_types_reverse_mapping.clone(),
                None,
                self.edge_types_mapping.clone(),
                self.edge_types_reverse_mapping.clone(),
                None,
            )?
        } else {
            Graph::new_undirected(
                sources,
                destinations,
                Some(self.nodes_mapping.clone()),
                Some(self.nodes_reverse_mapping.clone()),
                self.node_types.clone(),
                self.node_types_mapping.clone(),
                self.node_types_reverse_mapping.clone(),
                None,
                self.edge_types_mapping.clone(),
                self.edge_types_reverse_mapping.clone(),
                None,
                None,
            )?
        })
    }

    /// Returns holdout for training ML algorithms on the graph structure.
    ///
    /// The holdouts returned are a tuple of graphs. The first one, which
    /// is the training graph, is garanteed to have the same number of
    /// graph components as the initial graph. The second graph is the graph
    /// meant for testing or validation of the algorithm, and has no garantee
    /// to be connected. It will have at most (1-train_percentage) edges,
    /// as the bound of connectivity which is required for the training graph
    /// may lead to more edges being left into the training partition.
    ///
    /// # Arguments
    ///
    /// * seed:NodeT - The seed to use for the holdout,
    /// * train_percentage:f64 - Percentage target to reserve for training
    ///
    pub fn connected_holdout(
        &self,
        seed: NodeT,
        train_percentage: f64,
    ) -> Result<(Graph, Graph), String> {
        if train_percentage <= 0.0 || train_percentage >= 1.0 {
            return Err(String::from(
                "Given train percentage must be strictly between 0 and 1.",
            ));
        }

        let tree: HashSet<(NodeT, NodeT, Option<EdgeTypeT>)> = self.spanning_tree(seed);
        let mut used_edges: HashSet<(NodeT, NodeT, Option<EdgeTypeT>)> = HashSet::new();

        // generate and shuffle the indices of the edges
        let mut rng = SmallRng::seed_from_u64((seed ^ SEED_XOR) as u64);
        let mut edge_indices: Vec<NodeT> = (0..self.get_edges_number()).collect();
        edge_indices.shuffle(&mut rng);

        let valid_edges_number =
            (self.get_edges_number() as f64 * (1.0 - train_percentage)) as usize;
        let train_edges_number = (self.get_edges_number() as f64 * train_percentage) as usize;
        let mut valid_edges_number_total = 0;

        let mut valid_sources: Vec<NodeT> = Vec::with_capacity(valid_edges_number);
        let mut valid_destinations: Vec<NodeT> = Vec::with_capacity(valid_edges_number);
        let mut valid_weights: Vec<WeightT> = Vec::with_capacity(valid_edges_number);
        let mut valid_edge_types: Vec<EdgeTypeT> = Vec::with_capacity(valid_edges_number);

        let mut train_sources: Vec<NodeT> = Vec::with_capacity(train_edges_number);
        let mut train_destinations: Vec<NodeT> = Vec::with_capacity(train_edges_number);
        let mut train_weights: Vec<WeightT> = Vec::with_capacity(train_edges_number);
        let mut train_edge_types: Vec<EdgeTypeT> = Vec::with_capacity(train_edges_number);

        for edge in edge_indices.iter() {
            let src = self.sources[*edge];
            let dst = self.destinations[*edge];
            let edge_type = if let Some(et) = &self.edge_types {
                Some(et[*edge])
            } else {
                None
            };
            // If the spanning tree does not include the current edge
            // and, if we are in an undirected graph, does not include neither
            // the graph in the opposite direction:
            if !(tree.contains(&(src, dst, edge_type))
                || !self.is_directed && tree.contains(&(dst, src, edge_type)))
            {
                // We stop adding edges when we have reached the minimum amount.
                if valid_edges_number_total < valid_edges_number
                    && (self.is_directed || !used_edges.contains(&(dst, src, edge_type)))
                {
                    // add the edge
                    self.copy_from_index(
                        *edge,
                        &mut valid_sources,
                        &mut valid_destinations,
                        &mut valid_weights,
                        &mut valid_edge_types,
                    );
                    used_edges.insert((src, dst, edge_type));
                    valid_edges_number_total += 1;
                    if !self.is_directed {
                        valid_edges_number_total += 1;
                    }
                    continue;
                }
            }
            // Otherwise we add the edges to the training set.
            //
            // When the graph is directed we need to check that the edge
            // in the opposite direction was not already inserted.
            if self.is_directed || !used_edges.contains(&(dst, src, edge_type)) {
                used_edges.insert((src, dst, edge_type));
                //println!("Training {}, {}", self.nodes_reverse_mapping[src], self.nodes_reverse_mapping[dst]);
                self.copy_from_index(
                    *edge,
                    &mut train_sources,
                    &mut train_destinations,
                    &mut train_weights,
                    &mut train_edge_types,
                );
            }
        }

        Ok(if self.is_directed {
            (
                Graph::new_directed(
                    train_sources,
                    train_destinations,
                    Some(self.nodes_mapping.clone()),
                    Some(self.nodes_reverse_mapping.clone()),
                    self.node_types.clone(),
                    self.node_types_mapping.clone(),
                    self.node_types_reverse_mapping.clone(),
                    if self.edge_types.is_some() {
                        Some(train_edge_types)
                    } else {
                        None
                    },
                    self.edge_types_mapping.clone(),
                    self.edge_types_reverse_mapping.clone(),
                    if self.weights.is_some() {
                        Some(train_weights)
                    } else {
                        None
                    },
                )?,
                Graph::new_directed(
                    valid_sources,
                    valid_destinations,
                    Some(self.nodes_mapping.clone()),
                    Some(self.nodes_reverse_mapping.clone()),
                    self.node_types.clone(),
                    self.node_types_mapping.clone(),
                    self.node_types_reverse_mapping.clone(),
                    if self.edge_types.is_some() {
                        Some(valid_edge_types)
                    } else {
                        None
                    },
                    self.edge_types_mapping.clone(),
                    self.edge_types_reverse_mapping.clone(),
                    if self.weights.is_some() {
                        Some(valid_weights)
                    } else {
                        None
                    },
                )?,
            )
        } else {
            (
                Graph::new_undirected(
                    train_sources,
                    train_destinations,
                    Some(self.nodes_mapping.clone()),
                    Some(self.nodes_reverse_mapping.clone()),
                    self.node_types.clone(),
                    self.node_types_mapping.clone(),
                    self.node_types_reverse_mapping.clone(),
                    if self.edge_types.is_some() {
                        Some(train_edge_types)
                    } else {
                        None
                    },
                    self.edge_types_mapping.clone(),
                    self.edge_types_reverse_mapping.clone(),
                    if self.weights.is_some() {
                        Some(train_weights)
                    } else {
                        None
                    },
                    None,
                )?,
                Graph::new_undirected(
                    valid_sources,
                    valid_destinations,
                    Some(self.nodes_mapping.clone()),
                    Some(self.nodes_reverse_mapping.clone()),
                    self.node_types.clone(),
                    self.node_types_mapping.clone(),
                    self.node_types_reverse_mapping.clone(),
                    if self.edge_types.is_some() {
                        Some(valid_edge_types)
                    } else {
                        None
                    },
                    self.edge_types_mapping.clone(),
                    self.edge_types_reverse_mapping.clone(),
                    if self.weights.is_some() {
                        Some(valid_weights)
                    } else {
                        None
                    },
                    None,
                )?,
            )
        })
    }
    /// Returns random holdout for training ML algorithms on the graph edges.
    ///
    /// The holdouts returned are a tuple of graphs. In neither holdouts the
    /// graph connectivity is necessarily preserved. To maintain that, use
    /// the method `connected_holdout`.
    ///
    /// # Arguments
    ///
    /// * seed:NodeT - The seed to use for the holdout,
    /// * train_percentage:f64 - Percentage target to reserve for training
    ///
    pub fn random_holdout(
        &self,
        seed: NodeT,
        train_percentage: f64,
    ) -> Result<(Graph, Graph), String> {
        if train_percentage <= 0.0 || train_percentage >= 1.0 {
            return Err(String::from(
                "Given train percentage must be strictly between 0 and 1.",
            ));
        }

        let mut used_edges: HashSet<(NodeT, NodeT, Option<EdgeTypeT>)> = HashSet::new();

        // generate and shuffle the indices of the edges
        let mut rng = SmallRng::seed_from_u64((seed ^ SEED_XOR) as u64);
        let mut edge_indices: Vec<NodeT> = (0..self.get_edges_number()).collect();
        edge_indices.shuffle(&mut rng);

        let valid_edges_number =
            (self.get_edges_number() as f64 * (1.0 - train_percentage)) as usize;
        let train_edges_number = (self.get_edges_number() as f64 * train_percentage) as usize;
        let mut valid_edges_number_total = 0;

        let mut valid_sources: Vec<NodeT> = Vec::with_capacity(valid_edges_number);
        let mut valid_destinations: Vec<NodeT> = Vec::with_capacity(valid_edges_number);
        let mut valid_weights: Vec<WeightT> = Vec::with_capacity(valid_edges_number);
        let mut valid_edge_types: Vec<EdgeTypeT> = Vec::with_capacity(valid_edges_number);

        let mut train_sources: Vec<NodeT> = Vec::with_capacity(train_edges_number);
        let mut train_destinations: Vec<NodeT> = Vec::with_capacity(train_edges_number);
        let mut train_weights: Vec<WeightT> = Vec::with_capacity(train_edges_number);
        let mut train_edge_types: Vec<EdgeTypeT> = Vec::with_capacity(train_edges_number);

        for edge in edge_indices.iter() {
            let src = self.sources[*edge];
            let dst = self.destinations[*edge];
            let edge_type = if let Some(et) = &self.edge_types {
                Some(et[*edge])
            } else {
                None
            };
            // We stop adding edges when we have reached the minimum amount.
            if valid_edges_number_total < valid_edges_number
                && (self.is_directed || !used_edges.contains(&(dst, src, edge_type)))
            {
                // add the edge
                self.copy_from_index(
                    *edge,
                    &mut valid_sources,
                    &mut valid_destinations,
                    &mut valid_weights,
                    &mut valid_edge_types,
                );
                used_edges.insert((src, dst, edge_type));
                valid_edges_number_total += 1;
                if !self.is_directed {
                    valid_edges_number_total += 1;
                }
                continue;
            }
            // Otherwise we add the edges to the training set.
            //
            // When the graph is directed we need to check that the edge
            // in the opposite direction was not already inserted.
            if self.is_directed || !used_edges.contains(&(dst, src, edge_type)) {
                used_edges.insert((src, dst, edge_type));
                //println!("Training {}, {}", self.nodes_reverse_mapping[src], self.nodes_reverse_mapping[dst]);
                self.copy_from_index(
                    *edge,
                    &mut train_sources,
                    &mut train_destinations,
                    &mut train_weights,
                    &mut train_edge_types,
                );
            }
        }

        Ok(if self.is_directed {
            (
                Graph::new_directed(
                    train_sources,
                    train_destinations,
                    Some(self.nodes_mapping.clone()),
                    Some(self.nodes_reverse_mapping.clone()),
                    self.node_types.clone(),
                    self.node_types_mapping.clone(),
                    self.node_types_reverse_mapping.clone(),
                    if self.edge_types.is_some() {
                        Some(train_edge_types)
                    } else {
                        None
                    },
                    self.edge_types_mapping.clone(),
                    self.edge_types_reverse_mapping.clone(),
                    if self.weights.is_some() {
                        Some(train_weights)
                    } else {
                        None
                    },
                )?,
                Graph::new_directed(
                    valid_sources,
                    valid_destinations,
                    Some(self.nodes_mapping.clone()),
                    Some(self.nodes_reverse_mapping.clone()),
                    self.node_types.clone(),
                    self.node_types_mapping.clone(),
                    self.node_types_reverse_mapping.clone(),
                    if self.edge_types.is_some() {
                        Some(valid_edge_types)
                    } else {
                        None
                    },
                    self.edge_types_mapping.clone(),
                    self.edge_types_reverse_mapping.clone(),
                    if self.weights.is_some() {
                        Some(valid_weights)
                    } else {
                        None
                    },
                )?,
            )
        } else {
            (
                Graph::new_undirected(
                    train_sources,
                    train_destinations,
                    Some(self.nodes_mapping.clone()),
                    Some(self.nodes_reverse_mapping.clone()),
                    self.node_types.clone(),
                    self.node_types_mapping.clone(),
                    self.node_types_reverse_mapping.clone(),
                    if self.edge_types.is_some() {
                        Some(train_edge_types)
                    } else {
                        None
                    },
                    self.edge_types_mapping.clone(),
                    self.edge_types_reverse_mapping.clone(),
                    if self.weights.is_some() {
                        Some(train_weights)
                    } else {
                        None
                    },
                    None,
                )?,
                Graph::new_undirected(
                    valid_sources,
                    valid_destinations,
                    Some(self.nodes_mapping.clone()),
                    Some(self.nodes_reverse_mapping.clone()),
                    self.node_types.clone(),
                    self.node_types_mapping.clone(),
                    self.node_types_reverse_mapping.clone(),
                    if self.edge_types.is_some() {
                        Some(valid_edge_types)
                    } else {
                        None
                    },
                    self.edge_types_mapping.clone(),
                    self.edge_types_reverse_mapping.clone(),
                    if self.weights.is_some() {
                        Some(valid_weights)
                    } else {
                        None
                    },
                    None,
                )?,
            )
        })
    }
}
