use graph::{EdgeT, EdgeTypeT, Graph, NodeT, NodeTypeT, ParamsT, WeightT};
use pyo3::exceptions;
use pyo3::prelude::*;
use pyo3::types::PyDict;
use numpy::{PyArray1, ToPyArray};
use std::collections::HashMap;

#[pymodule]
fn ensmallen_graph(_py: Python, m: &PyModule) -> PyResult<()> {
    // PyO3 aware function. All of our Python interfaces could be declared in a separate module.
    // Note that the `#[pyfn()]` annotation automatically converts the arguments from
    // Python objects to Rust values, and the Rust return value back into a Python object.
    // The `_py` argument represents that we're holding the GIL.
    m.add_class::<EnsmallenGraph>()?;
    env_logger::init();
    Ok(())
}

#[pyclass]
#[derive(Clone)]
#[text_signature = "(sources, destinations, *, nodes_mapping, nodes_reverse_mapping, node_types, node_types_mapping, node_types_reverse_mapping, edge_types, edge_types_mapping, edge_types_reverse_mapping, weights, validate_input_data, force_conversion_to_undirected)"]
/// Return new EnsmallenGraph.
/// 
/// sources: List[int],
///     The list of source nodes.
/// destinations: List[int],
///     The list of destination nodes.
/// nodes_mapping: Dict[str, int] = None,
///     The dictionary with mapping between the node name and its node ID.
/// nodes_reverse_mapping: List[str] = None,
///     The reverse mapping between numeric node ID and node name.
/// node_types: List[int] = None,
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
/// validate_input_data: bool = True,
///     Wethever to validate or not the input data.
/// force_conversion_to_undirected: bool = False,
///     Wethever to force the conversion from directed graph to undirected
///     when there are bidirectional directed edges in the given graph.
/// 
struct EnsmallenGraph {
    graph: Graph,
}

fn extract_value(val: &PyAny) -> &str {
    val.extract::<&str>().unwrap()
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
    )->PyResult<Self>{
        let graph = if let Some(kwargs) = py_kwargs {
            if directed {
                Graph::new_directed(
                    sources,
                    destinations,
                    kwargs.get_item("node_types").map(
                        |val| val.extract::<HashMap<String, NodeT>>().unwrap()
                    ),
                    kwargs.get_item("node_types_mapping").map(
                        |val| val.extract::<Vec<String>>().unwrap()
                    ),
                    kwargs.get_item("node_types").map(
                        |val| val.extract::<Vec<NodeTypeT>>().unwrap()
                    ),
                    kwargs.get_item("node_types_mapping").map(
                        |val| val.extract::<HashMap<String, NodeTypeT>>().unwrap()
                    ),
                    kwargs.get_item("node_types_reverse_mapping").map(
                        |val| val.extract::<Vec<String>>().unwrap()
                    ),
                    kwargs.get_item("edge_types").map(
                        |val| val.extract::<Vec<EdgeTypeT>>().unwrap()
                    ),
                    kwargs.get_item("edge_types_mapping").map(
                        |val| val.extract::<HashMap<String, EdgeTypeT>>().unwrap()
                    ),
                    kwargs.get_item("edge_types_reverse_mapping").map(
                        |val| val.extract::<Vec<String>>().unwrap()
                    ),
                    kwargs.get_item("weights").map(
                        |val| val.extract::<Vec<WeightT>>().unwrap()
                    ),
                    kwargs.get_item("validate_input_data").map(
                        |val| val.extract::<bool>().unwrap()
                    )
                )
            } else {
                Graph::new_undirected(
                    sources,
                    destinations,
                    kwargs.get_item("node_types").map(
                        |val| val.extract::<HashMap<String, NodeT>>().unwrap()
                    ),
                    kwargs.get_item("node_types_mapping").map(
                        |val| val.extract::<Vec<String>>().unwrap()
                    ),
                    kwargs.get_item("node_types").map(
                        |val| val.extract::<Vec<NodeTypeT>>().unwrap()
                    ),
                    kwargs.get_item("node_types_mapping").map(
                        |val| val.extract::<HashMap<String, NodeTypeT>>().unwrap()
                    ),
                    kwargs.get_item("node_types_reverse_mapping").map(
                        |val| val.extract::<Vec<String>>().unwrap()
                    ),
                    kwargs.get_item("edge_types").map(
                        |val| val.extract::<Vec<EdgeTypeT>>().unwrap()
                    ),
                    kwargs.get_item("edge_types_mapping").map(
                        |val| val.extract::<HashMap<String, EdgeTypeT>>().unwrap()
                    ),
                    kwargs.get_item("edge_types_reverse_mapping").map(
                        |val| val.extract::<Vec<String>>().unwrap()
                    ),
                    kwargs.get_item("weights").map(
                        |val| val.extract::<Vec<WeightT>>().unwrap()
                    ),
                    kwargs.get_item("validate_input_data").map(
                        |val| val.extract::<bool>().unwrap()
                    ),
                    kwargs.get_item("force_conversion_to_undirected").map(
                        |val| val.extract::<bool>().unwrap()
                    )
                )
            }
        } else if directed {
            Graph::new_directed(
                sources,
                destinations,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None
            )
        } else {
            Graph::new_undirected(
                sources,
                destinations,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None
            )
        };
            
        match graph {
            Ok(g) => Ok(EnsmallenGraph { graph: g }),
            Err(e) => Err(PyErr::new::<exceptions::ValueError, _>(e)),
        }
    }

    #[staticmethod]
    #[args(py_kwargs = "**")]
    #[text_signature = "(edge_path, sources_column, destinations_column, directed, *, edge_types_column, default_edge_type, weights_column, default_weight, node_path, nodes_column, node_types_column, default_node_type, edge_sep, node_sep, validate_input_data, ignore_duplicated_edges, ignore_duplicated_nodes, force_conversion_to_undirected)"]
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
    /// validate_input_data:bool=True,
    ///     Wethever to validate or not the files. This should be disabled when
    ///     you are SURE that the graph data are valid, otherwise the system will
    ///     panic.
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
            let graph = Graph::from_csv(
                edge_path,
                sources_column,
                destinations_column,
                directed,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None
            );

            return match graph {
                Ok(g) => Ok(EnsmallenGraph { graph: g }),
                Err(e) => Err(PyErr::new::<exceptions::ValueError, _>(e)),
            };
        }

        let kwargs = py_kwargs.unwrap();

        let graph = Graph::from_csv(
            edge_path,
            sources_column,
            destinations_column,
            directed,
            kwargs.get_item("edge_types_column").map(extract_value),
            kwargs.get_item("default_edge_type").map(extract_value),
            kwargs.get_item("weights_column").map(extract_value),
            kwargs
                .get_item("default_weight")
                .map(|val| val.extract::<WeightT>().unwrap()),
            kwargs.get_item("node_path").map(extract_value),
            kwargs.get_item("nodes_column").map(extract_value),
            kwargs.get_item("node_types_column").map(extract_value),
            kwargs.get_item("default_node_type").map(extract_value),
            kwargs.get_item("edge_sep").map(extract_value),
            kwargs.get_item("node_sep").map(extract_value),
            kwargs
                .get_item("validate_input_data")
                .map(|val| val.extract::<bool>().unwrap()),
            kwargs
                .get_item("ignore_duplicated_edges")
                .map(|val| val.extract::<bool>().unwrap()),
            kwargs
                .get_item("ignore_duplicated_nodes")
                .map(|val| val.extract::<bool>().unwrap()),
            kwargs
                .get_item("force_conversion_to_undirected")
                .map(|val| val.extract::<bool>().unwrap()),
        );

        match graph {
            Ok(g) => Ok(EnsmallenGraph { graph: g }),
            Err(e) => Err(PyErr::new::<exceptions::ValueError, _>(e)),
        }
    }

    #[text_signature = "($self, node_id)"]
    /// Return random walks done on the graph using Rust.
    ///
    /// Parameters
    /// ---------------------
    /// node_id: int,
    ///     Numeric ID of the node.
    ///
    /// Returns
    /// ---------------------
    /// Return the id of the node type of the node.
    fn get_node_type_id(&self, node_id: NodeT) -> PyResult<NodeTypeT> {
        match self.graph.get_node_type_id(node_id) {
            Ok(g) => Ok(g),
            Err(e) => Err(PyErr::new::<exceptions::ValueError, _>(e)),
        }
    }

    #[text_signature = "($self, edge_id)"]
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
    fn get_edge_type_id(&self, edge_id: EdgeT) -> PyResult<EdgeTypeT> {
        match self.graph.get_edge_type_id(edge_id) {
            Ok(g) => Ok(g),
            Err(e) => Err(PyErr::new::<exceptions::ValueError, _>(e)),
        }
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
        match self.graph.get_edge_id(src, dst) {
            Ok(g) => Ok(g),
            Err(e) => Err(PyErr::new::<exceptions::ValueError, _>(e)),
        }
    }

    #[args(py_kwargs = "**")]
    #[text_signature = "($self, length, *, iterations, start_node, end_node, min_length, return_weight, explore_weight, change_edge_type_weight, change_node_type_weight, verbose)"]
    /// Return random walks done on the graph using Rust.
    ///
    /// Parameters
    /// ---------------------
    /// length: int,
    ///     Maximal length of the random walk.
    ///     On graphs without traps, all walks have this length.
    /// iterations: int = 1,
    ///     Number of cycles on the graphs to execute.
    /// start_node: int = None,
    ///     Node ID from where to start the random walk.
    ///     If not provided, defaults to 0.
    /// end_node: int = None,
    ///     Node ID from where to end the random walk.
    ///     If not provided, has two possible behaviours:
    ///        - If start_node was provided, this is assumed to be
    ///          a single node walk, and end_node = start_node +1
    ///        - If start_node was not provided, this is assumed to be
    ///          a full graph walk, and end_node = total nodes number.
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
    /// verbose: int = True,
    ///     Wethever to show or not the loading bar of the walks.
    ///
    /// Returns
    /// ----------------------------
    /// List of list of walks containing the numeric IDs of nodes.
    ///
    fn walk(
        &self,
        length: usize,
        py_kwargs: Option<&PyDict>,
    ) -> PyResult<Vec<Vec<NodeT>>> {
        if py_kwargs.is_none() {
            let w = self
                .graph
                .walk(length, None, None, None, None, None, None, None, None, None);

            return match w {
                Ok(g) => Ok(g),
                Err(e) => Err(PyErr::new::<exceptions::ValueError, _>(e)),
            };
        }

        let kwargs = py_kwargs.unwrap();

        let w = self.graph.walk(
            length,
            kwargs
                .get_item("iterations")
                .map(|val| val.extract::<usize>().unwrap()),
            kwargs
                .get_item("start_node")
                .map(|val| val.extract::<usize>().unwrap()),
            kwargs
                .get_item("end_node")
                .map(|val| val.extract::<usize>().unwrap()),
            kwargs
                .get_item("min_length")
                .map(|val| val.extract::<usize>().unwrap()),
            kwargs
                .get_item("return_weight")
                .map(|val| val.extract::<ParamsT>().unwrap()),
            kwargs
                .get_item("explore_weight")
                .map(|val| val.extract::<ParamsT>().unwrap()),
            kwargs
                .get_item("change_node_type_weight")
                .map(|val| val.extract::<ParamsT>().unwrap()),
            kwargs
                .get_item("change_edge_type_weight")
                .map(|val| val.extract::<ParamsT>().unwrap()),
            kwargs
                .get_item("verbose")
                .map(|val| val.extract::<bool>().unwrap()),
        );

        match w {
            Ok(g) => Ok(g),
            Err(e) => Err(PyErr::new::<exceptions::ValueError, _>(e)),
        }
    }

    #[args(py_kwargs = "**")]
    #[text_signature = "($self, length, *, window_size, iterations, min_length, return_weight, explore_weight, change_edge_type_weight, change_node_type_weight, verbose)"]
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
        if py_kwargs.is_none() {
            let w = self
                .graph
                .cooccurence_matrix(length, None, None, None, None, None, None, None, None);
            let gil = pyo3::Python::acquire_gil();
            return match w {
                Ok(g) => Ok(
                    (
                        g.0.to_pyarray(gil.python()).cast::<f64>(false).unwrap().to_owned(),
                        g.1.to_pyarray(gil.python()).cast::<f64>(false).unwrap().to_owned(),
                        g.2.to_pyarray(gil.python()).cast::<f64>(false).unwrap().to_owned()
                    )
                ),
                Err(e) => Err(PyErr::new::<exceptions::ValueError, _>(e)),
            };
        }

        let kwargs = py_kwargs.unwrap();

        let w = self.graph.cooccurence_matrix(
            length,
            kwargs
                .get_item("window_size")
                .map(|val| val.extract::<usize>().unwrap()),
            kwargs
                .get_item("iterations")
                .map(|val| val.extract::<usize>().unwrap()),
            kwargs
                .get_item("min_length")
                .map(|val| val.extract::<usize>().unwrap()),
            kwargs
                .get_item("return_weight")
                .map(|val| val.extract::<ParamsT>().unwrap()),
            kwargs
                .get_item("explore_weight")
                .map(|val| val.extract::<ParamsT>().unwrap()),
            kwargs
                .get_item("change_node_type_weight")
                .map(|val| val.extract::<ParamsT>().unwrap()),
            kwargs
                .get_item("change_edge_type_weight")
                .map(|val| val.extract::<ParamsT>().unwrap()),
            kwargs
                .get_item("verbose")
                .map(|val| val.extract::<bool>().unwrap()),
        );
        
        let gil = pyo3::Python::acquire_gil();
        match w {
            Ok(g) => Ok(
                (
                    g.0.to_pyarray(gil.python()).cast::<f64>(false).unwrap().to_owned(),
                    g.1.to_pyarray(gil.python()).cast::<f64>(false).unwrap().to_owned(),
                    g.2.to_pyarray(gil.python()).cast::<f64>(false).unwrap().to_owned()
                )
            ),
            Err(e) => Err(PyErr::new::<exceptions::ValueError, _>(e)),
        }
    }

    #[args(py_kwargs = "**")]
    #[text_signature = "($self, idx, batch_size, length, *, iterations, window_size, negative_samples, shuffle, iterations, min_length, return_weight, explore_weight, change_edge_type_weight, change_node_type_weight)"]
    /// Return cooccurence matrix-based triples of words, contexts and frequencies.
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
    ///
    /// Returns
    /// ----------------------------
    /// Triple with vector of integer with words, contexts and labels.
    ///
    fn skipgrams(
        &self,
        idx:usize,
        batch_size:usize,
        length: usize,
        py_kwargs: Option<&PyDict>,
    ) -> PyResult<((Py<PyArray1<f64>>, Py<PyArray1<f64>>), Py<PyArray1<f64>>)>{
        if py_kwargs.is_none() {
            let w = self
                .graph
                .skipgrams(idx, batch_size, length, None, None, None, None, None, None, None, None, None);

            let gil = pyo3::Python::acquire_gil();
            return match w {
                Ok(g) => Ok((
                    (
                        (g.0).0.to_pyarray(gil.python()).cast::<f64>(false).unwrap().to_owned(),
                        (g.0).1.to_pyarray(gil.python()).cast::<f64>(false).unwrap().to_owned()
                    ),
                    g.1.to_pyarray(gil.python()).cast::<f64>(false).unwrap().to_owned()
                )),
                Err(e) => Err(PyErr::new::<exceptions::ValueError, _>(e)),
            };
        }

        let kwargs = py_kwargs.unwrap();

        let w = self.graph.skipgrams(
            idx, batch_size, length,
            kwargs
                .get_item("iterations")
                .map(|val| val.extract::<usize>().unwrap()),
            kwargs
                .get_item("window_size")
                .map(|val| val.extract::<usize>().unwrap()),
            kwargs
                .get_item("negative_samples")
                .map(|val| val.extract::<f64>().unwrap()),
            kwargs
                .get_item("shuffle")
                .map(|val| val.extract::<bool>().unwrap()),
            kwargs
                .get_item("min_length")
                .map(|val| val.extract::<usize>().unwrap()),
            kwargs
                .get_item("return_weight")
                .map(|val| val.extract::<ParamsT>().unwrap()),
            kwargs
                .get_item("explore_weight")
                .map(|val| val.extract::<ParamsT>().unwrap()),
            kwargs
                .get_item("change_node_type_weight")
                .map(|val| val.extract::<ParamsT>().unwrap()),
            kwargs
                .get_item("change_edge_type_weight")
                .map(|val| val.extract::<ParamsT>().unwrap()),
        );
        
        let gil = pyo3::Python::acquire_gil();
        match w {
            Ok(g) => Ok((
                (
                    (g.0).0.to_pyarray(gil.python()).cast::<f64>(false).unwrap().to_owned(),
                    (g.0).1.to_pyarray(gil.python()).cast::<f64>(false).unwrap().to_owned()
                ),
                g.1.to_pyarray(gil.python()).cast::<f64>(false).unwrap().to_owned()
            )),
            Err(e) => Err(PyErr::new::<exceptions::ValueError, _>(e)),
        }
    }

    #[getter]
    fn sources(&self) -> Py<PyArray1<NodeT>> {
        let gil = pyo3::Python::acquire_gil();
        self.graph.sources().clone().to_pyarray(gil.python()).to_owned()
    }

    #[getter]
    fn destinations(&self) ->  Py<PyArray1<NodeT>>  {
        let gil = pyo3::Python::acquire_gil();
        self.graph.destinations().clone().to_pyarray(gil.python()).to_owned()
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
    fn jaccard_index(&self, one: NodeT, two: NodeT) -> f64 {
        self.graph.jaccard_index(one, two)
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
    fn adamic_adar_index(&self, one: NodeT, two: NodeT) -> f64 {
        self.graph.adamic_adar_index(one, two)
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
    fn resource_allocation_index(&self, one: NodeT, two: NodeT) -> f64 {
        self.graph.resource_allocation_index(one, two)
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
    fn degrees_product(&self, one: NodeT, two: NodeT) -> usize {
        self.graph.degrees_product(one, two)
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
    /// Numpy array with all teh degrees of the graph.
    ///
    fn degrees(&self) -> Py<PyArray1<EdgeT>> {
        let degrees = self.graph.degrees();
        let gil = pyo3::Python::acquire_gil();
        degrees.to_pyarray(gil.python()).cast::<EdgeT>(false).unwrap().to_owned()
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

    #[text_signature = "(self)"]
    /// Return the number of NON-SINGLETONS nodes in the graph.
    fn get_nodes_number(&self) -> usize {
        self.graph.get_nodes_number()
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
        self.graph
            .report()
            .iter()
            .map(|(k, v)| (*k, v.clone()))
            .collect()
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
    fn holdout(&self, seed:NodeT, train_percentage:f64) -> PyResult<(EnsmallenGraph, EnsmallenGraph)> {
        match self.graph.holdout(seed, train_percentage) {
            Ok((g1, g2)) => Ok((EnsmallenGraph{graph:g1}, EnsmallenGraph{graph:g2})),
            Err(e) => Err(PyErr::new::<exceptions::ValueError, _>(e)),
        }
    }

    #[args(py_kwargs = "**")]
    #[text_signature = "($self, batch_size, negative_samples, graph_to_avoid, shuffle)"]
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
    fn link_prediction(&self, batch_size:u64, py_kwargs: Option<&PyDict>) -> PyResult<(Py<PyArray1<NodeT>>, Py<PyArray1<NodeT>>, Py<PyArray1<u8>>)> {
        let results = if let Some(kwargs) = py_kwargs {
            let ensmallen_graph = kwargs
                .get_item("graph_to_avoid")
                .map(|val| val.extract::<EnsmallenGraph>());
            let graph = if let Some(eg) = &ensmallen_graph {
                match eg {
                    Ok(g) => Some(&g.graph),
                    Err(_) => None
                }
            } else {
                None
            };
            self.graph.link_prediction(
                batch_size,
                kwargs
                    .get_item("negative_samples")
                    .map(|val| val.extract::<f64>().unwrap()),
                graph,
                kwargs
                    .get_item("shuffle")
                    .map(|val| val.extract::<bool>().unwrap())
            )
        } else {
            self.graph.link_prediction(batch_size, None, None, None)
        };
        let gil = pyo3::Python::acquire_gil();
        match results {
            Ok((sources, destinations, labels)) => Ok((
                sources.to_pyarray(gil.python()).cast::<NodeT>(false).unwrap().to_owned(),
                destinations.to_pyarray(gil.python()).cast::<NodeT>(false).unwrap().to_owned(),
                labels.to_pyarray(gil.python()).cast::<u8>(false).unwrap().to_owned()
            )),
            Err(e) => Err(PyErr::new::<exceptions::ValueError, _>(e)),
        }
    }
}