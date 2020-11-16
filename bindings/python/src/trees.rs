use super::*;
use graph::NodeT;
use std::collections::HashSet;

#[pymethods]
impl EnsmallenGraph {
    #[text_signature = "($self, verbose)"]
    /// Returns set of edges forming the spanning tree of given graph.
    ///
    /// Parameters
    /// ------------------------
    /// verbose: bool = True,
    ///     Wether to show a loading bar.
    ///
    /// Raises
    /// ------------------------
    /// ValueError,
    ///     If the given graph is not undirected.
    ///
    /// Returns
    /// ------------------------
    /// Set of tuples of NodeIds forming the spanning tree.
    /// 
    /// References
    /// ------------------------
    /// This is the implementaiton of the algorithm presented in the paper
    /// A Fast, Parallel Spanning Tree Algorithm for Symmetric Multiprocessors
    /// by David A. Bader and Guojing Cong.
    fn spanning_arborescence(
        &self,
        verbose: Option<bool>,
    ) -> PyResult<HashSet<(NodeT, NodeT)>> {
        pyex!(self.graph.spanning_arborescence(verbose.unwrap_or(true)))
    }
}
