use super::*;
use numpy::{PyArray, PyArray1};

#[pymethods]
impl EnsmallenGraph {
    #[text_signature = "($self)"]
    /// Return numpy array with degree centrality of the nodes in the considered graph.
    fn get_degree_centrality(&self) -> PyResult<Py<PyArray1<f64>>> {
        let gil = pyo3::Python::acquire_gil();
        Ok(to_ndarray_1d!(gil, pe!(self.graph.get_degree_centrality()), f64))
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

    #[text_signature = "($self, normalize, verbose)"]
    /// Return numpy array with stress centrality of the nodes in the considered graph.
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
    /// Numpy array with stress centralities for each node.
    fn get_stress_centrality(
        &self,
        normalize: Option<bool>,
        verbose: Option<bool>,
    ) -> Py<PyArray1<f64>> {
        let gil = pyo3::Python::acquire_gil();
        to_ndarray_1d!(
            gil,
            self.graph.get_stress_centrality(normalize, verbose),
            f64
        )
    }

    #[text_signature = "($self, verbose)"]
    /// Return numpy array with unweighted closeness centrality of the nodes in the considered graph.
    ///
    /// Parameters
    /// ------------------------
    /// verbose: bool = True,
    ///     Whether to show a loading bar,
    ///
    /// Returns
    /// ------------------------
    /// Numpy array with unweighted closeness centralities for each node.
    fn get_unweighted_closeness_centrality(&self, verbose: Option<bool>) -> Py<PyArray1<f64>> {
        let gil = pyo3::Python::acquire_gil();
        to_ndarray_1d!(
            gil,
            self.graph.get_unweighted_closeness_centrality(verbose),
            f64
        )
    }

    #[text_signature = "($self, verbose)"]
    /// Return numpy array with weighted closeness centrality of the nodes in the considered graph.
    ///
    /// Parameters
    /// ------------------------
    /// verbose: bool = True,
    ///     Whether to show a loading bar,
    ///
    /// Returns
    /// ------------------------
    /// Numpy array with weighted closeness centralities for each node.
    fn get_weighted_closeness_centrality(&self, verbose: Option<bool>) -> PyResult<Py<PyArray1<f64>>> {
        let gil = pyo3::Python::acquire_gil();
        Ok(to_ndarray_1d!(
            gil,
            pe!(self.graph.get_weighted_closeness_centrality(verbose)),
            f64
        ))
    }

    #[text_signature = "($self, verbose)"]
    /// Return numpy array with unweighted harmonic centrality of the nodes in the considered graph.
    ///
    /// Parameters
    /// ------------------------
    /// verbose: bool = True,
    ///     Whether to show a loading bar,
    ///
    /// Returns
    /// ------------------------
    /// Numpy array with unweighted harmonic centralities for each node.
    fn get_unweighted_harmonic_centrality(&self, verbose: Option<bool>) -> Py<PyArray1<f64>> {
        let gil = pyo3::Python::acquire_gil();
        to_ndarray_1d!(
            gil,
            self.graph.get_unweighted_harmonic_centrality(verbose),
            f64
        )
    }

    #[text_signature = "($self, verbose)"]
    /// Return numpy array with weighted harmonic centrality of the nodes in the considered graph.
    ///
    /// Parameters
    /// ------------------------
    /// verbose: bool = True,
    ///     Whether to show a loading bar,
    ///
    /// Returns
    /// ------------------------
    /// Numpy array with unweighted harmonic centralities for each node.
    fn get_weighted_harmonic_centrality(&self, verbose: Option<bool>) -> PyResult<Py<PyArray1<f64>>> {
        let gil = pyo3::Python::acquire_gil();
        Ok(to_ndarray_1d!(
            gil,
            pe!(self.graph.get_weighted_harmonic_centrality(verbose))?,
            f64
        ))
    }

    /// Returns vector with unweighted eigenvector centrality.
    ///
    /// Parameters
    /// ----------------
    /// maximum_iterations_number: int = 1000,
    ///     The maximum number of iterations to consider.
    /// tollerance: float = 1e-6,
    ///     The maximum error tollerance for convergence.
    pub fn get_unweighted_eigenvector_centrality(
        &self,
        maximum_iterations_number: Option<usize>,
        tollerance: Option<f64>,
    ) -> PyResult<Py<PyArray1<f64>>> {
        let gil = pyo3::Python::acquire_gil();
        Ok(to_ndarray_1d!(
            gil,
            pe!(self
                .graph
                .get_unweighted_eigenvector_centrality(maximum_iterations_number, tollerance))?,
            f64
        ))
    }

    /// Returns vector with weighted eigenvector centrality.
    ///
    /// Parameters
    /// ----------------
    /// maximum_iterations_number: int = 1000,
    ///     The maximum number of iterations to consider.
    /// tollerance: float = 1e-6,
    ///     The maximum error tollerance for convergence.
    pub fn get_weighted_eigenvector_centrality(
        &self,
        maximum_iterations_number: Option<usize>,
        tollerance: Option<f64>,
    ) -> PyResult<Py<PyArray1<f64>>> {
        let gil = pyo3::Python::acquire_gil();
        Ok(to_ndarray_1d!(
            gil,
            pe!(self
                .graph
                .get_weighted_eigenvector_centrality(maximum_iterations_number, tollerance))?,
            f64
        ))
    }
}
