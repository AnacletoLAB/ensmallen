use pyo3::exceptions;
use pyo3::prelude::*;
use pyo3::types::PyDict;
use pyo3::{wrap_pymodule};
mod macros;
pub(crate) use crate::macros::*;
mod edge_file_writer;
mod from_csv;
mod getters;
mod setters;
mod edge_lists;
mod filters;
mod metrics;
mod node_file_writer;
mod preprocessing;
mod remap;
mod trees;
mod connected_components;
mod tarjan;
mod thread_safe;
mod utilities;
pub(crate) use crate::preprocessing::*;
pub(crate) use crate::utilities::*;
mod types;
pub(crate) use crate::types::*;
mod walks;
pub(crate) use crate::types::EnsmallenGraph;
mod modifiers;

mod remove;
mod holdout;
mod operators;

#[pymodule]
fn ensmallen_graph(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<EnsmallenGraph>()?;
    m.add_wrapped(wrap_pymodule!(preprocessing))?;
    env_logger::init();
    Ok(())
}
