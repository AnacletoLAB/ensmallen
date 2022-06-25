use super::*;
use numpy::PyArray2;

///
#[pyclass]
#[derive(Debug, Clone)]
#[text_signature = "(*, embedding_size, window_size, clipping_value, walk_length, return_weight, explore_weight, change_edge_type_weight, change_node_type_weight, max_neighbours, random_state, iterations, normalize_by_degree)"]
pub struct GloVe {
    pub inner: cpu_models::GloVe,
}

impl From<cpu_models::GloVe> for GloVe {
    fn from(val: cpu_models::GloVe) -> GloVe {
        GloVe { inner: val }
    }
}

impl From<GloVe> for cpu_models::GloVe {
    fn from(val: GloVe) -> cpu_models::GloVe {
        val.inner
    }
}

#[pymethods]
impl GloVe {
    #[new]
    #[args(py_kwargs = "**")]
    /// Return a new instance of the GloVe model.
    ///
    /// Parameters
    /// ------------------------
    /// embedding_size: Optional[int] = 100
    ///     Size of the embedding.
    /// window_size: Optional[int] = 10
    ///     Window size defining the contexts.
    /// clipping_value: Optional[float] = 6.0
    ///     Value at which we clip the dot product, mostly for numerical stability issues.
    ///     By default, `6.0`, where the loss is already close to zero.
    /// alpha: f64 = 0.75
    ///     Alpha to use for the loss.
    /// walk_length: Optional[int] = 32
    ///     Maximal length of the random walk.
    ///     On graphs without traps, all walks have this length.
    /// return_weight: float = 1.0
    ///     Weight on the probability of returning to node coming from
    ///     Having this higher tends the walks to be
    ///     more like a Breadth-First Search.
    ///     Having this very high  (> 2) makes search very local.
    ///     Equal to the inverse of p in the Node2Vec paper.
    /// explore_weight: float = 1.0
    ///     Weight on the probability of visiting a neighbor node
    ///     to the one we're coming from in the random walk
    ///     Having this higher tends the walks to be
    ///     more like a Depth-First Search.
    ///     Having this very high makes search more outward.
    ///     Having this very low makes search very local.
    ///     Equal to the inverse of q in the Node2Vec paper.
    /// change_edge_type_weight: float = 1.0
    ///     Weight on the probability of visiting a neighbor node of a
    ///     different type than the previous node. This only applies to
    ///     colored graphs, otherwise it has no impact.
    /// change_node_type_weight: float = 1.0
    ///     Weight on the probability of visiting a neighbor edge of a
    ///     different type than the previous edge. This only applies to
    ///     multigraphs, otherwise it has no impact.
    /// random_state: int = 42
    ///     random_state to use to reproduce the walks.
    /// iterations: int = 1
    ///     Number of cycles on the graphs to execute.
    /// max_neighbours: Optional[int] = 100
    ///     Maximum number of randomly sampled neighbours to consider.
    ///     If this parameter is used, the walks becomes probabilistic in nature
    ///     and becomes an approximation of an exact walk.
    /// normalize_by_degree: Optional[bool] = False
    ///     Whether to normalize the random walks by the node degree.
    pub fn new(py_kwargs: Option<&PyDict>) -> PyResult<GloVe> {
        let py = pyo3::Python::acquire_gil();
        let kwargs = normalize_kwargs!(py_kwargs, py.python());

        pe!(validate_kwargs(
            kwargs,
            build_walk_parameters_list(&[
                "embedding_size",
                "window_size",
                "clipping_value",
                "alpha",
            ])
            .as_slice()
        ))?;

        let parameters = pe!(build_walk_parameters(kwargs))?;

        Ok(Self {
            inner: pe!(cpu_models::GloVe::new(
                extract_value_rust_result!(kwargs, "embedding_size", usize),
                Some(parameters),
                extract_value_rust_result!(kwargs, "window_size", usize),
                extract_value_rust_result!(kwargs, "clipping_value", f32),
                extract_value_rust_result!(kwargs, "alpha", f32),
            ))?,
        })
    }
}

#[pymethods]
impl GloVe {
    #[args(py_kwargs = "**")]
    #[text_signature = "($self, graph, *, epochs, learning_rate, learning_rate_decay, verbose)"]
    /// Return numpy embedding with GloVe node embedding.
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

        let kwargs = normalize_kwargs!(py_kwargs, gil.python());

        pe!(validate_kwargs(
            kwargs,
            &["epochs", "learning_rate", "learning_rate_decay", "verbose"]
        ))?;

        let rows_number = graph.inner.get_number_of_nodes() as usize;
        let columns_number = self.inner.get_embedding_size();
        let embedding = PyArray2::new(gil.python(), [rows_number, columns_number], false);

        let embedding_slice = unsafe { embedding.as_slice_mut().unwrap() };

        // We always use the racing version of the fit transform
        // as we generally do not care about memory collisions.
        pe!(self.inner.fit_transform(
            &graph.inner,
            embedding_slice,
            extract_value_rust_result!(kwargs, "epochs", usize),
            extract_value_rust_result!(kwargs, "learning_rate", f32),
            extract_value_rust_result!(kwargs, "learning_rate_decay", f32),
            extract_value_rust_result!(kwargs, "verbose", bool),
        ))?;

        Ok(embedding.into_py(gil.python()))
    }
}
