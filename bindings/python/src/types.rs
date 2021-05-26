use graph::{Graph, NodeT};
use numpy::{PyArray1, PyArray2};
use pyo3::prelude::*;

#[pyclass]
#[derive(Clone, Debug)]
pub(crate) struct EnsmallenGraph {
    pub(crate) graph: Graph,
}

pub type PyContexts = Py<PyArray2<NodeT>>;
pub type PyWords = Py<PyArray1<NodeT>>;
pub type PyFrequencies = Py<PyArray1<f64>>;

pub struct ThreadDataRaceAware<'a, T> {
    pub(crate) t: &'a T,
}

unsafe impl<'a, T> Sync for ThreadDataRaceAware<'a, T> {}
