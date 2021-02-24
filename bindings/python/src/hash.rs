use super::*;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

#[pymethods]
impl EnsmallenGraph {
    #[text_signature = "($self)"]
    /// Returns a 64-bit hash of the graph.
    pub fn hash(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        self.graph.hash(&mut hasher);
        hasher.finish()
    }
}