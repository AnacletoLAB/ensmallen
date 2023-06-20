use super::*;
use crate::mmap_numpy_npy::create_memory_mapped_numpy_array;
use crate::mmap_numpy_npy::Dtype;
use cpu_models::GraphConvolution as GC;
use cpu_models::MatrixShape;
use half::f16;
use num_traits::AsPrimitive;
use numpy::Element;
use std::convert::TryInto;

/// GraphConvolution model.
#[pyclass]
#[derive(Clone)]
#[pyo3(text_signature = "(*, number_of_convolutions, concatenate_features, dtype)")]
pub struct GraphConvolution {
    inner: GC,
}

#[pymethods]
impl GraphConvolution {
    #[new]
    #[args(py_kwargs = "**")]
    /// Return a new instance of the GraphConvolution model.
    ///
    /// Parameters
    /// ------------------------
    /// number_of_convolutions: int = 2
    ///     The number of convolutions to execute.
    /// concatenate_features: bool = False
    ///     Whether to concatenate the features at each convolution.
    ///     By default, `false`.
    /// normalize_rows: bool = True
    ///     Whether to normalize the rows between different convolutions.
    ///     By default, `true`.
    /// dtype: str = "f32"
    ///     The data type to use for the convolved features.
    ///     The supported values are `f16`, `f32` and `f64`.
    ///
    pub fn new(py_kwargs: Option<&PyDict>) -> PyResult<GraphConvolution> {
        let py = pyo3::Python::acquire_gil();
        let kwargs = normalize_kwargs!(py_kwargs, py.python());

        pe!(validate_kwargs(
            kwargs,
            &["number_of_convolutions", "concatenate_features",  "normalize_rows", "dtype"],
        ))?;

        Ok(Self {
            inner: pe!(GC::new(
                extract_value_rust_result!(kwargs, "number_of_convolutions", usize),
                extract_value_rust_result!(kwargs, "concatenate_features", bool),
                extract_value_rust_result!(kwargs, "normalize_rows", bool),
                extract_value_rust_result!(kwargs, "dtype", String),
            ))?,
        })
    }
}

impl GraphConvolution {
    fn _transform<
        F1: Send + Sync + Copy + Element + AsPrimitive<f64> + AsPrimitive<f32> + AsPrimitive<f16>,
    >(
        &self,
        support: &Graph,
        node_features: &PyArray2<F1>,
        path: Option<&str>,
    ) -> PyResult<Py<PyAny>> {
        let gil = Python::acquire_gil();
        if !node_features.is_c_contiguous() {
            return pe!(Err(concat!(
                "The provided node features is not a contiguos vector in ",
                "C orientation."
            )));
        }

        let dimensionality = node_features.shape()[1];
        let target_dimensionality = if self.inner.get_concatenate_features() {
            dimensionality * (1 + self.inner.get_number_of_convolutions())
        } else {
            dimensionality
        };
        let shape = MatrixShape::BiDimensional(
            support.get_number_of_nodes() as usize,
            target_dimensionality,
        );
        let data_type = pe!(self.inner.get_dtype().try_into())?;

        let convoluted_features = create_memory_mapped_numpy_array(
            gil.python(),
            path,
            data_type,
            &<MatrixShape as Into<Vec<isize>>>::into(shape),
            false,
        );

        let node_features_ref = unsafe { node_features.as_slice()? };
        match data_type {
            Dtype::F16 => {
                let convoluted_features_array =
                    convoluted_features.cast_as::<PyArray2<f16>>(gil.python())?;
                let convoluted_features_ref = unsafe { convoluted_features_array.as_slice_mut()? };
                pe!(self.inner.transform::<F1, f16>(
                    &support.inner,
                    node_features_ref,
                    dimensionality,
                    convoluted_features_ref,
                ))?;
            }
            Dtype::F32 => {
                let convoluted_features_array =
                    convoluted_features.cast_as::<PyArray2<f32>>(gil.python())?;
                let convoluted_features_ref = unsafe { convoluted_features_array.as_slice_mut()? };
                pe!(self.inner.transform::<F1, f32>(
                    &support.inner,
                    node_features_ref,
                    dimensionality,
                    convoluted_features_ref,
                ))?;
            }
            Dtype::F64 => {
                let convoluted_features_array =
                    convoluted_features.cast_as::<PyArray2<f64>>(gil.python())?;
                let convoluted_features_ref = unsafe { convoluted_features_array.as_slice_mut()? };
                pe!(self.inner.transform::<F1, f64>(
                    &support.inner,
                    node_features_ref,
                    dimensionality,
                    convoluted_features_ref,
                ))?;
            }
            this_type => {
                return pe!(Err(format!(
                    concat!(
                        "The provided data type {:?} is not supported. ",
                        "We expected f16, f32 or f64."
                    ),
                    this_type
                )));
            }
        }
        Ok(convoluted_features)
    }
}

#[pymethods]
impl GraphConvolution {
    #[args(py_kwargs = "**")]
    #[pyo3(text_signature = "($self,)")]
    /// Returns whether the features will be concatenated to the embeddings.
    fn get_concatenate_features(&self) -> bool {
        self.inner.get_concatenate_features()
    }

    #[args(py_kwargs = "**")]
    #[pyo3(text_signature = "($self,)")]
    /// Returns the number of convolutions to execute.
    fn get_number_of_convolutions(&self) -> usize {
        self.inner.get_number_of_convolutions()
    }

    #[pyo3(text_signature = "($self, support, node_features, path)")]
    /// Returns the convolved features.
    ///
    /// Parameters
    /// ------------------------
    /// support: &Graph
    ///     The graph whose topology is to be learned.
    /// node_features: np.ndarray
    ///     The node features.
    /// path: Option<&str>
    ///     The path where to mmap to the convolved features.
    ///
    /// Raises
    /// ------------------------
    /// ValueError
    ///     If the provided node features are not of the same length as the number of nodes.
    ///
    fn transform(
        &self,
        support: &Graph,
        node_features: Py<PyAny>,
        path: Option<&str>,
    ) -> PyResult<Py<PyAny>> {
        let gil = Python::acquire_gil();

        let node_features = node_features.as_ref(gil.python());
        if let Ok(node_features) = <&PyArray2<f16>>::extract(&node_features) {
            self._transform::<f16>(support, node_features, path)
        } else if let Ok(node_features) = <&PyArray2<f32>>::extract(&node_features) {
            self._transform::<f32>(support, node_features, path)
        } else if let Ok(node_features) = <&PyArray2<f64>>::extract(&node_features) {
            self._transform::<f64>(support, node_features, path)
        } else if let Ok(node_features) = <&PyArray2<u8>>::extract(&node_features) {
            self._transform::<u8>(support, node_features, path)
        } else if let Ok(node_features) = <&PyArray2<u16>>::extract(&node_features) {
            self._transform::<u16>(support, node_features, path)
        } else if let Ok(node_features) = <&PyArray2<u32>>::extract(&node_features) {
            self._transform::<u32>(support, node_features, path)
        } else if let Ok(node_features) = <&PyArray2<u64>>::extract(&node_features) {
            self._transform::<u64>(support, node_features, path)
        } else if let Ok(node_features) = <&PyArray2<i8>>::extract(&node_features) {
            self._transform::<i8>(support, node_features, path)
        } else if let Ok(node_features) = <&PyArray2<i16>>::extract(&node_features) {
            self._transform::<i16>(support, node_features, path)
        } else if let Ok(node_features) = <&PyArray2<i32>>::extract(&node_features) {
            self._transform::<i32>(support, node_features, path)
        } else if let Ok(node_features) = <&PyArray2<i64>>::extract(&node_features) {
            self._transform::<i64>(support, node_features, path)
        } else {
            pe!(Err(concat!(
                "The provided node features are not a supported type. ",
                "We expected a 2D numpy array of type f16, f32 or f64, or ",
                "u8, u16, u32, u64, i8, i16, i32 or i64."
            )))
        }
    }

    #[staticmethod]
    #[pyo3(text_signature = "(path,)")]
    /// Loads model from the provided path.
    ///
    /// Parameters
    /// ----------------
    /// path: str
    ///     Path from where to load the model.
    fn load(path: String) -> PyResult<Self> {
        Ok(GraphConvolution {
            inner: pe!(GC::load(path.as_ref()))?,
        })
    }

    #[staticmethod]
    #[pyo3(text_signature = "(json,)")]
    /// Loads model from provided JSON string.
    ///
    /// Parameters
    /// ----------------
    /// json: str
    ///     JSON string containing model metadata.
    fn loads(json: String) -> PyResult<Self> {
        Ok(GraphConvolution {
            inner: pe!(GC::loads(json.as_str()))?,
        })
    }

    #[pyo3(text_signature = "(&self, path)")]
    /// Dump model to the provided path.
    ///
    /// Parameters
    /// ----------------
    /// path: str
    ///     Path where to dump the model.
    fn dump(&self, path: String) -> PyResult<()> {
        pe!(self.inner.dump(path.as_ref()))
    }

    #[pyo3(text_signature = "(&self)")]
    /// Dumps model to JSON string.
    fn dumps(&self) -> PyResult<String> {
        pe!(self.inner.dumps())
    }
}
