use super::*;
use numpy::{PyArray1, PyArray2};
use pyo3::wrap_pyfunction;

#[pymodule]
fn express_measures(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(cosine_similarity_from_indices_unchecked))?;
    Ok(())
}

#[module(preprocessing)]
#[pyfunction()]
#[text_signature = "(matrix, sources, destinations)"]
/// Returns cosine similarity of the provided source and destinations using the provided features.
///
/// Arguments
/// ------------
/// matrix: np.ndarray
///     2D Matrix containing the feaures.
/// sources: np.ndarray
///     Indices of the source features.
/// destinations: np.ndarray
///     Indices of the destination features.
///
fn cosine_similarity_from_indices_unchecked(
    matrix: Py<PyArray2<f32>>,
    sources: Py<PyArray1<u32>>,
    destinations: Py<PyArray1<u32>>,
) -> PyResult<Py<PyArray1<f32>>> {
    let gil = pyo3::Python::acquire_gil();
    let matrix = matrix.as_ref(gil.python());
    let matrix_ref = unsafe { matrix.as_slice().unwrap() };
    let sources = sources.as_ref(gil.python());
    let sources_ref = unsafe { sources.as_slice().unwrap() };
    let destinations = destinations.as_ref(gil.python());
    let destinations_ref = unsafe { destinations.as_slice().unwrap() };
    let similarities = PyArray1::new(gil.python(), [sources_ref.len()], false);
    let similarities_ref = unsafe { similarities.as_slice_mut().unwrap() };
    pe!(unsafe {
        ::express_measures::cosine_similarity_from_indices_unchecked(
            similarities_ref,
            matrix_ref,
            sources_ref,
            destinations_ref,
            matrix.shape()[1],
        )
    })?;
    Ok(similarities.to_owned())
}