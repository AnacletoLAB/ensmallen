#![feature(adt_const_params)]
#![feature(impl_trait_in_assoc_type)]
use numpy::{PyArray, PyArray1, PyArray2, PyArray4};
use pyo3::exceptions::{PyAttributeError, PyTypeError, PyValueError};
use pyo3::prelude::*;
use pyo3::types::PyDict;
use std::collections::{HashMap, HashSet};

pub use graph::{EdgeT, EdgeTypeT, NodeT, NodeTypeT, Result, WeightT};
use tags::*;

pub mod mmap_numpy_npy;

mod from_pd;

mod macros;
pub(crate) use crate::macros::*;
mod express_measures;
pub(crate) use crate::express_measures::*;

mod node2vec;
pub use crate::node2vec::*;

mod edge_prediction_perceptron;
pub(crate) use edge_prediction_perceptron::*;

mod node_label_prediction_perceptron;
pub(crate) use node_label_prediction_perceptron::*;

mod hyper_jaccard;
pub(crate) use hyper_jaccard::*;

mod hyper_sketching;
pub(crate) use hyper_sketching::*;

mod dag_resnik;
pub use dag_resnik::*;

mod triad_census;

mod basic_embedding_model_binding;
pub(crate) use basic_embedding_model_binding::*;

mod basic_siamese_model_binding;
pub(crate) use basic_siamese_model_binding::*;

mod dense;

mod graph_embedder;
pub use graph_embedder::*;

mod edge_file_writer;
mod hash;
mod node_file_writer;
mod preprocessing;
mod subgraphs;
mod utilities;
pub(crate) use crate::preprocessing::*;
pub(crate) use crate::utilities::*;
mod types;
pub(crate) use crate::types::*;
mod alpine;
mod operators;
pub(crate) use alpine::*;

mod graph_convolution;
pub use graph_convolution::*;

mod weighted_spine;
pub(crate) use weighted_spine::*;
mod walks;

#[cfg(feature = "register_pymodule")]
#[pymodule]
pub fn ensmallen(py: Python, m: &PyModule) -> PyResult<()> {
    register_ensmallen(py, m)?;
    Ok(())
}

pub fn register_models(_py: Python, _m: &PyModule) -> PyResult<()> {
    _m.add_class::<CBOW>()?;
    _m.add_class::<GloVe>()?;
    _m.add_class::<SkipGram>()?;
    _m.add_class::<WalkletsCBOW>()?;
    _m.add_class::<WalkletsGloVe>()?;
    _m.add_class::<WalkletsSkipGram>()?;
    _m.add_class::<TransE>()?;
    _m.add_class::<Unstructured>()?;
    _m.add_class::<StructuredEmbedding>()?;
    _m.add_class::<FirstOrderLINE>()?;
    _m.add_class::<SecondOrderLINE>()?;
    _m.add_class::<DegreeSPINE>()?;
    _m.add_class::<NodeLabelSPINE>()?;
    _m.add_class::<ScoreSPINE>()?;
    _m.add_class::<RUBICONE>()?;
    _m.add_class::<RUINE>()?;
    _m.add_class::<DegreeWINE>()?;
    _m.add_class::<NodeLabelWINE>()?;
    _m.add_class::<ScoreWINE>()?;
    _m.add_class::<WeightedSPINE>()?;
    _m.add_class::<EdgePredictionPerceptron>()?;
    _m.add_class::<NodeLabelPredictionPerceptron>()?;
    _m.add_class::<DAGResnik>()?;
    _m.add_class::<HyperJaccard>()?;
    _m.add_class::<HyperSketching>()?;
    _m.add_class::<GraphConvolution>()?;
    Ok(())
}

// automatically generated files
mod auto_generated_bindings;
pub use auto_generated_bindings::*;
