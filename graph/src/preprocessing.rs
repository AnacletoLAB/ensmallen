use super::*;
use hashbrown::HashMap;
use indicatif::{ProgressBar, ProgressIterator, ProgressStyle};
use rand::seq::SliceRandom;
use rand::thread_rng;
use rayon::prelude::*;
use vec_rand::gen_random_vec;
use vec_rand::xorshift::xorshift as rand_u64;

#[macro_export]
macro_rules! max {
    ($a: expr, $b: expr) => {
        if $a >= $b {
            $a
        } else {
            $b
        }
    };
}
#[macro_export]
macro_rules! min {
    ($a: expr, $b: expr) => {
        if $a < $b {
            $a
        } else {
            $b
        }
    };
}

fn skipgram_vector_length(walk_length: usize, window_size: usize) -> usize {
    (0..walk_length)
        .map(|i| {
            min!(walk_length, i + window_size + 1)
                - if i > window_size { i - window_size } else { 0 }
                - 1
        })
        .sum()
}

/// Preprocessing for ML algorithms on graph.
impl Graph {
    fn skipgram(
        &self,
        walk: &[usize],
        window_size: Option<usize>,
        negative_samples: Option<f64>,
        shuffle: Option<bool>,
        seed: u64,
    ) -> ((Vec<usize>, Vec<usize>), Vec<u8>) {
        let _negative_samples = negative_samples.unwrap_or(1.0);
        let _window_size = window_size.unwrap_or(4);
        let _shuffle = shuffle.unwrap_or(true);
        let _seed = seed ^ 0xBAD5eedBAD5eed11;

        let vector_length: usize = skipgram_vector_length(walk.len(), _window_size);
        // create the positive data
        let total_length = (vector_length as f64 * (1.0 + _negative_samples)) as usize;
        let mut words: Vec<NodeT> = Vec::with_capacity(total_length);
        let mut contexts: Vec<NodeT> = Vec::with_capacity(total_length);
        for (i, wi) in walk.iter().enumerate() {
            let window_start = if i > _window_size {
                i - _window_size
            } else {
                0
            };
            let window_end = min!(walk.len(), i + _window_size + 1);
            let delta = window_end - window_start - 1;
            words.extend_from_slice(&vec![*wi; delta][..]);
            contexts.extend_from_slice(&walk[window_start..i]);
            contexts.extend_from_slice(&walk[i + 1..window_end]);
        }
        let mut labels = vec![1; vector_length];

        if _negative_samples > 0.0 {
            // TODO! This thing can create false negatives!!
            // The issue was already present in the original TensorFlow implementation.
            let num_negatives = (vector_length as f64 * _negative_samples) as usize;
            let nodes_number = self.get_nodes_number();
            let words_neg: Vec<NodeT> = gen_random_vec(num_negatives, _seed)
                .iter()
                .map(|i| walk[(*i as NodeT) % walk.len()])
                .collect();
            let contexts_seed = rand_u64(_seed);
            let contexts_neg: Vec<NodeT> = gen_random_vec(num_negatives, contexts_seed)
                .iter()
                .map(|i| (*i as NodeT) % nodes_number)
                .collect();
            let labels_neg = vec![0; num_negatives];
            // merge positives and negatives labels
            words.extend(words_neg.iter());
            contexts.extend(contexts_neg.iter());
            labels.extend(labels_neg.iter());
        }
        if _shuffle {
            let mut indices: Vec<usize> = (0..words.len() as usize).collect();
            indices.shuffle(&mut thread_rng());
            words = indices.iter().map(|i| words[*i]).collect();
            contexts = indices.iter().map(|i| contexts[*i]).collect();
            labels = indices.iter().map(|i| labels[*i]).collect();
        }
        ((words, contexts), labels)
    }

    /// Return training batches for SkipGram model.
    ///
    /// The batch is composed of a tuple as the following:
    ///
    /// - (Central node, contextual node): the tuple of nodes
    /// - Label: boolean label representing if given nodes are actually in the
    ///     local context or are randomly sampled.
    ///
    /// # Arguments
    ///
    /// * idx: usize - Index of the batch
    /// * batch_size - Batch size, number of nodes to include in this iteration.
    /// * length: usize - Length of the random walks.
    /// * iterations: Option<usize> - Iterations on every node.
    /// * window_size: Option<usize> - Window size to consider for the sequences.
    /// * negative_samples: Option<f64>,
    ///     Factor of negative samples to use.
    ///     This is the factor for the number of non-context nodes to return
    ///     as negatives in any given batch.
    /// * shuffle: Option<bool>,
    ///     Wethever to shuffle the vectors on return.
    /// * min_length: Option<usize>,
    ///     Minimum length of the walks.
    ///     Walks which are smaller than the given minimum length will be
    ///     filtered and ignored.
    /// * return_weight: Option<ParamsT>,
    ///     Weight for the probability of exploitation.
    ///     This is the inverse of the p parameter.
    ///     The default value is 1.0.
    /// * explore_weight: Option<ParamsT>,
    ///     Weight for the probability of exploration.
    ///     This is the inverse of the q parameter.
    ///     The default value is 1.0.
    /// * change_node_type_weight: Option<ParamsT>,
    ///     Weight for changing the node type at every step of the walk.
    ///     The default value is 1.0.
    /// * change_edge_type_weight: Option<ParamsT>,
    ///     Weight for changing the edge type at every step of the walk.
    ///     The default value is 1.0.
    /// * graph_to_avoid: Option<&Graph>,
    ///     Graph to optionally avoid while generating.
    /// 
    /// # Implementation Details
    /// In order to correctly compute the positives and negatives, we would need to make a 
    /// k-th order transitive closure (which is equivalent to raising to the k-th the 
    /// adiacency matrix of the graph). This could result in multipying by k the number of
    /// edges in the graph. This k is the window size, therefore, this would greately increase
    /// the memory needed to correctly compute the associated graph. Or alternitivelty 
    /// we would need check the neighbors of each node in the walk, which will greately slow
    /// the computation.
    /// 
    /// Thererefore, **when we generate negatives, we just check that the edges is not present
    /// in the original graph**, this is just an approxiamation we do to make the problem
    /// tractable.
    pub fn skipgrams(
        &self,
        idx: usize,
        batch_size: usize,
        length: usize,
        iterations: Option<usize>,
        window_size: Option<usize>,
        negative_samples: Option<f64>,
        shuffle: Option<bool>,
        min_length: Option<usize>,
        return_weight: Option<ParamsT>,
        explore_weight: Option<ParamsT>,
        change_node_type_weight: Option<ParamsT>,
        change_edge_type_weight: Option<ParamsT>,
        graph_to_avoid: Option<&Graph>,
    ) -> Result<((Vec<usize>, Vec<usize>), Vec<u8>), String> {
        // check parameters validity
        let opt_idx = idx.checked_add(1);
        if opt_idx.is_none() {
            return Err(format!("The Index {} + 1 oveflow the u64.", idx));
        }

        let opt_end_node = opt_idx.unwrap().checked_mul(batch_size);
        if opt_end_node.is_none() {
            return Err(format!(
                "The Index+1 {} and batchsize {} when multiplied oveflow the u64.",
                idx + 1,
                batch_size
            ));
        }

        let start_node = idx * batch_size;
        let end_node = min!(self.get_nodes_number(), opt_end_node.unwrap());

        if start_node >= self.get_nodes_number() {
            return Err(format!(
                concat!(
                    "The given walk index {idx} with batch size {batch_size} ",
                    "is larger than the number of nodes {nodes} in the graph."
                ),
                idx = idx,
                batch_size = batch_size,
                nodes = self.get_nodes_number()
            ));
        }

        // do the walks and check the result
        let walks = self.walk(
            length,
            iterations,
            Some(start_node),
            Some(end_node),
            min_length,
            return_weight,
            explore_weight,
            change_node_type_weight,
            change_edge_type_weight,
            Some(false),
        )?;

        if walks.is_empty() {
            return Err(String::from(concat!(
                "An empty set of walks was generated.\n",
                "Consider changing the minimum length parameter, ",
                "increasing the batch size, the iterations and ",
                "checking the number of trap nodes in the graph."
            )));
        }
        // Setup the cumulative sum (this compute the index for the windows of each node,
        // this is only done to be able to parallelize)
        let mut cumsum: Vec<usize> = Vec::with_capacity(walks.len());
        let _window_size = window_size.unwrap_or(4);
        let _negative_samples = negative_samples.unwrap_or(1.0);

        if _negative_samples < 0.0 || !_negative_samples.is_finite() {
            return Err(String::from("Negative sample must be a posive real value."));
        }

        for i in 0..walks.len() {
            let new_value = (skipgram_vector_length(walks[i].len(), _window_size) as f64
                * (1.0 + _negative_samples)) as usize;
            cumsum.push(if i == 0 {
                new_value
            } else {
                cumsum[i - 1] + new_value
            });
        }

        let vector_length = cumsum[cumsum.len() - 1];
        // setup the result vectors
        let mut words = vec![0; vector_length];
        let mut contexts = vec![0; vector_length];
        let mut labels = vec![1; vector_length];
        {
            let mut words_indices = Vec::new();
            let mut remaining_words_array = words.as_mut_slice();
            let mut contexts_indices = Vec::new();
            let mut remaining_contexts_array = contexts.as_mut_slice();
            let mut labels_indices = Vec::new();
            let mut remaining_labels_array = labels.as_mut_slice();
            for i in 0..cumsum.len() {
                let start = if i == 0 { 0 } else { cumsum[i - 1] };
                let (words_left, words_right) =
                    remaining_words_array.split_at_mut(cumsum[i] - start);
                let (contexts_left, contexts_right) =
                    remaining_contexts_array.split_at_mut(cumsum[i] - start);
                let (labels_left, labels_right) =
                    remaining_labels_array.split_at_mut(cumsum[i] - start);
                words_indices.push(words_left);
                contexts_indices.push(contexts_left);
                labels_indices.push(labels_left);
                remaining_words_array = words_right;
                remaining_contexts_array = contexts_right;
                remaining_labels_array = labels_right;
            }

            walks
                .par_iter()
                .zip(words_indices.par_iter_mut())
                .zip(contexts_indices.par_iter_mut())
                .zip(labels_indices.par_iter_mut())
                .enumerate()
                .for_each(
                    |(i, (((walk, words_index), contexts_index), labels_index))| {
                        let ((_words, _contexts), _labels) = self.skipgram(
                            walk,
                            window_size,
                            Some(_negative_samples),
                            shuffle,
                            (idx + i) as u64,
                        );
                        (*words_index).copy_from_slice(&_words);
                        (*contexts_index).copy_from_slice(&_contexts);
                        (*labels_index).copy_from_slice(&_labels);
                    },
                );
        }

        // remove false negatives & edges that are inside of graph to avoid
        let false_negatives: Vec<bool> = words
            .par_iter()
            .zip(contexts.par_iter())
            .zip(labels.par_iter())
            .map(|((src, dst), label)| {
                // Keep the edge if:
                // it's positive and its in the graph 
                ((*label == 1) || self.has_edge(*src, *dst)) &&
                // or it isn't in the graph to avoid
                if let Some(gta) = &graph_to_avoid {
                    !gta.has_edge(*src, *dst)
                }
                else {
                    true
                }
            })
            .collect();
            
        words = false_negatives
            .par_iter()
            .zip(words.par_iter())
            .filter(|(false_negative, _)| **false_negative)
            .map(|(_, src)| *src)
            .collect();
        contexts = false_negatives
            .par_iter()
            .zip(contexts.par_iter())
            .filter(|(false_negative, _)| **false_negative)
            .map(|(_, src)| *src)
            .collect();
        labels = false_negatives
            .par_iter()
            .zip(labels.par_iter())
            .filter(|(false_negative, _)| **false_negative)
            .map(|(_, src)| *src)
            .collect();

        Ok(((words, contexts), labels))
    }

    /// Return training batches for CBOW model.
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
    /// * idx: usize - Index of the batch
    /// * batch_size - Batch size, number of nodes to include in this iteration.
    /// * length: usize - Length of the random walks.
    /// * iterations: Option<usize> - Iterations on every node.
    /// * window_size: Option<usize> - Window size to consider for the sequences.
    /// * shuffle: Option<bool>,
    ///     Wethever to shuffle the vectors on return.
    /// * min_length: Option<usize>,
    ///     Minimum length of the walks.
    ///     Walks which are smaller than the given minimum length will be
    ///     filtered and ignored.
    /// * return_weight: Option<ParamsT>,
    ///     Weight for the probability of exploitation.
    ///     This is the inverse of the p parameter.
    ///     The default value is 1.0.
    /// * explore_weight: Option<ParamsT>,
    ///     Weight for the probability of exploration.
    ///     This is the inverse of the q parameter.
    ///     The default value is 1.0.
    /// * change_node_type_weight: Option<ParamsT>,
    ///     Weight for changing the node type at every step of the walk.
    ///     The default value is 1.0.
    /// * change_edge_type_weight: Option<ParamsT>,
    ///     Weight for changing the edge type at every step of the walk.
    ///     The default value is 1.0.
    ///
    pub fn cbow(
        &self,
        idx: usize,
        batch_size: usize,
        length: usize,
        iterations: Option<usize>,
        window_size: Option<usize>,
        shuffle: Option<bool>,
        min_length: Option<usize>,
        return_weight: Option<ParamsT>,
        explore_weight: Option<ParamsT>,
        change_node_type_weight: Option<ParamsT>,
        change_edge_type_weight: Option<ParamsT>
    ) -> Result<(Vec<Vec<usize>>, Vec<usize>), String> {
        // check parameters validity
        let opt_idx = idx.checked_add(1);
        if opt_idx.is_none() {
            return Err(format!("The Index {} + 1 oveflow the u64.", idx));
        }

        let opt_end_node = opt_idx.unwrap().checked_mul(batch_size);
        if opt_end_node.is_none() {
            return Err(format!(
                "The Index+1 {} and batchsize {} when multiplied oveflow the u64.",
                idx + 1,
                batch_size
            ));
        }

        let start_node = idx * batch_size;
        let end_node = min!(self.get_nodes_number(), opt_end_node.unwrap());

        if start_node >= self.get_nodes_number() {
            return Err(format!(
                concat!(
                    "The given walk index {idx} with batch size {batch_size} ",
                    "is larger than the number of nodes {nodes} in the graph."
                ),
                idx = idx,
                batch_size = batch_size,
                nodes = self.get_nodes_number()
            ));
        }

        // do the walks and check the result
        let walks = self.walk(
            length,
            iterations,
            Some(start_node),
            Some(end_node),
            min_length,
            return_weight,
            explore_weight,
            change_node_type_weight,
            change_edge_type_weight,
            Some(false),
        )?;

        if walks.is_empty() {
            return Err(String::from(concat!(
                "An empty set of walks was generated.\n",
                "Consider changing the minimum length parameter, ",
                "increasing the batch size, the iterations and ",
                "checking the number of trap nodes in the graph."
            )));
        }

        let _window_size = window_size.unwrap_or(4);
        let _shuffle:bool = shuffle.unwrap_or(true);
        let context_length = _window_size*2;

        let mut walks_centers:Vec<Vec<NodeT>> = walks.par_iter().map(
            |walk|
            vec![0; walk.len()]
        ).collect();
        let mut walks_filters:Vec<Vec<bool>> = walks.par_iter().map(
            |walk|
            vec![false; walk.len()]
        ).collect();
        let mut contexts:Vec<Vec<NodeT>> = walks
            .par_iter()
            .zip(walks_centers.par_iter_mut())
            .zip(walks_filters.par_iter_mut())
            .map(
                |((walk, centers), filters)|
                walk.iter().enumerate().zip(centers.iter_mut()).map(
                    |((i, word), center)| {
                        let start = if i<=_window_size{
                            0
                        } else {
                            i - _window_size
                        };
                        let end = min!(walk.len(), i + _window_size);
                        *center = *word;        
                        let context:Vec<NodeT> = walk[start..end].to_vec();
                        context
                    }
                ).zip(filters.iter_mut()).filter_map(
                    |(context, filter)| if context.len() == context_length{
                        *filter = true;
                        Some(context)
                    } else {
                        *filter = false;
                        None
                    }
                ).collect::<Vec<Vec<NodeT>>>()
        ).flatten().collect();

        let filters:Vec<bool> = walks_filters
            .iter().flatten().cloned().collect();

        let mut centers:Vec<NodeT> = walks_centers
            .iter().flatten().cloned()
            .zip(filters.iter())
            .filter_map(
                |(center, filter)|
                if *filter {
                    Some(center)
                } else {
                    None
                }
            )
            .collect();

        if _shuffle {
            let mut indices: Vec<usize> = (0..centers.len() as usize).collect();
            indices.shuffle(&mut thread_rng());

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
    /// * length: usize - Length of the random walks.
    /// * iterations: Option<usize> - Iterations on every node.
    /// * window_size: Option<usize> - Window size to consider for the sequences.
    /// * min_length: Option<usize>,
    ///     Minimum length of the walks.
    ///     Walks which are smaller than the given minimum length will be
    ///     filtered and ignored.
    /// * return_weight: Option<ParamsT>,
    ///     Weight for the probability of exploitation.
    ///     This is the inverse of the p parameter.
    ///     The default value is 1.0.
    /// * explore_weight: Option<ParamsT>,
    ///     Weight for the probability of exploration.
    ///     This is the inverse of the q parameter.
    ///     The default value is 1.0.
    /// * change_node_type_weight: Option<ParamsT>,
    ///     Weight for changing the node type at every step of the walk.
    ///     The default value is 1.0.
    /// * change_edge_type_weight: Option<ParamsT>,
    ///     Weight for changing the edge type at every step of the walk.
    ///     The default value is 1.0.
    /// * verbose: Option<bool>,
    ///     Wethever to show the progress bars.
    ///     The default behaviour is false.
    ///     
    pub fn cooccurence_matrix(
        &self,
        length: usize,
        iterations: Option<usize>,
        window_size: Option<usize>,
        min_length: Option<usize>,
        return_weight: Option<ParamsT>,
        explore_weight: Option<ParamsT>,
        change_node_type_weight: Option<ParamsT>,
        change_edge_type_weight: Option<ParamsT>,
        verbose: Option<bool>,
    ) -> Result<(Vec<NodeT>, Vec<NodeT>, Vec<f64>), String> {
        let _verbose = verbose.unwrap_or(false);
        let _window_size = window_size.unwrap_or(4);

        // TODO: if in Rust is possible to return a generator, we could
        // iterate directly on the walks without storing them into an array.
        let walks = self.walk(
            length,
            iterations,
            None,
            None,
            min_length,
            return_weight,
            explore_weight,
            change_node_type_weight,
            change_edge_type_weight,
            Some(_verbose),
        )?;

        let mut cooccurence_matrix: HashMap<(NodeT, NodeT), f64> = HashMap::new();
        let pb1 = if _verbose {
            let pb1 = ProgressBar::new(walks.len() as u64);
            pb1.set_style(ProgressStyle::default_bar().template(
                "Computing cooccurrence mapping {spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] ({pos}/{len}, ETA {eta})",
            ));
            pb1.set_draw_delta(walks.len() as u64 / 100);
            pb1
        } else {
            ProgressBar::hidden()
        };

        for i in (0..walks.len()).progress_with(pb1) {
            let walk = &walks[i];
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

    /// Returns triple with source nodes, destination nodes and labels for training model for link prediction.
    ///
    /// # Arguments
    ///
    /// * idx:u64 - The index of the batch to generate, behaves like a random seed,
    /// * batch_size:usize - The maximal size of the batch to generate,
    /// * negative_samples: Option<f64> - The component of netagetive samples to use,
    /// * graph_to_avoid: Option<&Graph> - The graph whose edges are to be avoided during the generation of false negatives,
    /// * shuffle: Option<bool> - Whetever to shuffle or not the batch.
    ///
    pub fn link_prediction(
        &self,
        idx: u64,
        batch_size: usize,
        negative_samples: Option<f64>,
        graph_to_avoid: Option<&Graph>,
        shuffle: Option<bool>,
    ) -> Result<(Vec<NodeT>, Vec<NodeT>, Vec<u8>), String> {
        let _negative_samples = negative_samples.unwrap_or(1.0);
        let _shuffle = shuffle.unwrap_or(true);
        let seed = idx ^ 0xBAD5eedBAD5eed11;
        // The number of negatives is given by computing their fraction of batchsize
        let negatives_number: usize =
            ((batch_size as f64 / (1.0 + _negative_samples)) * _negative_samples) as usize;
        // All the remaining values then are positives
        let positives_number: usize = batch_size - negatives_number;

        if _negative_samples < 0.0 || !_negative_samples.is_finite() {
            return Err(String::from("Negative sample must be a posive real value."));
        }

        let edges_number = self.get_edges_number() as u64;
        let positives: Vec<(NodeT, NodeT)> = gen_random_vec(positives_number, seed)
            .into_par_iter()
            .map(|random_value| (random_value % edges_number) as EdgeT)
            .map(|edge| (self.sources[edge], self.destinations[edge]))
            .collect();

        let negatives: Vec<(NodeT, NodeT)> = if negatives_number != 0 {
            let sources_seed = rand_u64(seed);
            let destinations_seed = rand_u64(sources_seed);
            gen_random_vec(negatives_number, sources_seed)
                .into_par_iter()
                .zip(gen_random_vec(negatives_number, destinations_seed).into_par_iter())
                .map(|(random_src, random_dst)| {
                    (
                        self.sources[(random_src % edges_number) as EdgeT],
                        self.destinations[(random_dst % edges_number) as EdgeT],
                    )
                })
                .filter(|(src, dst)| {
                    !(self.has_edge(*src, *dst)
                        || if let Some(g) = &graph_to_avoid {
                            g.has_edge(*src, *dst)
                        } else {
                            false
                        })
                })
                .collect()
        } else {
            vec![]
        };

        let mut labels: Vec<u8> = vec![1 as u8; positives.len()];
        labels.extend(vec![0 as u8; negatives.len()]);

        let mut edges: Vec<(NodeT, NodeT)> = positives;
        edges.extend(negatives);

        if _shuffle {
            let mut indices: Vec<usize> = (0..edges.len() as usize).collect();
            indices.shuffle(&mut thread_rng());

            labels = indices.par_iter().map(|i| labels[*i]).collect();
            edges = indices.par_iter().map(|i| edges[*i]).collect();
        }

        let sources: Vec<NodeT> = edges.par_iter().map(|(src, _)| *src).collect();
        let destinations: Vec<NodeT> = edges.par_iter().map(|(_, dst)| *dst).collect();

        Ok((sources, destinations, labels))
    }
}
