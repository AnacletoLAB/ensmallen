use super::*;
use graph::NodeT;
use numpy::{PyArray, PyArray1};

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
    ) -> PyResult<(Py<PyArray1<NodeT>>, NodeT, NodeT, NodeT)> {
        let (components, number, min_size, max_size) =
            pyex!(self.graph.connected_components(verbose.unwrap_or(true)))?;
        let gil = pyo3::Python::acquire_gil();
        Ok((
            to_nparray_1d!(gil, components, NodeT),
            number,
            min_size,
            max_size,
        ))
    }
}
