use super::*;
use cpu_models::BasicEmbeddingModel;

#[derive(Debug, Clone)]
pub struct BasicEmbeddingModelBinding<M>
where
    M: From<BasicEmbeddingModel> + cpu_models::GraphEmbedder,
{
    inner: M,
}

impl<M> From<BasicEmbeddingModel> for BasicEmbeddingModelBinding<M>
where
    M: From<BasicEmbeddingModel> + cpu_models::GraphEmbedder,
{
    fn from(model: BasicEmbeddingModel) -> Self {
        Self {
            inner: model.into(),
        }
    }
}

impl<M> GraphEmbedderBinding<M> for BasicEmbeddingModelBinding<M>
where
    M: From<BasicEmbeddingModel> + cpu_models::GraphEmbedder,
{
    fn get_model(&self) -> &M {
        &self.inner
    }
}

impl<M> FromPyDict for BasicEmbeddingModelBinding<M>
where
    M: From<BasicEmbeddingModel> + cpu_models::GraphEmbedder,
{
    fn from_pydict(py_kwargs: Option<&PyDict>) -> PyResult<Self> {
        let py = pyo3::Python::acquire_gil();
        let kwargs = normalize_kwargs!(py_kwargs, py.python());

        pe!(validate_kwargs(
            kwargs,
            &[
                "embedding_size",
                "epochs",
                "learning_rate",
                "learning_rate_decay",
                "random_state",
                "verbose",
            ]
        ))?;

        Ok(pe!(BasicEmbeddingModel::new(
            extract_value_rust_result!(kwargs, "embedding_size", usize),
            extract_value_rust_result!(kwargs, "epochs", usize),
            extract_value_rust_result!(kwargs, "learning_rate", f32),
            extract_value_rust_result!(kwargs, "learning_rate_decay", f32),
            extract_value_rust_result!(kwargs, "random_state", u64),
            extract_value_rust_result!(kwargs, "verbose", bool),
        ))?
        .into())
    }
}

#[pyclass]
#[derive(Debug, Clone)]
#[pyo3(text_signature = "(*, embedding_size, epochs, learning_rate, learning_rate_decay, random_state, verbose)")]
pub struct FirstOrderLINE {
    pub inner: BasicEmbeddingModelBinding<cpu_models::FirstOrderLINE>,
}

#[pymethods]
impl FirstOrderLINE {
    #[new]
    #[args(py_kwargs = "**")]
    /// Return a new instance of the FirstOrderLINE model.
    ///
    /// Parameters
    /// ------------------------
    /// embedding_size: Optional[int] = 100
    ///     Size of the embedding.
    /// epochs: int = 100
    ///     Number of epochs to train the model for.
    /// learning_rate: float = 0.01
    ///     Learning rate of the model.
    /// learning_rate_decay: float = 0.9
    ///     Amount of learning rate decay for each epoch.
    /// random_state: int = 42
    ///     random_state to use to reproduce the walks.
    /// verbose: bool = True
    ///     Whether to show the loading bar.
    pub fn new(py_kwargs: Option<&PyDict>) -> PyResult<FirstOrderLINE> {
        Ok(Self {
            inner: BasicEmbeddingModelBinding::from_pydict(py_kwargs)?,
        })
    }
}

#[pymethods]
impl FirstOrderLINE {
    #[args(py_kwargs = "**")]
    #[pyo3(text_signature = "($self, graph)")]
    /// Return numpy embedding with FirstOrderLINE node embedding.
    ///
    /// Parameters
    /// ---------
    /// graph: Graph
    ///     The graph to embed.
    fn fit_transform(&self, graph: &Graph) -> PyResult<Py<PyAny>> {
        Ok(self.inner.fit_transform(graph)?.first().unwrap().to_owned())
    }
}

#[pyclass]
#[derive(Debug, Clone)]
#[pyo3(text_signature = "(*, embedding_size, epochs, learning_rate, learning_rate_decay, random_state, verbose)")]
pub struct SecondOrderLINE {
    pub inner: BasicEmbeddingModelBinding<cpu_models::SecondOrderLINE>,
}

#[pymethods]
impl SecondOrderLINE {
    #[new]
    #[args(py_kwargs = "**")]
    /// Return a new instance of the SecondOrderLINE model.
    ///
    /// Parameters
    /// ------------------------
    /// embedding_size: Optional[int] = 100
    ///     Size of the embedding.
    /// epochs: int = 100
    ///     Number of epochs to train the model for.
    /// learning_rate: float = 0.01
    ///     Learning rate of the model.
    /// learning_rate_decay: float = 0.9
    ///     Amount of learning rate decay for each epoch.
    /// random_state: int = 42
    ///     random_state to use to reproduce the walks.
    /// verbose: bool = True
    ///     Whether to show the loading bar.
    pub fn new(py_kwargs: Option<&PyDict>) -> PyResult<SecondOrderLINE> {
        Ok(Self {
            inner: BasicEmbeddingModelBinding::from_pydict(py_kwargs)?,
        })
    }
}

#[pymethods]
impl SecondOrderLINE {
    #[args(py_kwargs = "**")]
    #[pyo3(text_signature = "($self, graph)")]
    /// Return numpy embedding with SecondOrderLINE node embedding.
    ///
    /// Parameters
    /// ---------
    /// graph: Graph
    ///     The graph to embed.
    fn fit_transform(&self, graph: &Graph) -> PyResult<Py<PyAny>> {
        Ok(self.inner.fit_transform(graph)?.first().unwrap().to_owned())
    }
}
