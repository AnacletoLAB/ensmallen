use super::*;

#[pymethods]
impl Graph {
    #[args(py_kwargs = "**")]
    #[text_signature = "($self)"]
    /// Return tuple with edge node IDs and edge weights.
    pub fn get_laplacian_coo_matrix(&self) -> (Py<PyArray2<NodeT>>, Py<PyArray1<WeightT>>) {
        // Acquire the python gil.
        let gil = pyo3::Python::acquire_gil();
        // Get the expected number of edges
        let edges_number = self.inner.get_laplacian_coo_matrix_edges_number() as usize;
        // Create the return vectors
        let edge_node_ids = PyArray2::new(gil.python(), [edges_number, 2], false);
        let edge_weights = PyArray1::new(gil.python(), [edges_number], false);
        self.inner.iter_laplacian_coo_matrix().enumerate().for_each(
            |(i, (src, dst, edge_weight))| unsafe {
                *edge_node_ids.uget_mut([i, 0]) = src;
                *edge_node_ids.uget_mut([i, 1]) = dst;
                *edge_weights.uget_mut([i]) = edge_weight;
            },
        );
        (edge_node_ids.to_owned(), edge_weights.to_owned())
    }

    #[args(py_kwargs = "**")]
    #[text_signature = "($self)"]
    /// Return tuple with edge node IDs and edge weights.
    pub fn get_random_walk_normalized_laplacian_coo_matrix(
        &self,
    ) -> (Py<PyArray2<NodeT>>, Py<PyArray1<WeightT>>) {
        // Acquire the python gil.
        let gil = pyo3::Python::acquire_gil();
        // Get the expected number of edges
        let edges_number = self.inner.get_laplacian_coo_matrix_edges_number() as usize;
        // Create the return vectors
        let edge_node_ids = PyArray2::new(gil.python(), [edges_number, 2], false);
        let edge_weights = PyArray1::new(gil.python(), [edges_number], false);
        self.inner
            .iter_random_walk_normalized_laplacian_coo_matrix()
            .enumerate()
            .for_each(|(i, (src, dst, edge_weight))| unsafe {
                *edge_node_ids.uget_mut([i, 0]) = src;
                *edge_node_ids.uget_mut([i, 1]) = dst;
                *edge_weights.uget_mut([i]) = edge_weight;
            });
        (edge_node_ids.to_owned(), edge_weights.to_owned())
    }

    #[args(py_kwargs = "**")]
    #[text_signature = "($self)"]
    /// Return tuple with edge node IDs and edge weights.
    pub fn get_symmetric_normalized_laplacian_coo_matrix(
        &self,
    ) -> (Py<PyArray2<NodeT>>, Py<PyArray1<WeightT>>) {
        // Acquire the python gil.
        let gil = pyo3::Python::acquire_gil();
        // Get the expected number of edges
        let edges_number = self.inner.get_laplacian_coo_matrix_edges_number() as usize;
        // Create the return vectors
        let edge_node_ids = PyArray2::new(gil.python(), [edges_number, 2], false);
        let edge_weights = PyArray1::new(gil.python(), [edges_number], false);
        self.inner
            .iter_symmetric_normalized_laplacian_coo_matrix()
            .enumerate()
            .for_each(|(i, (src, dst, edge_weight))| unsafe {
                *edge_node_ids.uget_mut([i, 0]) = src;
                *edge_node_ids.uget_mut([i, 1]) = dst;
                *edge_weights.uget_mut([i]) = edge_weight;
            });
        (edge_node_ids.to_owned(), edge_weights.to_owned())
    }

    #[args(py_kwargs = "**")]
    #[text_signature = "($self)"]
    /// Return tuple with edge node IDs and edge weights.
    pub fn get_symmetric_normalized_coo_matrix(
        &self,
    ) -> PyResult<(Py<PyArray2<NodeT>>, Py<PyArray1<WeightT>>)> {
        // Acquire the python gil.
        let gil = pyo3::Python::acquire_gil();
        // Get the expected number of edges
        let edges_number = self.inner.get_laplacian_coo_matrix_edges_number() as usize;
        // Create the return vectors
        let edge_node_ids = PyArray2::new(gil.python(), [edges_number, 2], false);
        let edge_weights = PyArray1::new(gil.python(), [edges_number], false);
        pe!(self.inner.iter_symmetric_normalized_coo_matrix())?
            .enumerate()
            .for_each(|(i, (src, dst, edge_weight))| unsafe {
                *edge_node_ids.uget_mut([i, 0]) = src;
                *edge_node_ids.uget_mut([i, 1]) = dst;
                *edge_weights.uget_mut([i]) = edge_weight;
            });
        Ok((edge_node_ids.to_owned(), edge_weights.to_owned()))
    }
}
