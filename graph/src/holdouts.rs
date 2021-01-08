use super::*;
use counter::Counter;
use indicatif::ParallelProgressIterator;
use indicatif::ProgressIterator;
use rand::rngs::SmallRng;
use rand::seq::SliceRandom;
use rand::SeedableRng;
use rayon::iter::IndexedParallelIterator;
use rayon::iter::IntoParallelIterator;
use rayon::iter::ParallelIterator;
use roaring::{RoaringBitmap, RoaringTreemap};
use std::collections::HashSet;
use std::iter::FromIterator;
use vec_rand::xorshift::xorshift as rand_u64;
use vec_rand::{gen_random_vec, sample_uniform};

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
    /// * `random_state`: EdgeT - random_state to use to reproduce negative edge set.
    /// * `negatives_number`: EdgeT - Number of negatives edges to include.
    /// * `seed_graph`: Option<Graph> - Optional graph to use to filter the negative edges. The negative edges generated when this variable is provided will always have a node within this graph.
    /// * `only_from_same_component`: bool - Wether to sample negative edges only from nodes that are from the same component.
    /// * `verbose`: bool - Wether to show the loading bar.
    ///
    pub fn sample_negatives(
        &self,
        mut random_state: EdgeT,
        negatives_number: EdgeT,
        seed_graph: Option<&Graph>,
        only_from_same_component: bool,
        verbose: bool,
    ) -> Result<Graph, String> {
        if negatives_number == 0 {
            return Err(String::from("The number of negatives cannot be zero."));
        }
        let seed_nodes: Option<RoaringBitmap> = if let Some(sg) = &seed_graph {
            if !self.overlaps(&sg)? {
                return Err(String::from(
                    "The given seed graph does not overlap with the current graph instance.",
                ));
            }
            Some(
                sg.get_nodes_names_iter()
                    .map(|(_, node_name, _)| self.get_unchecked_node_id(&node_name))
                    .collect::<RoaringBitmap>(),
            )
        } else {
            None
        };
        // In a complete directed graph allowing selfloops with N nodes there are N^2
        // edges. In a complete directed graph without selfloops there are N*(N-1) edges.
        // We can rewrite the first formula as (N*(N-1)) + N.
        //
        // In a complete undirected graph allowing selfloops with N nodes there are
        // (N*(N-1))/2 + N edges.

        // Here we use unique edges number because on a multigraph the negative
        // edges cannot have an edge type.
        let nodes_number = self.get_nodes_number() as EdgeT;

        // Wether to sample negative edges only from the same connected component.
        let (node_components, mut complete_edges_number) = if only_from_same_component {
            let node_components = self.get_node_components_vector(verbose);
            let complete_edges_number: EdgeT = Counter::init(node_components.clone())
                .into_iter()
                .map(|(_, nodes_number): (_, &usize)| {
                    (*nodes_number * (*nodes_number - 1)) as EdgeT
                })
                .sum();
            (Some(node_components), complete_edges_number)
        } else {
            (None, nodes_number * (nodes_number - 1))
        };

        // Here we compute the number of edges that a complete graph would have if it had the same number of nodes
        // of the current graph. Moreover, the complete graph will have selfloops IFF the current graph has at
        // least one of them.
        if self.has_selfloops() {
            complete_edges_number += nodes_number;
        }

        // Now we compute the maximum number of negative edges that we can actually generate
        let max_negative_edges = complete_edges_number - self.unique_edges_number;

        // We check that the number of requested negative edges is compatible with the
        // current graph instance.
        if negatives_number > max_negative_edges {
            return Err(format!(
                concat!(
                    "The requested negatives number {} is more than the ",
                    "number of negative edges that exist in the graph ({})."
                ),
                negatives_number, max_negative_edges
            ));
        }

        let pb1 = get_loading_bar(
            verbose,
            "Computing negative edges",
            negatives_number as usize,
        );

        // xorshift breaks if the random_state is zero
        // so we initialize xor it with a constat
        // to mitigate this problem
        random_state ^= SEED_XOR as EdgeT;

        let mut negative_edges_hashset = HashSet::new();
        let mut last_length = 0;
        let mut sampling_round: usize = 0;

        // randomly extract negative edges until we have the choosen number
        while negative_edges_hashset.len() < negatives_number as usize {
            // generate two random_states for reproducibility porpouses
            let src_random_state = rand_u64(random_state);
            let dst_random_state = rand_u64(src_random_state);
            random_state = rand_u64(dst_random_state);

            let tmp_tb = get_loading_bar(
                verbose,
                format!("Negatives sampling round {}", sampling_round).as_ref(),
                negatives_number as usize,
            );
            sampling_round += 1;

            // generate the random edge-sources
            let sampled_edge_ids = gen_random_vec(negatives_number as usize, src_random_state)
                .into_par_iter()
                .zip(gen_random_vec(negatives_number as usize, dst_random_state).into_par_iter())
                // convert them to plain (src, dst)
                .progress_with(tmp_tb)
                .filter_map(|(src_seed, dst_seed)| {
                    let src = sample_uniform(nodes_number as u64, src_seed as u64) as NodeT;
                    let dst = sample_uniform(nodes_number as u64, dst_seed as u64) as NodeT;
                    if let Some(sn) = &seed_nodes {
                        if !sn.contains(src) && !sn.contains(dst) {
                            return None;
                        }
                    }
                    if let Some(ncs) = &node_components {
                        if ncs[src as usize] != ncs[dst as usize] {
                            return None;
                        }
                    }
                    // If the edge is not a self-loop or the user allows self-loops and
                    // the graph is directed or the edges are inserted in a way to avoid
                    // inserting bidirectional edges.
                    match (self.has_selfloops() || src != dst) && !self.has_edge(src, dst, None) {
                        true => Some((src, dst)),
                        false => None,
                    }
                })
                .flat_map(|(src, dst)| {
                    if !self.is_directed() && src != dst {
                        vec![self.encode_edge(src, dst), self.encode_edge(dst, src)]
                    } else {
                        vec![self.encode_edge(src, dst)]
                    }
                })
                .collect::<Vec<EdgeT>>();

            let pb3 = get_loading_bar(
                verbose,
                "Inserting negative graph edges",
                negatives_number as usize,
            );

            for edge_id in sampled_edge_ids.iter().progress_with(pb3) {
                if negative_edges_hashset.len() >= negatives_number as usize {
                    break;
                }
                negative_edges_hashset.insert(*edge_id);
            }

            pb1.inc((negative_edges_hashset.len() - last_length as usize) as u64);
            last_length = negative_edges_hashset.len();
        }

        pb1.finish();

        Graph::from_integer_unsorted(
            negative_edges_hashset
                .iter()
                .cloned()
                .map(|edge| {
                    let (src, dst) = self.decode_edge(edge);
                    Ok((src, dst, None, None))
                }),
            self.nodes.clone(),
            self.node_types.clone(),
            None,
            self.directed,
            true,
            format!("{} negatives", self.name.clone()),
            false,
            self.has_edge_types(),
            self.has_weights(),
            verbose
        )
    }

    /// Compute the training and validation edges number from the training rate
    fn get_holdouts_edges_number(
        &self,
        train_size: f64,
        include_all_edge_types: bool,
    ) -> Result<(EdgeT, EdgeT), String> {
        if train_size <= 0.0 || train_size >= 1.0 {
            return Err(String::from("Train rate must be strictly between 0 and 1."));
        }
        if self.directed && self.get_edges_number() == 1
            || !self.directed && self.get_edges_number() == 2
        {
            return Err(String::from(
                "The current graph instance has only one edge. You cannot build an holdout with one edge.",
            ));
        }
        let total_edges_number = if include_all_edge_types {
            self.unique_edges_number
        } else {
            self.get_edges_number()
        };
        let train_edges_number = (total_edges_number as f64 * train_size) as EdgeT;
        let valid_edges_number = total_edges_number - train_edges_number;

        if train_edges_number == 0 || train_edges_number >= total_edges_number {
            return Err(String::from(
                "The training set has 0 edges! Change the training rate.",
            ));
        }
        if valid_edges_number == 0 {
            return Err(String::from(
                "The validation set has 0 edges! Change the training rate.",
            ));
        }

        Ok((train_edges_number, valid_edges_number))
    }

    fn holdout(
        &self,
        random_state: EdgeT,
        valid_edges_number: EdgeT,
        include_all_edge_types: bool,
        user_condition: impl Fn(EdgeT, NodeT, NodeT, Option<EdgeTypeT>) -> bool,
        verbose: bool,
    ) -> Result<(Graph, Graph), String> {
        let pb1 = get_loading_bar(
            verbose,
            "Picking validation edges",
            valid_edges_number as usize,
        );

        // generate and shuffle the indices of the edges
        let mut rng = SmallRng::seed_from_u64(random_state ^ SEED_XOR as EdgeT);
        let mut edge_indices: Vec<EdgeT> = (0..self.get_edges_number()).collect();
        edge_indices.shuffle(&mut rng);

        let mut valid_edges_bitmap = RoaringTreemap::new();
        let mut last_length = 0;

        for (edge_id, (src, dst, edge_type)) in edge_indices
            .iter()
            .cloned()
            .map(|edge_id| (edge_id, self.get_edge_triple(edge_id)))
        {
            // If the graph is undirected and we have extracted an edge that is a
            // simmetric one, we can skip this iteration.
            if !self.directed && src > dst {
                continue;
            }

            // We stop adding edges when we have reached the minimum amount.
            if user_condition(edge_id, src, dst, edge_type) {
                // Compute the forward edge ids that are required.
                valid_edges_bitmap.extend(self.compute_edge_ids_vector(
                    edge_id,
                    src,
                    dst,
                    include_all_edge_types,
                ));

                // If the graph is undirected
                if !self.directed {
                    // we compute also the backward edge ids that are required.
                    valid_edges_bitmap.extend(self.compute_edge_ids_vector(
                        self.get_unchecked_edge_id(dst, src, edge_type),
                        dst,
                        src,
                        include_all_edge_types,
                    ));
                }
                pb1.inc(valid_edges_bitmap.len() - last_length);
                last_length = valid_edges_bitmap.len();
            }

            // We stop the iteration when we found all the edges.
            if valid_edges_bitmap.len() >= valid_edges_number {
                break;
            }
        }

        if valid_edges_bitmap.len() < valid_edges_number {
            let actual_valid_edges_number = valid_edges_bitmap.len();
            return Err(format!(
                concat!(
                    "With the given configuration for the holdout, it is not possible to ",
                    "generate a validation set composed of {valid_edges_number} edges from the current graph.\n",
                    "The validation set can be composed of at most {actual_valid_edges_number} edges.\n"
                ),
                valid_edges_number=valid_edges_number,
                actual_valid_edges_number=actual_valid_edges_number,
            ));
        }

        // Creating the loading bar for the building of both the training and validation.
        let pb_valid = get_loading_bar(
            verbose,
            "Building the valid partition",
            valid_edges_bitmap.len() as usize,
        );
        let pb_train = get_loading_bar(
            verbose,
            "Building the train partition",
            (self.get_edges_number() - valid_edges_bitmap.len()) as usize,
        );

        Ok((
            Graph::build_graph(
                (0..self.get_edges_number())
                    .filter(|edge_id| !valid_edges_bitmap.contains(*edge_id))
                    .progress_with(pb_train)
                    .map(|edge_id| Ok(self.get_edge_quadruple(edge_id))),
                self.get_edges_number() - valid_edges_bitmap.len() as EdgeT,
                self.nodes.clone(),
                self.node_types.clone(),
                match &self.edge_types {
                    Some(ets) => Some(ets.vocabulary.clone()),
                    None => None,
                },
                self.directed,
                format!("{} training", self.name.clone()),
                false,
                self.has_edge_types(),
                self.has_weights(),
            )?,
            Graph::build_graph(
                valid_edges_bitmap
                    .iter()
                    .progress_with(pb_valid)
                    .map(|edge_id| Ok(self.get_edge_quadruple(edge_id))),
                valid_edges_bitmap.len() as EdgeT,
                self.nodes.clone(),
                self.node_types.clone(),
                match &self.edge_types {
                    Some(ets) => Some(ets.vocabulary.clone()),
                    None => None,
                },
                self.directed,
                format!("{} testing", self.name.clone()),
                false,
                self.has_edge_types(),
                self.has_weights(),
            )?,
        ))
    }

    /// Returns holdout for training ML algorithms on the graph structure.
    ///
    /// The holdouts returned are a tuple of graphs. The first one, which
    /// is the training graph, is garanteed to have the same number of
    /// graph components as the initial graph. The second graph is the graph
    /// meant for testing or validation of the algorithm, and has no garantee
    /// to be connected. It will have at most (1-train_size) edges,
    /// as the bound of connectivity which is required for the training graph
    /// may lead to more edges being left into the training partition.
    ///
    /// In the option where a list of edge types has been provided, these
    /// edge types will be those put into the validation set.
    ///
    /// # Arguments
    ///
    /// * `random_state`: NodeT - The random_state to use for the holdout,
    /// * `train_size`: f64 - Rate target to reserve for training.
    /// * `edge_types`: Option<Vec<String>> - Edge types to be selected for in the validation set.
    /// * `include_all_edge_types`: bool - Wethever to include all the edges between two nodes.
    /// * `verbose`: bool - Wethever to show the loading bar.
    ///
    ///
    pub fn connected_holdout(
        &self,
        random_state: EdgeT,
        train_size: f64,
        edge_types: Option<Vec<String>>,
        include_all_edge_types: bool,
        verbose: bool,
    ) -> Result<(Graph, Graph), String> {
        if train_size <= 0.0 || train_size >= 1.0 {
            return Err(String::from("Train rate must be strictly between 0 and 1."));
        }

        let edge_type_ids = if let Some(ets) = edge_types {
            Some(
                self.translate_edge_types(ets)?
                    .into_iter()
                    .collect::<HashSet<EdgeTypeT>>(),
            )
        } else {
            None
        };

        let tree = self
            .random_spanning_arborescence_kruskal(random_state, &edge_type_ids, verbose)
            .0;

        let edge_factor = if self.is_directed() { 1 } else { 2 };
        let train_edges_number = (self.get_edges_number() as f64 * train_size) as usize;
        let mut valid_edges_number = (self.get_edges_number() as f64 * (1.0 - train_size)) as EdgeT;

        if let Some(etis) = &edge_type_ids {
            let selected_edges_number: EdgeT = etis
                .iter()
                .map(|et| self.get_unchecked_edge_count_by_edge_type(*et) as EdgeT)
                .sum();
            valid_edges_number = (selected_edges_number as f64 * (1.0 - train_size)) as EdgeT;
        }

        if tree.len() * edge_factor > train_edges_number {
            return Err(format!(
                concat!(
                    "The given spanning tree of the graph contains {} edges ",
                    "that is more than the required training edges number {}.\n",
                    "This makes impossible to create a validation set using ",
                    "{} edges.\nIf possible, you should increase the ",
                    "train_size parameter which is currently equal to ",
                    "{}.\nThe deny map, by itself, is requiring at least ",
                    "a train rate of {}."
                ),
                tree.len() * edge_factor,
                train_edges_number,
                valid_edges_number,
                train_size,
                (tree.len() * edge_factor) as f64 / train_edges_number as f64
            ));
        }

        self.holdout(
            random_state,
            valid_edges_number,
            include_all_edge_types,
            |_, src, dst, edge_type| {
                let is_in_tree = tree.contains(&(src, dst));
                let singleton_self_loop = src == dst && self.get_node_degree(src) == 1;
                let correct_edge_type = match &edge_type_ids {
                    Some(etis) => etis.contains(&edge_type.unwrap()),
                    None => true,
                };
                // The tree must not contain the provided edge ID
                // And this is not a self-loop edge with degree 1
                // And the edge type of the edge ID is within the provided edge type
                !is_in_tree && !singleton_self_loop && correct_edge_type
            },
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
    /// * `random_state`: NodeT - The random_state to use for the holdout,
    /// * `train_size`: f64 - rate target to reserve for training
    /// * `include_all_edge_types`: bool - Wethever to include all the edges between two nodes.
    /// * `edge_types`: Option<Vec<String>> - The edges to include in validation set.
    /// * `min_number_overlaps`: Option<usize> - The minimum number of overlaps to include the edge into the validation set.
    /// * `verbose`: bool - Wethever to show the loading bar.
    ///
    pub fn random_holdout(
        &self,
        random_state: EdgeT,
        train_size: f64,
        include_all_edge_types: bool,
        edge_types: Option<Vec<String>>,
        min_number_overlaps: Option<EdgeT>,
        verbose: bool,
    ) -> Result<(Graph, Graph), String> {
        let (_, valid_edges_number) =
            self.get_holdouts_edges_number(train_size, include_all_edge_types)?;
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
            random_state,
            valid_edges_number,
            include_all_edge_types,
            |_, src, dst, edge_type| {
                // If a list of edge types was provided and the edge type
                // of the current edge is not within the provided list,
                // we skip the current edge.
                if let Some(etis) = &edge_type_ids {
                    if !etis.contains(&edge_type.unwrap()) {
                        return false;
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
    /// sampled using given random_state and includes all neighbouring nodes until
    /// the required number of nodes is reached. All the edges connecting any
    /// of the selected nodes are then inserted into this graph.
    ///
    ///
    ///
    /// # Arguments
    ///
    /// * `random_state`: usize - Random random_state to use.
    /// * `nodes_number`: usize - Number of nodes to extract.
    /// * `verbose`: bool - Wethever to show the loading bar.
    ///
    pub fn random_subgraph(
        &self,
        random_state: usize,
        nodes_number: NodeT,
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

        // Creating the loading bars
        let pb1 = get_loading_bar(verbose, "Sampling nodes subset", nodes_number as usize);
        let pb2 = get_loading_bar(verbose, "Computing subgraph edges", nodes_number as usize);
        let pb3 = get_loading_bar(
            verbose,
            "Building subgraph",
            self.get_edges_number() as usize,
        );

        // Creating the random number generator
        let mut rnd = SmallRng::seed_from_u64((random_state ^ SEED_XOR) as u64);

        // Nodes indices
        let mut nodes: Vec<NodeT> = (0..self.get_nodes_number()).collect();

        // Shuffling the components using the given random_state.
        nodes.shuffle(&mut rnd);

        // Initializing stack and set of nodes
        let mut unique_nodes = RoaringBitmap::new();
        let mut stack: Vec<NodeT> = Vec::new();

        // We iterate on the components
        'outer: for node in nodes.iter() {
            // If the current node is a trap there is no need to continue with the current loop.
            if self.is_node_trap(*node) {
                continue;
            }
            stack.push(*node);
            while !stack.is_empty() {
                let src = stack.pop().unwrap();
                for dst in self.get_source_destinations_range(src) {
                    if !unique_nodes.contains(dst) && src != dst {
                        stack.push(dst);
                    }

                    unique_nodes.insert(*node);
                    unique_nodes.insert(dst);
                    pb1.inc(2);

                    // If we reach the desired number of unique nodes we can stop the iteration.
                    if unique_nodes.len() as NodeT >= nodes_number {
                        break 'outer;
                    }
                }
            }
        }

        pb1.finish();

        let edges_bitmap =
            RoaringTreemap::from_iter(unique_nodes.iter().progress_with(pb2).flat_map(|src| {
                let (min_edge_id, max_edge_id) = self.get_destinations_min_max_edge_ids(src);
                (min_edge_id..max_edge_id)
                    .filter(|edge_id| unique_nodes.contains(self.get_destination(*edge_id)))
                    .collect::<Vec<EdgeT>>()
            }));

        Graph::build_graph(
            edges_bitmap
                .iter()
                .progress_with(pb3)
                .map(|edge_id| Ok(self.get_edge_quadruple(edge_id))),
            edges_bitmap.len() as EdgeT,
            self.nodes.clone(),
            self.node_types.clone(),
            match &self.edge_types {
                Some(ets) => Some(ets.vocabulary.clone()),
                None => None,
            },
            self.directed,
            format!("{} subgraph", self.name.clone()),
            false,
            self.has_edge_types(),
            self.has_weights(),
        )
    }

    /// Returns train and test graph following kfold validation scheme.
    ///
    /// The edges are splitted into k chunks. The k_index-th chunk is used to build
    /// the validation graph, all the other edges create the training graph.
    ///
    /// # Arguments
    ///
    /// * `edge_types`: Option<Vec<String>> - Edge types to be selected when computing the folds
    ///         (All the edge types not listed here will be always be used in the training set).
    /// * `k`: u64 - The number of folds.
    /// * `k_index`: u64 - Which fold to use for the validation.
    /// * `random_state`: NodeT - The random_state (seed) to use for the holdout,
    /// * `verbose`: bool - Wethever to show the loading bar.
    ///
    pub fn kfold(
        &self,
        k: EdgeT,
        k_index: u64,
        edge_types: Option<Vec<String>>,
        random_state: EdgeT,
        verbose: bool,
    ) -> Result<(Graph, Graph), String> {
        if k == 1 {
            return Err(String::from("Cannot do a k-fold with only one fold."));
        }
        if k_index >= k {
            return Err(String::from(
                "The index of the k-fold must be strictly less than the number of folds.",
            ));
        }

        // If edge types is not None, to compute the chunks only use the edges
        // of the chosen edge_types
        let mut indices = if let Some(ets) = edge_types {
            if ets.is_empty() {
                return Err(String::from(
                    "Required edge types must be a non-empty list.",
                ));
            }
            if !self.has_edge_types() {
                return Err(String::from(
                    "Edge types-based k-fold requested but the edge types are not available in this graph."
                ));
            }

            let edge_type_ids: HashSet<EdgeTypeT> = self
                .translate_edge_types(ets)?
                .iter()
                .cloned()
                .collect::<HashSet<EdgeTypeT>>();

            self.get_edges_triples(self.directed)
                .filter_map(|(edge_id, _, _, edge_type)| {
                    if !edge_type_ids.contains(&edge_type.unwrap()) {
                        return None;
                    }
                    Some(edge_id)
                })
                .collect::<Vec<EdgeT>>()
        } else {
            self.get_edges_iter(self.directed)
                .map(|(edge_id, _, _)| edge_id)
                .collect::<Vec<EdgeT>>()
        };

        if k >= indices.len() as EdgeT {
            return Err(String::from(
                "Cannot do a number of k-fold greater than the number of available edges.",
            ));
        }

        // shuffle the indices
        let mut rng = SmallRng::seed_from_u64(random_state ^ SEED_XOR as EdgeT);
        indices.shuffle(&mut rng);
        // Get the k_index-th chunk
        let chunk_size = indices.len() as f64 / k as f64;
        let start = (k_index as f64 * chunk_size).ceil() as EdgeT;
        let end = min!(
            indices.len() as EdgeT,
            (((k_index + 1) as f64) * chunk_size).ceil() as EdgeT
        );
        let chunk =
            RoaringTreemap::from_iter(indices[start as usize..end as usize].iter().cloned());
        // Create the two graphs
        self.holdout(
            random_state,
            end - start,
            false,
            |edge_id, _, _, _| chunk.contains(edge_id),
            verbose,
        )
    }
}
