use super::*;
use graph::{cooccurence_matrix as rust_cooccurence_matrix, word2vec as rust_word2vec, NodeT};
use numpy::{PyArray, PyArray1, PyArray2};
use pyo3::exceptions;
use pyo3::prelude::*;
use pyo3::types::PyDict;

#[pyfunction(py_kwargs = "**")]
#[text_signature = "(seed, sequences, *, window_size, shuffle)"]
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
/// shuffle: bool,
///     Wethever to shuffle the vectors on return.
/// seed: int,
///     The seed for reproducibility.
///
fn word2vec(
    seed: usize,
    sequences: Vec<Vec<usize>>,
    py_kwargs: Option<&PyDict>,
) -> PyResult<(Py<PyArray2<NodeT>>, Py<PyArray1<NodeT>>)> {
    let gil = pyo3::Python::acquire_gil();
    let kwargs = normalize_kwargs!(py_kwargs, gil.python());
    validate_kwargs(kwargs, ["window_size", "shuffle"].iter().map(|x| x.to_string()).collect())?;
    let (contexts, words) = pyex!(rust_word2vec(
        sequences,
        extract_value!(kwargs, "window_size", usize),
        extract_value!(kwargs, "shuffle", bool),
        seed,
    ))?;
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
    sequences: Vec<Vec<usize>>,
    py_kwargs: Option<&PyDict>,
) -> PyResult<(Py<PyArray1<NodeT>>, Py<PyArray1<NodeT>>, Py<PyArray1<f64>>)> {
    let gil = pyo3::Python::acquire_gil();
    let kwargs = normalize_kwargs!(py_kwargs, gil.python());
    validate_kwargs(kwargs, ["window_size", "verbose"].iter().map(|x| x.to_string()).collect())?;
    let (words, contexts, frequencies) = pyex!(rust_cooccurence_matrix(
        sequences,
        extract_value!(kwargs, "window_size", usize),
        extract_value!(kwargs, "verbose", bool),
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
    #[text_signature = "($self, length, *, window_size, iterations, min_length, return_weight, explore_weight, change_edge_type_weight, change_node_type_weight, dense_nodes_mapping, seed, verbose)"]
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
    /// min_length: int = 0,
    ///     Minimal length of the random walk. Will filter out smaller
    ///     random walks.
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
    /// dense_nodes_mapping: Dict[int, int],
    ///     Mapping to use for converting sparse walk space into a dense space.
    ///     This object can be created using the method available from graph
    ///     called `get_dense_nodes_mapping` that returns a mapping from
    ///     the non trap nodes (those from where a walk could start) and
    ///     maps these nodes into a dense range of values.
    /// seed: int,
    ///     Seed to use to reproduce the walks.
    /// verbose: int = True,
    ///     Wethever to show or not the loading bar of the walks.
    ///
    /// Returns
    /// ----------------------------
    /// Triple with integer vectors of words and contexts and max-min normalized frequencies.
    ///
    fn cooccurence_matrix(
        &self,
        length: usize,
        py_kwargs: Option<&PyDict>,
    ) -> PyResult<(Py<PyArray1<NodeT>>, Py<PyArray1<NodeT>>, Py<PyArray1<f64>>)> {
        let gil = pyo3::Python::acquire_gil();
        let kwargs = normalize_kwargs!(py_kwargs, gil.python());

        validate_kwargs(
            kwargs,
            build_walk_parameters_list(&["window_size", "verbose"]),
        )?;

        let parameters = pyex!(self.build_walk_parameters(
            length,
            0,
            self.graph.get_not_trap_nodes_number(),
            kwargs
        ))?;

        let (words, contexts, frequencies) = pyex!(self.graph.cooccurence_matrix(
            &parameters,
            extract_value!(kwargs, "window_size", usize),
            extract_value!(kwargs, "verbose", bool),
        ))?;

        Ok((
            to_nparray_1d!(gil, words, NodeT),
            to_nparray_1d!(gil, contexts, NodeT),
            to_nparray_1d!(gil, frequencies, f64),
        ))
    }

    #[args(py_kwargs = "**")]
    #[text_signature = "($self, idx, batch_size, length, *, iterations, window_size, shuffle, iterations, min_length, return_weight, explore_weight, change_edge_type_weight, change_node_type_weight, dense_nodes_mapping, seed)"]
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
    /// idx: int,
    ///     Identifier of the batch to generate.
    /// batch_size:
    ///     Number of walks to include within this batch.
    ///     Consider that the walks may be filtered by the given min_length.
    ///     In some pathological cases, this might leed to an empty batch.
    ///     These cases include graphs with particularly high number of traps.
    ///     Consider using the method graph.report() to verify if this might
    ///     apply to your use case.
    /// length: int,
    ///     Maximal length of the random walk.
    ///     On graphs without traps, all walks have this length.
    /// iterations: int = 1,
    ///     Number of iterations for each node.
    /// window_size: int = 4,
    ///     Size of the window for local contexts.
    /// min_length: int = 0,
    ///     Minimal length of the random walk. Will filter out smaller
    ///     random walks.
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
    /// dense_nodes_mapping: Dict[int, int],
    ///     Mapping to use for converting sparse walk space into a dense space.
    ///     This object can be created using the method available from graph
    ///     called `get_dense_nodes_mapping` that returns a mapping from
    ///     the non trap nodes (those from where a walk could start) and
    ///     maps these nodes into a dense range of values.
    /// seed: int,
    ///     Seed to use to reproduce the walks.
    ///
    /// Returns
    /// ----------------------------
    /// Tuple with vector of integer with contexts and words.
    fn node2vec(
        &self,
        idx: usize,
        batch_size: usize,
        length: usize,
        py_kwargs: Option<&PyDict>,
    ) -> PyResult<(Py<PyArray2<NodeT>>, Py<PyArray1<NodeT>>)> {
        let gil = pyo3::Python::acquire_gil();
        let kwargs = normalize_kwargs!(py_kwargs, gil.python());

        validate_kwargs(
            kwargs,
            build_walk_parameters_list(&["window_size", "shuffle"]),
        )?;

        let (start_node, end_node) = self.get_batch_range(idx, batch_size);

        let parameters = pyex!(self.build_walk_parameters(length, start_node, end_node, kwargs))?;

        let (contexts, words) = pyex!(self.graph.node2vec(
            &parameters,
            extract_value!(kwargs, "window_size", usize),
            extract_value!(kwargs, "shuffle", bool),
            idx,
        ))?;

        Ok((
            to_nparray_2d!(gil, contexts, NodeT),
            to_nparray_1d!(gil, words, NodeT),
        ))
    }

    #[args(py_kwargs = "**")]
    #[text_signature = "($self, idx, batch_size, negative_samples, graph_to_avoid, avoid_self_loops)"]
    /// Returns
    ///
    ///
    /// Parameters
    /// -----------------------------
    /// idx:int,
    ///     Index corresponding to batch to be rendered.
    /// batch_size: int = 2**10,
    ///     The batch size to use.
    /// negative_samples: float = 1.0,
    ///     Factor of negatives to use in every batch.
    ///     For example, with a batch size of 128 and negative_samples equal
    ///     to 1.0, there will be 64 positives and 64 negatives.
    /// graph_to_avoid: EnsmallenGraph = None,
    ///     Graph to avoid when generating the links.
    ///     This can be the validation component of the graph, for example.
    /// avoid_self_loops: bool = False,
    ///     If the result should be filtered of self loops.
    ///
    /// Returns
    /// -----------------------------
    /// Tuple containing training and validation graphs.
    ///
    fn link_prediction(
        &self,
        idx: u64,
        batch_size: usize,
        py_kwargs: Option<&PyDict>,
    ) -> PyResult<(Py<PyArray2<NodeT>>, Py<PyArray1<u8>>)> {
        let gil = pyo3::Python::acquire_gil();
        let kwargs = normalize_kwargs!(py_kwargs, gil.python());

        validate_kwargs(
            kwargs,
            ["graph_to_avoid", "negative_samples", "avoid_self_loops"]
                .iter()
                .map(|x| x.to_string())
                .collect(),
        )?;
        let graph_to_avoid = extract_value!(kwargs, "graph_to_avoid", EnsmallenGraph);
        
        let (edges, labels) = pyex!(self.graph.link_prediction(
            idx,
            batch_size,
            extract_value!(kwargs, "negative_samples", f64),
            match &graph_to_avoid {
                Some(g) => Some(&g.graph),
                None => None
            },
            extract_value!(kwargs, "avoid_self_loops", bool),
        ))?;

        Ok((
            to_nparray_2d!(gil, edges, NodeT),
            to_nparray_1d!(gil, labels, u8),
        ))
    }
}
