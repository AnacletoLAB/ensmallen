use super::*;
use numpy::PyArray2;

///
#[pyclass]
#[derive(Debug, Clone)]
#[text_signature = "(*, embedding_size)"]
pub struct SPINE {
    pub inner: cpu_models::SPINE,
}

impl From<cpu_models::SPINE> for SPINE {
    fn from(val: cpu_models::SPINE) -> SPINE {
        SPINE { inner: val }
    }
}

impl From<SPINE> for cpu_models::SPINE {
    fn from(val: SPINE) -> cpu_models::SPINE {
        val.inner
    }
}

#[pymethods]
impl SPINE {
    #[new]
    #[args(py_kwargs = "**")]
    /// Return a new instance of the SPINE model.
    ///
    /// Parameters
    /// ------------------------
    /// embedding_size: Optional[int] = 100
    ///     Size of the embedding.
    pub fn new(py_kwargs: Option<&PyDict>) -> PyResult<SPINE> {
        let py = pyo3::Python::acquire_gil();
        let kwargs = normalize_kwargs!(py_kwargs, py.python());

        pe!(validate_kwargs(kwargs, &["embedding_size",]))?;

        Ok(Self {
            inner: pe!(cpu_models::SPINE::new(extract_value_rust_result!(
                kwargs,
                "embedding_size",
                usize
            ),))?,
        })
    }
}

macro_rules! impl_spine_embedding {
    ($($dtype:ty),*) => {
        #[pymethods]
        impl SPINE {
            #[args(py_kwargs = "**")]
            #[text_signature = "($self, graph, *, dtype, verbose)"]
            /// Return numpy embedding with SPINE node embedding.
            ///
            /// Do note that the embedding is returned transposed.
            ///
            /// Parameters
            /// ---------
            /// graph: Graph
            ///     The graph to embed.
            /// dtype: Optional[str] = None
            ///     Dtype to use for the embedding. Note that an improper dtype may cause overflows.
            ///     When not provided, we automatically infer the best one by using the diameter.
            /// verbose: Optional[bool] = True
            ///     Whether to show the loading bar. By default true.
            fn fit_transform(
                &self,
                graph: &Graph,
                py_kwargs: Option<&PyDict>,
            ) -> PyResult<Py<PyAny>> {
                let gil = pyo3::Python::acquire_gil();

                let py = pyo3::Python::acquire_gil();
                let kwargs = normalize_kwargs!(py_kwargs, py.python());

                pe!(validate_kwargs(
                    kwargs,
                    &["dtype", "verbose"]
                ))?;

                let verbose = extract_value_rust_result!(kwargs, "verbose", bool);
                let dtype = match extract_value_rust_result!(kwargs, "dtype", &str) {
                    Some(dtype) => dtype,
                    None => {
                        let (max_u8, max_u16, max_u32) = (u8::MAX as usize, u16::MAX as usize, u32::MAX as usize);
                        match pe!(graph.get_diameter(Some(true), verbose))? as usize {
                            x if (0..=max_u8).contains(&x) => "u8",
                            x if (max_u8..=max_u16).contains(&x) => "u16",
                            x if (max_u16..=max_u32).contains(&x) => "u32",
                            _ => "u64",
                        }
                    }
                };

                let rows_number = graph.inner.get_nodes_number() as usize;
                let columns_number = self.inner.get_embedding_size();
                match dtype {
                    $(
                        stringify!($dtype) => {
                            let embedding: &PyArray2<$dtype> = PyArray2::new(gil.python(), [columns_number, rows_number], false);

                            let embedding_slice = unsafe { embedding.as_slice_mut().unwrap() };

                            // We always use the racing version of the fit transform
                            // as we generally do not care about memory collisions.
                            pe!(self.inner.fit_transform(
                                &graph.inner,
                                embedding_slice,
                                verbose,
                            ))?;

                            Ok(embedding.into_py(gil.python()))
                        }
                    )*
                    dtype => pe!(Err(format!(
                        concat!(
                            "The provided dtype {} is not supported. The supported ",
                            "data types are `u8`, `u16`, `u32` and `u64`."
                        ),
                        dtype
                    ))),
                }

            }
        }
    };
}

impl_spine_embedding! {u8, u16, u32}
