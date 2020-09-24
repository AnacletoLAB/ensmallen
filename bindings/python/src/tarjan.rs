use super::*;
use graph::{NodeT};
use std::collections::HashSet;

#[pymethods]
impl EnsmallenGraph {
    #[text_signature = "($self)"]
    /// Returns list of sets of nodes of connected components.
    ///
    /// Raises
    /// ------------------------
    /// TODO: update the set of exceptions
    ///
    /// Returns
    /// ------------------------
    /// List of sets of connected components.
    ///
    fn strongly_connected_components(&self) -> Vec<HashSet<NodeT>> {
        self.graph.strongly_connected_components()
    }
}
