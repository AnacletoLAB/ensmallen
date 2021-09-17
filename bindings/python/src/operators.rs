use super::*;
use pyo3::class::number::PyNumberProtocol;

#[pyproto]
impl PyNumberProtocol for Graph {
    fn __or__(lhs: Graph, rhs: Graph) -> PyResult<Graph> {
        Ok(Graph {
            inner: pe!(&lhs.inner | &rhs.inner)?,
        })
    }

    fn __sub__(lhs: Graph, rhs: Graph) -> PyResult<Graph> {
        Ok(Graph {
            inner: pe!(&lhs.inner - &rhs.inner)?,
        })
    }

    fn __and__(lhs: Graph, rhs: Graph) -> PyResult<Graph> {
        Ok(Graph {
            inner: pe!(&lhs.inner & &rhs.inner)?,
        })
    }

    fn __xor__(lhs: Graph, rhs: Graph) -> PyResult<Graph> {
        Ok(Graph {
            inner: pe!(&lhs.inner ^ &rhs.inner)?,
        })
    }
}
