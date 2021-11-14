use counter::Counter;
use funty::IsInteger;
use indicatif::{ParallelProgressIterator, ProgressIterator};
use itertools::Itertools;
use log::info;
use rayon::prelude::*;
use std::collections::HashMap;
use std::hash::Hash;
use std::sync::atomic::{AtomicUsize, Ordering};
use tokenizers::tokenizer::Tokenizer;

use super::*;

#[manual_binding]
/// Return vector of hashmaps containing the non-zero frequencies.
///
/// # Arguments
/// * `documents`: &[Vec<T>] - The documents to be processed.
/// * `k1`: Option<f32> - The default parameter for k1, tipically between 1.2 and 2.0.
/// * `b`: Option<f32> - The default parameter for b, tipically equal to 0.75.
/// * `verbose`: Option<bool> - Whether to show a loading bar. By default, true.
///
pub fn iter_okapi_bm25_tfidf_from_documents<'a, T: Eq + Hash + Send + Sync + Clone + Copy>(
    documents: &'a [Vec<T>],
    k1: Option<f32>,
    b: Option<f32>,
    verbose: Option<bool>,
) -> Result<impl IndexedParallelIterator<Item = HashMap<T, f32>> + 'a> {
    if documents.is_empty() {
        return Err("The given documents set is empty!".to_string());
    }
    let verbose = verbose.unwrap_or(true);
    let k1 = k1.unwrap_or(1.5);
    let b = b.unwrap_or(0.75);
    let number_of_documents = documents.len();
    info!("Starting to compute the vocabulary mapping.");
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
    info!("Starting to compute the word counts.");
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
    Ok(documents.par_iter().progress_with(pb).map(move |document| {
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
                    / (word_frequency + k1 * (1.0 - b + b * document_len / average_document_len));
                (
                    *word_name,
                    inverse_document_frequency * adjusted_word_frequency,
                )
            })
            .collect::<HashMap<T, f32>>()
    }))
}

pub enum Tokens {
    TokensU8(Vec<Vec<u8>>),
    TokensU16(Vec<Vec<u16>>),
    TokensU32(Vec<Vec<u32>>),
    TokensU64(Vec<Vec<u64>>),
}

impl Tokens {
    fn default_allocation<T: IsInteger>(rows_number: usize) -> Vec<Vec<T>> {
        (0..rows_number).map(|_| Vec::new()).collect()
    }

    fn set(&mut self, values: &[u32], index: usize) {
        match self {
            Tokens::TokensU8(inner) => {
                inner[index] = values.into_iter().cloned().map(|x| x as u8).collect();
            }
            Tokens::TokensU16(inner) => {
                inner[index] = values.into_iter().cloned().map(|x| x as u16).collect();
            }
            Tokens::TokensU32(inner) => {
                inner[index] = values.into_iter().cloned().map(|x| x as u32).collect();
            }
            Tokens::TokensU64(inner) => {
                inner[index] = values.into_iter().cloned().map(|x| x as u64).collect();
            }
        }
    }

    pub fn len(&self) -> usize {
        match self {
            Tokens::TokensU8(inner) => inner.len(),
            Tokens::TokensU16(inner) => inner.len(),
            Tokens::TokensU32(inner) => inner.len(),
            Tokens::TokensU64(inner) => inner.len(),
        }
    }

    fn new(rows_number: usize, max_value: usize) -> Self {
        if max_value < 2.pow(8) {
            Tokens::TokensU8(Tokens::default_allocation(rows_number))
        } else if max_value < 2.pow(16) {
            Tokens::TokensU16(Tokens::default_allocation(rows_number))
        } else if max_value < 2.pow(32) {
            Tokens::TokensU32(Tokens::default_allocation(rows_number))
        } else {
            Tokens::TokensU64(Tokens::default_allocation(rows_number))
        }
    }
}

#[manual_binding]
/// Return vector of vector of tokens extracted from given CSV.
///
/// # Arguments
/// * `path`: &str - The path to be processed.
/// * `columns`: Option<Vec<String>> - The columns to be read. If none are given, all the columns will be used.
/// * `separator`: Option<char> - The separator for the CSV.
/// * `header`: Option<bool> - Whether to skip the header.
/// * `pretrained_model_name_or_path`: Option<&str> - Name of the tokenizer model to be retrieved.
///
pub fn get_tokenized_csv(
    path: &str,
    columns: Option<Vec<String>>,
    separator: Option<char>,
    header: Option<bool>,
    pretrained_model_name_or_path: Option<&str>,
) -> Result<Tokens> {
    // Set the pretrained model if none where given
    let pretrained_model_name_or_path =
        pretrained_model_name_or_path.unwrap_or("allenai/scibert_scivocab_uncased");
    // Retrieve the pretrained tokenizer from HuggngFace
    let tokenizer = Tokenizer::from_pretrained(pretrained_model_name_or_path, None)
        .map_err(|err| err.to_string())?;
    // Get the tokens size
    let vocabulary_size = tokenizer.get_vocab_size(false);
    // Create the parallel CSV reader
    let reader = CSVFileReader::new(path, "document".to_string())?
        .set_parallel(Some(true))
        .set_header(header)?
        .set_separator(separator)?;
    // Use all of the CSV columns if no columns were specified.
    let columns = if let Some(columns) = columns {
        columns
    } else {
        reader.get_header()?
    };
    // Validate the provided columns and convert them to the curresponding integer.
    let columns: Vec<usize> = columns
        .into_iter()
        .map(|column| reader.get_column_number(column))
        .collect::<Result<Vec<usize>>>()?;
    // Get the number of lines in the file, where each one is a document.
    let rows_number = reader.count_rows()? - reader.get_total_lines_to_skip(reader.header)?;
    // Allocate the vector of tokens to be populated in parallel by multiple threads
    let mut tokens: Tokens = Tokens::new(rows_number, vocabulary_size);
    // Wrap the tokens in a way that can be shared by threads
    let thread_shared_tokens = ThreadDataRaceAware::new(&mut tokens);
    // Start to read the CSV file
    reader
        .read_lines(Some(columns))?
        .filter_map(|line| line.ok())
        .for_each(|(i, values)| unsafe {
            // Convert the elements in the line to a phrase
            let phrase = values.into_iter().filter_map(|value| value).join(" ");
            // Try to tokenize and if the tokenization is successfull
            if let Ok(tokens) = tokenizer.encode(phrase, false) {
                // We assign it to the curresponding vector.
                (*thread_shared_tokens.value.get()).set(tokens.get_ids(), i);
            };
        });
    // Return the resulting token.
    Ok(tokens)
}

#[manual_binding]
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
) -> Result<Vec<HashMap<T, f32>>> {
    Ok(
        iter_okapi_bm25_tfidf_from_documents(documents, k1, b, verbose)?
            .collect::<Vec<HashMap<T, f32>>>(),
    )
}
