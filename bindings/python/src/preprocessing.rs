use super::*;
use graph::{cooccurence_matrix as rust_cooccurence_matrix, word2vec as rust_word2vec, NodeT};
use numpy::{PyArray, PyArray1, PyArray2};
use pyo3::wrap_pyfunction;
use rayon::prelude::*;
use std::sync::atomic::{AtomicUsize, Ordering};
use thread_safe::ThreadSafe;

#[pymodule]
fn preprocessing(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(word2vec))?;
    m.add_wrapped(wrap_pyfunction!(cooccurence_matrix))?;
    Ok(())
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
fn word2vec(sequences: Vec<Vec<NodeT>>, window_size: usize) -> PyResult<(PyContexts, PyWords)> {
    let _ = ctrlc::set_handler(|| std::process::exit(2));
    let (contexts, words) = pyex!(rust_word2vec(sequences.into_par_iter(), window_size))?.unzip();
    let gil = pyo3::Python::acquire_gil();
    Ok((
        to_nparray_2d!(gil, contexts, NodeT),
        to_nparray_1d!(gil, words, NodeT),
    ))
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
///     Wethever to show the progress bars.
///     The default behaviour is false.
///     
fn cooccurence_matrix(
    sequences: Vec<Vec<NodeT>>,
    py_kwargs: Option<&PyDict>,
) -> PyResult<(PyWords, PyWords, PyFrequencies)> {
    let _ = ctrlc::set_handler(|| std::process::exit(2));
    let gil = pyo3::Python::acquire_gil();
    let kwargs = normalize_kwargs!(py_kwargs, gil.python());
    pyex!(validate_kwargs(
        kwargs,
        ["window_size", "verbose"]
            .iter()
            .map(|x| x.to_string())
            .collect(),
    ))?;
    let len = sequences.len();
    let (words, contexts, frequencies) = pyex!(rust_cooccurence_matrix(
        sequences.into_par_iter(),
        pyex!(extract_value!(kwargs, "window_size", usize))?.unwrap_or(3),
        len,
        pyex!(extract_value!(kwargs, "verbose", bool))?.unwrap_or(true),
    ))?;

    Ok((
        to_nparray_1d!(gil, words, NodeT),
        to_nparray_1d!(gil, contexts, NodeT),
        to_nparray_1d!(gil, frequencies, f64),
    ))
}

#[pymethods]
impl EnsmallenGraph {
    
    #[args(py_kwargs = "**")]
    #[text_signature = "($self, length, *, window_size, iterations, return_weight, explore_weight, change_edge_type_weight, change_node_type_weight, dense_node_mapping, max_neighbours, random_state, verbose)"]
    /// Return cooccurence matrix-based triples of words, contexts and frequencies.
    ///
    /// Parameters
    /// ---------------------
    /// length: int,
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
    ///     Wethever to show or not the loading bar of the walks.
    ///
    /// Returns
    /// ----------------------------
    /// Triple with integer vectors of words and contexts and max-min normalized frequencies.
    ///
    fn cooccurence_matrix(
        &self,
        length: NodeT,
        py_kwargs: Option<&PyDict>,
    ) -> PyResult<(PyWords, PyWords, PyFrequencies)> {
        let gil = pyo3::Python::acquire_gil();
        let kwargs = normalize_kwargs!(py_kwargs, gil.python());

        pyex!(validate_kwargs(
            kwargs,
            build_walk_parameters_list(&["window_size", "verbose"]),
        ))?;

        let parameters = pyex!(self.build_walk_parameters(length, kwargs))?;

        let (words, contexts, frequencies) = pyex!(self.graph.cooccurence_matrix(
            &parameters,
            pyex!(extract_value!(kwargs, "window_size", usize))?.unwrap_or(3),
            pyex!(extract_value!(kwargs, "verbose", bool))?.unwrap_or(true),
        ))?;

        Ok((
            to_nparray_1d!(gil, words, NodeT),
            to_nparray_1d!(gil, contexts, NodeT),
            to_nparray_1d!(gil, frequencies, f64),
        ))
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
        walk_length: NodeT,
        window_size: usize,
        py_kwargs: Option<&PyDict>,
    ) -> PyResult<(PyContexts, PyWords)> {
        let gil = pyo3::Python::acquire_gil();
        let kwargs = normalize_kwargs!(py_kwargs, gil.python());
        pyex!(validate_kwargs(
            kwargs,
            build_walk_parameters_list(&["window_size"])
        ))?;
        let parameters = pyex!(self.build_walk_parameters(walk_length, kwargs))?;

        let iter = pyex!(self.graph.node2vec(&parameters, batch_size, window_size))?;
        let elements_per_batch = (walk_length as usize - window_size * 2 - 1)
            * batch_size as usize
            * parameters.get_iterations() as usize;

        let contexts = ThreadSafe {
            t: PyArray2::new(
                gil.python(),
                [elements_per_batch, window_size * 2],
                false,
            ),
        };
        let words = ThreadSafe {
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
    #[text_signature = "($self, idx, batch_size, method, negative_samples, avoid_false_negatives, maximal_sampling_attempts, graph_to_avoid)"]
    /// Returns
    ///
    ///
    /// Parameters
    /// -----------------------------
    /// idx:int,
    ///     Index corresponding to batch to be rendered.
    /// batch_size: int,
    ///     The batch size to use.
    /// method: str,
    ///     The method to use for the edge embedding.
    /// negative_samples: float = 1.0,
    ///     Factor of negatives to use in every batch.
    ///     For example, with a batch size of 128 and negative_samples equal
    ///     to 1.0, there will be 64 positives and 64 negatives.
    /// avoid_false_negatives: bool = False,
    ///     Wether to filter out false negatives.
    ///     By default False.
    ///     Enabling this will slow down the batch generation while (likely) not
    ///     introducing any significant gain to the model performance.
    /// maximal_sampling_attempts: usize = 100,
    ///     Number of attempts to execute to sample the negative edges.
    /// graph_to_avoid: EnsmallenGraph = None,
    ///     Graph to avoid when generating the links.
    ///     This can be the validation component of the graph, for example.
    ///
    /// Returns
    /// -----------------------------
    /// Tuple containing training and validation graphs.
    ///
    fn link_prediction(
        &self,
        idx: u64,
        batch_size: usize,
        method: &str,
        py_kwargs: Option<&PyDict>,
    ) -> PyResult<(Py<PyArray2<f64>>, Py<PyArray1<bool>>)> {
        let gil = pyo3::Python::acquire_gil();
        let kwargs = normalize_kwargs!(py_kwargs, gil.python());

        pyex!(validate_kwargs(
            kwargs,
            [
                "negative_samples",
                "avoid_false_negatives",
                "maximal_sampling_attempts",
                "graph_to_avoid",
            ]
            .iter()
            .map(|x| x.to_string())
            .collect(),
        ))?;
        let graph_to_avoid = pyex!(extract_value!(kwargs, "graph_to_avoid", EnsmallenGraph))?;
        let maybe_graph = match &graph_to_avoid {
            Some(g) => Some(&g.graph),
            None => None,
        };

        let iter = pyex!(self.graph.link_prediction(
            idx,
            batch_size,
            method,
            pyex!(extract_value!(kwargs, "negative_samples", f64))?.unwrap_or(1.0),
            pyex!(extract_value!(kwargs, "avoid_false_negatives", bool))?.unwrap_or(false),
            pyex!(extract_value!(kwargs, "maximal_sampling_attempts", usize))?.unwrap_or(100),
            &maybe_graph
        ))?;

        let embedding_size = pyex!(self.graph.get_embedding_size())?;
        let edge_vector_len = match method {
            "Concatenate" => embedding_size*2,
            _ => embedding_size
        };

        let edges = ThreadSafe {
            t: PyArray2::new(
                gil.python(),
                [batch_size, edge_vector_len],
                false,
            ),
        };
        let labels = ThreadSafe {
            t: PyArray1::new(gil.python(), [batch_size], false),
        };
        unsafe {
            iter.for_each(|(i, edge_embedding, label)| {
                edge_embedding.iter().enumerate().for_each(|(j, v)| {
                    *(edges.t.uget_mut([i, j])) = *v;
                });
                *(labels.t.uget_mut([i])) = label;
            });
        }
        Ok((edges.t.to_owned(), labels.t.to_owned()))
    }
}
