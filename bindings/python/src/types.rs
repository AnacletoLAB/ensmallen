use graph::Graph;
use pyo3::prelude::*;
use pyo3::wrap_pymodule;

#[pymodule]
fn ensmallen_graph(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<EnsmallenGraph>()?;
    //m.add_wrapped(wrap_pymodule!(preprocessing))?;
    env_logger::init();
    Ok(())
}

#[pyclass]
#[derive(Clone, PartialEq)]
pub(crate) struct EnsmallenGraph {
    pub(crate) graph: Graph,
}