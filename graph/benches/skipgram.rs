use rayon::prelude::*;

extern crate rand;
use rand::Rng;  
use rand::seq::SliceRandom;
use rand::thread_rng;


pub fn gen_random_usize_vec(num: usize, max: usize) -> Vec<usize> {
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

pub fn naife_skipgram_preprocessing(
    walk: &Vec<usize>,
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
    let mut words: Vec<usize> = Vec::with_capacity(vector_length);
    let mut contexts: Vec<usize> = Vec::with_capacity(vector_length);
    
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
    let words_neg: Vec<usize> = gen_random_usize_vec(num_negatives, walk_len - 1)
        .iter()
        .map(|i| walk[*i])
        .collect();
    let contexts_neg: Vec<usize> = gen_random_usize_vec(num_negatives, vocabulary_size - 1);
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

    return (
        words,
        contexts,
        labels
    )

}
