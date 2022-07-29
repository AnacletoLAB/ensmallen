use super::*;

#[pymethods]
impl Graph {
    fn __or__(&self, rhs: Self) -> PyResult<Self> {
        Ok(Self {
            inner: pe!(&self.inner | &rhs.inner)?,
        })
    }

    fn __sub__(&self, rhs: Self) -> PyResult<Self> {
        Ok(Self {
            inner: pe!(&self.inner - &rhs.inner)?,
        })
    }

    fn __and__(&self, rhs: Self) -> PyResult<Self> {
        Ok(Self {
            inner: pe!(&self.inner & &rhs.inner)?,
        })
    }

    fn __xor__(&self, rhs: Self) -> PyResult<Self> {
        Ok(Self {
            inner: pe!(&self.inner ^ &rhs.inner)?,
        })
    }
}

#[pymethods]
impl Graph {
    fn __getitem__(&self, idx: Py<PyAny>) -> PyResult<Py<PyAny>> {
        let gil = pyo3::Python::acquire_gil();

        if let Ok(node_id) = idx.extract::<u32>(gil.python()) {
            return pe!(self
                .inner
                .get_neighbour_node_ids_from_node_id(node_id as _)
                .map(|x| x.into_py(gil.python())));
        }

        if let Ok(node_name) = idx.extract::<&str>(gil.python()) {
            return pe!(self
                .inner
                .get_neighbour_node_names_from_node_name(node_name)
                .map(|x| x.into_py(gil.python())));
        }

        Err(PyValueError::new_err(format!(
            concat!(
                "A graph can be indexed using node ids and node names.",
                " The type '{}' is not currently supported.",
            ),
            idx.as_ref(gil.python()).get_type().to_string()
        )))
    }
}
