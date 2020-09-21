use super::*;
use std::collections::HashMap;
use indicatif::{ProgressBar, ProgressIterator, ProgressStyle};
use rand::rngs::StdRng;
use rand::seq::SliceRandom;
use rand::SeedableRng;
use rayon::prelude::*;
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
/// * window_size: Option<usize> - Window size to consider for the sequences.
/// * shuffle: Option<bool> - Wethever to shuffle the vectors on return.
/// * seed: usize - The seed for reproducibility.
///
pub fn word2vec(
    sequences: Vec<Vec<usize>>,
    window_size: Option<usize>,
    shuffle: Option<bool>,
    seed: usize,
) -> Result<(Vec<Vec<usize>>, Vec<usize>), String> {
    let _window_size = window_size.unwrap_or(4);
    let _shuffle: bool = shuffle.unwrap_or(true);
    let context_length = _window_size.checked_mul(2).ok_or(
        "The given window size is too big, using this would result in an overflowing of a u64.",
    )?;

    let mut sequences_centers: Vec<Vec<usize>> = sequences
        .par_iter()
        .map(|sequence| vec![0; sequence.len()])
        .collect();
    let mut sequences_filters: Vec<Vec<bool>> = sequences
        .par_iter()
        .map(|sequence| vec![false; sequence.len()])
        .collect();
    let mut contexts: Vec<Vec<usize>> = sequences
        .par_iter()
        .zip(sequences_centers.par_iter_mut())
        .zip(sequences_filters.par_iter_mut())
        .map(|((sequence, centers), filters)| {
            sequence
                .iter()
                .enumerate()
                .zip(centers.iter_mut())
                .map(|((i, word), center)| {
                    let start = if i <= _window_size {
                        0
                    } else {
                        i - _window_size
                    };
                    let end = min!(sequence.len(), i + _window_size);
                    *center = *word;
                    let context: Vec<usize> = sequence[start..end].to_vec();
                    context
                })
                .zip(filters.iter_mut())
                .filter_map(|(context, filter)| {
                    if context.len() == context_length {
                        *filter = true;
                        Some(context)
                    } else {
                        *filter = false;
                        None
                    }
                })
                .collect::<Vec<Vec<usize>>>()
        })
        .flatten()
        .collect();

    let filters: Vec<bool> = sequences_filters.iter().flatten().cloned().collect();

    let mut centers: Vec<usize> = sequences_centers
        .iter()
        .flatten()
        .cloned()
        .zip(filters.iter())
        .filter_map(|(center, filter)| if *filter { Some(center) } else { None })
        .collect();

    if _shuffle {
        let mut indices: Vec<usize> = (0..centers.len() as usize).collect();
        let mut rng: StdRng = SeedableRng::seed_from_u64(seed as u64);
        indices.shuffle(&mut rng);

        contexts = indices.par_iter().map(|i| contexts[*i].clone()).collect();
        centers = indices.par_iter().map(|i| centers[*i]).collect();
    }

    Ok((contexts, centers))
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
    sequences: Vec<Vec<usize>>,
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
                        .entry((central_word_id, context_id))
                        .or_insert(0.0) += 1.0 / distance as f64;
                } else {
                    *cooccurence_matrix
                        .entry((context_id, central_word_id))
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
    /// * window_size: Option<usize> - Window size to consider for the sequences.
    /// * shuffle: Option<bool> - Wethever to shuffle the vectors on return.
    /// * idx: usize - The seed for reproducibility.
    ///
    pub fn node2vec(
        &self,
        walk_parameters: &WalksParameters,
        window_size: Option<usize>,
        shuffle: Option<bool>,
        seed: usize,
    ) -> Result<(Contexts, Words), String> {
        // do the walks and check the result
        let walks = self.walk(walk_parameters)?;

        if walks.is_empty() {
            return Err(concat!(
                "In the current graph, with the given parameters, no walk could ",
                "be performed which is above the given min-length"
            )
            .to_string());
        }

        word2vec(walks, window_size, shuffle, seed)
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
        let walks = self.walk(walks_parameters)?;

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
        // generate a random vec of u64s and use them as indices
        let positives: Vec<Vec<NodeT>> = gen_random_vec(positives_number, seed)
            .into_par_iter()
            // to extract the random edges
            .map(|random_value| (random_value % edges_number) as EdgeT)
            .map(|edge| vec![self.sources[edge], self.destinations[edge]])
            // filter away the self_loops if the flag is set
            .filter(|edge| !_avoid_self_loops || edge[0] != edge[1])
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
                        self.sources[(random_src % edges_number) as EdgeT],
                        self.destinations[(random_dst % edges_number) as EdgeT],
                    )
                })
                // filter away the negatives that are:
                .filter(|(src, dst)| {
                    !(
                        // false negatives or
                        self.has_edge(*src, *dst)
                        // are in the graph to avoid
                        || if let Some(g) = &graph_to_avoid {
                            g.has_edge(*src, *dst)
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
