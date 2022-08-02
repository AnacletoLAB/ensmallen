use super::*;

#[pymethods]
impl Graph {
    #[pyo3(text_signature = "($self)")]
    /// Returns a 64-bit hash of the graph.
    pub fn hash(&self) -> u64 {
        self.inner.compute_hash()
    }
}
