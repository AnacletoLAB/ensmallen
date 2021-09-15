use super::*;
use pyo3::class::number::PyNumberProtocol;
use rayon::iter::{IndexedParallelIterator, IntoParallelRefIterator, ParallelIterator};

#[pyproto]
impl PyNumberProtocol for Graph {
    fn __or__(lhs: Graph, rhs: Graph) -> PyResult<Graph> {
        Ok(Graph {
            graph: pe!(&lhs.graph | &rhs.graph)?,
        })
    }

    fn __sub__(lhs: Graph, rhs: Graph) -> PyResult<Graph> {
        Ok(Graph {
            graph: pe!(&lhs.graph - &rhs.graph)?,
        })
    }

    fn __and__(lhs: Graph, rhs: Graph) -> PyResult<Graph> {
        Ok(Graph {
            graph: pe!(&lhs.graph & &rhs.graph)?,
        })
    }

    fn __xor__(lhs: Graph, rhs: Graph) -> PyResult<Graph> {
        Ok(Graph {
            graph: pe!(&lhs.graph ^ &rhs.graph)?,
        })
    }
}
