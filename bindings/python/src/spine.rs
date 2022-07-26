use super::*;
use cpu_models::{AnchorFeatureTypes, AnchorTypes, AnchorsInferredNodeEmbeddingModel, BasicSPINE};
use numpy::{PyArray1, PyArray2};

#[derive(Debug, Clone)]
pub struct BasicSPINEBinding<Model, const AFT: AnchorFeatureTypes, const AT: AnchorTypes>
where
    Model: AnchorsInferredNodeEmbeddingModel<AT, AFT>,
{
    pub inner: Model,
}

impl FromPyDict for BasicSPINE {
    fn from_pydict(py_kwargs: Option<&PyDict>) -> PyResult<Self> {
        let py = pyo3::Python::acquire_gil();
        let kwargs = normalize_kwargs!(py_kwargs, py.python());

        pe!(validate_kwargs(
            kwargs,
            &["embedding_size", "maximum_depth", "verbose"]
        ))?;

        pe!(BasicSPINE::new(
            extract_value_rust_result!(kwargs, "embedding_size", usize),
            extract_value_rust_result!(kwargs, "maximum_depth", usize),
            extract_value_rust_result!(kwargs, "verbose", bool),
        ))
    }
}

impl<Model, const AFT: AnchorFeatureTypes, const AT: AnchorTypes> FromPyDict
    for BasicSPINEBinding<Model, AFT, AT>
where
    Model: AnchorsInferredNodeEmbeddingModel<AT, AFT>,
    Model: From<BasicSPINE>,
{
    fn from_pydict(py_kwargs: Option<&PyDict>) -> PyResult<Self> {
        Ok(Self {
            inner: BasicSPINE::from_pydict(py_kwargs)?.into(),
        })
    }
}

macro_rules! impl_spine_embedding {
    ($($dtype:ty),*) => {
        impl<Model, const AFT: AnchorFeatureTypes, const AT: AnchorTypes> BasicSPINEBinding<Model, AFT, AT> where
            Model: AnchorsInferredNodeEmbeddingModel<AT, AFT>,
        {
            /// Return numpy embedding with SPINE node embedding.
            ///
            /// Do note that the embedding is returned transposed.
            ///
            /// Parameters
            /// --------------
            /// graph: Graph
            ///     The graph to embed.
            /// dtype: Optional[str] = None
            ///     Dtype to use for the embedding. Note that an improper dtype may cause overflows.
            ///     When not provided, we automatically infer the best one by using the diameter.
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
                    &["dtype",]
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

                let rows_number = graph.inner.get_number_of_nodes() as usize;
                let columns_number = pe!(self.inner.get_embedding_size(&graph.inner))?;
                match dtype {
                    $(
                        stringify!($dtype) => {
                            let embedding: &PyArray2<$dtype> = PyArray2::new(gil.python(), [rows_number, columns_number], true);

                            let embedding_slice = unsafe { embedding.as_slice_mut().unwrap() };

                            // We always use the racing version of the fit transform
                            // as we generally do not care about memory collisions.
                            pe!(self.inner.fit_transform(
                                &graph.inner,
                                embedding_slice,
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

impl_spine_embedding! {u8, u16, u32, u64}

#[pyclass]
#[derive(Debug, Clone)]
#[text_signature = "(*, embedding_size, maximum_depth, verbose)"]
pub struct DegreeSPINE {
    pub inner: BasicSPINEBinding<
        cpu_models::DegreeSPINE,
        { AnchorFeatureTypes::ShortestPaths },
        { AnchorTypes::Degrees },
    >,
}

#[pymethods]
impl DegreeSPINE {
    #[new]
    #[args(py_kwargs = "**")]
    /// Return a new instance of the DegreeSPINE model.
    ///
    /// Parameters
    /// ------------------------
    /// embedding_size: int = 100
    ///     Size of the embedding.
    /// maximum_depth: Optional[int] = None
    ///     Maximum depth of the shortest path.
    /// verbose: bool = True
    ///     Whether to show loading bars.
    pub fn new(py_kwargs: Option<&PyDict>) -> PyResult<DegreeSPINE> {
        Ok(Self {
            inner: BasicSPINEBinding::from_pydict(py_kwargs)?,
        })
    }

    #[args(py_kwargs = "**")]
    #[text_signature = "($self, graph, *, dtype)"]
    /// Return numpy embedding with Degree SPINE node embedding.
    ///
    /// Do note that the embedding is returned transposed.
    ///
    /// Parameters
    /// --------------
    /// graph: Graph
    ///     The graph to embed.
    /// dtype: Optional[str] = None
    ///     Dtype to use for the embedding. Note that an improper dtype may cause overflows.
    ///     When not provided, we automatically infer the best one by using the diameter.
    fn fit_transform(&self, graph: &Graph, py_kwargs: Option<&PyDict>) -> PyResult<Py<PyAny>> {
        self.inner.fit_transform(graph, py_kwargs)
    }
}

#[pyclass]
#[derive(Debug, Clone)]
#[text_signature = "(*, embedding_size, maximum_depth, verbose)"]
pub struct NodeLabelSPINE {
    pub inner: BasicSPINEBinding<
        cpu_models::NodeLabelSPINE,
        { AnchorFeatureTypes::ShortestPaths },
        { AnchorTypes::NodeTypes },
    >,
}

#[pymethods]
impl NodeLabelSPINE {
    #[new]
    #[args(py_kwargs = "**")]
    /// Return a new instance of the NodeLabelSPINE model.
    ///
    /// Parameters
    /// ------------------------
    /// embedding_size: int = 100
    ///     Size of the embedding.
    /// maximum_depth: Optional[int] = None
    ///     Maximum depth of the shortest path.
    /// verbose: bool = True
    ///     Whether to show loading bars.
    pub fn new(py_kwargs: Option<&PyDict>) -> PyResult<NodeLabelSPINE> {
        Ok(Self {
            inner: BasicSPINEBinding::from_pydict(py_kwargs)?,
        })
    }

    #[args(py_kwargs = "**")]
    #[text_signature = "($self, graph, *, dtype)"]
    /// Return numpy embedding with Degree SPINE node embedding.
    ///
    /// Do note that the embedding is returned transposed.
    ///
    /// Parameters
    /// --------------
    /// graph: Graph
    ///     The graph to embed.
    /// dtype: Optional[str] = None
    ///     Dtype to use for the embedding. Note that an improper dtype may cause overflows.
    ///     When not provided, we automatically infer the best one by using the diameter.
    fn fit_transform(&self, graph: &Graph, py_kwargs: Option<&PyDict>) -> PyResult<Py<PyAny>> {
        self.inner.fit_transform(graph, py_kwargs)
    }
}

#[pyclass]
#[derive(Debug, Clone)]
#[text_signature = "(*, embedding_size, maximum_depth, verbose)"]
pub struct ScoreSPINE {
    inner: BasicSPINE,
}

#[pymethods]
impl ScoreSPINE {
    #[new]
    #[args(py_kwargs = "**")]
    /// Return a new instance of the ScoreSPINE model.
    ///
    /// Parameters
    /// ------------------------
    /// embedding_size: int = 100
    ///     Size of the embedding.
    /// maximum_depth: Optional[int] = None
    ///     Maximum depth of the shortest path.
    /// verbose: bool = True
    ///     Whether to show loading bars.
    pub fn new(py_kwargs: Option<&PyDict>) -> PyResult<ScoreSPINE> {
        Ok(Self {
            inner: BasicSPINE::from_pydict(py_kwargs)?,
        })
    }

    #[args(py_kwargs = "**")]
    #[text_signature = "($self, scores, graph, *, dtype)"]
    /// Return numpy embedding with Degree SPINE node embedding.
    ///
    /// Do note that the embedding is returned transposed.
    ///
    /// Parameters
    /// --------------
    /// scores: np.ndarray
    ///     Scores to create the node groups.
    /// graph: Graph
    ///     The graph to embed.
    /// dtype: Optional[str] = None
    ///     Dtype to use for the embedding. Note that an improper dtype may cause overflows.
    ///     When not provided, we automatically infer the best one by using the diameter.
    fn fit_transform(
        &self,
        scores: Py<PyArray1<f32>>,
        graph: &Graph,
        py_kwargs: Option<&PyDict>,
    ) -> PyResult<Py<PyAny>> {
        let gil = pyo3::Python::acquire_gil();
        let scores_ref = scores.as_ref(gil.python());
        BasicSPINEBinding {
            inner: cpu_models::ScoreSPINE::new(self.inner.clone(), unsafe {
                scores_ref.as_slice().unwrap()
            }),
        }
        .fit_transform(graph, py_kwargs)
    }
}
