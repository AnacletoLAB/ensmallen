use super::*;

#[pymethods]
impl EnsmallenGraph {
    #[text_signature = "($self)"]
    /// Returns a 64-bit hash of the graph.
    pub fn hash(&self) -> u64 {
        self.graph.compute_hash();
    }
}