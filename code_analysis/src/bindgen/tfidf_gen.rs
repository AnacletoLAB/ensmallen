use std::collections::HashSet;

/// Pre-compute the TD-IDF weight for each term of each binding.
/// Then write the compute weights in a file at the given path.
pub fn tfidf_gen(method_names: &[&str]) -> (Vec<String>, Vec<Vec<(String, f64)>>) {
    let documents = method_names
        .iter()
        .map(|x| split_words(x))
        .collect::<Vec<Vec<String>>>();

    let vals = documents
        .iter()
        .map(|x| x.iter().map(String::as_str).collect::<Vec<&str>>())
        .collect::<Vec<Vec<&str>>>();

    let tfidf = okapi_bm25_tfidf(&vals[..], None, None, None, None).unwrap();

    let mut terms = HashSet::new();

    for document in &documents {
        for term in document {
            terms.insert(term.clone());
        }
    }

    (
        terms.into_iter().collect::<Vec<_>>(), 
        tfidf.into_iter()
        .map(|vals| 
            vals.into_iter()
            .map(|(k, v)| 
                (k.to_string(), v)
            ).collect::<Vec<(String, f64)>>()
        )
        .collect::<Vec<Vec<(String, f64)>>>()
    )
}


fn split_words(method_name: &str) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();
    for word in method_name.split('_') {
        match word {
            "type" | "types" | "id" | "ids" | "name" | "names" => match result.last_mut() {
                Some(last) => {
                    last.push('_');
                    last.push_str(word);
                }
                None => {
                    result.push(word.to_string());
                }
            },
            _ => {
                result.push(word.to_string());
            }
        };
    }

    result.into_iter().filter(|x| !x.is_empty()).collect()
}

use counter::Counter;
use indicatif::{ParallelProgressIterator, ProgressIterator};
use indicatif::{ProgressBar, ProgressStyle};
use rayon::iter::{IntoParallelIterator, IntoParallelRefIterator, ParallelIterator};
use std::collections::hash_map::Entry::{Occupied, Vacant};
use std::collections::HashMap;
use std::hash::Hash;

pub fn get_loading_bar(verbose: bool, desc: &str, total_iterations: usize) -> ProgressBar {
    if verbose {
        let pb = ProgressBar::new(total_iterations as u64);
        let candidate_iterations = total_iterations as u64 / 1000;
        let candidate_iterations = candidate_iterations.max(1);
        pb.set_draw_delta(candidate_iterations);
        pb.set_style(ProgressStyle::default_bar().template(&format!(
            "{desc} {{spinner:.green}} [{{elapsed_precise}}] [{{bar:40.cyan/blue}}] ({{pos}}/{{len}}, ETA {{eta}})",
            desc=desc
        )));
        pb
    } else {
        ProgressBar::hidden()
    }
}

/// Return vector of hashmaps containing the non-zero frequencies.
///
/// # Arguments
/// * `documents`: &[Vec<&str>] - The documents to parse
/// * `k1`: Option<f64> - The default parameter for k1, tipically between 1.2 and 2.0.
/// * `b`: Option<f64> - The default parameter for b, tipically equal to 0.75.
/// * `vocabulary_size`: Option<usize> - The expected vocabulary size.
/// * `verbose`: Option<bool> - Whether to show a loading bar.
///
pub fn okapi_bm25_tfidf<T1: Eq + Hash + Send + Sync + Clone + Copy + Eq>(
    documents: &[Vec<T1>],
    k1: Option<f64>,
    b: Option<f64>,
    vocabulary_size: Option<usize>,
    verbose: Option<bool>,
) -> Result<Vec<HashMap<T1, f64>>, String> {
    if documents.is_empty() {
        return Ok(Vec::new());
    }
    let verbose = verbose.unwrap_or(true);
    let k1 = k1.unwrap_or(1.5);
    let b = b.unwrap_or(0.75);
    let number_of_documents = documents.len();
    let vocabulary_size = vocabulary_size.unwrap_or(100);
    let mut total_documents_length = 0;
    let mut vocabulary: HashMap<&T1, usize> = HashMap::with_capacity(vocabulary_size);
    let mut word_counts: Vec<usize> = Vec::new();
    let pb = get_loading_bar(verbose, "Building vocabulary", number_of_documents);
    for document in documents.iter().progress_with(pb) {
        total_documents_length += document.len();
        for word in document.iter() {
            let current_vocabulary_length = vocabulary.len();
            match vocabulary.entry(word) {
                Vacant(entry) => {
                    entry.insert(current_vocabulary_length);
                    word_counts.push(1);
                }
                Occupied(entry) => {
                    word_counts[*entry.get()] += 1;
                }
            }
        }
    }
    // Computing average document size
    let average_document_len = total_documents_length as f64 / number_of_documents as f64;
    // Computing inverse document frequencies
    let inverse_document_frequencies = word_counts
        .into_par_iter()
        .map(|counts| {
            ((number_of_documents as f64 - counts as f64 + 0.5) / (counts as f64 + 0.5)).ln_1p()
        })
        .collect::<Vec<f64>>();
    // Creating loading bar for actually computing TFIDF
    let pb = get_loading_bar(verbose, "Building TFIDF", number_of_documents);
    // Computing TFIDF of provided words and documents
    Ok(documents
        .par_iter()
        .progress_with(pb)
        .map(|document| {
            let document_len = document.len() as f64;
            let counts: Counter<&T1, usize> = document.iter().collect();
            counts
                .into_iter()
                .map(|(&word_name, &word_count)| {
                    // Surely the word is, by definition in the vocabulary.
                    let word_id = *vocabulary.get(word_name).unwrap();
                    let word_frequency = word_count as f64 / document_len;
                    (
                        *word_name,
                        inverse_document_frequencies[word_id] * (word_frequency * (k1 + 1.0))
                            / (word_frequency
                                + k1 * (1.0 - b + b * document_len / average_document_len)),
                    )
                })
                .collect::<HashMap<T1, f64>>()
        })
        .collect::<Vec<HashMap<T1, f64>>>())
}
