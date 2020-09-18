use super::*;
use graph::{NodeT, NodeTypeT};
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
    ///
    fn spanning_tree(
        &self,
        seed: NodeT,
        include_all_edge_types: bool,
    ) -> HashSet<(NodeT, NodeT, Option<NodeTypeT>)> {
        let tree: HashSet<(NodeT, NodeT, Option<NodeTypeT>)> = self
            .graph
            .spanning_tree(seed, include_all_edge_types)
            .iter()
            .cloned()
            .collect();
        tree
    }
}