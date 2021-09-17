use super::*;
use graph::{
    cooccurence_matrix as rust_cooccurence_matrix, okapi_bm25_tfidf as rust_okapi_bm25_tfidf,
    word2vec as rust_word2vec, NodeT, NodeTypeT,
};
use numpy::{PyArray, PyArray1, PyArray2};
use pyo3::wrap_pyfunction;
use rayon::prelude::*;
use std::sync::atomic::{AtomicUsize, Ordering};
use types::ThreadDataRaceAware;

#[pymodule]
fn preprocessing(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(word2vec))?;
    m.add_wrapped(wrap_pyfunction!(cooccurence_matrix))?;
    m.add_wrapped(wrap_pyfunction!(okapi_bm25_tfidf_int))?;
    m.add_wrapped(wrap_pyfunction!(okapi_bm25_tfidf_str))?;
    Ok(())
}

#[pyfunction()]
#[text_signature = "(documents, k1, b, vocabulary_size, verbose)"]
/// Return vocabulary and TFIDF matrix of given documents.
///
///
/// Arguments
/// ---------
/// documents: List[List[str]],
///     The documents to parse
/// k1: Optional[float],
///     The default parameter for k1, tipically between 1.2 and 2.0.
/// b: Optional[float],
///     The default parameter for b, tipically equal to 0.75.
/// vocabulary_size: Optional[int],
///     The expected vocabulary size.
/// verbose: Optional[bool],
///     Whether to show a loading bar.
///
fn okapi_bm25_tfidf_int(
    documents: Vec<Vec<u64>>,
    k1: Option<f64>,
    b: Option<f64>,
    vocabulary_size: Option<usize>,
    verbose: Option<bool>,
) -> PyResult<Vec<HashMap<u64, f64>>> {
    pe!(rust_okapi_bm25_tfidf::<u64>(
        &documents,
        k1,
        b,
        vocabulary_size,
        verbose
    ))
}

#[pyfunction()]
#[text_signature = "(documents, k1, b, vocabulary_size, verbose)"]
/// Return vocabulary and TFIDF matrix of given documents.
///
///
/// Arguments
/// ---------
/// documents: List[List[str]],
///     The documents to parse
/// k1: Optional[float],
///     The default parameter for k1, tipically between 1.2 and 2.0.
/// b: Optional[float],
///     The default parameter for b, tipically equal to 0.75.
/// vocabulary_size: Optional[int],
///     The expected vocabulary size.
/// verbose: Optional[bool],
///     Whether to show a loading bar.
///
fn okapi_bm25_tfidf_str(
    documents: Vec<Vec<&str>>,
    k1: Option<f64>,
    b: Option<f64>,
    vocabulary_size: Option<usize>,
    verbose: Option<bool>,
) -> PyResult<Vec<HashMap<&str, f64>>> {
    pe!(rust_okapi_bm25_tfidf::<&str>(
        &documents,
        k1,
        b,
        vocabulary_size,
        verbose
    ))
}

#[pyfunction(py_kwargs = "**")]
#[text_signature = "(sequences, window_size)"]
/// Return training batches for Word2Vec models.
///
/// The batch is composed of a tuple as the following:
///
/// - (Contexts indices, central nodes indices): the tuple of nodes
///
/// This does not provide any output value as the model uses NCE loss
/// and basically the central nodes that are fed as inputs work as the
/// outputs value.
///
/// Arguments
/// ---------
///
/// sequences: List[List[int]],
///     the sequence of sequences of integers to preprocess.
/// window_size: int,
///     Window size to consider for the sequences.
///
fn word2vec(sequences: Vec<Vec<NodeT>>, window_size: usize) -> (PyContexts, PyWords) {
    let (contexts, words): (Vec<Vec<NodeT>>, Vec<NodeT>) =
        rust_word2vec(sequences.into_par_iter(), window_size).unzip();
    let gil = pyo3::Python::acquire_gil();
    (
        to_ndarray_2d!(gil, contexts, NodeT),
        to_ndarray_1d!(gil, words, NodeT),
    )
}

#[pyfunction(py_kwargs = "**")]
#[text_signature = "(sequences, *, window_size, verbose)"]
/// Return triple with CSR representation of cooccurrence matrix.
///
/// The first vector has the sources, the second vector the destinations
/// and the third one contains the min-max normalized frequencies.
///
/// Arguments
/// ---------
///
/// sequences: List[List[int]],
///     the sequence of sequences of integers to preprocess.
/// window_size: int = 4,
///     Window size to consider for the sequences.
/// verbose: bool = False,
///     whether to show the progress bars.
///     The default behaviour is false.
///     
fn cooccurence_matrix(
    sequences: Vec<Vec<NodeT>>,
    py_kwargs: Option<&PyDict>,
) -> PyResult<(PyWords, PyWords, PyFrequencies)> {
    let _ = ctrlc::set_handler(|| std::process::exit(2));
    let gil = pyo3::Python::acquire_gil();
    let kwargs = normalize_kwargs!(py_kwargs, gil.python());
    pe!(validate_kwargs(kwargs, &["window_size", "verbose"]))?;
    let len = sequences.len();

    let (number_of_elements, iter) = pe!(rust_cooccurence_matrix(
        sequences.into_par_iter(),
        extract_value!(kwargs, "window_size", usize).unwrap_or(3),
        len,
        extract_value!(kwargs, "verbose", bool)
    ))?;

    let srcs = PyArray1::new(gil.python(), [number_of_elements], false);
    let dsts = PyArray1::new(gil.python(), [number_of_elements], false);
    let frequencies = PyArray1::new(gil.python(), [number_of_elements], false);

    iter.enumerate().for_each(|(i, (src, dst, freq))| unsafe {
        *srcs.uget_mut(i) = src;
        *dsts.uget_mut(i) = dst;
        *frequencies.uget_mut(i) = freq;
    });

    Ok((srcs.to_owned(), dsts.to_owned(), frequencies.to_owned()))
}

#[pymethods]
impl Graph {
    #[args(py_kwargs = "**")]
    #[text_signature = "($self, walk_length, *, window_size, iterations, return_weight, explore_weight, change_edge_type_weight, change_node_type_weight, dense_node_mapping, max_neighbours, random_state, verbose)"]
    /// Return cooccurence matrix-based triples of words, contexts and frequencies.
    ///
    /// Parameters
    /// ---------------------
    /// walk_length: int,
    ///     Maximal length of the random walk.
    ///     On graphs without traps, all walks have this length.
    /// window_size: int = 4,
    ///     Size of the window for local contexts.
    /// iterations: int = 1,
    ///     Number of cycles on the graphs to execute.
    /// return_weight: float = 1.0,
    ///     Weight on the probability of returning to node coming from
    ///     Having this higher tends the walks to be
    ///     more like a Breadth-First Search.
    ///     Having this very high  (> 2) makes search very local.
    ///     Equal to the inverse of p in the Node2Vec paper.
    /// explore_weight: float = 1.0,
    ///     Weight on the probability of visiting a neighbor node
    ///     to the one we're coming from in the random walk
    ///     Having this higher tends the walks to be
    ///     more like a Depth-First Search.
    ///     Having this very high makes search more outward.
    ///     Having this very low makes search very local.
    ///     Equal to the inverse of q in the Node2Vec paper.
    /// change_node_type_weight: float = 1.0,
    ///     Weight on the probability of visiting a neighbor node of a
    ///     different type than the previous node. This only applies to
    ///     colored graphs, otherwise it has no impact.
    /// change_edge_type_weight: float = 1.0,
    ///     Weight on the probability of visiting a neighbor edge of a
    ///     different type than the previous edge. This only applies to
    ///     multigraphs, otherwise it has no impact.
    /// dense_node_mapping: Dict[int, int] = None,
    ///     Mapping to use for converting sparse walk space into a dense space.
    ///     This object can be created using the method available from graph
    ///     called `get_dense_node_mapping` that returns a mapping from
    ///     the non trap nodes (those from where a walk could start) and
    ///     maps these nodes into a dense range of values.
    /// max_neighbours: int = None,
    ///     Maximum number of randomly sampled neighbours to consider.
    ///     If this parameter is used, the walks becomes probabilistic in nature
    ///     and becomes an approximation of an exact walk.
    /// random_state: int = 42,
    ///     random_state to use to reproduce the walks.
    /// verbose: int = True,
    ///     whether to show or not the loading bar of the walks.
    ///
    /// Returns
    /// ----------------------------
    /// Triple with integer vectors of words and contexts and max-min normalized frequencies.
    ///
    fn cooccurence_matrix(
        &self,
        walk_length: u64,
        py_kwargs: Option<&PyDict>,
    ) -> PyResult<(PyWords, PyWords, PyFrequencies)> {
        let gil = pyo3::Python::acquire_gil();
        let kwargs = normalize_kwargs!(py_kwargs, gil.python());

        pe!(validate_kwargs(
            kwargs,
            build_walk_parameters_list(&["window_size", "verbose"]).as_slice(),
        ))?;

        let parameters = pe!(self.build_walk_parameters(walk_length, kwargs))?;

        let (number_of_elements, iter) = pe!(self.inner.cooccurence_matrix(
            &parameters,
            extract_value!(kwargs, "window_size", usize).unwrap_or(3),
            extract_value!(kwargs, "verbose", bool)
        ))?;

        let srcs = PyArray1::new(gil.python(), [number_of_elements], false);
        let dsts = PyArray1::new(gil.python(), [number_of_elements], false);
        let frequencies = PyArray1::new(gil.python(), [number_of_elements], false);

        iter.enumerate().for_each(|(i, (src, dst, freq))| unsafe {
            *srcs.uget_mut(i) = src;
            *dsts.uget_mut(i) = dst;
            *frequencies.uget_mut(i) = freq;
        });

        Ok((srcs.to_owned(), dsts.to_owned(), frequencies.to_owned()))
    }

    #[args(py_kwargs = "**")]
    #[text_signature = "($self, batch_size, walk_length, window_size, *, iterations, return_weight, explore_weight, change_edge_type_weight, change_node_type_weight, dense_node_mapping, max_neighbours, random_state)"]
    /// Return training batches for Node2Vec models.
    ///
    /// The batch is composed of a tuple as the following:
    ///
    /// - (Contexts indices, central nodes indices): the tuple of nodes
    ///
    /// This does not provide any output value as the model uses NCE loss
    /// and basically the central nodes that are fed as inputs work as the
    /// outputs value.
    ///
    /// Parameters
    /// ---------------------
    /// batch_size:
    ///     Number of walks to include within this batch.
    ///     In some pathological cases, this might leed to an empty batch.
    ///     These cases include graphs with particularly high number of traps.
    ///     Consider using the method graph.report() to verify if this might
    ///     apply to your use case.
    /// walk_length: int,
    ///     Maximal length of the random walk.
    ///     On graphs without traps, all walks have this length.
    /// window_size: int,
    ///     Size of the window for local contexts.
    /// iterations: int = 1,
    ///     Number of iterations for each node.
    /// return_weight: float = 1.0,
    ///     Weight on the probability of returning to node coming from
    ///     Having this higher tends the walks to be
    ///     more like a Breadth-First Search.
    ///     Having this very high  (> 2) makes search very local.
    ///     Equal to the inverse of p in the Node2Vec paper.
    /// explore_weight: float = 1.0,
    ///     Weight on the probability of visiting a neighbor node
    ///     to the one we're coming from in the random walk
    ///     Having this higher tends the walks to be
    ///     more like a Depth-First Search.
    ///     Having this very high makes search more outward.
    ///     Having this very low makes search very local.
    ///     Equal to the inverse of q in the Node2Vec paper.
    /// change_node_type_weight: float = 1.0,
    ///     Weight on the probability of visiting a neighbor node of a
    ///     different type than the previous node. This only applies to
    ///     colored graphs, otherwise it has no impact.
    /// change_edge_type_weight: float = 1.0,
    ///     Weight on the probability of visiting a neighbor edge of a
    ///     different type than the previous edge. This only applies to
    ///     multigraphs, otherwise it has no impact.
    /// dense_node_mapping: Dict[int, int],
    ///     Mapping to use for converting sparse walk space into a dense space.
    ///     This object can be created using the method available from graph
    ///     called `get_dense_node_mapping` that returns a mapping from
    ///     the non trap nodes (those from where a walk could start) and
    ///     maps these nodes into a dense range of values.
    /// max_neighbours: int = None,
    ///     Maximum number of randomly sampled neighbours to consider.
    ///     If this parameter is used, the walks becomes probabilistic in nature
    ///     and becomes an approximation of an exact walk.
    /// random_state: int,
    ///     random_state to use to reproduce the walks.
    ///
    /// Returns
    /// ----------------------------
    /// Tuple with vector of integer with contexts and words.
    fn node2vec(
        &self,
        batch_size: NodeT,
        walk_length: u64,
        window_size: usize,
        py_kwargs: Option<&PyDict>,
    ) -> PyResult<(PyContexts, PyWords)> {
        let gil = pyo3::Python::acquire_gil();
        let kwargs = normalize_kwargs!(py_kwargs, gil.python());
        pe!(validate_kwargs(
            kwargs,
            build_walk_parameters_list(&[]).as_slice()
        ))?;
        let parameters = pe!(self.build_walk_parameters(walk_length, kwargs))?;

        let iter = pe!(self.inner.node2vec(&parameters, batch_size, window_size))?;

        let elements_per_batch = (walk_length as usize - window_size * 2)
            * batch_size as usize
            * parameters.get_iterations() as usize;

        let contexts = ThreadDataRaceAware {
            t: PyArray2::new(gil.python(), [elements_per_batch, window_size * 2], false),
        };
        let words = ThreadDataRaceAware {
            t: PyArray1::new(gil.python(), [elements_per_batch], false),
        };
        let global_i = AtomicUsize::new(0);

        iter.for_each(|(context, word)| {
            let i = global_i.fetch_add(1, Ordering::SeqCst);
            context.iter().enumerate().for_each(|(j, v)| unsafe {
                *(contexts.t.uget_mut([i, j])) = *v;
            });
            unsafe {
                *(words.t.uget_mut([i])) = word;
            }
        });
        Ok((contexts.t.to_owned(), words.t.to_owned()))
    }

    #[args(py_kwargs = "**")]
    #[text_signature = "($self, idx, batch_size, include_central_node, return_edge_weights, max_neighbours)"]
    /// Return iterator over neighbours for the given node
    ///
    /// Parameters
    /// -----------------------------
    /// `idx`: int - Seed for the batch.
    /// `batch_size`: Optional[int] = 1024 - The dimension of the batch.
    /// `include_central_node`: Optional[bool] - Whether to include the central node.
    /// `return_edge_weights`: Optional[bool] - Whether to return the edge weights.
    /// `max_neighbours`: Optional[int] - Maximal number of neighbours to sample.
    ///
    /// Returns
    /// -----------------------------
    /// Tuple with input nodes, optionally edge weights and one-hot encoded node types.
    ///
    fn get_node_label_prediction_mini_batch(
        &self,
        idx: u64,
        batch_size: Option<NodeT>,
        include_central_node: Option<bool>,
        return_edge_weights: Option<bool>,
        max_neighbours: Option<NodeT>,
    ) -> PyResult<(
        (Vec<Vec<NodeT>>, Option<Vec<Vec<WeightT>>>),
        Py<PyArray2<NodeTypeT>>,
    )> {
        let gil = pyo3::Python::acquire_gil();

        let nodes_number = self.inner.get_nodes_number();
        // Get the batch size
        let batch_size = batch_size.unwrap_or(1024).min(nodes_number);
        // Whether to include or not the edge weights
        let return_edge_weights = return_edge_weights.unwrap_or(false);

        // We retrieve the batch iterator.
        let iter = pe!(self.inner.get_node_label_prediction_mini_batch(
            idx,
            Some(batch_size),
            include_central_node,
            Some(return_edge_weights),
            max_neighbours,
        ))?;

        // We create the vector of zeros for the one-hot encoded labels.
        // This is also used for the multi-label case.
        // This vector has the same number of rows as the previous vector,
        // that is the number of requested node IDs, while the number
        // of columns is the number of node types in the graph.
        let labels = ThreadDataRaceAware {
            t: PyArray2::zeros(
                gil.python(),
                [
                    batch_size as usize,
                    pe!(self.inner.get_node_types_number())? as usize,
                ],
                false,
            ),
        };

        // We iterate over the batch.
        let (destinations, edge_weights) = if return_edge_weights {
            let (destinations, edge_weights): (Vec<Vec<NodeT>>, Vec<Vec<WeightT>>) = iter
                .enumerate()
                .map(|(i, ((destinations, weights), node_types))| {
                    node_types.into_iter().for_each(|label| unsafe {
                        *labels.t.uget_mut([i, label as usize]) = 1;
                    });
                    (destinations, weights.unwrap())
                })
                .unzip();
            (destinations, Some(edge_weights))
        } else {
            (
                iter.enumerate()
                    .map(|(i, ((destinations, _), node_types))| {
                        node_types.into_iter().for_each(|label| unsafe {
                            *labels.t.uget_mut([i, label as usize]) = 1;
                        });
                        destinations
                    })
                    .collect::<Vec<Vec<NodeT>>>(),
                None,
            )
        };

        Ok(((destinations, edge_weights), labels.t.to_owned()))
    }

    #[text_signature = "($self, idx, batch_size, negative_samples_rate, return_node_types, return_edge_types, avoid_false_negatives, maximal_sampling_attempts, shuffle, graph_to_avoid)"]
    /// Returns n-ple with index to build numpy array, source node, source node type, destination node, destination node type, edge type and whether this edge is real or artificial.
    ///
    /// Parameters
    /// -------------
    /// idx: int,
    ///     The index of the batch to generate, behaves like a random random_state,
    /// batch_size: Optional[int],
    ///     The maximal size of the batch to generate,
    /// negative_samples: Optional[float],
    ///     The component of netagetive samples to use.
    /// return_node_types: Optional[bool],
    ///     Whether to return the source and destination nodes node types.
    /// return_edge_types: Optional[bool],
    ///     Whether to return the edge types. The negative edges edge type will be samples at random.
    /// return_edge_metrics: Optional[bool],
    ///     Whether to return the edge metrics.
    /// avoid_false_negatives: Optional[bool],
    ///     Whether to remove the false negatives when generated. It should be left to false, as it has very limited impact on the training, but enabling this will slow things down.
    /// maximal_sampling_attempts: Optional[int],
    ///     Number of attempts to execute to sample the negative edges.
    /// shuffle: Optional[bool],
    ///     Whether to shuffle the samples within the batch.
    /// graph_to_avoid: Optional[Graph],
    ///     The graph whose edges are to be avoided during the generation of false negatives,
    ///
    /// Raises
    /// ---------
    /// ValueError
    ///     If the given amount of negative samples is not a positive finite real value.
    /// ValueError
    ///     If node types are requested but the graph does not contain any.
    /// ValueError
    ///     If node types are requested but the graph contains unknown node types.
    /// ValueError
    ///     If edge types are requested but the graph does not contain any.
    /// ValueError
    ///     If edge types are requested but the graph contains unknown edge types.
    fn get_edge_prediction_mini_batch(
        &self,
        idx: u64,
        batch_size: Option<usize>,
        negative_samples_rate: Option<f64>,
        return_node_types: Option<bool>,
        return_edge_types: Option<bool>,
        return_edge_metrics: Option<bool>,
        avoid_false_negatives: Option<bool>,
        maximal_sampling_attempts: Option<usize>,
        shuffle: Option<bool>,
        graph_to_avoid: Option<Graph>,
    ) -> PyResult<(
        Py<PyArray1<NodeT>>,
        Option<Py<PyArray2<NodeTypeT>>>,
        Py<PyArray1<NodeT>>,
        Option<Py<PyArray2<NodeTypeT>>>,
        Option<Py<PyArray2<f64>>>,
        Option<Py<PyArray1<EdgeTypeT>>>,
        Py<PyArray1<bool>>,
    )> {
        let gil = pyo3::Python::acquire_gil();
        let return_node_types = return_node_types.unwrap_or(false);
        let return_edge_types = return_edge_types.unwrap_or(false);
        let return_edge_metrics = return_edge_metrics.unwrap_or(false);
        let batch_size = batch_size.unwrap_or(1024);

        let graph_to_avoid = graph_to_avoid.map(|ensmallen| ensmallen.inner);
        let par_iter = pe!(self.inner.get_edge_prediction_mini_batch(
            idx,
            Some(batch_size),
            negative_samples_rate,
            Some(return_node_types),
            Some(return_edge_types),
            Some(return_edge_metrics),
            avoid_false_negatives,
            maximal_sampling_attempts,
            shuffle,
            graph_to_avoid.as_ref(),
        ))?;

        let srcs = ThreadDataRaceAware {
            t: PyArray1::new(gil.python(), [batch_size], false),
        };
        let dsts = ThreadDataRaceAware {
            t: PyArray1::new(gil.python(), [batch_size], false),
        };
        let (src_node_type_ids, dst_node_type_ids) = if return_node_types {
            let max_node_type_count = pe!(self.inner.get_maximum_multilabel_count())? as usize;
            (
                Some(ThreadDataRaceAware {
                    t: PyArray2::new(gil.python(), [batch_size, max_node_type_count], false),
                }),
                Some(ThreadDataRaceAware {
                    t: PyArray2::new(gil.python(), [batch_size, max_node_type_count], false),
                }),
            )
        } else {
            (None, None)
        };
        let edges_metrics = if return_edge_metrics {
            Some(ThreadDataRaceAware {
                t: PyArray2::new(gil.python(), [batch_size, 4], false),
            })
        } else {
            None
        };
        let edge_type_ids = if return_edge_types {
            Some(ThreadDataRaceAware {
                t: PyArray1::new(gil.python(), [batch_size], false),
            })
        } else {
            None
        };
        let labels = ThreadDataRaceAware {
            t: PyArray1::new(gil.python(), [batch_size], false),
        };

        unsafe {
            par_iter.enumerate().for_each(
                |(i, (src, src_node_type, dst, dst_node_type, edge_features, edge_type, label))| {
                    *(dsts.t.uget_mut([i])) = src;
                    *(srcs.t.uget_mut([i])) = dst;
                    if let (Some(src_node_type_ids), Some(dst_node_type_ids)) =
                        (src_node_type_ids.as_ref(), dst_node_type_ids.as_ref())
                    {
                        src_node_type.unwrap().into_iter().enumerate().for_each(
                            |(j, node_type)| {
                                *(src_node_type_ids.t.uget_mut([i, j])) = node_type;
                            },
                        );
                        dst_node_type.unwrap().into_iter().enumerate().for_each(
                            |(j, node_type)| {
                                *(dst_node_type_ids.t.uget_mut([i, j])) = node_type;
                            },
                        );
                    }
                    if let Some(edges_metrics) = edges_metrics.as_ref() {
                        edge_features
                            .unwrap()
                            .into_iter()
                            .enumerate()
                            .for_each(|(j, metric)| {
                                *(edges_metrics.t.uget_mut([i, j])) = metric;
                            });
                    }
                    if let Some(edge_type_ids) = edge_type_ids.as_ref() {
                        *(edge_type_ids.t.uget_mut([i])) = edge_type.unwrap();
                    }
                    *(labels.t.uget_mut([i])) = label;
                },
            );
        }

        Ok((
            srcs.t.to_owned(),
            src_node_type_ids.map(|x| x.t.to_owned()),
            dsts.t.to_owned(),
            dst_node_type_ids.map(|x| x.t.to_owned()),
            edges_metrics.map(|x| x.t.to_owned()),
            edge_type_ids.map(|x| x.t.to_owned()),
            labels.t.to_owned(),
        ))
    }

    #[text_signature = "($self, source_node_ids, destination_node_ids, normalize, verbose)"]
    /// Returns all available edge prediction metrics for given edges.
    ///
    /// The metrics returned are, in order:
    /// - Adamic Adar index
    /// - Jaccard Coefficient
    /// - Resource Allocation index
    /// - Normalized preferential attachment score
    ///
    /// Parameters
    /// -----------------------------
    /// source_node_ids: List[int],
    ///     List of source node IDs.
    /// destination_node_ids: List[int],
    ///     List of destination node IDs.
    /// normalize: Optional[bool] = True,
    ///     Whether to normalize the metrics.
    /// verbose: Optional[bool] = True,
    ///     Whether to show a loading bar.
    ///
    /// Returns
    /// -----------------------------
    /// 2D numpy array with metrics.
    fn get_unchecked_edge_prediction_metrics(
        &self,
        source_node_ids: Vec<NodeT>,
        destination_node_ids: Vec<NodeT>,
        normalize: Option<bool>,
        verbose: Option<bool>,
    ) -> Py<PyArray2<f64>> {
        let gil = pyo3::Python::acquire_gil();

        let batch_metrics = ThreadDataRaceAware {
            t: PyArray2::new(gil.python(), [source_node_ids.len(), 4], false),
        };

        unsafe {
            self.inner
                .par_iter_unchecked_edge_prediction_metrics(
                    source_node_ids,
                    destination_node_ids,
                    normalize,
                    verbose,
                )
                .enumerate()
                .for_each(|(i, metrics)| {
                    metrics.into_iter().enumerate().for_each(|(j, metric)| {
                        *(batch_metrics.t.uget_mut([i, j])) = metric;
                    });
                });
        }

        batch_metrics.t.to_owned()
    }

    #[text_signature = "($self, normalize, verbose)"]
    /// Returns all available edge prediction metrics for given edges.
    ///
    /// The metrics returned are, in order:
    /// - Adamic Adar index
    /// - Jaccard Coefficient
    /// - Resource Allocation index
    /// - Normalized preferential attachment score
    ///
    /// Parameters
    /// -----------------------------
    /// normalize: Optional[bool] = True,
    ///     Whether to normalize the metrics.
    /// verbose: Optional[bool] = True,
    ///     Whether to show a loading bar.
    ///
    /// Returns
    /// -----------------------------
    /// 2D numpy array with metrics.
    fn get_edge_prediction_metrics(
        &self,
        normalize: Option<bool>,
        verbose: Option<bool>,
    ) -> Py<PyArray2<f64>> {
        let gil = pyo3::Python::acquire_gil();

        let batch_metrics = ThreadDataRaceAware {
            t: PyArray2::new(
                gil.python(),
                [self.inner.get_directed_edges_number() as usize, 4],
                false,
            ),
        };

        self.inner
            .par_iter_edge_prediction_metrics(normalize, verbose)
            .enumerate()
            .for_each(|(i, metrics)| {
                metrics
                    .into_iter()
                    .enumerate()
                    .for_each(|(j, metric)| unsafe {
                        *(batch_metrics.t.uget_mut([i, j])) = metric;
                    });
            });

        batch_metrics.t.to_owned()
    }
}
