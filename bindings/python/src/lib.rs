#![feature(adt_const_params)]
#![feature(generic_const_exprs)]
use numpy::{PyArray, PyArray1, PyArray2, PyArray3};
use pyo3::exceptions::{PyAttributeError, PyTypeError, PyValueError};
use pyo3::prelude::*;
use pyo3::types::PyDict;
use std::collections::{HashMap, HashSet};

#[allow(unused_imports)]
use graph::{EdgeT, EdgeTypeT, NodeT, NodeTypeT, Result, WeightT};
use tags::*;

pub(crate) mod mmap_numpy_npy;

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

mod hyperball_jaccard;
pub(crate) use hyperball_jaccard::*;

mod distance_node_label_prediction_perceptron;
pub(crate) use distance_node_label_prediction_perceptron::*;

mod dag_resnik;
pub use dag_resnik::*;

mod triad_census;
pub use triad_census::*;

mod basic_embedding_model_binding;
pub(crate) use basic_embedding_model_binding::*;

mod basic_siamese_model_binding;
pub(crate) use basic_siamese_model_binding::*;

mod dense;
pub use dense::*;

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

mod weighted_spine;
pub(crate) use weighted_spine::*;
mod walks;

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
    _m.add_class::<DistanceNodeLabelPredictionPerceptron>()?;
    _m.add_class::<DAGResnik>()?;
    _m.add_class::<HyperBallJaccard>()?;
    Ok(())
}

// automatically generated files
mod auto_generated_bindings;
pub use auto_generated_bindings::*;
