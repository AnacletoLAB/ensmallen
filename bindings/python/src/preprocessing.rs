use super::*;
use graph::{
    cooccurence_matrix as rust_cooccurence_matrix, word2vec as rust_word2vec, NodeT, NodeTypeT,
};
use numpy::{PyArray, PyArray1, PyArray2};
use pyo3::wrap_pyfunction;
use rayon::prelude::*;
use std::sync::atomic::{AtomicUsize, Ordering};
use types::ThreadSafe;

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
    let (contexts, words): (Vec<Vec<NodeT>>, Vec<NodeT>) =
        pe!(rust_word2vec(sequences.into_par_iter(), window_size))?.unzip();
    let gil = pyo3::Python::acquire_gil();
    Ok((
        to_nparray_2d!(gil, contexts, NodeT),
        to_ndarray_1d!(gil, words, NodeT),
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
        extract_value!(kwargs, "verbose", bool).unwrap_or(true),
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
impl EnsmallenGraph {
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

        let (number_of_elements, iter) = pe!(self.graph.cooccurence_matrix(
            &parameters,
            extract_value!(kwargs, "window_size", usize).unwrap_or(3),
            extract_value!(kwargs, "verbose", bool).unwrap_or(true),
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

        let iter = pe!(self.graph.node2vec(&parameters, batch_size, window_size))?;

        let elements_per_batch = (walk_length as usize - window_size * 2)
            * batch_size as usize
            * parameters.get_iterations() as usize;

        let contexts = ThreadSafe {
            t: PyArray2::new(gil.python(), [elements_per_batch, window_size * 2], false),
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
    #[text_signature = "($self, node_ids, *, random_state, include_central_node, offset, max_neighbours)"]
    /// Return iterator over neighbours for the given node IDs, optionally including given the node IDs, and node type.
    ///
    /// This method is meant to be used to predict node labels using the NoLaN model.
    ///
    /// If you need to predict the node label of a node, not during training,
    /// use `max_neighbours=None`.
    ///
    /// Parameters
    /// -----------------------------
    /// - node_ids: List[int],
    ///     The node ID to retrieve neighbours for.
    /// - random_state: int = 42,
    ///     The random state to use to extract the neighbours.
    /// - include_central_node: bool = True,
    ///     Whether to include the node ID in the returned iterator.
    /// - offset: int = 1,
    ///     Offset for padding porposes.
    /// - max_neighbours: int = None,
    ///     Number of maximum neighbours to consider.
    ///
    /// Returns
    /// -----------------------------
    /// Tuple with input nodes and output node types.
    ///
    fn get_node_label_prediction_tuple_from_node_ids(
        &self,
        node_ids: Vec<NodeT>,
        py_kwargs: Option<&PyDict>,
    ) -> PyResult<(Py<PyArray2<NodeT>>, Py<PyArray2<NodeTypeT>>)> {
        let gil = pyo3::Python::acquire_gil();

        // First we normalize the kwargs so that we always at least
        // an empty dictionary
        let kwargs = normalize_kwargs!(py_kwargs, gil.python());

        // Then we validate the provided kwargs, that is we verify
        // that the valid kwarg names are provided.
        pe!(validate_kwargs(
            kwargs,
            &[
                "random_state",
                "include_central_node",
                "offset",
                "max_neighbours",
            ],
        ))?;

        // We check that the provided list is not empty.
        if node_ids.is_empty() {
            return pe!(Err("Given list of node IDs is empty!".to_string()));
        }

        // We get the maximum degree among the provided nodes.
        // We will use this as a maximal value for the size of the batch.
        // This way if there are no high-degree centrality nodes in this
        // batch we do not allocate extra memory for no reason.
        // We can always unwrap this value because we have requested
        // just above that the list cannot be empty.
        let mut max_degree = node_ids
            .iter()
            .map(|node_id| self.graph.get_node_degree_from_node_id(*node_id).unwrap())
            .max()
            .unwrap();

        // We get the number of the requested nodes IDs.
        let nodes_number = node_ids.len();
        // Extract the maximum neighbours parameter.
        let max_neighbours = extract_value!(kwargs, "max_neighbours", NodeT);
        // And whether to include or not the central node.
        let include_central_node =
            extract_value!(kwargs, "include_central_node", bool).unwrap_or(true);

        // If the maximum neighbours was provided, we set the minimum value
        // between max degree and maximum neighbours as the size of the
        // bacth vector to return.
        if let Some(mn) = &max_neighbours {
            max_degree = std::cmp::min(max_degree, *mn);
        }

        // If the batch includes also the central node we need to add an
        // additional column for it.
        if include_central_node {
            max_degree += 1;
        }

        // We retrieve the batch iterator.
        let iter = pe!(self.graph.get_node_label_prediction_tuple_from_node_ids(
            node_ids,
            extract_value!(kwargs, "random_state", u64).unwrap_or(42),
            include_central_node,
            extract_value!(kwargs, "offset", NodeT).unwrap_or(1),
            max_neighbours,
        ))?;

        // We create the vector of zeros where to allocate the neighbours
        // This vector has `nodes_number` rows, that is the number of required
        // node IDs, and `max_degree` rows, that is the maximum degree.
        let neighbours = PyArray2::zeros(gil.python(), [nodes_number, max_degree as usize], false);
        // We create the vector of zeros for the one-hot encoded labels.
        // This is also used for the multi-label case.
        // This vector has the same number of rows as the previous vector,
        // that is the number of requested node IDs, while the number
        // of columns is the number of node types in the graph.
        let labels = PyArray2::zeros(
            gil.python(),
            [
                nodes_number,
                pe!(self.graph.get_node_types_number())? as usize,
            ],
            false,
        );

        // We iterate over the batch.
        iter.enumerate()
            .for_each(|(i, (neighbours_iterator, node_types))| {
                neighbours_iterator
                    .enumerate()
                    .for_each(|(j, node_id)| unsafe {
                        *neighbours.uget_mut([i, j]) = node_id;
                    });
                if let Some(nts) = node_types {
                    nts.into_iter().for_each(|label| unsafe {
                        *labels.uget_mut([i, label as usize]) = 1;
                    });
                }
            });

        Ok((neighbours.to_owned(), labels.to_owned()))
    }

    #[args(py_kwargs = "**")]
    #[text_signature = "($self, idx, batch_size, negative_samples, avoid_false_negatives, maximal_sampling_attempts, graph_to_avoid)"]
    /// Returns ids for a link prediction training batch.
    ///
    /// Parameters
    /// -----------------------------
    /// idx:int,
    ///     Index corresponding to batch to be rendered.
    /// batch_size: int,
    ///     The batch size to use.
    /// negative_samples: float = 1.0,
    ///     Factor of negatives to use in every batch.
    ///     For example, with a batch size of 128 and negative_samples equal
    ///     to 1.0, there will be 64 positives and 64 negatives.
    /// avoid_false_negatives: bool = False,
    ///     Whether to filter out false negatives.
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
    /// Triple with source and destination nodes and labels.
    ///
    fn link_prediction_ids(
        &self,
        idx: u64,
        batch_size: usize,
        py_kwargs: Option<&PyDict>,
    ) -> PyResult<(Py<PyArray1<NodeT>>, Py<PyArray1<NodeT>>, Py<PyArray1<bool>>)> {
        let gil = pyo3::Python::acquire_gil();
        let kwargs = normalize_kwargs!(py_kwargs, gil.python());

        pe!(validate_kwargs(
            kwargs,
            &[
                "negative_samples",
                "avoid_false_negatives",
                "maximal_sampling_attempts",
                "graph_to_avoid",
            ],
        ))?;
        let graph_to_avoid = extract_value!(kwargs, "graph_to_avoid", EnsmallenGraph);
        let maybe_graph = match &graph_to_avoid {
            Some(g) => Some(&g.graph),
            None => None,
        };

        let iter = pe!(self.graph.link_prediction_ids(
            idx,
            batch_size,
            extract_value!(kwargs, "negative_samples", f64).unwrap_or(1.0),
            extract_value!(kwargs, "avoid_false_negatives", bool).unwrap_or(false),
            extract_value!(kwargs, "maximal_sampling_attempts", usize).unwrap_or(100),
            &maybe_graph,
        ))?;

        let srcs = ThreadSafe {
            t: PyArray1::new(gil.python(), [batch_size], false),
        };
        let dsts = ThreadSafe {
            t: PyArray1::new(gil.python(), [batch_size], false),
        };
        let labels = ThreadSafe {
            t: PyArray1::new(gil.python(), [batch_size], false),
        };

        unsafe {
            iter.for_each(|(i, src, dst, label)| {
                *(dsts.t.uget_mut([i])) = src;
                *(srcs.t.uget_mut([i])) = dst;
                *(labels.t.uget_mut([i])) = label;
            });
        }

        Ok((srcs.t.to_owned(), dsts.t.to_owned(), labels.t.to_owned()))
    }

    #[args(py_kwargs = "**")]
    #[text_signature = "($self, idx, batch_size, normalize, negative_samples, avoid_false_negatives, maximal_sampling_attempts, graph_to_avoid)"]
    /// Returns
    ///
    ///
    /// Parameters
    /// -----------------------------
    /// idx:int,
    ///     Index corresponding to batch to be rendered.
    /// batch_size: int,
    ///     The batch size to use.
    /// normalize: bool=True,
    ///      Divide the degrees by the max, this way the values are in [0, 1].
    /// negative_samples: float = 1.0,
    ///     Factor of negatives to use in every batch.
    ///     For example, with a batch size of 128 and negative_samples equal
    ///     to 1.0, there will be 64 positives and 64 negatives.
    /// avoid_false_negatives: bool = False,
    ///     Whether to filter out false negatives.
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
    fn link_prediction_degrees(
        &self,
        idx: u64,
        batch_size: usize,
        py_kwargs: Option<&PyDict>,
    ) -> PyResult<(Py<PyArray1<f64>>, Py<PyArray1<f64>>, Py<PyArray1<bool>>)> {
        let gil = pyo3::Python::acquire_gil();
        let kwargs = normalize_kwargs!(py_kwargs, gil.python());

        pe!(validate_kwargs(
            kwargs,
            &[
                "normalize",
                "negative_samples",
                "avoid_false_negatives",
                "maximal_sampling_attempts",
                "graph_to_avoid",
            ],
        ))?;
        let graph_to_avoid = extract_value!(kwargs, "graph_to_avoid", EnsmallenGraph);
        let maybe_graph = match &graph_to_avoid {
            Some(g) => Some(&g.graph),
            None => None,
        };

        let iter = pe!(self.graph.link_prediction_degrees(
            idx,
            batch_size,
            extract_value!(kwargs, "normalize", bool).unwrap_or(true),
            extract_value!(kwargs, "negative_samples", f64).unwrap_or(1.0),
            extract_value!(kwargs, "avoid_false_negatives", bool).unwrap_or(false),
            extract_value!(kwargs, "maximal_sampling_attempts", usize).unwrap_or(100),
            &maybe_graph,
        ))?;

        let srcs = ThreadSafe {
            t: PyArray1::new(gil.python(), [batch_size], false),
        };
        let dsts = ThreadSafe {
            t: PyArray1::new(gil.python(), [batch_size], false),
        };
        let labels = ThreadSafe {
            t: PyArray1::new(gil.python(), [batch_size], false),
        };

        unsafe {
            iter.for_each(|(i, src, dst, label)| {
                *(dsts.t.uget_mut([i])) = src;
                *(srcs.t.uget_mut([i])) = dst;
                *(labels.t.uget_mut([i])) = label;
            });
        }

        Ok((srcs.t.to_owned(), dsts.t.to_owned(), labels.t.to_owned()))
    }
}
