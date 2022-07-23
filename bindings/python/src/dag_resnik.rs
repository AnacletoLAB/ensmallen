use super::*;
use numpy::PyArray1;

///
#[pyclass]
#[derive(Clone)]
#[text_signature = "()"]
pub struct DAGResnik {
    pub inner: cpu_models::DAGResnik,
}

#[pymethods]
impl DAGResnik {
    #[new]
    #[args(py_kwargs = "**")]
    /// Return a new instance of the DAG-based Resnik similarity model.
    pub fn new() -> DAGResnik {
        Self {
            inner: cpu_models::DAGResnik::new(),
        }
    }
}

#[pymethods]
impl DAGResnik {
    #[args(py_kwargs = "**")]
    #[text_signature = "($self, graph, node_frequencies)"]
    /// Fit the current model instance with the provided graph and node features.
    ///
    /// Parameters
    /// ---------
    /// graph: Graph
    ///     The graph whose edges are to be learned.
    /// node_frequencies: Optional[np.ndarray]
    ///     Optional vector of node frequencies.
    fn fit(&mut self, graph: &Graph, node_frequencies: Option<Py<PyArray1<f32>>>) -> PyResult<()> {
        let gil = pyo3::Python::acquire_gil();
        let node_frequencies_ref = node_frequencies
            .as_ref()
            .map(|node_frequencies| node_frequencies.as_ref(gil.python()));
        pe!(self.inner.fit(
            &graph.inner,
            node_frequencies_ref
                .map(|node_frequencies_ref| unsafe { node_frequencies_ref.as_slice().unwrap() }),
        ))
    }

    #[text_signature = "($self)"]
    /// Returns the node frequencies of the model.
    fn get_node_frequencies(&self) -> PyResult<Py<PyArray1<f32>>> {
        let gil = pyo3::Python::acquire_gil();
        Ok(to_ndarray_1d!(
            gil,
            pe!(self.inner.get_node_frequencies())?,
            f32
        ))
    }

    #[text_signature = "($self, first_node_id, second_node_id)"]
    /// Return the similarity between the two provided nodes.
    ///
    /// # Arguments
    /// * `first_node_id`: NodeT - The first node for which to compute the similarity.
    /// * `second_node_id`: NodeT - The second node for which to compute the similarity.
    pub fn get_similarity_from_node_id(
        &self,
        first_node_id: NodeT,
        second_node_id: NodeT,
    ) -> PyResult<f32> {
        pe!(self
            .inner
            .get_similarity_from_node_id(first_node_id, second_node_id))
    }

    #[text_signature = "($self, graph)"]
    /// Return numpy array with edge similarities for provided graph.
    ///
    /// Parameters
    /// ---------
    /// graph: Graph
    ///     The graph whose edges are to be predicted.
    fn get_similarities_from_graph(&self, graph: &Graph) -> PyResult<Py<PyArray1<f32>>> {
        let gil = pyo3::Python::acquire_gil();
        let similarities = PyArray1::new(
            gil.python(),
            [graph.get_number_of_directed_edges() as usize],
            false,
        );
        let similarities_ref = unsafe { similarities.as_slice_mut().unwrap() };

        pe!(self
            .inner
            .get_similarities_from_graph(similarities_ref, &graph.inner,))?;

        Ok(similarities.to_owned())
    }
}
