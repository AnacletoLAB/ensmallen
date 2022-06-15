use super::*;
use cpu_models::EdgeFeatureName;
use std::convert::TryFrom;

///
#[pyclass]
#[derive(Debug, Clone)]
#[text_signature = "(*, edge_feature_name, number_of_epochs, number_of_edges_per_mini_batch, sample_only_edges_with_heterogeneous_node_types, learning_rate, random_state)"]
pub struct EdgePredictionFeaturePerceptron {
    pub inner: cpu_models::EdgePredictionFeaturePerceptron,
}

impl From<cpu_models::EdgePredictionFeaturePerceptron> for EdgePredictionFeaturePerceptron {
    fn from(val: cpu_models::EdgePredictionFeaturePerceptron) -> EdgePredictionFeaturePerceptron {
        EdgePredictionFeaturePerceptron { inner: val }
    }
}

impl From<EdgePredictionFeaturePerceptron> for cpu_models::EdgePredictionFeaturePerceptron {
    fn from(val: EdgePredictionFeaturePerceptron) -> cpu_models::EdgePredictionFeaturePerceptron {
        val.inner
    }
}

#[pymethods]
impl EdgePredictionFeaturePerceptron {
    #[new]
    #[args(py_kwargs = "**")]
    /// Return a new instance of the EdgePredictionFeaturePerceptron model.
    ///
    /// Parameters
    /// ------------------------
    /// edge_feature_name: str
    ///     The edge feature name to use. By default the source and destination node degrees is used.
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
    pub fn new(py_kwargs: Option<&PyDict>) -> PyResult<EdgePredictionFeaturePerceptron> {
        let py = pyo3::Python::acquire_gil();
        let kwargs = normalize_kwargs!(py_kwargs, py.python());

        pe!(validate_kwargs(
            kwargs,
            &[
                "edge_feature_name",
                "number_of_epochs",
                "number_of_edges_per_mini_batch",
                "sample_only_edges_with_heterogeneous_node_types",
                "learning_rate",
                "random_state"
            ]
        ))?;

        Ok(Self {
            inner: pe!(cpu_models::EdgePredictionFeaturePerceptron::new(
                pe!(
                    extract_value_rust_result!(kwargs, "edge_feature_name", String)
                        .map(|name| EdgeFeatureName::try_from(name.as_str()))
                        .transpose()
                )?,
                extract_value_rust_result!(kwargs, "number_of_epochs", usize),
                extract_value_rust_result!(kwargs, "number_of_edges_per_mini_batch", usize),
                extract_value_rust_result!(
                    kwargs,
                    "sample_only_edges_with_heterogeneous_node_types",
                    bool
                ),
                extract_value_rust_result!(kwargs, "learning_rate", f64),
                extract_value_rust_result!(kwargs, "random_state", u64),
            ))?,
        })
    }
}

#[pymethods]
impl EdgePredictionFeaturePerceptron {
    #[args(py_kwargs = "**")]
    #[text_signature = "($self, graph, verbose, support, graph_to_avoid)"]
    /// Fit the current model instance with the provided graph and node features.
    ///
    /// Parameters
    /// ---------
    /// graph: Graph
    ///     The graph whose edges are to be learned.
    /// verbose: bool = True
    ///     Whether to show a loading bar for the epochs. By default, True.
    /// support: Optional[Graph] = None
    ///     Graph to use to check for false negatives and for the edge features. When one is not provided, the `graph` parameter is used.
    /// graph_to_avoid: Optional[Graph] = None
    ///     The graph whose edges are to be avoided during the generation of false negatives,
    fn fit(
        &mut self,
        graph: &Graph,
        verbose: Option<bool>,
        support: Option<&Graph>,
        graph_to_avoid: Option<&Graph>,
    ) -> PyResult<()> {
        let support = support.map(|support| &support.inner);
        let graph_to_avoid = graph_to_avoid.map(|graph_to_avoid| &graph_to_avoid.inner);

        pe!(self
            .inner
            .fit(&graph.inner, verbose, support, graph_to_avoid))
    }

    #[text_signature = "($self)"]
    /// Returns the weights of the model.
    fn get_weights(&self) -> PyResult<Py<PyArray1<f64>>> {
        let gil = pyo3::Python::acquire_gil();
        Ok(to_ndarray_1d!(gil, pe!(self.inner.get_weights())?, f64))
    }

    #[text_signature = "($self)"]
    /// Returns the bias of the model.
    fn get_bias(&self) -> PyResult<f64> {
        pe!(self.inner.get_bias())
    }

    #[args(py_kwargs = "**")]
    #[text_signature = "($self, graph, support)"]
    /// Return numpy array with edge predictions for provided graph.
    ///
    /// Parameters
    /// ---------
    /// graph: Graph
    ///     The graph whose edges are to be predicted.
    /// support: Optional[Graph] = None
    ///     Graph to use to check for false negatives and for the edge features. When one is not provided, the `graph` parameter is used.
    fn predict(&self, graph: &Graph, support: Option<&Graph>) -> PyResult<Py<PyArray1<f32>>> {
        let gil = pyo3::Python::acquire_gil();
        let support = support.map(|support| &support.inner);

        let predictions = PyArray1::new(
            gil.python(),
            [graph.get_number_of_directed_edges() as usize],
            false,
        );
        let predictions_ref = unsafe { predictions.as_slice_mut().unwrap() };

        pe!(self.inner.predict(predictions_ref, &graph.inner, support,))?;

        Ok(predictions.to_owned())
    }
}
