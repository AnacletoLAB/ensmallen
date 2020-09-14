use super::*;
use rand::rngs::SmallRng;
use rand::seq::SliceRandom;
use rand::SeedableRng;
use std::collections::{BTreeMap, HashSet};
use vec_rand::xorshift::xorshift;

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
            edge_types.push(et.ids[index]);
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
        mut seed: EdgeT,
        negatives_number: EdgeT,
        allow_selfloops: bool,
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

        // xorshift breaks if the seed is zero
        // so we initialize xor it with a constat
        // to mitigate this problem
        seed ^= SEED_XOR;

        // initialize the vectors for the result
        let mut unique_edges_tree: GraphDictionary = BTreeMap::new();

        // randomly extract negative edges until we have the choosen number
        while unique_edges_tree.len() == negatives_number {
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
                if !self.is_directed {
                    unique_edges_tree.insert((dst, src), None);
                }
            }
        }

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

    fn holdout(
        &self,
        seed: NodeT,
        train_percentage: f64,
        include_all_edge_types: bool,
        user_condition: impl Fn(NodeT, NodeT, Option<EdgeTypeT>) -> bool,
    ) -> Result<(Graph, Graph), String> {
        if train_percentage <= 0.0 || train_percentage >= 1.0 {
            return Err(String::from(
                "Train percentage must be strictly between 0 and 1.",
            ));
        }

        let valid_edges_number =
            (self.get_edges_number() as f64 * (1.0 - train_percentage)) as usize;

        // generate and shuffle the indices of the edges
        let mut rng = SmallRng::seed_from_u64((seed ^ SEED_XOR) as u64);
        let mut edge_indices: Vec<NodeT> = (0..self.get_edges_number()).collect();
        edge_indices.shuffle(&mut rng);

        let mut train: GraphDictionary = GraphDictionary::new();
        let mut valid: GraphDictionary = GraphDictionary::new();

        for edge in edge_indices.iter() {
            let src = self.sources[*edge];
            let dst = self.destinations[*edge];
            let edge_type = if let Some(et) = &self.edge_types {
                Some(et.ids[*edge])
            } else {
                None
            };
            let mut metadata =
                ConstructorEdgeMetadata::new(self.has_weights(), self.has_edge_types());
            if let Some(md) = &mut metadata {
                md.set(
                    // TODO TODO TODO
                    // THIS THING DOES NOT CONSIDER include_all_edge_types CURRENTLY!!!
                    // TODO TODO TODO
                    self.get_link_weights(src, dst),
                    self.get_link_edge_types(src, dst),
                );
            }
            // We stop adding edges when we have reached the minimum amount.
            if user_condition(src, dst, edge_type)
                && valid.len() < valid_edges_number
                && (self.is_directed || src <= dst)
            {
                valid.insert((src, dst), metadata.clone());
                // If the current edge is not a self loop and the graph
                // is not directed, we add the simmetrical graph
                if !self.is_directed && src != dst {
                    valid.insert((dst, src), metadata);
                }
            } else {
                // Otherwise we add the edges to the training set.
                //
                // When the graph is directed we need to check that the edge
                // in the opposite direction was not already inserted.
                train.insert((src, dst), metadata.clone());
                // If the current edge is not a self loop and the graph
                // is not directed, we add the simmetrical graph
                if !self.is_directed && src != dst {
                    train.insert((dst, src), metadata);
                }
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
    ///
    pub fn connected_holdout(
        &self,
        seed: NodeT,
        train_percentage: f64,
        include_all_edge_types: bool,
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
    ///
    pub fn random_holdout(
        &self,
        seed: NodeT,
        train_percentage: f64,
        include_all_edge_types: bool,
    ) -> Result<(Graph, Graph), String> {
        self.holdout(seed, train_percentage, include_all_edge_types, |_, _, _| {
            true
        })
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
    pub fn random_subgraph(&self, seed: usize, nodes_number: usize) -> Result<Graph, String> {
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
        for node in nodes {
            if self.is_node_trap(node) {
                continue;
            }
            stack.push(node);
            unique_nodes.insert(node);
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
}
