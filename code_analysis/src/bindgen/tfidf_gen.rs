use itertools::Itertools;
use rayon::prelude::*;
use std::sync::atomic::{AtomicUsize, Ordering};

/// Pre-compute the TD-IDF weight for each term of each binding.
/// Then write the compute weights in a file at the given path.
pub fn tfidf_gen(method_names: &[&str]) -> (Vec<String>, Vec<Vec<(String, f32)>>) {
    let documents = method_names
        .iter()
        .map(|x| split_words(x))
        .collect::<Vec<Vec<String>>>();

    let vals = documents
        .iter()
        .map(|x| x.iter().map(String::as_str).collect::<Vec<&str>>())
        .collect::<Vec<Vec<&str>>>();

    let tfidf = get_okapi_bm25_tfidf_from_documents(&vals[..], None, None, Some(false)).unwrap();

    let unique_terms_list = documents
        .iter()
        .flat_map(|document| document.into_iter())
        .cloned()
        .unique()
        .collect();

    (
        unique_terms_list,
        tfidf
            .into_iter()
            .map(|vals| {
                vals.into_iter()
                    .map(|(k, v)| (k.to_string(), v))
                    .collect::<Vec<(String, f32)>>()
            })
            .collect::<Vec<Vec<(String, f32)>>>(),
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
/// * `documents`: &[Vec<T>] - The documents to be processed.
/// * `k1`: Option<f32> - The default parameter for k1, tipically between 1.2 and 2.0.
/// * `b`: Option<f32> - The default parameter for b, tipically equal to 0.75.
/// * `verbose`: Option<bool> - Whether to show a loading bar. By default, true.
///
pub fn get_okapi_bm25_tfidf_from_documents<T: Eq + Hash + Send + Sync + Clone + Copy + Eq>(
    documents: &[Vec<T>],
    k1: Option<f32>,
    b: Option<f32>,
    verbose: Option<bool>,
) -> Result<Vec<HashMap<T, f32>>, String> {
    if documents.is_empty() {
        return Err("The given documents set is empty!".to_string());
    }
    let verbose = verbose.unwrap_or(true);
    let k1 = k1.unwrap_or(1.5);
    let b = b.unwrap_or(0.75);
    let number_of_documents = documents.len();
    let pb = get_loading_bar(verbose, "Computing vocabulary", number_of_documents);
    // We start to iterate over the documents list and create the vocabulary.
    let vocabulary: HashMap<&T, usize> = documents
        .iter()
        .progress_with(pb)
        .flat_map(|document| document.iter())
        .unique()
        .enumerate()
        .map(|(element_id, element)| (element, element_id))
        .collect();
    // We start to compute, for each word, the number of documents that contain this word.
    let pb = get_loading_bar(verbose, "Computing word counts", number_of_documents);
    let unique_document_occurrencies_per_word: Vec<AtomicUsize> =
        (0..vocabulary.len()).map(|_| AtomicUsize::new(0)).collect();
    let total_documents_length: usize = documents
        .par_iter()
        .progress_with(pb)
        .map(|document| {
            document.iter().unique().for_each(|element| {
                unique_document_occurrencies_per_word[*vocabulary.get(element).unwrap()]
                    .fetch_add(1, Ordering::Relaxed);
            });
            document.len()
        })
        .sum();
    // Transforming the memory allocated for AtomicUsize vector into a normal vector of usize.
    let unique_document_occurrencies_per_word = unsafe {
        std::mem::transmute::<Vec<AtomicUsize>, Vec<usize>>(unique_document_occurrencies_per_word)
    };
    // Computing average document size
    let average_document_len = total_documents_length as f32 / number_of_documents as f32;
    // Creating loading bar for actually computing TFIDF
    let pb = get_loading_bar(verbose, "Building TFIDF", number_of_documents);
    // Computing TFIDF of provided words and documents
    Ok(documents
        .par_iter()
        .progress_with(pb)
        .map(|document| {
            let document_len = document.len() as f32;
            document
                .iter()
                .collect::<Counter<&T, usize>>()
                .into_iter()
                .map(|(&word_name, &current_document_word_count)| {
                    // Surely the word is, by definition in the vocabulary.
                    let word_id = *vocabulary.get(word_name).unwrap();
                    let word_frequency = current_document_word_count as f32 / document_len;
                    let unique_document_occurrencies =
                        unique_document_occurrencies_per_word[word_id] as f32;
                    let inverse_document_frequency =
                        ((number_of_documents as f32 - unique_document_occurrencies + 0.5)
                            / (unique_document_occurrencies + 0.5))
                            .ln_1p();
                    let adjusted_word_frequency = (word_frequency * (k1 + 1.0))
                        / (word_frequency
                            + k1 * (1.0 - b + b * document_len / average_document_len));
                    (
                        *word_name,
                        inverse_document_frequency * adjusted_word_frequency,
                    )
                })
                .collect::<HashMap<T, f32>>()
        })
        .collect::<Vec<HashMap<T, f32>>>())
}
