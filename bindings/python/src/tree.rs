use super::*;
use graph::{EdgeTypeT, NodeT};
use pyo3::prelude::*;
use std::collections::HashSet;

#[pymethods]
impl EnsmallenGraph {
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
    /// include_all_edge_types: bool,
    ///     Wethever to include all the edges between two nodes.
    ///     This is only relevant in multi-graphs.
    /// 
    /// Raises
    /// ------------------------
    /// TODO: update the set of exceptions
    ///
    /// Returns
    /// ------------------------
    /// Set of triple (node, node, edge type) representing a spanning tree.
    ///
    fn spanning_tree(
        &self,
        seed: NodeT,
        include_all_edge_types: bool,
    ) -> HashSet<(NodeT, NodeT, Option<EdgeTypeT>)> {
        self.graph.spanning_tree(seed, include_all_edge_types)
    }
}
