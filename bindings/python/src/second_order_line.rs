use super::*;
use numpy::PyArray2;

///
#[pyclass]
#[derive(Debug, Clone)]
#[text_signature = "(*, embedding_size, random_state)"]
pub struct SecondOrderLINE {
    pub inner: cpu_models::SecondOrderLINE,
}

impl From<cpu_models::SecondOrderLINE> for SecondOrderLINE {
    fn from(val: cpu_models::SecondOrderLINE) -> SecondOrderLINE {
        SecondOrderLINE { inner: val }
    }
}

impl From<SecondOrderLINE> for cpu_models::SecondOrderLINE {
    fn from(val: SecondOrderLINE) -> cpu_models::SecondOrderLINE {
        val.inner
    }
}

#[pymethods]
impl SecondOrderLINE {
    #[new]
    #[args(py_kwargs = "**")]
    /// Return a new instance of the second-order LINE model.
    ///
    /// Parameters
    /// ------------------------
    /// embedding_size: Optional[int] = 100
    ///     Size of the embedding.
    /// random_state: int = 42
    ///     random_state to use to reproduce the walks.
    pub fn new(py_kwargs: Option<&PyDict>) -> PyResult<SecondOrderLINE> {
        let py = pyo3::Python::acquire_gil();
        let kwargs = normalize_kwargs!(py_kwargs, py.python());

        pe!(validate_kwargs(
            kwargs,
            &["embedding_size", "random_state",]
        ))?;

        Ok(Self {
            inner: pe!(cpu_models::SecondOrderLINE::new(
                extract_value_rust_result!(kwargs, "embedding_size", usize),
                extract_value_rust_result!(kwargs, "random_state", u64),
            ))?,
        })
    }
}

#[pymethods]
impl SecondOrderLINE {
    #[args(py_kwargs = "**")]
    #[text_signature = "($self, graph, *, epochs, learning_rate, learning_rate_decay, verbose)"]
    /// Return numpy embedding with SecondOrderLINE node embedding.
    ///
    /// Parameters
    /// ---------
    /// graph: Graph
    ///     The graph to embed.
    /// epochs: Optional[int] = 10
    ///     How many epochs the model will train for.
    ///     In this context an epoch means that the model will compute a random
    ///     walk starting from every node in the graph.
    /// learning_rate: Optional[float] = 0.01
    ///     The learning rate to update the gradient.
    /// learning_rate_decay: Optional[float] = 0.9
    ///     Factor to reduce the learning rate for at each epoch. By default 0.9.
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

        pe!(validate_kwargs(
            kwargs,
            &["epochs", "learning_rate", "learning_rate_decay", "verbose"]
        ))?;

        let columns_number = self.inner.get_embedding_size();
        let node_embedding = PyArray2::zeros(
            gil.python(),
            [graph.inner.get_number_of_nodes() as usize, columns_number],
            false,
        );

        let node_embedding_slice = unsafe { node_embedding.as_slice_mut().unwrap() };

        // We always use the racing version of the fit transform
        // as we generally do not care about memory collisions.
        pe!(self.inner.fit_transform(
            &graph.inner,
            node_embedding_slice,
            extract_value_rust_result!(kwargs, "epochs", usize),
            extract_value_rust_result!(kwargs, "learning_rate", f32),
            extract_value_rust_result!(kwargs, "learning_rate_decay", f32),
            extract_value_rust_result!(kwargs, "verbose", bool),
        ))?;

        Ok(node_embedding.into_py(gil.python()))
    }
}
