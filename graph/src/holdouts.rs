use super::*;
use indicatif::{ProgressBar, ProgressIterator, ProgressStyle};
use rand::rngs::SmallRng;
use rand::seq::SliceRandom;
use rand::SeedableRng;
use std::collections::{BTreeMap, HashSet};
use vec_rand::xorshift::xorshift;

/// # Holdouts.
impl Graph {
    /// Returns Graph with given amount of negative edges as positive edges.
    ///
    /// The graph generated may be used as a testing negatives partition to be
    /// fed into the argument "graph_to_avoid" of the link_prediction or the
    /// skipgrams algorithm.
    ///
    ///
    /// # Arguments
    ///
    /// * `seed`: EdgeT - Seed to use to reproduce negative edge set.
    /// * `negatives_number`: EdgeT - Number of negatives edges to include.
    /// * `allow_selfloops`: bool - Wethever to allow creation of selfloops or not.
    /// * `verbose`: bool - Wether to show the loading bar.
    ///
    pub fn sample_negatives(
        &self,
        mut seed: EdgeT,
        negatives_number: EdgeT,
        allow_selfloops: bool,
        verbose: bool,
    ) -> Result<Graph, String> {
        if negatives_number == 0 {
            return Err(String::from("The number of negatives cannot be zero."));
        }
        let total_negative_edges = self.get_nodes_number().pow(2)
            - self.get_edges_number()
            - if allow_selfloops {
                0
            } else {
                self.get_nodes_number() - self.selfloops_number()
            };

        if negatives_number > total_negative_edges {
            return Err(format!(
                concat!(
                    "The requested negatives number {} is more than the ",
                    "number of negative edges that exist in the graph ({})."
                ),
                negatives_number, total_negative_edges
            ));
        }

        let pb = if verbose {
            let pb = ProgressBar::new(negatives_number as u64);
            pb.set_draw_delta(negatives_number as u64 / 100);
            pb.set_style(ProgressStyle::default_bar().template(
                "Computing negative edges {spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] ({pos}/{len}, ETA {eta})",
            ));
            pb
        } else {
            ProgressBar::hidden()
        };

        // xorshift breaks if the seed is zero
        // so we initialize xor it with a constat
        // to mitigate this problem
        seed ^= SEED_XOR;

        // initialize the vectors for the result
        let mut unique_edges_tree: GraphDictionary = BTreeMap::new();

        // randomly extract negative edges until we have the choosen number
        while unique_edges_tree.len() <= negatives_number {
            seed = xorshift(seed as u64) as usize;
            let src: NodeT = self.sources[seed % self.sources.len()];
            seed = xorshift(seed as u64) as usize;
            let dst: NodeT = self.destinations[seed % self.sources.len()];
            // If the edge is not a self-loop or the user allows self-loops and
            // the graph is directed or the edges are inserted in a way to avoid
            // inserting bidirectional edges, avoiding to execute the check
            // of edge types so to insert them twice if the edge types are
            // different.
            if (allow_selfloops || src != dst)
                && !self.has_edge(src, dst)
                && !unique_edges_tree.contains_key(&(src, dst))
            {
                unique_edges_tree.insert((src, dst), None);
                pb.inc(1);
                if !self.is_directed {
                    unique_edges_tree.insert((dst, src), None);
                    pb.inc(1);
                }
            }
        }
        pb.finish();
        Ok(build_graph(
            &mut unique_edges_tree,
            self.nodes.clone(),
            self.node_types.clone(),
            if let Some(et) = &self.edge_types {
                Some(et.vocabulary.clone())
            } else {
                None
            },
            self.is_directed,
        ))
    }

    pub(crate) fn extend_tree(
        &self,
        tree: &mut GraphDictionary,
        src: NodeT,
        dst: NodeT,
        edge_type: Option<EdgeTypeT>,
        weight: Option<WeightT>,
        include_all_edge_types: bool,
    ) {
        let metadata = if let Some(md) = tree.get(&(src, dst)) {
            let mut metadata = md.to_owned();
            if let Some(md) = &mut metadata {
                md.add(weight, edge_type);
            }
            metadata
        } else {
            let mut metadata =
                ConstructorEdgeMetadata::new(self.has_weights(), self.has_edge_types());
            if let Some(md) = &mut metadata {
                if include_all_edge_types {
                    md.set(
                        self.get_link_weights(src, dst),
                        self.get_link_edge_types(src, dst),
                    );
                } else {
                    md.add(weight, edge_type);
                }
            }
            metadata
        };
        tree.insert((src, dst), metadata.clone());
        // If the current edge is not a self loop and the graph
        // is not directed, we add the simmetrical graph
        if !self.is_directed && src != dst {
            tree.insert((dst, src), metadata);
        }
    }

    fn holdout(
        &self,
        seed: NodeT,
        train_percentage: f64,
        include_all_edge_types: bool,
        user_condition: impl Fn(NodeT, NodeT, Option<EdgeTypeT>) -> bool,
        verbose: bool,
    ) -> Result<(Graph, Graph), String> {
        if train_percentage <= 0.0 || train_percentage >= 1.0 {
            return Err(String::from(
                "Train percentage must be strictly between 0 and 1.",
            ));
        }

        let valid_edges_number =
            (self.get_edges_number() as f64 * (1.0 - train_percentage)) as usize;

        let pb = if verbose {
            let pb = ProgressBar::new(self.get_edges_number() as u64);
            pb.set_draw_delta(self.get_edges_number() as u64 / 100);
            pb.set_style(ProgressStyle::default_bar().template(
                "Generating holdout {spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] ({pos}/{len}, ETA {eta})",
            ));
            pb
        } else {
            ProgressBar::hidden()
        };

        // generate and shuffle the indices of the edges
        let mut rng = SmallRng::seed_from_u64((seed ^ SEED_XOR) as u64);
        let mut edge_indices: Vec<NodeT> = (0..self.get_edges_number()).collect();
        edge_indices.shuffle(&mut rng);

        let mut train: GraphDictionary = GraphDictionary::new();
        let mut valid: GraphDictionary = GraphDictionary::new();

        for edge in edge_indices.iter().progress_with(pb) {
            let src = self.sources[*edge];
            let dst = self.destinations[*edge];

            if !self.is_directed && src > dst {
                continue;
            }

            let edge_type = if let Some(et) = &self.edge_types {
                Some(et.ids[*edge])
            } else {
                None
            };

            // Check if the edge with the considered edge type as already been added.
            if [&train, &valid]
                .iter()
                .any(|tree| match tree.get(&(src, dst)) {
                    Some(metadata) => {
                        if let Some(md) = metadata {
                            md.contains_edge_type(edge_type)
                        } else {
                            unreachable!(
                                "This is not reacheable as it would imply duplicated edges."
                            );
                        }
                    }
                    None => false,
                })
            {
                // The edge that is currently being considered as already
                // been added in the current heterogenous multi-graph
                // by the use of the parameter `include_all_edge_types`
                continue;
            }

            let weight = if let Some(w) = &self.weights {
                Some(w[*edge])
            } else {
                None
            };

            // We stop adding edges when we have reached the minimum amount.
            if user_condition(src, dst, edge_type) && valid.len() < valid_edges_number {
                self.extend_tree(
                    &mut valid,
                    src,
                    dst,
                    edge_type,
                    weight,
                    include_all_edge_types,
                );
            } else {
                // Otherwise we add the edges to the training set.
                self.extend_tree(
                    &mut train,
                    src,
                    dst,
                    edge_type,
                    weight,
                    include_all_edge_types,
                );
            }
        }

        Ok((
            build_graph(
                &mut train,
                self.nodes.clone(),
                self.node_types.clone(),
                if let Some(et) = &self.edge_types {
                    Some(et.vocabulary.clone())
                } else {
                    None
                },
                self.is_directed,
            ),
            build_graph(
                &mut valid,
                self.nodes.clone(),
                self.node_types.clone(),
                if let Some(et) = &self.edge_types {
                    Some(et.vocabulary.clone())
                } else {
                    None
                },
                self.is_directed,
            ),
        ))
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
    /// * `seed`:NodeT - The seed to use for the holdout,
    /// * `train_percentage`:f64 - Percentage target to reserve for training.
    /// * `include_all_edge_types`: bool - Wethever to include all the edges between two nodes.
    /// * `verbose`: bool - Wethever to show the loading bar.
    ///
    pub fn connected_holdout(
        &self,
        seed: NodeT,
        train_percentage: f64,
        include_all_edge_types: bool,
        verbose: bool,
    ) -> Result<(Graph, Graph), String> {
        if train_percentage <= 0.0 || train_percentage >= 1.0 {
            return Err(String::from(
                "Train percentage must be strictly between 0 and 1.",
            ));
        }
        let tree = self.spanning_tree(seed, include_all_edge_types);

        let edge_factor = if self.is_directed { 1 } else { 2 };
        let train_edges_number = (self.get_edges_number() as f64 * train_percentage) as usize;
        let valid_edges_number =
            (self.get_edges_number() as f64 * (1.0 - train_percentage)) as usize;

        if tree.len() * edge_factor > train_edges_number {
            return Err(format!(
                concat!(
                    "The given spanning tree of the graph contains {} edges ",
                    "that is more than the required training edges number {}.\n",
                    "This makes impossible to create a validation set using ",
                    "{} edges.\nIf possible, you should increase the ",
                    "train_percentage parameter which is currently equal to ",
                    "{}.\nThe deny map, by itself, is requiring at least ",
                    "a train percentage of {}."
                ),
                tree.len() * edge_factor,
                train_edges_number,
                valid_edges_number,
                train_percentage,
                (tree.len() * edge_factor) as f64 / train_edges_number as f64
            ));
        }

        self.holdout(
            seed,
            train_percentage,
            include_all_edge_types,
            |src, dst, edge_type| tree.contains(&(src, dst, edge_type)),
            verbose,
        )
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
    /// * `include_all_edge_types`: bool - Wethever to include all the edges between two nodes.
    /// * `verbose`: bool - Wethever to show the loading bar.
    ///
    pub fn random_holdout(
        &self,
        seed: NodeT,
        train_percentage: f64,
        include_all_edge_types: bool,
        verbose: bool,
    ) -> Result<(Graph, Graph), String> {
        self.holdout(
            seed,
            train_percentage,
            include_all_edge_types,
            |_, _, _| true,
            verbose,
        )
    }

    /// Returns subgraph with given number of nodes.
    ///
    /// This method creates a subset of the graph starting from a random node
    /// sampled using given seed and includes all neighbouring nodes until
    /// the required number of nodes is reached. All the edges connecting any
    /// of the selected nodes are then inserted into this graph.
    ///
    /// # Arguments
    ///
    /// * seed: usize - Random seed to use.
    /// * nodes_number: usize - Number of nodes to extract.
    /// * `verbose`: bool - Wethever to show the loading bar.
    ///
    pub fn random_subgraph(
        &self,
        seed: usize,
        nodes_number: usize,
        verbose: bool,
    ) -> Result<Graph, String> {
        if nodes_number <= 1 {
            return Err(String::from("Required nodes number must be more than 1."));
        }
        if nodes_number > self.get_nodes_number() {
            return Err(format!(
                concat!(
                    "Required number of nodes ({}) is more than available ",
                    "number of nodes ({}) in current graph."
                ),
                nodes_number,
                self.get_nodes_number()
            ));
        }

        let pb = if verbose {
            let pb = ProgressBar::new(self.get_edges_number() as u64);
            pb.set_draw_delta(self.get_edges_number() as u64 / 100);
            pb.set_style(ProgressStyle::default_bar().template(
                "Generating subgraph {spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] ({pos}/{len}, ETA {eta})",
            ));
            pb
        } else {
            ProgressBar::hidden()
        };

        // Creating the random number generator
        let mut rnd = SmallRng::seed_from_u64((seed ^ SEED_XOR) as u64);

        // Nodes indices
        let mut nodes: Vec<NodeT> = (0..self.get_nodes_number()).collect();

        // Shuffling the components using the given seed.
        nodes.shuffle(&mut rnd);

        // Initializing the vector for creating the new graph.
        let mut graph_data: GraphDictionary = GraphDictionary::new();

        // Initializing stack and set of nodes
        let mut unique_nodes: HashSet<NodeT> = HashSet::with_capacity(nodes_number);
        let mut stack: Vec<NodeT> = Vec::new();

        // We iterate on the components
        for node in nodes.iter().progress_with(pb) {
            if self.is_node_trap(*node) {
                continue;
            }
            stack.push(*node);
            unique_nodes.insert(*node);
            while !stack.is_empty() {
                let src = stack.pop().unwrap();
                let (min_edge, max_edge) = self.get_min_max_edge(src);
                for edge_id in min_edge..max_edge {
                    let dst: NodeT = self.destinations[edge_id];
                    if !unique_nodes.contains(&dst) {
                        stack.push(dst);
                        unique_nodes.insert(dst);
                        let mut metadata =
                            ConstructorEdgeMetadata::new(self.has_weights(), self.has_edge_types());
                        if let Some(md) = &mut metadata {
                            md.set(
                                self.get_link_weights(src, dst),
                                self.get_link_edge_types(src, dst),
                            );
                        }
                        graph_data.insert((src, dst), metadata.clone());
                        if !self.is_directed && src != dst {
                            graph_data.insert((dst, src), metadata);
                        }
                    }
                }
            }
        }

        Ok(build_graph(
            &mut graph_data,
            self.nodes.clone(),
            self.node_types.clone(),
            if let Some(et) = &self.edge_types {
                Some(et.vocabulary.clone())
            } else {
                None
            },
            self.is_directed,
        ))
    }

    /// Returns subgraph with given set of edge types.
    ///
    /// This method creates a subset of the graph by keeping also the edges
    /// of the given edge types.
    ///
    /// # Arguments
    ///
    /// * edge_types: Vec<String> - Vector of edge types to keep in the graph.
    /// * `verbose`: bool - Wethever to show the loading bar.
    ///
    pub fn edge_types_subgraph(
        &self,
        edge_types: Vec<String>,
        verbose: bool,
    ) -> Result<Graph, String> {
        if edge_types.is_empty() {
            return Err(String::from(
                "Required edge types must be a non-empty list.",
            ));
        }

        match &self.edge_types {
            None => Err(String::from("Current graph does not have edge types.")),
            Some(ets) => {
                let edge_type_ids = edge_types
                    .iter()
                    .map(|edge_type| match ets.get(edge_type) {
                        None => Err(format!(
                            "The edge type {} does not exist in current graph. The available edge types are {}.",
                            edge_type,
                            ets.keys().join(", ")
                        )),
                        Some(et) => Ok(*et),
                    })
                    .collect::<Result<HashSet<EdgeTypeT>, String>>()?;

                let pb = if verbose {
                    let pb = ProgressBar::new(self.get_edges_number() as u64);
                    pb.set_draw_delta(self.get_edges_number() as u64 / 100);
                    pb.set_style(ProgressStyle::default_bar().template(
                        "Generating subgraph {spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] ({pos}/{len}, ETA {eta})",
                    ));
                    pb
                } else {
                    ProgressBar::hidden()
                };

                // Initializing the vector for creating the new graph.
                let mut graph_data: GraphDictionary = GraphDictionary::new();

                (0..self.get_edges_number())
                    .progress_with(pb)
                    .map(|edge| {
                        let src = self.sources[edge];
                        let dst = self.destinations[edge];

                        let edge_type = ets.ids[edge];

                        let weight = if let Some(w) = &self.weights {
                            Some(w[edge])
                        } else {
                            None
                        };

                        (src, dst, edge_type, weight)
                    })
                    .filter(|(_, _, edge_type, _)| edge_type_ids.contains(edge_type))
                    .for_each(|(src, dst, edge_type, weight)| {
                        self.extend_tree(&mut graph_data, src, dst, Some(edge_type), weight, false)
                    });

                Ok(build_graph(
                    &mut graph_data,
                    self.nodes.clone(),
                    self.node_types.clone(),
                    Some(ets.vocabulary.clone()),
                    self.is_directed,
                ))
            }
        }
    }
}
