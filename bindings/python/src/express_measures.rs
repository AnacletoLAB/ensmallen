use super::*;
use crate::mmap_numpy_npy::to_numpy_array;
use numpy::{PyArray1, PyArray2};
use pyo3::wrap_pyfunction;

macro_rules! impl_express_measures {
    ($(($method_name:ident, $function_name:ident),)*) => {

pub fn register_express_measures(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<BinaryConfusionMatrix>()?;
    $(
        m.add_wrapped(wrap_pyfunction!($function_name))?;
    )*
    m.add_wrapped(wrap_pyfunction!(all_binary_metrics))?;
    m.add_wrapped(wrap_pyfunction!(binary_auroc))?;
    m.add_wrapped(wrap_pyfunction!(binary_auprc))?;
    m.add_wrapped(wrap_pyfunction!(cosine_similarity_from_indices_unchecked))?;
    m.add_wrapped(wrap_pyfunction!(pairwise_cosine_similarity))?;
    Ok(())
}

#[pyclass]
#[derive(Debug, Clone)]
pub struct BinaryConfusionMatrix {
    pub inner: ::express_measures::BinaryConfusionMatrix,
}

#[pymethods]
impl BinaryConfusionMatrix {
$(
    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Return the $function_name
    pub fn $method_name(&self) -> f64 {
        self.inner.$method_name() as f64
    }
)*

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Return a dictionary with all the binary metrics
    pub fn get_all_binary_metrics(&self) -> HashMap<String, f64> {
        self.inner.get_all_binary_metrics()
    }
}

$(
    #[module(express_measures)]
    #[pyfunction()]
    #[pyo3(text_signature = "(ground_truths, predictions)")]
    /// Returns the $function_name of the given binary predictions against the provided binary ground truth.
    ///
    /// Arguments
    /// ---------
    /// ground_truths: np.ndarray
    ///     Boolean 1D Numpy array with the ground truths classes.
    /// predictions: np.ndarray
    ///     Boolean 1D Numpy array with the predicted classes or the prediction
    ///     probabilities.
    /// threshold: float = 0.5
    ///     The cuttoff probability we use to distinguis positives from negatives.
    ///
    /// Raises
    /// ------
    /// ValueError: if you expressely pass a threshold but the given predictions
    ///     are already bools.
    ///
    fn $function_name(
        ground_truths: Py<PyArray1<bool>>,
        predictions: Py<PyAny>,
        threshold: Option<f64>
    ) -> PyResult<f64> {
        let gil = pyo3::Python::acquire_gil();
        let ground_truths = ground_truths.as_ref(gil.python());
        let ground_truths_ref = unsafe { ground_truths.as_slice().unwrap() };
        let predictions = predictions.as_ref(gil.python());

        let matrix = if let Ok(casted_pred) = <&PyArray1<f32>>::extract(&predictions) {
            ::express_measures::BinaryConfusionMatrix::from_probabilities_slices(
                ground_truths_ref,
                unsafe{casted_pred.as_slice().unwrap()},
                threshold.unwrap_or(0.5) as f32,
            )
        } else if let Ok(casted_pred) = <&PyArray1<f64>>::extract(&predictions) {
            ::express_measures::BinaryConfusionMatrix::from_probabilities_slices(
                ground_truths_ref,
                unsafe{casted_pred.as_slice().unwrap()},
                threshold.unwrap_or(0.5),
            )
        } else if let Ok(casted_pred) = <&PyArray1<bool>>::extract(&predictions){
            if threshold.is_some() {
                return pe!(Err(format!(concat!(
                    "Cannot compute $fuction_name on boolean predictions with threshold {}.",
                    "If you want to use boolean predictions remove the threshold.",
                    ), threshold.unwrap(),
                )));
            }

            ::express_measures::BinaryConfusionMatrix::from_binary_slices(
               ground_truths_ref,
               unsafe{casted_pred.as_slice().unwrap()},
           )
        } else {
            Err(format!(
                "Invalid prediction type '{}' the predictions can only be numpy 1D arrays with dtype either np.float32 of bool",
                pe!(predictions.get_type().name().map_err(|error| error.to_string()))?
            ))
        };

        Ok(pe!(matrix)?.$method_name() as f64)
    }
)*

#[module(express_measures)]
#[pyfunction()]
#[pyo3(text_signature = "(ground_truths, predictions)")]
/// Returns the $function_name of the given binary predictions against the provided binary ground truth.
///
/// Arguments
/// ---------
/// ground_truths: np.ndarray
///     Boolean 1D Numpy array with the ground truths classes.
/// predictions: np.ndarray
///     Boolean 1D Numpy array with the predicted classes or the prediction
///     probabilities.
/// threshold: float = 0.5
///     The cuttoff probability we use to distinguis positives from negatives.
///
/// Raises
/// ------
/// ValueError: if you expressely pass a threshold but the given predictions
///     are already bools.
///
fn all_binary_metrics(
    ground_truths: Py<PyArray1<bool>>,
    predictions: Py<PyAny>,
    threshold: Option<f64>
) -> PyResult<HashMap<String, f64>> {
    let gil = pyo3::Python::acquire_gil();
    let ground_truths = ground_truths.as_ref(gil.python());
    let ground_truths_ref = unsafe { ground_truths.as_slice().unwrap() };
    let predictions = predictions.as_ref(gil.python());

    let matrix = if let Ok(casted_pred) = <&PyArray1<f32>>::extract(&predictions) {
        ::express_measures::BinaryConfusionMatrix::from_probabilities_slices(
            ground_truths_ref,
            unsafe{casted_pred.as_slice().unwrap()},
            threshold.unwrap_or(0.5) as f32,
        )
    } else if let Ok(casted_pred) = <&PyArray1<f64>>::extract(&predictions) {
        ::express_measures::BinaryConfusionMatrix::from_probabilities_slices(
            ground_truths_ref,
            unsafe{casted_pred.as_slice().unwrap()},
            threshold.unwrap_or(0.5),
        )
    } else if let Ok(casted_pred) = <&PyArray1<bool>>::extract(&predictions){
        if threshold.is_some() {
            return pe!(Err(format!(concat!(
                "Cannot compute $fuction_name on boolean predictions with threshold {}.",
                "If you want to use boolean predictions remove the threshold.",
                ), threshold.unwrap(),
            )));
        }

        ::express_measures::BinaryConfusionMatrix::from_binary_slices(
           ground_truths_ref,
           unsafe{casted_pred.as_slice().unwrap()},
       )
    } else {
        Err(format!(
            "Invalid prediction type '{}' the predictions can only be numpy 1D arrays with dtype either np.float32 of bool",
            pe!(predictions.get_type().name().map_err(|error| error.to_string()))?
        ))
    };

    Ok(pe!(matrix)?.get_all_binary_metrics())
}

#[module(express_measures)]
#[pyfunction()]
#[pyo3(text_signature = "(ground_truths, predictions)")]
/// Returns the binary auroc of the given predictions against the provided binary ground truth.
///
/// Arguments
/// ---------
/// ground_truths: np.ndarray
///     Boolean 1D Numpy array with the ground truths classes.
/// predictions: np.ndarray
///     Boolean 1D Numpy array with the predicted classes.
///
fn binary_auroc(
    ground_truths: Py<PyArray1<bool>>,
    predictions: Py<PyAny>,
) -> PyResult<f64> {
    let gil = pyo3::Python::acquire_gil();
    let ground_truths = ground_truths.as_ref(gil.python());
    let ground_truths_ref = unsafe { ground_truths.as_slice().unwrap() };
    let predictions = predictions.as_ref(gil.python());

    pe!(if let Ok(casted_pred) = <&PyArray1<f32>>::extract(&predictions) {
        ::express_measures::get_binary_auroc(
            ground_truths_ref,
            unsafe{casted_pred.as_slice().unwrap()},
        )
    } else if let Ok(casted_pred) = <&PyArray1<f64>>::extract(&predictions) {
        ::express_measures::get_binary_auroc(
            ground_truths_ref,
            unsafe{casted_pred.as_slice().unwrap()},
        )
    } else {
        Err(format!(
            "Invalid prediction type '{}' the predictions can only be numpy 1D arrays with dtype either np.float32 of bool",
            pe!(predictions.get_type().name().map_err(|error| error.to_string()))?
        ))
    })
}

#[module(express_measures)]
#[pyfunction()]
#[pyo3(text_signature = "(ground_truths, predictions)")]
/// Returns the binary auprc of the given predictions against the provided binary ground truth.
///
/// Arguments
/// ---------
/// ground_truths: np.ndarray
///     Boolean 1D Numpy array with the ground truths classes.
/// predictions: np.ndarray
///     Boolean 1D Numpy array with the predicted classes.
///
fn binary_auprc(
    ground_truths: Py<PyArray1<bool>>,
    predictions: Py<PyAny>,
) -> PyResult<f64> {
    let gil = pyo3::Python::acquire_gil();
    let ground_truths = ground_truths.as_ref(gil.python());
    let ground_truths_ref = unsafe { ground_truths.as_slice().unwrap() };
    let predictions = predictions.as_ref(gil.python());

    pe!(if let Ok(casted_pred) = <&PyArray1<f32>>::extract(&predictions) {
        ::express_measures::get_binary_auprc(
            ground_truths_ref,
            unsafe{casted_pred.as_slice().unwrap()},
        )
    } else if let Ok(casted_pred) = <&PyArray1<f64>>::extract(&predictions) {
        ::express_measures::get_binary_auprc(
            ground_truths_ref,
            unsafe{casted_pred.as_slice().unwrap()},
        )
    } else {
        Err(format!(
            "Invalid prediction type '{}' the predictions can only be numpy 1D arrays with dtype either np.float32 of bool",
            pe!(predictions.get_type().name().map_err(|error| error.to_string()))?
        ))
    })
}

};
}

macro_rules! impl_cosine_distance {
    ($($dtype:ty),*) => {
        #[module(express_measures)]
        #[pyfunction()]
        #[pyo3(text_signature = "(matrix, sources, destinations)")]
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
            matrix: Py<PyAny>,
            sources: Py<PyArray1<u32>>,
            destinations: Py<PyArray1<u32>>,
        ) -> PyResult<Py<PyArray1<f32>>> {
            let gil = pyo3::Python::acquire_gil();
            let matrix = matrix.as_ref(gil.python());
            let sources = sources.as_ref(gil.python());
            let sources_ref = unsafe { sources.as_slice().unwrap() };
            let destinations = destinations.as_ref(gil.python());
            let destinations_ref = unsafe { destinations.as_slice().unwrap() };
            let similarities = unsafe { PyArray1::new(gil.python(), [sources_ref.len()], false) };
            let similarities_ref = unsafe { similarities.as_slice_mut().unwrap() };
            $(
                if let Ok(matrix) = <&PyArray2<$dtype>>::extract(&matrix) {

                    if !matrix.is_c_contiguous(){
                        return pe!(Err(
                            concat!(
                                "The provided vector is not a contiguos vector in ",
                                "C orientation."
                            )
                        ));
                    }

                    let matrix_ref = unsafe { matrix.as_slice().unwrap() };

                    pe!(unsafe {
                        ::express_measures::cosine_similarity_from_indices_unchecked(
                            similarities_ref,
                            matrix_ref,
                            sources_ref,
                            destinations_ref,
                            matrix.shape()[1],
                        )
                    })?;

                    return Ok(similarities.to_owned());
                }
            )*

            pe!(Err(concat!(
                "The provided features are not supported ",
                "in the cosine similarity computation!"
            ).to_string()))
        }

        #[module(express_measures)]
        #[pyfunction()]
        #[pyo3(text_signature = "(matrix, sources, destinations, minimum_threshold, maximum_threshold, verbose)")]
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
        /// lower_threshold: float = 1.0
        ///     Only returns values that are lower than this score. By default `1.0`.
        /// higher_threshold`: float = -1.0
        ///     Only returns values that are higher than this score. By default `-1.0`.
        /// verbose: bool = True
        ///     Whether to show loading bars.
        ///
        fn pairwise_cosine_similarity(
            matrix: Py<PyAny>,
            sources: Py<PyAny>,
            destinations: Py<PyAny>,
            minimum_threshold: Option<f32>,
            maximum_threshold: Option<f32>,
            verbose: Option<bool>
        ) -> PyResult<(Py<PyAny>, Py<PyArray1<f32>>)> {
            let gil = pyo3::Python::acquire_gil();
            let matrix = matrix.as_ref(gil.python());
            let sources = sources.as_ref(gil.python());
            let destinations = destinations.as_ref(gil.python());
            $(
                if let Ok(matrix) = <&PyArray2<$dtype>>::extract(&matrix) {

                    if !matrix.is_c_contiguous(){
                        return pe!(Err(
                            concat!(
                                "The provided vector is not a contiguos vector in ",
                                "C orientation."
                            )
                        ));
                    }

                    let matrix_ref = unsafe { matrix.as_slice().unwrap() };

                    if let (Ok(sources), Ok(destinations)) = (
                        <&PyArray1<u8>>::extract(&sources),
                        <&PyArray1<u8>>::extract(&destinations)
                    ) {

                        let sources_ref = unsafe { sources.as_slice().unwrap() };
                        let destinations_ref = unsafe { destinations.as_slice().unwrap() };

                        let (edge_node_ids, similarities) = pe!(
                            ::express_measures::pairwise_cosine_similarity(
                                matrix_ref,
                                sources_ref,
                                destinations_ref,
                                matrix.shape()[1],
                                minimum_threshold,
                                maximum_threshold,
                                verbose
                            )
                        )?;

                        return Ok((
                            pe!(to_numpy_array(
                                gil.python(),
                                edge_node_ids,
                                &[similarities.len(), 2],
                                false
                            ))?,
                            to_ndarray_1d!(gil, similarities, f32),
                        ))
                    }

                    if let (Ok(sources), Ok(destinations)) = (
                        <&PyArray1<u16>>::extract(&sources),
                        <&PyArray1<u16>>::extract(&destinations)
                    ) {

                        let sources_ref = unsafe { sources.as_slice().unwrap() };
                        let destinations_ref = unsafe { destinations.as_slice().unwrap() };

                        let (edge_node_ids, similarities) = pe!(
                            ::express_measures::pairwise_cosine_similarity(
                                matrix_ref,
                                sources_ref,
                                destinations_ref,
                                matrix.shape()[1],
                                minimum_threshold,
                                maximum_threshold,
                                verbose
                            )
                        )?;

                        return Ok((
                            pe!(to_numpy_array(
                                gil.python(),
                                edge_node_ids,
                                &[similarities.len(), 2],
                                false
                            ))?,
                            to_ndarray_1d!(gil, similarities, f32),
                        ))
                    }

                    if let (Ok(sources), Ok(destinations)) = (
                        <&PyArray1<u32>>::extract(&sources),
                        <&PyArray1<u32>>::extract(&destinations)
                    ) {

                        let sources_ref = unsafe { sources.as_slice().unwrap() };
                        let destinations_ref = unsafe { destinations.as_slice().unwrap() };

                        let (edge_node_ids, similarities) = pe!(
                            ::express_measures::pairwise_cosine_similarity(
                                matrix_ref,
                                sources_ref,
                                destinations_ref,
                                matrix.shape()[1],
                                minimum_threshold,
                                maximum_threshold,
                                verbose
                            )
                        )?;

                        return Ok((
                            pe!(to_numpy_array(
                                gil.python(),
                                edge_node_ids,
                                &[similarities.len(), 2],
                                false
                            ))?,
                            to_ndarray_1d!(gil, similarities, f32),
                        ))
                    }

                    if let (Ok(sources), Ok(destinations)) = (
                        <&PyArray1<u64>>::extract(&sources),
                        <&PyArray1<u64>>::extract(&destinations)
                    ) {

                        let sources_ref = unsafe { sources.as_slice().unwrap() };
                        let destinations_ref = unsafe { destinations.as_slice().unwrap() };

                        let (edge_node_ids, similarities) = pe!(
                            ::express_measures::pairwise_cosine_similarity(
                                matrix_ref,
                                sources_ref,
                                destinations_ref,
                                matrix.shape()[1],
                                minimum_threshold,
                                maximum_threshold,
                                verbose
                            )
                        )?;

                        return Ok((
                            pe!(to_numpy_array(
                                gil.python(),
                                edge_node_ids,
                                &[similarities.len(), 2],
                                false
                            ))?,
                            to_ndarray_1d!(gil, similarities, f32),
                        ))
                    }

                    return pe!(Err(concat!(
                        "The provided sources or destination do not have ",
                        "a datatype currently supported in the cosine similarity computation!"
                    ).to_string()));
                }
            )*

            pe!(Err(concat!(
                "The provided features do not have ",
                "a datatype currently supported in the cosine similarity computation!"
            ).to_string()))
        }
    };
}

impl_cosine_distance! {
    u8, u16, u32, u64, i8, i16, i32, i64, f32, f64
}

impl_express_measures! {
    (get_number_of_true_positives, number_of_true_positives),
    (get_number_of_true_negatives, number_of_true_negatives),
    (get_number_of_false_positives, number_of_false_positives),
    (get_number_of_false_negatives, number_of_false_negatives),
    (get_number_of_positive_values, number_of_positive_values),
    (get_number_of_negative_values, number_of_negative_values),
    (get_number_of_positive_predictions, number_of_positive_predictions),
    (get_number_of_negative_predictions, number_of_negative_predictions),
    (get_number_of_correct_predictions, number_of_correct_predictions),
    (get_number_of_incorrect_predictions, number_of_incorrect_predictions),
    (get_number_of_samples, number_of_samples),
    (get_binary_accuracy, binary_accuracy),
    (get_binary_recall, binary_recall),
    (get_binary_specificity, binary_specificity),
    (get_binary_miss_rate, binary_miss_rate),
    (get_binary_fall_out, binary_fall_out),
    (get_binary_informedness, binary_informedness),
    (get_binary_prevalence_threshold, binary_prevalence_threshold),
    (get_binary_prevalence, binary_prevalence),
    (get_binary_balanced_accuracy, binary_balanced_accuracy),
    (get_binary_precision, binary_precision),
    (get_binary_false_discovery_rate, binary_false_discovery_rate),
    (get_binary_false_omission_rate, binary_false_omission_rate),
    (get_binary_negative_predictive_value, binary_negative_predictive_value),
    (get_binary_positive_likelyhood_ratio, binary_positive_likelyhood_ratio),
    (get_binary_negative_likelyhood_ratio, binary_negative_likelyhood_ratio),
    (get_binary_markedness, binary_markedness),
    (get_binary_diagnostic_odds_ratio, binary_diagnostic_odds_ratio),
    (get_binary_f1_score, binary_f1_score),
    (get_binary_fowlkes_mallows_index, binary_fowlkes_mallows_index),
    (get_binary_threat_score, binary_threat_score),
    (get_binary_matthews_correlation_coefficient, binary_matthews_correlation_coefficient),
}
