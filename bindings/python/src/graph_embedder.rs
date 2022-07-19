use super::*;
use cpu_models::MatrixShape;
use numpy::{PyArray1, PyArray2, PyArray3};

pub trait GraphEmbedderBinding<M>
where
    M: cpu_models::GraphEmbedder,
{
    /// Computes in the provided memory slice the graph embedding.
    ///
    /// # Arguments
    /// `graph`: &Graph - The graph to embed
    fn fit_transform(&self, graph: &Graph) -> PyResult<Vec<Py<PyAny>>> {
        let gil = pyo3::Python::acquire_gil();

        let mut embeddings = Vec::new();
        let mut embedding_slices: Vec<&mut [f32]> = Vec::new();

        for shape in pe!(self.get_model().get_embedding_shapes(&graph.inner))? {
            match shape {
                MatrixShape::OneDimensional(one) => {
                    let vector = PyArray1::new(gil.python(), [one], false);
                    embedding_slices.push(pe!(unsafe { vector.as_slice_mut() })?);
                    embeddings.push(vector.into_py(gil.python()));
                }
                MatrixShape::BiDimensional(one, two) => {
                    let vector = PyArray2::new(gil.python(), [one, two], false);
                    embedding_slices.push(pe!(unsafe { vector.as_slice_mut() })?);
                    embeddings.push(vector.into_py(gil.python()));
                }
                MatrixShape::ThreeDimensional(one, two, three) => {
                    let vector = PyArray3::new(gil.python(), [one, two, three], false);
                    embedding_slices.push(pe!(unsafe { vector.as_slice_mut() })?);
                    embeddings.push(vector.into_py(gil.python()));
                }
            }
        }

        // We always use the racing version of the fit transform
        // as we generally do not care about memory collisions.
        pe!(self
            .get_model()
            .fit_transform(&graph.inner, embedding_slices.as_mut_slice(),))?;

        Ok(embeddings)
    }

    fn get_model_name(&self) -> String {
        self.get_model().get_model_name()
    }

    fn get_model(&self) -> &M;
}
