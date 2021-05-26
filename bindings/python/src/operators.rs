use super::*;
extern crate edit_distance;
use pyo3::class::basic::PyObjectProtocol;
use pyo3::class::number::PyNumberProtocol;

#[pyproto]
impl PyNumberProtocol for EnsmallenGraph {
    fn __or__(lhs: EnsmallenGraph, rhs: EnsmallenGraph) -> PyResult<EnsmallenGraph> {
        Ok(EnsmallenGraph {
            graph: pe!(&lhs.graph | &rhs.graph)?,
        })
    }

    fn __sub__(lhs: EnsmallenGraph, rhs: EnsmallenGraph) -> PyResult<EnsmallenGraph> {
        Ok(EnsmallenGraph {
            graph: pe!(&lhs.graph - &rhs.graph)?,
        })
    }

    fn __and__(lhs: EnsmallenGraph, rhs: EnsmallenGraph) -> PyResult<EnsmallenGraph> {
        Ok(EnsmallenGraph {
            graph: pe!(&lhs.graph & &rhs.graph)?,
        })
    }

    fn __xor__(lhs: EnsmallenGraph, rhs: EnsmallenGraph) -> PyResult<EnsmallenGraph> {
        Ok(EnsmallenGraph {
            graph: pe!(&lhs.graph ^ &rhs.graph)?,
        })
    }
}

#[pyproto]
impl PyObjectProtocol for EnsmallenGraph {
    fn __str__(&'p self) -> PyResult<String> {
        pe!(self.graph.textual_report(true))
    }
    fn __repr__(&'p self) -> PyResult<String> {
        self.__str__()
    }

    fn __hash__(&'p self) -> PyResult<isize> {
        Ok(self.hash() as isize)
    }
}

#[pymethods]
impl EnsmallenGraph {
    fn _repr_html_(&self) -> PyResult<String> {
        Ok(format!(
            r#"<h4>{}</h4><p style="text-align: justify; text-justify: inter-word;">{}</p>"#,
            self.graph.get_name(),
            pe!(self.__repr__())?
        ))
    }
}
