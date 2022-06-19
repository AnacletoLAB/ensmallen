use super::*;
use cpu_models::NodeFeaturesBasedEdgePrediction;
use numpy::PyArray2;

#[derive(Debug, Clone)]
pub(crate) struct NodeFeaturesBasedEdgePredictionModelBinding<Model>
where
    Model: NodeFeaturesBasedEdgePrediction,
{
    model: Model,
}

impl<Model> NodeFeaturesBasedEdgePredictionModelBinding<Model>
where
    Model: NodeFeaturesBasedEdgePrediction,
{
    pub fn new(model: Model) -> Self {
        Self { model }
    }

    pub fn get_model(&self) -> &Model {
        &self.model
    }

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
    pub(crate) fn fit(
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

        pe!(self.model.fit(
            &graph.inner,
            node_features_ref,
            node_features.shape()[1],
            verbose,
            support,
            graph_to_avoid
        ))
    }

    /// Return numpy array with edge predictions for provided graph.
    ///
    /// Parameters
    /// ---------
    /// graph: Graph
    ///     The graph whose edges are to be predicted.
    /// node_features: np.ndarray
    ///     A node features numpy array.
    pub(crate) fn predict(
        &self,
        graph: &Graph,
        node_features: Py<PyArray2<f32>>,
    ) -> PyResult<Py<PyArray1<f32>>> {
        let gil = pyo3::Python::acquire_gil();

        let node_features = node_features.as_ref(gil.python());
        let node_features_ref = unsafe { node_features.as_slice().unwrap() };
        let predictions = PyArray1::new(
            gil.python(),
            [graph.get_number_of_directed_edges() as usize],
            false,
        );
        let predictions_ref = unsafe { predictions.as_slice_mut().unwrap() };

        pe!(self.model.predict(
            predictions_ref,
            &graph.inner,
            node_features_ref,
            node_features.shape()[1],
        ))?;

        Ok(predictions.to_owned())
    }
}
