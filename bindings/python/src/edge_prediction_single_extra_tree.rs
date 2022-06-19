use super::*;
use numpy::PyArray2;
use std::convert::TryInto;

///
#[pyclass]
#[derive(Debug, Clone)]
#[text_signature = "(*, edge_embedding_method_name, number_of_epochs, number_of_edges_per_mini_batch, sample_only_edges_with_heterogeneous_node_types, learning_rate, random_state)"]
pub struct EdgePredictionSingleExtraTree {
    model: NodeFeaturesBasedEdgePredictionModelBinding<
        cpu_models::EdgePredictionSingleExtraTree<u32, u16, f32>,
    >,
}

#[pymethods]
impl EdgePredictionSingleExtraTree {
    #[new]
    #[args(py_kwargs = "**")]
    /// Return a new instance of the EdgePredictionSingleExtraTree model.
    ///
    /// Parameters
    /// ------------------------
    /// metric: str = "F1Score"
    ///     The metric to use. By default, f1 score.
    /// edge_embedding_method_name: str = "CosineSimilarity"
    ///     The embedding method to use. By default the cosine similarity is used.
    /// number_of_edges_to_sample_per_tree_node: int = 2048
    ///     The number of epochs to train the model for. By default, 2048.
    /// number_of_splits_per_tree_node: int = 10
    ///     The number of samples to include for each mini-batch. By default 10.
    /// sample_only_edges_with_heterogeneous_node_types: bool = False
    ///     Whether to sample negative edges only with source and
    ///     destination nodes that have different node types. By default false.
    /// negative_edges_rate: float = 0.5
    ///     Rate of negative edges over total.
    /// depth: int = 10
    ///     Depth of tree. By default 10.
    /// random_state: int = 42
    ///     The random state to reproduce the model initialization and training. By default, 42.
    pub fn new(py_kwargs: Option<&PyDict>) -> PyResult<EdgePredictionSingleExtraTree> {
        let py = pyo3::Python::acquire_gil();
        let kwargs = normalize_kwargs!(py_kwargs, py.python());

        pe!(validate_kwargs(
            kwargs,
            &[
                "metric",
                "edge_embedding_method_name",
                "number_of_edges_to_sample_per_tree_node",
                "number_of_splits_per_tree_node",
                "sample_only_edges_with_heterogeneous_node_types",
                "negative_edges_rate",
                "depth",
                "random_state",
            ]
        ))?;

        Ok(Self {
            model: NodeFeaturesBasedEdgePredictionModelBinding::new(pe!(
                cpu_models::EdgePredictionSingleExtraTree::new(
                    pe!(extract_value_rust_result!(kwargs, "metric", String)
                        .map(|name| name.try_into())
                        .transpose())?,
                    pe!(
                        extract_value_rust_result!(kwargs, "edge_embedding_method_name", String)
                            .map(|name| name.try_into())
                            .transpose()
                    )?,
                    extract_value_rust_result!(
                        kwargs,
                        "number_of_edges_to_sample_per_tree_node",
                        usize
                    ),
                    extract_value_rust_result!(kwargs, "number_of_splits_per_tree_node", usize),
                    extract_value_rust_result!(
                        kwargs,
                        "sample_only_edges_with_heterogeneous_node_types",
                        bool
                    ),
                    extract_value_rust_result!(kwargs, "negative_edges_rate", f32),
                    extract_value_rust_result!(kwargs, "depth", f32),
                    extract_value_rust_result!(kwargs, "random_state", u64),
                )
            )?),
        })
    }
}

#[pymethods]
impl EdgePredictionSingleExtraTree {
    #[args(py_kwargs = "**")]
    #[text_signature = "($self, graph, node_features, verbose, support, graph_to_avoid)"]
    /// Fit the current model instance with the provided graph and node features.
    ///
    /// Parameters
    /// -------------
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
        self.model
            .fit(graph, node_features, verbose, support, graph_to_avoid)
    }

    #[args(py_kwargs = "**")]
    #[text_signature = "($self, graph, node_features)"]
    /// Return numpy array with edge predictions for provided graph.
    ///
    /// Parameters
    /// --------------
    /// graph: Graph
    ///     The graph whose edges are to be predicted.
    /// node_features: np.ndarray
    ///     A node features numpy array.
    fn predict(
        &self,
        graph: &Graph,
        node_features: Py<PyArray2<f32>>,
    ) -> PyResult<Py<PyArray1<f32>>> {
        self.model.predict(graph, node_features)
    }

    #[text_signature = "($self)"]
    /// Returns the weights of the model.
    fn get_weights(&self) -> PyResult<Py<PyArray1<f32>>> {
        let gil = pyo3::Python::acquire_gil();
        Ok(to_ndarray_1d!(
            gil,
            pe!(self.model.get_model().get_weights())?,
            f32
        ))
    }

    #[text_signature = "($self)"]
    /// Returns the bias of the model.
    fn get_bias(&self) -> PyResult<f32> {
        pe!(self.model.get_model().get_bias())
    }
}
