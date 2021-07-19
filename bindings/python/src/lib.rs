use numpy::{PyArray, PyArray1, PyArray2};
use pyo3::exceptions::{PyAttributeError, PyTypeError, PyValueError};
use pyo3::prelude::*;
use pyo3::types::PyDict;
use std::collections::{HashMap, HashSet};

use graph::*;
use tags::*;

mod macros;
pub(crate) use crate::macros::*;
mod edge_file_writer;
mod hash;
mod node_file_writer;
mod preprocessing;
mod trees;
mod utilities;
pub(crate) use crate::preprocessing::*;
pub(crate) use crate::utilities::*;
mod types;
pub(crate) use crate::types::*;
mod walks;
pub(crate) use crate::types::EnsmallenGraph;
mod operators;

// automatically generated files
mod auto_generated_bindings;
mod method_names_list;
pub use method_names_list::*;
