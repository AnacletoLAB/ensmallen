use super::*;
use graph::{
    get_okapi_bm25_tfidf_from_documents as rust_get_okapi_bm25_tfidf_from_documents,
    get_tokenized_csv as rust_get_tokenized_csv, iter_okapi_bm25_tfidf_from_documents,
    word2vec as rust_word2vec, NodeT, NodeTypeT, Tokens,
};
use numpy::{PyArray, PyArray1, PyArray2};
use pyo3::wrap_pyfunction;
use rayon::prelude::*;
use std::sync::atomic::{AtomicUsize, Ordering};
use types::ThreadDataRaceAware;

pub fn register_preprocessing(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(word2vec, m)?)?;
    m.add_function(wrap_pyfunction!(
        get_okapi_bm25_tfidf_from_documents_u16,
        m
    )?)?;
    m.add_function(wrap_pyfunction!(
        get_okapi_bm25_tfidf_from_documents_u32,
        m
    )?)?;
    m.add_function(wrap_pyfunction!(
        get_okapi_bm25_tfidf_from_documents_u64,
        m
    )?)?;
    m.add_function(wrap_pyfunction!(
        get_okapi_bm25_tfidf_from_documents_str,
        m
    )?)?;
    m.add_function(wrap_pyfunction!(
        get_okapi_tfidf_weighted_textual_embedding,
        m
    )?)?;
    Ok(())
}

#[pyfunction()]
#[pyo3(text_signature = "(documents, k1, b, verbose)")]
/// Return list of vocabularies (with same length of the number of documents) with the term and their associated OKAPI BM25 TFIDF score.
///
/// Arguments
/// ---------
/// documents: List[List[str]],
///     The documents to parse
/// k1: Optional[float]
///     The default parameter for k1, tipically between 1.2 and 2.0.
/// b: Optional[float]
///     The default parameter for b, tipically equal to 0.75.
/// verbose: Optional[bool]
///     Whether to show a loading bar. By default true.
///
fn get_okapi_bm25_tfidf_from_documents_u16(
    documents: Vec<Vec<u16>>,
    k1: Option<f32>,
    b: Option<f32>,
    verbose: Option<bool>,
) -> PyResult<Vec<HashMap<u16, f32>>> {
    pe!(rust_get_okapi_bm25_tfidf_from_documents::<u16>(
        &documents, k1, b, verbose
    ))
}

#[pyfunction()]
#[pyo3(text_signature = "(documents, k1, b, verbose)")]
/// Return list of vocabularies (with same length of the number of documents) with the term and their associated OKAPI BM25 TFIDF score.
///
/// Arguments
/// ---------
/// documents: List[List[str]],
///     The documents to parse
/// k1: Optional[float]
///     The default parameter for k1, tipically between 1.2 and 2.0.
/// b: Optional[float]
///     The default parameter for b, tipically equal to 0.75.
/// verbose: Optional[bool]
///     Whether to show a loading bar. By default true.
///
fn get_okapi_bm25_tfidf_from_documents_u32(
    documents: Vec<Vec<u32>>,
    k1: Option<f32>,
    b: Option<f32>,
    verbose: Option<bool>,
) -> PyResult<Vec<HashMap<u32, f32>>> {
    pe!(rust_get_okapi_bm25_tfidf_from_documents::<u32>(
        &documents, k1, b, verbose
    ))
}

#[pyfunction()]
#[pyo3(text_signature = "(documents, k1, b, verbose)")]
/// Return list of vocabularies (with same length of the number of documents) with the term and their associated OKAPI BM25 TFIDF score.
///
/// Arguments
/// ---------
/// documents: List[List[str]],
///     The documents to parse
/// k1: Optional[float]
///     The default parameter for k1, tipically between 1.2 and 2.0.
/// b: Optional[float]
///     The default parameter for b, tipically equal to 0.75.
/// verbose: Optional[bool]
///     Whether to show a loading bar. By default true.
///
fn get_okapi_bm25_tfidf_from_documents_u64(
    documents: Vec<Vec<u64>>,
    k1: Option<f32>,
    b: Option<f32>,
    verbose: Option<bool>,
) -> PyResult<Vec<HashMap<u64, f32>>> {
    pe!(rust_get_okapi_bm25_tfidf_from_documents::<u64>(
        &documents, k1, b, verbose
    ))
}

#[pyfunction()]
#[pyo3(text_signature = "(documents, k1, b, verbose)")]
/// Return list of vocabularies (with same length of the number of documents) with the term and their associated OKAPI BM25 TFIDF score.
///
///
/// Arguments
/// ---------
/// documents: List[List[str]],
///     The documents to parse
/// k1: Optional[float]
///     The default parameter for k1, tipically between 1.2 and 2.0.
/// b: Optional[float]
///     The default parameter for b, tipically equal to 0.75.
/// verbose: Optional[bool]
///     Whether to show a loading bar. By default true.
///
fn get_okapi_bm25_tfidf_from_documents_str(
    documents: Vec<Vec<&str>>,
    k1: Option<f32>,
    b: Option<f32>,
    verbose: Option<bool>,
) -> PyResult<Vec<HashMap<&str, f32>>> {
    pe!(rust_get_okapi_bm25_tfidf_from_documents::<&str>(
        &documents, k1, b, verbose
    ))
}

use half::f16;

#[pyfunction()]
#[pyo3(
    text_signature = "(path, embedding, pretrained_model_name_or_path, k1, b, columns, separator, header, verbose)"
)]
/// Returns embedding of all the term in given CSV weighted by OKAPI/TFIDF.
///
/// Arguments
/// ------------
/// path: str,
///     The path to be processed.
/// embedding: np.ndarray
///     The numpy array to use for the dictionary.
///     This must be compatible with the provided pretrained_model_name_or_path!
/// tokenizer_path: str
///     Path to the tokenizer to use.
/// k1: Optional[float]
///     The default parameter for k1, tipically between 1.2 and 2.0.
/// b: Optional[float]
///     The default parameter for b, tipically equal to 0.75.
/// columns: Optional[List[str]]
///     The columns to be read.
///     If none are given, all the columns will be used.
/// separator: Optional[str]
///     The separator for the CSV.
/// header: Optional[bool]
///     Whether to skip the header.
/// verbose: Optional[bool]
///     Whether to show a loading bar. By default true.
///
fn get_okapi_tfidf_weighted_textual_embedding(
    path: &str,
    embedding: Py<PyArray2<f32>>,
    tokenizer_path: String,
    k1: Option<f32>,
    b: Option<f32>,
    columns: Option<Vec<String>>,
    separator: Option<char>,
    header: Option<bool>,
    verbose: Option<bool>,
) -> PyResult<Py<PyAny>> {
    let tokens = pe!(rust_get_tokenized_csv(
        path,
        tokenizer_path.as_str(),
        columns,
        separator,
        header,
    ))?;
    let rows_number = tokens.len();
    let gil = pyo3::Python::acquire_gil();
    let actual_embedding = embedding.as_ref(gil.python());
    let columns_number = actual_embedding.shape()[1];
    let resulting_embedding = ThreadDataRaceAware {
        t: PyArray2::zeros(gil.python(), [rows_number, columns_number], false),
    };
    let actual_embedding = ThreadDataRaceAware {
        t: actual_embedding,
    };
    match tokens {
        Tokens::TokensU8(inner) => {
            pe!(iter_okapi_bm25_tfidf_from_documents(&inner, k1, b, verbose,))?
                .enumerate()
                .for_each(|(i, scores)| unsafe {
                    let inner = resulting_embedding.t;
                    let original = actual_embedding.t;
                    let document_size = scores.len() as f32;
                    scores.into_iter().for_each(|(k, score)| {
                        let k = k as usize;
                        (0..columns_number).for_each(|j| {
                            *(inner.uget_mut([i, j])) = (f16::from_bits(*(inner.uget_mut([i, j])))
                                + f16::from_f32(original.uget([k, j]) * score / document_size))
                            .to_bits();
                        });
                    });
                });
        }
        Tokens::TokensU16(inner) => {
            pe!(iter_okapi_bm25_tfidf_from_documents(&inner, k1, b, verbose,))?
                .enumerate()
                .for_each(|(i, scores)| unsafe {
                    let inner = resulting_embedding.t;
                    let original = actual_embedding.t;
                    let document_size = scores.len() as f32;
                    scores.into_iter().for_each(|(k, score)| {
                        let k = k as usize;
                        (0..columns_number).for_each(|j| {
                            *(inner.uget_mut([i, j])) = (f16::from_bits(*(inner.uget_mut([i, j])))
                                + f16::from_f32(original.uget([k, j]) * score / document_size))
                            .to_bits();
                        });
                    });
                });
        }
        Tokens::TokensU32(inner) => {
            pe!(iter_okapi_bm25_tfidf_from_documents(&inner, k1, b, verbose,))?
                .enumerate()
                .for_each(|(i, scores)| unsafe {
                    let inner = resulting_embedding.t;
                    let original = actual_embedding.t;
                    let document_size = scores.len() as f32;
                    scores.into_iter().for_each(|(k, score)| {
                        let k = k as usize;
                        (0..columns_number).for_each(|j| {
                            *(inner.uget_mut([i, j])) = (f16::from_bits(*(inner.uget_mut([i, j])))
                                + f16::from_f32(original.uget([k, j]) * score / document_size))
                            .to_bits();
                        });
                    });
                });
        }
        Tokens::TokensU64(inner) => {
            pe!(iter_okapi_bm25_tfidf_from_documents(&inner, k1, b, verbose,))?
                .enumerate()
                .for_each(|(i, scores)| unsafe {
                    let inner = resulting_embedding.t;
                    let original = actual_embedding.t;
                    let document_size = scores.len() as f32;
                    scores.into_iter().for_each(|(k, score)| {
                        let k = k as usize;
                        (0..columns_number).for_each(|j| {
                            *(inner.uget_mut([i, j])) = (f16::from_bits(*(inner.uget_mut([i, j])))
                                + f16::from_f32(original.uget([k, j]) * score / document_size))
                            .to_bits();
                        });
                    });
                });
        }
    }

    let embedding = resulting_embedding.t.to_owned();
    unsafe {
        let ptr = &mut *(*embedding.as_ref(gil.python())).as_array_ptr();
        //libc::free(ptr.descr);
        ptr.descr = numpy::npyffi::PY_ARRAY_API
            .PyArray_DescrFromType(gil.python(), numpy::npyffi::NPY_TYPES::NPY_HALF as _);
    }

    Ok(embedding.into_py(gil.python()))
}

#[pyfunction(py_kwargs = "**")]
#[pyo3(text_signature = "(sequences, window_size)")]
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
/// sequences: List[List[int]]
///     the sequence of sequences of integers to preprocess.
/// window_size: int
///     Window size to consider for the sequences.
///
fn word2vec(
    sequences: Vec<Vec<NodeT>>,
    window_size: usize,
) -> (Py<PyArray2<NodeT>>, Py<PyArray1<NodeT>>) {
    let (contexts, words): (Vec<Vec<NodeT>>, Vec<NodeT>) =
        rust_word2vec(sequences.into_par_iter(), window_size).unzip();
    let gil = pyo3::Python::acquire_gil();
    (
        to_ndarray_2d!(gil, contexts, NodeT),
        to_ndarray_1d!(gil, words, NodeT),
    )
}

#[pymethods]
impl Graph {
    #[args(py_kwargs = "**")]
    #[pyo3(
        text_signature = "($self, batch_size, walk_length, window_size, *, iterations, return_weight, explore_weight, change_edge_type_weight, change_node_type_weight, dense_node_mapping, max_neighbours, random_state)"
    )]
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
    /// batch_size: int
    ///     Number of walks to include within this batch.
    ///     In some pathological cases, this might leed to an empty batch.
    ///     These cases include graphs with particularly high number of traps.
    ///     Consider using the method graph.report() to verify if this might
    ///     apply to your use case.
    /// walk_length: int
    ///     Maximal length of the random walk.
    ///     On graphs without traps, all walks have this length.
    /// window_size: int
    ///     Size of the window for local contexts.
    /// iterations: int = 1
    ///     Number of iterations for each node.
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
    /// change_node_type_weight: float = 1.0
    ///     Weight on the probability of visiting a neighbor node of a
    ///     different type than the previous node. This only applies to
    ///     colored graphs, otherwise it has no impact.
    /// change_edge_type_weight: float = 1.0
    ///     Weight on the probability of visiting a neighbor edge of a
    ///     different type than the previous edge. This only applies to
    ///     multigraphs, otherwise it has no impact.
    /// dense_node_mapping: Dict[int, int]
    ///     Mapping to use for converting sparse walk space into a dense space.
    ///     This object can be created using the method available from graph
    ///     called `get_dense_node_mapping` that returns a mapping from
    ///     the non trap nodes (those from where a walk could start) and
    ///     maps these nodes into a dense range of values.
    /// max_neighbours: Optional[int] = 100
    ///     Maximum number of randomly sampled neighbours to consider.
    ///     If this parameter is used, the walks becomes probabilistic in nature
    ///     and becomes an approximation of an exact walk.
    /// random_state: int
    ///     random_state to use to reproduce the walks.
    ///
    /// Returns
    /// ----------------------------
    /// Tuple with vector of integer with contexts and words.
    fn node2vec(
        &self,
        batch_size: NodeT,
        window_size: usize,
        py_kwargs: Option<&PyDict>,
    ) -> PyResult<(Py<PyArray2<NodeT>>, Py<PyArray1<NodeT>>)> {
        let gil = pyo3::Python::acquire_gil();
        let kwargs = normalize_kwargs!(py_kwargs, gil.python());
        pe!(validate_kwargs(
            kwargs,
            build_walk_parameters_list(&[]).as_slice()
        ))?;
        let parameters = pe!(build_walk_parameters(kwargs))?;
        let walk_length = parameters.get_random_walk_length();

        let iter = pe!(self.inner.node2vec(&parameters, batch_size, window_size))?;

        let elements_per_batch = (walk_length as usize - window_size * 2)
            * batch_size as usize
            * parameters.get_iterations() as usize;

        let contexts = ThreadDataRaceAware {
            t: unsafe { PyArray2::new(gil.python(), [elements_per_batch, window_size * 2], false) },
        };
        let words = ThreadDataRaceAware {
            t: unsafe { PyArray1::new(gil.python(), [elements_per_batch], false) },
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

    #[pyo3(
        text_signature = "($self, random_state, batch_size, return_edge_ids, return_node_types, return_edge_types, return_edge_metrics, sample_only_edges_with_heterogeneous_node_types, negative_samples_rate, avoid_false_negatives, maximal_sampling_attempts, shuffle, use_scale_free_distribution, graph_to_avoid)"
    )]
    /// Returns n-ple with index to build numpy array, source node, source node type, destination node, destination node type, edge type and whether this edge is real or artificial.
    ///
    /// Parameters
    /// -------------
    /// random_state: int
    ///     The index of the batch to generate, behaves like a random random_state,
    /// batch_size: int
    ///     The maximal size of the batch to generate,
    /// return_edge_ids: bool
    ///     Whether to return the edge ids.
    /// return_node_types: bool
    ///     Whether to return the source and destination nodes node types.
    /// return_edge_types: bool
    ///    Whether to return the edge types.
    /// return_edge_metrics: bool
    ///     Whether to return the edge metrics.
    /// sample_only_edges_with_heterogeneous_node_types: bool
    ///     Whether to sample negative edges only with source and destination nodes that have different node types.
    /// negative_samples_rate: Optional[float]
    ///     The component of netagetive samples to use.
    /// avoid_false_negatives: Optional[bool]
    ///     Whether to remove the false negatives when generated. It should be left to false, as it has very limited impact on the training, but enabling this will slow things down.
    /// maximal_sampling_attempts: Optional[int]
    ///     Number of attempts to execute to sample the negative edges.
    /// use_scale_free_distribution: bool = True
    ///     Whether to sample the negative edges following a scale_free distribution.
    ///     By default True.
    /// support: Optional[Graph]
    ///     Graph to use to compute the edge metrics.
    ///     When not provided, the current graph (self) is used.
    /// graph_to_avoid: Optional[Graph]
    ///     The graph whose edges are to be avoided during the generation of false negatives,
    ///
    /// Raises
    /// ---------
    /// ValueError
    ///     If the given amount of negative samples is not a positive finite real value.
    /// ValueError
    ///     If node types are requested but the graph does not contain any.
    /// ValueError
    ///     If the `sample_only_edges_with_heterogeneous_node_types` argument is provided as true, but the graph does not have node types.
    fn get_edge_prediction_mini_batch(
        &self,
        random_state: u64,
        batch_size: usize,
        return_edge_ids: bool,
        return_node_types: bool,
        return_edge_types: bool,
        return_edge_metrics: bool,
        sample_only_edges_with_heterogeneous_node_types: bool,
        negative_samples_rate: Option<f64>,
        avoid_false_negatives: Option<bool>,
        maximal_sampling_attempts: Option<usize>,
        use_scale_free_distribution: Option<bool>,
        support: Option<&Graph>,
        graph_to_avoid: Option<&Graph>,
    ) -> PyResult<(
        Option<Py<PyArray1<EdgeT>>>,
        Py<PyArray1<NodeT>>,
        Option<Py<PyArray2<NodeTypeT>>>,
        Py<PyArray1<NodeT>>,
        Option<Py<PyArray2<NodeTypeT>>>,
        Option<Py<PyArray1<EdgeTypeT>>>,
        Option<Py<PyArray2<f32>>>,
        Py<PyArray1<bool>>,
    )> {
        let gil = pyo3::Python::acquire_gil();

        let graph_to_avoid: Option<&graph::Graph> =
            graph_to_avoid.as_ref().map(|ensmallen| &ensmallen.inner);
        let support: Option<&graph::Graph> = support.as_ref().map(|ensmallen| &ensmallen.inner);
        let par_iter = pe!(self.inner.par_iter_attributed_edge_prediction_mini_batch(
            random_state,
            batch_size,
            return_edge_ids,
            return_node_types,
            return_edge_types,
            return_edge_metrics,
            sample_only_edges_with_heterogeneous_node_types,
            negative_samples_rate,
            avoid_false_negatives,
            maximal_sampling_attempts,
            use_scale_free_distribution,
            support,
            graph_to_avoid,
        ))?;

        let edge_ids = if return_edge_ids {
            Some(ThreadDataRaceAware {
                t: unsafe { PyArray1::new(gil.python(), [batch_size], false) },
            })
        } else {
            None
        };
        let srcs = ThreadDataRaceAware {
            t: unsafe { PyArray1::new(gil.python(), [batch_size], false) },
        };
        let dsts = ThreadDataRaceAware {
            t: unsafe { PyArray1::new(gil.python(), [batch_size], false) },
        };
        let (src_node_type_ids, dst_node_type_ids) = if return_node_types {
            let max_node_type_count = pe!(self.inner.get_maximum_multilabel_count())? as usize;
            (
                Some(ThreadDataRaceAware {
                    t: unsafe {
                        PyArray2::new(gil.python(), [batch_size, max_node_type_count], false)
                    },
                }),
                Some(ThreadDataRaceAware {
                    t: unsafe {
                        PyArray2::new(gil.python(), [batch_size, max_node_type_count], false)
                    },
                }),
            )
        } else {
            (None, None)
        };
        let edge_type_ids = if return_edge_types {
            Some(ThreadDataRaceAware {
                t: unsafe { PyArray1::new(gil.python(), [batch_size], false) },
            })
        } else {
            None
        };
        let edges_metrics = if return_edge_metrics {
            Some(ThreadDataRaceAware {
                t: unsafe { PyArray2::new(gil.python(), [batch_size, 4], false) },
            })
        } else {
            None
        };
        let labels = ThreadDataRaceAware {
            t: unsafe { PyArray1::new(gil.python(), [batch_size], false) },
        };

        unsafe {
            par_iter.enumerate().for_each(
                |(
                    i,
                    (
                        edge_id,
                        src,
                        src_node_type,
                        dst,
                        dst_node_type,
                        edge_type,
                        edge_features,
                        label,
                    ),
                )| {
                    *(dsts.t.uget_mut([i])) = src;
                    *(srcs.t.uget_mut([i])) = dst;
                    if let Some(edge_ids) = edge_ids.as_ref() {
                        *(edge_ids.t.uget_mut([i])) = edge_id.unwrap_or(EdgeT::MAX);
                    }
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
                    if let Some(edge_type_ids) = edge_type_ids.as_ref() {
                        *(edge_type_ids.t.uget_mut([i])) = edge_type.unwrap();
                    }
                    if let Some(edges_metrics) = edges_metrics.as_ref() {
                        edge_features.unwrap().iter().copied().enumerate().for_each(
                            |(j, metric)| {
                                *(edges_metrics.t.uget_mut([i, j])) = metric;
                            },
                        );
                    }
                    *(labels.t.uget_mut([i])) = label;
                },
            );
        }

        Ok((
            edge_ids.map(|x| x.t.to_owned()),
            srcs.t.to_owned(),
            src_node_type_ids.map(|x| x.t.to_owned()),
            dsts.t.to_owned(),
            dst_node_type_ids.map(|x| x.t.to_owned()),
            edge_type_ids.map(|x| x.t.to_owned()),
            edges_metrics.map(|x| x.t.to_owned()),
            labels.t.to_owned(),
        ))
    }

    #[pyo3(text_signature = "($self, random_state, batch_size)")]
    /// Returns n-ple with terms used for training a siamese network.
    ///
    /// Parameters
    /// -------------
    /// random_state: int
    ///     Random state to reproduce sampling
    /// batch_size: int
    ///     The maximal size of the batch to generate,
    ///
    fn get_siamese_mini_batch(
        &self,
        random_state: u64,
        batch_size: usize,
    ) -> (
        Py<PyArray1<NodeT>>,
        Py<PyArray1<NodeT>>,
        Py<PyArray1<NodeT>>,
        Py<PyArray1<NodeT>>,
    ) {
        let gil = pyo3::Python::acquire_gil();

        let srcs = ThreadDataRaceAware {
            t: unsafe { PyArray1::new(gil.python(), [batch_size], false) },
        };

        let dsts = ThreadDataRaceAware {
            t: unsafe { PyArray1::new(gil.python(), [batch_size], false) },
        };

        let not_srcs = ThreadDataRaceAware {
            t: unsafe { PyArray1::new(gil.python(), [batch_size], false) },
        };

        let not_dsts = ThreadDataRaceAware {
            t: unsafe { PyArray1::new(gil.python(), [batch_size], false) },
        };

        self.inner
            .par_iter_siamese_mini_batch(random_state, batch_size)
            .enumerate()
            .for_each(|(i, (_, src, dst, not_src, not_dst))| unsafe {
                for (node_ndarray, node) in [
                    (&srcs, src),
                    (&dsts, dst),
                    (&not_srcs, not_src),
                    (&not_dsts, not_dst),
                ] {
                    *(node_ndarray.t.uget_mut([i])) = node;
                }
            });

        (
            srcs.t.to_owned(),
            dsts.t.to_owned(),
            not_srcs.t.to_owned(),
            not_dsts.t.to_owned(),
        )
    }

    #[pyo3(text_signature = "($self, random_state, batch_size)")]
    /// Returns n-ple with terms used for training a siamese network.
    ///
    /// Parameters
    /// -------------
    /// random_state: int
    ///     Random state to reproduce sampling
    /// batch_size: int
    ///     The maximal size of the batch to generate,
    ///
    fn get_siamese_mini_batch_with_edge_types(
        &self,
        random_state: u64,
        batch_size: usize,
    ) -> (
        Py<PyArray1<NodeT>>,
        Py<PyArray1<NodeT>>,
        Py<PyArray1<NodeT>>,
        Py<PyArray1<NodeT>>,
        Py<PyArray1<EdgeTypeT>>,
    ) {
        let gil = pyo3::Python::acquire_gil();

        let srcs = ThreadDataRaceAware {
            t: unsafe { PyArray1::new(gil.python(), [batch_size], false) },
        };

        let dsts = ThreadDataRaceAware {
            t: unsafe { PyArray1::new(gil.python(), [batch_size], false) },
        };

        let not_srcs = ThreadDataRaceAware {
            t: unsafe { PyArray1::new(gil.python(), [batch_size], false) },
        };

        let not_dsts = ThreadDataRaceAware {
            t: unsafe { PyArray1::new(gil.python(), [batch_size], false) },
        };

        let edge_type_ids = ThreadDataRaceAware {
            t: PyArray1::zeros(gil.python(), [batch_size], false),
        };

        let edge_types_offset = if self.inner.has_unknown_edge_types().unwrap_or(false) {
            1
        } else {
            0
        };

        self.inner
            .par_iter_siamese_mini_batch_with_edge_types(random_state, batch_size)
            .enumerate()
            .for_each(|(i, (_, src, dst, not_src, not_dst, edge_type))| unsafe {
                for (node_ndarray, node) in [
                    (&srcs, src),
                    (&dsts, dst),
                    (&not_srcs, not_src),
                    (&not_dsts, not_dst),
                ] {
                    *(node_ndarray.t.uget_mut([i])) = node;
                }
                if let Some(edge_type) = edge_type {
                    *(edge_type_ids.t.uget_mut([i])) = edge_type + edge_types_offset;
                }
            });

        (
            srcs.t.to_owned(),
            dsts.t.to_owned(),
            not_srcs.t.to_owned(),
            not_dsts.t.to_owned(),
            edge_type_ids.t.to_owned(),
        )
    }

    #[pyo3(
        text_signature = "($self, idx, graph, batch_size, return_node_types, return_edge_types, return_edge_metrics)"
    )]
    /// Returns n-ple for running edge predictions on a graph, sampling the graph properties from the graph used in training.
    ///
    /// Parameters
    /// -------------
    /// idx: int
    ///     The index of the mini-batch to generate.
    /// graph: Graph
    ///     The graph from which to extract the edges to return.
    /// batch_size: int
    ///     Maximal size of the mini-batch. The last batch may be smaller.
    /// return_node_types: bool
    ///     Whether to return the node types properties of the nodes.
    /// return_edge_types: bool
    ///     Whether to return the edge types properties of the edges.
    /// return_edge_metrics: bool
    ///     Whether to return the edge metrics that can be computed on generic edges (existing or not) using the training graph (the self).
    ///
    /// Raises
    /// -------------
    /// ValueError
    ///     If the current graph does not have node types and node types are requested.
    /// ValueError
    ///     If the current graph does not have edge types and edge types are requested.
    fn get_edge_prediction_chunk_mini_batch(
        &self,
        idx: usize,
        graph: &Graph,
        batch_size: usize,
        return_node_types: bool,
        return_edge_types: bool,
        return_edge_metrics: bool,
    ) -> PyResult<(
        Py<PyArray1<NodeT>>,
        Option<Py<PyArray2<NodeTypeT>>>,
        Py<PyArray1<NodeT>>,
        Option<Py<PyArray2<NodeTypeT>>>,
        Option<Py<PyArray1<EdgeTypeT>>>,
        Option<Py<PyArray2<f32>>>,
    )> {
        let gil = pyo3::Python::acquire_gil();

        let par_iter = pe!(self.inner.get_edge_prediction_chunk_mini_batch(
            idx,
            &graph.inner,
            batch_size,
            return_node_types,
            return_edge_types,
            return_edge_metrics,
        ))?;

        let actual_batch_size = par_iter.len();

        let srcs = ThreadDataRaceAware {
            t: unsafe { PyArray1::new(gil.python(), [actual_batch_size], false) },
        };
        let dsts = ThreadDataRaceAware {
            t: unsafe { PyArray1::new(gil.python(), [actual_batch_size], false) },
        };
        let (src_node_type_ids, dst_node_type_ids) = if return_node_types {
            let max_node_type_count = pe!(self.inner.get_maximum_multilabel_count())? as usize;
            (
                Some(ThreadDataRaceAware {
                    t: unsafe {
                        PyArray2::new(
                            gil.python(),
                            [actual_batch_size, max_node_type_count],
                            false,
                        )
                    },
                }),
                Some(ThreadDataRaceAware {
                    t: unsafe {
                        PyArray2::new(
                            gil.python(),
                            [actual_batch_size, max_node_type_count],
                            false,
                        )
                    },
                }),
            )
        } else {
            (None, None)
        };
        let edge_types = if return_edge_types {
            Some(ThreadDataRaceAware {
                t: PyArray1::zeros(gil.python(), [actual_batch_size], false),
            })
        } else {
            None
        };
        let edges_metrics = if return_edge_metrics {
            Some(ThreadDataRaceAware {
                t: unsafe {
                    PyArray2::new(
                        gil.python(),
                        [
                            actual_batch_size,
                            self.inner.get_number_of_available_edge_metrics(),
                        ],
                        false,
                    )
                },
            })
        } else {
            None
        };

        unsafe {
            par_iter.enumerate().for_each(
                |(i, (src, src_node_type, dst, dst_node_type, edge_type, edge_features))| {
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
                    if let Some(edge_types) = edge_types.as_ref() {
                        *(edge_types.t.uget_mut([i])) = edge_type.unwrap_or(0);
                    }
                    if let Some(edges_metrics) = edges_metrics.as_ref() {
                        edge_features.unwrap().iter().copied().enumerate().for_each(
                            |(j, metric)| {
                                *(edges_metrics.t.uget_mut([i, j])) = metric;
                            },
                        );
                    }
                },
            );
        }

        Ok((
            srcs.t.to_owned(),
            src_node_type_ids.map(|x| x.t.to_owned()),
            dsts.t.to_owned(),
            dst_node_type_ids.map(|x| x.t.to_owned()),
            edge_types.map(|x| x.t.to_owned()),
            edges_metrics.map(|x| x.t.to_owned()),
        ))
    }
}
