extern crate edit_distance;
use edit_distance::edit_distance;
use graph::{
    binary_skipgrams as rust_binary_skipgrams, cooccurence_matrix as rust_cooccurence_matrix,
    word2vec as rust_word2vec, EdgeT, EdgeTypeT, FromCsvBuilder, Graph, NodeT, NodeTypeT, ParamsT,
    SingleWalkParameters, WalkWeights, WalksParameters, WeightT,
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
        match $kwargs.get_item($key){
            None => None,
            Some(v) => {
                Some(python_exception!(
                    v.extract::<$_type>(), 
                    format!(
                        "The value passed for {} cannot be casted from {} to {}.",
                        $key, v.get_type().name(), stringify!($_type)
                    )
                )?)
            }
        }
    };
}

macro_rules! to_nparray_1d {
    ($gil: expr, $value: expr, $_type: ty) => {
        python_exception!(
                PyArray::from_vec($gil.python(), $value).cast::<$_type>(false),
            format!("The given array cannot be casted to {}.", stringify!($_type))
        )?.to_owned()
    };
}

macro_rules! to_nparray_2d {
    ($gil: expr, $value: expr, $_type: ty) => {
        python_exception!(
            python_exception!(
                PyArray::from_vec2($gil.python(), &$value),
                "The given value cannot be casted to a 2d numpy array."
            )?.cast::<$_type>(false),
            format!("The given 2d array cannot be casted to {}.", stringify!($_type))
        )?.to_owned()
    };
}

fn validate_kwargs(kwargs: &PyDict, columns: &[&str]) -> PyResult<()>{
    let mut keys: HashSet<&str> = kwargs.keys().iter().map(
        |v| v.extract::<&str>().unwrap()
    ).collect();
    let columns: HashSet<&str> = columns.iter().cloned().collect();
    to_python_exception!(if keys.is_subset(&columns) {
        return Ok(());
    } else {
        for k in &columns {
            keys.remove(k);
        }
        let mut err_msg = String::new();
        for k in &keys {
            let (distance, column) = columns.iter().map(
                |col|
                    (edit_distance(k, col), col)
            ).min_by_key(|x| x.0).unwrap();

            if distance <= 2 {
                err_msg = format!(
                        "The passed argument {} is not a valid one.\n Did you mean {} ?\nThe available ones are: \n{:?}",
                        k, column, columns
                    );
                break
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
        weights = to_python_exception!(
            weights.set_return_weight(extract_value!(kwargs, "return_weight", ParamsT))
        )?;
        weights = to_python_exception!(
            weights.set_explore_weight(extract_value!(kwargs, "explore_weight", ParamsT))
        )?;
        weights = to_python_exception!(
            weights.set_change_edge_type_weight(extract_value!(kwargs, "change_edge_type_weight", ParamsT))
        )?;
        weights = to_python_exception!(
            weights.set_change_node_type_weight(extract_value!(kwargs, "change_node_type_weight", ParamsT))
        )?;
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
) ->  PyResult<SingleWalkParameters> {
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
) -> PyResult<WalksParameters> {
    let mut weights = to_python_exception!(WalksParameters::new(
        build_single_walk_parameters(length, py_kwargs)?,
        start_node,
        end_node,
    ))?;
    if let Some(kwargs) = &py_kwargs {
        validate_kwargs(kwargs,&[
            "iterations", "min_length", "dense_nodes_mapping", 
            "return_weight", "explore_weight", "change_edge_type_weight", 
            "change_node_type_weight", "verbose"
            ])?;
        weights = to_python_exception!(weights.set_iterations(extract_value!(kwargs, "iterations", usize)))?;
        weights = to_python_exception!(weights.set_min_length(extract_value!(kwargs, "min_length", usize)))?;
        weights = weights.set_dense_nodes_mapping(extract_value!(kwargs, "dense_nodes_mapping", HashMap<NodeT, NodeT>));
    }
    Ok(weights)
}

#[pymethods]
impl EnsmallenGraph {
    #[new]
    #[args(py_kwargs = "**")]
    fn new(
        sources: Vec<NodeT>,
        destinations: Vec<NodeT>,
        directed: bool,
        py_kwargs: Option<&PyDict>,
    ) -> PyResult<Self> {
        let mut graph = Graph::builder(sources, destinations, directed);

        if py_kwargs.is_none() {
            return match graph.build(None) {
                Ok(g) => Ok(EnsmallenGraph { graph: g }),
                Err(e) => Err(PyErr::new::<exceptions::ValueError, _>(e)),
            };
        }
        let kwargs = py_kwargs.unwrap();
        validate_kwargs(kwargs, &[
            "weights", "nodes_mapping", "nodes_reverse_mapping",
            "node_types", "node_types_mapping", "node_types_reverse_mapping",
            "edge_types", "edge_types_mapping", "edge_types_reverse_mapping",
            "force_conversion_to_undirected"
            ])?;

        let weights = extract_value!(kwargs, "weights", Vec<WeightT>);

        if let Some(w) = weights {
            graph = graph.add_weights(w);
        }

        let nodes_mapping = extract_value!(kwargs, "nodes_mapping", HashMap<String, NodeT>);
        let nodes_reverse_mapping = extract_value!(kwargs, "nodes_reverse_mapping", Vec<String>);
        // check passage consistency
        if !((nodes_mapping.is_some() && nodes_reverse_mapping.is_some())
            || (nodes_mapping.is_none() && nodes_reverse_mapping.is_none()))
        {
            return Err(PyErr::new::<exceptions::ValueError, _>(concat!(
                "You must either pass both nodes_mapping, and nodes_reverse_mapping \n",
                "Or none of them."
            )));
        }
        if let Some(nm) = nodes_mapping {
            if let Some(nrm) = nodes_reverse_mapping {
                graph = graph.add_nodes(
                    nm,
                    nrm,
                    extract_value!(kwargs, "node_types", Vec<NodeTypeT>),
                    extract_value!(kwargs, "node_types_mapping", HashMap<String, NodeTypeT>),
                    extract_value!(kwargs, "node_types_reverse_mapping", Vec<String>),
                );
            }
        }
       

        let edge_types =  extract_value!(kwargs, "edge_types", Vec<EdgeTypeT>);
        let edge_types_mapping =  extract_value!(kwargs, "edge_types_mapping", HashMap<String, EdgeTypeT>);
        let edge_types_reverse_mapping =  extract_value!(kwargs, "edge_types_reverse_mapping", Vec<String>);
        // check passage consistency
        if !((edge_types.is_some()
            && edge_types_mapping.is_some()
            && edge_types_reverse_mapping.is_some())
            || (edge_types.is_none()
                && edge_types_mapping.is_none()
                && edge_types_reverse_mapping.is_none()))
        {
            return Err(PyErr::new::<exceptions::ValueError, _>(concat!(
                "You must either pass all edge_types, edge_types_mapping, and edge_types_reverse_mapping \n",
                "Or none of them."
            )));
        }

        if let Some(et) = edge_types {
            if let Some(etm) = edge_types_mapping {
                if let Some(etrm) = edge_types_reverse_mapping {
                    graph = graph.add_edge_types(et, etm, etrm);
                }
            }
        }

        match graph.build(
            kwargs
                .get_item("force_conversion_to_undirected")
                .map(|val| val.extract::<bool>().unwrap()),
        ) {
            Ok(g) => Ok(EnsmallenGraph { graph: g }),
            Err(e) => Err(PyErr::new::<exceptions::ValueError, _>(e)),
        }
    }

    #[staticmethod]
    #[args(py_kwargs = "**")]
    #[text_signature = "(edge_path, sources_column, destinations_column, directed, *, edge_types_column, default_edge_type, weights_column, default_weight, node_path, nodes_column, node_types_column, default_node_type, edge_sep, node_sep, ignore_duplicated_edges, ignore_duplicated_nodes, force_conversion_to_undirected)"]
    /// Build the graph from a csv (or tsv) in Rust.
    ///
    /// Parameters
    /// ---------------------
    /// edge_path:str,
    ///     Path to CSV file from where to load the edge data.
    /// sources_column:str,
    ///     Column name of the edge file where the source nodes are listed.
    /// destinations_column:str,
    ///     Column name of the edge file where the destination nodes are listed.
    /// directed:bool,
    ///     Boolean representing if given graph is directed or undirected.
    /// edge_types_column:str,
    ///     Column name of the edge file where the edge types are listed.
    /// default_edge_type:str,
    ///     The default edge type to use when an empty edge type is found in the
    ///     provided edge file. It is REQUIRED when passing an edge types column.
    /// weights_column:str,
    ///     Column name of the edge file where the edge weights are listed.
    /// default_weight:float,
    ///     The default weight to use when an empty weight is found in the
    ///     provided edge file. It is REQUIRED when passing a weights column.
    /// node_path:str,
    ///     Path to CSV file from where to load the node data.
    /// nodes_column:str,
    ///     Column name of the node file where the nodes names are listed.
    /// default_node_type:str,
    ///     The default node type to use when an empty node type is found in the
    ///     provided node file. It is REQUIRED when passing an node types column.
    /// node_types_column:str,
    ///     Column name of the node file where the node types are listed.
    /// edge_sep:str="\t",
    ///     Separator to use for the edge files.
    /// node_sep:str="\t",
    ///     Separator to use for the node files.
    /// ignore_duplicated_edges:bool=False,
    ///     Wethever to ignore duplicated edges or to raise an exception.
    ///     The duplication includes the edge type, if provided, so for example
    ///     an edge from A to B of type 1 is different from an edge A to B
    ///     of type 2.
    ///     The default behaviour is to raise an exception.
    /// ignore_duplicated_nodes:bool=False,
    ///     Wethever to ignore duplicated nodes or to raise an exception.
    ///     The default behaviour is to raise an exception.
    /// force_conversion_to_undirected:bool=False,
    ///     Wethever to force conversion of a directed graph to an undirected one.
    ///     This will remove bidirectional edges between two nodes that have the
    ///     same type before doing the conversion.
    ///     When false (default) and a forced conversion is required, an exception
    ///     will be raised.
    ///
    fn from_csv(
        edge_path: &str,
        sources_column: &str,
        destinations_column: &str,
        directed: bool,
        py_kwargs: Option<&PyDict>,
    ) -> PyResult<Self> {
        if py_kwargs.is_none() {
            let mut result = match FromCsvBuilder::new(
                edge_path,
                sources_column,
                destinations_column,
                directed,
                None,
            ) {
                Ok(g) => Ok(g),
                Err(e) => Err(PyErr::new::<exceptions::ValueError, _>(e)),
            }?;
            return match result.build() {
                Ok(g) => Ok(EnsmallenGraph { graph: g }),
                Err(e) => Err(PyErr::new::<exceptions::ValueError, _>(e)),
            };
        }
        let kwargs = py_kwargs.unwrap();
        validate_kwargs(kwargs, &[
            "edge_sep", "weights_column", "default_weight",
            "node_path", "nodes_column", "node_types_column",
            "default_node_type", "node_sep", "ignore_duplicated_nodes",
            "edge_types_column", "default_edge_type", "ignore_duplicated_edges",
            "force_conversion_to_undirected", "validate_input_data", 
            ])?;

        let mut result = match FromCsvBuilder::new(
            edge_path,
            sources_column,
            destinations_column,
            directed,
            extract_value!(kwargs, "edge_sep", &str),
        ) {
            Ok(g) => Ok(g),
            Err(e) => Err(PyErr::new::<exceptions::ValueError, _>(e)),
        }?;

        let weights_column = extract_value!(kwargs, "weights_column", &str);
        if let Some(wc) = weights_column {
            result = result.set_weights(
                wc,
                extract_value!(kwargs, "default_weight", WeightT),
            );
        }
        let node_path = extract_value!(kwargs, "node_path", &str);
        let nodes_column = extract_value!(kwargs, "nodes_column", &str);
        let node_types_column = extract_value!(kwargs, "node_types_column", &str);
        let default_node_type = extract_value!(kwargs, "default_node_type", &str);
        let node_sep = extract_value!(kwargs, "node_sep", &str);
        let ignore_duplicated_nodes = extract_value!(kwargs, "ignore_duplicated_nodes", bool);
        // check passage consistency
        if !((node_path.is_some() && nodes_column.is_some() && node_types_column.is_some())
            || (node_path.is_none() && nodes_column.is_none() && node_types_column.is_none()))
        {
            return Err(PyErr::new::<exceptions::ValueError, _>(concat!(
                "You must either pass all node_types, nodes_column, and node_types_column \n",
                "Or none of them."
            )));
        }
        if node_path.is_some() {
            result = match result.load_nodes_csv(
                node_path.unwrap(),
                nodes_column.unwrap(),
                node_types_column.unwrap(),
                default_node_type,
                node_sep,
                ignore_duplicated_nodes,
            ) {
                Ok(g) => Ok(g),
                Err(e) => Err(PyErr::new::<exceptions::ValueError, _>(e)),
            }?;
        }

        let edge_types_column = extract_value!(kwargs, "edge_types_column", &str);
        if let Some(etc) = edge_types_column {
            result =
                result.set_edge_types(etc, extract_value!(kwargs, "default_edge_type", &str));
        }

        let ignore_duplicated_edges = extract_value!(kwargs, "ignore_duplicated_edges", bool);
        if let Some(ide) = ignore_duplicated_edges {
            if ide {
                result = result.set_ignore_duplicated_edges();
            }
        }
        let force_conversion_to_undirected = extract_value!(kwargs, "force_conversion_to_undirected", bool);
        if let Some(fctu) = force_conversion_to_undirected {
            if fctu {
                result = result.set_force_conversion_to_undirected();
            }
        }

        match result.build() {
            Ok(g) => Ok(EnsmallenGraph { graph: g }),
            Err(e) => Err(PyErr::new::<exceptions::ValueError, _>(e)),
        }
    }

    #[args(py_kwargs = "**")]
    #[text_signature = "($self, nodes_path, separator, nodes_column, node_types_column)"]
    /// Save the nodes to a loadable csv / tsv.
    /// In this method we use csv and tsv interchangably.
    ///
    /// Parameters
    /// ---------------------
    /// nodes_path: str,
    ///     Where to save the nodes csv.
    /// separator: str = "\t",
    ///     The separator to use for the csv or tsv file.
    /// nodes_column: str = "id",
    ///     The name of the column with the names of the nodes.
    /// node_types_column: str = "category",
    ///     The name of the column with the types of the nodes.
    ///
    fn to_nodes_csv(&self, 
        nodes_path: String,
        py_kwargs: Option<&PyDict>
    ) -> PyResult<()> {
        match if let Some(kwargs) = &py_kwargs{
            validate_kwargs(kwargs, &["separator", "nodes_column", "node_types_column"])?;
            self.graph.to_nodes_csv(&nodes_path, 
                extract_value!(kwargs, "separator", &str),
                extract_value!(kwargs, "nodes_column", &str),
                extract_value!(kwargs, "node_types_column", &str),
            )
        } else {
            self.graph.to_nodes_csv(&nodes_path, None, None, None)
        } {
            Ok(g) => Ok(g),
            Err(_) => Err(PyErr::new::<exceptions::ValueError, _>("Generic file error, check that the given path is valid.")),
        }
    }

    #[args(py_kwargs = "**")]
    #[text_signature = "($self, edges_path, separator, sources_column, destinations_column, edge_types_column, weights_column)"]
    /// Save the edges to a loadable csv / tsv.
    /// In this method we use csv and tsv interchangably.
    ///
    /// Parameters
    /// ---------------------
    /// edges_path: str,
    ///     Where to save the nodes csv.
    /// separator: str = "\t",
    ///     The separator to use for the csv or tsv file.
    /// sources_column: str = "subject",
    ///     The name of the column with the names of the sources nodes.
    /// destinations_column: str = "object",
    ///     The name of the column with the names of the destinations nodes.
    /// edge_types_column: str = "edge_label",
    ///     The name of the column with the types of the edges.
    /// weights_column: str = "weight"
    ///     The name of the column with the weight of the edges.
    ///
    fn to_edges_csv(&self, 
        edges_path: String,
        py_kwargs: Option<&PyDict>
    ) -> PyResult<()> {
        python_exception!(if let Some(kwargs) = &py_kwargs{
                validate_kwargs(kwargs, &[
                    "separator", "sources_column", "destinations_column",
                    "edge_types_column", "weights_column"
                    ])?;
                self.graph.to_edges_csv(&edges_path, 
                    extract_value!(kwargs, "separator", &str),
                    extract_value!(kwargs, "sources_column", &str),
                    extract_value!(kwargs, "destinations_column", &str),
                    extract_value!(kwargs, "edge_types_column", &str),
                    extract_value!(kwargs, "weights_column", &str),
                )
            } else {
                self.graph.to_edges_csv(&edges_path, None, None, None, None, None)
            }, "Generic file error, check that the given path is valid."
        )  
    }

    #[text_signature = "($self, node_id)"]
    /// Return the id of the node type of the node.
    ///
    /// Parameters
    /// ---------------------
    /// node_id: int,
    ///     Numeric ID of the node.
    ///
    /// Returns
    /// ---------------------
    /// Id of the node type of the node.
    fn get_node_type_id(&self, node_id: NodeT) -> PyResult<NodeTypeT> {
        to_python_exception!(self.graph.get_node_type_id(node_id))
    }

    #[text_signature = "($self, edge_id)"]
    /// Return the id of the edge type of the edge.
    ///
    /// Parameters
    /// ---------------------
    /// edge_id: int,
    ///     Numeric ID of the edge.
    ///
    /// Returns
    /// ---------------------
    /// Id of the edge type of the edge.
    fn get_edge_type_id(&self, edge_id: EdgeT) -> PyResult<EdgeTypeT> {
        to_python_exception!(self.graph.get_edge_type_id(edge_id))
    }

    #[text_signature = "($self, src, dst)"]
    /// Return random walks done on the graph using Rust.
    ///
    /// Parameters
    /// ---------------------
    /// edge_id: int,
    ///     Numeric ID of the edge.
    ///
    /// Returns
    /// ---------------------
    /// Return the id of the edge type of the edge.
    fn get_edge_id(&self, src: NodeT, dst: NodeT) -> PyResult<EdgeT> {
        to_python_exception!(self.graph.get_edge_id(src, dst))
    }

    #[text_signature = "($self)"]
    /// Return the count of how many time an edge type appears.
    fn get_edge_type_counts(&self) -> PyResult<HashMap<EdgeTypeT, usize>> {
        to_python_exception!(self.graph.get_edge_type_counts())
    }

    #[text_signature = "($self)"]
    /// Return the count of how many time an node type appears.
    fn get_node_type_counts(&self) -> PyResult<HashMap<EdgeTypeT, usize>> {
        to_python_exception!(self.graph.get_node_type_counts())
    }

    #[text_signature = "($self, k)"]
    /// Return vectors with the nodes and node types within the top k most common.
    ///
    /// Parameters
    /// --------------------------
    /// k: int,
    ///     Number of common node types to return.
    ///
    /// Returns
    /// --------------------------
    /// Tuple with node IDs and node types within k most common node types.
    fn get_top_k_nodes_by_node_type(
        &self,
        k: usize,
    ) -> PyResult<(Py<PyArray1<NodeT>>, Py<PyArray1<NodeTypeT>>)> {
        match self.graph.get_top_k_nodes_by_node_type(k) {
            Ok((nodes, node_types)) => {
                let gil = pyo3::Python::acquire_gil();
                Ok((
                    to_nparray_1d!(gil, nodes, NodeT),
                    to_nparray_1d!(gil, node_types, NodeTypeT),
                ))
            }
            Err(e) => Err(PyErr::new::<exceptions::ValueError, _>(e)),
        }
    }

    #[text_signature = "($self)"]
    /// Return vectors with the edges and ed ge types within the top k most common.
    ///
    /// Parameters
    /// --------------------------
    /// k: int,
    ///     Number of common edge types to return.
    ///
    /// Returns
    /// --------------------------
    /// Tuple with edge IDs and edge types within k most common edge types.
    fn get_top_k_edges_by_edge_type(
        &self,
        k: usize,
    ) -> PyResult<(Py<PyArray1<NodeT>>, Py<PyArray1<NodeTypeT>>)> {
        match self.graph.get_top_k_edges_by_edge_type(k) {
            Ok((edges, edge_types)) => {
                let gil = pyo3::Python::acquire_gil();
                Ok((
                    to_nparray_1d!(gil, edges, EdgeT),
                    to_nparray_1d!(gil, edge_types, EdgeTypeT),
                ))
            }
            Err(e) => Err(PyErr::new::<exceptions::ValueError, _>(e)),
        }
    }

    /// Return start node and end node for given batch.
    fn get_batch_range(&self, idx: usize, batch_size: usize) -> (usize, usize) {
        let (start_node, end_node) = (idx * batch_size, (idx + 1) * batch_size);
        (
            start_node,
            if end_node > self.get_not_trap_nodes_number() {
                self.get_not_trap_nodes_number()
            } else {
                end_node
            },
        )
    }

    #[args(py_kwargs = "**")]
    #[text_signature = "($self, length, *, iterations, min_length, return_weight, explore_weight, change_edge_type_weight, change_node_type_weight, dense_nodes_mapping, verbose)"]
    /// Return random walks done on the graph using Rust.
    ///
    /// Parameters
    /// ---------------------
    /// length: int,
    ///     Maximal length of the random walk.
    ///     On graphs without traps, all walks have this length.
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
    /// verbose: int = True,
    ///     Wethever to show or not the loading bar of the walks.
    ///
    /// Returns
    /// ----------------------------
    /// List of list of walks containing the numeric IDs of nodes.
    ///
    fn walk(&self, length: usize, py_kwargs: Option<&PyDict>) -> PyResult<Vec<Vec<NodeT>>> {
        match build_walk_parameters(length, 0, self.graph.get_not_trap_nodes_number(), py_kwargs) {
            Ok(walk_parameters) => match self.graph.walk(&walk_parameters) {
                Ok(w) => Ok(w),
                Err(e) => Err(PyErr::new::<exceptions::ValueError, _>(e)),
            },
            Err(e) => Err(PyErr::new::<exceptions::ValueError, _>(e)),
        }
    }

    #[args(py_kwargs = "**")]
    #[text_signature = "($self, length, *, window_size, iterations, min_length, return_weight, explore_weight, change_edge_type_weight, change_node_type_weight, dense_nodes_mapping, verbose)"]
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
        match build_walk_parameters(length, 0, self.graph.get_not_trap_nodes_number(), py_kwargs) {
            Ok(wp) => {
                let csr = if let Some(kwargs) = &py_kwargs {
                    validate_kwargs(kwargs, &["window_size", "verbose"])?;
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
    #[text_signature = "($self, idx, batch_size, length, *, iterations, window_size, negative_samples, shuffle, iterations, min_length, return_weight, explore_weight, change_edge_type_weight, change_node_type_weight, dense_nodes_mapping)"]
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
        match build_walk_parameters(length, start_node, end_node, py_kwargs) {
            Ok(wp) => {
                let batch = if let Some(kwargs) = &py_kwargs {
                    validate_kwargs(kwargs, &["window_size", "negative_samples", "shuffle"])?;
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
    #[text_signature = "($self, idx, batch_size, length, *, iterations, window_size, shuffle, iterations, min_length, return_weight, explore_weight, change_edge_type_weight, change_node_type_weight, dense_nodes_mapping)"]
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
        let (start_node, end_node) = self.get_batch_range(idx, batch_size);
        match build_walk_parameters(length, start_node, end_node, py_kwargs) {
            Ok(wp) => {
                let batch = if let Some(kwargs) = &py_kwargs {
                    validate_kwargs(kwargs, &["window_size", "shuffle", 
                        "iterations", "min_length", "dense_nodes_mapping", 
                        "return_weight", "explore_weight", "change_edge_type_weight", 
                        "change_node_type_weight", "verbose"
                    ])?;
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

    #[getter]
    fn sources(&self) -> PyResult<Py<PyArray1<NodeT>>> {
        let gil = pyo3::Python::acquire_gil();
        Ok(to_nparray_1d!(gil, self.graph.sources().clone(), NodeT))
    }

    #[getter]
    fn destinations(&self) -> PyResult<Py<PyArray1<NodeT>>> {
        let gil = pyo3::Python::acquire_gil();
        Ok(to_nparray_1d!(gil, self.graph.destinations().clone(), NodeT))
    }

    #[getter]
    fn nodes_mapping(&self) -> HashMap<String, NodeT> {
        self.graph
            .nodes_mapping()
            .iter()
            .map(|(k, v)| (k.clone(), *v))
            .collect()
    }

    #[getter]
    fn nodes_reverse_mapping(&self) -> Vec<String> {
        self.graph.nodes_reverse_mapping().clone()
    }

    #[getter]
    fn unique_edges(&self) -> HashMap<(NodeT, NodeT), EdgeT> {
        self.graph
            .unique_edges()
            .iter()
            .map(|(k, v)| (*k, *v))
            .collect()
    }

    #[getter]
    fn outbounds(&self) -> Vec<EdgeT> {
        self.graph.outbounds().clone()
    }

    #[getter]
    fn weights(&self) -> Option<Vec<WeightT>> {
        self.graph.weights().clone()
    }

    #[getter]
    fn node_types(&self) -> Option<Vec<NodeTypeT>> {
        self.graph.node_types().clone()
    }

    #[getter]
    fn node_types_mapping(&self) -> Option<HashMap<String, NodeTypeT>> {
        match self.graph.node_types_mapping() {
            None => None,
            Some(g) => Some(g.iter().map(|(k, v)| (k.clone(), *v)).collect()),
        }
    }

    #[getter]
    fn node_types_reverse_mapping(&self) -> Option<Vec<String>> {
        self.graph.node_types_reverse_mapping().clone()
    }

    #[getter]
    fn edge_types(&self) -> Option<Vec<EdgeTypeT>> {
        self.graph.edge_types().clone()
    }

    #[getter]
    fn edge_types_mapping(&self) -> Option<HashMap<String, EdgeTypeT>> {
        match self.graph.edge_types_mapping() {
            None => None,
            Some(g) => Some(g.iter().map(|(k, v)| (k.clone(), *v)).collect()),
        }
    }

    #[getter]
    fn edge_types_reverse_mapping(&self) -> Option<Vec<String>> {
        self.graph.edge_types_reverse_mapping().clone()
    }

    #[text_signature = "($self, one, two)"]
    /// Return the Jaccard Index for the two given nodes.
    ///
    /// Parameters
    /// ---------------------
    /// one: int,
    ///     First node ID to use to compute Jaccard Index.
    /// two: int,
    ///     Second node ID to use to compute Jaccard Index.
    ///
    /// Returns
    /// ----------------------------
    /// Jaccard Index for the two given nodes.
    ///
    fn jaccard_index(&self, one: NodeT, two: NodeT) -> PyResult<f64> {
        to_python_exception!(self.graph.jaccard_index(one, two))
    }

    #[text_signature = "($self, one, two)"]
    /// Return the Adamic/Adar for the two given nodes.
    ///
    /// Parameters
    /// ---------------------
    /// one: int,
    ///     First node ID to use to compute Adamic/Adar.
    /// two: int,
    ///     Second node ID to use to compute Adamic/Adar.
    ///
    /// Returns
    /// ----------------------------
    /// Adamic/Adar for the two given nodes.
    ///
    fn adamic_adar_index(&self, one: NodeT, two: NodeT) -> PyResult<f64> {
        to_python_exception!(self.graph.adamic_adar_index(one, two))
    }

    #[text_signature = "($self, one, two)"]
    /// Return the Resource Allocation Index for the two given nodes.
    ///
    /// Parameters
    /// ---------------------
    /// one: int,
    ///     First node ID to use to compute Resource Allocation Index.
    /// two: int,
    ///     Second node ID to use to compute Resource Allocation Index.
    ///
    /// Returns
    /// ----------------------------
    /// Resource Allocation Index for the two given nodes.
    ///
    fn resource_allocation_index(&self, one: NodeT, two: NodeT) -> PyResult<f64> {
        to_python_exception!(self.graph.resource_allocation_index(one, two))
    }

    #[text_signature = "($self, one, two)"]
    /// Return the degrees product for the two given nodes.
    ///
    /// Parameters
    /// ---------------------
    /// one: int,
    ///     First node ID to use to compute degrees product.
    /// two: int,
    ///     Second node ID to use to compute degrees product.
    ///
    /// Returns
    /// ----------------------------
    /// degrees product for the two given nodes.
    ///
    fn degrees_product(&self, one: NodeT, two: NodeT) -> PyResult<usize> {
        to_python_exception!(self.graph.degrees_product(one, two))
    }

    #[text_signature = "(self)"]
    /// Return the traps rate of the graph.
    ///
    /// This feature is EXPERIMENTAL and still required proving.
    ///
    fn traps_rate(&self) -> f64 {
        self.graph.traps_rate()
    }

    #[text_signature = "($self, node)"]
    /// Return the degree for the given node.
    ///
    /// Parameters
    /// ---------------------
    /// node: int,
    ///     Node ID to use to compute degrees product.
    ///
    /// Returns
    /// ----------------------------
    /// degrees product for the two given nodes.
    ///
    fn degree(&self, node: NodeT) -> NodeT {
        self.graph.degree(node)
    }

    #[text_signature = "($self)"]
    /// Return all the degrees of the nodes graph.
    ///
    /// Returns
    /// ----------------------------
    /// Numpy array with all the degrees of the graph.
    ///
    fn degrees(&self) -> PyResult<Py<PyArray1<EdgeT>>> {
        let degrees = self.graph.degrees();
        let gil = pyo3::Python::acquire_gil();
        Ok(to_nparray_1d!(gil, degrees, EdgeT))
    }

    #[text_signature = "($self)"]
    /// Return mapping from instance not trap nodes to dense range of nodes.
    ///
    /// Returns
    /// ----------------------------
    /// Dict with mapping from not trap nodes to dense range of nodes.
    ///
    fn get_dense_nodes_mapping(&self) -> HashMap<NodeT, NodeT> {
        self.graph.get_dense_nodes_mapping()
    }

    #[text_signature = "($self, src, dst)"]
    /// Return boolean representing if given edge exists in graph.
    ///
    /// Parameters
    /// ---------------------
    /// src: int,
    ///     Node ID to use as source of given edge.
    /// dst: int,
    ///     Node ID to use as destination of given edge.
    ///
    /// Returns
    /// ----------------------------
    /// Boolean representing if given edge exists in graph.
    ///
    fn has_edge(&self, src: NodeT, dst: NodeT) -> bool {
        self.graph.has_edge(src, dst)
    }

    /// Return true if given graph has any edge overlapping with current graph.
    ///
    /// Parameters
    /// ----------------------------
    /// graph: EnsmallenGraph,
    ///     The graph to check against.
    ///
    /// Returns
    /// ----------------------------
    /// Boolean representing if any overlapping edge was found.
    pub fn overlaps(&self, graph: &EnsmallenGraph) -> bool {
        self.graph.overlaps(&graph.graph)
    }

    /// Return true if given graph edges are all contained within current graph.
    ///
    /// Parameters
    /// ----------------------------
    /// graph: EnsmallenGraph,
    ///     The graph to check against.
    ///
    /// Returns
    /// ----------------------------
    /// Boolean representing if graph contains completely the othe graph.
    pub fn contains(&self, graph: &EnsmallenGraph) -> bool {
        self.graph.contains(&graph.graph)
    }

    #[text_signature = "(self)"]
    /// Return the number of nodes in the graph.
    fn get_nodes_number(&self) -> usize {
        self.graph.get_nodes_number()
    }

    #[text_signature = "(self)"]
    /// Return the number of non trap nodes in the graph.
    fn get_not_trap_nodes_number(&self) -> usize {
        self.graph.get_not_trap_nodes_number()
    }

    #[text_signature = "(self)"]
    /// Return the number of edges in the graph.
    fn get_edges_number(&self) -> usize {
        self.graph.get_edges_number()
    }

    #[text_signature = "(self)"]
    /// Return the number of edges types in the graph.
    ///
    /// This method will include, if found necessary by a missing value,
    /// also the default edge type in the count of total edge types.
    ///
    fn get_edge_types_number(&self) -> usize {
        self.graph.get_edge_types_number()
    }

    #[text_signature = "(self)"]
    /// Return the number of edges in the graph.
    ///
    /// This method will include, if found necessary by a missing value,
    /// also the default node type in the count of total node types.
    ///
    fn get_node_types_number(&self) -> usize {
        self.graph.get_node_types_number()
    }

    #[text_signature = "($self, node)"]
    /// Return boolean representing if given node is a trap.
    ///
    /// A trap node is a node with no outbounds edges.
    ///
    /// Parameters
    /// ---------------------
    /// node: int,
    ///     Node ID to search if it's a trap.
    ///
    /// Returns
    /// ----------------------------
    /// Boolean representing if given node is a trap.
    ///
    fn is_node_trap(&self, node: NodeT) -> bool {
        self.graph.is_node_trap(node)
    }

    #[text_signature = "($self, edge)"]
    /// Return boolean representing if given edge is a trap.
    ///
    /// A trap edge is a edge with a destination node that is a trap node.
    ///
    /// Parameters
    /// ---------------------
    /// node: int,
    ///     Node ID to search if it's a trap.
    ///
    /// Returns
    /// ----------------------------
    /// Boolean representing if given edge is a trap.
    ///
    fn is_edge_trap(&self, edge: EdgeT) -> bool {
        self.graph.is_edge_trap(edge)
    }

    #[text_signature = "($self, node)"]
    /// Return list of Node IDs of the neighbours of given node.
    ///
    /// Parameters
    /// ---------------------
    /// node: int,
    ///     Node ID to
    ///
    /// Returns
    /// ----------------------------
    /// List of Node IDs of the neighbouring nodes.
    ///
    fn get_node_neighbours(&self, node: NodeT) -> Vec<NodeT> {
        self.graph.get_node_neighbours(node)
    }

    #[text_signature = "($self)"]
    /// Returns mean node degree of the graph.
    pub fn degrees_mean(&self) -> f64 {
        self.graph.degrees_mean()
    }

    #[text_signature = "($self)"]
    /// Returns median node degree of the graph.
    pub fn degrees_median(&self) -> NodeT {
        self.graph.degrees_median()
    }

    #[text_signature = "($self)"]
    /// Returns mode node degree of the graph.
    pub fn degrees_mode(&self) -> NodeT {
        self.graph.degrees_mode()
    }

    #[text_signature = "($self)"]
    /// Returns report relative to the graph metrics.
    ///
    /// The report includes a few useful metrics like:
    ///
    /// * degrees_median: the median degree of the nodes.
    /// * degrees_mean: the mean degree of the nodes.
    /// * degrees_mode: the mode degree of the nodes.
    /// * nodes_number: the number of nodes in the graph.
    /// * edges_number: the number of edges in the graph.
    /// * unique_node_types_number: the number of different node types in the graph.
    /// * unique_edge_types_number: the number of different edge types in the graph.
    ///
    fn report(&self) -> HashMap<&str, String> {
        self.graph.report()
    }

    #[text_signature = "($self, seed)"]
    /// Returns set of (typed) edges that form a spanning tree.NodeT
    ///
    /// The spanning tree is not minimal or maximal.
    /// The provided seed is not the root of the tree, but is only needed
    /// to identify a specific spanning tree.
    /// This spanning tree algorithm can run also on graph with multiple
    /// components.
    ///
    /// Parameters
    /// ------------------------
    /// seed: int,
    ///     The seed for the spanning tree.
    ///
    fn spanning_tree(&self, seed: NodeT) -> HashSet<(NodeT, NodeT, Option<NodeTypeT>)> {
        let tree: HashSet<(NodeT, NodeT, Option<NodeTypeT>)> =
            self.graph.spanning_tree(seed).iter().cloned().collect();
        tree
    }

    #[text_signature = "($self, seed, train_percentage)"]
    /// Returns training and validation holdouts extracted from current graph.
    ///
    /// The holdouts is generated in such a way that the training set remains
    /// connected if the starting graph is connected by using a spanning tree.
    ///
    /// Parameters
    /// -----------------------------
    /// seed: int,
    ///     The seed to use to generate the holdout.
    /// train_percentage: float,
    ///     The percentage to reserve for the training.
    ///
    /// Returns
    /// -----------------------------
    /// Tuple containing training and validation graphs.
    fn connected_holdout(
        &self,
        seed: NodeT,
        train_percentage: f64,
    ) -> PyResult<(EnsmallenGraph, EnsmallenGraph)> {
        match self.graph.connected_holdout(seed, train_percentage) {
            Ok((g1, g2)) => Ok((EnsmallenGraph { graph: g1 }, EnsmallenGraph { graph: g2 })),
            Err(e) => Err(PyErr::new::<exceptions::ValueError, _>(e)),
        }
    }

    #[text_signature = "($self, seed, nodes_number)"]
    /// Returns partial subgraph.
    ///
    /// This method creates a subset of the graph starting from a random node
    /// sampled using given seed and includes all neighbouring nodes until
    /// the required number of nodes is reached. All the edges connecting any
    /// of the selected nodes are then inserted into this graph.
    ///
    /// Parameters
    /// -----------------------------
    /// seed: int,
    ///     The seed to use to generate the partial graph.
    /// nodes_number: int,
    ///     The number of edges to insert in the partial graph.
    ///
    /// Returns
    /// -----------------------------
    /// Partial graph.
    fn random_subgraph(&self, seed: NodeT, nodes_number: NodeT) -> PyResult<EnsmallenGraph> {
        match self.graph.random_subgraph(seed, nodes_number) {
            Ok(g) => Ok(EnsmallenGraph { graph: g }),
            Err(e) => Err(PyErr::new::<exceptions::ValueError, _>(e)),
        }
    }

    #[text_signature = "($self, seed, train_percentage)"]
    /// Returns training and validation holdouts extracted from current graph.
    ///
    /// The holdouts edges are randomly sampled and have no garanties that any
    /// particular graph structure is maintained.
    ///
    /// Parameters
    /// -----------------------------
    /// seed: int,
    ///     The seed to use to generate the holdout.
    /// train_percentage: float,
    ///     The percentage to reserve for the training.
    ///
    /// Returns
    /// -----------------------------
    /// Tuple containing training and validation graphs.
    fn random_holdout(
        &self,
        seed: NodeT,
        train_percentage: f64,
    ) -> PyResult<(EnsmallenGraph, EnsmallenGraph)> {
        match self.graph.random_holdout(seed, train_percentage) {
            Ok((g1, g2)) => Ok((EnsmallenGraph { graph: g1 }, EnsmallenGraph { graph: g2 })),
            Err(e) => Err(PyErr::new::<exceptions::ValueError, _>(e)),
        }
    }

    #[text_signature = "($self, seed, negatives_number, allow_selfloops)"]
    /// Returns Graph with given amount of negative edges as positive edges.
    ///
    /// The graph generated may be used as a testing negatives partition to be
    /// fed into the argument "graph_to_avoid" of the link_prediction or the
    /// binary_skipgrams algorithm.
    ///
    ///
    /// Parameters
    /// -----------------------------
    /// seed: int,
    ///     The seed to use to generate the holdout.
    /// negatives_number: int,
    ///     The number of negative edges to use.
    /// allow_selfloops: bool,
    ///     Wethever to allow creation of self-loops.
    ///
    /// Returns
    /// -----------------------------
    /// Graph containing given amount of missing edges.
    fn sample_negatives(
        &self,
        seed: EdgeT,
        negatives_number: EdgeT,
        allow_selfloops: bool,
    ) -> PyResult<EnsmallenGraph> {
        match self
            .graph
            .sample_negatives(seed, negatives_number, allow_selfloops)
        {
            Ok(g) => Ok(EnsmallenGraph { graph: g }),
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
            validate_kwargs(kwargs, &[
                "graph_to_avoid", "negative_samples", "avoid_self_loops"
                ])?;
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

#[pyproto]
impl PyNumberProtocol for EnsmallenGraph {
    fn __add__(lhs: EnsmallenGraph, rhs: EnsmallenGraph) -> PyResult<EnsmallenGraph> {
        match lhs.graph.sum(&rhs.graph) {
            Ok(g) => Ok(EnsmallenGraph { graph: g }),
            Err(e) => Err(PyErr::new::<exceptions::ValueError, _>(e)),
        }
    }
}

#[pyproto]
impl PyObjectProtocol for EnsmallenGraph {
    fn __richcmp__(&self, other: EnsmallenGraph, op: CompareOp) -> PyResult<bool> {
        Ok(match op {
            CompareOp::Lt => other.graph.contains(&self.graph) && &other != self,
            CompareOp::Le => other.graph.contains(&self.graph),
            CompareOp::Eq => &other == self,
            CompareOp::Ne => &other != self,
            CompareOp::Gt => self.graph.contains(&other.graph) && &other != self,
            CompareOp::Ge => self.graph.contains(&other.graph),
        })
    }
}
