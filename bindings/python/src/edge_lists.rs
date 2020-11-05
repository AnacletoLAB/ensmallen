use super::*;
use graph::NodeT;
use numpy::{PyArray, PyArray2};
use std::collections::HashSet;

#[pymethods]
impl EnsmallenGraph {
    #[text_signature = "(self, *, directed, removed_existing_edges, directed, removed_existing_edges, first_nodes_set, second_nodes_set, first_node_types_set, second_node_types_set)"]
    /// Return vector of tuple of Node IDs that form the edges of the required bipartite graph.
    ///
    /// Parameteres
    /// ----------------------
    /// directed: bool = None,
    ///     Wether to return the edges as directed or undirected. By default, equal to the graph.
    /// removed_existing_edges: bool = True,
    ///     Wether to filter out the existing edges. By default, true.
    /// first_nodes_set: Set[str] = None,
    ///     Optional set of nodes to use to create the first set of nodes of the graph.
    /// second_nodes_set: Set[str] = None,
    ///     Optional set of nodes to use to create the second set of nodes of the graph.
    /// first_node_types_set: Set[str] = None,
    ///     Optional set of node types to create the first set of nodes of the graph.
    /// second_node_types_set: Set[str] = None,
    ///     Optional set of node types to create the second set of nodes of the graph.
    ///
    /// Returns
    /// ----------------------
    /// Numpy vector of the edges IDs forming the required bipartite graph.
    pub fn get_bipartite_edges(
        &self,
        directed: Option<bool>,
        removed_existing_edges: Option<bool>,
        first_nodes_set: Option<HashSet<String>>,
        second_nodes_set: Option<HashSet<String>>,
        first_node_types_set: Option<HashSet<String>>,
        second_node_types_set: Option<HashSet<String>>,
    ) -> PyResult<Py<PyArray2<NodeT>>> {
        let gil = pyo3::Python::acquire_gil();
        Ok(to_nparray_2d!(
            gil,
            self.graph.get_bipartite_edges(
                directed,
                removed_existing_edges,
                first_nodes_set,
                second_nodes_set,
                first_node_types_set,
                second_node_types_set,
            ),
            NodeT
        ))
    }

    #[text_signature = "(self, *, directed, removed_existing_edges, directed, removed_existing_edges, first_nodes_set, second_nodes_set, first_node_types_set, second_node_types_set)"]
    /// Return vector of tuple of Node names that form the edges of the required bipartite graph.
    ///
    /// Parameteres
    /// ----------------------
    /// directed: bool = None,
    ///     Wether to return the edges as directed or undirected. By default, equal to the graph.
    /// removed_existing_edges: bool = True,
    ///     Wether to filter out the existing edges. By default, true.
    /// first_nodes_set: Set[str] = None,
    ///     Optional set of nodes to use to create the first set of nodes of the graph.
    /// second_nodes_set: Set[str] = None,
    ///     Optional set of nodes to use to create the second set of nodes of the graph.
    /// first_node_types_set: Set[str] = None,
    ///     Optional set of node types to create the first set of nodes of the graph.
    /// second_node_types_set: Set[str] = None,
    ///     Optional set of node types to create the second set of nodes of the graph.
    ///
    /// Returns
    /// ----------------------
    /// Numpy vector of the edges names forming the required bipartite graph.
    pub fn get_bipartite_edge_names(
        &self,
        directed: Option<bool>,
        removed_existing_edges: Option<bool>,
        first_nodes_set: Option<HashSet<String>>,
        second_nodes_set: Option<HashSet<String>>,
        first_node_types_set: Option<HashSet<String>>,
        second_node_types_set: Option<HashSet<String>>,
    ) -> Vec<Vec<String>> {
        self.graph.get_bipartite_edge_names(
            directed,
            removed_existing_edges,
            first_nodes_set,
            second_nodes_set,
            first_node_types_set,
            second_node_types_set,
        )
    }

    #[text_signature = "(self, *, directed, removed_existing_edges, allow_node_type_set, allow_node_set)"]
    /// Return vector of tuple of Node IDs that form the edges of the required clique.
    ///
    /// Parameters
    /// ---------------------
    /// directed: bool = None,
    ///     Wether to return the edges as directed or undirected. By default, equal to the graph.
    /// removed_existing_edges = True,
    ///     Wether to filter out the existing edges. By default, true.
    /// allow_node_type_set: Set[str] = None,
    ///     Node types to include in the clique.
    /// allow_node_set: Set[str] = None,
    ///     Nodes to include i the clique.
    ///
    /// Returns
    /// ----------------------
    /// Numpy vector of the edges IDs forming the required clique.
    pub fn get_clique_edges(
        &self,
        directed: Option<bool>,
        removed_existing_edges: Option<bool>,
        allow_node_type_set: Option<HashSet<String>>,
        allow_node_set: Option<HashSet<String>>,
    ) -> PyResult<Py<PyArray2<NodeT>>> {
        let gil = pyo3::Python::acquire_gil();
        Ok(to_nparray_2d!(
            gil,
            self.graph.get_clique_edges(
                directed,
                removed_existing_edges,
                allow_node_type_set,
                allow_node_set,
            ),
            NodeT
        ))
    }

    #[text_signature = "(self, *, directed, removed_existing_edges, allow_node_type_set, allow_node_set)"]
    /// Return vector of tuple of Node IDs that form the edges of the required clique.
    ///
    /// Parameters
    /// ---------------------
    /// directed: bool = None,
    ///     Wether to return the edges as directed or undirected. By default, equal to the graph.
    /// removed_existing_edges = True,
    ///     Wether to filter out the existing edges. By default, true.
    /// allow_node_type_set: Set[str] = None,
    ///     Node types to include in the clique.
    /// allow_node_set: Set[str] = None,
    ///     Nodes to include i the clique.
    ///
    /// Returns
    /// ----------------------
    /// Numpy vector of the edges names forming the required clique.
    pub fn get_clique_edge_names(
        &self,
        directed: Option<bool>,
        removed_existing_edges: Option<bool>,
        allow_node_type_set: Option<HashSet<String>>,
        allow_node_set: Option<HashSet<String>>,
    ) -> Vec<Vec<String>> {
        self.graph.get_clique_edge_names(
            directed,
            removed_existing_edges,
            allow_node_type_set,
            allow_node_set,
        )
    }
}
