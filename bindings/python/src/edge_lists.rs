use super::*;
use graph::NodeT;
use numpy::{PyArray, PyArray2};
use std::collections::HashSet;

#[pymethods]
impl EnsmallenGraph {
    #[text_signature = "(self, *, removed_existing_edges, first_nodes_set, second_nodes_set, first_node_types_set, second_node_types_set)"]
    /// Return vector of tuple of Node IDs that form the edges of the required bipartite graph.
    ///
    /// Parameteres
    /// ----------------------
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
    /// Raises
    /// ----------------------
    /// ValueError,
    ///     If given node sets lead to an empty bipartite graph.
    /// ValueError,
    ///     If given node sets overlap.
    ///
    /// Returns
    /// ----------------------
    /// Numpy vector of the node IDs forming the required bipartite graph.
    pub fn get_bipartite_edges(
        &self,
        removed_existing_edges: Option<bool>,
        first_nodes_set: Option<HashSet<String>>,
        second_nodes_set: Option<HashSet<String>>,
        first_node_types_set: Option<HashSet<String>>,
        second_node_types_set: Option<HashSet<String>>,
    ) -> PyResult<Py<PyArray2<NodeT>>> {
        let gil = pyo3::Python::acquire_gil();
        Ok(to_nparray_2d!(
            gil,
            pyex!(self.graph.get_bipartite_edges(
                removed_existing_edges,
                first_nodes_set,
                second_nodes_set,
                first_node_types_set,
                second_node_types_set,
            ))?,
            NodeT
        ))
    }

    #[text_signature = "(self, *, removed_existing_edges, first_nodes_set, second_nodes_set, first_node_types_set, second_node_types_set)"]
    /// Return vector of tuple of Node names that form the edges of the required bipartite graph.
    ///
    /// Parameteres
    /// ----------------------
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
    /// Raises
    /// ----------------------
    /// ValueError,
    ///     If given node sets lead to an empty bipartite graph.
    /// ValueError,
    ///     If given node sets overlap.
    ///
    /// Returns
    /// ----------------------
    /// Numpy vector of the node names forming the required bipartite graph.
    pub fn get_bipartite_edge_names(
        &self,
        removed_existing_edges: Option<bool>,
        first_nodes_set: Option<HashSet<String>>,
        second_nodes_set: Option<HashSet<String>>,
        first_node_types_set: Option<HashSet<String>>,
        second_node_types_set: Option<HashSet<String>>,
    ) -> PyResult<Vec<Vec<String>>> {
        pyex!(self.graph.get_bipartite_edge_names(
            removed_existing_edges,
            first_nodes_set,
            second_nodes_set,
            first_node_types_set,
            second_node_types_set,
        ))
    }

    #[text_signature = "(self, *, central_node, removed_existing_edges, star_points_nodes_set, star_points_node_types_set)"]
    /// Return vector of tuple of Node IDs that form the edges of the required star.
    ///
    /// Parameteres
    /// ----------------------
    /// central_node: str,
    ///     Name of the node to use as center of the star.
    /// removed_existing_edges: bool = True,
    ///     Wether to filter out the existing edges. By default, true.
    /// star_points_nodes_set: Set[str] = None,
    ///     Optional set of nodes to use to create the set of star points.
    /// star_points_node_types_set: Set[str] = None,
    ///     Optional set of node types to create the set of star points.
    ///
    /// Raises
    /// ----------------------
    /// ValueError,
    ///     If given node sets lead to an empty star graph.
    /// ValueError,
    ///     If given central is present in given node set.
    ///
    /// Returns
    /// ----------------------
    /// Numpy vector of the node IDs forming the required star graph.
    pub fn get_star_edges(
        &self,
        central_node: String,
        removed_existing_edges: Option<bool>,
        star_points_nodes_set: Option<HashSet<String>>,
        star_points_node_types_set: Option<HashSet<String>>,
    ) -> PyResult<Py<PyArray2<NodeT>>> {
        let gil = pyo3::Python::acquire_gil();
        Ok(to_nparray_2d!(
            gil,
            pyex!(self.graph.get_star_edges(
                central_node,
                removed_existing_edges,
                star_points_nodes_set,
                star_points_node_types_set
            ))?,
            NodeT
        ))
    }

    #[text_signature = "(self, *, central_node, removed_existing_edges, star_points_nodes_set, star_points_node_types_set)"]
    /// Return vector of tuple of Node names that form the edges of the required star.
    ///
    /// Parameteres
    /// ----------------------
    /// central_node: str,
    ///     Name of the node to use as center of the star.
    /// removed_existing_edges: bool = True,
    ///     Wether to filter out the existing edges. By default, true.
    /// star_points_nodes_set: Set[str] = None,
    ///     Optional set of nodes to use to create the set of star points.
    /// star_points_node_types_set: Set[str] = None,
    ///     Optional set of node types to create the set of star points.
    ///
    /// Raises
    /// ----------------------
    /// ValueError,
    ///     If given node sets lead to an empty star graph.
    /// ValueError,
    ///     If given central is present in given node set.
    ///
    /// Returns
    /// ----------------------
    /// Numpy vector of the node names forming the required star graph.
    pub fn get_star_edge_names(
        &self,
        central_node: String,
        removed_existing_edges: Option<bool>,
        star_points_nodes_set: Option<HashSet<String>>,
        star_points_node_types_set: Option<HashSet<String>>,
    ) -> PyResult<Vec<Vec<String>>> {
        pyex!(self.graph.get_star_edge_names(
            central_node,
            removed_existing_edges,
            star_points_nodes_set,
            star_points_node_types_set
        ))
    }

    #[text_signature = "(self, *, directed, removed_existing_edges, allow_node_type_set, allow_node_set)"]
    /// Return vector of tuple of Node IDs that form the edges of the required clique.
    ///
    /// Parameters
    /// ---------------------
    /// directed: bool = None,
    ///     Wether to return the edges as directed or undirected. By default, equal to the graph.
    /// allow_self_loops: bool = None,
    ///     Wether to allow self-loops in the clique. By default, equal to the graph.
    /// removed_existing_edges = True,
    ///     Wether to filter out the existing edges. By default, true.
    /// allow_node_type_set: Set[str] = None,
    ///     Node types to include in the clique.
    /// allow_node_set: Set[str] = None,
    ///     Nodes to include i the clique.
    ///
    /// Returns
    /// ----------------------
    /// Numpy vector of the node IDs forming the required clique.
    pub fn get_clique_edges(
        &self,
        directed: Option<bool>,
        allow_self_loops: Option<bool>,
        removed_existing_edges: Option<bool>,
        allow_node_type_set: Option<HashSet<String>>,
        allow_node_set: Option<HashSet<String>>,
    ) -> PyResult<Py<PyArray2<NodeT>>> {
        let gil = pyo3::Python::acquire_gil();
        Ok(to_nparray_2d!(
            gil,
            self.graph.get_clique_edges(
                directed,
                allow_self_loops,
                removed_existing_edges,
                allow_node_type_set,
                allow_node_set,
            ),
            NodeT
        ))
    }

    #[text_signature = "(self, *, directed, allow_self_loops, removed_existing_edges, allow_node_type_set, allow_node_set)"]
    /// Return vector of tuple of Node IDs that form the edges of the required clique.
    ///
    /// Parameters
    /// ---------------------
    /// directed: bool = None,
    ///     Wether to return the edges as directed or undirected. By default, equal to the graph.
    /// allow_self_loops: bool = None,
    ///     Wether to allow self-loops in the clique. By default, equal to the graph.
    /// removed_existing_edges = True,
    ///     Wether to filter out the existing edges. By default, true.
    /// allow_node_type_set: Set[str] = None,
    ///     Node types to include in the clique.
    /// allow_node_set: Set[str] = None,
    ///     Nodes to include i the clique.
    ///
    /// Returns
    /// ----------------------
    /// Numpy vector of the node names forming the required clique.
    pub fn get_clique_edge_names(
        &self,
        directed: Option<bool>,
        allow_self_loops: Option<bool>,
        removed_existing_edges: Option<bool>,
        allow_node_type_set: Option<HashSet<String>>,
        allow_node_set: Option<HashSet<String>>,
    ) -> Vec<Vec<String>> {
        self.graph.get_clique_edge_names(
            directed,
            allow_self_loops,
            removed_existing_edges,
            allow_node_type_set,
            allow_node_set,
        )
    }
}
