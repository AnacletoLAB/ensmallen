use preprocessing::{
    cooccurence_matrix as rust_cooccurence_matrix, okapi_bm25_tfidf as rust_okapi_bm25_tfidf,
    word2vec as rust_word2vec,
};
use pyo3::prelude::*;
use shared::*;

use crate::types::*;
use crate::utilities::validate_kwargs;

use numpy::PyArray1;
use pyo3::types::PyDict;
use rayon::prelude::*;
use std::collections::HashMap;

#[pyfunction()]
#[text_signature = "(documents, k1, b, vocabulary_size, verbose)"]
/// Return vocabulary and TFIDF matrix of given documents.
///
///
/// Arguments
/// ---------
/// documents: List[List[String]],
///     The documents to parse
/// k1: Optional[float],
///     The default parameter for k1, tipically between 1.2 and 2.0.
/// b: Optional[float],
///     The default parameter for b, tipically equal to 0.75.
/// vocabulary_size: Optional[usize],
///     The expected vocabulary size.
/// verbose: Optional[bool],
///     Whether to show a loading bar.
///
pub fn okapi_bm25_tfidf(
    documents: Vec<Vec<&str>>,
    k1: Option<f64>,
    b: Option<f64>,
    vocabulary_size: Option<usize>,
    verbose: Option<bool>,
) -> PyResult<Vec<HashMap<String, f64>>> {
    pe!(rust_okapi_bm25_tfidf(
        &documents,
        k1,
        b,
        vocabulary_size,
        verbose
    ))
}

#[pyfunction(py_kwargs = "**")]
#[text_signature = "(sequences, window_size)"]
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
/// Arguments
/// ---------
///
/// sequences: List[List[int]],
///     the sequence of sequences of integers to preprocess.
/// window_size: int,
///     Window size to consider for the sequences.
///
pub fn word2vec(sequences: Vec<Vec<NodeT>>, window_size: usize) -> (PyContexts, PyWords) {
    let (contexts, words): (Vec<Vec<NodeT>>, Vec<NodeT>) =
        rust_word2vec(sequences.into_par_iter(), window_size).unzip();
    let gil = pyo3::Python::acquire_gil();
    (
        to_ndarray_2d!(gil, contexts, NodeT),
        to_ndarray_1d!(gil, words, NodeT),
    )
}

#[pyfunction(py_kwargs = "**")]
#[text_signature = "(sequences, *, window_size, verbose)"]
/// Return triple with CSR representation of cooccurrence matrix.
///
/// The first vector has the sources, the second vector the destinations
/// and the third one contains the min-max normalized frequencies.
///
/// Arguments
/// ---------
///
/// sequences: List[List[int]],
///     the sequence of sequences of integers to preprocess.
/// window_size: int = 4,
///     Window size to consider for the sequences.
/// verbose: bool = False,
///     whether to show the progress bars.
///     The default behaviour is false.
///     
pub fn cooccurence_matrix(
    sequences: Vec<Vec<NodeT>>,
    py_kwargs: Option<&PyDict>,
) -> PyResult<(PyWords, PyWords, PyFrequencies)> {
    let _ = ctrlc::set_handler(|| std::process::exit(2));
    let gil = pyo3::Python::acquire_gil();
    let kwargs = normalize_kwargs!(py_kwargs, gil.python());
    pe!(validate_kwargs(kwargs, &["window_size", "verbose"]))?;
    let len = sequences.len();

    let (number_of_elements, iter) = pe!(rust_cooccurence_matrix(
        sequences.into_par_iter(),
        extract_value!(kwargs, "window_size", usize).unwrap_or(3),
        len,
        extract_value!(kwargs, "verbose", bool)
    ))?;

    let srcs = PyArray1::new(gil.python(), [number_of_elements], false);
    let dsts = PyArray1::new(gil.python(), [number_of_elements], false);
    let frequencies = PyArray1::new(gil.python(), [number_of_elements], false);

    iter.enumerate().for_each(|(i, (src, dst, freq))| unsafe {
        *srcs.uget_mut(i) = src;
        *dsts.uget_mut(i) = dst;
        *frequencies.uget_mut(i) = freq;
    });

    Ok((srcs.to_owned(), dsts.to_owned(), frequencies.to_owned()))
}
