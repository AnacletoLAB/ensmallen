use counter::Counter;
use indicatif::{ParallelProgressIterator, ProgressIterator};
use rayon::iter::{IntoParallelIterator, IntoParallelRefIterator, ParallelIterator};
use std::collections::hash_map::Entry::{Occupied, Vacant};
use std::collections::HashMap;

use super::*;

#[manual_binding]
/// Return vector of hashmaps containing the non-zero frequencies.
///
/// # Arguments
/// * `documents`: &[Vec<&str>] - The documents to parse
/// * `k1`: Option<f64> - The default parameter for k1, tipically between 1.2 and 2.0.
/// * `b`: Option<f64> - The default parameter for b, tipically equal to 0.75.
/// * `vocabulary_size`: Option<usize> - The expected vocabulary size.
/// * `verbose`: Option<bool> - Whether to show a loading bar.
///
pub fn okapi_bm25_tfidf(
    documents: &[Vec<&str>],
    k1: Option<f64>,
    b: Option<f64>,
    vocabulary_size: Option<usize>,
    verbose: Option<bool>,
) -> Result<Vec<HashMap<String, f64>>> {
    if documents.is_empty() {
        return Err("The given documents set is empty!".to_string());
    }
    let verbose = verbose.unwrap_or(true);
    let k1 = k1.unwrap_or(1.5);
    let b = b.unwrap_or(0.75);
    let number_of_documents = documents.len();
    let vocabulary_size = vocabulary_size.unwrap_or(100);
    let mut total_documents_length = 0;
    let mut vocabulary: HashMap<&str, usize> = HashMap::with_capacity(vocabulary_size);
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
            let counts: Counter<&str, usize> = document.iter().cloned().collect();
            counts
                .into_iter()
                .map(|(word_name, word_count)| {
                    let word_id = *vocabulary.get(word_name).unwrap();
                    let word_frequency = *word_count as f64 / document_len;
                    (
                        word_name.to_string(),
                        inverse_document_frequencies[word_id] * (word_frequency * (k1 + 1.0))
                            / (word_frequency
                                + k1 * (1.0 - b + b * document_len / average_document_len)),
                    )
                })
                .collect::<HashMap<String, f64>>()
        })
        .collect::<Vec<HashMap<String, f64>>>())
}
