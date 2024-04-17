use super::mmap_numpy_npy::{create_memory_mapped_numpy_array, Dtype};
use super::*;
use cpu_models::MatrixShape;
use numpy::{PyArray1, PyArray2, PyArray3};

macro_rules! impl_graph_embedder {
    ($($dtype:ty : $dtype_enum:expr),*) => {
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

        let embedding_shapes = pe!(self.get_model().get_embedding_shapes(&graph.inner))?;
        let paths = self.get_paths();

        if embedding_shapes.len() != paths.len() {
            unreachable!(
                concat!(
                    "An implementative error was found! The provided embedding shapes were {} ",
                    "but the provided number of paths was {}."
                ),
                embedding_shapes.len(),
                paths.len()
            )
        }

        match self.get_dtype().as_str() {
            $(
                stringify!($dtype) => {
                    let embeddings = embedding_shapes
                        .iter()
                        .zip(paths.into_iter())
                        .map(|(&shape, path)| {
                            create_memory_mapped_numpy_array(
                                gil.python(),
                                path.as_ref().map(|x| x.as_str()),
                                $dtype_enum,
                                &<MatrixShape as Into<Vec<isize>>>::into(shape),
                                false,
                            )
                        })
                        .collect::<Vec<_>>();

                    let mut array1d_references = Vec::new();
                    let mut array2d_references = Vec::new();
                    let mut array3d_references = Vec::new();
                    let mut array4d_references = Vec::new();
                    let mut embedding_slices = Vec::new();

                    for (embedding, shape) in embeddings.iter().zip(embedding_shapes.into_iter()) {
                        match shape {
                            MatrixShape::OneDimensional(_) => {
                                let embedding_reference = embedding.cast_as::<PyArray1<$dtype>>(gil.python())?;
                                array1d_references.push(embedding_reference);
                                embedding_slices.push(unsafe { embedding_reference.as_slice_mut()? });
                            }
                            MatrixShape::BiDimensional(_, _) => {
                                let embedding_reference = embedding.cast_as::<PyArray2<$dtype>>(gil.python())?;
                                array2d_references.push(embedding_reference);
                                embedding_slices.push(unsafe { embedding_reference.as_slice_mut()? });
                            }
                            MatrixShape::ThreeDimensional(_, _, _) => {
                                let embedding_reference = embedding.cast_as::<PyArray3<$dtype>>(gil.python())?;
                                array3d_references.push(embedding_reference);
                                embedding_slices.push(unsafe { embedding_reference.as_slice_mut()? });
                            }
                            MatrixShape::FourDimensional(_, _, _, _) => {
                                let embedding_reference = embedding.cast_as::<PyArray4<$dtype>>(gil.python())?;
                                array4d_references.push(embedding_reference);
                                embedding_slices.push(unsafe { embedding_reference.as_slice_mut()? });
                            }
                        }
                    }

                    // We always use the racing version of the fit transfor
                    // as we generally do not care about memory collisions.
                    pe!(self
                        .get_model()
                        .fit_transform(&graph.inner, embedding_slices.as_mut_slice(),))?;

                    Ok(embeddings)
                }
            )*
            dtype => pe!(Err(format!(
                concat!(
                    "The provided dtype {} is not supported. The supported ",
                    "data types are `f32` and `f64`."
                ),
                dtype
            ))),
        }
    }

    fn get_model_name(&self) -> String {
        self.get_model().get_model_name()
    }

    fn get_dtype(&self) -> String {
        self.get_model().get_dtype()
    }

    fn is_verbose(&self) -> bool {
        self.get_model().is_verbose()
    }

    fn get_paths(&self) -> Vec<Option<String>>;

    fn get_model(&self) -> &M;
}};}

impl_graph_embedder! {
    f32: Dtype::F32,
    f64: Dtype::F64
}
