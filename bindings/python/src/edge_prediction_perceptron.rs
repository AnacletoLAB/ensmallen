use super::*;
use numpy::PyArray2;
use std::convert::TryInto;

///
#[pyclass]
#[derive(Debug, Clone)]
#[text_signature = "(*, edge_embeddings, edge_features, cooccurrence_iterations, cooccurrence_window_size, number_of_epochs, number_of_edges_per_mini_batch, sample_only_edges_with_heterogeneous_node_types, learning_rate, learning_rate_decay, random_state)"]
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
    /// edge_embeddings: List[str]
    ///     The embedding methods to use for the provided node features.
    /// edge_features: List[str]
    ///     The edge features to compute for each edge.
    /// cooccurrence_iterations: int = 100
    ///     Number of iterations to run when computing the cooccurrence metric.
    ///     By default 100.
    /// cooccurrence_window_size: int = 10
    ///     Window size to consider to measure the cooccurrence.
    ///     By default 100.
    /// number_of_epochs: int = 100
    ///     The number of epochs to train the model for. By default, 100.
    /// number_of_edges_per_mini_batch: int = 4096
    ///     The number of samples to include for each mini-batch.
    ///     By default 4096.
    /// sample_only_edges_with_heterogeneous_node_types: bool = False
    ///     Whether to sample negative edges only with source and
    ///     destination nodes that have different node types. By default false.
    /// learning_rate: float = 0.001
    ///     Learning rate to use while training the model.
    ///     By default 0.001.
    /// learning_rate_decay: float = 0.99
    ///     Learning rate decay to use while training the model.
    ///     By default 0.99.
    /// random_state: int = 42
    ///     The random state to reproduce the model initialization and training. By default, 42.
    pub fn new(py_kwargs: Option<&PyDict>) -> PyResult<EdgePredictionPerceptron> {
        let py = pyo3::Python::acquire_gil();
        let kwargs = normalize_kwargs!(py_kwargs, py.python());

        pe!(validate_kwargs(
            kwargs,
            &[
                "edge_embeddings",
                "edge_features",
                "cooccurrence_iterations",
                "cooccurrence_window_size",
                "number_of_epochs",
                "number_of_edges_per_mini_batch",
                "sample_only_edges_with_heterogeneous_node_types",
                "learning_rate",
                "learning_rate_decay",
                "random_state"
            ]
        ))?;

        Ok(Self {
            inner: pe!(cpu_models::EdgePredictionPerceptron::new(
                pe!(
                    extract_value_rust_result!(kwargs, "edge_embeddings", Vec<String>)
                        .map(|names| names
                            .into_iter()
                            .map(|name| name.try_into())
                            .collect::<Result<Vec<_>>>())
                        .transpose()
                )?
                .unwrap_or_else(|| Vec::new()),
                pe!(
                    extract_value_rust_result!(kwargs, "edge_features", Vec<String>)
                        .map(|names| {
                            names
                                .into_iter()
                                .map(|name| name.try_into())
                                .collect::<Result<Vec<_>>>()
                        })
                        .transpose()
                )?
                .unwrap_or_else(|| Vec::new()),
                extract_value_rust_result!(kwargs, "cooccurrence_iterations", u64),
                extract_value_rust_result!(kwargs, "cooccurrence_window_size", u64),
                extract_value_rust_result!(kwargs, "number_of_epochs", usize),
                extract_value_rust_result!(kwargs, "number_of_edges_per_mini_batch", usize),
                extract_value_rust_result!(
                    kwargs,
                    "sample_only_edges_with_heterogeneous_node_types",
                    bool
                ),
                extract_value_rust_result!(kwargs, "learning_rate", f32),
                extract_value_rust_result!(kwargs, "learning_rate_decay", f32),
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
    /// node_features: List[np.ndarray]
    ///     A list of node features numpy array.
    /// verbose: bool = True
    ///     Whether to show a loading bar for the epochs. By default, True.
    /// support: Optional[Graph] = None
    ///     Graph to use to check for false negatives.
    /// graph_to_avoid: Optional[Graph] = None
    ///     The graph whose edges are to be avoided during the generation of false negatives,
    fn fit(
        &mut self,
        graph: &Graph,
        node_features: Vec<Py<PyArray2<f32>>>,
        verbose: Option<bool>,
        support: Option<&Graph>,
        graph_to_avoid: Option<&Graph>,
    ) -> PyResult<()> {
        let gil = pyo3::Python::acquire_gil();

        let support = support.map(|support| &support.inner);
        let graph_to_avoid = graph_to_avoid.map(|graph_to_avoid| &graph_to_avoid.inner);
        let node_features = node_features
            .iter()
            .map(|node_feature| node_feature.as_ref(gil.python()))
            .collect::<Vec<_>>();
        let dimensions = node_features
            .iter()
            .map(|node_feature| node_feature.shape()[1])
            .collect::<Vec<usize>>();
        let node_features_ref = node_features
            .iter()
            .map(|node_feature| unsafe { node_feature.as_slice().unwrap() })
            .collect::<Vec<_>>();

        pe!(self.inner.fit(
            &graph.inner,
            node_features_ref.as_slice(),
            dimensions.as_slice(),
            verbose,
            support,
            graph_to_avoid
        ))
    }

    #[text_signature = "($self)"]
    /// Returns the weights of the model.
    fn get_weights(&self) -> PyResult<Py<PyArray1<f32>>> {
        let gil = pyo3::Python::acquire_gil();
        Ok(to_ndarray_1d!(gil, pe!(self.inner.get_weights())?, f32))
    }

    #[text_signature = "($self)"]
    /// Returns the bias of the model.
    fn get_bias(&self) -> PyResult<f32> {
        pe!(self.inner.get_bias())
    }

    #[args(py_kwargs = "**")]
    #[text_signature = "($self, graph, node_features)"]
    /// Return numpy array with edge predictions for provided graph.
    ///
    /// Parameters
    /// ---------
    /// graph: Graph
    ///     The graph whose edges are to be predicted.
    /// node_features: List[np.ndarray]
    ///     A node features numpy array.
    /// support: Optional[Graph] = None
    ///     Graph to use to check for false negatives.

    fn predict(
        &self,
        graph: &Graph,
        support: Option<&Graph>,
        node_features: Vec<Py<PyArray2<f32>>>,
    ) -> PyResult<Py<PyArray1<f32>>> {
        let gil = pyo3::Python::acquire_gil();

        let support = support.map(|support| &support.inner);
        let node_features = node_features
            .iter()
            .map(|node_feature| node_feature.as_ref(gil.python()))
            .collect::<Vec<_>>();
        let dimensions = node_features
            .iter()
            .map(|node_feature| node_feature.shape()[1])
            .collect::<Vec<usize>>();
        let node_features_ref = node_features
            .iter()
            .map(|node_feature| unsafe { node_feature.as_slice().unwrap() })
            .collect::<Vec<_>>();
        let predictions = PyArray1::new(
            gil.python(),
            [graph.get_number_of_directed_edges() as usize],
            false,
        );
        let predictions_ref = unsafe { predictions.as_slice_mut().unwrap() };

        pe!(self.inner.predict(
            predictions_ref,
            &graph.inner,
            node_features_ref.as_slice(),
            dimensions.as_slice(),
            support
        ))?;

        Ok(predictions.to_owned())
    }
}
