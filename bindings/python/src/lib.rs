use numpy::{PyArray, PyArray1, PyArray2};
use pyo3::exceptions::PyTypeError;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::PyDict;
use pyo3::wrap_pymodule;
use std::collections::{HashMap, HashSet};

use graph::*;
use tags::*;

mod macros;
pub(crate) use crate::macros::*;
mod edge_file_writer;
mod from_csv;
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
pub(crate) use method_names_list::*;



#[pymodule]
fn ensmallen_graph(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<EnsmallenGraph>()?;
    m.add_wrapped(wrap_pymodule!(preprocessing))?;
    env_logger::init();
    Ok(())
}
