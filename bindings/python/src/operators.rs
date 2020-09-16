#[pyproto]
impl PyNumberProtocol for EnsmallenGraph {
    fn __add__(lhs: EnsmallenGraph, rhs: EnsmallenGraph) -> PyResult<EnsmallenGraph> {
        match lhs.graph.sum(&rhs.graph) {
            Ok(g) => Ok(EnsmallenGraph { graph: g }),
            Err(e) => Err(PyErr::new::<exceptions::ValueError, _>(e)),
        }
    }
}

impl EnsmallenGraph {
    // separate the method from the __richcmp__ so that we can caputre and convert all the exceptions
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
        to_python_exception!(self.compare_graphs(other, op))
    }
}
