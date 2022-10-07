use super::*;
use crate::utilities::normalize_features;
use cpu_models::Adam;

type InnerModel = cpu_models::DistanceNodeLabelPredictionPerceptron<Adam<f32, Vec<f32>>>;

///
#[pyclass]
#[derive(Clone)]
#[pyo3(
    text_signature = "(*, number_of_centroids_per_class, number_of_epochs, learning_rate, first_order_decay_factor, second_order_decay_factor, random_state)"
)]
pub struct DistanceNodeLabelPredictionPerceptron {
    pub inner: InnerModel,
}

#[pymethods]
impl DistanceNodeLabelPredictionPerceptron {
    #[new]
    #[args(py_kwargs = "**")]
    /// Return a new instance of the DistanceNodeLabelPredictionPerceptron model.
    ///
    /// Parameters
    /// ------------------------
    /// number_of_centroids_per_class: int = 1
    ///     Number of centroids to compute for each of the classes.
    /// number_of_epochs: int = 100
    ///     The number of epochs to train the model for. By default, 100.
    /// learning_rate: float = 0.001
    ///     Learning rate to use while training the model.
    ///     By default 0.001.
    /// first_order_decay_factor: float = 0.9
    ///     First order decay factor for the first order momentum.
    ///     By default 0.9.
    /// second_order_decay_factor: float = 0.999
    ///     Second order decay factor for the second order momentum.
    ///     By default 0.999.
    /// random_state: int = 42
    ///     The random state to reproduce the model initialization and training. By default, 42.
    pub fn new(py_kwargs: Option<&PyDict>) -> PyResult<DistanceNodeLabelPredictionPerceptron> {
        let py = pyo3::Python::acquire_gil();
        let kwargs = normalize_kwargs!(py_kwargs, py.python());

        pe!(validate_kwargs(
            kwargs,
            &[
                "number_of_centroids_per_class",
                "number_of_epochs",
                "learning_rate",
                "first_order_decay_factor",
                "second_order_decay_factor",
                "random_state"
            ]
        ))?;

        Ok(Self {
            inner: pe!(cpu_models::DistanceNodeLabelPredictionPerceptron::new(
                cpu_models::Adam::new(
                    extract_value_rust_result!(kwargs, "learning_rate", f32),
                    extract_value_rust_result!(kwargs, "first_order_decay_factor", f32),
                    extract_value_rust_result!(kwargs, "second_order_decay_factor", f32),
                ),
                extract_value_rust_result!(kwargs, "number_of_epochs", usize),
                extract_value_rust_result!(kwargs, "number_of_centroids_per_class", usize),
                extract_value_rust_result!(kwargs, "random_state", u64),
            ))?,
        })
    }
}

#[pymethods]
impl DistanceNodeLabelPredictionPerceptron {
    #[args(py_kwargs = "**")]
    #[pyo3(text_signature = "($self, graph, node_features, verbose)")]
    /// Fit the current model instance with the provided graph and node features.
    ///
    /// Parameters
    /// ---------
    /// graph: Graph
    ///     The graph whose nodes are to be learned.
    /// node_features: List[np.ndarray]
    ///     A list of node features numpy array.
    /// verbose: bool = True
    ///     Whether to show a loading bar for the epochs. By default, True.
    fn fit(
        &mut self,
        graph: &Graph,
        node_features: Vec<Py<PyAny>>,
        verbose: Option<bool>,
    ) -> PyResult<()> {
        let gil = pyo3::Python::acquire_gil();
        let (_numpy_references, dimensions, slices) =
            normalize_features(&gil, node_features.as_slice())?;
        pe!(self.inner.fit(
            &graph.inner,
            slices.as_slice(),
            dimensions.as_slice(),
            verbose,
        ))
    }

    #[pyo3(text_signature = "($self)")]
    /// Returns the weights of the model.
    fn get_weights(&self) -> PyResult<Py<PyArray2<f32>>> {
        let gil = pyo3::Python::acquire_gil();
        Ok(to_ndarray_2d!(gil, pe!(self.inner.get_weights())?, f32))
    }

    #[pyo3(text_signature = "($self)")]
    /// Returns the centroids of the model.
    fn get_centroids(&self) -> PyResult<Py<PyArray3<f32>>> {
        let gil = pyo3::Python::acquire_gil();
        Ok(to_ndarray_3d!(gil, pe!(self.inner.get_centroids())?, f32))
    }

    #[pyo3(text_signature = "($self)")]
    /// Returns the bias of the model.
    fn get_bias(&self) -> PyResult<Py<PyArray1<f32>>> {
        let gil = pyo3::Python::acquire_gil();
        Ok(to_ndarray_1d!(gil, pe!(self.inner.get_bias())?, f32))
    }

    #[pyo3(text_signature = "($self, graph, node_features)")]
    /// Return numpy array with node label predictions for provided graph.
    ///
    /// Parameters
    /// ----------------
    /// graph: Graph
    ///     The graph whose node labels are to be predicted.
    /// node_features: List[np.ndarray]
    ///     A node features numpy array.
    fn compute_similarities(
        &self,
        graph: &Graph,
        node_features: Vec<Py<PyAny>>,
    ) -> PyResult<Py<PyArray2<f32>>> {
        let gil = pyo3::Python::acquire_gil();
        let (_numpy_references, dimensions, slices) =
            normalize_features(&gil, node_features.as_slice())?;

        let (similarities, dimension) = pe!(self.inner.compute_similarities(
            &graph.inner,
            slices.as_slice(),
            dimensions.as_slice(),
            None
        ))?;

        let vector = similarities
            .chunks(dimension)
            .map(|chunk| chunk.to_vec())
            .collect::<Vec<Vec<f32>>>();

        Ok(to_ndarray_2d!(gil, vector, f32))
    }

    #[pyo3(text_signature = "($self, graph, node_features)")]
    /// Return numpy array with node label predictions for provided graph.
    ///
    /// Parameters
    /// ----------------
    /// graph: Graph
    ///     The graph whose node labels are to be predicted.
    /// node_features: List[np.ndarray]
    ///     A node features numpy array.
    fn predict(&self, graph: &Graph, node_features: Vec<Py<PyAny>>) -> PyResult<Py<PyArray2<f32>>> {
        let gil = pyo3::Python::acquire_gil();
        let predictions = unsafe {
            PyArray2::new(
                gil.python(),
                [
                    graph.get_number_of_nodes() as usize,
                    pe!(self.inner.get_number_of_outputs())?,
                ],
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
        Ok(DistanceNodeLabelPredictionPerceptron {
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
        Ok(DistanceNodeLabelPredictionPerceptron {
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
