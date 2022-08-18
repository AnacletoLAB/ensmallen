use super::mmap_numpy_npy::{
    create_memory_mapped_numpy_array, init_memory_mapped_numpy_array,
    load_memory_mapped_numpy_array, Dtype,
};
use super::*;
use cpu_models::{
    AnchorFeatureTypes, AnchorTypes, AnchorsInferredNodeEmbeddingModel, BasicSPINE, BasicWINE,
};
use indicatif::ProgressIterator;
use indicatif::{ProgressBar, ProgressStyle};
use mmap::*;
use numpy::{PyArray1, PyArray2};
use rayon::prelude::*;
use std::convert::TryFrom;
use types::ThreadDataRaceAware;

#[derive(Debug, Clone)]
pub struct BasicSPINEBinding<Model, const AFT: AnchorFeatureTypes, const AT: AnchorTypes>
where
    Model: AnchorsInferredNodeEmbeddingModel<AT, AFT>,
{
    pub inner: Model,
    pub path: Option<String>,
}

impl FromPyDict for BasicSPINE {
    fn from_pydict(py_kwargs: Option<&PyDict>) -> PyResult<Self> {
        let py = pyo3::Python::acquire_gil();
        let kwargs = normalize_kwargs!(py_kwargs, py.python());

        pe!(validate_kwargs(
            kwargs,
            &["embedding_size", "maximum_depth", "path", "verbose"]
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
            path: match py_kwargs {
                None => None,
                Some(kwargs) => {
                    extract_value_rust_result!(kwargs, "path", String)
                }
            },
        })
    }
}

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

macro_rules! impl_alpine_embedding {
    ($($dtype:ty : $dtype_enum:expr),*) => {
pub trait BaseALPINEBinding<Model, const AFT: AnchorFeatureTypes, const AT: AnchorTypes>
where
    Model: AnchorsInferredNodeEmbeddingModel<AT, AFT> {

    fn get_model(&self) -> &Model;

    fn get_path(&self) -> Option<String>;

    /// Initialize numpy embedding for provided graph at path.
    ///
    /// Do note that the embedding is in FORTRAN format.
    ///
    /// Parameters
    /// --------------
    /// graph: Graph
    ///     The graph to embed.
    /// dtype: Optional[str] = None
    ///     Dtype to use for the embedding. Note that an improper dtype may cause overflows.
    ///     When not provided, we automatically infer the best one by using the diameter.
    /// verbose: bool = False
    ///     Whether to show loading bars.
    fn init_mmap(&self, graph: &Graph, py_kwargs: Option<&PyDict>,) -> PyResult<usize> {
        self.must_have_path()?;

        let gil = pyo3::Python::acquire_gil();

        let number_of_nodes = graph.inner.get_number_of_nodes() as isize;
        let embedding_size = pe!(self.get_model().get_embedding_size(&graph.inner))? as isize;

        let kwargs = normalize_kwargs!(py_kwargs, gil.python());

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

        match dtype {
            $(
                stringify!($dtype) => {
                    let (_, aligned_size) = init_memory_mapped_numpy_array(
                        gil.python(),
                        self.get_path().as_ref().map(|path| path.as_str()),
                        $dtype_enum,
                        &[number_of_nodes as isize, embedding_size as isize],
                        true,
                    );
                    Ok(aligned_size)
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

    /// Raises an error if model has no path
    fn must_have_path(&self) -> PyResult<()> {
        if self.get_path().is_none() {
            pe!(Err(format!(
                concat!(
                    "The current instance of {} ",
                    "was not instantiated with a mmap path."
                ),
                self.get_model().get_model_name()
            )))
        } else {
            Ok(())
        }
    }

    /// Transpose computed embedding and stores to provided position.
    ///
    /// Parameters
    /// --------------
    /// path: str
    ///     Position where to store the mmapped vector.
    ///
    /// Raises
    /// --------------
    /// ValueError
    ///     If the path was not provided to the constructor.
    /// ValueError
    ///     If no embedding exists at the provided path.
    fn transpose_mmap(
        &self,
        path: String
    ) -> PyResult<()> {
        self.must_have_path()?;
        let gil = pyo3::Python::acquire_gil();

        let (embedding_dtype, is_fortran, embedding) = load_memory_mapped_numpy_array(
            gil.python(),
            self.get_path().as_ref().map(|x| x.as_str())
        );

        match pe!(Dtype::try_from(embedding_dtype))?.to_string().as_str() {
            $(
                stringify!($dtype) => {

                    let casted_embedding = embedding.cast_as::<PyArray2<$dtype>>(gil.python())?;
                    let embedding_slice = unsafe { casted_embedding.as_slice()? };

                    let number_of_nodes: usize = casted_embedding.shape()[0] as usize;
                    let embedding_size: usize = casted_embedding.shape()[1] as usize;

                    let progress_bar = if self.get_model().is_verbose() {
                        let pb = ProgressBar::new(embedding_size as u64);
                        pb.set_style(ProgressStyle::default_bar().template(concat!(
                            "Transposing {spinner:.green} ",
                            "[{elapsed_precise}] [{bar:40.cyan/blue}] ",
                            "({pos}/{len}, ETA {eta})"
                        )));
                        pb
                    } else {
                        ProgressBar::hidden()
                    };

                    let transposed_embedding = create_memory_mapped_numpy_array(
                        gil.python(),
                        Some(path.as_str()),
                        $dtype_enum,
                        &[number_of_nodes as isize, embedding_size as isize],
                        !is_fortran,
                    );

                    let casted_transposed_embedding = transposed_embedding.cast_as::<PyArray2<$dtype>>(gil.python())?;

                    if is_fortran {
                        embedding_slice.as_ref().chunks(number_of_nodes).progress_with(progress_bar).enumerate().for_each(|(j, feature)|{
                            feature.iter().copied().enumerate().for_each(|(i, feature_value)| unsafe {
                                *(casted_transposed_embedding.uget_mut([i, j])) = feature_value;
                            });
                        });
                    } else {
                        embedding_slice.as_ref().chunks(embedding_size).progress_with(progress_bar).enumerate().for_each(|(i, node_embedding)|{
                            node_embedding.iter().copied().enumerate().for_each(|(j, feature_value)| unsafe {
                                *(casted_transposed_embedding.uget_mut([i, j])) = feature_value;
                            });
                        });
                    }

                    Ok(())
                }
            )*
            dtype => pe!(Err(format!(
                concat!(
                    "The provided dtype {:?} is not supported. The supported ",
                    "data types are `u8`, `u16`, `u32` and `u64`."
                ),
                dtype
            ))),
        }
    }

    /// Return numpy embedding curresponding to the provided indices.
    ///
    /// Parameters
    /// --------------
    /// node_ids: np.ndarray
    ///     Numpy vector with node IDs to be queried.
    /// path: Optional[str] = None
    ///     Path where to read the embedding from.
    ///     If not provided, the path of the current SPINE object is used.
    ///
    /// Raises
    /// --------------
    /// ValueError
    ///     If the path was not provided to the constructor.
    /// ValueError
    ///     If no embedding exists at the provided path.
    fn get_mmap_node_embedding_from_node_ids(
        &self,
        node_ids: Py<PyArray1<NodeT>>,
        mut path: Option<String>
    ) -> PyResult<Py<PyAny>> {
        if path.is_none() {
            self.must_have_path()?;
            path = self.get_path();
        }

        let gil = pyo3::Python::acquire_gil();
        let node_ids = node_ids.as_ref(gil.python());
        let node_ids_ref = unsafe { node_ids.as_slice()? };

        let (embedding_dtype, is_fortran, embedding) = load_memory_mapped_numpy_array(
            gil.python(),
            path.as_ref().map(|x| x.as_str())
        );

        match pe!(Dtype::try_from(embedding_dtype))?.to_string().as_str() {
            $(
                stringify!($dtype) => {
                    let casted_embedding = embedding.cast_as::<PyArray2<$dtype>>(gil.python())?;
                    let number_of_nodes: usize = casted_embedding.shape()[0] as usize;
                    let embedding_size: usize = casted_embedding.shape()[1] as usize;
                    let embedding_slice = unsafe { casted_embedding.as_slice()? };
                    let result:  &PyArray2<$dtype> = unsafe{PyArray2::new(gil.python(), [node_ids_ref.len(), embedding_size], false)};
                    if is_fortran {
                        let shared_result_slice = ThreadDataRaceAware {
                            t: result,
                        };
                        embedding_slice.as_ref().par_chunks(number_of_nodes).enumerate().for_each(|(j, feature)|{
                            node_ids_ref.iter().enumerate().for_each(|(i, &node_id)| unsafe {
                                *(shared_result_slice.t.uget_mut([i, j])) = feature[node_id as usize];
                            });
                        });
                    } else {
                        let shared_result_slice = unsafe { casted_embedding.as_slice_mut()? };
                        node_ids_ref.par_iter().zip(shared_result_slice.par_chunks_mut(embedding_size)).for_each(|(&node_id, node_embedding)| {
                            embedding_slice[embedding_size * (node_id as usize)..embedding_size * (node_id as usize + 1)].iter().zip(node_embedding.iter_mut()).for_each(|(src, dst)|{
                                *dst = *src;
                            });
                        });
                    }

                    Ok(result.to_owned().into())
                }
            )*
            dtype => pe!(Err(format!(
                concat!(
                    "The provided dtype {:?} is not supported. The supported ",
                    "data types are `u8`, `u16`, `u32` and `u64`."
                ),
                dtype
            ))),
        }
    }

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
        let kwargs = normalize_kwargs!(py_kwargs, gil.python());

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

        let rows_number = graph.inner.get_number_of_nodes() as isize;
        let columns_number = pe!(self.get_model().get_embedding_size(&graph.inner))? as isize;
        match dtype {
            $(
                stringify!($dtype) => {
                    let embedding = create_memory_mapped_numpy_array(
                        gil.python(),
                        self.get_path().as_ref().map(|x| x.as_str()),
                        $dtype_enum,
                        &[rows_number, columns_number],
                        true,
                    );

                    let s = embedding.cast_as::<PyArray2<$dtype>>(gil.python())?;

                    let embedding_slice = unsafe { s.as_slice_mut()? };

                    pe!(self.get_model().fit_transform(
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

    /// Fit the provided feature number through disk MMAP.
    ///
    /// Do note that the embedding produced is in FORTRAN format.
    ///
    /// Parameters
    /// --------------
    /// graph: Graph
    ///     The graph to embed.
    /// dtype: String
    ///     Dtype of the features.
    /// feature_number: int
    ///     The number of the feature to compute.
    /// aligned_size: int
    ///     Size of the header of the Numpy object that is returned
    ///     during the initialization phase.
    fn fit_transform_feature(
        &self,
        graph: &Graph,
        dtype: String,
        feature_number: usize,
        aligned_size: usize,
    ) -> PyResult<()> {
        self.must_have_path()?;
        let gil = pyo3::Python::acquire_gil();

        let number_of_nodes = graph.inner.get_number_of_nodes() as usize;
        let columns_number = pe!(self.get_model().get_embedding_size(&graph.inner))? as isize;
        match dtype.as_str() {
            $(
                stringify!($dtype) => {

                    let slice_size = number_of_nodes * core::mem::size_of::<$dtype>();

                    let mut mmap = MemoryMapped::new_mut(
                        self.get_path(),
                        Some(slice_size), Some(aligned_size + (feature_number * number_of_nodes))
                    ).expect("Could not mmap the file");

                    let embedding_slice = pe!(mmap.get_slice_mut::<$dtype>(0, None).map_err(|e| e.to_string()))?;

                    pe!(self.get_model().fit_transform_feature(
                        &graph.inner,
                        feature_number,
                        embedding_slice,
                    ))
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
}};}

impl_alpine_embedding! {
    u8 : Dtype::U8,
    u16: Dtype::U16,
    u32: Dtype::U32,
    u64: Dtype::U64
}

impl<Model, const AFT: AnchorFeatureTypes, const AT: AnchorTypes> BaseALPINEBinding<Model, AFT, AT>
    for BasicSPINEBinding<Model, AFT, AT>
where
    Model: AnchorsInferredNodeEmbeddingModel<AT, AFT>,
{
    fn get_model(&self) -> &Model {
        &self.inner
    }

    fn get_path(&self) -> Option<String> {
        self.path.clone()
    }
}

impl<Model, const AFT: AnchorFeatureTypes, const AT: AnchorTypes> BaseALPINEBinding<Model, AFT, AT>
    for BasicWINEBinding<Model, AFT, AT>
where
    Model: AnchorsInferredNodeEmbeddingModel<AT, AFT>,
{
    fn get_model(&self) -> &Model {
        &self.inner
    }

    fn get_path(&self) -> Option<String> {
        self.path.clone()
    }
}

#[pyclass]
#[derive(Debug, Clone)]
#[pyo3(text_signature = "(*, embedding_size, maximum_depth, verbose)")]
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

    #[pyo3(text_signature = "($self, path)")]
    /// Transpose computed embedding and stores to provided position.
    ///
    /// Parameters
    /// --------------
    /// path: str
    ///     Position where to store the mmapped vector.
    ///
    /// Raises
    /// --------------
    /// ValueError
    ///     If the path was not provided to the constructor.
    /// ValueError
    ///     If no embedding exists at the provided path.
    fn transpose_mmap(&self, path: String) -> PyResult<()> {
        self.inner.transpose_mmap(path)
    }

    #[pyo3(text_signature = "($self, node_ids, path)")]
    /// Return numpy embedding curresponding to the provided indices.
    ///
    /// Parameters
    /// --------------
    /// node_ids: np.ndarray
    ///     Numpy vector with node IDs to be queried.
    /// path: Optional[str] = None
    ///     The path to be used to load the embedding.
    ///     If not provided, the path of the current SPINE model is used.
    ///
    /// Raises
    /// --------------
    /// ValueError
    ///     If the path was not provided to the constructor.
    /// ValueError
    ///     If no embedding exists at the provided path.
    fn get_mmap_node_embedding_from_node_ids(
        &self,
        node_ids: Py<PyArray1<NodeT>>,
        path: Option<String>,
    ) -> PyResult<Py<PyAny>> {
        self.inner
            .get_mmap_node_embedding_from_node_ids(node_ids, path)
    }

    #[args(py_kwargs = "**")]
    #[pyo3(text_signature = "($self, graph, *, dtype, verbose)")]
    /// Initialize numpy embedding for provided graph at path.
    ///
    /// Do note that the embedding is in FORTRAN format.
    ///
    /// Parameters
    /// --------------
    /// graph: Graph
    ///     The graph to embed.
    /// dtype: Optional[str] = None
    ///     Dtype to use for the embedding. Note that an improper dtype may cause overflows.
    ///     When not provided, we automatically infer the best one by using the diameter.
    /// verbose: bool = False
    ///     Whether to show loading bars.
    fn init_mmap(&self, graph: &Graph, py_kwargs: Option<&PyDict>) -> PyResult<usize> {
        self.inner.init_mmap(graph, py_kwargs)
    }

    #[args(py_kwargs = "**")]
    #[pyo3(text_signature = "($self, graph, *, dtype, verbose)")]
    /// Return numpy embedding with Degree SPINE node embedding.
    ///
    /// Do note that the embedding is returned transposed.
    ///
    /// Parameters
    /// --------------
    /// graph: Graph
    ///     The graph to embed.
    /// dtype: Optional[str] = None
    ///     Dtype to use for the embedding.
    ///     When not provided, we automatically infer the best one by using the diameter.
    /// verbose: bool = False
    ///     Whether to show loading bars.
    fn fit_transform(&self, graph: &Graph, py_kwargs: Option<&PyDict>) -> PyResult<Py<PyAny>> {
        self.inner.fit_transform(graph, py_kwargs)
    }

    #[args(py_kwargs = "**")]
    #[pyo3(text_signature = "($self, graph, dtype, feature_number, aligned_size)")]
    /// Fit the provided feature number through disk MMAP.
    ///
    /// Do note that the embedding produced is in FORTRAN format.
    ///
    /// Parameters
    /// --------------
    /// graph: Graph
    ///     The graph to embed.
    /// dtype: String
    ///     Dtype of the features.
    /// feature_number: int
    ///     The number of the feature to compute.
    /// aligned_size: int
    ///     Size of the header of the Numpy object that is returned
    ///     during the initialization phase.
    fn fit_transform_feature(
        &self,
        graph: &Graph,
        dtype: String,
        feature_number: usize,
        aligned_size: usize,
    ) -> PyResult<()> {
        self.inner
            .fit_transform_feature(graph, dtype, feature_number, aligned_size)
    }
}

#[pyclass]
#[derive(Debug, Clone)]
#[pyo3(text_signature = "(*, embedding_size, maximum_depth, verbose)")]
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

    #[pyo3(text_signature = "($self, path)")]
    /// Transpose computed embedding and stores to provided position.
    ///
    /// Parameters
    /// --------------
    /// path: str
    ///     Position where to store the mmapped vector.
    ///
    /// Raises
    /// --------------
    /// ValueError
    ///     If the path was not provided to the constructor.
    /// ValueError
    ///     If no embedding exists at the provided path.
    fn transpose_mmap(&self, path: String) -> PyResult<()> {
        self.inner.transpose_mmap(path)
    }

    #[pyo3(text_signature = "($self, node_ids, path)")]
    /// Return numpy embedding curresponding to the provided indices.
    ///
    /// Parameters
    /// --------------
    /// node_ids: np.ndarray
    ///     Numpy vector with node IDs to be queried.
    /// path: Optional[str] = None
    ///     The path to be used to load the embedding.
    ///     If not provided, the path of the current SPINE model is used.
    ///
    /// Raises
    /// --------------
    /// ValueError
    ///     If the path was not provided to the constructor.
    /// ValueError
    ///     If no embedding exists at the provided path.
    fn get_mmap_node_embedding_from_node_ids(
        &self,
        node_ids: Py<PyArray1<NodeT>>,
        path: Option<String>,
    ) -> PyResult<Py<PyAny>> {
        self.inner
            .get_mmap_node_embedding_from_node_ids(node_ids, path)
    }

    #[args(py_kwargs = "**")]
    #[pyo3(text_signature = "($self, graph, *, dtype, verbose)")]
    /// Initialize numpy embedding for provided graph at path.
    ///
    /// Do note that the embedding is in FORTRAN format.
    ///
    /// Parameters
    /// --------------
    /// graph: Graph
    ///     The graph to embed.
    /// dtype: Optional[str] = None
    ///     Dtype to use for the embedding. Note that an improper dtype may cause overflows.
    ///     When not provided, we automatically infer the best one by using the diameter.
    /// verbose: bool = False
    ///     Whether to show loading bars.
    fn init_mmap(&self, graph: &Graph, py_kwargs: Option<&PyDict>) -> PyResult<usize> {
        self.inner.init_mmap(graph, py_kwargs)
    }

    #[args(py_kwargs = "**")]
    #[pyo3(text_signature = "($self, graph, *, dtype, verbose)")]
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
    /// verbose: bool = False
    ///     Whether to show loading bars.
    fn fit_transform(&self, graph: &Graph, py_kwargs: Option<&PyDict>) -> PyResult<Py<PyAny>> {
        self.inner.fit_transform(graph, py_kwargs)
    }

    #[args(py_kwargs = "**")]
    #[pyo3(text_signature = "($self, graph, dtype, feature_number, aligned_size)")]
    /// Fit the provided feature number through disk MMAP.
    ///
    /// Do note that the embedding produced is in FORTRAN format.
    ///
    /// Parameters
    /// --------------
    /// graph: Graph
    ///     The graph to embed.
    /// dtype: String
    ///     Dtype of the features.
    /// feature_number: int
    ///     The number of the feature to compute.
    /// aligned_size: int
    ///     Size of the header of the Numpy object that is returned
    ///     during the initialization phase.
    fn fit_transform_feature(
        &self,
        graph: &Graph,
        dtype: String,
        feature_number: usize,
        aligned_size: usize,
    ) -> PyResult<()> {
        self.inner
            .fit_transform_feature(graph, dtype, feature_number, aligned_size)
    }
}

#[pyclass]
#[derive(Debug, Clone)]
#[pyo3(text_signature = "(*, embedding_size, maximum_depth, path, verbose)")]
pub struct ScoreSPINE {
    inner: BasicSPINE,
    path: Option<String>,
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
    /// path: Optional[str] = None
    ///     If passed, create a `.npy` file which will be mem-mapped
    ///     to allow processing embeddings that do not fit in RAM
    /// verbose: bool = True
    ///     Whether to show loading bars.
    pub fn new(py_kwargs: Option<&PyDict>) -> PyResult<ScoreSPINE> {
        Ok(Self {
            inner: BasicSPINE::from_pydict(py_kwargs)?,
            path: match py_kwargs {
                None => None,
                Some(kwargs) => {
                    extract_value_rust_result!(kwargs, "path", String)
                }
            },
        })
    }

    #[pyo3(text_signature = "($self, path)")]
    /// Transpose computed embedding and stores to provided position.
    ///
    /// Parameters
    /// --------------
    /// path: str
    ///     Position where to store the mmapped vector.
    ///
    /// Raises
    /// --------------
    /// ValueError
    ///     If the path was not provided to the constructor.
    /// ValueError
    ///     If no embedding exists at the provided path.
    fn transpose_mmap(&self, path: String) -> PyResult<()> {
        BasicSPINEBinding {
            inner: cpu_models::DegreeSPINE::from(self.inner.clone()),
            path: self.path.clone(),
        }
        .transpose_mmap(path)
    }

    #[pyo3(text_signature = "($self, node_ids, path)")]
    /// Return numpy embedding curresponding to the provided indices.
    ///
    /// Parameters
    /// --------------
    /// node_ids: np.ndarray
    ///     Numpy vector with node IDs to be queried.
    /// path: Optional[str] = None
    ///     The path to be used to load the embedding.
    ///     If not provided, the path of the current SPINE model is used.
    ///
    /// Raises
    /// --------------
    /// ValueError
    ///     If the path was not provided to the constructor.
    /// ValueError
    ///     If no embedding exists at the provided path.
    fn get_mmap_node_embedding_from_node_ids(
        &self,
        node_ids: Py<PyArray1<NodeT>>,
        path: Option<String>,
    ) -> PyResult<Py<PyAny>> {
        BasicSPINEBinding {
            inner: cpu_models::DegreeSPINE::from(self.inner.clone()),
            path: self.path.clone(),
        }
        .get_mmap_node_embedding_from_node_ids(node_ids, path)
    }

    #[args(py_kwargs = "**")]
    #[pyo3(text_signature = "($self, graph, *, dtype, verbose)")]
    /// Initialize numpy embedding for provided graph at path.
    ///
    /// Do note that the embedding is in FORTRAN format.
    ///
    /// Parameters
    /// --------------
    /// graph: Graph
    ///     The graph to embed.
    /// dtype: Optional[str] = None
    ///     Dtype to use for the embedding. Note that an improper dtype may cause overflows.
    ///     When not provided, we automatically infer the best one by using the diameter.
    /// verbose: bool = False
    ///     Whether to show loading bars.
    fn init_mmap(&self, graph: &Graph, py_kwargs: Option<&PyDict>) -> PyResult<usize> {
        BasicSPINEBinding {
            inner: cpu_models::DegreeSPINE::from(self.inner.clone()),
            path: self.path.clone(),
        }
        .init_mmap(graph, py_kwargs)
    }

    #[args(py_kwargs = "**")]
    #[pyo3(text_signature = "($self, scores, graph, *, dtype, verbose)")]
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
    /// verbose: bool = False
    ///     Whether to show loading bars.
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
            path: self.path.clone(),
        }
        .fit_transform(graph, py_kwargs)
    }

    #[args(py_kwargs = "**")]
    #[pyo3(text_signature = "($self, scores, graph, dtype, feature_number, aligned_size)")]
    /// Fit the provided feature number through disk MMAP.
    ///
    /// Do note that the embedding produced is in FORTRAN format.
    ///
    /// Parameters
    /// --------------
    /// scores: np.ndarray
    ///     Scores to create the node groups.
    /// graph: Graph
    ///     The graph to embed.
    /// dtype: String
    ///     Dtype of the features.
    /// feature_number: int
    ///     The number of the feature to compute.
    /// aligned_size: int
    ///     Size of the header of the Numpy object that is returned
    ///     during the initialization phase.
    fn fit_transform_feature(
        &self,
        scores: Py<PyArray1<f32>>,
        graph: &Graph,
        dtype: String,
        feature_number: usize,
        aligned_size: usize,
    ) -> PyResult<()> {
        let gil = pyo3::Python::acquire_gil();
        let scores_ref = scores.as_ref(gil.python());
        BasicSPINEBinding {
            inner: cpu_models::ScoreSPINE::new(self.inner.clone(), unsafe {
                scores_ref.as_slice().unwrap()
            }),
            path: self.path.clone(),
        }
        .fit_transform_feature(graph, dtype, feature_number, aligned_size)
    }
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

    #[pyo3(text_signature = "($self, path)")]
    /// Transpose computed embedding and stores to provided position.
    ///
    /// Parameters
    /// --------------
    /// path: str
    ///     Position where to store the mmapped vector.
    ///
    /// Raises
    /// --------------
    /// ValueError
    ///     If the path was not provided to the constructor.
    /// ValueError
    ///     If no embedding exists at the provided path.
    fn transpose_mmap(&self, path: String) -> PyResult<()> {
        self.inner.transpose_mmap(path)
    }

    #[pyo3(text_signature = "($self, node_ids, path)")]
    /// Return numpy embedding curresponding to the provided indices.
    ///
    /// Parameters
    /// --------------
    /// node_ids: np.ndarray
    ///     Numpy vector with node IDs to be queried.
    /// path: Optional[str] = None
    ///     The path to be used to load the embedding.
    ///     If not provided, the path of the current SPINE model is used.
    ///
    /// Raises
    /// --------------
    /// ValueError
    ///     If the path was not provided to the constructor.
    /// ValueError
    ///     If no embedding exists at the provided path.
    fn get_mmap_node_embedding_from_node_ids(
        &self,
        node_ids: Py<PyArray1<NodeT>>,
        path: Option<String>,
    ) -> PyResult<Py<PyAny>> {
        self.inner
            .get_mmap_node_embedding_from_node_ids(node_ids, path)
    }

    #[args(py_kwargs = "**")]
    #[pyo3(text_signature = "($self, graph, *, dtype, verbose)")]
    /// Initialize numpy embedding for provided graph at path.
    ///
    /// Do note that the embedding is in FORTRAN format.
    ///
    /// Parameters
    /// --------------
    /// graph: Graph
    ///     The graph to embed.
    /// dtype: Optional[str] = None
    ///     Dtype to use for the embedding. Note that an improper dtype may cause overflows.
    ///     When not provided, we automatically infer the best one by using the diameter.
    /// verbose: bool = False
    ///     Whether to show loading bars.
    fn init_mmap(&self, graph: &Graph, py_kwargs: Option<&PyDict>) -> PyResult<usize> {
        self.inner.init_mmap(graph, py_kwargs)
    }

    #[args(py_kwargs = "**")]
    #[pyo3(text_signature = "($self, graph, *, dtype, verbose)")]
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
    /// verbose: bool = False
    ///     Whether to show loading bars.
    fn fit_transform(&self, graph: &Graph, py_kwargs: Option<&PyDict>) -> PyResult<Py<PyAny>> {
        self.inner.fit_transform(graph, py_kwargs)
    }

    #[args(py_kwargs = "**")]
    #[pyo3(text_signature = "($self, graph, dtype, feature_number, aligned_size)")]
    /// Fit the provided feature number through disk MMAP.
    ///
    /// Do note that the embedding produced is in FORTRAN format.
    ///
    /// Parameters
    /// --------------
    /// graph: Graph
    ///     The graph to embed.
    /// dtype: String
    ///     Dtype of the features.
    /// feature_number: int
    ///     The number of the feature to compute.
    /// aligned_size: int
    ///     Size of the header of the Numpy object that is returned
    ///     during the initialization phase.
    fn fit_transform_feature(
        &self,
        graph: &Graph,
        dtype: String,
        feature_number: usize,
        aligned_size: usize,
    ) -> PyResult<()> {
        self.inner
            .fit_transform_feature(graph, dtype, feature_number, aligned_size)
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

    #[pyo3(text_signature = "($self, path)")]
    /// Transpose computed embedding and stores to provided position.
    ///
    /// Parameters
    /// --------------
    /// path: str
    ///     Position where to store the mmapped vector.
    ///
    /// Raises
    /// --------------
    /// ValueError
    ///     If the path was not provided to the constructor.
    /// ValueError
    ///     If no embedding exists at the provided path.
    fn transpose_mmap(&self, path: String) -> PyResult<()> {
        self.inner.transpose_mmap(path)
    }

    #[pyo3(text_signature = "($self, node_ids, path)")]
    /// Return numpy embedding curresponding to the provided indices.
    ///
    /// Parameters
    /// --------------
    /// node_ids: np.ndarray
    ///     Numpy vector with node IDs to be queried.
    /// path: Optional[str] = None
    ///     The path to be used to load the embedding.
    ///     If not provided, the path of the current SPINE model is used.
    ///
    /// Raises
    /// --------------
    /// ValueError
    ///     If the path was not provided to the constructor.
    /// ValueError
    ///     If no embedding exists at the provided path.
    fn get_mmap_node_embedding_from_node_ids(
        &self,
        node_ids: Py<PyArray1<NodeT>>,
        path: Option<String>,
    ) -> PyResult<Py<PyAny>> {
        self.inner
            .get_mmap_node_embedding_from_node_ids(node_ids, path)
    }

    #[args(py_kwargs = "**")]
    #[pyo3(text_signature = "($self, graph, *, dtype, verbose)")]
    /// Initialize numpy embedding for provided graph at path.
    ///
    /// Do note that the embedding is in FORTRAN format.
    ///
    /// Parameters
    /// --------------
    /// graph: Graph
    ///     The graph to embed.
    /// dtype: Optional[str] = None
    ///     Dtype to use for the embedding. Note that an improper dtype may cause overflows.
    ///     When not provided, we automatically infer the best one by using the diameter.
    /// verbose: bool = False
    ///     Whether to show loading bars.
    fn init_mmap(&self, graph: &Graph, py_kwargs: Option<&PyDict>) -> PyResult<usize> {
        self.inner.init_mmap(graph, py_kwargs)
    }

    #[args(py_kwargs = "**")]
    #[pyo3(text_signature = "($self, graph, *, dtype, verbose)")]
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
    /// verbose: bool = False
    ///     Whether to show loading bars.
    fn fit_transform(&self, graph: &Graph, py_kwargs: Option<&PyDict>) -> PyResult<Py<PyAny>> {
        self.inner.fit_transform(graph, py_kwargs)
    }

    #[args(py_kwargs = "**")]
    #[pyo3(text_signature = "($self, graph, dtype, feature_number, aligned_size)")]
    /// Fit the provided feature number through disk MMAP.
    ///
    /// Do note that the embedding produced is in FORTRAN format.
    ///
    /// Parameters
    /// --------------
    /// graph: Graph
    ///     The graph to embed.
    /// dtype: String
    ///     Dtype of the features.
    /// feature_number: int
    ///     The number of the feature to compute.
    /// aligned_size: int
    ///     Size of the header of the Numpy object that is returned
    ///     during the initialization phase.
    fn fit_transform_feature(
        &self,
        graph: &Graph,
        dtype: String,
        feature_number: usize,
        aligned_size: usize,
    ) -> PyResult<()> {
        self.inner
            .fit_transform_feature(graph, dtype, feature_number, aligned_size)
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

    #[pyo3(text_signature = "($self, path)")]
    /// Transpose computed embedding and stores to provided position.
    ///
    /// Parameters
    /// --------------
    /// path: str
    ///     Position where to store the mmapped vector.
    ///
    /// Raises
    /// --------------
    /// ValueError
    ///     If the path was not provided to the constructor.
    /// ValueError
    ///     If no embedding exists at the provided path.
    fn transpose_mmap(&self, path: String) -> PyResult<()> {
        BasicWINEBinding {
            inner: cpu_models::DegreeWINE::from(self.inner.clone()),
            path: self.path.clone(),
        }
        .transpose_mmap(path)
    }

    #[pyo3(text_signature = "($self, node_ids, path)")]
    /// Return numpy embedding curresponding to the provided indices.
    ///
    /// Parameters
    /// --------------
    /// node_ids: np.ndarray
    ///     Numpy vector with node IDs to be queried.
    /// path: Optional[str] = None
    ///     The path to be used to load the embedding.
    ///     If not provided, the path of the current WINE model is used.
    ///
    /// Raises
    /// --------------
    /// ValueError
    ///     If the path was not provided to the constructor.
    /// ValueError
    ///     If no embedding exists at the provided path.
    fn get_mmap_node_embedding_from_node_ids(
        &self,
        node_ids: Py<PyArray1<NodeT>>,
        path: Option<String>,
    ) -> PyResult<Py<PyAny>> {
        BasicWINEBinding {
            inner: cpu_models::DegreeWINE::from(self.inner.clone()),
            path: self.path.clone(),
        }
        .get_mmap_node_embedding_from_node_ids(node_ids, path)
    }

    #[args(py_kwargs = "**")]
    #[pyo3(text_signature = "($self, graph, *, dtype, verbose)")]
    /// Initialize numpy embedding for provided graph at path.
    ///
    /// Do note that the embedding is in FORTRAN format.
    ///
    /// Parameters
    /// --------------
    /// graph: Graph
    ///     The graph to embed.
    /// dtype: Optional[str] = None
    ///     Dtype to use for the embedding. Note that an improper dtype may cause overflows.
    ///     When not provided, we automatically infer the best one by using the diameter.
    /// verbose: bool = False
    ///     Whether to show loading bars.
    fn init_mmap(&self, graph: &Graph, py_kwargs: Option<&PyDict>) -> PyResult<usize> {
        BasicWINEBinding {
            inner: cpu_models::DegreeWINE::from(self.inner.clone()),
            path: self.path.clone(),
        }
        .init_mmap(graph, py_kwargs)
    }

    #[args(py_kwargs = "**")]
    #[pyo3(text_signature = "($self, scores, graph, *, dtype, verbose)")]
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
    /// verbose: bool = False
    ///     Whether to show loading bars.
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

    #[args(py_kwargs = "**")]
    #[pyo3(text_signature = "($self, scores, graph, dtype, feature_number, aligned_size)")]
    /// Fit the provided feature number through disk MMAP.
    ///
    /// Do note that the embedding produced is in FORTRAN format.
    ///
    /// Parameters
    /// --------------
    /// scores: np.ndarray
    ///     Scores to create the node groups.
    /// graph: Graph
    ///     The graph to embed.
    /// dtype: String
    ///     Dtype of the features.
    /// feature_number: int
    ///     The number of the feature to compute.
    /// aligned_size: int
    ///     Size of the header of the Numpy object that is returned
    ///     during the initialization phase.
    fn fit_transform_feature(
        &self,
        scores: Py<PyArray1<f32>>,
        graph: &Graph,
        dtype: String,
        feature_number: usize,
        aligned_size: usize,
    ) -> PyResult<()> {
        let gil = pyo3::Python::acquire_gil();
        let scores_ref = scores.as_ref(gil.python());
        BasicWINEBinding {
            inner: cpu_models::ScoreWINE::new(self.inner.clone(), unsafe {
                scores_ref.as_slice().unwrap()
            }),
            path: self.path.clone(),
        }
        .fit_transform_feature(graph, dtype, feature_number, aligned_size)
    }
}
