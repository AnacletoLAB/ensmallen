use super::*;
use numpy::{PyArray1, PyArray2};

///
#[pyclass]
#[derive(Clone)]
#[pyo3(text_signature = "(verbose,)")]
pub struct DAGResnik {
    pub inner: cpu_models::DAGResnik<f32>,
}

#[pymethods]
impl DAGResnik {
    #[new]
    /// Return a new instance of the DAG-based Resnik similarity model.
    ///
    /// Parameters
    /// ---------------------
    /// verbose: bool = True
    ///     Whether to show a loading bar while computing the pairwise distances.
    pub fn new(verbose: Option<bool>) -> DAGResnik {
        Self {
            inner: cpu_models::DAGResnik::new(verbose),
        }
    }
}

#[pymethods]
impl DAGResnik {
    #[args(py_kwargs = "**")]
    #[pyo3(text_signature = "($self, graph, node_counts, node_frequencies)")]
    /// Fit the current model instance with the provided graph and node features.
    ///
    /// Parameters
    /// ---------
    /// graph: Graph
    ///     The graph whose edges are to be learned.
    /// node_counts: Optional[Dict[String, int]] = None
    ///      These counts should represent how many times a given node appears in a set.
    /// node_frequencies: Optional[np.ndarray] = None
    ///     Optional vector of node frequencies.
    fn fit(
        &mut self,
        graph: &Graph,
        node_counts: Option<HashMap<String, u32>>,
        node_frequencies: Option<Py<PyArray1<f32>>>,
    ) -> PyResult<()> {
        let gil = pyo3::Python::acquire_gil();
        let node_frequencies_ref = node_frequencies
            .as_ref()
            .map(|node_frequencies| node_frequencies.as_ref(gil.python()));
        pe!(self.inner.fit(
            &graph.inner,
            node_counts.as_ref(),
            node_frequencies_ref
                .map(|node_frequencies_ref| unsafe { node_frequencies_ref.as_slice().unwrap() }),
        ))
    }

    #[pyo3(text_signature = "($self)")]
    /// Returns the node frequencies of the model.
    fn get_information_contents(&self) -> PyResult<Py<PyArray1<f32>>> {
        let gil = pyo3::Python::acquire_gil();
        Ok(to_ndarray_1d!(
            gil,
            pe!(self.inner.get_information_contents())?,
            f32
        ))
    }

    #[pyo3(
        text_signature = "($self, first_node_ids, second_node_ids, minimum_similarity)"
    )]
    /// Return the similarity between the two provided ids nodes.
    ///
    /// Parameters
    /// -------------------
    /// first_node_ids: List[int]
    ///     The first nodes for which to compute the similarity.
    /// second_node_ids: List[int]
    ///     The second nodes for which to compute the similarity.
    /// minimum_similarity: Optional[float] = 0.0
    ///     Minimum similarity to be kept. Values below this amount are filtered.
    pub fn get_node_ids_and_similarity_from_node_ids(
        &self,
        first_node_ids: Vec<NodeT>,
        second_node_ids: Vec<NodeT>,
        minimum_similarity: Option<f32>,
    ) -> PyResult<(Py<PyArray2<NodeT>>, Py<PyArray1<f32>>)> {
        let gil = pyo3::Python::acquire_gil();
        let (node_ids, similarities): (Vec<Vec<NodeT>>, Vec<f32>) =
            pe!(self.inner.get_node_ids_and_similarity_from_node_ids(
                &first_node_ids,
                &second_node_ids,
                minimum_similarity
            ))?;
        Ok((
            to_ndarray_2d!(gil, node_ids, NodeT),
            to_ndarray_1d!(gil, similarities, f32),
        ))
    }

    #[pyo3(
        text_signature = "($self, first_node_names, second_node_names, minimum_similarity)"
    )]
    /// Return the similarity between the two provided names nodes.
    ///
    /// Parameters
    /// -------------------
    /// first_node_names: List[str]
    ///     The first nodes for which to compute the similarity.
    /// second_node_names: List[str]
    ///     The second nodes for which to compute the similarity.
    /// minimum_similarity: Optional[float] = 0.0
    ///     Minimum similarity to be kept. Values below this amount are filtered.
    pub fn get_node_ids_and_similarity_from_node_names(
        &self,
        first_node_names: Vec<&str>,
        second_node_names: Vec<&str>,
        minimum_similarity: Option<f32>,
    ) -> PyResult<(Py<PyArray2<NodeT>>, Py<PyArray1<f32>>)> {
        let gil = pyo3::Python::acquire_gil();
        let (node_ids, similarities): (Vec<Vec<NodeT>>, Vec<f32>) =
            pe!(self.inner.get_node_ids_and_similarity_from_node_names(
                &first_node_names,
                &second_node_names,
                minimum_similarity
            ))?;
        Ok((
            to_ndarray_2d!(gil, node_ids, NodeT),
            to_ndarray_1d!(gil, similarities, f32),
        ))
    }
    
    #[pyo3(
        text_signature = "($self, first_node_prefixes, second_node_prefixes, minimum_similarity)"
    )]
    /// Return the similarity between the two provided prefixes nodes.
    ///
    /// Parameters
    /// -------------------
    /// first_node_prefixes: List[str]
    ///     The first nodes for which to compute the similarity.
    /// second_node_prefixes: List[str]
    ///     The second nodes for which to compute the similarity.
    /// minimum_similarity: Optional[float] = 0.0
    ///     Minimum similarity to be kept. Values below this amount are filtered.
    pub fn get_node_ids_and_similarity_from_node_prefixes(
        &self,
        first_node_prefixes: Vec<&str>,
        second_node_prefixes: Vec<&str>,
        minimum_similarity: Option<f32>,
    ) -> PyResult<(Py<PyArray2<NodeT>>, Py<PyArray1<f32>>)> {
        let gil = pyo3::Python::acquire_gil();
        let (node_ids, similarities): (Vec<Vec<NodeT>>, Vec<f32>) =
            pe!(self.inner.get_node_ids_and_similarity_from_node_prefixes(
                &first_node_prefixes,
                &second_node_prefixes,
                minimum_similarity
            ))?;
        Ok((
            to_ndarray_2d!(gil, node_ids, NodeT),
            to_ndarray_1d!(gil, similarities, f32),
        ))
    }

    #[pyo3(
        text_signature = "($self, first_node_type_ids, second_node_type_ids, minimum_similarity)"
    )]
    /// Return the similarity between the two provided node_type_ids nodes.
    ///
    /// Parameters
    /// -------------------
    /// first_node_type_ids: List[int]
    ///     The first nodes type ids for which to compute the similarity.
    /// second_node_type_ids: List[int]
    ///     The second nodes type ids for which to compute the similarity.
    /// minimum_similarity: Optional[float] = 0.0
    ///     Minimum similarity to be kept. Values below this amount are filtered.
    pub fn get_node_ids_and_similarity_from_node_type_ids(
        &self,
        first_node_type_ids: Vec<Option<NodeTypeT>>,
        second_node_type_ids: Vec<Option<NodeTypeT>>,
        minimum_similarity: Option<f32>,
    ) -> PyResult<(Py<PyArray2<NodeT>>, Py<PyArray1<f32>>)> {
        let gil = pyo3::Python::acquire_gil();
        let (node_ids, similarities): (Vec<Vec<NodeT>>, Vec<f32>) =
            pe!(self.inner.get_node_ids_and_similarity_from_node_type_ids(
                &first_node_type_ids,
                &second_node_type_ids,
                minimum_similarity
            ))?;
        Ok((
            to_ndarray_2d!(gil, node_ids, NodeT),
            to_ndarray_1d!(gil, similarities, f32),
        ))
    }
    
    #[pyo3(
        text_signature = "($self, first_node_type_names, second_node_type_names, minimum_similarity)"
    )]
    /// Return the similarity between the two provided node_type_names nodes.
    ///
    /// Parameters
    /// -------------------
    /// first_node_type_names: List[str]
    ///     The first nodes type names for which to compute the similarity.
    /// second_node_type_names: List[str]
    ///     The second nodes type names for which to compute the similarity.
    /// minimum_similarity: Optional[float] = 0.0
    ///     Minimum similarity to be kept. Values below this amount are filtered.
    pub fn get_node_ids_and_similarity_from_node_type_names(
        &self,
        first_node_type_names: Vec<Option<&str>>,
        second_node_type_names: Vec<Option<&str>>,
        minimum_similarity: Option<f32>,
    ) -> PyResult<(Py<PyArray2<NodeT>>, Py<PyArray1<f32>>)> {
        let gil = pyo3::Python::acquire_gil();
        let (node_ids, similarities): (Vec<Vec<NodeT>>, Vec<f32>) =
            pe!(self.inner.get_node_ids_and_similarity_from_node_type_names(
                &first_node_type_names,
                &second_node_type_names,
                minimum_similarity
            ))?;
        Ok((
            to_ndarray_2d!(gil, node_ids, NodeT),
            to_ndarray_1d!(gil, similarities, f32),
        ))
    }
}
