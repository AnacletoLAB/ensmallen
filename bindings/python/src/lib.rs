use numpy::{PyArray, PyArray1, PyArray2};
use pyo3::exceptions::{PyAttributeError, PyTypeError, PyValueError};
use pyo3::prelude::*;
use pyo3::types::PyDict;
use std::collections::{HashMap, HashSet};

#[allow(unused_imports)]
use graph::{EdgeT, EdgeTypeT, NodeT, NodeTypeT, Result, WeightT};
use tags::*;

mod macros;
pub(crate) use crate::macros::*;
mod cbow;
pub(crate) use cbow::*;
mod edge_file_writer;
mod hash;
mod node_file_writer;
mod preprocessing;
mod skipgram;
pub(crate) use skipgram::*;
mod subgraphs;
mod trees;
mod utilities;
pub(crate) use crate::preprocessing::*;
pub(crate) use crate::utilities::*;
mod types;
pub(crate) use crate::types::*;
mod laplacian;
mod operators;
mod spine;
mod walks;

#[pymodule]
fn models(_py: Python, _m: &PyModule) -> PyResult<()> {
    _m.add_class::<CBOW>()?;
    _m.add_class::<SkipGram>()?;
    Ok(())
}

// automatically generated files
mod auto_generated_bindings;
pub use auto_generated_bindings::*;
