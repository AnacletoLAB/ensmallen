use super::*;
use atomic_float::AtomicF64;
use bitvec::prelude::*;
use indicatif::{ParallelProgressIterator, ProgressIterator};
use itertools::Itertools;
use num_traits::Pow;
use rayon::prelude::*;
use std::collections::{HashMap, VecDeque};
use vec_rand::gen_random_vec;
use vec_rand::xorshift::xorshift;

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

#[manual_binding]
/// Return triple with CSR representation of cooccurrence matrix.
///
/// The first vector has the sources, the second vector the destinations
/// and the third one contains the min-max normalized frequencies.
///
/// # Arguments
///
/// * `sequences`: impl ParallelIterator<Item = Vec<NodeT>> - the sequence of sequences of integers to preprocess.
/// * `window_size`: Option<usize> - Window size to consider for the sequences.
/// * `verbose`: Option<bool> - Whether to show the progress bars. The default behaviour is false.
///     
pub fn cooccurence_matrix(
    sequences: impl ParallelIterator<Item = Vec<NodeT>>,
    window_size: usize,
    number_of_sequences: usize,
    verbose: Option<bool>,
) -> Result<(usize, impl Iterator<Item = (NodeT, NodeT, f64)>), String> {
    let verbose = verbose.unwrap_or(false);
    let mut cooccurence_matrix: HashMap<(NodeT, NodeT), f64> = HashMap::new();
    let mut max_frequency = 0.0;
    let pb1 = get_loading_bar(verbose, "Computing frequencies", number_of_sequences);

    // TODO!: Avoid this collect and create the cooccurrence matrix in a parallel way.
    // We are currently working on this but is terribly non-trivial,
    // as most parallel implementations end up being slower than sequential
    // ones or require massive amounts of additional memory.
    let vec = sequences.collect::<Vec<Vec<NodeT>>>();
    vec.iter().progress_with(pb1).for_each(|sequence| {
        let walk_length = sequence.len();
        for (central_index, &central_word_id) in sequence.iter().enumerate() {
            let upperbound = std::cmp::min(1 + window_size, walk_length - central_index);

            for distance in 1..upperbound {
                let context_id = sequence[central_index + distance];

                let (smaller, bigger) = (
                    std::cmp::min(central_word_id, context_id),
                    std::cmp::max(central_word_id, context_id),
                );

                let freq = 1.0 / distance as f64;

                // Get the current value for this pair of nodes
                let ptr = cooccurence_matrix
                    .entry((smaller, bigger))
                    .and_modify(|e| *e += freq)
                    .or_insert(freq);
                // Update the max
                if *ptr > max_frequency {
                    max_frequency = *ptr;
                }
            }
        }
    });

    let number_of_elements = cooccurence_matrix.len();
    let pb2 = get_loading_bar(
        verbose,
        "Converting mapping into CSR matrix",
        cooccurence_matrix.len(),
    );
    Ok((
        number_of_elements,
        cooccurence_matrix
            .into_iter()
            .progress_with(pb2)
            .map(move |((word, context), frequency)| (word, context, frequency / max_frequency)),
    ))
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
    ) -> Result<impl ParallelIterator<Item = (Vec<NodeT>, NodeT)> + 'a, String> {
        Ok(word2vec(
            self.iter_random_walks(quantity, walk_parameters)?,
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
    ///
    /// * `walks_parameters`: &'a WalksParameters - the walks parameters.
    /// * `window_size`: usize - Window size to consider for the sequences.
    /// * `verbose`: Option<bool> - Whether to show the progress bars. The default behaviour is false.
    ///     
    pub fn cooccurence_matrix<'a>(
        &'a self,
        walks_parameters: &'a WalksParameters,
        window_size: usize,
        verbose: Option<bool>,
    ) -> Result<(usize, impl Iterator<Item = (NodeT, NodeT, f64)> + 'a), String> {
        self.must_have_edges()?;
        let walks = self.iter_complete_walks(walks_parameters)?;
        cooccurence_matrix(
            walks,
            window_size,
            (self.get_unique_source_nodes_number() * walks_parameters.iterations) as usize,
            verbose,
        )
    }

    #[manual_binding]
    /// Return iterator over neighbours for the given node ID, optionally including given node ID.
    ///
    /// This method is meant to be used to predict node labels using the NoLaN model.
    ///
    /// If you need to predict the node label of a node, not during training,
    /// use `max_neighbours=None`.
    ///
    /// # Arguments
    ///
    /// * `central_node_id`: NodeT - The node ID to retrieve neighbours for.
    /// * `random_state`: u64 - The random state to use to extract the neighbours.
    /// * `include_central_node`: bool - Whether to include the node ID in the returned iterator.
    /// * `offset`: NodeT - Offset for padding porposes.
    /// * `max_neighbours`: Option<NodeT> - Number of maximum neighbours to consider.
    ///
    pub(crate) fn get_neighbours_from_node_id(
        &self,
        central_node_id: NodeT,
        random_state: u64,
        include_central_node: bool,
        offset: NodeT,
        max_neighbours: Option<NodeT>,
    ) -> impl Iterator<Item = NodeT> + '_ {
        (if include_central_node {
            vec![central_node_id]
        } else {
            vec![]
        })
        .into_iter()
        .chain(
            self.get_unchecked_destination_node_ids_from_node_id(
                central_node_id,
                random_state,
                max_neighbours,
            )
            .into_iter(),
        )
        .map(move |node_id| node_id + offset)
    }

    /// Return tuple with iterator over neighbours for the given node ID, optionally including given node ID, and node type.
    ///
    /// This method is meant to be used to predict node labels using the NoLaN model.
    ///
    /// If you need to predict the node label of a node, not during training,
    /// use `max_neighbours=None`.
    ///
    /// # Arguments
    ///
    /// * `node_id`: NodeT - The node ID to retrieve neighbours for.
    /// * `random_state`: u64 - The random state to use to extract the neighbours.
    /// * `include_central_node`: bool - Whether to include the node ID in the returned iterator.
    /// * `offset`: NodeT - Offset for padding porposes.
    /// * `max_neighbours`: Option<NodeT> - Number of maximum neighbours to consider.
    ///
    /// # Safety
    /// The method will return a None node type also when the graph does
    /// not contain node types.
    pub(crate) unsafe fn get_node_label_prediction_tuple_from_node_id(
        &self,
        node_id: NodeT,
        random_state: u64,
        include_central_node: bool,
        offset: NodeT,
        max_neighbours: Option<NodeT>,
    ) -> (impl Iterator<Item = NodeT> + '_, Option<Vec<NodeTypeT>>) {
        (
            self.get_neighbours_from_node_id(
                node_id,
                random_state,
                include_central_node,
                offset,
                max_neighbours,
            ),
            self.get_unchecked_node_type_id_from_node_id(node_id),
        )
    }

    #[manual_binding]
    /// Return iterator over neighbours for the given node IDs, optionally including given the node IDs, and node type.
    ///
    /// This method is meant to be used to predict node labels using the NoLaN model.
    ///
    /// If you need to predict the node label of a node, not during training,
    /// use `max_neighbours=None`.
    ///
    /// # Arguments
    ///
    /// * `node_ids`: Vec<NodeT> - The node ID to retrieve neighbours for.
    /// * `random_state`: u64 - The random state to use to extract the neighbours.
    /// * `include_central_node`: bool - Whether to include the node ID in the returned iterator.
    /// * `offset`: NodeT - Offset for padding porposes.
    /// * `max_neighbours`: Option<NodeT> - Number of maximum neighbours to consider.
    ///
    /// # Example
    /// Suppose you want to the get the neighbours of the first 10 nodes:
    /// ```rust
    /// # use rayon::iter::ParallelIterator;
    /// # use graph::NodeT;
    /// # use rayon::iter::IndexedParallelIterator;
    /// # let graph = graph::test_utilities::load_ppi(true, true, true, false, false, false);
    /// let node_ids = (0..10).collect::<Vec<NodeT>>();
    /// let include_central_nodes = true;
    /// let offset = 0;
    /// let max_neighbours = 5;
    /// let iterator = graph.get_node_label_prediction_tuple_from_node_ids(
    ///    node_ids.clone(), 42, include_central_nodes, offset, Some(max_neighbours)
    /// ).unwrap();
    /// iterator.enumerate().for_each(|(i, (neighbours_iter, labels))|{
    ///     for (j, node_id) in neighbours_iter.enumerate(){
    ///         if j==0 && include_central_nodes{
    ///             assert!(node_id==node_ids[i]);
    ///         }
    ///         assert!(
    ///             max_neighbours + include_central_nodes as NodeT > j as NodeT,
    ///             "The index {} is higher than the given maximum neighbours number {}!",
    ///             j,
    ///             max_neighbours
    ///         );
    ///     }
    /// });
    /// ```
    ///
    /// # Raises
    /// * If the graph does not contain node type IDs.
    pub fn get_node_label_prediction_tuple_from_node_ids(
        &self,
        node_ids: Vec<NodeT>,
        random_state: u64,
        include_central_node: bool,
        offset: NodeT,
        max_neighbours: Option<NodeT>,
    ) -> Result<
        impl Iterator<Item = (impl Iterator<Item = NodeT> + '_, Option<Vec<NodeTypeT>>)> + '_,
        String,
    > {
        self.must_have_node_types()?;
        Ok(node_ids.into_iter().map(move |node_id| unsafe {
            self.get_node_label_prediction_tuple_from_node_id(
                node_id,
                random_state,
                include_central_node,
                offset,
                max_neighbours,
            )
        }))
    }

    #[manual_binding]
    /// Returns n-ple with index to build numpy array, source node, source node type, destination node, destination node type, edge type and whether this edge is real or artificial.
    ///
    /// # Arguments
    /// * `idx`: u64 - The index of the batch to generate, behaves like a random random_state,
    /// * `batch_size`: Option<usize> - The maximal size of the batch to generate,
    /// * `negative_samples_percentage`: Option<f64> - The component of netagetive samples to use.
    /// * `return_node_types`: Option<bool> - Whether to return the source and destination nodes node types.
    /// * `return_edge_types`: Option<bool> - Whether to return the edge types. The negative edges edge type will be samples at random.
    /// * `avoid_false_negatives`: Option<bool> - Whether to remove the false negatives when generated. It should be left to false, as it has very limited impact on the training, but enabling this will slow things down.
    /// * `maximal_sampling_attempts`: Option<usize> - Number of attempts to execute to sample the negative edges.
    /// * `graph_to_avoid`: &'a Option<&Graph> - The graph whose edges are to be avoided during the generation of false negatives,
    ///
    /// # Raises
    /// * If the given amount of negative samples is not a positive finite real value.
    /// * If node types are requested but the graph does not contain any.
    /// * If node types are requested but the graph contains unknown node types.
    /// * If edge types are requested but the graph does not contain any.
    /// * If edge types are requested but the graph contains unknown edge types.
    pub fn link_prediction_ids<'a>(
        &'a self,
        idx: u64,
        batch_size: Option<usize>,
        negative_samples_percentage: Option<f64>,
        return_node_types: Option<bool>,
        return_edge_types: Option<bool>,
        avoid_false_negatives: Option<bool>,
        maximal_sampling_attempts: Option<usize>,
        graph_to_avoid: Option<&'a Graph>,
    ) -> Result<
        impl IndexedParallelIterator<
                Item = (
                    NodeT,
                    Option<Vec<NodeTypeT>>,
                    NodeT,
                    Option<Vec<NodeTypeT>>,
                    Option<EdgeTypeT>,
                    bool,
                ),
            > + 'a,
        String,
    > {
        let batch_size = batch_size.unwrap_or(1024);
        let negative_samples_percentage = negative_samples_percentage.unwrap_or(0.5);
        let avoid_false_negatives = avoid_false_negatives.unwrap_or(false);
        let maximal_sampling_attempts = maximal_sampling_attempts.unwrap_or(10_000);

        let return_node_types = return_node_types.unwrap_or(false);
        let (maximum_node_types_number, multi_label) = if return_node_types {
            self.must_not_contain_unknown_node_types()?;
            (
                self.get_maximum_multilabel_count()? as usize,
                self.has_multilabel_node_types()?,
            )
        } else {
            (0, false)
        };

        let return_edge_types = return_edge_types.unwrap_or(false);
        let edge_types_number = if return_edge_types {
            self.must_not_contain_unknown_edge_types()?;
            self.get_edge_types_number()?
        } else {
            0
        };

        if negative_samples_percentage <= 0.0
            || negative_samples_percentage >= 1.0
            || !negative_samples_percentage.is_finite()
        {
            return Err("Negative sample must be a posive real value between 0 and 1.".to_string());
        }

        let negative_samples_threshold =
            (negative_samples_percentage * u64::MAX as f64).ceil() as u64;

        // All the remaining values then are positives
        let graph_has_no_selfloops = !self.has_selfloops();

        let edges_number = self.get_directed_edges_number();
        let nodes_number = self.get_nodes_number();

        let get_node_type_ids =
            move |node_id: NodeT| -> Option<Vec<NodeTypeT>> {
                if return_node_types {
                    let node_type_ids =
                        unsafe { self.get_unchecked_node_type_id_from_node_id(node_id) };
                    if multi_label {
                        let mut padded_node_type_ids = vec![0; maximum_node_types_number];
                        node_type_ids.unwrap().into_iter().enumerate().for_each(
                            |(i, node_type)| {
                                // We need to add one because we need to reserve 0 for the mask.
                                padded_node_type_ids[i] = node_type + 1;
                            },
                        );
                        Some(padded_node_type_ids)
                    } else {
                        node_type_ids
                    }
                } else {
                    None
                }
            };

        Ok(gen_random_vec(batch_size, splitmix64(idx))
            .into_par_iter()
            .map(move |mut sampled| unsafe {
                if  sampled > negative_samples_threshold{
                    let edge_id = sampled % edges_number;
                    let (src, dst) = self.get_unchecked_node_ids_from_edge_id(edge_id);
                    let edge_type = if return_edge_types {
                        self.get_unchecked_edge_type_id_from_edge_id(edge_id)
                    } else {
                        None
                    };
                    return (
                        src,
                        get_node_type_ids(src),
                        dst,
                        get_node_type_ids(dst),
                        edge_type,
                        true
                    );
                }
                for _ in 0..maximal_sampling_attempts {
                    // split the random u64 into 2 u32 and mod them to have
                    // usable nodes (this is slightly biased towards low values)
                    let src = (sampled & 0xffffffff) as u32 % nodes_number;
                    let dst = (sampled >> 32) as u32 % nodes_number;

                    if avoid_false_negatives && self.has_edge_from_node_ids(src, dst)
                        || graph_has_no_selfloops && src == dst
                        || graph_to_avoid.as_ref().map_or(false, |g| g.has_edge_from_node_ids(src, dst)){
                        sampled = xorshift(sampled);
                        continue;
                    }

                    let edge_type = if return_edge_types {
                        Some(sampled as EdgeTypeT % edge_types_number)
                    } else {
                        None
                    };

                    return (
                        src,
                        get_node_type_ids(src),
                        dst,
                        get_node_type_ids(dst),
                        edge_type,
                        false
                    );
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
    /// Returns triple with the degrees of source nodes, destination nodes and labels for training model for link prediction.
    /// This method is just for setting the lowerbound on the simplest possible model.
    ///
    /// # Arguments
    ///
    /// * `idx`: u64 - The index of the batch to generate, behaves like a random random_state,
    /// * `batch_size`: usize - The maximal size of the batch to generate,
    /// * `normalize`: bool - Divide the degrees by the max, this way the values are in [0, 1],
    /// * `negative_samples`: f64 - The component of netagetive samples to use,
    /// * `avoid_false_negatives`: bool - Whether to remove the false negatives when generated. It should be left to false, as it has very limited impact on the training, but enabling this will slow things down.
    /// * `maximal_sampling_attempts`: usize - Number of attempts to execute to sample the negative edges.
    /// * `graph_to_avoid`: &'a Option<&Graph> - The graph whose edges are to be avoided during the generation of false negatives,
    ///
    /// # Raises
    /// * If the given amount of negative samples is not a positive finite real value.
    pub fn link_prediction_degrees<'a>(
        &'a self,
        idx: u64,
        batch_size: Option<usize>,
        normalize: Option<bool>,
        negative_samples: Option<f64>,
        avoid_false_negatives: Option<bool>,
        maximal_sampling_attempts: Option<usize>,
        graph_to_avoid: Option<&'a Graph>,
    ) -> Result<impl ParallelIterator<Item = (f64, f64, bool)> + 'a, String> {
        let iter = self.link_prediction_ids(
            idx,
            batch_size,
            negative_samples,
            Some(false),
            Some(false),
            avoid_false_negatives,
            maximal_sampling_attempts,
            graph_to_avoid,
        )?;

        let normalize = normalize.unwrap_or(true);

        let max_degree = match normalize {
            true => self.get_unweighted_max_node_degree()? as f64,
            false => 1.0,
        };

        Ok(iter.map(move |(src, _, dst, _, _, label)| unsafe {
            (
                self.get_unchecked_unweighted_node_degree_from_node_id(src) as f64 / max_degree,
                self.get_unchecked_unweighted_node_degree_from_node_id(dst) as f64 / max_degree,
                label,
            )
        }))
    }

    #[manual_binding]
    /// Returns all available edge prediction metrics for given edges.
    ///
    /// The metrics returned are, in order:
    /// - Adamic Adar index
    /// - Jaccard Coefficient
    /// - Resource Allocation index
    /// - Normalized preferential attachment score
    ///
    /// # Arguments
    /// source_node_ids: Vec<NodeT> - List of source node IDs.
    /// destination_node_ids: Vec<NodeT> - List of destination node IDs.
    ///
    /// # Safety
    /// If one of the given nodes does not exists in the graph, i.e. that is
    /// higher than the number of nodes in the graph, the method will panic
    /// and crash.
    ///
    pub unsafe fn par_iter_unchecked_edge_prediction_metrics(
        &self,
        source_node_ids: Vec<NodeT>,
        destination_node_ids: Vec<NodeT>,
    ) -> impl IndexedParallelIterator<Item = Vec<f64>> + '_ {
        source_node_ids
            .into_par_iter()
            .zip(destination_node_ids.into_par_iter())
            .map(move |(source_node_id, destination_node_id)| {
                vec![
                    self.get_unchecked_adamic_adar_index(source_node_id, destination_node_id),
                    self.get_unchecked_jaccard_coefficient(source_node_id, destination_node_id),
                    self.get_unchecked_resource_allocation_index(
                        source_node_id,
                        destination_node_id,
                    ),
                    self.get_unchecked_preferential_attachment(
                        source_node_id,
                        destination_node_id,
                        true,
                    ),
                ]
            })
    }

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
    ) -> Result<Vec<Vec<f64>>, String> {
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
    ) -> Result<Vec<Vec<f64>>, String> {
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
