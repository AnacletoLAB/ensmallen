use super::*;
use numpy::PyArray2;

#[pymethods]
impl Graph {
    #[pyo3(text_signature = "($self)")]
    /// Return 2D numpy array with base 13 triad census.
    fn get_base_13_triad_census_per_node(&self) -> PyResult<Py<PyArray2<u64>>> {
        let py = pyo3::Python::acquire_gil();

        let triad_census = PyArray2::zeros(
            py.python(),
            [self.get_number_of_nodes() as usize, 13],
            false,
        );

        pe!(self
            .inner
            .get_base_13_triad_census_per_node(pe!(unsafe { triad_census.as_slice_mut() })?))?;

        Ok(triad_census.to_owned())
    }

    #[pyo3(text_signature = "($self)")]
    /// Return 2D numpy array with base 30 triad census.
    fn get_base_30_triad_census_per_node(&self) -> PyResult<Py<PyArray2<u64>>> {
        let py = pyo3::Python::acquire_gil();

        let triad_census = PyArray2::zeros(
            py.python(),
            [self.get_number_of_nodes() as usize, 30],
            false,
        );

        pe!(self
            .inner
            .get_base_30_triad_census_per_node(pe!(unsafe { triad_census.as_slice_mut() })?))?;

        Ok(triad_census.to_owned())
    }
}
