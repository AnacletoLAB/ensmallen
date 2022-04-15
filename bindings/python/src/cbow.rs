use super::*;
use graph::{
    NodeT, NodeTypeT, Tokens,
};
use numpy::{PyArray, PyArray1, PyArray2};
use pyo3::wrap_pyfunction;
use rayon::prelude::*;
use std::sync::atomic::{AtomicUsize, Ordering};
use types::ThreadDataRaceAware;


#[pymethods]
impl Graph {
    #[args(py_kwargs = "**")]
    #[text_signature = "($self, *, embedding_size, epochs, walk_length, window_size, negatives_number, learning_rate, random_state, verbose)"]
    fn cbow(
        &self,
        embedding_size: Option<usize>,
        epochs: Option<usize>,
        walk_length: Option<u64>,
        window_size: Option<usize>,
        negatives_number: Option<usize>,
        learning_rate: Option<f32>,
        random_state: Option<u64>,
        verbose: Option<bool>,
    ) -> PyResult<Py<PyArray2<f32>>> {
        let gil = pyo3::Python::acquire_gil();
    
        self.graph.cbow(
            embedding_size,
            epochs,
            walk_length,
            window_size,
            negatives_number,
            learning_rate,
            random_state,
            verbose,
        )
    }
}