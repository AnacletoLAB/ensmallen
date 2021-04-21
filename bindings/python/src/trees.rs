use super::*;
use graph::{EdgeT, EdgeTypeT, NodeT};
use numpy::{PyArray, PyArray1, PyArray2};
use std::collections::HashSet;

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
            pe!(self.graph.connected_components(verbose.unwrap_or(true)))?;
        let gil = pyo3::Python::acquire_gil();
        Ok((
            to_ndarray_1d!(gil, components, NodeT),
            number,
            min_size,
            max_size,
        ))
    }

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
    /// Numpy array of tuples of NodeIds forming the spanning tree.
    ///
    /// References
    /// ------------------------
    /// This is the implementaiton of the algorithm presented in the paper
    /// A Fast, Parallel Spanning Tree Algorithm for Symmetric Multiprocessors
    /// by David A. Bader and Guojing Cong.
    fn spanning_arborescence(&self, verbose: Option<bool>) -> PyResult<Py<PyArray2<NodeT>>> {
        let py = pyo3::Python::acquire_gil();
        let (edges_number, iter) = pe!(self.graph.spanning_arborescence(verbose.unwrap_or(true)))?;
        let array = ThreadSafe {
            t: PyArray2::new(py.python(), [edges_number, 2], false),
        };
        unsafe {
            iter.enumerate().for_each(|(index, (src, dst))| {
                *(array.t.uget_mut([index, 0])) = src;
                *(array.t.uget_mut([index, 1])) = dst;
            });
        }
        Ok(array.t.to_owned())
    }

    #[text_signature = "($self, random_state, undesired_edge_types, verbose)"]
    /// Returns set of edges composing a spanning tree and connected components.
    ///
    ///  The spanning tree is NOT minimal.
    ///  The given random_state is NOT the root of the tree.
    ///
    /// Parameters
    /// --------------
    /// random_state: int,
    /// 	The random_state to use for the holdout,
    /// undesired_edge_types: Union[Dict[Option<int, None]]>,
    /// 	Which edge types id to try to avoid.
    /// verbose: bool,
    /// 	Whether to show a loading bar or not.
    ///
    /// [Automatically generated binding]
    /// [Automatically generated documentation]
    fn random_spanning_arborescence_kruskal(
        &self,
        random_state: EdgeT,
        undesired_edge_types: Option<HashSet<Option<EdgeTypeT>>>,
        verbose: bool,
    ) -> (HashSet<(NodeT, NodeT)>, Vec<NodeT>, NodeT, NodeT, NodeT) {
        self.graph.random_spanning_arborescence_kruskal(
            random_state,
            &undesired_edge_types,
            verbose,
        )
    }

    #[text_signature = "($self, verbose)"]
    /// Returns consistent spanning arborescence using Kruskal.
    ///
    ///  The spanning tree is NOT minimal.
    ///
    /// Parameters
    /// --------------
    /// verbose: bool,
    /// 	Whether to show a loading bar or not.
    ///
    /// [Automatically generated binding]
    /// [Automatically generated documentation]
    fn spanning_arborescence_kruskal(
        &self,
        verbose: bool,
    ) -> (HashSet<(NodeT, NodeT)>, Vec<NodeT>, NodeT, NodeT, NodeT) {
        self.graph.spanning_arborescence_kruskal(verbose)
    }
}
