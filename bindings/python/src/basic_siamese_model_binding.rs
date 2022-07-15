use super::*;
use cpu_models::{BasicSiameseModel, BasicEmbeddingModel};

#[derive(Debug, Clone)]
pub struct BasicSiameseModelBinding<M>
where
    M: From<BasicSiameseModel> + cpu_models::GraphEmbedder,
{
    inner: M,
}

impl<M> From<BasicSiameseModel> for BasicSiameseModelBinding<M>
where
    M: From<BasicSiameseModel> + cpu_models::GraphEmbedder,
{
    fn from(model: BasicSiameseModel) -> Self {
        Self {
            inner: model.into(),
        }
    }
}

impl<M> GraphEmbedderBinding<M> for BasicSiameseModelBinding<M>
where
    M: From<BasicSiameseModel> + cpu_models::GraphEmbedder,
{
    fn get_model(&self) -> &M {
        &self.inner
    }
}

impl<M> FromPyDict for BasicSiameseModelBinding<M>
where
    M: From<BasicSiameseModel> + cpu_models::GraphEmbedder,
{
    fn from_pydict(py_kwargs: Option<&PyDict>) -> PyResult<Self> {
        let py = pyo3::Python::acquire_gil();
        let kwargs = normalize_kwargs!(py_kwargs, py.python());

        pe!(validate_kwargs(
            kwargs,
            &[
                "relu_bias",
                "embedding_size",
                "epochs",
                "learning_rate",
                "learning_rate_decay",
                "random_state",
                "verbose",
            ]
        ))?;

        Ok(pe!(BasicSiameseModel::new(
            pe!(BasicEmbeddingModel::new(
                extract_value_rust_result!(kwargs, "embedding_size", usize),
                extract_value_rust_result!(kwargs, "epochs", usize),
                extract_value_rust_result!(kwargs, "learning_rate", f32),
                extract_value_rust_result!(kwargs, "learning_rate_decay", f32),
                extract_value_rust_result!(kwargs, "random_state", u64),
                extract_value_rust_result!(kwargs, "verbose", bool),
            ))?,
            extract_value_rust_result!(kwargs, "relu_bias", f32),
        ))?
        .into())
    }
}


#[pyclass]
#[derive(Debug, Clone)]
#[text_signature = "(*, relu_bias, embedding_size, epochs, learning_rate, learning_rate_decay, random_state, verbose)"]
pub struct TransE {
    pub inner: BasicSiameseModelBinding<cpu_models::TransE>,
}

#[pymethods]
impl TransE {
    #[new]
    #[args(py_kwargs = "**")]
    /// Return a new instance of the TransE model.
    ///
    /// Parameters
    /// ------------------------
    /// relu_bias: Optional[float] = 1.0
    ///     The bias to apply to the relu. By default, 1.0.
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
    pub fn new(py_kwargs: Option<&PyDict>) -> PyResult<TransE> {
        Ok(Self {
            inner: BasicSiameseModelBinding::from_pydict(py_kwargs)?,
        })
    }
}

#[pymethods]
impl TransE {
    #[args(py_kwargs = "**")]
    #[text_signature = "($self, graph)"]
    /// Return numpy embedding with TransE node embedding.
    ///
    /// Parameters
    /// ---------
    /// graph: Graph
    ///     The graph to embed.
    fn fit_transform(&self, graph: &Graph) -> PyResult<Py<PyArray2<f32>>> {
        Ok(self.inner.fit_transform(graph)?.first().unwrap().to_owned())
    }
}

#[pyclass]
#[derive(Debug, Clone)]
#[text_signature = "(*, relu_bias, embedding_size, epochs, learning_rate, learning_rate_decay, random_state, verbose)"]
pub struct Unstructured {
    pub inner: BasicSiameseModelBinding<cpu_models::Unstructured>,
}

#[pymethods]
impl Unstructured {
    #[new]
    #[args(py_kwargs = "**")]
    /// Return a new instance of the Unstructured model.
    ///
    /// Parameters
    /// ------------------------
    /// relu_bias: Optional[float] = 1.0
    ///     The bias to apply to the relu. By default, 1.0.
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
    pub fn new(py_kwargs: Option<&PyDict>) -> PyResult<Unstructured> {
        Ok(Self {
            inner: BasicSiameseModelBinding::from_pydict(py_kwargs)?,
        })
    }
}

#[pymethods]
impl Unstructured {
    #[args(py_kwargs = "**")]
    #[text_signature = "($self, graph)"]
    /// Return numpy embedding with Unstructured node embedding.
    ///
    /// Parameters
    /// ---------
    /// graph: Graph
    ///     The graph to embed.
    fn fit_transform(&self, graph: &Graph) -> PyResult<Py<PyArray2<f32>>> {
        Ok(self.inner.fit_transform(graph)?.first().unwrap().to_owned())
    }
}

#[pyclass]
#[derive(Debug, Clone)]
#[text_signature = "(*, relu_bias, embedding_size, epochs, learning_rate, learning_rate_decay, random_state, verbose)"]
pub struct TransH {
    pub inner: BasicSiameseModelBinding<cpu_models::TransH>,
}

#[pymethods]
impl TransH {
    #[new]
    #[args(py_kwargs = "**")]
    /// Return a new instance of the TransH model.
    ///
    /// Parameters
    /// ------------------------
    /// relu_bias: Optional[float] = 1.0
    ///     The bias to apply to the relu. By default, 1.0.
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
    pub fn new(py_kwargs: Option<&PyDict>) -> PyResult<TransH> {
        Ok(Self {
            inner: BasicSiameseModelBinding::from_pydict(py_kwargs)?,
        })
    }
}

#[pymethods]
impl TransH {
    #[args(py_kwargs = "**")]
    #[text_signature = "($self, graph)"]
    /// Return numpy embedding with TransH node embedding.
    ///
    /// Parameters
    /// ---------
    /// graph: Graph
    ///     The graph to embed.
    fn fit_transform(&self, graph: &Graph) -> PyResult<Py<PyArray2<f32>>> {
        Ok(self.inner.fit_transform(graph)?.first().unwrap().to_owned())
    }
}