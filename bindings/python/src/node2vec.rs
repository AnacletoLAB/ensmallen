use super::*;
use cpu_models::{IdentifyWalkTransformer, Node2Vec, Node2VecModels, WalkTransformer, Walklets};

#[derive(Debug, Clone)]
pub(crate) struct Node2VecBinding<W>
where
    W: WalkTransformer,
{
    central_nodes_embedding_path: Option<String>,
    contextual_nodes_embedding_path: Option<String>,
    node2vec: Node2Vec<W>,
}

impl<W> GraphEmbedderBinding<Node2Vec<W>> for Node2VecBinding<W>
where
    W: WalkTransformer,
{
    fn get_model(&self) -> &Node2Vec<W> {
        &self.node2vec
    }

    fn get_paths(&self) -> Vec<Option<String>> {
        vec![
            self.central_nodes_embedding_path.clone(),
            self.contextual_nodes_embedding_path.clone(),
        ]
    }
}

impl<W> Node2VecBinding<W>
where
    W: WalkTransformer,
{
    pub(crate) fn new(model_type: Node2VecModels, py_kwargs: Option<&PyDict>) -> PyResult<Self> {
        let py = pyo3::Python::acquire_gil();
        let kwargs = normalize_kwargs!(py_kwargs, py.python());

        pe!(validate_kwargs(
            kwargs,
            build_walk_parameters_list(&[
                "embedding_size",
                "window_size",
                "clipping_value",
                "number_of_negative_samples",
                "epochs",
                "learning_rate",
                "learning_rate_decay",
                "alpha",
                "maximum_cooccurrence_count_threshold",
                "central_nodes_embedding_path",
                "contextual_nodes_embedding_path",
                "stochastic_downsample_by_degree",
                "normalize_learning_rate_by_degree",
                "use_scale_free_distribution",
                "dtype",
                "verbose"
            ])
            .as_slice()
        ))?;

        let parameters = pe!(build_walk_parameters(kwargs))?;

        Ok(Self {
            central_nodes_embedding_path: extract_value_rust_result!(
                kwargs,
                "central_nodes_embedding_path",
                String
            ),
            contextual_nodes_embedding_path: extract_value_rust_result!(
                kwargs,
                "contextual_nodes_embedding_path",
                String
            ),
            node2vec: pe!(Node2Vec::new(
                model_type,
                W::default(),
                extract_value_rust_result!(kwargs, "embedding_size", usize),
                Some(parameters),
                extract_value_rust_result!(kwargs, "window_size", usize),
                extract_value_rust_result!(kwargs, "clipping_value", f32),
                extract_value_rust_result!(kwargs, "number_of_negative_samples", usize),
                extract_value_rust_result!(kwargs, "epochs", usize),
                extract_value_rust_result!(kwargs, "learning_rate", f32),
                extract_value_rust_result!(kwargs, "learning_rate_decay", f32),
                extract_value_rust_result!(kwargs, "alpha", f32),
                extract_value_rust_result!(kwargs, "maximum_cooccurrence_count_threshold", NodeT),
                extract_value_rust_result!(kwargs, "stochastic_downsample_by_degree", bool),
                extract_value_rust_result!(kwargs, "normalize_learning_rate_by_degree", bool),
                extract_value_rust_result!(kwargs, "use_scale_free_distribution", bool),
                extract_value_rust_result!(kwargs, "dtype", String),
                extract_value_rust_result!(kwargs, "verbose", bool),
            ))?,
        })
    }
}

#[derive(Debug, Clone)]
pub(crate) struct WalkletsBinding {
    central_nodes_embedding_path: Option<String>,
    contextual_nodes_embedding_path: Option<String>,
    walklets: Walklets,
}

impl GraphEmbedderBinding<Walklets> for WalkletsBinding {
    fn get_model(&self) -> &Walklets {
        &self.walklets
    }

    fn get_paths(&self) -> Vec<Option<String>> {
        (0..self.walklets.get_window_size())
            .flat_map(|window_size| {
                vec![
                    self.central_nodes_embedding_path
                        .clone()
                        .map(|path| path.replace("{window_size}", &window_size.to_string())),
                    self.contextual_nodes_embedding_path
                        .clone()
                        .map(|path| path.replace("{window_size}", &window_size.to_string())),
                ]
            })
            .collect()
    }
}

impl WalkletsBinding {
    pub(crate) fn new(model_type: Node2VecModels, py_kwargs: Option<&PyDict>) -> PyResult<Self> {
        let py = pyo3::Python::acquire_gil();
        let kwargs = normalize_kwargs!(py_kwargs, py.python());
        let central_nodes_embedding_path =
            extract_value_rust_result!(kwargs, "central_nodes_embedding_path", String);
        let contextual_nodes_embedding_path =
            extract_value_rust_result!(kwargs, "contextual_nodes_embedding_path", String);
        if let Some(central_nodes_embedding_path) = central_nodes_embedding_path.as_ref() {
            if !central_nodes_embedding_path.contains("{window_size}") {
                return pe!(Err(format!(
                    concat!(
                        "The provided central_nodes_embedding_path parameter `{}` ",
                        "does not contain the expected `{{window_size}}` placeholder."
                    ),
                    central_nodes_embedding_path
                )));
            }
        }
        if let Some(contextual_nodes_embedding_path) = contextual_nodes_embedding_path.as_ref() {
            if !contextual_nodes_embedding_path.contains("{window_size}") {
                return pe!(Err(format!(
                    concat!(
                        "The provided contextual_nodes_embedding_path parameter `{}` ",
                        "does not contain the expected `{{window_size}}` placeholder."
                    ),
                    contextual_nodes_embedding_path
                )));
            }
        }

        Ok(Self {
            central_nodes_embedding_path,
            contextual_nodes_embedding_path,
            walklets: Walklets::new(Node2VecBinding::new(model_type, py_kwargs)?.node2vec),
        })
    }
}

#[pyclass]
#[derive(Debug, Clone)]
#[pyo3(
    text_signature = "(*, embedding_size, window_size, number_of_negative_samples, walk_length, return_weight, explore_weight, change_edge_type_weight, change_node_type_weight, random_state, iterations, max_neighbours, normalize_by_degree, epochs, learning_rate, learning_rate_decay, central_nodes_embedding_path, contextual_nodes_embedding_path, stochastic_downsample_by_degree, normalize_learning_rate_by_degree, use_scale_free_distribution, clipping_value, dtype, verbose)"
)]
pub struct CBOW {
    inner: Node2VecBinding<IdentifyWalkTransformer>,
}

#[pymethods]
impl CBOW {
    #[new]
    #[args(py_kwargs = "**")]
    /// Return a new instance of the CBOW model.
    ///
    /// Parameters
    /// ------------------------
    /// embedding_size: Optional[int] = 100
    ///     Size of the embedding.
    /// window_size: Optional[int] = 10
    ///     Window size defining the contexts.
    /// number_of_negative_samples: Optional[int] = 5
    ///     Number of negative samples to extract for each context.
    /// walk_length: Optional[int] = 32
    ///     Maximal length of the random walk.
    ///     On graphs without traps, all walks have this length.
    /// return_weight: float = 1.0
    ///     Weight on the probability of returning to node coming from
    ///     Having this higher tends the walks to be
    ///     more like a Breadth-First Search.
    ///     Having this very high  (> 2) makes search very local.
    ///     Equal to the inverse of p in the Node2Vec paper.
    /// explore_weight: float = 1.0
    ///     Weight on the probability of visiting a neighbor node
    ///     to the one we're coming from in the random walk
    ///     Having this higher tends the walks to be
    ///     more like a Depth-First Search.
    ///     Having this very high makes search more outward.
    ///     Having this very low makes search very local.
    ///     Equal to the inverse of q in the Node2Vec paper.
    /// change_edge_type_weight: float = 1.0
    ///     Weight on the probability of visiting a neighbor node of a
    ///     different type than the previous node. This only applies to
    ///     colored graphs, otherwise it has no impact.
    /// change_node_type_weight: float = 1.0
    ///     Weight on the probability of visiting a neighbor edge of a
    ///     different type than the previous edge. This only applies to
    ///     multigraphs, otherwise it has no impact.
    /// random_state: int = 42
    ///     random_state to use to reproduce the walks.
    /// iterations: int = 1
    ///     Number of cycles on the graphs to execute.
    /// max_neighbours: Optional[int] = 100
    ///     Maximum number of randomly sampled neighbours to consider.
    ///     If this parameter is used, the walks becomes probabilistic in nature
    ///     and becomes an approximation of an exact walk.
    /// normalize_by_degree: Optional[bool] = False
    ///     Whether to normalize the random walks by the node degree.
    /// epochs: int = 100
    ///     Number of epochs to train the model for.
    /// learning_rate: float = 0.01
    ///     Learning rate of the model.
    /// learning_rate_decay: float = 0.9
    ///     Amount of learning rate decay for each epoch.
    /// central_nodes_embedding_path: Optional[str] = None
    ///     Path where to mmap and store the central nodes embedding.
    ///     This is necessary to embed large graphs whose embedding will not
    ///     fit into the available main memory.
    /// contextual_nodes_embedding_path: Optional[str] = None
    ///     Path where to mmap and store the central nodes embedding.
    ///     This is necessary to embed large graphs whose embedding will not
    ///     fit into the available main memory.
    /// stochastic_downsample_by_degree: Optional[bool]
    ///     Randomly skip samples with probability proportional to the degree of the central node. By default false.
    /// normalize_learning_rate_by_degree: Optional[bool]
    ///     Divide the learning rate by the degree of the central node. By default false.
    /// use_scale_free_distribution: Optional[bool]
    ///     Sample negatives proportionally to their degree. By default true.
    /// clipping_value: Optional[float] = 6.0
    ///     Value at which we clip the dot product, mostly for numerical stability issues.
    ///     By default, `6.0`, where the loss is already close to zero.
    /// dtype: str
    ///     The data type to be employed, by default f32.
    /// verbose: bool = True
    ///     Whether to show the loading bar.
    pub fn new(py_kwargs: Option<&PyDict>) -> PyResult<CBOW> {
        Ok(Self {
            inner: Node2VecBinding::new(Node2VecModels::CBOW, py_kwargs)?,
        })
    }
}

#[pymethods]
impl CBOW {
    #[args(py_kwargs = "**")]
    #[pyo3(text_signature = "($self, graph)")]
    /// Return numpy embedding with CBOW node embedding.
    ///
    /// Parameters
    /// ---------
    /// graph: Graph
    ///     The graph to embed.
    fn fit_transform(&self, graph: &Graph) -> PyResult<Vec<Py<PyAny>>> {
        self.inner.fit_transform(graph)
    }
}

#[pyclass]
#[derive(Debug, Clone)]
#[pyo3(
    text_signature = "(*, embedding_size, window_size, number_of_negative_samples, walk_length, return_weight, explore_weight, change_edge_type_weight, change_node_type_weight, random_state, iterations, max_neighbours, normalize_by_degree, epochs, learning_rate, learning_rate_decay, alpha, maximum_cooccurrence_count_threshold, central_nodes_embedding_path, contextual_nodes_embedding_path, stochastic_downsample_by_degree, normalize_learning_rate_by_degree, use_scale_free_distribution, clipping_value, dtype, verbose)"
)]
pub struct GloVe {
    inner: Node2VecBinding<IdentifyWalkTransformer>,
}

#[pymethods]
impl GloVe {
    #[new]
    #[args(py_kwargs = "**")]
    /// Return a new instance of the GloVe model.
    ///
    /// Parameters
    /// ------------------------
    /// embedding_size: Optional[int] = 100
    ///     Size of the embedding.
    /// window_size: Optional[int] = 10
    ///     Window size defining the contexts.
    /// number_of_negative_samples: Optional[int] = 5
    ///     Number of negative samples to extract for each context.
    /// walk_length: Optional[int] = 32
    ///     Maximal length of the random walk.
    ///     On graphs without traps, all walks have this length.
    /// return_weight: float = 1.0
    ///     Weight on the probability of returning to node coming from
    ///     Having this higher tends the walks to be
    ///     more like a Breadth-First Search.
    ///     Having this very high  (> 2) makes search very local.
    ///     Equal to the inverse of p in the Node2Vec paper.
    /// explore_weight: float = 1.0
    ///     Weight on the probability of visiting a neighbor node
    ///     to the one we're coming from in the random walk
    ///     Having this higher tends the walks to be
    ///     more like a Depth-First Search.
    ///     Having this very high makes search more outward.
    ///     Having this very low makes search very local.
    ///     Equal to the inverse of q in the Node2Vec paper.
    /// change_edge_type_weight: float = 1.0
    ///     Weight on the probability of visiting a neighbor node of a
    ///     different type than the previous node. This only applies to
    ///     colored graphs, otherwise it has no impact.
    /// change_node_type_weight: float = 1.0
    ///     Weight on the probability of visiting a neighbor edge of a
    ///     different type than the previous edge. This only applies to
    ///     multigraphs, otherwise it has no impact.
    /// random_state: int = 42
    ///     random_state to use to reproduce the walks.
    /// iterations: int = 1
    ///     Number of cycles on the graphs to execute.
    /// max_neighbours: Optional[int] = 100
    ///     Maximum number of randomly sampled neighbours to consider.
    ///     If this parameter is used, the walks becomes probabilistic in nature
    ///     and becomes an approximation of an exact walk.
    /// normalize_by_degree: Optional[bool] = False
    ///     Whether to normalize the random walks by the node degree.
    /// epochs: int = 100
    ///     Number of epochs to train the model for.
    /// learning_rate: float = 0.01
    ///     Learning rate of the model.
    /// learning_rate_decay: float = 0.9
    ///     Amount of learning rate decay for each epoch.
    /// alpha: float = 0.75
    ///     Exponent of the adaptative learning rate.
    /// maximum_cooccurrence_count_threshold: int = 100
    ///     Maximum cooccurrence threshold for the adaptative learning rate.
    /// central_nodes_embedding_path: Optional[str] = None
    ///     Path where to mmap and store the central nodes embedding.
    ///     This is necessary to embed large graphs whose embedding will not
    ///     fit into the available main memory.
    /// contextual_nodes_embedding_path: Optional[str] = None
    ///     Path where to mmap and store the central nodes embedding.
    ///     This is necessary to embed large graphs whose embedding will not
    ///     fit into the available main memory.
    /// stochastic_downsample_by_degree: Optional[bool]
    ///     Randomly skip samples with probability proportional to the degree of the central node. By default false.
    /// normalize_learning_rate_by_degree: Optional[bool]
    ///     Divide the learning rate by the degree of the central node. By default false.
    /// use_scale_free_distribution: Optional[bool]
    ///     Sample negatives proportionally to their degree. By default true.
    /// clipping_value: Optional[float] = 6.0
    ///     Value at which we clip the dot product, mostly for numerical stability issues.
    ///     By default, `6.0`, where the loss is already close to zero.
    /// dtype: str
    ///     The data type to be employed, by default f32.
    /// verbose: bool = True
    ///     Whether to show the loading bar.
    pub fn new(py_kwargs: Option<&PyDict>) -> PyResult<GloVe> {
        Ok(Self {
            inner: Node2VecBinding::new(Node2VecModels::GloVe, py_kwargs)?,
        })
    }
}

#[pymethods]
impl GloVe {
    #[args(py_kwargs = "**")]
    #[pyo3(text_signature = "($self, graph)")]
    /// Return numpy embedding with GloVe node embedding.
    ///
    /// Parameters
    /// ---------
    /// graph: Graph
    ///     The graph to embed.
    fn fit_transform(&self, graph: &Graph) -> PyResult<Vec<Py<PyAny>>> {
        self.inner.fit_transform(graph)
    }
}

#[pyclass]
#[derive(Debug, Clone)]
#[pyo3(
    text_signature = "(*, embedding_size, window_size, number_of_negative_samples, walk_length, return_weight, explore_weight, change_edge_type_weight, change_node_type_weight, random_state, iterations, max_neighbours, normalize_by_degree, epochs, learning_rate, learning_rate_decay, central_nodes_embedding_path, contextual_nodes_embedding_path, stochastic_downsample_by_degree, normalize_learning_rate_by_degree, use_scale_free_distribution, clipping_value, dtype, verbose)"
)]
pub struct SkipGram {
    inner: Node2VecBinding<IdentifyWalkTransformer>,
}

#[pymethods]
impl SkipGram {
    #[new]
    #[args(py_kwargs = "**")]
    /// Return a new instance of the SkipGram model.
    ///
    /// Parameters
    /// ------------------------
    /// embedding_size: Optional[int] = 100
    ///     Size of the embedding.
    /// window_size: Optional[int] = 10
    ///     Window size defining the contexts.
    /// number_of_negative_samples: Optional[int] = 5
    ///     Number of negative samples to extract for each context.
    /// walk_length: Optional[int] = 32
    ///     Maximal length of the random walk.
    ///     On graphs without traps, all walks have this length.
    /// return_weight: float = 1.0
    ///     Weight on the probability of returning to node coming from
    ///     Having this higher tends the walks to be
    ///     more like a Breadth-First Search.
    ///     Having this very high  (> 2) makes search very local.
    ///     Equal to the inverse of p in the Node2Vec paper.
    /// explore_weight: float = 1.0
    ///     Weight on the probability of visiting a neighbor node
    ///     to the one we're coming from in the random walk
    ///     Having this higher tends the walks to be
    ///     more like a Depth-First Search.
    ///     Having this very high makes search more outward.
    ///     Having this very low makes search very local.
    ///     Equal to the inverse of q in the Node2Vec paper.
    /// change_edge_type_weight: float = 1.0
    ///     Weight on the probability of visiting a neighbor node of a
    ///     different type than the previous node. This only applies to
    ///     colored graphs, otherwise it has no impact.
    /// change_node_type_weight: float = 1.0
    ///     Weight on the probability of visiting a neighbor edge of a
    ///     different type than the previous edge. This only applies to
    ///     multigraphs, otherwise it has no impact.
    /// random_state: int = 42
    ///     random_state to use to reproduce the walks.
    /// iterations: int = 1
    ///     Number of cycles on the graphs to execute.
    /// max_neighbours: Optional[int] = 100
    ///     Maximum number of randomly sampled neighbours to consider.
    ///     If this parameter is used, the walks becomes probabilistic in nature
    ///     and becomes an approximation of an exact walk.
    /// normalize_by_degree: Optional[bool] = False
    ///     Whether to normalize the random walks by the node degree.
    /// epochs: int = 100
    ///     Number of epochs to train the model for.
    /// learning_rate: float = 0.01
    ///     Learning rate of the model.
    /// learning_rate_decay: float = 0.9
    ///     Amount of learning rate decay for each epoch.
    /// central_nodes_embedding_path: Optional[str] = None
    ///     Path where to mmap and store the central nodes embedding.
    ///     This is necessary to embed large graphs whose embedding will not
    ///     fit into the available main memory.
    /// contextual_nodes_embedding_path: Optional[str] = None
    ///     Path where to mmap and store the central nodes embedding.
    ///     This is necessary to embed large graphs whose embedding will not
    ///     fit into the available main memory.
    /// stochastic_downsample_by_degree: Optional[bool]
    ///     Randomly skip samples with probability proportional to the degree of the central node. By default false.
    /// normalize_learning_rate_by_degree: Optional[bool]
    ///     Divide the learning rate by the degree of the central node. By default false.
    /// use_scale_free_distribution: Optional[bool]
    ///     Sample negatives proportionally to their degree. By default true.
    /// clipping_value: Optional[float] = 6.0
    ///     Value at which we clip the dot product, mostly for numerical stability issues.
    ///     By default, `6.0`, where the loss is already close to zero.
    /// dtype: str
    ///     The data type to be employed, by default f32.
    /// verbose: bool = True
    ///     Whether to show the loading bar.
    pub fn new(py_kwargs: Option<&PyDict>) -> PyResult<SkipGram> {
        Ok(Self {
            inner: Node2VecBinding::new(Node2VecModels::SkipGram, py_kwargs)?,
        })
    }
}

#[pymethods]
impl SkipGram {
    #[args(py_kwargs = "**")]
    #[pyo3(text_signature = "($self, graph)")]
    /// Return numpy embedding with SkipGram node embedding.
    ///
    /// Parameters
    /// ---------
    /// graph: Graph
    ///     The graph to embed.
    fn fit_transform(&self, graph: &Graph) -> PyResult<Vec<Py<PyAny>>> {
        self.inner.fit_transform(graph)
    }
}

#[pyclass]
#[derive(Debug, Clone)]
#[pyo3(
    text_signature = "(*, embedding_size, window_size, number_of_negative_samples, walk_length, return_weight, explore_weight, change_edge_type_weight, change_node_type_weight, random_state, iterations, max_neighbours, normalize_by_degree, epochs, learning_rate, learning_rate_decay, central_nodes_embedding_path, contextual_nodes_embedding_path, stochastic_downsample_by_degree, normalize_learning_rate_by_degree, use_scale_free_distribution, clipping_value, dtype, verbose)"
)]
pub struct WalkletsCBOW {
    inner: WalkletsBinding,
}

#[pymethods]
impl WalkletsCBOW {
    #[new]
    #[args(py_kwargs = "**")]
    /// Return a new instance of the Walklets CBOW model.
    ///
    /// Parameters
    /// ------------------------
    /// embedding_size: Optional[int] = 100
    ///     Size of the embedding.
    /// window_size: Optional[int] = 10
    ///     Window size defining the contexts.
    /// number_of_negative_samples: Optional[int] = 5
    ///     Number of negative samples to extract for each context.
    /// walk_length: Optional[int] = 32
    ///     Maximal length of the random walk.
    ///     On graphs without traps, all walks have this length.
    /// return_weight: float = 1.0
    ///     Weight on the probability of returning to node coming from
    ///     Having this higher tends the walks to be
    ///     more like a Breadth-First Search.
    ///     Having this very high  (> 2) makes search very local.
    ///     Equal to the inverse of p in the Node2Vec paper.
    /// explore_weight: float = 1.0
    ///     Weight on the probability of visiting a neighbor node
    ///     to the one we're coming from in the random walk
    ///     Having this higher tends the walks to be
    ///     more like a Depth-First Search.
    ///     Having this very high makes search more outward.
    ///     Having this very low makes search very local.
    ///     Equal to the inverse of q in the Node2Vec paper.
    /// change_edge_type_weight: float = 1.0
    ///     Weight on the probability of visiting a neighbor node of a
    ///     different type than the previous node. This only applies to
    ///     colored graphs, otherwise it has no impact.
    /// change_node_type_weight: float = 1.0
    ///     Weight on the probability of visiting a neighbor edge of a
    ///     different type than the previous edge. This only applies to
    ///     multigraphs, otherwise it has no impact.
    /// random_state: int = 42
    ///     random_state to use to reproduce the walks.
    /// iterations: int = 1
    ///     Number of cycles on the graphs to execute.
    /// max_neighbours: Optional[int] = 100
    ///     Maximum number of randomly sampled neighbours to consider.
    ///     If this parameter is used, the walks becomes probabilistic in nature
    ///     and becomes an approximation of an exact walk.
    /// normalize_by_degree: Optional[bool] = False
    ///     Whether to normalize the random walks by the node degree.
    /// epochs: int = 100
    ///     Number of epochs to train the model for.
    /// learning_rate: float = 0.01
    ///     Learning rate of the model.
    /// learning_rate_decay: float = 0.9
    ///     Amount of learning rate decay for each epoch.
    /// central_nodes_embedding_path: Optional[str] = None
    ///     Path where to mmap and store the central nodes embedding.
    ///     If provided, we expect the path to contain the substring `{window_size}` which
    ///     will be replaced with the i-th window size embedding that is being computed.
    ///     This is necessary to embed large graphs whose embedding will not
    ///     fit into the available main memory.
    /// contextual_nodes_embedding_path: Optional[str] = None
    ///     Path where to mmap and store the central nodes embedding.
    ///     If provided, we expect the path to contain the substring `{window_size}` which
    ///     will be replaced with the i-th window size embedding that is being computed.
    ///     This is necessary to embed large graphs whose embedding will not
    ///     fit into the available main memory.
    /// stochastic_downsample_by_degree: Optional[bool]
    ///     Randomly skip samples with probability proportional to the degree of the central node. By default false.
    /// normalize_learning_rate_by_degree: Optional[bool]
    ///     Divide the learning rate by the degree of the central node. By default false.
    /// use_scale_free_distribution: Optional[bool]
    ///     Sample negatives proportionally to their degree. By default true.
    /// clipping_value: Optional[float] = 6.0
    ///     Value at which we clip the dot product, mostly for numerical stability issues.
    ///     By default, `6.0`, where the loss is already close to zero.
    /// dtype: str
    ///     The data type to be employed, by default f32.
    /// verbose: bool = True
    ///     Whether to show the loading bar.
    pub fn new(py_kwargs: Option<&PyDict>) -> PyResult<WalkletsCBOW> {
        Ok(Self {
            inner: WalkletsBinding::new(Node2VecModels::CBOW, py_kwargs)?,
        })
    }
}

#[pymethods]
impl WalkletsCBOW {
    #[args(py_kwargs = "**")]
    #[pyo3(text_signature = "($self, graph)")]
    /// Return numpy embedding with Walklets CBOW node embedding.
    ///
    /// Parameters
    /// ---------
    /// graph: Graph
    ///     The graph to embed.
    fn fit_transform(&self, graph: &Graph) -> PyResult<Vec<Py<PyAny>>> {
        self.inner.fit_transform(graph)
    }
}

#[pyclass]
#[derive(Debug, Clone)]
#[pyo3(
    text_signature = "(*, embedding_size, window_size, number_of_negative_samples, walk_length, return_weight, explore_weight, change_edge_type_weight, change_node_type_weight, random_state, iterations, max_neighbours, normalize_by_degree, epochs, learning_rate, learning_rate_decay, central_nodes_embedding_path, contextual_nodes_embedding_path, stochastic_downsample_by_degree, normalize_learning_rate_by_degree, use_scale_free_distribution, clipping_value, dtype, verbose)"
)]
pub struct WalkletsSkipGram {
    inner: WalkletsBinding,
}

#[pymethods]
impl WalkletsSkipGram {
    #[new]
    #[args(py_kwargs = "**")]
    /// Return a new instance of the Walklets SkipGram model.
    ///
    /// Parameters
    /// ------------------------
    /// embedding_size: Optional[int] = 100
    ///     Size of the embedding.
    /// window_size: Optional[int] = 10
    ///     Window size defining the contexts.
    /// number_of_negative_samples: Optional[int] = 5
    ///     Number of negative samples to extract for each context.
    /// walk_length: Optional[int] = 32
    ///     Maximal length of the random walk.
    ///     On graphs without traps, all walks have this length.
    /// return_weight: float = 1.0
    ///     Weight on the probability of returning to node coming from
    ///     Having this higher tends the walks to be
    ///     more like a Breadth-First Search.
    ///     Having this very high  (> 2) makes search very local.
    ///     Equal to the inverse of p in the Node2Vec paper.
    /// explore_weight: float = 1.0
    ///     Weight on the probability of visiting a neighbor node
    ///     to the one we're coming from in the random walk
    ///     Having this higher tends the walks to be
    ///     more like a Depth-First Search.
    ///     Having this very high makes search more outward.
    ///     Having this very low makes search very local.
    ///     Equal to the inverse of q in the Node2Vec paper.
    /// change_edge_type_weight: float = 1.0
    ///     Weight on the probability of visiting a neighbor node of a
    ///     different type than the previous node. This only applies to
    ///     colored graphs, otherwise it has no impact.
    /// change_node_type_weight: float = 1.0
    ///     Weight on the probability of visiting a neighbor edge of a
    ///     different type than the previous edge. This only applies to
    ///     multigraphs, otherwise it has no impact.
    /// random_state: int = 42
    ///     random_state to use to reproduce the walks.
    /// iterations: int = 1
    ///     Number of cycles on the graphs to execute.
    /// max_neighbours: Optional[int] = 100
    ///     Maximum number of randomly sampled neighbours to consider.
    ///     If this parameter is used, the walks becomes probabilistic in nature
    ///     and becomes an approximation of an exact walk.
    /// normalize_by_degree: Optional[bool] = False
    ///     Whether to normalize the random walks by the node degree.
    /// epochs: int = 100
    ///     Number of epochs to train the model for.
    /// learning_rate: float = 0.01
    ///     Learning rate of the model.
    /// learning_rate_decay: float = 0.9
    ///     Amount of learning rate decay for each epoch.
    /// central_nodes_embedding_path: Optional[str] = None
    ///     Path where to mmap and store the central nodes embedding.
    ///     If provided, we expect the path to contain the substring `{window_size}` which
    ///     will be replaced with the i-th window size embedding that is being computed.
    ///     This is necessary to embed large graphs whose embedding will not
    ///     fit into the available main memory.
    /// contextual_nodes_embedding_path: Optional[str] = None
    ///     Path where to mmap and store the central nodes embedding.
    ///     If provided, we expect the path to contain the substring `{window_size}` which
    ///     will be replaced with the i-th window size embedding that is being computed.
    ///     This is necessary to embed large graphs whose embedding will not
    ///     fit into the available main memory.
    /// stochastic_downsample_by_degree: Optional[bool]
    ///     Randomly skip samples with probability proportional to the degree of the central node. By default false.
    /// normalize_learning_rate_by_degree: Optional[bool]
    ///     Divide the learning rate by the degree of the central node. By default false.
    /// use_scale_free_distribution: Optional[bool]
    ///     Sample negatives proportionally to their degree. By default true.
    /// clipping_value: Optional[float] = 6.0
    ///     Value at which we clip the dot product, mostly for numerical stability issues.
    ///     By default, `6.0`, where the loss is already close to zero.
    /// dtype: str
    ///     The data type to be employed, by default f32.
    /// verbose: bool = True
    ///     Whether to show the loading bar.
    pub fn new(py_kwargs: Option<&PyDict>) -> PyResult<WalkletsSkipGram> {
        Ok(Self {
            inner: WalkletsBinding::new(Node2VecModels::SkipGram, py_kwargs)?,
        })
    }
}

#[pymethods]
impl WalkletsSkipGram {
    #[args(py_kwargs = "**")]
    #[pyo3(text_signature = "($self, graph)")]
    /// Return numpy embedding with Walklets SkipGram node embedding.
    ///
    /// Parameters
    /// ---------
    /// graph: Graph
    ///     The graph to embed.
    fn fit_transform(&self, graph: &Graph) -> PyResult<Vec<Py<PyAny>>> {
        self.inner.fit_transform(graph)
    }
}

#[pyclass]
#[derive(Debug, Clone)]
#[pyo3(
    text_signature = "(*, embedding_size, window_size, number_of_negative_samples, walk_length, return_weight, explore_weight, change_edge_type_weight, change_node_type_weight, random_state, iterations, max_neighbours, normalize_by_degree, epochs, learning_rate, learning_rate_decay, central_nodes_embedding_path, contextual_nodes_embedding_path, stochastic_downsample_by_degree, normalize_learning_rate_by_degree, use_scale_free_distribution, clipping_value, dtype, verbose)"
)]
pub struct WalkletsGloVe {
    inner: WalkletsBinding,
}

#[pymethods]
impl WalkletsGloVe {
    #[new]
    #[args(py_kwargs = "**")]
    /// Return a new instance of the Walklets GloVe model.
    ///
    /// Parameters
    /// ------------------------
    /// embedding_size: Optional[int] = 100
    ///     Size of the embedding.
    /// window_size: Optional[int] = 10
    ///     Window size defining the contexts.
    /// number_of_negative_samples: Optional[int] = 5
    ///     Number of negative samples to extract for each context.
    /// walk_length: Optional[int] = 32
    ///     Maximal length of the random walk.
    ///     On graphs without traps, all walks have this length.
    /// return_weight: float = 1.0
    ///     Weight on the probability of returning to node coming from
    ///     Having this higher tends the walks to be
    ///     more like a Breadth-First Search.
    ///     Having this very high  (> 2) makes search very local.
    ///     Equal to the inverse of p in the Node2Vec paper.
    /// explore_weight: float = 1.0
    ///     Weight on the probability of visiting a neighbor node
    ///     to the one we're coming from in the random walk
    ///     Having this higher tends the walks to be
    ///     more like a Depth-First Search.
    ///     Having this very high makes search more outward.
    ///     Having this very low makes search very local.
    ///     Equal to the inverse of q in the Node2Vec paper.
    /// change_edge_type_weight: float = 1.0
    ///     Weight on the probability of visiting a neighbor node of a
    ///     different type than the previous node. This only applies to
    ///     colored graphs, otherwise it has no impact.
    /// change_node_type_weight: float = 1.0
    ///     Weight on the probability of visiting a neighbor edge of a
    ///     different type than the previous edge. This only applies to
    ///     multigraphs, otherwise it has no impact.
    /// random_state: int = 42
    ///     random_state to use to reproduce the walks.
    /// iterations: int = 1
    ///     Number of cycles on the graphs to execute.
    /// max_neighbours: Optional[int] = 100
    ///     Maximum number of randomly sampled neighbours to consider.
    ///     If this parameter is used, the walks becomes probabilistic in nature
    ///     and becomes an approximation of an exact walk.
    /// normalize_by_degree: Optional[bool] = False
    ///     Whether to normalize the random walks by the node degree.
    /// epochs: int = 100
    ///     Number of epochs to train the model for.
    /// learning_rate: float = 0.01
    ///     Learning rate of the model.
    /// learning_rate_decay: float = 0.9
    ///     Amount of learning rate decay for each epoch.
    /// central_nodes_embedding_path: Optional[str] = None
    ///     Path where to mmap and store the central nodes embedding.
    ///     If provided, we expect the path to contain the substring `{window_size}` which
    ///     will be replaced with the i-th window size embedding that is being computed.
    ///     This is necessary to embed large graphs whose embedding will not
    ///     fit into the available main memory.
    /// contextual_nodes_embedding_path: Optional[str] = None
    ///     Path where to mmap and store the central nodes embedding.
    ///     If provided, we expect the path to contain the substring `{window_size}` which
    ///     will be replaced with the i-th window size embedding that is being computed.
    ///     This is necessary to embed large graphs whose embedding will not
    ///     fit into the available main memory.
    /// stochastic_downsample_by_degree: Optional[bool]
    ///     Randomly skip samples with probability proportional to the degree of the central node. By default false.
    /// normalize_learning_rate_by_degree: Optional[bool]
    ///     Divide the learning rate by the degree of the central node. By default false.
    /// use_scale_free_distribution: Optional[bool]
    ///     Sample negatives proportionally to their degree. By default true.
    /// clipping_value: Optional[float] = 6.0
    ///     Value at which we clip the dot product, mostly for numerical stability issues.
    ///     By default, `6.0`, where the loss is already close to zero.
    /// dtype: str
    ///     The data type to be employed, by default f32.
    /// verbose: bool = True
    ///     Whether to show the loading bar.
    pub fn new(py_kwargs: Option<&PyDict>) -> PyResult<WalkletsGloVe> {
        Ok(Self {
            inner: WalkletsBinding::new(Node2VecModels::GloVe, py_kwargs)?,
        })
    }
}

#[pymethods]
impl WalkletsGloVe {
    #[args(py_kwargs = "**")]
    #[pyo3(text_signature = "($self, graph)")]
    /// Return numpy embedding with Walklets GloVe node embedding.
    ///
    /// Parameters
    /// ---------
    /// graph: Graph
    ///     The graph to embed.
    fn fit_transform(&self, graph: &Graph) -> PyResult<Vec<Py<PyAny>>> {
        self.inner.fit_transform(graph)
    }
}
