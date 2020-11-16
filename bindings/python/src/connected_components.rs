use super::*;
use graph::NodeT;

#[pymethods]
impl EnsmallenGraph {
    #[text_signature = "($self, verbose)"]
    /// Returns tuple containing the components and relative data.
    ///
    /// Specifically, the values are:
    ///     - Vector of the components for each node.
    ///     - Number of components
    ///     - Minimum component size
    ///     - Maximum component size.
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
    fn connected_components(
        &self,
        verbose: Option<bool>,
    ) -> PyResult<(Vec<NodeT>, NodeT, NodeT, NodeT)> {
        pyex!(self.graph.connected_components(verbose.unwrap_or(true)))
    }
}
