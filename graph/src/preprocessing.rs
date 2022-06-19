use super::*;
use atomic_float::AtomicF64;
use bitvec::prelude::*;
use hashbrown::HashMap;
use indicatif::{ParallelProgressIterator, ProgressIterator};
use itertools::Itertools;
use num_traits::Pow;
use rayon::prelude::*;
use std::collections::VecDeque;
use vec_rand::splitmix64;

#[manual_binding]
/// Return training batches for Word2Vec models.
///
/// The batch is composed of a tuple as the following:
///
/// - (Contexts indices, central nodes indices): the tuple of nodes
///
/// This does not provide any output value as the model uses NCE loss
/// and basically the central nodes that are fed as inputs work as the
/// outputs value.
///
/// # Arguments
///
/// * `sequences`: impl ParallelIterator<Item = Vec<NodeT>> + 'a - the sequence of sequences of integers to preprocess.
/// * `window_size`: usize - Window size to consider for the sequences.
///
pub fn word2vec<'a>(
    sequences: impl ParallelIterator<Item = Vec<NodeT>> + 'a,
    window_size: usize,
) -> impl ParallelIterator<Item = (Vec<NodeT>, NodeT)> + 'a {
    sequences.flat_map_iter(move |sequence| {
        let sequence_length = sequence.len();
        if sequence_length < window_size * 2 + 1 {
            panic!(
                "
            Cannot compute word2vec, got a sequence of length {} and window size {}.
            for the current window_size the minimum sequence length required is {}",
                sequence_length,
                window_size,
                window_size * 2 + 1,
            );
        }
        (window_size..(sequence_length - window_size)).map(move |i| {
            (
                (i - window_size..i)
                    .chain(i + 1..window_size + i + 1)
                    .map(|j| sequence[j])
                    .collect(),
                sequence[i],
            )
        })
    })
}

/// # Preprocessing for ML algorithms on graph.
impl Graph {
    #[manual_binding]
    /// Return training batches for Node2Vec models.
    ///
    /// The batch is composed of a tuple as the following:
    ///
    /// - (Contexts indices, central nodes indices): the tuple of nodes
    ///
    /// This does not provide any output value as the model uses NCE loss
    /// and basically the central nodes that are fed as inputs work as the
    /// outputs value.
    ///
    /// # Arguments
    ///
    /// * `walk_parameters`: &'a WalksParameters - the weighted walks parameters.
    /// * `quantity`: NodeT - Number of nodes to consider.
    /// * `window_size`: usize - Window size to consider for the sequences.
    ///
    /// # Raises
    /// * If the graph does not contain edges.
    /// * If the graph is directed.
    /// * If the given walks parameters are not compatible with the current graph instance.
    pub fn node2vec<'a>(
        &'a self,
        walk_parameters: &'a WalksParameters,
        quantity: NodeT,
        window_size: usize,
    ) -> Result<impl ParallelIterator<Item = (Vec<NodeT>, NodeT)> + 'a> {
        Ok(word2vec(
            self.par_iter_random_walks(quantity, walk_parameters)?,
            window_size,
        ))
    }

    #[manual_binding]
    /// Return triple with CSR representation of cooccurrence matrix.
    ///
    /// The first vector has the sources, the second vector the destinations
    /// and the third one contains the min-max normalized frequencies.
    ///
    /// # Arguments
    /// * `walks_parameters`: &'a WalksParameters - the walks parameters.
    /// * `window_size`: usize - Window size to consider for the sequences.
    pub fn par_iter_cooccurence_matrix<'a>(
        &'a self,
        walks_parameters: &'a WalksParameters,
        window_size: usize,
    ) -> Result<impl ParallelIterator<Item = (NodeT, NodeT, f32)> + 'a> {
        Ok(self
            .par_iter_complete_walks(walks_parameters)?
            .flat_map(move |sequence| {
                let mut cooccurence_matrix: HashMap<(NodeT, NodeT), f32> = HashMap::new();
                let mut total = 0.0;
                (0..sequence.len())
                    .map(|position| {
                        (
                            sequence[position],
                            &sequence[(position.saturating_sub(window_size)
                                ..(position + window_size).min(sequence.len()))],
                        )
                    })
                    .for_each(|(central_id, context)| {
                        context.iter().copied().for_each(|context_id| {
                            // Get the current value for this pair of nodes
                            cooccurence_matrix
                                .entry((central_id, context_id))
                                .and_modify(|e| *e += 1.0)
                                .or_insert(1.0);
                            total += 1.0;
                        });
                    });
                cooccurence_matrix
                    .into_par_iter()
                    .map(move |((central_id, context_id), freq)| {
                        (central_id, context_id, freq / total)
                    })
            }))
    }

    #[manual_binding]
    /// Return iterator over neighbours of random nodes, optionally including the central node IDs, and its node type.
    ///
    /// # Arguments
    /// * `ids`: u64 -  The index of the batch to generate, behaves like a random random_state.
    /// * `batch_size`: Option<NodeT> - Number of nodes to sample.
    /// * `include_central_node`: Option<bool> - Whether to include the node ID in the returned iterator.
    /// * `return_edge_weights`: Option<bool> - Whether to include the edge weights.
    /// * `max_neighbours`: Option<NodeT> - Number of maximum neighbours to consider.
    ///
    /// # Raises
    /// * If the graph does not contain node type IDs.
    /// * If the graph does not contain known node type IDs.
    /// * If the graph does not contain unknown node type IDs.
    /// * If the edge weights have been requested but the graph does not have any.
    /// * If the central node ID is to be included and the edge weights are requested as well.
    ///
    /// TODO!: Add balanced mini-batch option.
    /// TODO!: Add option to return other edge metrics than weights, like Jaccard.
    /// TODO!: Add option to generate random neighborhoods.
    /// TODO!: Add option to sample neighbours with distance k.
    /// TODO!: Add option to return stuff like the path length to random nodes.
    pub fn get_node_label_prediction_mini_batch(
        &self,
        idx: u64,
        batch_size: Option<NodeT>,
        include_central_node: Option<bool>,
        return_edge_weights: Option<bool>,
        max_neighbours: Option<NodeT>,
    ) -> Result<
        impl IndexedParallelIterator<Item = ((Vec<NodeT>, Option<Vec<WeightT>>), &Vec<NodeTypeT>)> + '_,
    > {
        if let Some(return_edge_weights) = return_edge_weights {
            if return_edge_weights {
                self.must_have_edge_weights()?;
            }
        }
        self.must_have_known_node_types()?;
        self.must_have_unknown_node_types()?;
        let nodes_number = self.get_nodes_number();
        let batch_size = batch_size.unwrap_or(1024).min(nodes_number);
        let return_edge_weights = return_edge_weights.unwrap_or(false);
        let include_central_node = include_central_node.unwrap_or(false);
        if return_edge_weights && include_central_node {
            return Err(concat!(
                "The edge weights have been ",
                "requested, but also the central node ID has been requested.\n",
                "It is not possible to return the central node ID with the ",
                "requested parameters, as we do not have its edge weight.\n",
                "If you want to include the central node ID still, consider ",
                "adding self-loops to the graph by using the method ",
                "`graph.add_selfloops()`."
            )
            .to_string());
        }
        let random_state = splitmix64(idx);
        Ok((0..batch_size).into_par_iter().map(move |i| unsafe {
            let mut random_state = splitmix64(random_state + i as u64);
            let (node_id, node_type_ids) = loop {
                random_state = splitmix64(random_state);
                let node_id = self.get_random_node(random_state);
                let node_type_ids = self.get_unchecked_node_type_ids_from_node_id(node_id);
                if node_type_ids.is_some() {
                    break (node_id, node_type_ids.unwrap());
                }
            };
            let (min_edge_id, max_edge_id, destinations, probabilistic_indices) = self
                .get_unchecked_edges_and_destinations_from_source_node_id(
                    max_neighbours,
                    random_state,
                    node_id,
                );
            let mut destinations = self
                .get_destinations_slice(min_edge_id, max_edge_id, &destinations)
                .to_owned();
            if include_central_node {
                destinations.push(node_id);
            }
            (
                (
                    destinations,
                    if return_edge_weights {
                        Some(self.get_edge_weighted_transitions(
                            min_edge_id,
                            max_edge_id,
                            &probabilistic_indices,
                        ))
                    } else {
                        None
                    },
                ),
                node_type_ids,
            )
        }))
    }

    unsafe fn get_unchecked_edge_prediction_node_type_ids(
        &self,
        node_id: NodeT,
    ) -> Option<Vec<NodeTypeT>> {
        Some(
            match (
                self.get_unchecked_node_type_ids_from_node_id(node_id),
                self.has_multilabel_node_types().unwrap(),
            ) {
                (None, false) => {
                    if self.has_unknown_node_types().unwrap() {
                        vec![0]
                    } else {
                        unreachable!(concat!(
                            "In a graph without unknown node types it is not possible ",
                            "to have `None` node types."
                        ));
                    }
                }
                (None, true) => {
                    if self.has_unknown_node_types().unwrap() {
                        vec![0; self.get_maximum_multilabel_count().unwrap() as usize]
                    } else {
                        unreachable!(concat!(
                            "In a graph without unknown node types it is not possible ",
                            "to have `None` node types."
                        ));
                    }
                }
                (Some(node_type_ids), false) => {
                    let mut node_type_ids = node_type_ids.clone();
                    if self.has_unknown_node_types().unwrap() {
                        node_type_ids.iter_mut().for_each(|node_type_id| {
                            *node_type_id += 1;
                        });
                        node_type_ids
                    } else {
                        node_type_ids
                    }
                }
                (Some(node_type_ids), true) => {
                    let mut padded_node_type_ids =
                        vec![0; self.get_maximum_multilabel_count().unwrap() as usize];
                    node_type_ids
                        .into_iter()
                        .zip(padded_node_type_ids.iter_mut())
                        .for_each(|(node_type, target)| {
                            // We need to add one because we need to reserve 0 for the mask.
                            *target = node_type + 1;
                        });
                    padded_node_type_ids
                }
            },
        )
    }

    #[manual_binding]
    /// Returns n-ple with source, destination and whether the edge is real.
    ///
    /// # Arguments
    /// * `random_state`: u64 - Random state of the batch to generate.
    /// * `batch_size`: usize - The maximal size of the batch to generate,
    /// * `sample_only_edges_with_heterogeneous_node_types`: bool - Whether to sample negative edges only with source and destination nodes that have different node types.
    /// * `negative_samples_rate`: Option<f64> - The component of netagetive samples to use.
    /// * `avoid_false_negatives`: Option<bool> - Whether to remove the false negatives when generated. It should be left to false, as it has very limited impact on the training, but enabling this will slow things down.
    /// * `maximal_sampling_attempts`: Option<usize> - Number of attempts to execute to sample the negative edges.
    /// * `use_zipfian_sampling`: Option<bool> - Whether to sample the nodes using zipfian distribution. By default True. Not using this may cause significant biases.
    /// * `support`: Option<&'a Graph> - Graph to use to compute the edge metrics. When not provided, the current graph (self) is used.
    /// * `graph_to_avoid`: &'a Option<&Graph> - The graph whose edges are to be avoided during the generation of false negatives,
    ///
    /// # Raises
    /// * If the given amount of negative samples is not a positive finite real value.
    /// * If node types are requested but the graph does not contain any.
    /// * If the `sample_only_edges_with_heterogeneous_node_types` argument is provided as true, but the graph does not have node types.
    ///
    pub fn iter_edge_prediction_mini_batch<'a>(
        &'a self,
        mut random_state: u64,
        batch_size: usize,
        sample_only_edges_with_heterogeneous_node_types: bool,
        negative_samples_rate: Option<f64>,
        avoid_false_negatives: Option<bool>,
        maximal_sampling_attempts: Option<usize>,
        use_zipfian_sampling: Option<bool>,
        support: Option<&'a Graph>,
        graph_to_avoid: Option<&'a Graph>,
    ) -> Result<impl Iterator<Item = (NodeT, NodeT, bool)> + 'a> {
        let support = support.unwrap_or(&self);
        let avoid_false_negatives = avoid_false_negatives.unwrap_or(false);
        let maximal_sampling_attempts = maximal_sampling_attempts.unwrap_or(10_000);
        let use_zipfian_sampling = use_zipfian_sampling.unwrap_or(true);

        if sample_only_edges_with_heterogeneous_node_types && !self.has_node_types() {
            return Err(concat!(
                "The parameter `sample_only_edges_with_heterogeneous_node_types` was provided with value `true` ",
                "but the current graph instance does not contain any node type. ",
                "If you expected to have node types within this graph, maybe you have either dropped them ",
                "with a wrong filter operation or use the wrong parametrization to load the graph."
            ).to_string());
        }

        if sample_only_edges_with_heterogeneous_node_types
            && self.has_exclusively_homogeneous_node_types().unwrap()
        {
            return Err(concat!(
                "The parameter `sample_only_edges_with_heterogeneous_node_types` was provided with value `true` ",
                "but the current graph instance has exclusively homogeneous node types, that is all the nodes have ",
                "the same node type. ",
                "If you expected to have heterogeneous node types within this graph, maybe you have either dropped them ",
                "with a wrong filter operation or use the wrong parametrization to load the graph."
            ).to_string());
        }

        let negative_samples_threshold = if let Some(negative_samples_rate) = &negative_samples_rate
        {
            if *negative_samples_rate < 0.0
                || *negative_samples_rate > 1.0
                || !negative_samples_rate.is_finite()
            {
                return Err(format!(
                    concat!(
                        "Negative sample must be a posive ",
                        "real value between 0 and 1. ",
                        "You have provided {}."
                    ),
                    *negative_samples_rate
                ));
            }
            (negative_samples_rate * u64::MAX as f64).ceil() as u64
        } else {
            0
        };

        random_state = splitmix64(random_state);

        Ok((0..batch_size).map(move |i| unsafe {
            let mut random_state = splitmix64(random_state + i as u64);
            if random_state > negative_samples_threshold {
                random_state = splitmix64(random_state);
                let (src, dst) =
                    self.get_unchecked_node_ids_from_edge_id(self.get_random_edge_id(random_state));
                return (src, dst, true);
            }

            for _ in 0..maximal_sampling_attempts {
                random_state = splitmix64(random_state);
                let (src, dst) = if use_zipfian_sampling {
                    (
                        self.get_random_zipfian_node(random_state),
                        self.get_random_zipfian_node(random_state.wrapping_mul(2)),
                    )
                } else {
                    (
                        self.get_random_node(random_state),
                        self.get_random_node(random_state.wrapping_mul(2)),
                    )
                };

                if avoid_false_negatives && support.has_edge_from_node_ids(src, dst)
                    || sample_only_edges_with_heterogeneous_node_types && {
                        self.get_unchecked_node_type_ids_from_node_id(src)
                            == self.get_unchecked_node_type_ids_from_node_id(dst)
                    }
                    || graph_to_avoid
                        .as_ref()
                        .map_or(false, |g| g.has_edge_from_node_ids(src, dst))
                {
                    continue;
                }

                return (src, dst, false);
            }

            panic!(
                concat!(
                    "Executed more than {} attempts to sample a negative edge.\n",
                    "If your graph is so small that you see this error, you may want to consider ",
                    "using one of the edge embedding transformer from the Embiggen library."
                ),
                maximal_sampling_attempts
            );
        }))
    }

    #[manual_binding]
    /// Returns n-ple with source, destination and whether the edge is real.
    ///
    /// # Arguments
    /// * `random_state`: u64 - Random state of the batch to generate.
    /// * `batch_size`: usize - The maximal size of the batch to generate,
    /// * `sample_only_edges_with_heterogeneous_node_types`: bool - Whether to sample negative edges only with source and destination nodes that have different node types.
    /// * `negative_samples_rate`: Option<f64> - The component of netagetive samples to use.
    /// * `avoid_false_negatives`: Option<bool> - Whether to remove the false negatives when generated. It should be left to false, as it has very limited impact on the training, but enabling this will slow things down.
    /// * `maximal_sampling_attempts`: Option<usize> - Number of attempts to execute to sample the negative edges.
    /// * `use_zipfian_sampling`: Option<bool> - Whether to sample the nodes using zipfian distribution. By default True. Not using this may cause significant biases.
    /// * `support`: Option<&'a Graph> - Graph to use to compute the edge metrics. When not provided, the current graph (self) is used.
    /// * `graph_to_avoid`: &'a Option<&Graph> - The graph whose edges are to be avoided during the generation of false negatives,
    ///
    /// # Raises
    /// * If the given amount of negative samples is not a positive finite real value.
    /// * If node types are requested but the graph does not contain any.
    /// * If the `sample_only_edges_with_heterogeneous_node_types` argument is provided as true, but the graph does not have node types.
    ///
    pub fn par_iter_edge_prediction_mini_batch<'a>(
        &'a self,
        mut random_state: u64,
        batch_size: usize,
        sample_only_edges_with_heterogeneous_node_types: bool,
        negative_samples_rate: Option<f64>,
        avoid_false_negatives: Option<bool>,
        maximal_sampling_attempts: Option<usize>,
        use_zipfian_sampling: Option<bool>,
        support: Option<&'a Graph>,
        graph_to_avoid: Option<&'a Graph>,
    ) -> Result<impl IndexedParallelIterator<Item = (NodeT, NodeT, bool)> + 'a> {
        let support = support.unwrap_or(&self);
        let avoid_false_negatives = avoid_false_negatives.unwrap_or(false);
        let maximal_sampling_attempts = maximal_sampling_attempts.unwrap_or(10_000);
        let use_zipfian_sampling = use_zipfian_sampling.unwrap_or(true);

        if sample_only_edges_with_heterogeneous_node_types && !self.has_node_types() {
            return Err(concat!(
                "The parameter `sample_only_edges_with_heterogeneous_node_types` was provided with value `true` ",
                "but the current graph instance does not contain any node type. ",
                "If you expected to have node types within this graph, maybe you have either dropped them ",
                "with a wrong filter operation or use the wrong parametrization to load the graph."
            ).to_string());
        }

        if sample_only_edges_with_heterogeneous_node_types
            && self.has_exclusively_homogeneous_node_types().unwrap()
        {
            return Err(concat!(
                "The parameter `sample_only_edges_with_heterogeneous_node_types` was provided with value `true` ",
                "but the current graph instance has exclusively homogeneous node types, that is all the nodes have ",
                "the same node type. ",
                "If you expected to have heterogeneous node types within this graph, maybe you have either dropped them ",
                "with a wrong filter operation or use the wrong parametrization to load the graph."
            ).to_string());
        }

        let negative_samples_threshold = if let Some(negative_samples_rate) = &negative_samples_rate
        {
            if *negative_samples_rate < 0.0
                || *negative_samples_rate > 1.0
                || !negative_samples_rate.is_finite()
            {
                return Err(format!(
                    concat!(
                        "Negative sample must be a posive ",
                        "real value between 0 and 1. ",
                        "You have provided {}."
                    ),
                    *negative_samples_rate
                ));
            }
            (negative_samples_rate * u64::MAX as f64).ceil() as u64
        } else {
            0
        };

        random_state = splitmix64(random_state);

        Ok((0..batch_size).into_par_iter().map(move |i| unsafe {
            let mut random_state = splitmix64(random_state + i as u64);
            if random_state > negative_samples_threshold {
                random_state = splitmix64(random_state);
                let (src, dst) =
                    self.get_unchecked_node_ids_from_edge_id(self.get_random_edge_id(random_state));
                return (src, dst, true);
            }

            for _ in 0..maximal_sampling_attempts {
                random_state = splitmix64(random_state);
                let (src, dst) = if use_zipfian_sampling {
                    (
                        self.get_random_zipfian_node(random_state),
                        self.get_random_zipfian_node(random_state.wrapping_mul(2)),
                    )
                } else {
                    (
                        self.get_random_node(random_state),
                        self.get_random_node(random_state.wrapping_mul(2)),
                    )
                };

                if avoid_false_negatives && support.has_edge_from_node_ids(src, dst)
                    || sample_only_edges_with_heterogeneous_node_types && {
                        self.get_unchecked_node_type_ids_from_node_id(src)
                            == self.get_unchecked_node_type_ids_from_node_id(dst)
                    }
                    || graph_to_avoid
                        .as_ref()
                        .map_or(false, |g| g.has_edge_from_node_ids(src, dst))
                {
                    continue;
                }

                return (src, dst, false);
            }

            panic!(
                concat!(
                    "Executed more than {} attempts to sample a negative edge.\n",
                    "If your graph is so small that you see this error, you may want to consider ",
                    "using one of the edge embedding transformer from the Embiggen library."
                ),
                maximal_sampling_attempts
            );
        }))
    }

    #[manual_binding]
    /// Returns n-ple with index to build numpy array, source node, source node type, destination node, destination node type, edge type and whether this edge is real or artificial.
    ///
    /// # Arguments
    /// * `random_state`: u64 - Random state of the batch to generate.
    /// * `batch_size`: usize - The maximal size of the batch to generate,
    /// * `return_node_types`: bool - Whether to return the source and destination nodes node types.
    /// * `return_edge_metrics`: bool - Whether to return the edge metrics available for both positive and negative edges.
    /// * `sample_only_edges_with_heterogeneous_node_types`: bool - Whether to sample negative edges only with source and destination nodes that have different node types.
    /// * `negative_samples_rate`: Option<f64> - The component of netagetive samples to use.
    /// * `avoid_false_negatives`: Option<bool> - Whether to remove the false negatives when generated. It should be left to false, as it has very limited impact on the training, but enabling this will slow things down.
    /// * `maximal_sampling_attempts`: Option<usize> - Number of attempts to execute to sample the negative edges.
    /// * `use_zipfian_sampling`: Option<bool> - Whether to sample the nodes using zipfian distribution. By default True. Not using this may cause significant biases.
    /// * `support`: Option<&'a Graph> - Graph to use to compute the edge metrics. When not provided, the current graph (self) is used.
    /// * `graph_to_avoid`: &'a Option<&Graph> - The graph whose edges are to be avoided during the generation of false negatives,
    ///
    /// # Raises
    /// * If the given amount of negative samples is not a positive finite real value.
    /// * If node types are requested but the graph does not contain any.
    /// * If the `sample_only_edges_with_heterogeneous_node_types` argument is provided as true, but the graph does not have node types.
    ///
    pub fn par_iter_attributed_edge_prediction_mini_batch<'a>(
        &'a self,
        random_state: u64,
        batch_size: usize,
        return_node_types: bool,
        return_edge_metrics: bool,
        sample_only_edges_with_heterogeneous_node_types: bool,
        negative_samples_rate: Option<f64>,
        avoid_false_negatives: Option<bool>,
        maximal_sampling_attempts: Option<usize>,
        use_zipfian_sampling: Option<bool>,
        support: Option<&'a Graph>,
        graph_to_avoid: Option<&'a Graph>,
    ) -> Result<
        impl IndexedParallelIterator<
                Item = (
                    NodeT,
                    Option<Vec<NodeTypeT>>,
                    NodeT,
                    Option<Vec<NodeTypeT>>,
                    Option<Vec<f32>>,
                    bool,
                ),
            > + 'a,
    > {
        let support = support.unwrap_or(&self);

        Ok(self
            .par_iter_edge_prediction_mini_batch(
                random_state,
                batch_size,
                sample_only_edges_with_heterogeneous_node_types,
                negative_samples_rate,
                avoid_false_negatives,
                maximal_sampling_attempts,
                use_zipfian_sampling,
                Some(&support),
                graph_to_avoid,
            )?
            .map(move |(src, dst, label)| {
                (
                    src,
                    if return_node_types {
                        unsafe { self.get_unchecked_edge_prediction_node_type_ids(src) }
                    } else {
                        None
                    },
                    dst,
                    if return_node_types {
                        unsafe { self.get_unchecked_edge_prediction_node_type_ids(dst) }
                    } else {
                        None
                    },
                    if return_edge_metrics {
                        Some(
                            support
                                .get_all_edge_metrics_from_node_ids_tuple(src, dst, true)
                                .unwrap(),
                        )
                    } else {
                        None
                    },
                    label,
                )
            }))
    }

    /// Returns n-ple with terms used for training a siamese network.
    ///
    /// # Arguments
    /// * `random_state`: u64 - The random state to reproduce the batch.
    /// * `batch_size`: usize - The maximal size of the batch to generate,
    /// * `use_zipfian_sampling`: Option<bool> - Whether to sample the nodes using zipfian distribution. By default True. Not using this may cause significant biases.
    ///
    pub fn par_iter_siamese_mini_batch(
        &self,
        random_state: u64,
        batch_size: usize,
        use_zipfian_sampling: Option<bool>,
    ) -> impl IndexedParallelIterator<Item = (NodeT, NodeT, NodeT, NodeT, Option<EdgeTypeT>)> + '_
    {
        let use_zipfian_sampling = use_zipfian_sampling.unwrap_or(true);
        let random_state = splitmix64(random_state);
        (0..batch_size).into_par_iter().map(move |i| unsafe {
            let mut random_state = splitmix64(random_state + i as u64);
            let edge_id = self.get_random_edge_id(random_state);
            let (src, dst) = self.get_unchecked_node_ids_from_edge_id(edge_id);
            random_state = splitmix64(random_state);
            let (not_src, not_dst) = if use_zipfian_sampling {
                (
                    self.get_random_zipfian_node(random_state),
                    self.get_random_zipfian_node(random_state.wrapping_mul(2)),
                )
            } else {
                (
                    self.get_random_node(random_state),
                    self.get_random_node(random_state.wrapping_mul(2)),
                )
            };
            (
                src,
                dst,
                not_src,
                not_dst,
                self.get_unchecked_edge_type_id_from_edge_id(edge_id),
            )
        })
    }

    /// Returns n-ple with terms used for training a kgsiamese network.
    ///
    /// # Arguments
    /// * `random_state`: u64 - The random state to reproduce the batch.
    /// * `batch_size`: usize - The maximal size of the batch to generate,
    /// * `use_zipfian_sampling`: Option<bool> - Whether to sample the nodes using zipfian distribution. By default True. Not using this may cause significant biases.
    ///
    pub fn par_iter_kgsiamese_mini_batch(
        &self,
        random_state: u64,
        batch_size: usize,
        use_zipfian_sampling: Option<bool>,
    ) -> impl IndexedParallelIterator<
        Item = (
            NodeT,
            NodeT,
            NodeT,
            NodeT,
            Option<Vec<NodeTypeT>>,
            Option<Vec<NodeTypeT>>,
            Option<Vec<NodeTypeT>>,
            Option<Vec<NodeTypeT>>,
            Option<EdgeTypeT>,
        ),
    > + '_ {
        self.par_iter_siamese_mini_batch(random_state, batch_size, use_zipfian_sampling)
            .map(move |(src, dst, not_src, not_dst, edge_type)| unsafe {
                (
                    src,
                    dst,
                    not_src,
                    not_dst,
                    self.get_unchecked_node_type_ids_from_node_id(src)
                        .map(|x| x.clone()),
                    self.get_unchecked_node_type_ids_from_node_id(dst)
                        .map(|x| x.clone()),
                    self.get_unchecked_node_type_ids_from_node_id(not_src)
                        .map(|x| x.clone()),
                    self.get_unchecked_node_type_ids_from_node_id(not_dst)
                        .map(|x| x.clone()),
                    edge_type,
                )
            })
    }

    /// Returns n-ple with index to build numpy array, source node, source node type, destination node, destination node type.
    ///
    /// # Arguments
    /// * `idx`: usize - The index of the batch to generate,
    /// * `graph`: &Graph - The graph from which to extract the edge IDs.
    /// * `batch_size`: usize - The maximal size of the batch to generate,
    /// * `return_node_types`: bool - Whether to return the source and destination nodes node types.
    /// * `return_edge_types`: bool - Whether to return the edge types.
    /// * `return_edge_metrics`: bool - Whether to return the edge metrics.
    ///
    /// # Raises
    /// * If node types are requested but the graph does not contain any.
    /// * If edge types are requested but the graph does not contain any.
    ///
    pub fn get_edge_prediction_chunk_mini_batch<'a>(
        &'a self,
        idx: usize,
        graph: &'a Graph,
        batch_size: usize,
        return_node_types: bool,
        return_edge_types: bool,
        return_edge_metrics: bool,
    ) -> Result<
        impl IndexedParallelIterator<
                Item = (
                    NodeT,
                    Option<Vec<NodeTypeT>>,
                    NodeT,
                    Option<Vec<NodeTypeT>>,
                    Option<EdgeTypeT>,
                    Option<Vec<f32>>,
                ),
            > + 'a,
    > {
        if return_node_types {
            self.must_have_known_node_types()?;
        }
        if return_edge_types {
            self.must_have_known_edge_types()?;
        }
        Ok((batch_size * idx
            ..(batch_size * (idx + 1)).min(graph.get_number_of_directed_edges() as usize))
            .into_par_iter()
            .map(move |edge_id| unsafe {
                let (src, dst) = graph.get_unchecked_node_ids_from_edge_id(edge_id as u64);
                (
                    src,
                    if return_node_types {
                        self.get_unchecked_edge_prediction_node_type_ids(src)
                    } else {
                        None
                    },
                    dst,
                    if return_node_types {
                        self.get_unchecked_edge_prediction_node_type_ids(dst)
                    } else {
                        None
                    },
                    if return_edge_types {
                        self.get_edge_type_id_from_edge_node_ids(src, dst)
                            .unwrap_or(None)
                    } else {
                        None
                    },
                    if return_edge_metrics {
                        Some(
                            self.get_unchecked_all_edge_metrics_from_node_ids_tuple(src, dst, true),
                        )
                    } else {
                        None
                    },
                )
            }))
    }

    /// Returns triple with the degrees of source nodes, destination nodes and labels for training model for link prediction.
    /// This method is just for setting the lowerbound on the simplest possible model.
    ///
    /// # Arguments
    ///
    /// * `idx`: u64 - The index of the batch to generate, behaves like a random random_state,
    /// * `batch_size`: usize - The maximal size of the batch to generate,
    /// * `normalize`: bool - Divide the degrees by the max, this way the values are in [0, 1],
    /// * `negative_samples`: Option<f64> - The component of netagetive samples to use,
    /// * `avoid_false_negatives`: bool - Whether to remove the false negatives when generated. It should be left to false, as it has very limited impact on the training, but enabling this will slow things down.
    /// * `maximal_sampling_attempts`: usize - Number of attempts to execute to sample the negative edges.
    /// * `shuffle`: Option<bool> - Whether to shuffle the samples within the batch.
    /// * `sample_only_edges_with_heterogeneous_node_types`: Option<bool> - Whether to sample negative edges only with source and destination nodes that have different node types.
    /// * `use_zipfian_sampling`: Option<bool> - Whether to sample the nodes using zipfian distribution. By default True. Not using this may cause significant biases.
    /// * `support`: Option<&'a Graph> - Graph to use to compute the edge metrics. When not provided, the current graph (self) is used.
    /// * `graph_to_avoid`: &'a Option<&Graph> - The graph whose edges are to be avoided during the generation of false negatives,
    ///
    /// # Raises
    /// * If the given amount of negative samples is not a positive finite real value.
    pub fn link_prediction_degrees<'a>(
        &'a self,
        random_state: u64,
        batch_size: usize,
        normalize: Option<bool>,
        negative_samples: Option<f64>,
        avoid_false_negatives: Option<bool>,
        maximal_sampling_attempts: Option<usize>,
        sample_only_edges_with_heterogeneous_node_types: bool,
        use_zipfian_sampling: Option<bool>,
        support: Option<&'a Graph>,
        graph_to_avoid: Option<&'a Graph>,
    ) -> Result<impl ParallelIterator<Item = (f64, f64, bool)> + 'a> {
        let iter = self.par_iter_attributed_edge_prediction_mini_batch(
            random_state,
            batch_size,
            false,
            false,
            sample_only_edges_with_heterogeneous_node_types,
            negative_samples,
            avoid_false_negatives,
            maximal_sampling_attempts,
            use_zipfian_sampling,
            support,
            graph_to_avoid,
        )?;

        let normalize = normalize.unwrap_or(true);

        let max_degree = match normalize {
            true => self.get_maximum_node_degree()? as f64,
            false => 1.0,
        };

        Ok(iter.map(move |(src, _, dst, _, _, label)| unsafe {
            (
                self.get_unchecked_node_degree_from_node_id(src) as f64 / max_degree,
                self.get_unchecked_node_degree_from_node_id(dst) as f64 / max_degree,
                label,
            )
        }))
    }

    #[fuzz_type(iterations: Option<u8>)]
    /// Returns okapi node features propagation within given maximal distance.
    ///
    /// # Arguments
    /// * `features`: Vec<Option<Vec<f64>>> - The features to propagate. Use None to represent eventual unknown features.
    /// * `iterations`: Option<usize> - The number of iterations to execute. By default one.
    /// * `maximal_distance`: Option<usize> - The distance to consider for the cooccurrences. The default value is 3.
    /// * `k1`: Option<f64> - The k1 parameter from okapi. Tipicaly between 1.2 and 2.0. It can be seen as a smoothing.
    /// * `b`: Option<f64> - The b parameter from okapi. Tipicaly 0.75.
    /// * `include_central_node`: Option<bool> - Whether to include the central node. By default true.
    /// * `verbose`: Option<bool> - Whether to show loading bar.
    ///
    /// # Raises
    /// * If the graph does not have node types.
    ///
    /// # References
    /// The algorithm implemented is a generalization of the OKAPI BM25 TFIDF
    /// algorithm generalized for graphs.
    pub fn get_okapi_bm25_node_feature_propagation(
        &self,
        mut features: Vec<Vec<f64>>,
        iterations: Option<usize>,
        maximal_distance: Option<usize>,
        k1: Option<f64>,
        b: Option<f64>,
        include_central_node: Option<bool>,
        verbose: Option<bool>,
    ) -> Result<Vec<Vec<f64>>> {
        // The graph must have nodes to support node feature propagation
        self.must_have_nodes()?;
        // Validate the provided features
        validate_features(&features, self.get_nodes_number() as usize)?;
        // We use as default distance 3
        let maximal_distance = maximal_distance.unwrap_or(3);
        // K1 values are typically between 1.2 and 2.0 in absence of additional
        // tuning of the model.
        let k1 = k1.unwrap_or(1.5);
        if k1 <= 0.0 {
            return Err("The value of k1 must be a strictly positive real number.".to_string());
        }
        // b values are tipically equal to 0.75 in abscence of additional tuning.
        let b = b.unwrap_or(0.75);
        if k1 <= 0.0 {
            return Err("The value of b must be a strictly positive real number.".to_string());
        }
        // By default we only execute 1 iteration
        let iterations = iterations.unwrap_or(1);
        // The number of iterations must be equal or greater than one.
        if iterations == 0 {
            return Err(
                "The number of iterations must be a strictly positive integer.".to_string(),
            );
        }
        // By default we include the features of the central node.
        // This is a bias in the context of labels.
        let include_central_node = include_central_node.unwrap_or(true);
        // Get the number of possible elements in the features vocabulary
        let features_number = features[0].len() as usize;
        // Get the number of 'documents'
        let nodes_number = self.get_nodes_number() as usize;
        // Loading bar
        let iterations_progress_bar = get_loading_bar(
            verbose.unwrap_or(true) && iterations > 1,
            "[Iterating features] propagation",
            iterations,
        );
        // Execute the propagation
        for _ in (0..iterations).progress_with(iterations_progress_bar) {
            // Computing the inverse document features (IDF)
            let inverse_document_frequencies = (0..features_number)
                .map(|feature_number| {
                    let feature_sum = self
                        .iter_node_ids()
                        .map(|node_id| (features[node_id as usize][feature_number] > 0.0) as NodeT)
                        .sum::<NodeT>();
                    // Definition of the IDF from Okapi, generalized for the
                    // real frequencies.
                    ((nodes_number as f64 - feature_sum as f64 + 0.5) / (feature_sum as f64 + 0.5)
                        + 1.0)
                        .ln()
                })
                .collect::<Vec<f64>>();
            let total_document_size = AtomicF64::new(0.0);
            // Creating loading bar
            let pb = get_loading_bar(
                verbose.unwrap_or(true),
                "Computing new co-occurrences",
                nodes_number,
            );
            // Update features
            features = self
                .par_iter_node_ids()
                .progress_with(pb)
                .map(|node_id| {
                    // Create a new empty queue.
                    let mut neighbours_stack = VecDeque::with_capacity(nodes_number);
                    // Put the distance of the original node as 0.
                    neighbours_stack.push_front((node_id, 0));
                    // Create a binary mask for the visited node.
                    let mut visited = bitvec![Lsb0, u8; 0; nodes_number];
                    // Initialize the sum of the features
                    let mut document_features_sum = 0.0;
                    // We set the current root node as visited
                    unsafe { *visited.get_unchecked_mut(node_id as usize) = true };
                    // We initialize the local weighted co-occurrences
                    let mut cooccurrences = if include_central_node {
                        features[node_id as usize].clone()
                    } else {
                        vec![0.0; features_number]
                    };
                    // Iterating over
                    while let Some((current_node_id, distance)) = neighbours_stack.pop_back() {
                        let new_distance = distance + 1;
                        unsafe {
                            self.iter_unchecked_neighbour_node_ids_from_source_node_id(
                                current_node_id,
                            )
                        }
                        .for_each(|neighbour_node_id| {
                            if visited[neighbour_node_id as usize] {
                                return;
                            }
                            unsafe {
                                *visited.get_unchecked_mut(neighbour_node_id as usize) = true
                            };
                            features[neighbour_node_id as usize]
                                .iter()
                                .cloned()
                                .enumerate()
                                .for_each(|(i, feature)| {
                                    let normalized_feature = feature / (new_distance as f64).pow(2);
                                    document_features_sum += normalized_feature;
                                    cooccurrences[i] += normalized_feature;
                                });
                            if new_distance <= maximal_distance {
                                neighbours_stack.push_front((neighbour_node_id, new_distance));
                            }
                        });
                    }
                    total_document_size
                        .fetch_add(document_features_sum, std::sync::atomic::Ordering::Relaxed);
                    cooccurrences
                })
                .collect::<Vec<Vec<f64>>>();
            // Computing average document size
            let average_document_size = total_document_size
                .load(std::sync::atomic::Ordering::Relaxed)
                / nodes_number as f64;
            // Creating loading bar
            let pb = get_loading_bar(
                verbose.unwrap_or(true),
                "Propagating features",
                nodes_number,
            );
            features
                .par_iter_mut()
                .progress_with(pb)
                .for_each(|node_cooccurrences| {
                    let document_features_sum = node_cooccurrences.iter().sum::<f64>();
                    if document_features_sum > 0.0 {
                        node_cooccurrences.iter_mut().enumerate().for_each(
                            |(node_type, cooccurrence)| {
                                *cooccurrence = inverse_document_frequencies[node_type]
                                    * ((*cooccurrence * (k1 + 1.0))
                                        / (*cooccurrence
                                            + k1 * (1.0 - b
                                                + b * document_features_sum
                                                    / average_document_size)));
                            },
                        );
                    }
                });
            // We have to run a min-max scaling because otherwise
            // the biases caused by a large BFS exploration will obscure the
            // local variance.
            let min_max = (0..features_number)
                .map(|feature_number| {
                    self.iter_node_ids()
                        .map(|node_id| features[node_id as usize][feature_number])
                        .minmax()
                        .into_option()
                        .unwrap()
                })
                .collect::<Vec<_>>();
            features.par_iter_mut().for_each(|node_features| {
                node_features.iter_mut().zip(min_max.iter()).for_each(
                    |(feature, &(min_feature, max_feature))| {
                        *feature =
                            (*feature - min_feature) / (max_feature - min_feature + f64::EPSILON);
                    },
                );
            });
        }
        Ok(features)
    }

    #[fuzz_type(iterations: Option<u8>)]
    /// Returns okapi node label propagation within given maximal distance.
    ///
    /// # Arguments
    /// * `iterations`: Option<usize> - The number of iterations to execute. By default one.
    /// * `maximal_distance`: Option<usize> - The distance to consider for the cooccurrences. The default value is 3.
    /// * `k1`: Option<f64> - The k1 parameter from okapi. Tipicaly between 1.2 and 2.0. It can be seen as a smoothing.
    /// * `b`: Option<f64> - The b parameter from okapi. Tipicaly 0.75.
    /// * `verbose`: Option<bool> - Whether to show loading bar.
    ///
    /// # Raises
    /// * If the graph does not have node types.
    ///
    /// # References
    /// The algorithm implemented is a generalization of the OKAPI BM25 TFIDF
    /// algorithm generalized for graphs.
    pub fn get_okapi_bm25_node_label_propagation(
        &self,
        iterations: Option<usize>,
        maximal_distance: Option<usize>,
        k1: Option<f64>,
        b: Option<f64>,
        verbose: Option<bool>,
    ) -> Result<Vec<Vec<f64>>> {
        self.get_okapi_bm25_node_feature_propagation(
            self.get_one_hot_encoded_node_types()?
                .into_iter()
                .map(|dummies| {
                    dummies
                        .into_iter()
                        .map(|dummie| if dummie { 1.0 } else { 0.0 })
                        .collect()
                })
                .collect(),
            iterations,
            maximal_distance,
            k1,
            b,
            Some(false),
            verbose,
        )
    }
}
