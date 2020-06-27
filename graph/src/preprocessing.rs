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

/// Properties and measurements of the graph
impl Graph {
    pub fn skipgram_preprocessing(
        walk: &Vec<NodeT>,
        vocabulary_size: usize,
        window_size: usize,
        negative_samples: f64,
        shuffle: bool,
    ) -> (
        Vec<usize>,
        Vec<usize>,
        Vec<u8>   
    ){
        let walk_len = walk.len();
        let vector_length: usize = (0..walk_len)
            .into_iter()
            .map(|i| min!(walk_len, i + window_size + 1) - max!(0, i - window_size) - 1)
            .sum();

        // create the positive data
        let mut words: Vec<NodeT> = Vec::with_capacity(vector_length);
        let mut contexts: Vec<NodeT> = Vec::with_capacity(vector_length);
        
        for (i, wi) in walk.iter().enumerate() {
            let window_start = if i > window_size {
                i - window_size
            } else {
                0
            };
            let window_end = min!(walk_len, i + window_size + 1); 
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
        let num_negatives = (words.len() as f64 * negative_samples) as usize;
        let words_neg: Vec<NodeT> = gen_random_usize_vec(num_negatives, walk_len - 1)
            .iter()
            .map(|i| walk[*i])
            .collect();
        let contexts_neg: Vec<NodeT> = gen_random_usize_vec(num_negatives, vocabulary_size - 1);
        let labels_neg = vec![0; vector_length];

        // merge positives and negatives labels
        words.extend(words_neg.iter());
        contexts.extend(contexts_neg.iter());
        labels.extend(labels_neg.iter());

        if shuffle {
            let mut indices: Vec<usize> = (0..words.len() as usize).into_iter().collect();
            indices.shuffle(&mut thread_rng());

            words = indices.iter().map(|i| words[*i]).collect();
            contexts = indices.iter().map(|i| contexts[*i]).collect();
            labels = indices.iter().map(|i| labels[*i]).collect();
        }

        (
            words,
            contexts,
            labels
        )
    }

    pub fn cooccurence_matrix(
        &self,
        window_size: usize,
        iterations: usize,
        length: usize,
        min_length: Option<usize>,
        return_weight: Option<ParamsT>,
        explore_weight: Option<ParamsT>,
        change_node_type_weight: Option<ParamsT>,
        change_edge_type_weight: Option<ParamsT>,
    ) -> (Vec<NodeT>, Vec<NodeT>, Vec<f64>) {
        let walks = self.walk(
            iterations,
            length,
            min_length,
            return_weight,
            explore_weight,
            change_node_type_weight,
            change_edge_type_weight
        ).unwrap();
        let mut cooccurence_matrix: HashMap<(NodeT, NodeT), f64> = HashMap::new();
        let pb1 = ProgressBar::new(walks.len() as u64);
        pb1.set_style(ProgressStyle::default_bar().template(
            "Computing cooccurrence matrix {spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] ({pos}/{len}, ETA {eta})",
        ));

        // TODO: Add a TQDM-like loading bar.
        for i in (0..walks.len()).progress_with(pb1){
            let walk = &walks[i];
            let walk_length = walk.len();
            for (central_index, &central_word_id) in walk.iter().enumerate(){
                for distance in 1..1+window_size{
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
        let pb2 = ProgressBar::new(walks.len() as u64);
        pb2.set_style(ProgressStyle::default_bar().template(
            "Converting cooccurrence matrix {spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] ({pos}/{len}, ETA {eta})",
        ));

        cooccurence_matrix
            .iter()
            .zip((0..cooccurence_matrix.len()).progress_with(pb2))
            .for_each(|(((word, context), frequency), i)|{
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

        (words, contexts, frequencies)
    }
}