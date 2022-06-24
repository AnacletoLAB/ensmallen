use super::*;
use numpy::PyArray2;

pub trait GraphEmbedderBinding<F, M>
where
    F: numpy::Element,
    M: cpu_models::GraphEmbedder<F>,
{
    /// Computes in the provided memory slice the graph embedding.
    ///
    /// # Arguments
    /// `graph`: &Graph - The graph to embed
    fn fit_transform(&self, graph: &Graph) -> PyResult<Vec<Py<PyArray2<F>>>> {
        let gil = pyo3::Python::acquire_gil();

        let embeddings = self
            .get_model()
            .get_embedding_sizes(&graph.inner)
            .into_iter()
            .map(|(number_of_rows, number_of_columns)| {
                PyArray2::new(gil.python(), [number_of_rows, number_of_columns], false)
            })
            .collect::<Vec<_>>();

        let mut embedding_slices = embeddings
            .iter()
            .map(|embedding| unsafe { embedding.as_slice_mut().unwrap() })
            .collect::<Vec<_>>();

        // We always use the racing version of the fit transform
        // as we generally do not care about memory collisions.
        pe!(self
            .get_model()
            .fit_transform(&graph.inner, embedding_slices.as_mut_slice(),))?;

        Ok(embeddings
            .into_iter()
            .map(|embedding| embedding.into_py(gil.python()))
            .collect())
    }

    fn get_model_name(&self) -> String {
        self.get_model().get_model_name()
    }

    fn get_model(&self) -> &M;
}
