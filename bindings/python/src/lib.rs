use pyo3::exceptions::PyTypeError;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::PyDict;
use pyo3::wrap_pymodule;
mod macros;
pub(crate) use crate::macros::*;
mod edge_file_writer;
mod edge_lists;
mod filters;
mod from_csv;
mod getters;
mod hash;
mod metrics;
mod node_file_writer;
mod thickeners;
mod preprocessing;
mod remap;
mod setters;
mod tarjan;
mod polygons;
mod vertex_cover;
mod trees;
mod utilities;
pub(crate) use crate::preprocessing::*;
pub(crate) use crate::utilities::*;
mod types;
pub(crate) use crate::types::*;
mod walks;
pub(crate) use crate::types::EnsmallenGraph;
mod modifiers;
mod dijkstra;

mod compression;
mod getters_boolean;
mod holdout;
mod operators;
mod queries;
mod queries_boolean;
mod centrality;
mod remove;
mod replace;
mod report;
mod validators;

#[pymodule]
fn ensmallen_graph(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<EnsmallenGraph>()?;
    m.add_wrapped(wrap_pymodule!(preprocessing))?;
    env_logger::init();
    Ok(())
}
