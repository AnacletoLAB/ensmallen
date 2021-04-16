use super::*;
use indicatif::ProgressIterator;
use rand::rngs::StdRng;
use rand::seq::SliceRandom;
use rand::SeedableRng;
use rayon::prelude::*;
use std::collections::HashMap;
use vec_rand::gen_random_vec;
use vec_rand::xorshift::xorshift;

#[inline(always)]
/// Computes val % n using lemires fast method for u32.
/// https://lemire.me/blog/2016/06/27/a-fast-alternative-to-the-modulo-reduction/
/// This is supposed to be ~5 times faster.
fn fast_u32_modulo(val: u32, n: u32) -> u32 {
    ((val as u64 * n as u64) >> 32) as u32
}

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
) -> Result<impl ParallelIterator<Item = (Vec<NodeT>, NodeT)> + 'a, String> {
    Ok(sequences.flat_map_iter(move |sequence| {
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
    }))
}

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
    verbose: bool,
) -> Result<(usize, impl Iterator<Item = (NodeT, NodeT, f64)>), String> {
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
    /// * `walk_parameters`: &WalksParameters - the weighted walks parameters.
    /// * `quantity`: NodeT - Number of nodes to consider.
    /// * `window_size`: usize - Window size to consider for the sequences.
    ///
    pub fn node2vec<'a>(
        &'a self,
        walk_parameters: &'a WalksParameters,
        quantity: NodeT,
        window_size: usize,
    ) -> Result<impl ParallelIterator<Item = (Vec<NodeT>, NodeT)> + 'a, String> {
        // do the walks and check the result
        word2vec(
            self.random_walks_iter(quantity, walk_parameters)?,
            window_size,
        )
    }

    /// Return triple with CSR representation of cooccurrence matrix.
    ///
    /// The first vector has the sources, the second vector the destinations
    /// and the third one contains the min-max normalized frequencies.
    ///
    /// # Arguments
    ///
    /// * `parameters`: &WalksParameters - the walks parameters.
    /// * `window_size`: Option<usize> - Window size to consider for the sequences.
    /// * `verbose`: bool - Whether to show the progress bars. The default behaviour is false.
    ///     
    pub fn cooccurence_matrix<'a>(
        &'a self,
        walks_parameters: &'a WalksParameters,
        window_size: usize,
        verbose: bool,
    ) -> Result<(usize, impl Iterator<Item = (NodeT, NodeT, f64)> + 'a), String> {
        if !self.has_edges() {
            return Err(
                "The cooccurence matrix on a graph without edges is not defined.".to_string(),
            );
        }
        let walks = self.complete_walks_iter(walks_parameters)?;
        cooccurence_matrix(
            walks,
            window_size,
            (self.get_unique_source_nodes_number() * walks_parameters.iterations) as usize,
            verbose,
        )
    }

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
    pub(crate) fn get_node_label_prediction_tuple_from_node_id(
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
    /// # let graph = graph::test_utilities::load_ppi(true, true, true, false, false, false).unwrap();
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
        if !self.has_node_types() {
            return Err("The current graph instance does not have node types!".to_string());
        }
        Ok(node_ids.into_iter().map(move |node_id| {
            self.get_node_label_prediction_tuple_from_node_id(
                node_id,
                random_state,
                include_central_node,
                offset,
                max_neighbours,
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
    /// * `negative_samples`: f64 - The component of netagetive samples to use,
    /// * `avoid_false_negatives`: bool - Whether to remove the false negatives when generated. It should be left to false, as it has very limited impact on the training, but enabling this will slow things down.
    /// * `maximal_sampling_attempts`: usize - Number of attempts to execute to sample the negative edges.
    /// * `graph_to_avoid`: &Option<&Graph> - The graph whose edges are to be avoided during the generation of false negatives,
    ///
    pub fn link_prediction_degrees<'a>(
        &'a self,
        idx: u64,
        batch_size: usize,
        normalize: bool,
        negative_samples: f64,
        avoid_false_negatives: bool,
        maximal_sampling_attempts: usize,
        graph_to_avoid: &'a Option<&Graph>,
    ) -> Result<impl ParallelIterator<Item = (usize, f64, f64, bool)> + 'a, String> {
        let iter = self.link_prediction_ids(
            idx,
            batch_size,
            negative_samples,
            avoid_false_negatives,
            maximal_sampling_attempts,
            graph_to_avoid,
        )?;

        let max_degree = match normalize {
            true => self.get_max_node_degree()? as f64,
            false => 1.0,
        };

        Ok(iter.map(move |(index, src, dst, label)| {
            (
                index,
                self.get_unchecked_node_degree_from_node_id(src) as f64 / max_degree,
                self.get_unchecked_node_degree_from_node_id(dst) as f64 / max_degree,
                label,
            )
        }))
    }

    /// Returns triple with the ids of source nodes, destination nodes and labels for training model for link prediction.
    ///
    /// # Arguments
    ///
    /// * `idx`: u64 - The index of the batch to generate, behaves like a random random_state,
    /// * `batch_size`: usize - The maximal size of the batch to generate,
    /// * `negative_samples`: f64 - The component of netagetive samples to use,
    /// * `avoid_false_negatives`: bool - Whether to remove the false negatives when generated. It should be left to false, as it has very limited impact on the training, but enabling this will slow things down.
    /// * `maximal_sampling_attempts`: usize - Number of attempts to execute to sample the negative edges.
    /// * `graph_to_avoid`: &Option<&Graph> - The graph whose edges are to be avoided during the generation of false negatives,
    ///
    pub fn link_prediction_ids<'a>(
        &'a self,
        idx: u64,
        batch_size: usize,
        negative_samples: f64,
        avoid_false_negatives: bool,
        maximal_sampling_attempts: usize,
        graph_to_avoid: &'a Option<&Graph>,
    ) -> Result<impl ParallelIterator<Item = (usize, NodeT, NodeT, bool)> + 'a, String> {
        // xor the random_state with a constant so that we have a good amount of 0s and 1s in the number
        // even with low values (this is needed becasue the random_state 0 make xorshift return always 0)
        let random_state = idx ^ SEED_XOR as u64;

        if negative_samples < 0.0 || !negative_samples.is_finite() {
            return Err("Negative sample must be a posive real value.".to_string());
        }

        // The number of negatives is given by computing their fraction of batchsize
        let negative_number: usize =
            ((batch_size as f64 / (1.0 + negative_samples)) * negative_samples) as usize;
        // All the remaining values then are positives
        let positive_number: usize = batch_size - negative_number;
        let graph_has_no_selfloops = !self.has_selfloops();

        let edges_number = self.get_directed_edges_number() as u64;
        let nodes_number = self.get_nodes_number() as u32;

        let mut rng: StdRng = SeedableRng::seed_from_u64(random_state);
        let random_values = gen_random_vec(batch_size, random_state);
        let mut indices: Vec<usize> = (0..batch_size).collect();
        indices.shuffle(&mut rng);

        Ok((0..batch_size)
            .into_par_iter()
            .map(move |i| {
                let mut sampled = random_values[i];
                if i < positive_number{
                    let (src, dst) = self.get_unchecked_node_ids_from_edge_id(sampled % edges_number);
                    (indices[i], src, dst, true)
                } else {
                    for _ in 0..maximal_sampling_attempts {
                        // split the random u64 into 2 u32 and mod them to have
                        // usable nodes (this is slightly biased towards low values)
                        let src = fast_u32_modulo((sampled & 0xffffffff) as u32, nodes_number);
                        let dst = fast_u32_modulo((sampled >> 32) as u32, nodes_number);

                        if avoid_false_negatives && self.has_edge_from_node_ids(src, dst) {
                            sampled = xorshift(sampled);
                            continue;
                        }

                        if let Some(g) = &graph_to_avoid {
                            if g.has_edge_from_node_ids(src, dst)  {
                                sampled = xorshift(sampled);
                                continue;
                            }
                        }

                        if graph_has_no_selfloops && src == dst {
                            sampled = xorshift(sampled);
                            continue;
                        }

                        return (indices[i], src, dst, false);
                    }
                    panic!(
                        concat!(
                            "Executed more than {} attempts to sample a negative edge.\n",
                            "If your graph is so small that you see this error, you may want to consider ",
                            "using one of the edge embedding transformer from the Embiggen library."
                        ),
                        maximal_sampling_attempts
                    );
                }
            }))
    }
}
