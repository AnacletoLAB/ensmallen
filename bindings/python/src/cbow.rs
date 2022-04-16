use super::*;
use numpy::PyArray2;

#[pymethods]
impl Graph {
    #[args(py_kwargs = "**")]
    #[text_signature = "($self, *, embedding_size, epochs, walk_length, return_weight, explore_weight, change_edge_type_weight, change_node_type_weight, iterations, max_neighbours, window_size, negatives_number, learning_rate, random_state, verbose)"]
    fn compute_cbow_embedding(
        &self,
        embedding_size: Option<usize>,
        epochs: Option<usize>,
        walk_length: Option<u64>,
        return_weight: Option<f32>,
        explore_weight: Option<f32>,
        change_edge_type_weight: Option<f32>,
        change_node_type_weight: Option<f32>,
        iterations: Option<NodeT>,
        max_neighbours: Option<NodeT>,
        window_size: Option<usize>,
        negatives_number: Option<usize>,
        learning_rate: Option<f32>,
        random_state: Option<u64>,
        verbose: Option<bool>,
    ) -> PyResult<Py<PyArray2<f32>>> {
        let gil = pyo3::Python::acquire_gil();
        let embedding_size = embedding_size.unwrap_or(100);

        let rows_number = self.inner.get_nodes_number() as usize;
        let columns_number = embedding_size;
        let embedding = PyArray2::zeros(gil.python(), [rows_number, columns_number], false);

        let embedding_slice = unsafe { embedding.as_slice_mut().unwrap() };

        pe!(self.inner.compute_cbow_embedding(
            embedding_slice,
            Some(embedding_size),
            epochs,
            walk_length,
            return_weight,
            explore_weight,
            change_edge_type_weight,
            change_node_type_weight,
            iterations,
            max_neighbours,
            window_size,
            negatives_number,
            learning_rate,
            random_state,
            verbose,
        ))?;

        Ok(embedding.into_py(gil.python()))
    }
}
