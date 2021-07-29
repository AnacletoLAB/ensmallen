use graph::Graph;
use shared::types::NodeT;
use numpy::{PyArray1, PyArray2};
use pyo3::prelude::*;

#[pyclass]
#[derive(Clone, Debug)]
pub struct EnsmallenGraph {
    pub(crate) graph: Graph,
}

pub type PyContexts = Py<PyArray2<NodeT>>;
pub type PyWords = Py<PyArray1<NodeT>>;
pub type PyFrequencies = Py<PyArray1<f64>>;