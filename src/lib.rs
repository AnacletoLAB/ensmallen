use graph::{Graph, NodeT, ParamsT, WeightT, NodeTypeT, EdgeT, EdgeTypeT};
use pyo3::prelude::*;
use pyo3::exceptions;
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
#[text_signature = "(edge_path, sources_column, destinations_column, directed, *, edge_types_column, weights_column, node_path, nodes_column, node_types_column, edge_sep, node_sep, validate_input_data)"]
/// Build the graph from a csv (or tsv) in Rust.
/// 
/// Parameters
/// ---------------------
/// edge_path,
/// sources_column,
/// destinations_column,
/// directed,
/// edge_types_column,
/// weights_column,
/// node_path,
/// nodes_column, 
/// node_types_column, 
/// edge_sep, 
/// node_sep, 
/// validate_input_data
/// 
struct EnsmallenGraph {
    graph: Graph
}

fn extract_value(val: &PyAny) -> &str {
    val.extract::<&str>().unwrap()
}

#[pymethods]
impl EnsmallenGraph {
    #[new]  
    #[args(
        py_kwargs = "**"
    )]
    fn new( edge_path: &str,
            sources_column: &str,
            destinations_column: &str,
            directed: bool,
            py_kwargs: Option<&PyDict>
        ) -> PyResult<Self> {

        if py_kwargs.is_none() {
            let graph = Graph::from_csv(
                edge_path,
                sources_column, 
                destinations_column, 
                directed,
                None, None, None, None,
                None, None, None, None,
                None, None, None
            );

            return match graph {
                Ok(g) => Ok(EnsmallenGraph{ graph: g }),
                Err(e) => Err(PyErr::new::<exceptions::ValueError, _>(e))
            }
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
            kwargs.get_item("default_weight").map(|val| val.extract::<WeightT>().unwrap()),
            kwargs.get_item("node_path").map(extract_value),
            kwargs.get_item("nodes_column").map(extract_value),
            kwargs.get_item("node_types_column").map(extract_value),
            kwargs.get_item("default_node_type").map(extract_value),
            kwargs.get_item("edge_sep").map(extract_value),
            kwargs.get_item("node_sep").map(extract_value),
            kwargs.get_item("validate_input_data").map(|val| val.extract::<bool>().unwrap())
        );

        
        return match graph {
            Ok(g) => Ok(EnsmallenGraph{ graph: g }),
            Err(e) => Err(PyErr::new::<exceptions::ValueError, _>(e))
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
    fn get_node_type_id(&self, node_id: NodeT) -> NodeTypeT{
         self.graph.get_node_type_id(node_id).unwrap()
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
    fn get_edge_type_id(&self, edge_id: EdgeT)->EdgeTypeT{
        self.graph.get_edge_type_id(edge_id).unwrap()
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
    fn get_edge_id(&self, src: NodeT, dst:NodeT)->EdgeT{
        self.graph.get_edge_id(src, dst)
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
    fn walk(&self,
        iterations: usize,
        length: usize,
        min_length: usize,
        return_weight: ParamsT,
        explore_weight: ParamsT,
        change_node_type_weight: ParamsT,
        change_edge_type_weight: ParamsT
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
        
        return match w {
            Ok(g) => Ok(g),
            Err(e) => Err(PyErr::new::<exceptions::ValueError, _>(e))
        };
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

    
}