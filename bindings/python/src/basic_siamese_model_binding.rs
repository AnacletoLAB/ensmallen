use super::*;
use cpu_models::{BasicEmbeddingModel, BasicSiameseModel};

#[derive(Debug, Clone)]
pub struct BasicSiameseModelBinding<M>
where
    M: From<BasicSiameseModel> + cpu_models::GraphEmbedder,
{
    inner: M,
    paths: Vec<Option<String>>,
}

impl<M> BasicSiameseModelBinding<M>
where
    M: From<BasicSiameseModel> + cpu_models::GraphEmbedder,
{
    fn add_path(&mut self, path: Option<String>) {
        self.paths.push(path);
    }
}

impl<M> From<BasicSiameseModel> for BasicSiameseModelBinding<M>
where
    M: From<BasicSiameseModel> + cpu_models::GraphEmbedder,
{
    fn from(model: BasicSiameseModel) -> Self {
        Self {
            paths: Vec::new(),
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

    fn get_paths(&self) -> Vec<Option<String>> {
        self.paths.clone()
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
                "node_embedding_path",
                "edge_type_embedding_path",
                "mult_edge_type_embedding_path",
                "bias_edge_type_embedding_path",
                "source_edge_type_embedding_path",
                "destination_edge_type_embedding_path",
                "dtype",
            ]
        ))?;

        let mut model: Self = pe!(BasicSiameseModel::new(
            pe!(BasicEmbeddingModel::new(
                extract_value_rust_result!(kwargs, "embedding_size", usize),
                extract_value_rust_result!(kwargs, "epochs", usize),
                extract_value_rust_result!(kwargs, "learning_rate", f32),
                extract_value_rust_result!(kwargs, "learning_rate_decay", f32),
                None,
                None,
                extract_value_rust_result!(kwargs, "random_state", u64),
                extract_value_rust_result!(kwargs, "dtype", String),
                extract_value_rust_result!(kwargs, "verbose", bool),
            ))?,
            extract_value_rust_result!(kwargs, "relu_bias", f32),
        ))?
        .into();
        model.add_path(extract_value_rust_result!(
            kwargs,
            "node_embedding_path",
            String
        ));
        Ok(model)
    }
}

#[pyclass]
#[derive(Debug, Clone)]
#[pyo3(
    text_signature = "(*, relu_bias, embedding_size, epochs, learning_rate, learning_rate_decay, node_embedding_path, edge_type_embedding_path, random_state, dtype, verbose)"
)]
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
    /// node_embedding_path: Optional[str] = None
    ///     Path where to mmap and store the nodes embedding.
    ///     This is necessary to embed large graphs whose embedding will not
    ///     fit into the available main memory.
    /// edge_type_embedding_path: Optional[str] = None
    ///     Path where to mmap and store the edge type embedding.
    ///     This is necessary to embed large graphs whose embedding will not
    ///     fit into the available main memory.
    /// random_state: int = 42
    ///     random_state to use to reproduce the walks.
    /// dtype: str
    ///     The data type to be employed, by default f32.
    /// verbose: bool = True
    ///     Whether to show the loading bar.
    pub fn new(py_kwargs: Option<&PyDict>) -> PyResult<TransE> {
        let mut inner = BasicSiameseModelBinding::from_pydict(py_kwargs)?;
        let py = pyo3::Python::acquire_gil();
        let kwargs = normalize_kwargs!(py_kwargs, py.python());
        inner.add_path(extract_value_rust_result!(
            kwargs,
            "edge_type_embedding_path",
            String
        ));
        Ok(Self { inner })
    }
}

#[pymethods]
impl TransE {
    #[args(py_kwargs = "**")]
    #[pyo3(text_signature = "($self, graph)")]
    /// Return numpy embedding with TransE node embedding.
    ///
    /// Parameters
    /// ---------
    /// graph: Graph
    ///     The graph to embed.
    fn fit_transform(&self, graph: &Graph) -> PyResult<Vec<Py<PyAny>>> {
        self.inner.fit_transform(graph)
    }
}

#[pyclass]
#[derive(Debug, Clone)]
#[pyo3(
    text_signature = "(*, relu_bias, embedding_size, epochs, learning_rate, learning_rate_decay, node_embedding_path, random_state, dtype, verbose)"
)]
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
    /// node_embedding_path: Optional[str] = None
    ///     Path where to mmap and store the nodes embedding.
    ///     This is necessary to embed large graphs whose embedding will not
    ///     fit into the available main memory.
    /// random_state: int = 42
    ///     random_state to use to reproduce the walks.
    /// dtype: str
    ///     The data type to be employed, by default f32.
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
    #[pyo3(text_signature = "($self, graph)")]
    /// Return numpy embedding with Unstructured node embedding.
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
#[pyo3(
    text_signature = "(*, relu_bias, embedding_size, epochs, learning_rate, learning_rate_decay, node_embedding_path, source_edge_type_embedding_path, destination_edge_type_embedding_path, random_state, dtype, verbose)"
)]
pub struct StructuredEmbedding {
    pub inner: BasicSiameseModelBinding<cpu_models::StructuredEmbedding>,
}

#[pymethods]
impl StructuredEmbedding {
    #[new]
    #[args(py_kwargs = "**")]
    /// Return a new instance of the StructuredEmbedding model.
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
    /// node_embedding_path: Optional[str] = None
    ///     Path where to mmap and store the nodes embedding.
    ///     This is necessary to embed large graphs whose embedding will not
    ///     fit into the available main memory.
    /// source_edge_type_embedding_path: Optional[str] = None
    ///     Path where to mmap and store the source edge type embedding.
    ///     This is necessary to embed large graphs whose embedding will not
    ///     fit into the available main memory.
    /// destination_edge_type_embedding_path: Optional[str] = None
    ///     Path where to mmap and store the destination edge type embedding.
    ///     This is necessary to embed large graphs whose embedding will not
    ///     fit into the available main memory.
    /// random_state: int = 42
    ///     random_state to use to reproduce the walks.
    /// dtype: str
    ///     The data type to be employed, by default f32.
    /// verbose: bool = True
    ///     Whether to show the loading bar.
    pub fn new(py_kwargs: Option<&PyDict>) -> PyResult<StructuredEmbedding> {
        let mut inner = BasicSiameseModelBinding::from_pydict(py_kwargs)?;
        let py = pyo3::Python::acquire_gil();
        let kwargs = normalize_kwargs!(py_kwargs, py.python());
        inner.add_path(extract_value_rust_result!(
            kwargs,
            "source_edge_type_embedding_path",
            String
        ));
        inner.add_path(extract_value_rust_result!(
            kwargs,
            "destination_edge_type_embedding_path",
            String
        ));
        Ok(Self { inner })
    }
}

#[pymethods]
impl StructuredEmbedding {
    #[args(py_kwargs = "**")]
    #[pyo3(text_signature = "($self, graph)")]
    /// Return numpy embedding with StructuredEmbedding node embedding.
    ///
    /// Parameters
    /// ---------
    /// graph: Graph
    ///     The graph to embed.
    fn fit_transform(&self, graph: &Graph) -> PyResult<Vec<Py<PyAny>>> {
        self.inner.fit_transform(graph)
    }
}
