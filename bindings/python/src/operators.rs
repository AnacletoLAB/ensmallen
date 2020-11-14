use super::*;
use pyo3::class::basic::PyObjectProtocol;
use pyo3::class::number::PyNumberProtocol;

#[pyproto]
impl PyNumberProtocol for EnsmallenGraph {
    fn __or__(lhs: EnsmallenGraph, rhs: EnsmallenGraph) -> PyResult<EnsmallenGraph> {
        Ok(EnsmallenGraph {
            graph: pyex!(&lhs.graph | &rhs.graph)?,
        })
    }

    fn __sub__(lhs: EnsmallenGraph, rhs: EnsmallenGraph) -> PyResult<EnsmallenGraph> {
        Ok(EnsmallenGraph {
            graph: pyex!(&lhs.graph - &rhs.graph)?,
        })
    }

    fn __and__(lhs: EnsmallenGraph, rhs: EnsmallenGraph) -> PyResult<EnsmallenGraph> {
        Ok(EnsmallenGraph {
            graph: pyex!(&lhs.graph & &rhs.graph)?,
        })
    }

    fn __xor__(lhs: EnsmallenGraph, rhs: EnsmallenGraph) -> PyResult<EnsmallenGraph> {
        Ok(EnsmallenGraph {
            graph: pyex!(&lhs.graph ^ &rhs.graph)?,
        })
    }
}

#[pyproto]
impl PyObjectProtocol for EnsmallenGraph {
    fn __str__(&'p self) -> PyResult<String> {
        pyex!(self.graph.textual_report(true))
    }
    fn __repr__(&'p self) -> PyResult<String> {
        self.__str__()
    }
}

#[pymethods]
impl EnsmallenGraph {
    fn _repr_html_(&self) -> PyResult<String> {
        Ok(format!(r#"<p style="text-align: justify; text-justify: inter-word;">{}</p>"#, self.__repr__()?))
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
