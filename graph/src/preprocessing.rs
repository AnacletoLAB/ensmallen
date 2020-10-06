use super::*;
use indicatif::{ProgressBar, ProgressIterator, ProgressStyle};
use rand::rngs::StdRng;
use rand::seq::SliceRandom;
use rand::SeedableRng;
use rayon::prelude::*;
use std::collections::HashMap;
use vec_rand::gen_random_vec;
use vec_rand::xorshift::xorshift as rand_u64;

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
pub fn word2vec(
    sequences: Vec<Vec<NodeT>>,
    window_size: usize,
) -> Result<(Vec<Vec<NodeT>>, Vec<NodeT>), String> {
    let context_length = window_size.checked_mul(2).ok_or(
        "The given window size is too big, using this would result in an overflowing of a u64.",
    )?;

    Ok(sequences
        .par_iter()
        .flat_map(|sequence| {
            sequence
                .iter()
                .enumerate()
                .filter_map(|(i, word)| {
                    let start = if i <= window_size { 0 } else { i - window_size };
                    let end = min!(sequence.len(), i + window_size);
                    if end - start == context_length {
                        Some((sequence[start..end].to_vec(), *word))
                    } else {
                        None
                    }
                })
                .collect::<Vec<(Vec<NodeT>, NodeT)>>()
        })
        .unzip())
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
    sequences: Vec<Vec<NodeT>>,
    window_size: Option<usize>,
    verbose: Option<bool>,
) -> Result<(Words, Words, Frequencies), String> {
    let _verbose = verbose.unwrap_or(false);
    let _window_size = window_size.unwrap_or(4);

    let mut cooccurence_matrix: HashMap<(NodeT, NodeT), f64> = HashMap::new();
    let pb1 = if _verbose {
        let pb1 = ProgressBar::new(sequences.len() as u64);
        pb1.set_style(ProgressStyle::default_bar().template(
            "Computing cooccurrence mapping {spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] ({pos}/{len}, ETA {eta})",
        ));
        pb1.set_draw_delta(sequences.len() as u64 / 100);
        pb1
    } else {
        ProgressBar::hidden()
    };

    for i in (0..sequences.len()).progress_with(pb1) {
        let walk = &sequences[i];
        let walk_length = walk.len();
        for (central_index, &central_word_id) in walk.iter().enumerate() {
            for distance in 1..1 + _window_size {
                if central_index + distance >= walk_length {
                    break;
                }
                let context_id = walk[central_index + distance];
                if central_word_id < context_id {
                    *cooccurence_matrix
                        .entry((central_word_id as NodeT, context_id as NodeT))
                        .or_insert(0.0) += 1.0 / distance as f64;
                } else {
                    *cooccurence_matrix
                        .entry((context_id as NodeT, central_word_id as NodeT))
                        .or_insert(0.0) += 1.0 / distance as f64;
                }
            }
        }
    }

    let elements = cooccurence_matrix.len() * 2;
    let mut max_frequency = 0.0;
    let mut words: Vec<NodeT> = vec![0; elements];
    let mut contexts: Vec<NodeT> = vec![0; elements];
    let mut frequencies: Vec<f64> = vec![0.0; elements];
    let pb2 = if _verbose {
        let pb2 = ProgressBar::new(cooccurence_matrix.len() as u64);
        pb2.set_style(ProgressStyle::default_bar().template(
            "Converting mapping into CSR matrix {spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] ({pos}/{len}, ETA {eta})",
        ));
        pb2.set_draw_delta(cooccurence_matrix.len() as u64 / 100);
        pb2
    } else {
        ProgressBar::hidden()
    };

    cooccurence_matrix
        .iter()
        .progress_with(pb2)
        .enumerate()
        .for_each(|(i, ((word, context), frequency))| {
            let (k, j) = (i * 2, i * 2 + 1);
            if *frequency > max_frequency {
                max_frequency = *frequency;
            }
            words[k] = *word;
            words[j] = words[k];
            contexts[k] = *context;
            contexts[j] = contexts[k];
            frequencies[k] = *frequency;
            frequencies[j] = frequencies[k];
        });

    frequencies
        .par_iter_mut()
        .for_each(|frequency| *frequency /= max_frequency);

    Ok((words, contexts, frequencies))
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
    pub fn node2vec(
        &self,
        walk_parameters: &WalksParameters,
        quantity: NodeT,
        window_size: usize,
    ) -> Result<(Contexts, Words), String> {
        // do the walks and check the result
        let walks = self.random_walks(quantity, walk_parameters)?;

        if walks.is_empty() {
            return Err(concat!(
                "In the current graph, with the given parameters, no walk could ",
                "be performed which is above the given min-length"
            )
            .to_string());
        }

        word2vec(walks, window_size)
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
    pub fn cooccurence_matrix(
        &self,
        walks_parameters: &WalksParameters,
        window_size: Option<usize>,
        verbose: Option<bool>,
    ) -> Result<(Words, Words, Frequencies), String> {
        let walks = self.complete_walks(walks_parameters)?;

        if walks.is_empty() {
            return Err(concat!(
                "In the current graph, with the given parameters, no walk could ",
                "be performed which is above the given min-length"
            )
            .to_string());
        }

        cooccurence_matrix(walks, window_size, verbose)
    }

    /// Returns triple with source nodes, destination nodes and labels for training model for link prediction.
    ///
    /// # Arguments
    ///
    /// * idx:u64 - The index of the batch to generate, behaves like a random seed,
    /// * batch_size:usize - The maximal size of the batch to generate,
    /// * negative_samples: Option<f64> - The component of netagetive samples to use,
    /// * graph_to_avoid: Option<&Graph> - The graph whose edges are to be avoided during the generation of false negatives,
    ///
    pub fn link_prediction(
        &self,
        idx: u64,
        batch_size: usize,
        negative_samples: Option<f64>,
        graph_to_avoid: Option<&Graph>,
        avoid_self_loops: Option<bool>,
    ) -> Result<(Contexts, Vec<u8>), String> {
        // xor the seed with a constant so that we have a good amount of 0s and 1s in the number
        // even with low values (this is needed becasue the seed 0 make xorshift return always 0)
        let seed = idx ^ SEED_XOR as u64;
        // extract options
        let _negative_samples = negative_samples.unwrap_or(1.0);

        if _negative_samples < 0.0 || !_negative_samples.is_finite() {
            return Err(String::from("Negative sample must be a posive real value."));
        }

        let _avoid_self_loops = avoid_self_loops.unwrap_or(false);
        // The number of negatives is given by computing their fraction of batchsize
        let negatives_number: usize =
            ((batch_size as f64 / (1.0 + _negative_samples)) * _negative_samples) as usize;
        // All the remaining values then are positives
        let positives_number: usize = batch_size - negatives_number;

        let edges_number = self.get_edges_number() as u64;
        let nodes_number = self.get_nodes_number() as u64;
        // generate a random vec of u64s and use them as indices
        let positives: Vec<Vec<NodeT>> = gen_random_vec(positives_number, seed)
            .into_par_iter()
            // to extract the random edges
            .filter_map(|random_value| {
                let edge_id = (random_value % edges_number) as EdgeT;
                let (src, dst) = self.get_edge_from_edge_id(edge_id);
                if !_avoid_self_loops || src != dst {
                    Some(vec![src, dst])
                } else {
                    None
                }
            })
            .collect();

        // generate the negatives
        let negatives: Vec<Vec<NodeT>> = if negatives_number == 0 {
            // if the number of negatives is 0 then just return an empty array
            vec![]
        } else {
            // generate two seeds for reproducibility porpouses
            let sources_seed = rand_u64(seed);
            let destinations_seed = rand_u64(sources_seed);
            // generate the random edge-sources
            gen_random_vec(negatives_number, sources_seed)
                .into_par_iter()
                // generate the random edge-destinations
                .zip(gen_random_vec(negatives_number, destinations_seed).into_par_iter())
                // convert them to plain (src, dst)
                .map(|(random_src, random_dst)| {
                    (
                        (random_src % nodes_number) as NodeT,
                        (random_dst % nodes_number) as NodeT,
                    )
                })
                // filter away the negatives that are:
                .filter(|(src, dst)| {
                    !(
                        // false negatives or
                        self.has_edge(*src, *dst, None)
                        // are in the graph to avoid
                        || if let Some(g) = &graph_to_avoid {
                            g.has_edge(*src, *dst, None)
                        } else {
                            false
                        }
                        // If it's a self loop and the flag is set
                        || (
                            _avoid_self_loops && src == dst
                        )
                    )
                })
                .map(|(src, dst)| vec![src, dst])
                .collect()
        };
        // create the corresponing labels
        let mut labels: Vec<u8> = vec![1 as u8; positives.len()];
        labels.extend(vec![0 as u8; negatives.len()]);
        // concat the two vectors of edges
        let mut edges: Vec<Vec<NodeT>> = positives;
        edges.extend(negatives);

        let mut indices: Vec<usize> = (0..labels.len() as usize).collect();
        let mut rng: StdRng = SeedableRng::seed_from_u64(seed);
        indices.shuffle(&mut rng);

        labels = indices.par_iter().map(|i| labels[*i]).collect();
        edges = indices.par_iter().map(|i| edges[*i].clone()).collect();

        Ok((edges, labels))
    }
}
