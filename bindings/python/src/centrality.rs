use super::*;
use numpy::{PyArray, PyArray1};

#[pymethods]
impl EnsmallenGraph {
    #[text_signature = "($self)"]
    /// Return numpy array with degree centrality of the nodes in the considered graph.
    fn get_degree_centrality(&self) -> Py<PyArray1<f64>> {
        let gil = pyo3::Python::acquire_gil();
        to_ndarray_1d!(gil, self.graph.get_degree_centrality(), f64)
    }

    #[text_signature = "($self, normalize, verbose)"]
    /// Return numpy array with betweenness centrality of the nodes in the considered graph.
    ///
    /// Parameters
    /// ------------------------
    /// normalize: bool = False,
    ///     Whether to normalize the values between 0 and 1.
    /// verbose: bool = True,
    ///     Whether to show a loading bar,
    ///
    /// Returns
    /// ------------------------
    /// Numpy array with betweenness centralities for each node.
    fn get_betweenness_centrality(
        &self,
        normalize: Option<bool>,
        verbose: Option<bool>,
    ) -> Py<PyArray1<f64>> {
        let gil = pyo3::Python::acquire_gil();
        to_ndarray_1d!(
            gil,
            self.graph.get_betweenness_centrality(normalize, verbose),
            f64
        )
    }
}
