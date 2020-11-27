use super::*;
use rand::rngs::StdRng;
use rand::seq::SliceRandom;
use rand::SeedableRng;
use rayon::prelude::*;
use vec_rand::gen_random_vec;
use vec_rand::xorshift::xorshift;
use std::cmp::{min, max};
use std::sync::Arc;

enum EdgeEmbeddingMethods {
    Hadamard,
    Average,
    Sum,
    L1,
    AbsoluteL1,
    L2,
}

impl EdgeEmbeddingMethods {
    fn new(method: &str) -> Result<EdgeEmbeddingMethods, String> {
        match method {
            "Hadamard" => Ok(EdgeEmbeddingMethods::Hadamard),
            "Average" => Ok(EdgeEmbeddingMethods::Average),
            "Sum" => Ok(EdgeEmbeddingMethods::Sum),
            "L1" => Ok(EdgeEmbeddingMethods::L1),
            "AbsoluteL1" => Ok(EdgeEmbeddingMethods::AbsoluteL1),
            "L2" => Ok(EdgeEmbeddingMethods::L2),
            _ => Err(format!(
                concat!(
                    "Given embedding method '{}' is not supported.",
                    "The supported methods are 'Hadamard', 'Average', 'Sum', 'AbsoluteL1', 'L1' and 'L2'."
                ),
                method
            )),
        }
    }
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
/// * sequences: Vec<Vec<usize>> - the sequence of sequences of integers to preprocess.
/// * window_size: usize - Window size to consider for the sequences.
///
pub fn word2vec<'a>(
    sequences: impl ParallelIterator<Item = Vec<NodeT>> + 'a,
    window_size: usize,
) -> Result<impl ParallelIterator<Item = (Vec<NodeT>, NodeT)> + 'a, String> {
    Ok(sequences.flat_map_iter(move |sequence| {
        let sequence_length = sequence.len();
        if sequence_length < window_size * 2 {
            panic!("You are providing sequences that are smaller than the the minimum amount.");
        }
        (window_size..(sequence_length - window_size - 1)).map(move |i| {
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
/// * sequences:Vec<Vec<usize>> - the sequence of sequences of integers to preprocess.
/// * window_size: Option<usize> - Window size to consider for the sequences.
/// * verbose: Option<bool>,
///     Wethever to show the progress bars.
///     The default behaviour is false.
///     
pub fn cooccurence_matrix(
    sequences: impl ParallelIterator<Item = Vec<NodeT>>,
    window_size: usize,
) -> (usize, impl Iterator<Item = ((NodeT, NodeT), f64)>) {
    let cooccurence_matrix = Arc::new(AtomicF64HashMap::<(NodeT, NodeT)>::new());
    // TODO!: This could be implemented using word2vec, it should be cleaner
    //        but we don't know about its performances yet.
    // fill the matrix
    sequences.for_each(|sequence| {
        // get a reference to the shared matrix
        let cooccurence_matrix = cooccurence_matrix.clone();
        let walk_length = sequence.len();
        // for each batch of size 2*window_size + 1
        (window_size..walk_length - window_size - 1).for_each(|i|{
            let central_word_id = sequence[i];
            // for each index in the current batch update the matrix
            (i - window_size..i)
                .chain(i + 1..window_size + i + 1)
                .for_each(|j| {
                    let smaller = min(central_word_id, sequence[j]) as NodeT;
                    let bigger  = max(central_word_id, sequence[j]) as NodeT;
                    cooccurence_matrix.add(&(smaller, bigger), 1.0 / (i as f64 - j as f64).abs());
                });
        });
    });

    (
        cooccurence_matrix.len(),
        Arc::try_unwrap(cooccurence_matrix).unwrap()
        .into_iter_normalized()
    )
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
    /// * walk_parameters: &WalksParameters - the weighted walks parameters.
    /// * quantity: usize - Number of nodes to consider.
    /// * window_size: usize - Window size to consider for the sequences.
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
    /// * parameters: &WalksParameters - the walks parameters.
    /// * window_size: Option<usize> - Window size to consider for the sequences.
    /// * verbose: Option<bool>,
    ///     Wethever to show the progress bars.
    ///     The default behaviour is false.
    ///     
    pub fn cooccurence_matrix<'a>(
        &'a self,
        walks_parameters: &'a WalksParameters,
        window_size: usize,
    ) -> Result<(usize, impl Iterator<Item = ((NodeT, NodeT), f64)> + 'a), String> {
        Ok(cooccurence_matrix(
            self.complete_walks_iter(walks_parameters)?,
            window_size,
        ))
    }

    /// Returns triple with source nodes, destination nodes and labels for training model for link prediction.
    ///
    /// # Arguments
    ///
    /// * idx:u64 - The index of the batch to generate, behaves like a random random_state,
    /// * batch_size: usize - The maximal size of the batch to generate,
    /// * method: &str - String representing the required edge embedding method.
    /// * negative_samples: f64 - The component of netagetive samples to use,
    /// * avoid_false_negatives: bool - Wether to remove the false negatives when generated.
    ///     - It should be left to false, as it has very limited impact on the training, but enabling this will slow things down.
    /// * maximal_sampling_attempts: usize - Number of attempts to execute to sample the negative edges.
    /// * graph_to_avoid: Option<&Graph> - The graph whose edges are to be avoided during the generation of false negatives,
    ///
    pub fn link_prediction<'a>(
        &'a self,
        idx: u64,
        batch_size: usize,
        method: &str,
        negative_samples: f64,
        avoid_false_negatives: bool,
        maximal_sampling_attempts: usize,
        graph_to_avoid: &'a Option<&Graph>,
    ) -> Result<
        impl ParallelIterator<Item = (usize, impl Iterator<Item = f64> + 'a, bool)> + 'a,
        String,
    > {
        // xor the random_state with a constant so that we have a good amount of 0s and 1s in the number
        // even with low values (this is needed becasue the random_state 0 make xorshift return always 0)
        let random_state = idx ^ SEED_XOR as u64;

        if negative_samples < 0.0 || !negative_samples.is_finite() {
            return Err("Negative sample must be a posive real value.".to_string());
        }

        if self.embedding.is_none() {
            return Err("Embedding object was not provided.".to_string());
        }

        let method = match EdgeEmbeddingMethods::new(method)? {
            EdgeEmbeddingMethods::Hadamard => |x1: f64, x2: f64| x1 * x2,
            EdgeEmbeddingMethods::Average => |x1: f64, x2: f64| (x1 + x2) / 2.0,
            EdgeEmbeddingMethods::Sum => |x1: f64, x2: f64| x1 + x2,
            EdgeEmbeddingMethods::L1 => |x1: f64, x2: f64| x1 - x2,
            EdgeEmbeddingMethods::AbsoluteL1 => |x1: f64, x2: f64| (x1 - x2).abs(),
            EdgeEmbeddingMethods::L2 => |x1: f64, x2: f64| (x1 - x2).powi(2),
        };

        // The number of negatives is given by computing their fraction of batchsize
        let negative_number: usize =
            ((batch_size as f64 / (1.0 + negative_samples)) * negative_samples) as usize;
        // All the remaining values then are positives
        let positive_number: usize = batch_size - negative_number;
        let graph_has_no_self_loops = !self.has_selfloops();

        let edges_number = self.get_edges_number() as u64;
        let nodes_number = self.get_nodes_number() as u64;
        let embedding_size = self.get_embedding_size()?;

        let mut rng: StdRng = SeedableRng::seed_from_u64(random_state);
        let random_values = gen_random_vec(batch_size, random_state);
        let mut indices: Vec<usize> = (0..batch_size).collect();
        indices.shuffle(&mut rng);

        match &self.embedding {
            Some(embedding) =>Ok((0..batch_size)
                .into_par_iter()
                .map(move |i| {
                    let mut sampled = random_values[i];
                    let (src, dst, label) = if i < positive_number{
                        let (src, dst) = self.get_edge_from_edge_id(sampled % edges_number);
                        (src, dst, true)
                    } else {
                        let mut attempts = 0;
                        loop {
                            if attempts > maximal_sampling_attempts {
                                panic!(format!(
                                    concat!(
                                        "Executed more than {} attempts to sample a negative edge.\n",
                                        "If your graph is so small that you see this error, you may want to consider ",
                                        "using one of the edge embedding transformer from the Embiggen library."
                                    ),
                                    maximal_sampling_attempts
                                ));
                            }
                            attempts += 1;
                            let random_src = sampled & 0xffffffff; // We need this to be an u64.
                            let random_dst = sampled >> 32; // We need this to be an u64.
                                                            // This technique is taken from:
                                                            // https://lemire.me/blog/2016/06/27/a-fast-alternative-to-the-modulo-reduction/
                            let src = ((random_src * nodes_number) >> 32) as NodeT;
                            let dst = ((random_dst * nodes_number) >> 32) as NodeT;
                            if avoid_false_negatives && self.has_edge(src, dst, None) {
                                sampled = xorshift(sampled);
                                continue;
                            }
                            if let Some(g) = &graph_to_avoid {
                                if g.has_edge(src, dst, None) {
                                    sampled = xorshift(sampled);
                                    continue;
                                }
                            }
                            if graph_has_no_self_loops && src == dst {
                                sampled = xorshift(sampled);
                                continue;
                            }
                            break (src, dst, false);
                        }
                    };
                    let src_embedding = &embedding[src as usize];
                    let dst_embedding = &embedding[dst as usize];
                    (
                        indices[i],
                        (0..embedding_size).map(move |i| {
                            let x1 = src_embedding[i];
                            let x2 = dst_embedding[i];
                            method(x1, x2)
                        }),
                        label
                    )
                })),
            None=>Err("Embedding object was not provided. Use the method 'set_embedding' to provide the embedding.".to_string())
        }
    }
}
