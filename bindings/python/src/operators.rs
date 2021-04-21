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
    /*
    fn __getattr__(&'p self, name: String) -> PyResult<()> {
        let gil = Python::acquire_gil();
        let py = gil.python();

        // BLACK MAGIC START
        let attrs = unsafe {py.from_owned_ptr::<PyList>(pyo3::ffi::PyObject_Dir(
            ((self as *const EnsmallenGraph as usize) - 24) as _
        ))};
        // BLACK MAGIC END

        println!("VIVO");

        let mut min = usize::MAX;
        let mut closest = "METHOD NOT FOUND".to_string();

        for attr in attrs.iter() {
            let attr_name = attr.extract::<String>().map_err(|_| {
                PyTypeError::new_err(format!(
                    "The value passed {} cannot be casted as from {} to String",
                    attr,
                    attr.get_type().name().unwrap(),
                ))
            })?;
            let distance = edit_distance(&name, &attr_name);

            if distance < min {
                min = distance;
                closest = attr_name;
            }
        };

        Err(PyTypeError::new_err(
            format!(
                "The method or attribute {} does not exists, did you mean {}?",
                name, closest
            )
        ))
    }
    */
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
        pe!(self.graph.overlaps(&graph.graph))
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
        pe!(self.graph.contains(&graph.graph))
    }
}
