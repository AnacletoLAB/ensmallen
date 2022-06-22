use super::*;

///
#[pyclass]
#[derive(Debug, Clone)]
#[text_signature = "(*, window_size, iterations, random_state)"]
pub struct CooccurrenceEdgePrediction {
    pub inner: cpu_models::CooccurrenceEdgePrediction,
}

impl From<cpu_models::CooccurrenceEdgePrediction> for CooccurrenceEdgePrediction {
    fn from(val: cpu_models::CooccurrenceEdgePrediction) -> CooccurrenceEdgePrediction {
        CooccurrenceEdgePrediction { inner: val }
    }
}

impl From<CooccurrenceEdgePrediction> for cpu_models::CooccurrenceEdgePrediction {
    fn from(val: CooccurrenceEdgePrediction) -> cpu_models::CooccurrenceEdgePrediction {
        val.inner
    }
}

#[pymethods]
impl CooccurrenceEdgePrediction {
    #[new]
    #[args(py_kwargs = "**")]
    /// Return a new instance of the CooccurrenceEdgePrediction model.
    ///
    /// Parameters
    /// ------------------------
    /// window_size: Optional[int]
    ///     Window size defining the contexts.
    /// iterations: Optional[int]
    ///     Number of walks to run from each node. By default 50.
    /// random_state: int = 42
    ///     The random state to reproduce the model initialization and training. By default, 42.
    pub fn new(py_kwargs: Option<&PyDict>) -> PyResult<CooccurrenceEdgePrediction> {
        let py = pyo3::Python::acquire_gil();
        let kwargs = normalize_kwargs!(py_kwargs, py.python());

        pe!(validate_kwargs(
            kwargs,
            &["window_size", "iterations", "random_state"]
        ))?;

        Ok(Self {
            inner: pe!(cpu_models::CooccurrenceEdgePrediction::new(
                extract_value_rust_result!(kwargs, "window_size", u64),
                extract_value_rust_result!(kwargs, "iterations", usize),
                extract_value_rust_result!(kwargs, "random_state", u64),
            ))?,
        })
    }
}

#[pymethods]
impl CooccurrenceEdgePrediction {
    #[args(py_kwargs = "**")]
    #[text_signature = "($self, graph)"]
    /// Return numpy array with edge predictions for provided graph.
    ///
    /// Parameters
    /// ---------
    /// graph: Graph
    ///     The graph whose edges are to be predicted.
    /// support: Optional[Graph]
    ///     The graph whose topology is to be used
    fn predict(&self, graph: &Graph, support: Option<&Graph>) -> PyResult<Py<PyArray1<f32>>> {
        let gil = pyo3::Python::acquire_gil();
        let support = support.map(|support| &support.inner);

        let predictions = PyArray1::new(
            gil.python(),
            [graph.get_number_of_directed_edges() as usize],
            false,
        );
        let predictions_ref = unsafe { predictions.as_slice_mut().unwrap() };

        pe!(self.inner.predict(predictions_ref, &graph.inner, support))?;

        Ok(predictions.to_owned())
    }
}
