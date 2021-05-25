use super::*;
use graph::NodeT;
use std::collections::HashSet;

#[pymethods]
impl EnsmallenGraph {
    #[text_signature = "($self)"]
    /// Returns a 2-approximated vertex cover.
    ///
    /// [Automatically generated binding]
    /// [Automatically generated documentation]
    fn approximated_vertex_cover(&self) -> HashSet<NodeT> {
        self.graph.approximated_vertex_cover_set()
    }
}
