use super::*;
use cpu_models::get_edge_embedding_method_name_from_string;
use numpy::PyArray2;

///
#[pyclass]
#[derive(Debug, Clone)]
#[text_signature = "(*, edge_embedding_method_name, number_of_epochs, number_of_edges_per_mini_batch, sample_only_edges_with_heterogeneous_node_types, learning_rate, random_state)"]
pub struct EdgePredictionPerceptron {
    pub inner: cpu_models::EdgePredictionPerceptron,
}

impl From<cpu_models::EdgePredictionPerceptron> for EdgePredictionPerceptron {
    fn from(val: cpu_models::EdgePredictionPerceptron) -> EdgePredictionPerceptron {
        EdgePredictionPerceptron { inner: val }
    }
}

impl From<EdgePredictionPerceptron> for cpu_models::EdgePredictionPerceptron {
    fn from(val: EdgePredictionPerceptron) -> cpu_models::EdgePredictionPerceptron {
        val.inner
    }
}

#[pymethods]
impl EdgePredictionPerceptron {
    #[new]
    #[args(py_kwargs = "**")]
    /// Return a new instance of the EdgePredictionPerceptron model.
    ///
    /// Parameters
    /// ------------------------
    /// edge_embedding_method_name: str
    ///     The embedding method to use. By default the cosine similarity is used.
    /// number_of_epochs: int = 100
    ///     The number of epochs to train the model for. By default, 100.
    /// number_of_edges_per_mini_batch: int = 1024
    ///     The number of samples to include for each mini-batch. By default 1024.
    /// sample_only_edges_with_heterogeneous_node_types: bool = False
    ///     Whether to sample negative edges only with source and
    ///     destination nodes that have different node types. By default false.
    /// learning_rate: float = 0.001
    ///     Learning rate to use while training the model. By default 0.001.
    /// random_state: int = 42
    ///     The random state to reproduce the model initialization and training. By default, 42.
    pub fn new(py_kwargs: Option<&PyDict>) -> PyResult<EdgePredictionPerceptron> {
        let py = pyo3::Python::acquire_gil();
        let kwargs = normalize_kwargs!(py_kwargs, py.python());

        pe!(validate_kwargs(
            kwargs,
            &[
                "edge_embedding_method_name",
                "number_of_epochs",
                "number_of_edges_per_mini_batch",
                "sample_only_edges_with_heterogeneous_node_types",
                "learning_rate",
                "random_state"
            ]
        ))?;

        Ok(Self {
            inner: pe!(cpu_models::EdgePredictionPerceptron::new(
                pe!(
                    extract_value_rust_result!(kwargs, "edge_embedding_method_name", String)
                        .map(|name| get_edge_embedding_method_name_from_string(&name))
                        .transpose()
                )?,
                extract_value_rust_result!(kwargs, "number_of_epochs", usize),
                extract_value_rust_result!(kwargs, "number_of_edges_per_mini_batch", usize),
                extract_value_rust_result!(
                    kwargs,
                    "sample_only_edges_with_heterogeneous_node_types",
                    bool
                ),
                extract_value_rust_result!(kwargs, "learning_rate", f32),
                extract_value_rust_result!(kwargs, "random_state", u64),
            ))?,
        })
    }
}

#[pymethods]
impl EdgePredictionPerceptron {
    #[args(py_kwargs = "**")]
    #[text_signature = "($self, graph, node_features, verbose, support, graph_to_avoid)"]
    /// Fit the current model instance with the provided graph and node features.
    ///
    /// Parameters
    /// ---------
    /// graph: Graph
    ///     The graph whose edges are to be learned.
    /// node_features: np.ndarray
    ///     A node features numpy array.
    /// verbose: bool = True
    ///     Whether to show a loading bar for the epochs. By default, True.
    /// support: Optional[Graph] = None
    ///     Graph to use to check for false negatives.
    /// graph_to_avoid: Optional[Graph] = None
    ///     The graph whose edges are to be avoided during the generation of false negatives,
    fn fit(
        &mut self,
        graph: &Graph,
        node_features: Py<PyArray2<f32>>,
        verbose: Option<bool>,
        support: Option<&Graph>,
        graph_to_avoid: Option<&Graph>,
    ) -> PyResult<()> {
        let gil = pyo3::Python::acquire_gil();

        let support = support.map(|support| &support.inner);
        let graph_to_avoid = graph_to_avoid.map(|graph_to_avoid| &graph_to_avoid.inner);
        let node_features = node_features.as_ref(gil.python());
        let node_features_ref = unsafe { node_features.as_slice().unwrap() };

        pe!(self.inner.fit(
            &graph.inner,
            node_features_ref,
            node_features.shape()[1],
            verbose,
            support,
            graph_to_avoid
        ))
    }

    #[args(py_kwargs = "**")]
    #[text_signature = "($self, graph, node_features)"]
    /// Return numpy array with edge predictions for provided graph.
    ///
    /// Parameters
    /// ---------
    /// graph: Graph
    ///     The graph whose edges are to be predicted.
    /// node_features: np.ndarray
    ///     A node features numpy array.
    fn predict(
        &mut self,
        graph: &Graph,
        node_features: Py<PyArray2<f32>>,
    ) -> PyResult<Py<PyArray1<f32>>> {
        let gil = pyo3::Python::acquire_gil();

        let node_features = node_features.as_ref(gil.python());
        let node_features_ref = unsafe { node_features.as_slice().unwrap() };
        let similarities = PyArray1::new(
            gil.python(),
            [graph.get_number_of_directed_edges() as usize],
            false,
        );
        let similarities_ref = unsafe { similarities.as_slice_mut().unwrap() };

        pe!(self.inner.predict(
            similarities_ref,
            &graph.inner,
            node_features_ref,
            node_features.shape()[1],
        ))?;

        Ok(similarities.to_owned())
    }
}
