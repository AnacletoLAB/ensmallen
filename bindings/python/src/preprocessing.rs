extern crate edit_distance;
use edit_distance::edit_distance;
use graph::{
    binary_skipgrams as rust_binary_skipgrams, cooccurence_matrix as rust_cooccurence_matrix,
    word2vec as rust_word2vec, EdgeT, EdgeTypeT, Graph, NodeT, NodeTypeT, ParamsT,
    SingleWalkParameters, WalkWeights, WalksParameters, WeightT, NodeFileWriter, EdgeFileWriter
};
use numpy::{PyArray, PyArray1, PyArray2};
use pyo3::class::basic::CompareOp;
use pyo3::class::basic::PyObjectProtocol;
use pyo3::class::number::PyNumberProtocol;
use pyo3::exceptions;
use pyo3::prelude::*;
use pyo3::types::PyDict;
use pyo3::{wrap_pyfunction, wrap_pymodule};
use std::collections::{HashMap, HashSet};

macro_rules! python_exception {
    ($value: expr, $msg: expr) => {
        match $value {
            Ok(v) => Ok(v),
            Err(_) => Err(PyErr::new::<exceptions::ValueError, _>($msg)),
        }
    };
}

macro_rules! to_python_exception {
    ($value: expr) => {
        match $value {
            Ok(v) => Ok(v),
            Err(e) => Err(PyErr::new::<exceptions::ValueError, _>(e)),
        }
    };
}

macro_rules! extract_value {
    ($kwargs: ident, $key: literal, $_type: ty) => {
        match $kwargs.get_item($key) {
            None => None,
            Some(v) => {
                if v.get_type().name() == "NoneType" {
                    None
                } else {
                    let extracted = v.extract::<$_type>();
                    Some(python_exception!(
                        extracted,
                        format!(
                            "The value passed for {} cannot be casted from {} to {}.",
                            $key,
                            v.get_type().name(),
                            stringify!($_type)
                        )
                    )?)
                }
            }
        }
    };
}

macro_rules! to_nparray_1d {
    ($gil: expr, $value: expr, $_type: ty) => {
        python_exception!(
            PyArray::from_vec($gil.python(), $value).cast::<$_type>(false),
            format!(
                "The given array cannot be casted to {}.",
                stringify!($_type)
            )
        )?
        .to_owned()
    };
}

macro_rules! to_nparray_2d {
    ($gil: expr, $value: expr, $_type: ty) => {
        python_exception!(
            python_exception!(
                PyArray::from_vec2($gil.python(), &$value),
                "The given value cannot be casted to a 2d numpy array."
            )?
            .cast::<$_type>(false),
            format!(
                "The given 2d array cannot be casted to {}.",
                stringify!($_type)
            )
        )?
        .to_owned()
    };
}

fn validate_kwargs(kwargs: &PyDict, columns: &[&str]) -> PyResult<()> {
    let mut keys: HashSet<&str> = kwargs
        .keys()
        .iter()
        .map(|v| v.extract::<&str>().unwrap())
        .collect();
    let columns: HashSet<&str> = columns.iter().cloned().collect();
    to_python_exception!(if keys.is_subset(&columns) {
        return Ok(());
    } else {
        for k in &columns {
            keys.remove(k);
        }
        let mut err_msg = String::new();
        for k in &keys {
            let (distance, column) = columns
                .iter()
                .map(|col| (edit_distance(k, col), col))
                .min_by_key(|x| x.0)
                .unwrap();

            if distance <= 2 {
                err_msg = format!(
                        "The passed argument {} is not a valid one.\n Did you mean {} ?\nThe available ones are: \n{:?}",
                        k, column, columns
                    );
                break;
            }
        }
        if err_msg.is_empty() {
            err_msg = format!(
                "The following arguments are not valid keyword arguments for this function. \n{:?}\n the available ones are: \n{:?}",
                keys, columns
            );
        }
        Err(err_msg)
    })
}

#[pymodule]
fn ensmallen_graph(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<EnsmallenGraph>()?;
    m.add_wrapped(wrap_pymodule!(preprocessing))?;
    env_logger::init();
    Ok(())
}

#[pymodule]
fn preprocessing(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(binary_skipgrams))?;
    m.add_wrapped(wrap_pyfunction!(word2vec))?;
    m.add_wrapped(wrap_pyfunction!(cooccurence_matrix))?;
    Ok(())
}

#[pyfunction(py_kwargs = "**")]
#[text_signature = "(seed, sequences, vocabulary_size, *, window_size, negative_samples, shuffle)"]
/// Returns skipgram batches for a given integers sequences.
///
/// Arguments
/// --------------
/// seed: int,
///     The seed to use for reproducibility.
/// sequences: List[List[int]],
///     Sequences of values to be converted.
/// vocabulary_size: usize,
///     Number of distrinct terms present in vocabulary.
/// window_size: int = 4,
///     Size of the window. By default is 4.
/// negative_samples: float = 1.0,
///     Factor of the negative samples to extract.
/// shuffle: bool = True,
///     Wethever to shuffle or not the words and contexts.
///
fn binary_skipgrams(
    seed: usize,
    sequences: Vec<Vec<usize>>,
    vocabulary_size: usize,
    py_kwargs: Option<&PyDict>,
) -> PyResult<((Py<PyArray1<f64>>, Py<PyArray1<f64>>), Py<PyArray1<f64>>)> {
    let batch = to_python_exception!(if let Some(kwargs) = &py_kwargs {
        validate_kwargs(kwargs, &["window_size", "negative_samples", "shuffle"])?;
        rust_binary_skipgrams(
            sequences,
            vocabulary_size,
            extract_value!(kwargs, "window_size", usize),
            extract_value!(kwargs, "negative_samples", f64),
            extract_value!(kwargs, "shuffle", bool),
            seed,
        )
    } else {
        rust_binary_skipgrams(sequences, vocabulary_size, None, None, None, seed)
    })?;

    let gil = pyo3::Python::acquire_gil();
    Ok((
        (
            to_nparray_1d!(gil, (batch.0).0, f64),
            to_nparray_1d!(gil, (batch.0).1, f64),
        ),
        to_nparray_1d!(gil, batch.1, f64),
    ))
}

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
) -> PyResult<(Py<PyArray2<f64>>, Py<PyArray1<f64>>)> {
    match if let Some(kwargs) = &py_kwargs {
        validate_kwargs(kwargs, &["window_size", "shuffle"])?;
        rust_word2vec(
            sequences,
            extract_value!(kwargs, "window_size", usize),
            extract_value!(kwargs, "shuffle", bool),
            seed,
        )
    } else {
        rust_word2vec(sequences, None, None, seed)
    } {
        Ok(batch) => {
            let gil = pyo3::Python::acquire_gil();
            Ok((
                to_nparray_2d!(gil, batch.0, f64),
                to_nparray_1d!(gil, batch.1, f64),
            ))
        }
        Err(e) => Err(PyErr::new::<exceptions::ValueError, _>(e)),
    }
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
) -> PyResult<(Py<PyArray1<f64>>, Py<PyArray1<f64>>, Py<PyArray1<f64>>)> {
    match if let Some(kwargs) = &py_kwargs {
        validate_kwargs(kwargs, &["window_size", "verbose"])?;
        rust_cooccurence_matrix(
            sequences,
            extract_value!(kwargs, "window_size", usize),
            extract_value!(kwargs, "verbose", bool),
        )
    } else {
        rust_cooccurence_matrix(sequences, None, None)
    } {
        Ok(csr) => {
            let gil = pyo3::Python::acquire_gil();
            Ok((
                to_nparray_1d!(gil, csr.0, f64),
                to_nparray_1d!(gil, csr.1, f64),
                to_nparray_1d!(gil, csr.2, f64),
            ))
        }
        Err(e) => Err(PyErr::new::<exceptions::ValueError, _>(e)),
    }
}

#[pyclass]
#[derive(Clone, PartialEq)]
#[text_signature = "(sources, destinations, *, nodes_mapping, nodes_reverse_mapping, node_types, node_types_mapping, node_types_reverse_mapping, edge_types, edge_types_mapping, edge_types_reverse_mapping, weights, force_conversion_to_undirected)"]
/// Return new EnsmallenGraph.
///
/// sources: List[int],
///     The list of source nodes.
/// destinations: List[int],
///     The list of destination nodes.
/// nodes_mapping: Dict[str, int] = None,
///     The dictionary with mappEnsmallenGraph,
///     List of the node types, must be as long as the nodes mapping.
/// node_types_mapping: Dict[str, int] = None,
///     Mapping between the node types names and their IDs.
/// node_types_reverse_mapping: List[str] = None,
///     Reverse mapping between numeric node Type IDs and their name.
/// edge_types: List[int] = None,
///     List of the egde types, must be as long as the egdes mapping.
/// edge_types_mapping: Dict[str, int] = None,
///     Mapping between the edge types names and their IDs.
/// edge_types_reverse_mapping: List[str] = None,
///     Reverse mapping between numeric egde Type IDs and their name.
/// weights: List[float] = None,
///     List of the weight for each edge.
/// force_conversion_to_undirected: bool = False,
///     Wethever to force the conversion from directed graph to undirected
///     when there are bidirectional directed edges in the given graph.
///
struct EnsmallenGraph {
    graph: Graph,
}

/// Build WalkWeights object from provided kwargs
///
/// # Arguments
///
/// * py_kwargs: Option<&PyDict> - The kwargs provided by the user.
fn build_walk_weights(py_kwargs: Option<&PyDict>) -> PyResult<WalkWeights> {
    let mut weights = WalkWeights::default();
    if let Some(kwargs) = &py_kwargs {
        weights = to_python_exception!(weights.set_return_weight(extract_value!(
            kwargs,
            "return_weight",
            ParamsT
        )))?;
        weights = to_python_exception!(weights.set_explore_weight(extract_value!(
            kwargs,
            "explore_weight",
            ParamsT
        )))?;
        weights = to_python_exception!(weights.set_change_edge_type_weight(extract_value!(
            kwargs,
            "change_edge_type_weight",
            ParamsT
        )))?;
        weights = to_python_exception!(weights.set_change_node_type_weight(extract_value!(
            kwargs,
            "change_node_type_weight",
            ParamsT
        )))?;
    }
    Ok(weights)
}

/// Build SingleWalkParameters object from provided kwargs
///
/// # Arguments
///
/// * length: usize - the length of the walks.
/// * py_kwargs: Option<&PyDict> - The kwargs provided by the user.
fn build_single_walk_parameters(
    length: usize,
    py_kwargs: Option<&PyDict>,
) -> PyResult<SingleWalkParameters> {
    to_python_exception!(SingleWalkParameters::new(
        length,
        build_walk_weights(py_kwargs)?,
    ))
}

/// Build WalksParameters object from provided kwargs
///
/// # Arguments
///
/// * length: usize - the length of the walks.
/// * py_kwargs: Option<&PyDict> - The kwargs provided by the user.
fn build_walk_parameters(
    length: usize,
    start_node: NodeT,
    end_node: NodeT,
    py_kwargs: Option<&PyDict>,
    validate: bool,
) -> PyResult<WalksParameters> {
    let mut walks_parameters = to_python_exception!(WalksParameters::new(
        build_single_walk_parameters(length, py_kwargs)?,
        start_node,
        end_node,
    ))?;
    if let Some(kwargs) = &py_kwargs {
        if validate {
            validate_kwargs(
                kwargs,
                &[
                    "iterations",
                    "min_length",
                    "dense_nodes_mapping",
                    "return_weight",
                    "explore_weight",
                    "change_edge_type_weight",
                    "change_node_type_weight",
                    "verbose",
                    "seed",
                ],
            )?;
        }
        walks_parameters = to_python_exception!(walks_parameters.set_iterations(extract_value!(
            kwargs,
            "iterations",
            usize
        )))?;
        walks_parameters = walks_parameters.set_verbose(extract_value!(kwargs, "verbose", bool));
        walks_parameters = walks_parameters.set_seed(extract_value!(kwargs, "seed", usize));
        walks_parameters = to_python_exception!(walks_parameters.set_min_length(extract_value!(
            kwargs,
            "min_length",
            usize
        )))?;
        walks_parameters = walks_parameters.set_dense_nodes_mapping(
            extract_value!(kwargs, "dense_nodes_mapping", HashMap<NodeT, NodeT>),
        );
    }
    Ok(walks_parameters)
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
    ) -> PyResult<(Py<PyArray1<f64>>, Py<PyArray1<f64>>, Py<PyArray1<f64>>)> {
        if let Some(kwargs) = &py_kwargs {
            validate_kwargs(
                kwargs,
                &[
                    "window_size",
                    "verbose",
                    "iterations",
                    "min_length",
                    "dense_nodes_mapping",
                    "return_weight",
                    "explore_weight",
                    "change_edge_type_weight",
                    "change_node_type_weight",
                    "seed",
                    "verbose",
                ],
            )?;
        }
        match build_walk_parameters(
            length,
            0,
            self.graph.get_not_trap_nodes_number(),
            py_kwargs,
            false,
        ) {
            Ok(wp) => {
                let csr = if let Some(kwargs) = &py_kwargs {
                    self.graph.cooccurence_matrix(
                        &wp,
                        extract_value!(kwargs, "window_size", usize),
                        extract_value!(kwargs, "verbose", bool),
                    )
                } else {
                    self.graph.cooccurence_matrix(&wp, None, None)
                };

                let gil = pyo3::Python::acquire_gil();
                match csr {
                    Ok(csr) => Ok((
                        to_nparray_1d!(gil, csr.0, f64),
                        to_nparray_1d!(gil, csr.1, f64),
                        to_nparray_1d!(gil, csr.2, f64),
                    )),
                    Err(e) => Err(PyErr::new::<exceptions::ValueError, _>(e)),
                }
            }
            Err(e) => Err(PyErr::new::<exceptions::ValueError, _>(e)),
        }
    }

    #[args(py_kwargs = "**")]
    #[text_signature = "($self, idx, batch_size, length, *, iterations, window_size, negative_samples, shuffle, iterations, min_length, return_weight, explore_weight, change_edge_type_weight, change_node_type_weight, dense_nodes_mapping, seed)"]
    /// Return batch triple for training BinarySkipGram model.
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
    /// negative_samples: float = 1.0,
    ///     Factor of negative samples to use.
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
    /// Triple with vector of integer with words, contexts and labels.
    ///
    fn binary_skipgrams(
        &self,
        idx: usize,
        batch_size: usize,
        length: usize,
        py_kwargs: Option<&PyDict>,
    ) -> PyResult<((Py<PyArray1<f64>>, Py<PyArray1<f64>>), Py<PyArray1<f64>>)> {
        let (start_node, end_node) = self.get_batch_range(idx, batch_size);
        if let Some(kwargs) = &py_kwargs {
            validate_kwargs(
                kwargs,
                &[
                    "window_size",
                    "shuffle",
                    "negative_samples",
                    "iterations",
                    "min_length",
                    "dense_nodes_mapping",
                    "return_weight",
                    "explore_weight",
                    "change_edge_type_weight",
                    "change_node_type_weight",
                    "seed",
                ],
            )?;
        }
        match build_walk_parameters(length, start_node, end_node, py_kwargs, false) {
            Ok(wp) => {
                let batch = if let Some(kwargs) = &py_kwargs {
                    self.graph.binary_skipgrams(
                        idx,
                        &wp,
                        extract_value!(kwargs, "window_size", usize),
                        extract_value!(kwargs, "negative_samples", f64),
                        extract_value!(kwargs, "shuffle", bool),
                    )
                } else {
                    self.graph.binary_skipgrams(idx, &wp, None, None, None)
                };

                let gil = pyo3::Python::acquire_gil();
                match batch {
                    Ok(batch) => Ok((
                        (
                            to_nparray_1d!(gil, (batch.0).0, f64),
                            to_nparray_1d!(gil, (batch.0).1, f64),
                        ),
                        to_nparray_1d!(gil, batch.1, f64),
                    )),
                    Err(e) => Err(PyErr::new::<exceptions::ValueError, _>(e)),
                }
            }
            Err(e) => Err(PyErr::new::<exceptions::ValueError, _>(e)),
        }
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
    ) -> PyResult<(Py<PyArray2<f64>>, Py<PyArray1<f64>>)> {
        if let Some(kwargs) = &py_kwargs {
            validate_kwargs(
                kwargs,
                &[
                    "window_size",
                    "shuffle",
                    "iterations",
                    "min_length",
                    "dense_nodes_mapping",
                    "return_weight",
                    "explore_weight",
                    "change_edge_type_weight",
                    "change_node_type_weight",
                    "seed",
                ],
            )?;
        }
        let (start_node, end_node) = self.get_batch_range(idx, batch_size);
        match build_walk_parameters(length, start_node, end_node, py_kwargs, false) {
            Ok(wp) => {
                let batch = if let Some(kwargs) = &py_kwargs {
                    self.graph.node2vec(
                        &wp,
                        extract_value!(kwargs, "window_size", usize),
                        extract_value!(kwargs, "shuffle", bool),
                        idx,
                    )
                } else {
                    self.graph.node2vec(&wp, None, None, idx)
                };

                match batch {
                    Ok(batch) => {
                        let gil = pyo3::Python::acquire_gil();
                        Ok((
                            to_nparray_2d!(gil, batch.0, f64),
                            to_nparray_1d!(gil, batch.1, f64),
                        ))
                    }
                    Err(e) => Err(PyErr::new::<exceptions::ValueError, _>(e)),
                }
            }
            Err(e) => Err(PyErr::new::<exceptions::ValueError, _>(e)),
        }
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
        let results = if let Some(kwargs) = py_kwargs {
            validate_kwargs(
                kwargs,
                &["graph_to_avoid", "negative_samples", "avoid_self_loops"],
            )?;
            let egraph = extract_value!(kwargs, "graph_to_avoid", EnsmallenGraph);
            self.graph.link_prediction(
                idx,
                batch_size,
                extract_value!(kwargs, "negative_samples", f64),
                if let Some(eg) = &egraph {
                    Some(&eg.graph)
                } else {
                    None
                },
                extract_value!(kwargs, "avoid_self_loops", bool),
            )
        } else {
            self.graph
                .link_prediction(idx, batch_size, None, None, None)
        };

        let gil = pyo3::Python::acquire_gil();
        match results {
            Ok((edges, labels)) => Ok((
                to_nparray_2d!(gil, edges, NodeT),
                to_nparray_1d!(gil, labels, u8),
            )),
            Err(e) => Err(PyErr::new::<exceptions::ValueError, _>(e)),
        }
    }
}

