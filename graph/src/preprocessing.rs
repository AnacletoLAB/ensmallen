use super::*;
use rayon::prelude::*;
extern crate rand;
use rand::Rng;  
use rand::seq::SliceRandom;
use rand::thread_rng;
use indicatif::{ProgressBar, ProgressIterator, ProgressStyle};
use hashbrown::{HashMap};

fn gen_random_usize_vec(num: usize, max: usize) -> Vec<usize> {
    // TODO! substitute with xorshiro
    let mut rng = rand::thread_rng();
    let vals: Vec<usize> = (0..num).map(|_| rng.gen_range(0, max)).collect();
    vals
}

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

fn skipgram_vector_length(walk_length:usize, window_size:usize)->usize{
    (0..walk_length)
        .map(|i| min!(walk_length, i + window_size + 1) - if i>window_size{i - window_size}else{0} - 1)
        .sum()
}

fn skipgram(
    walk: &[usize],
    vocabulary_size: usize,
    window_size: Option<usize>,
    negative_samples: Option<f64>,
    shuffle: Option<bool>,
) -> (
    (
        Vec<usize>,
        Vec<usize>
    ),
    Vec<u8>   
){
    let _negative_samples = negative_samples.unwrap_or(1.0);
    let _window_size = window_size.unwrap_or(4);
    let _shuffle = shuffle.unwrap_or(true);

    let vector_length: usize = skipgram_vector_length(walk.len(), _window_size);

    // create the positive data
    let mut words: Vec<NodeT> = Vec::with_capacity(vector_length*(1+_negative_samples as usize));
    let mut contexts: Vec<NodeT> = Vec::with_capacity(vector_length*(1+_negative_samples as usize));
    
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

    // create negative data
    // In this implementation, negative samples ARE MANDATORY.
    
    // TODO! This thing can create false negatives!!
    // The issue was already present in the original TensorFlow implementation.
    let num_negatives = vector_length*(_negative_samples as usize);
    let words_neg: Vec<NodeT> = gen_random_usize_vec(num_negatives, walk.len() - 1)
        .iter()
        .map(|i| walk[*i])
        .collect();
    let contexts_neg: Vec<NodeT> = gen_random_usize_vec(
        num_negatives,
        vocabulary_size - 1
    );
    let labels_neg = vec![0; vector_length];

    // merge positives and negatives labels
    words.extend(words_neg.iter());
    contexts.extend(contexts_neg.iter());
    labels.extend(labels_neg.iter());

    if _shuffle {
        let mut indices: Vec<usize> = (0..words.len() as usize).collect();
        indices.shuffle(&mut thread_rng());

        words = indices.iter().map(|i| words[*i]).collect();
        contexts = indices.iter().map(|i| contexts[*i]).collect();
        labels = indices.iter().map(|i| labels[*i]).collect();
    }

    (
        (words,
        contexts),
        labels
    )
}

/// Preprocessing for ML algorithms on graph.
impl Graph {
    
    // TODO docstring
    pub fn skipgrams(
        &self,
        idx: usize,
        batch_size: usize,
        length: usize,
        window_size: Option<usize>,
        negative_samples: Option<f64>,
        shuffle: Option<bool>,
        min_length: Option<usize>,
        return_weight: Option<ParamsT>,
        explore_weight: Option<ParamsT>,
        change_node_type_weight: Option<ParamsT>,
        change_edge_type_weight: Option<ParamsT>
    ) -> Result<((Vec<usize>,Vec<usize>),Vec<u8>), String>{
        let walks = self.walk(
            length,
            None,
            Some(idx*batch_size),
            Some(min!(self.get_nodes_number(), (idx+1)*batch_size)),
            min_length,
            return_weight,
            explore_weight,
            change_node_type_weight,
            change_edge_type_weight,
            Some(false)
        ).unwrap();

        let mut cumsum:Vec<usize> = Vec::with_capacity(walks.len());
        let _window_size = window_size.unwrap_or(4);
        let _negative_samples = negative_samples.unwrap_or(1.0);
        
        for i in 0..walks.len(){
            let new_value = skipgram_vector_length(walks[i].len(), _window_size) * (1 + _negative_samples as usize);
            cumsum.push(
                if i==0 {
                    new_value
                } else {
                    cumsum[i-1] + new_value
                }
            );
        }
        
        let vector_length = cumsum[cumsum.len()-1];

        let mut words = vec![0; vector_length];
        let mut contexts = vec![0; vector_length];
        let mut labels = vec![1; vector_length];

        walks.iter().enumerate().for_each(|(i, walk)|{
            let ((_words, _contexts), _labels) = skipgram(
                walk,
                self.get_nodes_number(),
                window_size,
                Some(_negative_samples),
                shuffle
            );
            let start = if i==0{
                0
            } else {
                cumsum[i-1]
            };
            words[start..cumsum[i]].copy_from_slice(&_words);
            contexts[start..cumsum[i]].copy_from_slice(&_contexts);
            labels[start..cumsum[i]].copy_from_slice(&_labels);
        });

        Ok(((words, contexts), labels))
    }

    // TODO docstring
    pub fn cooccurence_matrix(
        &self,
        length: usize,
        window_size: Option<usize>,
        iterations: Option<usize>,
        min_length: Option<usize>,
        return_weight: Option<ParamsT>,
        explore_weight: Option<ParamsT>,
        change_node_type_weight: Option<ParamsT>,
        change_edge_type_weight: Option<ParamsT>,
        verbose: Option<bool>
    ) -> Result<(Vec<NodeT>, Vec<NodeT>, Vec<f64>), String> {

        let _verbose = verbose.unwrap_or(true);
        let _window_size = window_size.unwrap_or(4);

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
            Some(_verbose)
        ).unwrap();

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

        for i in (0..walks.len()).progress_with(pb1){
            let walk = &walks[i];
            let walk_length = walk.len();
            for (central_index, &central_word_id) in walk.iter().enumerate(){
                for distance in 1..1+_window_size{
                    if central_index + distance >= walk_length{
                        break;
                    }
                    let context_id = walk[central_index + distance];
                    if central_word_id < context_id{
                        *cooccurence_matrix.entry((central_word_id, context_id)).or_insert(0.0) += 1.0 / distance as f64;
                    } else {
                        *cooccurence_matrix.entry((context_id, central_word_id)).or_insert(0.0) += 1.0 / distance as f64;
                    }
                }
            }
        }

        let elements = cooccurence_matrix.len()*2;
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
            .for_each(|(i, ((word, context), frequency))|{
            let (k, j) = (i*2, i*2+1);
            if *frequency > max_frequency{
                max_frequency = *frequency;
            }
            words[k] = *word;
            words[j] = words[k];
            contexts[k] = *context;
            contexts[j] = contexts[k];
            frequencies[k] = *frequency;
            frequencies[j] = frequencies[j];
        });

        frequencies.par_iter_mut().for_each(|frequency| {*frequency/=max_frequency});

        Ok((words, contexts, frequencies))
    }
}