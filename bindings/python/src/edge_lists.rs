use super::*;
use graph::NodeT;
use std::collections::HashSet;

#[pymethods]
impl EnsmallenGraph {
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
    pub fn get_clique_edges(
        &self,
        directed: Option<bool>,
        removed_existing_edges: Option<bool>,
        allow_node_type_set: Option<HashSet<String>>,
        allow_node_set: Option<HashSet<String>>,
    ) -> Vec<(NodeT, NodeT)> {
        self.graph.get_clique_edges(
            directed,
            removed_existing_edges,
            allow_node_type_set,
            allow_node_set,
        )
    }

    #[text_signature = "(self, *, directed, removed_existing_edges, allow_node_type_set, allow_node_set)"]
    /// Return vector of tuple of Node names that form the edges of the required clique.
    ///
    /// Parameters
    /// ------------------
    /// directed: bool = True,
    ///     Wether to return the edges as directed or undirected. By default, equal to the graph.
    /// removed_existing_edges = True - Wether to filter out the existing edges. By default, true.
    /// allow_node_type_set: Option<HashSet<String>> - Node types to include in the clique.
    /// allow_node_set: Option<HashSet<String>> - Nodes to include i the clique.
    pub fn get_clique_edge_names(
        &self,
        directed: Option<bool>,
        removed_existing_edges: Option<bool>,
        allow_node_type_set: Option<HashSet<String>>,
        allow_node_set: Option<HashSet<String>>,
    ) -> Vec<(String, String)> {
        self.graph.get_clique_edge_names(
            directed,
            removed_existing_edges,
            allow_node_type_set,
            allow_node_set,
        )
    }
}
