use super::*;
use numpy::{PyArray1, PyArray2};
use pyo3::wrap_pyfunction;

#[pymodule]
fn express_measures(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(cosine_similarity_from_indices_unchecked))?;
    m.add_wrapped(wrap_pyfunction!(binary_accuracy_score))?;
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

/// Returns accuracy score on the provided 1D numpy array.
/// 
/// # Arguments
/// * `ground_truth`: Py<PyArray1<T>> - 1D Numpy array with the ground truths classes.
/// * `predictions`: Py<PyArray1<T>> - 1D Numpy array with the predicted classes.
fn generic_accuracy_score<T>(
    ground_truth: Py<PyArray1<T>>,
    predictions: Py<PyArray1<T>>,
) -> PyResult<f32>
where
    T: numpy::Element + Eq + Send + Sync,
{
    let gil = pyo3::Python::acquire_gil();
    let ground_truth = ground_truth.as_ref(gil.python());
    let ground_truth_ref = unsafe { ground_truth.as_slice().unwrap() };
    let predictions = predictions.as_ref(gil.python());
    let predictions_ref = unsafe { predictions.as_slice().unwrap() };
    pe!(::express_measures::accuracy_score(
        ground_truth_ref,
        predictions_ref,
    ))
}

#[module(preprocessing)]
#[pyfunction()]
#[text_signature = "(ground_truths, predictions)"]
/// Returns the accuracy score of the given predictions against the provided ground truth.
///
/// # Arguments
/// ground_truths: np.ndarray
///     Boolean 1D Numpy array with the ground truths classes.
/// predictions: np.ndarray
///     Boolean 1D Numpy array with the predicted classes.
///
fn binary_accuracy_score(
    ground_truths: Py<PyArray1<bool>>,
    predictions: Py<PyArray1<bool>>,
) -> PyResult<f32> {
    generic_accuracy_score(ground_truth, predictions)
}
