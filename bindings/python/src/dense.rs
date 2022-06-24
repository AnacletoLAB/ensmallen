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
                self.inner.get_nodes_number() as usize,
                self.inner.get_nodes_number() as usize,
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
    /// Return the dense binary matrix.
    fn get_dense_binary_adjacency_matrix(&self) -> PyResult<Py<PyArray2<bool>>> {
        self.populate_adjacency_matrix(|matrix| {
            self.inner.populate_dense_binary_adjacency_matrix(matrix)
        })
    }

    /// Returns the dense shared ancestor sizes.
    ///
    /// Parameters
    /// --------------
    /// bfs: &ShortestPathsResultBFS
    ///     The BFS object to use for the ancestors.
    /// verbose: Option<bool>
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

    /// Returns the dense shared ancestor jaccard.
    ///
    /// Parameters
    /// --------------
    /// bfs: &ShortestPathsResultBFS
    ///     The BFS object to use for the ancestors.
    /// verbose: Option<bool>
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
