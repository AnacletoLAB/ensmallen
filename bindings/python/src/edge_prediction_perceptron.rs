use super::*;
use crate::utilities::normalize_features;
use cpu_models::Adam;
use std::convert::TryInto;

type InnerModel = cpu_models::EdgePredictionPerceptron<Adam<f32, f32>, Adam<f32, Vec<f32>>>;

///
#[pyclass]
#[derive(Clone)]
#[pyo3(
    text_signature = "(*, edge_embeddings, edge_features, cooccurrence_iterations, cooccurrence_window_size, number_of_epochs, number_of_edges_per_mini_batch, sample_only_edges_with_heterogeneous_node_types, learning_rate, first_order_decay_factor, second_order_decay_factor, avoid_false_negatives, use_scale_free_distribution, random_state)"
)]
pub struct EdgePredictionPerceptron {
    pub inner: InnerModel,
}

#[pymethods]
impl EdgePredictionPerceptron {
    #[new]
    #[args(py_kwargs = "**")]
    /// Return a new instance of the EdgePredictionPerceptron model.
    ///
    /// Parameters
    /// ------------------------
    /// edge_features: List[str] = "Jaccard"
    ///     The edge features to compute for each edge.
    /// edge_embeddings: Optional[List[str]] = None
    ///     The embedding methods to use for the provided node features.
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
    /// first_order_decay_factor: float = 0.9
    ///     First order decay factor for the first order momentum.
    ///     By default 0.9.
    /// second_order_decay_factor: float = 0.999
    ///     Second order decay factor for the second order momentum.
    ///     By default 0.999.
    /// avoid_false_negatives: bool = False
    ///     Whether to avoid sampling false negatives.
    ///     This may cause a slower training.
    /// use_scale_free_distribution: bool = True
    ///     Whether to train model using a scale free distribution for the negatives.
    /// random_state: int = 42
    ///     The random state to reproduce the model initialization and training. By default, 42.
    pub fn new(py_kwargs: Option<&PyDict>) -> PyResult<EdgePredictionPerceptron> {
        let py = pyo3::Python::acquire_gil();
        let kwargs = normalize_kwargs!(py_kwargs, py.python());

        pe!(validate_kwargs(
            kwargs,
            &[
                "edge_features",
                "edge_embeddings",
                "cooccurrence_iterations",
                "cooccurrence_window_size",
                "number_of_epochs",
                "number_of_edges_per_mini_batch",
                "sample_only_edges_with_heterogeneous_node_types",
                "learning_rate",
                "first_order_decay_factor",
                "second_order_decay_factor",
                "avoid_false_negatives",
                "use_scale_free_distribution",
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
                .unwrap_or_else(Vec::new),
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
                .unwrap_or_else(Vec::new),
                cpu_models::Adam::new(
                    extract_value_rust_result!(kwargs, "learning_rate", f32),
                    extract_value_rust_result!(kwargs, "first_order_decay_factor", f32),
                    extract_value_rust_result!(kwargs, "second_order_decay_factor", f32),
                ),
                extract_value_rust_result!(kwargs, "avoid_false_negatives", bool),
                extract_value_rust_result!(kwargs, "cooccurrence_iterations", u64),
                extract_value_rust_result!(kwargs, "cooccurrence_window_size", u64),
                extract_value_rust_result!(kwargs, "number_of_epochs", usize),
                extract_value_rust_result!(kwargs, "number_of_edges_per_mini_batch", usize),
                extract_value_rust_result!(
                    kwargs,
                    "sample_only_edges_with_heterogeneous_node_types",
                    bool
                ),
                extract_value_rust_result!(kwargs, "use_scale_free_distribution", bool),
                extract_value_rust_result!(kwargs, "random_state", u64),
            ))?,
        })
    }
}

#[pymethods]
impl EdgePredictionPerceptron {
    #[args(py_kwargs = "**")]
    #[pyo3(text_signature = "($self, graph, node_features, verbose, support, graph_to_avoid)")]
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
        node_features: Vec<Py<PyAny>>,
        verbose: Option<bool>,
        support: Option<&Graph>,
        graph_to_avoid: Option<&Graph>,
    ) -> PyResult<()> {
        let gil = pyo3::Python::acquire_gil();
        let support = support.map(|support| &support.inner);
        let graph_to_avoid = graph_to_avoid.map(|graph_to_avoid| &graph_to_avoid.inner);
        let (_numpy_references, dimensions, slices) =
            normalize_features(&gil, node_features.as_slice())?;
        pe!(self.inner.fit(
            &graph.inner,
            slices.as_slice(),
            dimensions.as_slice(),
            verbose,
            support,
            graph_to_avoid
        ))
    }

    #[pyo3(text_signature = "($self)")]
    /// Returns the weights of the model.
    fn get_weights(&self) -> PyResult<Py<PyArray1<f32>>> {
        let gil = pyo3::Python::acquire_gil();
        Ok(to_ndarray_1d!(gil, pe!(self.inner.get_weights())?, f32))
    }

    #[pyo3(text_signature = "($self)")]
    /// Returns the bias of the model.
    fn get_bias(&self) -> PyResult<f32> {
        pe!(self.inner.get_bias())
    }

    #[pyo3(text_signature = "($self)")]
    /// Returns the supported edge features.
    fn get_supported_edge_features(&self) -> Vec<String> {
        cpu_models::EdgeFeature::get_edge_feature_method_names()
    }

    #[pyo3(text_signature = "($self)")]
    /// Returns the supported edge embeddings.
    fn get_supported_edge_embeddings(&self) -> Vec<String> {
        cpu_models::EdgeEmbedding::get_edge_embedding_method_names()
    }

    #[pyo3(text_signature = "($self, graph, node_features, support)")]
    /// Return numpy array with edge predictions for provided graph.
    ///
    /// Parameters
    /// ----------------
    /// graph: Graph
    ///     The graph whose edges are to be predicted.
    /// node_features: List[np.ndarray]
    ///     A node features numpy array.
    /// support: Optional[Graph] = None
    ///     Graph to use to check for false negatives.
    fn predict(
        &self,
        graph: &Graph,
        node_features: Vec<Py<PyAny>>,
        support: Option<&Graph>,
    ) -> PyResult<Py<PyArray1<f32>>> {
        let gil = pyo3::Python::acquire_gil();
        let support = support.map(|support| &support.inner);
        let predictions = unsafe {
            PyArray1::new(
                gil.python(),
                [graph.get_number_of_directed_edges() as usize],
                false,
            )
        };
        let predictions_ref = unsafe { predictions.as_slice_mut()? };
        let (_numpy_references, dimensions, slices) =
            normalize_features(&gil, node_features.as_slice())?;

        pe!(self.inner.predict(
            predictions_ref,
            &graph.inner,
            slices.as_slice(),
            dimensions.as_slice(),
            support
        ))?;

        Ok(predictions.to_owned())
    }

    #[staticmethod]
    #[pyo3(text_signature = "(path,)")]
    /// Loads model from the provided path.
    ///
    /// Parameters
    /// ----------------
    /// path: str
    ///     Path from where to load the model.
    fn load(path: String) -> PyResult<Self> {
        Ok(EdgePredictionPerceptron {
            inner: pe!(InnerModel::load(path.as_ref()))?,
        })
    }

    #[staticmethod]
    #[pyo3(text_signature = "(json,)")]
    /// Loads model from provided JSON string.
    ///
    /// Parameters
    /// ----------------
    /// json: str
    ///     JSON string containing model metadata.
    fn loads(json: String) -> PyResult<Self> {
        Ok(EdgePredictionPerceptron {
            inner: pe!(InnerModel::loads(json.as_str()))?,
        })
    }

    #[pyo3(text_signature = "(&self, path)")]
    /// Dump model to the provided path.
    ///
    /// Parameters
    /// ----------------
    /// path: str
    ///     Path where to dump the model.
    fn dump(&self, path: String) -> PyResult<()> {
        pe!(self.inner.dump(path.as_ref()))
    }

    #[pyo3(text_signature = "(&self)")]
    /// Dumps model to JSON string.
    fn dumps(&self) -> PyResult<String> {
        pe!(self.inner.dumps())
    }
}
