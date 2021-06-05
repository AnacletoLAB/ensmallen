use super::*;
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
        pe!(self.graph.textual_report(Some(true)))
    }
    fn __repr__(&'p self) -> PyResult<String> {
        self.__str__()
    }

    fn __hash__(&'p self) -> PyResult<isize> {
        Ok(self.hash() as isize)
    }

    fn __getattr__(&self, name: String) -> PyResult<()> {
        let mut distances = METHODS_NAMES
            .iter()
            .map(|method_name| (method_name, levenshtein(&name, &method_name)))
            .collect::<Vec<(&&str, usize)>>();

        distances.sort_by(|(_, d1), (_, d2)| d1.cmp(d2));

        Err(PyTypeError::new_err(format!(
            "The method {} does not exists, did you mean {:?}?",
            name,
            &distances[..10]
                .iter()
                .map(|(method, _distance)| *method)
                .collect::<Vec<&&str>>(),
        )))
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
