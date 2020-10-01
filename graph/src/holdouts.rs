use super::*;
use bitvec::prelude::*;
use indicatif::{ProgressBar, ProgressIterator, ProgressStyle};
use rand::rngs::SmallRng;
use rand::seq::SliceRandom;
use rand::SeedableRng;
use std::collections::HashSet;
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
        // In a complete directed graph allowing selfloops with N nodes there are N^2
        // edges. In a complete directed graph without selfloops there are N*(N-1) edges.
        // We can rewrite the first formula as (N*(N-1)) + N.
        //
        // In a complete undirected graph allowing selfloops with N nodes there are
        // (N*(N-1))/2 + N edges.

        // Here we use unique edges number because on a multigraph the negative
        // edges cannot have an edge type.
        let nodes_number = self.get_nodes_number();
        let selfloops_in_graph = self.get_selfloops_number();
        let self_loops_number = if allow_selfloops {
            nodes_number
        } else {
            selfloops_in_graph
        };
        let total_negative_edges = if self.directed {
            nodes_number * (nodes_number - 1) + self_loops_number - self.get_unique_edges_number()
        } else {
            nodes_number * (nodes_number - 1) / 2 + self_loops_number
                - (self.get_unique_edges_number() - selfloops_in_graph) / 2
                - selfloops_in_graph
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

        let pb = get_loading_bar(verbose, "Computing negative edges", negatives_number as u64);

        // xorshift breaks if the seed is zero
        // so we initialize xor it with a constat
        // to mitigate this problem
        seed ^= SEED_XOR;

        // initialize the vectors for the result
        let mut unique_edges_tree = GraphDictionary::new();

        // randomly extract negative edges until we have the choosen number
        while unique_edges_tree.len() < negatives_number {
            seed = xorshift(seed as u64) as usize;
            let src: NodeT = seed % nodes_number;
            seed = xorshift(seed as u64) as usize;
            let dst: NodeT = seed % nodes_number;
            // If the edge is not a self-loop or the user allows self-loops and
            // the graph is directed or the edges are inserted in a way to avoid
            // inserting bidirectional edges, avoiding to execute the check
            // of edge types so to insert them twice if the edge types are
            // different.
            if (allow_selfloops || src != dst)
                && !self.has_edge(src, dst)
                && !unique_edges_tree.contains_key(&(src, dst))
            {
                unique_edges_tree.extend(self, src, dst, None, None, false);
                pb.inc(1 + !self.directed as u64);
            }
        }
        pb.finish();
        Ok(build_graph(
            &mut unique_edges_tree,
            self.nodes.clone(),
            self.node_types.clone(),
            None,
            self.directed,
        ))
    }

    fn holdout(
        &self,
        seed: NodeT,
        verbose: bool,
        train_edges_number: EdgeT,
        valid_edges_number: EdgeT,
        mask: BitVec,
    ) -> Result<(Graph, Graph), String> {
        let pb = get_loading_bar(
            verbose,
            "Generating holdout",
            self.get_edges_number() as u64,
        );

        Ok((
            build_graph(
                self.get_edge_quadruples()
                    .zip(mask)
                    .filter_map(|(quadruple, keep)| match keep {
                        true => None,
                        false => Some(Ok(quadruple)),
                    }),
                train_edges_number,
                self.nodes.len(),
                self.edge_types.map(|v| v.vocabulary.clone()),
                self.node_types.clone(),
                self.directed,
                self.nodes.clone(),
            )?,
            build_graph(
                self.get_edge_quadruples()
                    .zip(mask)
                    .filter_map(|(quadruple, keep)| match keep {
                        true => Some(Ok(quadruple)),
                        false => None,
                    }),
                valid_edges_number,
                self.nodes.len(),
                self.edge_types.map(|v| v.vocabulary.clone()),
                self.node_types.clone(),
                self.directed,
                self.nodes.clone(),
            )?,
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
        let tree = self.spanning_tree(seed, include_all_edge_types, verbose);

        let edge_factor = if self.directed { 1 } else { 2 };
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

        let (train, test) = self.holdout(
            seed,
            train_percentage,
            include_all_edge_types,
            |src, dst, edge_type| !tree.contains(&(src, dst, edge_type)),
            verbose,
        )?;
        Ok((train, test))
    }

    /// Returns random holdout for training ML algorithms on the graph edges.
    ///
    /// The holdouts returned are a tuple of graphs. In neither holdouts the
    /// graph connectivity is necessarily preserved. To maintain that, use
    /// the method `connected_holdout`.
    ///
    /// # Arguments
    ///
    /// * `seed`: NodeT - The seed to use for the holdout,
    /// * `train_percentage`: f64 - Percentage target to reserve for training
    /// * `include_all_edge_types`: bool - Wethever to include all the edges between two nodes.
    /// * `edge_types`: Option<Vec<String>> - The edges to include in validation set.
    /// * `min_number_overlaps`: Option<usize> - The minimum number of overlaps to include the edge into the validation set.
    /// * `verbose`: bool - Wethever to show the loading bar.
    ///
    pub fn random_holdout(
        &self,
        seed: NodeT,
        train_percentage: f64,
        include_all_edge_types: bool,
        edge_types: Option<Vec<String>>,
        min_number_overlaps: Option<usize>,
        verbose: bool,
    ) -> Result<(Graph, Graph), String> {
        let edge_type_ids = if let Some(ets) = edge_types {
            Some(
                self.translate_edge_types(ets)?
                    .into_iter()
                    .collect::<HashSet<EdgeTypeT>>(),
            )
        } else {
            None
        };
        if min_number_overlaps.is_some() && !self.is_multigraph() {
            return Err("Current graph is not a multigraph!".to_string());
        }
        self.holdout(
            seed,
            train_percentage,
            include_all_edge_types,
            |src, dst, edge_type| {
                // If a list of edge types was provided and the edge type
                // of the current edge is not within the provided list,
                // we skip the current edge.
                if let Some(etis) = &edge_type_ids {
                    if let Some(et) = &edge_type {
                        if !etis.contains(et) {
                            return false;
                        }
                    }
                }
                // If a minimum number of overlaps was provided and the current
                // edge has not the required minimum amount of overlaps.
                if let Some(mno) = min_number_overlaps {
                    if self.get_unchecked_edge_types_number_from_tuple(src, dst) < mno {
                        return false;
                    }
                }
                // Otherwise we accept the provided edge for the validation set
                true
            },
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
    ///
    ///
    /// # Arguments
    ///
    /// * `seed`: usize - Random seed to use.
    /// * `nodes_number`: usize - Number of nodes to extract.
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
        let not_singleton_nodes_number = self.get_not_singleton_nodes_number();
        if nodes_number > not_singleton_nodes_number {
            return Err(format!(
                concat!(
                    "Required number of nodes ({}) is more than available ",
                    "number of nodes ({}) that have edges in current graph."
                ),
                nodes_number, not_singleton_nodes_number
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
        'outer: for node in nodes.iter().progress_with(pb) {
            // If the current node is a trap there is no need to continue with the current loop.
            if self.is_node_trap(*node) {
                continue;
            }
            stack.push(*node);
            while !stack.is_empty() {
                let src = stack.pop().unwrap();
                for edge_id in self.get_unchecked_destinations_range(src) {
                    let dst: NodeT = self.get_destination(edge_id);
                    if !unique_nodes.contains(&dst) && src != dst {
                        stack.push(dst);
                    }

                    let edge_type = if let Some(et) = &self.edge_types {
                        Some(et.ids[edge_id])
                    } else {
                        None
                    };

                    let weight = if let Some(w) = &self.weights {
                        Some(w[edge_id])
                    } else {
                        None
                    };

                    unique_nodes.insert(*node);
                    unique_nodes.insert(dst);

                    graph_data.extend(&self, src, dst, edge_type, weight, true);
                    // If we reach the desired number of unique nodes we can stop the iteration.
                    if unique_nodes.len() >= nodes_number {
                        break 'outer;
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
            self.directed,
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

        let edge_type_ids = self.translate_edge_types(edge_types)?;

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

        self.get_edge_quadruples_enumerate()
            .progress_with(pb)
            .filter(|(_, _, _, edge_type, _)| edge_type_ids.contains(&edge_type.unwrap()))
            .for_each(|(_, src, dst, edge_type, weight)| {
                graph_data.extend(&self, src, dst, edge_type, weight, false)
            });

        Ok(build_graph(
            &mut graph_data,
            self.nodes.clone(),
            self.node_types.clone(),
            if let Some(et) = &self.edge_types {
                Some(et.vocabulary.clone())
            } else {
                None
            },
            self.directed,
        ))
    }
}
