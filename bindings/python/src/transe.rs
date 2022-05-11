use super::*;
use numpy::PyArray2;

///
#[pyclass]
#[derive(Debug, Clone)]
#[text_signature = "(*, embedding_size, renormalize, random_state)"]
pub struct TransE {
    pub inner: cpu_models::TransE,
}

impl From<cpu_models::TransE> for TransE {
    fn from(val: cpu_models::TransE) -> TransE {
        TransE { inner: val }
    }
}

impl From<TransE> for cpu_models::TransE {
    fn from(val: TransE) -> cpu_models::TransE {
        val.inner
    }
}

#[pymethods]
impl TransE {
    #[new]
    #[args(py_kwargs = "**")]
    /// Return a new instance of the TransE model.
    ///
    /// Parameters
    /// ------------------------
    /// embedding_size: Optional[int] = 100
    ///     Size of the embedding.
    /// renormalize: bool = True
    ///     Whether to renormalize at each loop, by default true.
    /// random_state: int = 42
    ///     random_state to use to reproduce the walks.
    pub fn new(py_kwargs: Option<&PyDict>) -> PyResult<TransE> {
        let py = pyo3::Python::acquire_gil();
        let kwargs = normalize_kwargs!(py_kwargs, py.python());

        pe!(validate_kwargs(
            kwargs,
            &["embedding_size", "renormalize", "random_state",]
        ))?;

        Ok(Self {
            inner: pe!(cpu_models::TransE::new(
                extract_value_rust_result!(kwargs, "embedding_size", usize),
                extract_value_rust_result!(kwargs, "renormalize", bool),
                extract_value_rust_result!(kwargs, "random_state", u64),
            ))?,
        })
    }
}

#[pymethods]
impl TransE {
    #[args(py_kwargs = "**")]
    #[text_signature = "($self, graph, *, epochs, learning_rate, learning_rate_decay, verbose)"]
    /// Return numpy embedding with TransE node embedding.
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
    ) -> PyResult<(Py<PyArray2<f32>>, Py<PyArray2<f32>>)> {
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
            [graph.inner.get_nodes_number() as usize, columns_number],
            false,
        );
        let edge_type_embedding = PyArray2::zeros(
            gil.python(),
            [
                pe!(graph.inner.get_edge_types_number())? as usize,
                columns_number,
            ],
            false,
        );

        let node_embedding_slice = unsafe { node_embedding.as_slice_mut().unwrap() };
        let edge_type_embedding_slice = unsafe { edge_type_embedding.as_slice_mut().unwrap() };

        // We always use the racing version of the fit transform
        // as we generally do not care about memory collisions.
        pe!(self.inner.fit_transform(
            &graph.inner,
            node_embedding_slice,
            edge_type_embedding_slice,
            extract_value_rust_result!(kwargs, "epochs", usize),
            extract_value_rust_result!(kwargs, "learning_rate", f32),
            extract_value_rust_result!(kwargs, "learning_rate_decay", f32),
            extract_value_rust_result!(kwargs, "verbose", bool),
        ))?;

        Ok((
            node_embedding.into_py(gil.python()),
            edge_type_embedding.into_py(gil.python())
        ))
    }
}
