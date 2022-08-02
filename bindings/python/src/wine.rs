use super::*;
use cpu_models::{AnchorFeatureTypes, AnchorTypes, AnchorsInferredNodeEmbeddingModel, BasicWINE};
use numpy::{PyArray1, PyArray2};

use super::mmap_numpy_npy::{create_memory_mapped_numpy_array, Dtype};

#[derive(Debug, Clone)]
pub struct BasicWINEBinding<Model, const AFT: AnchorFeatureTypes, const AT: AnchorTypes>
where
    Model: AnchorsInferredNodeEmbeddingModel<AT, AFT>,
{
    pub inner: Model,
    pub path: Option<String>,
}

impl FromPyDict for BasicWINE {
    fn from_pydict(py_kwargs: Option<&PyDict>) -> PyResult<Self> {
        let py = pyo3::Python::acquire_gil();
        let kwargs = normalize_kwargs!(py_kwargs, py.python());

        pe!(validate_kwargs(
            kwargs,
            &["embedding_size", "walk_length", "path", "verbose"]
        ))?;

        pe!(BasicWINE::new(
            extract_value_rust_result!(kwargs, "embedding_size", usize),
            extract_value_rust_result!(kwargs, "walk_length", usize),
            extract_value_rust_result!(kwargs, "verbose", bool),
        ))
    }
}

impl<Model, const AFT: AnchorFeatureTypes, const AT: AnchorTypes> FromPyDict
    for BasicWINEBinding<Model, AFT, AT>
where
    Model: AnchorsInferredNodeEmbeddingModel<AT, AFT>,
    Model: From<BasicWINE>,
{
    fn from_pydict(py_kwargs: Option<&PyDict>) -> PyResult<Self> {
        Ok(Self {
            inner: BasicWINE::from_pydict(py_kwargs)?.into(),
            path: match py_kwargs {
                None => None,
                Some(kwargs) => {
                    extract_value_rust_result!(kwargs, "path", String)
                }
            },
        })
    }
}

macro_rules! impl_WINE_embedding {
    ($($dtype:ty : $dtype_enum:expr),*) => {
        impl<Model, const AFT: AnchorFeatureTypes, const AT: AnchorTypes> BasicWINEBinding<Model, AFT, AT> where
            Model: AnchorsInferredNodeEmbeddingModel<AT, AFT>,
        {
            /// Return numpy embedding with WINE node embedding.
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
                let kwargs = normalize_kwargs!(py_kwargs, gil.python());

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

                let rows_number = graph.inner.get_number_of_nodes() as isize;
                let columns_number = pe!(self.inner.get_embedding_size(&graph.inner))? as isize;
                match dtype {
                    $(
                        stringify!($dtype) => {
                            let embedding = create_memory_mapped_numpy_array(
                                gil.python(),
                                self.path.as_ref().map(|x| x.as_str()),
                                $dtype_enum,
                                vec![rows_number, columns_number],
                                true,
                            );

                            let s = embedding.cast_as::<PyArray2<$dtype>>(gil.python())?;

                            let embedding_slice = unsafe { s.as_slice_mut().unwrap() };

                            pe!(self.inner.fit_transform(
                                &graph.inner,
                                embedding_slice,
                            ))?;

                            Ok(embedding)
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

impl_WINE_embedding! {
    u8 : Dtype::U8,
    u16: Dtype::U16,
    u32: Dtype::U32,
    u64: Dtype::U64
}

#[pyclass]
#[derive(Debug, Clone)]
#[pyo3(text_signature = "(*, embedding_size, walk_length, verbose)")]
pub struct DegreeWINE {
    pub inner: BasicWINEBinding<
        cpu_models::DegreeWINE,
        { AnchorFeatureTypes::Walks },
        { AnchorTypes::Degrees },
    >,
}

#[pymethods]
impl DegreeWINE {
    #[new]
    #[args(py_kwargs = "**")]
    /// Return a new instance of the DegreeWINE model.
    ///
    /// Parameters
    /// ------------------------
    /// embedding_size: int = 100
    ///     Size of the embedding.
    /// walk_length: int = 2
    ///     Length of the random walk.
    ///     By default 2, to capture exclusively the immediate context.
    /// verbose: bool = True
    ///     Whether to show loading bars.
    pub fn new(py_kwargs: Option<&PyDict>) -> PyResult<DegreeWINE> {
        Ok(Self {
            inner: BasicWINEBinding::from_pydict(py_kwargs)?,
        })
    }

    #[args(py_kwargs = "**")]
    #[pyo3(text_signature = "($self, graph, *, dtype)")]
    /// Return numpy embedding with Degree WINE node embedding.
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
#[pyo3(text_signature = "(*, embedding_size, walk_length, verbose)")]
pub struct NodeLabelWINE {
    pub inner: BasicWINEBinding<
        cpu_models::NodeLabelWINE,
        { AnchorFeatureTypes::Walks },
        { AnchorTypes::NodeTypes },
    >,
}

#[pymethods]
impl NodeLabelWINE {
    #[new]
    #[args(py_kwargs = "**")]
    /// Return a new instance of the NodeLabelWINE model.
    ///
    /// Parameters
    /// ------------------------
    /// embedding_size: int = 100
    ///     Size of the embedding.
    /// walk_length: int = 2
    ///     Length of the random walk.
    ///     By default 2, to capture exclusively the immediate context.
    /// verbose: bool = True
    ///     Whether to show loading bars.
    pub fn new(py_kwargs: Option<&PyDict>) -> PyResult<NodeLabelWINE> {
        Ok(Self {
            inner: BasicWINEBinding::from_pydict(py_kwargs)?,
        })
    }

    #[args(py_kwargs = "**")]
    #[pyo3(text_signature = "($self, graph, *, dtype)")]
    /// Return numpy embedding with Degree WINE node embedding.
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
#[pyo3(text_signature = "(*, embedding_size, walk_length, path, verbose)")]
pub struct ScoreWINE {
    inner: BasicWINE,
    path: Option<String>,
}

#[pymethods]
impl ScoreWINE {
    #[new]
    #[args(py_kwargs = "**")]
    /// Return a new instance of the ScoreWINE model.
    ///
    /// Parameters
    /// ------------------------
    /// embedding_size: int = 100
    ///     Size of the embedding.
    /// walk_length: int = 2
    ///     Length of the random walk.
    ///     By default 2, to capture exclusively the immediate context.
    /// path: Optional[str] = None
    ///     If passed, create a `.npy` file which will be mem-mapped
    ///     to allow processing embeddings that do not fit in RAM
    /// verbose: bool = True
    ///     Whether to show loading bars.
    pub fn new(py_kwargs: Option<&PyDict>) -> PyResult<ScoreWINE> {
        Ok(Self {
            inner: BasicWINE::from_pydict(py_kwargs)?,
            path: match py_kwargs {
                None => None,
                Some(kwargs) => {
                    extract_value_rust_result!(kwargs, "path", String)
                }
            },
        })
    }

    #[args(py_kwargs = "**")]
    #[pyo3(text_signature = "($self, scores, graph, *, dtype)")]
    /// Return numpy embedding with Degree WINE node embedding.
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
        BasicWINEBinding {
            inner: cpu_models::ScoreWINE::new(self.inner.clone(), unsafe {
                scores_ref.as_slice().unwrap()
            }),
            path: self.path.clone(),
        }
        .fit_transform(graph, py_kwargs)
    }
}
