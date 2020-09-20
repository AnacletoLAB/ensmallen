use pyo3::exceptions;
use pyo3::prelude::*;
use pyo3::types::PyDict;
use pyo3::{wrap_pyfunction, wrap_pymodule};
mod macros;
pub(crate) use crate::macros::*;
mod edge_file_writer;
mod from_csv;
mod getters;
mod holdout;
mod metrics;
mod node_file_writer;
mod operators;
mod preprocessing;
mod utilities;
pub(crate) use crate::preprocessing::*;
mod tree;
mod types;
pub(crate) use crate::types::*;
mod walks;
pub(crate) use crate::types::EnsmallenGraph;

#[pymodule]
fn ensmallen_graph(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<EnsmallenGraph>()?;
    m.add_wrapped(wrap_pymodule!(preprocessing))?;
    env_logger::init();
    Ok(())
}
