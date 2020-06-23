use graph::{EdgeT, EdgeTypeT, Graph, NodeT, NodeTypeT, ParamsT, WeightT};
use pyo3::exceptions;
use pyo3::prelude::*;
use pyo3::types::PyDict;
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
#[text_signature = "(edge_path, sources_column, destinations_column, directed, *, edge_types_column, default_edge_type, weights_column, default_weight, node_path, nodes_column, node_types_column, default_node_type, edge_sep, node_sep, validate_input_data)"]
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
/// edge_sep:str,
///     Separator to use for the edge files.
/// node_sep:str,
///     Separator to use for the node files.
/// validate_input_data:bool,
///     Wethever to validate or not the files. This should be disabled when
///     you are SURE that the graph data are valid, otherwise the system will
///     panic.
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
        );

        match graph {
            Ok(g) => Ok(EnsmallenGraph { graph: g }),
            Err(e) => Err(PyErr::new::<exceptions::ValueError, _>(e)),
        }
    }

    #[text_signature = "(node_id)"]
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

    #[text_signature = "(edge_id)"]
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

    #[text_signature = "(src, dst)"]
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

    #[text_signature = "(iterations, length, min_length, return_weight, explore_weight, change_edge_type_weight, change_node_type_weight)"]
    /// Return random walks done on the graph using Rust.
    ///
    /// Parameters
    /// ---------------------
    /// iterations,
    ///     Number of cycles on the graphs to execute.
    /// length,
    ///     Maximal length of the random walk.
    ///     On graphs without traps, all walks have this length.
    /// min_length: int = 0,,
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
    /// List of list of walks containing the numeric IDs of nodes.
    ///
    fn walk(
        &self,
        iterations: usize,
        length: usize,
        min_length: usize,
        return_weight: ParamsT,
        explore_weight: ParamsT,
        change_node_type_weight: ParamsT,
        change_edge_type_weight: ParamsT,
    ) -> PyResult<Vec<Vec<NodeT>>> {
        let w = self.graph.walk(
            iterations,
            length,
            Some(min_length),
            Some(return_weight),
            Some(explore_weight),
            Some(change_node_type_weight),
            Some(change_edge_type_weight),
        );

        match w {
            Ok(g) => Ok(g),
            Err(e) => Err(PyErr::new::<exceptions::ValueError, _>(e)),
        }
    }

    #[getter]
    fn sources(&self) -> Vec<NodeT> {
        self.graph.sources().clone()
    }

    #[getter]
    fn destinations(&self) -> Vec<NodeT> {
        self.graph.destinations().clone()
    }

    #[getter]
    fn worddictionary(&self) -> HashMap<String, NodeT> {
        self.graph.nodes_mapping().clone()
    }

    #[getter]
    fn reverse_worddictionary(&self) -> Vec<String> {
        self.graph.nodes_reverse_mapping().clone()
    }

    #[getter]
    fn unique_edges(&self) -> HashMap<(NodeT, NodeT), EdgeT> {
        self.graph.unique_edges().clone()
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
        self.graph.node_types_mapping().clone()
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
        self.graph.edge_types_mapping().clone()
    }

    #[getter]
    fn edge_types_reverse_mapping(&self) -> Option<Vec<String>> {
        self.graph.edge_types_reverse_mapping().clone()
    }
    
    #[text_signature = "(one, two)"]
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
    fn jaccard_index(&self, one: NodeT, two: NodeT) -> f64{
        self.graph.jaccard_index(one, two)
    }
    
    #[text_signature = "(one, two)"]
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
    fn adamic_adar_index(&self, one: NodeT, two: NodeT) -> f64{
        self.graph.adamic_adar_index(one, two)
    }


    #[text_signature = "(one, two)"]
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
    fn resource_allocation_index(&self, one: NodeT, two: NodeT) -> f64{
        self.graph.resource_allocation_index(one, two)
    }

    #[text_signature = "(one, two)"]
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
    fn degrees_product(&self, one: NodeT, two: NodeT) -> usize{
        self.graph.degrees_product(one, two)
    }


    #[text_signature = "(node)"]
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
    fn degree(&self, node: NodeT) -> NodeT{
        self.graph.degree(node)
    }

    #[text_signature = "(src, dst)"]
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

    
    #[text_signature = "()"]
    /// Return the number of NON-SINGLETONS nodes in the graph.
    fn get_nodes_number(&self) -> usize {
        self.graph.get_nodes_number()
    }

    #[text_signature = "()"]
    /// Return the number of edges in the graph.
    fn get_edges_number(&self) -> usize {
        self.graph.get_edges_number()
    }
    
    #[text_signature = "()"]
    /// Return the number of edges types in the graph.
    /// 
    /// This method will include, if found necessary by a missing value,
    /// also the default edge type in the count of total edge types.
    /// 
    fn get_edge_types_number(&self) -> usize {
        self.graph.get_edge_types_number()
    }
    
    #[text_signature = "()"]
    /// Return the number of edges in the graph.
    /// 
    /// This method will include, if found necessary by a missing value,
    /// also the default node type in the count of total node types.
    /// 
    fn get_node_types_number(&self) -> usize {
        self.graph.get_node_types_number()
    }

    #[text_signature = "(node)"]
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
    fn is_node_trap(&self, node: NodeT) -> bool{
        self.graph.is_node_trap(node)
    }

    #[text_signature = "(edge)"]
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
    fn is_edge_trap(&self, edge: EdgeT) -> bool{
        self.graph.is_edge_trap(edge)
    }
    
    #[text_signature = "(node)"]
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
    fn get_node_neighbours(&self, node:NodeT) -> Vec<NodeT>{
        self.graph.get_node_neighbours(node)
    }

    
}
