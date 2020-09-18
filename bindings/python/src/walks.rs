use super::*;
use graph::{NodeT, WalksParameters, WeightT};
use pyo3::exceptions;
use pyo3::prelude::*;
use pyo3::types::PyDict;
use std::collections::HashMap;

#[pymethods]
impl EnsmallenGraph {
    #[args(py_kwargs = "**")]
    #[text_signature = "($self, length, *, min_length, return_weight, explore_weight, change_edge_type_weight, change_node_type_weight, seed, verbose, iterations, dense_nodes_mapping)"]
    /// Return random walks done on the graph using Rust.
    ///
    /// Parameters
    /// ---------------------
    /// length: int,
    ///     Maximal length of the random walk.
    ///     On graphs without traps, all walks have this length.
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
    /// change_edge_type_weight: float = 1.0,
    ///     Weight on the probability of visiting a neighbor node of a
    ///     different type than the previous node. This only applies to
    ///     colored graphs, otherwise it has no impact.
    /// change_node_type_weight: float = 1.0,
    ///     Weight on the probability of visiting a neighbor edge of a
    ///     different type than the previous edge. This only applies to
    ///     multigraphs, otherwise it has no impact.
    /// seed: int = 42,
    ///     Seed to use to reproduce the walks.
    /// verbose: bool = False,
    ///     Wethever to show or not the loading bar of the walks.
    /// iterations: int = 1,
    ///     Number of cycles on the graphs to execute.
    /// dense_nodes_mapping: Dict[str, int] = None,
    ///     Mapping to use for converting sparse walk space into a dense space.
    ///     This object can be created using the method available from graph
    ///     called `get_dense_nodes_mapping` that returns a mapping from
    ///     the non trap nodes (those from where a walk could start) and
    ///     maps these nodes into a dense range of values.
    /// 
    /// Returns
    /// ----------------------------
    /// List of list of walks containing the numeric IDs of nodes.
    ///
    fn walk(&self, length: usize, py_kwargs: Option<&PyDict>) -> PyResult<Vec<Vec<NodeT>>> {
        let py = pyo3::Python::acquire_gil();
        let kwargs = normalize_kwargs!(py_kwargs, py.python());

        validate_kwargs(kwargs, &[
            "min_length",
            "return_weight",
            "explore_weight",
            "change_edge_type_weight",
            "change_node_type_weight",
            "seed",
            "verbose",
            "iterations",
            "dense_nodes_mapping"
        ])?;

        let parameters = pyex!(pyex!(pyex!(pyex!(pyex!(pyex!(pyex!(
            WalksParameters::new(length, 0, self.graph.get_not_trap_nodes_number())
        )?
        .set_change_edge_type_weight(extract_value!(
            kwargs,
            "change_edge_type_weight",
            WeightT
        )))?
        .set_change_node_type_weight(extract_value!(
            kwargs,
            "change_node_type_weight",
            WeightT
        )))?
        .set_explore_weight(extract_value!(kwargs, "explore_weight", WeightT)))?
        .set_return_weight(extract_value!(kwargs, "return_weight", WeightT)))?
        .set_seed(extract_value!(kwargs, "seed", usize))
        .set_verbose(extract_value!(kwargs, "verbose", bool))
        .set_iterations(extract_value!(kwargs, "iterations", usize)))?
        .set_min_length(extract_value!(kwargs, "min_length", usize)))?
        .set_dense_nodes_mapping(
            extract_value!(kwargs, "dense_nodes_mapping", HashMap<NodeT, NodeT>),
        );

        pyex!(self.graph.walk(&parameters))
    }
}
