use super::*;
use numpy::PyArray2;

impl Graph {
    fn populate_adjacency_matrix<F, C>(&self, callback: C) -> PyResult<Py<PyArray2<F>>>
    where
        F: Send + Sync + numpy::Element,
        C: Fn(&mut [F]) -> Result<()>,
    {
        let py = pyo3::Python::acquire_gil();
        let matrix = PyArray2::zeros(
            py.python(),
            [
                self.inner.get_number_of_nodes() as usize,
                self.inner.get_number_of_nodes() as usize,
            ],
            false,
        );

        let matrix_ref = unsafe { matrix.as_slice_mut().unwrap() };

        pe!(callback(matrix_ref))?;

        Ok(matrix.to_owned())
    }
}

#[pymethods]
impl Graph {
    #[text_signature = "($self)"]
    /// Return the dense binary adjacency matrix.
    fn get_dense_binary_adjacency_matrix(&self) -> PyResult<Py<PyArray2<bool>>> {
        self.populate_adjacency_matrix(|matrix| {
            self.inner.populate_dense_binary_adjacency_matrix(matrix)
        })
    }

    #[text_signature = "($self)"]
    /// Return the dense weighted adjacency matrix.
    fn get_dense_weighted_adjacency_matrix(&self) -> PyResult<Py<PyArray2<WeightT>>> {
        self.populate_adjacency_matrix(|matrix| {
            self.inner.populate_dense_weighted_adjacency_matrix(matrix)
        })
    }

    #[text_signature = "($self, verbose)"]
    /// Return the dense modularity matrix.
    ///
    /// Parameters
    /// --------------
    /// verbose: bool = True
    ///     Whether to show a loading bar. By default, true.
    fn get_dense_modularity_matrix(
        &self,
        verbose: Option<bool>,
    ) -> PyResult<Py<PyArray2<WeightT>>> {
        self.populate_adjacency_matrix(|matrix| {
            self.inner.populate_modularity_matrix(matrix, verbose)
        })
    }

    #[text_signature = "($self, bfs, verbose)"]
    /// Returns the dense shared ancestor sizes.
    ///
    /// Parameters
    /// --------------
    /// bfs: ShortestPathsResultBFS
    ///     The BFS object to use for the ancestors.
    /// verbose: bool = True
    ///     Whether to show a loading bar. By default, true.
    fn get_shared_ancestors_size_adjacency_matrix(
        &self,
        bfs: &ShortestPathsResultBFS,
        verbose: Option<bool>,
    ) -> PyResult<Py<PyArray2<f32>>> {
        self.populate_adjacency_matrix(|matrix| {
            self.inner
                .populate_shared_ancestors_size_adjacency_matrix(matrix, &bfs.inner, verbose)
        })
    }

    #[text_signature = "($self, bfs, verbose)"]
    /// Returns the dense shared ancestor jaccard.
    ///
    /// Parameters
    /// --------------
    /// bfs: ShortestPathsResultBFS
    ///     The BFS object to use for the ancestors.
    /// verbose: bool = True
    ///     Whether to show a loading bar. By default, true.
    fn get_shared_ancestors_jaccard_adjacency_matrix(
        &self,
        bfs: &ShortestPathsResultBFS,
        verbose: Option<bool>,
    ) -> PyResult<Py<PyArray2<f32>>> {
        self.populate_adjacency_matrix(|matrix| {
            self.inner
                .populate_shared_ancestors_jaccard_adjacency_matrix(matrix, &bfs.inner, verbose)
        })
    }
}
