use super::*;
use numpy::PyArray2;

///
#[pyclass]
#[derive(Debug, Clone)]
#[text_signature = "(*, embedding_size)"]
pub struct WeightedSPINE {
    pub inner: cpu_models::WeightedSPINE,
}

impl From<cpu_models::WeightedSPINE> for WeightedSPINE {
    fn from(val: cpu_models::WeightedSPINE) -> WeightedSPINE {
        WeightedSPINE { inner: val }
    }
}

impl From<WeightedSPINE> for cpu_models::WeightedSPINE {
    fn from(val: WeightedSPINE) -> cpu_models::WeightedSPINE {
        val.inner
    }
}

#[pymethods]
impl WeightedSPINE {
    #[new]
    #[args(py_kwargs = "**")]
    /// Return a new instance of the WeightedSPINE model.
    ///
    /// Parameters
    /// ------------------------
    /// embedding_size: Optional[int] = 100
    ///     Size of the embedding.
    /// use_edge_weights_as_probabilities: Optional[bool] = False
    ///     Whether to treat the weights as probabilities.
    pub fn new(py_kwargs: Option<&PyDict>) -> PyResult<WeightedSPINE> {
        let py = pyo3::Python::acquire_gil();
        let kwargs = normalize_kwargs!(py_kwargs, py.python());

        pe!(validate_kwargs(
            kwargs,
            build_walk_parameters_list(&["embedding_size", "use_edge_weights_as_probabilities"])
                .as_slice()
        ))?;

        Ok(Self {
            inner: pe!(cpu_models::WeightedSPINE::new(
                extract_value_rust_result!(kwargs, "embedding_size", usize),
                extract_value_rust_result!(kwargs, "use_edge_weights_as_probabilities", bool)
            ))?,
        })
    }
}

#[pymethods]
impl WeightedSPINE {
    #[args(py_kwargs = "**")]
    #[text_signature = "($self, graph, *, verbose)"]
    /// Return numpy embedding with Weighted SPINE node embedding.
    ///
    /// Do note that the embedding is returned transposed.
    /// 
    /// Parameters
    /// ---------
    /// graph: Graph
    ///     The graph to embed.
    /// verbose: Optional[bool] = True
    ///     Whether to show the loading bar.
    fn fit_transform(
        &self,
        graph: &Graph,
        py_kwargs: Option<&PyDict>,
    ) -> PyResult<Py<PyArray2<f32>>> {
        let gil = pyo3::Python::acquire_gil();

        let py = pyo3::Python::acquire_gil();
        let kwargs = normalize_kwargs!(py_kwargs, py.python());

        pe!(validate_kwargs(kwargs, &["verbose"]))?;

        let rows_number = graph.inner.get_nodes_number() as usize;
        let columns_number = self.inner.get_embedding_size();
        let embedding = PyArray2::new(gil.python(), [columns_number, rows_number], false);

        let embedding_slice = unsafe { embedding.as_slice_mut().unwrap() };

        // We always use the racing version of the fit transform
        // as we generally do not care about memory collisions.
        pe!(self.inner.fit_transform(
            &graph.inner,
            embedding_slice,
            extract_value_rust_result!(kwargs, "verbose", bool),
        ))?;

        Ok(embedding.into_py(gil.python()))
    }
}
