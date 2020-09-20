use super::*;
use pyo3::class::basic::CompareOp;
use pyo3::class::number::PyNumberProtocol;
use pyo3::class::PyObjectProtocol;
use pyo3::exceptions;

#[pyproto]
impl PyNumberProtocol for EnsmallenGraph {
    fn __add__(lhs: EnsmallenGraph, rhs: EnsmallenGraph) -> PyResult<EnsmallenGraph> {
        Ok(EnsmallenGraph {
            graph: pyex!(&lhs.graph + &rhs.graph)?,
        })
    }
}

#[pymethods]
impl EnsmallenGraph {
    /// Return true if given graph has any edge overlapping with current graph.
    ///
    /// Parameters
    /// ----------------------------
    /// graph: EnsmallenGraph,
    ///     The graph to check against.
    ///
    /// Returns
    /// ----------------------------
    /// Boolean representing if any overlapping edge was found.
    pub fn overlaps(&self, graph: &EnsmallenGraph) -> PyResult<bool> {
        pyex!(self.graph.overlaps(&graph.graph))
    }

    /// Return true if given graph edges are all contained within current graph.
    ///
    /// Parameters
    /// ----------------------------
    /// graph: EnsmallenGraph,
    ///     The graph to check against.
    ///
    /// Returns
    /// ----------------------------
    /// Boolean representing if graph contains completely the othe graph.
    pub fn contains(&self, graph: &EnsmallenGraph) -> PyResult<bool> {
        pyex!(self.graph.contains(&graph.graph))
    }
}
impl EnsmallenGraph {
    // separate the method from the __richcmp__ so that we can capture and convert all the exceptions
    // in an uniform way
    pub(crate) fn compare_graphs(
        &self,
        other: EnsmallenGraph,
        op: CompareOp,
    ) -> Result<bool, String> {
        Ok(match op {
            CompareOp::Lt => other.graph.contains(&self.graph)? && &other != self,
            CompareOp::Le => other.graph.contains(&self.graph)?,
            CompareOp::Eq => &other == self,
            CompareOp::Ne => &other != self,
            CompareOp::Gt => self.graph.contains(&other.graph)? && &other != self,
            CompareOp::Ge => self.graph.contains(&other.graph)?,
        })
    }
}

#[pyproto]
impl PyObjectProtocol for EnsmallenGraph {
    fn __richcmp__(&self, other: EnsmallenGraph, op: CompareOp) -> PyResult<bool> {
        pyex!(self.compare_graphs(other, op))
    }
}
